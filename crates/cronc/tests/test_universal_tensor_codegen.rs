//! Universal Tensor Codegen Verification Suite
//!
//! Validates that CRON's C23 codegen is a TRUE general recursive tensor expression compiler,
//! NOT an ad-hoc pattern matching script. Any arbitrary mathematical expression composed
//! of contractions (@), elementwise operators (+, -, *, /), activations (silu, tanh, relu, exp, sqrt),
//! and rank-broadcasting must compile to native C23, execute via GCC, and match MathEvaluator
//! down to numerical tolerance < 1e-4.

use cronc::cl_math::{parse_math_code, MathEvaluator};
use std::fs;
use std::process::Command;

fn run_c23_executable_and_capture(c23_code: &str, test_name: &str) -> String {
    let temp_dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let c23_src_path = temp_dir.join(format!("{}_{}.c", test_name, ts));
    let c23_bin_path = temp_dir.join(format!("{}_{}.exe", test_name, ts));

    fs::write(&c23_src_path, c23_code).expect("Failed to write C23 source file");

    let gcc_res = Command::new("gcc")
        .args([
            "-O3",
            c23_src_path.to_str().unwrap(),
            "-o",
            c23_bin_path.to_str().unwrap(),
            "-lm",
        ])
        .output();

    let gcc_out = gcc_res.expect("Failed to run GCC compiler");
    assert!(
        gcc_out.status.success(),
        "GCC compilation failed for {}:\nSTDOUT:\n{}\nSTDERR:\n{}\nCODE:\n{}",
        test_name,
        String::from_utf8_lossy(&gcc_out.stdout),
        String::from_utf8_lossy(&gcc_out.stderr),
        c23_code
    );

    let run_res = Command::new(&c23_bin_path)
        .output()
        .expect("Failed to execute compiled C23 binary");
    assert!(
        run_res.status.success(),
        "C23 execution failed for {}:\nSTDERR:\n{}",
        test_name,
        String::from_utf8_lossy(&run_res.stderr)
    );

    // Clean up temporary binaries
    let _ = fs::remove_file(&c23_src_path);
    let _ = fs::remove_file(&c23_bin_path);

    String::from_utf8(run_res.stdout).expect("Failed to parse stdout as UTF-8")
}

fn parse_c23_var_output(stdout: &str, var_name: &str) -> Vec<f32> {
    let target = format!("[=] {} =", var_name);
    for line in stdout.lines() {
        if line.contains(&target) {
            let eq_pos = line.rfind('=').unwrap();
            let after_eq = &line[eq_pos + 1..];
            let mut vals = Vec::new();
            for token in after_eq.split(|c: char| c == '[' || c == ']' || c == ',' || c.is_whitespace()) {
                let trimmed = token.trim();
                if !trimmed.is_empty() {
                    if let Ok(v) = trimmed.parse::<f32>() {
                        vals.push(v);
                    }
                }
            }
            return vals;
        }
    }
    panic!("Variable '{}' not found in stdout:\n{}", var_name, stdout);
}

#[test]
fn test_universal_1d_arbitrary_vector_expressions() {
    let code = r#"
        x = [1.0, -2.0, 3.0, -4.0]
        y = [0.5, 1.5, -0.5, 2.0]
        z = x + y
        act = silu(x) * tanh(y)
        poly = 2.5 * x - 1.25 * y + 0.1
    "#;

    let prog = parse_math_code(code).expect("Failed to parse 1D vector test");
    assert!(prog.verify_shapes().is_ok());

    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Rust MathEvaluator failed");

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_executable_and_capture(&c23_code, "test_1d_arbitrary");

    for var in &["z", "act", "poly"] {
        let rust_val = eval.env.get(*var).expect("Variable missing from Rust env");
        let rust_flat: Vec<f32> = rust_val.flatten_elements().into_iter().map(|v| v as f32).collect();
        let c23_flat = parse_c23_var_output(&stdout, var);

        assert_eq!(rust_flat.len(), c23_flat.len(), "Dimension mismatch on {}", var);
        for (i, (&r, &c)) in rust_flat.iter().zip(c23_flat.iter()).enumerate() {
            assert!(
                (r - c).abs() < 1e-4,
                "Parity mismatch on {}[{}]: Rust={:.6}, C23={:.6}",
                var, i, r, c
            );
        }
    }
}

