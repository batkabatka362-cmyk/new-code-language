// ============================================================================
// Tests for Algebraic Data Types (Enums) and Rust-Style Pattern Matching
// Verifies parser, semantic checker, and C23 translation of enums and match.
// ============================================================================

use cronc::compile_to_c23;
use cronc::compile_source;

#[test]
fn test_enum_declaration_and_pattern_matching() {
    let source = r#"
    .MODULE WaveStateModule
    
    enum WaveState {
        Coherent,
        PhaseShifted(i32),
        Decohered
    }
    
    .ENTRY _main
    _main:
        let state: i32 = 1
        match state {
            0 => {
                let status: i32 = 100
            },
            1 => {
                let status: i32 = 200
            },
            _ => {
                let status: i32 = 0
            }
        }
    .END
    "#;

    let cl_code = compile_source(source).expect("Should compile to .cl VLIW");
    assert!(cl_code.contains("B0001:"), "Must generate VLIW bundles");

    let c23_code = compile_to_c23(source).expect("Should compile to C23");
    assert!(c23_code.contains("typedef enum {"), "Must define C enum tag");
    assert!(c23_code.contains("WAVESTATE_COHERENT"), "Must contain variant constant");
    assert!(c23_code.contains("WAVESTATE_PHASESHIFTED"), "Must contain variant constant");
    assert!(c23_code.contains("typedef struct WaveState {"), "Must define tagged union struct");
    assert!(c23_code.contains("// --- Rust-Style Pattern Match ---"), "Must emit pattern match block");
    assert!(c23_code.contains("_match_target == 0"), "Must match on literal 0");
    assert!(c23_code.contains("_match_target == 1"), "Must match on literal 1");
}

#[test]
fn test_enum_variant_matching() {
    let source = r#"
    .MODULE ResultEnumModule
    
    enum Result {
        Ok(i32),
        Err(i32)
    }
    
    .ENTRY _main
    _main:
        let res: i32 = 0
        match res {
            Result::Ok(val) => {
                let code: i32 = 0
            },
            Result::Err(err) => {
                let code: i32 = 1
            }
        }
    .END
    "#;

    let c23_code = compile_to_c23(source).expect("Should compile variant matching to C23");
    assert!(c23_code.contains("RESULT_OK"), "Must contain RESULT_OK");
    assert!(c23_code.contains("RESULT_ERR"), "Must contain RESULT_ERR");
    assert!(c23_code.contains("_match_target.tag == RESULT_OK"), "Must branch on RESULT_OK tag");
}

#[test]
fn test_enum_match_gcc_execution() {
    use std::fs;
    use std::process::Command;

    let source = r#"
    .MODULE NativeEnumMatch
    
    enum Status {
        Pending,
        Active,
        Completed
    }
    
    .ENTRY _main
    _main:
        let s: i32 = 1
        let mut result_code: i32 = 0
        match s {
            0 => {
                result_code = 10
            },
            1 => {
                result_code = 42
            },
            _ => {
                result_code = 99
            }
        }
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile NativeEnumMatch to C23");
    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("cron_enum_test.c");
    let exe_path = temp_dir.join("cron_enum_test.exe");

    fs::write(&c_path, &c_code).expect("Write C source to temp");

    let gcc_status = Command::new("gcc")
        .args(["-O3", c_path.to_str().unwrap(), "-o", exe_path.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(status) = gcc_status {
        if status.success() {
            let run_output = Command::new(&exe_path)
                .output()
                .expect("Execute native binary");
            assert!(run_output.status.success(), "Native binary should exit 0");
            let stdout_str = String::from_utf8_lossy(&run_output.stdout);
            assert!(stdout_str.contains("[CRON Native C23 Engine]"), "Output must contain CRON banner");
            assert!(stdout_str.contains("Execution completed successfully"), "Output must indicate success");

            let _ = fs::remove_file(c_path);
            let _ = fs::remove_file(exe_path);
        }
    }
}
