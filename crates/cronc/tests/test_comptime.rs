use cronc::*;
use std::fs;
use std::process::Command;

#[test]
fn test_comptime_basic_arithmetic_folding() {
    let source = r#"
    .MODULE ComptimeBasic
    .ENTRY main

    def main() -> i64 {
        let folded: i64 = comptime {
            let a = 10;
            let b = 32;
            a + b
        };
        return folded;
    }
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Failed to compile comptime basic to C23");
    // Verify that the folded constant 42 appears directly in the generated code
    assert!(c_code.contains("42"), "Expected folded constant 42 in generated C23 code:\n{}", c_code);
}

#[test]
fn test_comptime_photonic_mzi_lut_generation() {
    let source = r#"
    .MODULE PhotonicMziComptime
    .ENTRY main

    def main() -> i64 {
        let mzi_phases = comptime {
            let mut phases = [];
            for k in 0..8 {
                // Precompute cos(k * PI / 4)
                let angle = (k as f64) * 3.141592653589793 / 4.0;
                phases.push(cos(angle));
            }
            phases
        };

        // At index 0, cos(0) == 1.0; at index 2, cos(PI/2) == 0.0
        let phase_0: f64 = mzi_phases[0];
        if (phase_0 > 0.99 and phase_0 < 1.01) {
            return 0;
        } else {
            return 1;
        }
    }
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Failed to compile Photonic MZI comptime");
    assert!(c_code.contains("1.0"), "Expected precomputed cos(0)=1.0 in C23 output:\n{}", c_code);

    // Verify native execution via gcc
    let test_dir = std::env::temp_dir();
    let c_path = test_dir.join("test_mzi_comptime.c");
    let bin_path = test_dir.join(if cfg!(windows) { "test_mzi_comptime.exe" } else { "test_mzi_comptime" });

    fs::write(&c_path, &c_code).expect("Failed to write temporary C file");
    let status = Command::new("gcc")
        .args(["-O3", c_path.to_str().unwrap(), "-o", bin_path.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(st) = status {
        if st.success() {
            let run_res = Command::new(&bin_path).status().expect("Failed to run binary");
            assert_eq!(run_res.code(), Some(0), "Compiled comptime program must return 0");
            let _ = fs::remove_file(bin_path);
        }
    }
    let _ = fs::remove_file(c_path);
}

#[test]
fn test_comptime_fibonacci_precomputation() {
    let source = r#"
    .MODULE ComptimeFib
    .ENTRY main

    def fib(n: i64) -> i64 {
        if n <= 1 {
            return n;
        }
        return fib(n - 1) + fib(n - 2);
    }

    def main() -> i64 {
        // Precomputed at compile time!
        let fib10: i64 = comptime { fib(10) };
        if fib10 == 55 {
            return 0;
        } else {
            return 1;
        }
    }
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Failed to compile fib comptime");
    assert!(c_code.contains("55"), "Expected precomputed fib(10)=55 in generated C23:\n{}", c_code);

    let test_dir = std::env::temp_dir();
    let c_path = test_dir.join("test_fib_comptime.c");
    let bin_path = test_dir.join(if cfg!(windows) { "test_fib_comptime.exe" } else { "test_fib_comptime" });

    fs::write(&c_path, &c_code).expect("Failed to write temporary C file");
    let status = Command::new("gcc")
        .args(["-O3", c_path.to_str().unwrap(), "-o", bin_path.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(st) = status {
        if st.success() {
            let run_res = Command::new(&bin_path).status().expect("Failed to run binary");
            assert_eq!(run_res.code(), Some(0), "Precomputed fib(10)=55 must evaluate to 0 exit code");
            let _ = fs::remove_file(bin_path);
        }
    }
    let _ = fs::remove_file(c_path);
}

#[test]
fn test_comptime_bitnet_weight_packing() {
    let source = r#"
    .MODULE BitNetComptime
    .ENTRY main

    def main() -> i64 {
        let packed_weights = comptime {
            // Pack 4 ternary weights {-1, 0, 1, -1} into a 8-bit mask: (code & 0x3)
            // -1 -> 0b00, 0 -> 0b01, +1 -> 0b10
            let w0 = 0; // -1
            let w1 = 1; // 0
            let w2 = 2; // +1
            let w3 = 0; // -1
            w0 | (w1 << 2) | (w2 << 4) | (w3 << 6)
        };

        // 0 | (1 << 2) | (2 << 4) | (0 << 6) = 0 + 4 + 32 + 0 = 36
        if packed_weights == 36 {
            return 0;
        } else {
            return 1;
        }
    }
    .END
    "#;

    let c_code = compile_to_c23(source).expect("Failed to compile bitnet packing");
    assert!(c_code.contains("36"), "Expected folded packed weight 36 in C23:\n{}", c_code);
}
