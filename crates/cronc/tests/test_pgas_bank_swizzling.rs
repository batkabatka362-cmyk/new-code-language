use cronc::{compile_to_c23, compile_source, check_source_diagnostics};
use std::process::Command;

#[test]
fn test_bank_swizzle_mathematical_proof_zero_conflicts() {
    // Mathematical Proof of Zero Bank Conflicts:
    // Given an SRAM with N banks (where N is a power of 2, e.g. 4, 8, 16):
    // For any columnar strided traversal (fixed column c, varying row r):
    // The physical bank accessed is: bank(r, c) = (col ^ (row & (stride - 1))) % N.
    // Since row & (stride - 1) produces every residue in [0, N-1] exactly once per N rows,
    // and XOR with a constant c is a bijective bijection on GF(2^k),
    // the sequence of bank indices is a strict permutation of 0..N-1.
    // Therefore, no two rows in any chunk of N rows access the same bank!
    for stride in [4, 8, 16, 32] {
        for c in 0..stride {
            let mut visited_banks = std::collections::HashSet::new();
            for r in 0..stride {
                let swizzled = (r * stride) + (c ^ (r & (stride - 1)));
                let bank = swizzled % stride;
                assert!(
                    visited_banks.insert(bank),
                    "Conflict detected at stride {}, col {}, row {} -> bank {}",
                    stride, c, r, bank
                );
            }
            assert_eq!(visited_banks.len(), stride, "All banks must be accessed orthogonally");
        }
    }
}

#[test]
fn test_tile_swizzle_and_sram_index_syntax_and_c23() {
    let source = r#"
    .MODULE SwizzleSyntaxTest

    _main:
        let idx: i64 = sram_swizzle_index(2, 3, 4)
        let t: tile4x4_f32 = tile4x4_f32(1.0)
        let swizzled_t: tile4x4_f32 = tile_swizzle(t)
        let v: f32 = tile_get(swizzled_t, 0, 0)
        return (idx as i32)
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(diags.is_empty(), "Swizzle syntax must pass semantic checking: {:?}", diags);

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    assert!(c_code.contains("sram_swizzle_index(2, 3, 4)"), "C23 must call sram_swizzle_index: {}", c_code);
    assert!(c_code.contains("tile_swizzle(t)"), "C23 must call tile_swizzle: {}", c_code);

    let vliw = compile_source(source).expect("VLIW compilation failed");
    assert!(vliw.contains("_TT"), "VLIW codegen must map tile_swizzle to _TT slot: {}", vliw);
}

#[test]
fn test_pgas_syntax_and_vliw_generation() {
    let source = r#"
    .MODULE PgasSyntaxTest

    _main:
        // Write to core (0, 1, 0, 0) at address 64
        pgas_write(0, 1, 0, 0, 64, 42)

        // Barrier synchronization
        pgas_barrier()

        // Read back from remote core
        let val: i64 = pgas_read(0, 1, 0, 0, 64)
        return (val as i32)
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(diags.is_empty(), "PGAS syntax must pass semantic check: {:?}", diags);

    let vliw = compile_source(source).expect("VLIW compilation failed");
    assert!(vliw.contains("_YD"), "VLIW codegen must emit _YD for pgas_write: {}", vliw);
    assert!(vliw.contains("_RC"), "VLIW codegen must emit _RC for pgas_read: {}", vliw);
    assert!(vliw.contains("_bb"), "VLIW codegen must emit _bb for pgas_barrier: {}", vliw);
}

#[test]
fn test_pgas_and_swizzling_gcc_native_execution() {
    let source = r#"
    .MODULE PgasNativeExecutionTest

    _main:
        // 1. Verify bank-conflict-free swizzle index calculation
        // For row=2, col=3, stride=4:
        // swizzled = 2 * 4 + (3 ^ (2 & 3)) = 8 + (3 ^ 2) = 8 + 1 = 9
        let idx: i64 = sram_swizzle_index(2, 3, 4)
        if idx != 9 {
            return 1
        }

        // 2. Perform 4D-Torus PGAS single-sided RDMA operations
        // Core (0, 0, 0, 0) writes to remote Core (2, 1, 0, 0) scratchpad address 128:
        pgas_write(2, 1, 0, 0, 128, 9999)

        // Barrier synchronization
        pgas_barrier()

        // Core (3, 0, 0, 0) reads from remote Core (2, 1, 0, 0) scratchpad address 128:
        let remote_val: i64 = pgas_read(2, 1, 0, 0, 128)
        if remote_val != 9999 {
            return 2
        }

        // 3. Test Systolic Tile Swizzling
        let orig: tile4x4_f32 = tile4x4_f32(0.0)
        // Set element (1, 2) = 7.0
        let modified: tile4x4_f32 = tile_set(orig, 1, 2, 7.0)
        let swizzled: tile4x4_f32 = tile_swizzle(modified)

        // In row 1: new col = col ^ (1 & 3) = 2 ^ 1 = 3
        let v: f32 = tile_get(swizzled, 1, 3)
        if (v as i32) != 7 {
            return 3
        }

        return 0
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("cron_test_pgas_swizzle_exec.c");
    let exe_file = temp_dir.join(if cfg!(windows) { "cron_test_pgas_swizzle_exec.exe" } else { "cron_test_pgas_swizzle_exec" });

    std::fs::write(&c_file, &c_code).expect("Failed to write temporary C file");

    let gcc_check = Command::new("gcc").arg("--version").output();
    if gcc_check.is_ok() {
        let compile_output = Command::new("gcc")
            .arg("-O2")
            .arg(&c_file)
            .arg("-o")
            .arg(&exe_file)
            .output()
            .expect("Failed to execute GCC compiler");

        if !compile_output.status.success() {
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            panic!("GCC compilation failed:\n{}\nSource:\n{}", stderr, c_code);
        }

        let run_output = Command::new(&exe_file)
            .output()
            .expect("Failed to run compiled PGAS binary");

        let exit_code = run_output.status.code().unwrap_or(-1);
        assert_eq!(exit_code, 0, "PGAS & Swizzling kernel executed with exit code: {}", exit_code);

        let _ = std::fs::remove_file(c_file);
        let _ = std::fs::remove_file(exe_file);
    }
}
