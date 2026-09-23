use cronc::{compile_to_c23, compile_source, check_source_diagnostics};
use std::process::Command;

#[test]
fn test_schedule_syntax_and_parsing() {
    let source = r#"
    .MODULE ScheduleSyntaxTest

    def matmul_kernel(a: i32, b: i32) -> i32 {
        return a * b
    }

    schedule matmul_kernel for "silicon_accelerator" {
        tile_size(4, 4)
        prefetch_to("sram")
        unroll(4)
        distribute_4d(axis: "X", cores: 4)
        vectorize(8)
    }

    _main:
        let res: i32 = matmul_kernel(3, 7)
        return res
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(diags.is_empty(), "Schedule syntax should pass semantic check without errors: {:?}", diags);

    let vliw = compile_source(source).expect("Compilation to VLIW assembly should succeed");
    assert!(!vliw.is_empty(), "Generated VLIW code must not be empty");
}

#[test]
fn test_schedule_unknown_target_fn_rejected_e0011() {
    let source = r#"
    .MODULE ScheduleUnknownFnTest

    def existing_fn(a: i32) -> i32 {
        return a + 1
    }

    schedule nonexistent_kernel for "accelerator" {
        tile_size(4, 4)
        unroll(2)
    }

    _main:
        return 0
    .END
    "#;

    let diags = check_source_diagnostics(source);
    assert!(!diags.is_empty(), "Must reject schedule targeting non-existent function");
    let has_e0011 = diags.iter().any(|d| d.code == "E0011" && d.message.contains("nonexistent_kernel"));
    assert!(has_e0011, "Must emit E0011 error: {:?}", diags);
}

#[test]
fn test_schedule_invalid_directives_rejected_e0012() {
    // 1. Zero unroll factor
    let source_zero_unroll = r#"
    .MODULE InvalidUnroll
    def kernel() -> i32 { return 0 }
    schedule kernel for "generic" {
        unroll(0)
    }
    _main:
        return 0
    .END
    "#;
    let diags1 = check_source_diagnostics(source_zero_unroll);
    assert!(diags1.iter().any(|d| d.code == "E0012"), "Must reject unroll(0): {:?}", diags1);

    // 2. Invalid 4D-Torus axis
    let source_invalid_axis = r#"
    .MODULE InvalidAxis
    def kernel() -> i32 { return 0 }
    schedule kernel for "generic" {
        distribute_4d(axis: "INVALID_AXIS", cores: 4)
    }
    _main:
        return 0
    .END
    "#;
    let diags2 = check_source_diagnostics(source_invalid_axis);
    assert!(diags2.iter().any(|d| d.code == "E0012"), "Must reject invalid axis: {:?}", diags2);

    // 3. Non-power-of-two vector width
    let source_non_pow2_vec = r#"
    .MODULE NonPow2Vec
    def kernel() -> i32 { return 0 }
    schedule kernel for "generic" {
        vectorize(7)
    }
    _main:
        return 0
    .END
    "#;
    let diags3 = check_source_diagnostics(source_non_pow2_vec);
    assert!(diags3.iter().any(|d| d.code == "E0012"), "Must reject non-power-of-two vector width: {:?}", diags3);
}

#[test]
fn test_schedule_c23_translation_and_pragmas() {
    let source = r#"
    .MODULE ScheduleC23PragmaTest

    def compute_tile(a: i32, b: i32) -> i32 {
        let mut sum: i32 = 0
        for i in 0..4 {
            sum = sum + a * b
        }
        return sum
    }

    schedule compute_tile for "torus_core" {
        tile_size(4, 4)
        prefetch_to("sram")
        unroll(4)
        distribute_4d(axis: "X", cores: 4)
        vectorize(4)
    }

    _main:
        let r: i32 = compute_tile(2, 3)
        return r
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    assert!(c_code.contains("// --- Decoupled Silicon Schedule for 'compute_tile' [Target: torus_core] ---"),
        "C23 must emit schedule header: {}", c_code);
    assert!(c_code.contains("// [Schedule Directive] TileSize: 4x4"),
        "C23 must emit TileSize directive comment: {}", c_code);
    assert!(c_code.contains("// [Schedule Directive] PrefetchTo: sram"),
        "C23 must emit PrefetchTo directive comment: {}", c_code);
    assert!(c_code.contains("#pragma GCC optimize (\"unroll-loops\")"),
        "C23 must emit unroll-loops pragma: {}", c_code);
    assert!(c_code.contains("#pragma GCC optimize (\"tree-vectorize\")"),
        "C23 must emit tree-vectorize pragma: {}", c_code);
    assert!(c_code.contains("// [Schedule Directive] Distribute4D: axis X across 4 cores"),
        "C23 must emit Distribute4D directive comment: {}", c_code);
}

#[test]
fn test_schedule_c23_native_execution() {
    let source = r#"
    .MODULE ScheduleExecutionTest

    def scheduled_gemm(count: i32) -> i32 {
        let mut acc: i32 = 0
        for i in 0..count {
            acc = acc + 5
        }
        return acc
    }

    schedule scheduled_gemm for "x86_64" {
        tile_size(4, 4)
        unroll(4)
        vectorize(4)
    }

    _main:
        let total: i32 = scheduled_gemm(10)
        if total != 50 {
            return 1
        }
        return 0
    .END
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("cron_test_schedule_exec.c");
    let exe_file = temp_dir.join(if cfg!(windows) { "cron_test_schedule_exec.exe" } else { "cron_test_schedule_exec" });

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
            .expect("Failed to run compiled scheduled binary");

        let exit_code = run_output.status.code().unwrap_or(-1);
        assert_eq!(exit_code, 0, "Scheduled kernel executed with non-zero exit code: {}", exit_code);

        let _ = std::fs::remove_file(c_file);
        let _ = std::fs::remove_file(exe_file);
    }
}
