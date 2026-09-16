use cronc::{compile_to_c23, compile_to_llvm, check_source_diagnostics};
use std::process::Command;

#[test]
fn test_spatial_memory_attributes_syntax() {
    let source = r#"
    .MODULE SpatialMemorySyntaxTest
    _main:
        let a: @sram(bank=0) i32 = 100
        let b: @hbm(channel=1) i32 = 200
        let c: @noc(0, 1, 2, 3) i32 = 300
        let sum: i32 = a + b + c
        return sum
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(diags.is_empty(), "Diagnostics should be empty for valid spatial memory attributes: {:?}", diags);
}

#[test]
fn test_spatial_memory_domain_crossing_violation() {
    let source = r#"
    .MODULE DomainCrossingViolation
    _main:
        let hbm_buf: @hbm(channel=0) i32 = 42
        // Direct assignment across memory tiers without DMA transfer is illegal in CRON:
        let sram_cache: @sram(bank=1) i32 = hbm_buf
        return sram_cache
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(!diags.is_empty(), "Semantic checker must detect direct spatial domain crossing");
    let has_domain_err = diags.iter().any(|d| d.code == "E0015" && d.message.contains("Spatial memory domain crossing"));
    assert!(has_domain_err, "Must emit E0015 error for direct cross-domain assignment: {:?}", diags);
}

#[test]
fn test_systolic_tile_c23_translation() {
    let source = r#"
    .MODULE SystolicTileC23Test
    _main:
        let a: tile4x4_f32 = tile4x4_f32(1.5)
        let b: tile4x4_f32 = tile_transpose(a)
        let c: tile4x4_f32 = tile_matmul(a, b)
        let d: tile4x4_f32 = tile_add(c, a)
        let v: f32 = tile_get(d, 0, 0)
        return (v as i32)
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    assert!(c_code.contains("cron_tile4x4_f32_t a = tile4x4_f32(1.5);"), "C23 must instantiate tile4x4_f32: {}", c_code);
    assert!(c_code.contains("tile_transpose(a)"), "C23 must call tile_transpose: {}", c_code);
    assert!(c_code.contains("tile_matmul(a, b)"), "C23 must call tile_matmul: {}", c_code);
    assert!(c_code.contains("tile_add(c, a)"), "C23 must call tile_add: {}", c_code);
}

#[test]
fn test_systolic_tile_llvm_ir_translation() {
    let source = r#"
    .MODULE SystolicTileLlvmTest
    _main:
        let a: tile4x4_f32 = tile4x4_f32(2.0)
        let v: f32 = tile_get(a, 1, 1)
        return (v as i32)
    .END
    "#;

    let llvm_ir = compile_to_llvm(source).expect("LLVM IR compilation failed");
    assert!(llvm_ir.contains("[4 x [4 x float]]"), "LLVM IR must contain 4x4 matrix tile type: {}", llvm_ir);
}

#[test]
fn test_systolic_tile_gcc_c23_native_execution() {
    let source = r#"
    .MODULE SystolicTileExecutionTest
    _main:
        // Create tile A with all 2.0
        let a: tile4x4_f32 = tile4x4_f32(2.0)

        // Mutate specific elements in A:
        // Set (0, 0) = 1.0, (1, 1) = 3.0
        let a_mod1: tile4x4_f32 = tile_set(a, 0, 0, 1.0)
        let a_mod2: tile4x4_f32 = tile_set(a_mod1, 1, 1, 3.0)

        // Transpose
        let a_t: tile4x4_f32 = tile_transpose(a_mod2)

        // Verify transpose swapped indices:
        let v00: f32 = tile_get(a_t, 0, 0)
        let v11: f32 = tile_get(a_t, 1, 1)
        let v01: f32 = tile_get(a_t, 0, 1)

        if (v00 as i32) != 1 {
            return 11
        }
        if (v11 as i32) != 3 {
            return 12
        }
        if (v01 as i32) != 2 {
            return 13
        }

        // Matmul: a (all 2.0) * a (all 2.0)
        // Each element of result should be sum_{k=0..3} 2.0 * 2.0 = 4 * 4.0 = 16.0
        let prod: tile4x4_f32 = tile_matmul(a, a)
        let p00: f32 = tile_get(prod, 0, 0)
        let p33: f32 = tile_get(prod, 3, 3)

        if (p00 as i32) != 16 {
            return 21
        }
        if (p33 as i32) != 16 {
            return 22
        }

        // FMA: a * a + a_mod2
        // Element (0, 0) should be 16.0 + 1.0 = 17.0
        let fma_res: tile4x4_f32 = tile_fma(a, a, a_mod2)
        let f00: f32 = tile_get(fma_res, 0, 0)
        if (f00 as i32) != 17 {
            return 31
        }

        // Return code 99 on full mathematical success
        return 99
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");

    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("test_systolic_tile.c");
    let exe_path = temp_dir.join("test_systolic_tile.exe");

    std::fs::write(&c_path, &c_code).expect("Failed to write C file");

    let gcc_status = Command::new("gcc")
        .args(&[
            "-std=c2x",
            "-O3",
            c_path.to_str().unwrap(),
            "-o",
            exe_path.to_str().unwrap(),
        ])
        .status()
        .expect("GCC execution failed");

    assert!(gcc_status.success(), "GCC compilation of systolic tiles must succeed without error");

    let run_output = Command::new(&exe_path)
        .output()
        .expect("Failed to execute native systolic tile binary");
    let exit_code = run_output.status.code().unwrap_or(-1);
    assert_eq!(exit_code, 99, "Systolic tile GEMM, transpose, and FMA must return 99, got {}", exit_code);
}
