use cronc::cl_math::{parse_math_code, MathEvaluator};
use std::fs;
use std::process::Command;

fn run_c23_code(c23_src: &str, bin_name: &str) -> Result<String, String> {
    let cwd = std::env::current_dir().unwrap_or_default();
    let temp_dir = cwd.join("target").join("test_composition");
    fs::create_dir_all(&temp_dir).map_err(|e| e.to_string())?;

    let src_path = temp_dir.join(format!("{}.c", bin_name));
    let bin_path = temp_dir.join(if cfg!(windows) {
        format!("{}.exe", bin_name)
    } else {
        bin_name.to_string()
    });

    fs::write(&src_path, c23_src).map_err(|e| e.to_string())?;

    let compile_status = Command::new("gcc")
        .args(&[
            "-O3",
            "-std=c2x",
            src_path.to_str().unwrap(),
            "-o",
            bin_path.to_str().unwrap(),
            "-lm",
        ])
        .output()
        .map_err(|e| format!("GCC invocation failed: {}", e))?;

    if !compile_status.status.success() {
        return Err(format!(
            "GCC Compilation failed:\n{}",
            String::from_utf8_lossy(&compile_status.stderr)
        ));
    }

    let run_output = Command::new(&bin_path)
        .output()
        .map_err(|e| format!("Executable run failed: {}", e))?;

    if !run_output.status.success() {
        return Err(format!(
            "Runtime error:\n{}",
            String::from_utf8_lossy(&run_output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
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
fn test_composition_reversed_dual_gated_mlp_with_offsets() {
    let code = r#"
        X = [[0.5, -0.2], [1.2, 0.8]]
        W1 = [[0.4, -0.1], [0.3, 0.5]]
        W2 = [[-0.2, 0.6], [0.1, -0.4]]
        W3 = [[0.5, 0.2], [-0.3, 0.7]]
        W4 = [[0.1, 0.2], [0.3, 0.4]]
        b1 = [0.1, -0.05]
        b2 = [-0.1, 0.2]

        // Y = ((X @ W2 + b2) * silu(X @ W1 + b1)) @ W3 + tanh(X @ W4)
        H_linear = X @ W2 + b2
        H_gate = silu(X @ W1 + b1)
        H_fused = H_linear * H_gate
        Y_proj = H_fused @ W3
        Y_res = tanh(X @ W4)
        Y = Y_proj + Y_res
    "#;

    let prog = parse_math_code(code).expect("Parse reversed dual-gated MLP");

    // 1. Evaluate with pure Rust MathEvaluator
    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Execute in MathEvaluator");
    let y_eval = eval.env.get("Y").expect("Y not found in evaluator env");
    let y_expected: Vec<f32> = y_eval.flatten_elements().into_iter().map(|v| v as f32).collect();

    // 2. Generate C23 executable
    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_code(&c23_code, "test_comp_dual_gated").expect("C23 run failed");
    let c23_flat = parse_c23_var_output(&stdout, "Y");

    assert_eq!(y_expected.len(), c23_flat.len());
    for i in 0..y_expected.len() {
        assert!(
            (y_expected[i] - c23_flat[i]).abs() < 1e-4,
            "Mismatch on Y[{}]: Rust={:.6}, C23={:.6}",
            i,
            y_expected[i],
            c23_flat[i]
        );
    }

    // 3. Verify PTX and VLIW emission
    let ptx = prog.to_ptx();
    assert!(ptx.contains(".version 7.5"), "Missing PTX version");

    let bundles = prog.to_vliw_bundles();
    assert!(!bundles.is_empty(), "Missing VLIW bundles");
}

#[test]
fn test_composition_spiking_lif_temporal_step() {
    let code = r#"
        X = [0.8, -0.4, 1.2, 0.5]
        W = [[0.5, 0.1, -0.2, 0.3], [0.2, 0.4, 0.1, -0.1], [-0.1, 0.3, 0.5, 0.2], [0.4, -0.2, 0.1, 0.6]]
        u_prev = [0.2, 0.4, 0.1, 0.0]
        s_prev = [1.0, 0.0, 0.0, 0.0]
        beta = 0.85
        v_th = 0.5

        // Leaky Integrate-and-Fire recurrence:
        // u_next = beta * u_prev + (X @ W) - s_prev * v_th
        current_input = X @ W
        u_decay = u_prev * beta
        u_reset = s_prev * v_th
        u_next = u_decay + current_input - u_reset
    "#;

    let prog = parse_math_code(code).expect("Parse Spiking LIF");
    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Execute in MathEvaluator");

    let u_val = eval.env.get("u_next").expect("u_next missing");
    let u_expected: Vec<f32> = u_val.flatten_elements().into_iter().map(|v| v as f32).collect();

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_code(&c23_code, "test_comp_lif").expect("C23 LIF run failed");
    let c23_flat = parse_c23_var_output(&stdout, "u_next");

    assert_eq!(u_expected.len(), c23_flat.len());
    for i in 0..u_expected.len() {
        assert!(
            (u_expected[i] - c23_flat[i]).abs() < 1e-4,
            "Mismatch on u_next[{}]: Rust={:.6}, C23={:.6}",
            i,
            u_expected[i],
            c23_flat[i]
        );
    }
}

#[test]
fn test_composition_mamba2_ssd_and_swiglu_hybrid_block() {
    let code = r#"
        X = [1.0, 2.0, -1.0, 0.5]
        decay = [0.9, 0.85, 0.95, 0.8]
        W_gate = [[0.4, -0.2], [0.1, 0.5], [-0.3, 0.2], [0.2, 0.1]]
        W_up = [[-0.1, 0.3], [0.5, -0.2], [0.2, 0.4], [0.1, -0.3]]
        W_down = [[0.6, 0.2, -0.1, 0.3], [-0.2, 0.5, 0.4, 0.1]]

        // 1. Mamba-2 State-Space Duality recurrence scan
        h = ssd_scan(X, decay)

        // 2. SwiGLU non-linear gating projection
        gate = silu(h @ W_gate)
        up = h @ W_up
        gated_act = gate * up
        mlp_out = gated_act @ W_down

        // 3. Residual connection
        out = h + mlp_out
    "#;

    let prog = parse_math_code(code).expect("Parse Mamba-2 + SwiGLU hybrid");
    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Execute in MathEvaluator");

    let out_val = eval.env.get("out").expect("out missing");
    let out_expected: Vec<f32> = out_val.flatten_elements().into_iter().map(|v| v as f32).collect();

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_code(&c23_code, "test_comp_mamba_swiglu").expect("C23 Mamba-SwiGLU run failed");
    let c23_flat = parse_c23_var_output(&stdout, "out");

    assert_eq!(out_expected.len(), c23_flat.len());
    for i in 0..out_expected.len() {
        assert!(
            (out_expected[i] - c23_flat[i]).abs() < 1e-4,
            "Mismatch on out[{}]: Rust={:.6}, C23={:.6}",
            i,
            out_expected[i],
            c23_flat[i]
        );
    }
}

#[test]
fn test_composition_clifford_4d_rotor_field() {
    let code = r#"
        theta = 0.7853981633974483
        half_th = theta * 0.5
        c = cos(half_th)
        s = sin(half_th)
        // 4D rotor bivector plane representation:
        rotor = [c, s, 0.0, 0.0]
        // Length preservation check: c^2 + s^2 = 1.0
        mag_sq = c * c + s * s
    "#;

    let prog = parse_math_code(code).expect("Parse Clifford 4D rotor");
    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).expect("Execute in MathEvaluator");

    let mag_val = eval.env.get("mag_sq").expect("mag_sq missing");
    let mag_scalar = mag_val.as_scalar().expect("Expected scalar for mag_sq");
    assert!((mag_scalar - 1.0).abs() < 1e-6, "Rotor magnitude squared must be 1.0, got {}", mag_scalar);

    let c23_code = prog.to_c23_executable();
    let stdout = run_c23_code(&c23_code, "test_comp_rotor").expect("C23 Rotor run failed");
    let c23_flat = parse_c23_var_output(&stdout, "rotor");
    assert_eq!(c23_flat.len(), 4);
    assert!((c23_flat[0] - (0.7853981633974483f32 * 0.5).cos()).abs() < 1e-4);
    assert!((c23_flat[1] - (0.7853981633974483f32 * 0.5).sin()).abs() < 1e-4);
}

#[test]
fn test_composition_algebraic_scalar_identity_folding() {
    let code = r#"
        x = 5.0
        y = (x + 0.0) * 1.0 - 0.0
        z = y / 1.0 + (3.0 * 2.0 - 1.0)
    "#;
    let prog = parse_math_code(code).expect("Parse math code");
    let opt_prog = prog.optimize_tensor_graph();

    let mut eval = MathEvaluator::new();
    eval.execute_program(&opt_prog).expect("Execute optimized program");

    let z_val = eval.env.get("z").expect("z missing");
    let z_scalar = z_val.as_scalar().expect("Expected scalar for z");
    assert_eq!(z_scalar, 10.0, "z should fold cleanly to 10.0");
}