#[test]
fn test_universal_2d_dual_gated_mlp_composition() {
    let code = r#"
        X = [[1.0, 2.0], [3.0, 4.0]]
        W1 = [[0.2, -0.1], [0.4, 0.3]]
        b1 = [0.05, -0.05]
        W2 = [[-0.3, 0.5], [0.1, -0.2]]
        b2 = [-0.1, 0.2]
        H1 = silu(X @ W1 + b1)
        H2 = tanh(X @ W2 + b2)
        Y = H1 * H2 + 0.01
    "#;

    let prog = parse_math_code(code).expect("Failed to parse dual gated MLP");
    assert!(prog.verify_shapes().is_ok());

    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Rust MathEvaluator failed");

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_executable_and_capture(&c23_code, "test_dual_gated_mlp");

    for var in &["H1", "H2", "Y"] {
        let rust_val = eval.env.get(*var).expect("Variable missing from Rust env");
        let rust_flat: Vec<f32> = rust_val.flatten_elements().into_iter().map(|v| v as f32).collect();
        let c23_flat = parse_c23_var_output(&stdout, var);

        assert_eq!(rust_flat.len(), c23_flat.len(), "Dimension mismatch on {}", var);
        for (i, (&r, &c)) in rust_flat.iter().zip(c23_flat.iter()).enumerate() {
            assert!(
                (r - c).abs() < 1e-4,
                "Parity mismatch on {}[{}]: Rust={:.6}, C23={:.6}",
                var, i, r, c
            );
        }
    }
}

#[test]
fn test_universal_gaussian_rbf_and_linear_mix() {
    let code = r#"
        X = [[0.5, -0.5], [1.0, 0.0]]
        W = [[0.8, -0.2], [0.3, 0.7]]
        Y = exp(-(X * X)) + 0.5 * (X @ W)
    "#;

    let prog = parse_math_code(code).expect("Failed to parse Gaussian RBF");
    assert!(prog.verify_shapes().is_ok());

    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Rust MathEvaluator failed");

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_executable_and_capture(&c23_code, "test_gaussian_rbf");

    let rust_val = eval.env.get("Y").expect("Variable missing from Rust env");
    let rust_flat: Vec<f32> = rust_val.flatten_elements().into_iter().map(|v| v as f32).collect();
    let c23_flat = parse_c23_var_output(&stdout, "Y");

    assert_eq!(rust_flat.len(), c23_flat.len(), "Dimension mismatch on Y");
    for (i, (&r, &c)) in rust_flat.iter().zip(c23_flat.iter()).enumerate() {
        assert!(
            (r - c).abs() < 1e-4,
            "Parity mismatch on Y[{}]: Rust={:.6}, C23={:.6}",
            i, r, c
        );
    }
}

#[test]
fn test_universal_dual_matmul_contraction_sum() {
    let code = r#"
        A = [[1.0, 2.0], [3.0, 4.0]]
        B = [[0.5, -0.5], [1.5, 0.5]]
        C = [[-1.0, 1.0], [2.0, -2.0]]
        D = [[0.2, 0.3], [0.4, 0.5]]
        Y = (A @ B) + (C @ D)
    "#;

    let prog = parse_math_code(code).expect("Failed to parse dual matmul sum");
    assert!(prog.verify_shapes().is_ok());

    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Rust MathEvaluator failed");

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_executable_and_capture(&c23_code, "test_dual_matmul_sum");

    let rust_val = eval.env.get("Y").expect("Variable missing from Rust env");
    let rust_flat: Vec<f32> = rust_val.flatten_elements().into_iter().map(|v| v as f32).collect();
    let c23_flat = parse_c23_var_output(&stdout, "Y");

    assert_eq!(rust_flat.len(), c23_flat.len(), "Dimension mismatch on Y");
    for (i, (&r, &c)) in rust_flat.iter().zip(c23_flat.iter()).enumerate() {
        assert!(
            (r - c).abs() < 1e-4,
            "Parity mismatch on Y[{}]: Rust={:.6}, C23={:.6}",
            i, r, c
        );
    }
}

#[test]
fn test_universal_3d_batched_tensor_arithmetic() {
    let code = r#"
        A = [[[1.0, 2.0], [3.0, 4.0]], [[5.0, 6.0], [7.0, 8.0]]]
        B = [[[0.5, 0.5], [0.5, 0.5]], [[1.0, -1.0], [2.0, -2.0]]]
        Y = silu(A) + tanh(B) * 0.5
    "#;

    let prog = parse_math_code(code).expect("Failed to parse 3D batched tensor arithmetic");
    assert!(prog.verify_shapes().is_ok());

    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Rust MathEvaluator failed");

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_executable_and_capture(&c23_code, "test_3d_tensor");

    let rust_val = eval.env.get("Y").expect("Variable missing from Rust env");
    let rust_flat: Vec<f32> = rust_val.flatten_elements().into_iter().map(|v| v as f32).collect();
    let c23_flat = parse_c23_var_output(&stdout, "Y");

    assert_eq!(rust_flat.len(), c23_flat.len(), "Dimension mismatch on Y");
    for (i, (&r, &c)) in rust_flat.iter().zip(c23_flat.iter()).enumerate() {
        assert!(
            (r - c).abs() < 1e-4,
            "Parity mismatch on 3D Y[{}]: Rust={:.6}, C23={:.6}",
            i, r, c
        );
    }
}


