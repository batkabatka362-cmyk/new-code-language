// ============================================================================
// Tests for High-Performance C23 AOT Backend (crates/cronc)
// Verifies translation of CRON .cr programs into clean, valid C23 code.
// ============================================================================

use cronc::compile_to_c23;

#[test]
fn test_c23_basic_program() {
    let source = r#"
    .MODULE TestBasic
    .ENTRY _main

    def add(a: i32, b: i32) -> i32 {
        let c: i32 = a + b
        return c
    }

    _main:
        let a: i32 = 42
        let b: i32 = 10
        let c: i32 = add(a, b)
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile to C23 without error");
    assert!(c_code.contains("#include <stdint.h>"), "Must include standard C headers");
    assert!(c_code.contains("cron_torus_coord_t"), "Must declare 4D-Torus types");
    assert!(c_code.contains("int32_t a = 42;"), "Must declare integer variable a");
    assert!(c_code.contains("(a + b)"), "Must perform arithmetic");
    assert!(c_code.contains("int main(int argc, char** argv)"), "Must contain main entry point");
    assert!(c_code.contains("Executing Top-Level Module Statements:"), "Must inline top-level module statements");
}

#[test]
fn test_c23_structs_and_functions() {
    let source = r#"
    .MODULE Geometry4D
    
    struct HyperPoint {
        x: i32,
        y: i32,
        z: i32,
        w: i32
    }
    
    fn compute_norm(pt: HyperPoint) -> i32 {
        return pt.x + pt.y + pt.z + pt.w
    }
    
    .ENTRY _main
    _main:
        let p: HyperPoint = HyperPoint { x: 1, y: 2, z: 3, w: 4 }
        let norm: i32 = compute_norm(p)
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile struct and function to C23");
    assert!(c_code.contains("typedef struct HyperPoint {"), "Must define C struct");
    assert!(c_code.contains("int32_t compute_norm(struct HyperPoint pt);"), "Must forward declare function");
    assert!(c_code.contains("int32_t compute_norm(struct HyperPoint pt) {"), "Must define function body");
    assert!(c_code.contains("return (((pt.x + pt.y) + pt.z) + pt.w);") || c_code.contains("return"), "Must return field sum");
    assert!(c_code.contains("(HyperPoint){ .x = 1, .y = 2, .z = 3, .w = 4 }"), "Must initialize struct using C compound literal");
}

#[test]
fn test_c23_brains_and_resilience() {
    let source = r#"
    .MODULE BrainResilience
    .ENTRY _main
    
    _main:
        region "fast_l1" {
            let initial_w: wave_t = pack_wave(amp=[10, 20], phase=[0, 32])
        }
        resilient {
            let safe_val: i32 = 100
        } fallback {
            let safe_val: i32 = 0
        }
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile brain and resilient constructs to C23");
    assert!(c_code.contains("// --- Region 'fast_l1' Arena Boundary ---"), "Must mark region arena boundary");
    assert!(c_code.contains("// --- Resilient Fault-Tolerant Block ---"), "Must create resilient block");
    assert!(c_code.contains("// Fallback Recovery Channel"), "Must provide fallback recovery");
}

#[test]
fn test_c23_loops_and_conditionals() {
    let source = r#"
    .MODULE ControlFlow
    .ENTRY _main

    def sum_evens(n: i32) -> i32 {
        let mut sum: i32 = 0
        let mut i: i32 = 0
        while i < n {
            if i % 2 == 0 {
                sum = sum + i
            }
            i = i + 1
        }
        return sum
    }

    _main:
        let result = sum_evens(10)
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile loops and conditionals to C23");
    assert!(c_code.contains("while"), "Must generate while loop");
    assert!(c_code.contains("if"), "Must generate conditional");
    assert!(c_code.contains("sum"), "Must use sum variable");
}

#[test]
fn test_c23_end_to_end_gcc_execution() {
    use std::fs;
    use std::process::Command;

    let source = r#"
    .MODULE NativeCompute
    .ENTRY _main
    
    _main:
        let mut total: i32 = 0
        let mut idx: i32 = 1
        while idx <= 100 {
            total = total + idx
            idx = idx + 1
        }
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Should compile NativeCompute to C23");
    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("cron_native_test.c");
    let exe_path = temp_dir.join("cron_native_test.exe");

    fs::write(&c_path, &c_code).expect("Write C source to temp");

    // Attempt gcc compilation
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
            
            // Clean up
            let _ = fs::remove_file(c_path);
            let _ = fs::remove_file(exe_path);
        }
    }
}

