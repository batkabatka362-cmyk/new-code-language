// ============================================================================
// CRON PHASE D1: NUMERICAL FINITE-DIFFERENCE & NUMERICAL STABILITY AUDIT
// Audits Analytical Reverse-Mode Autodiff Against Numerical Differentiation
// ============================================================================

use cronc::cl_math::{parse_math_code, MathEvaluator, MathValue};

/// Helper to compute central finite-difference gradient for a target variable
fn compute_numerical_gradient(
    eval: &mut MathEvaluator,
    var_name: &str,
    loss_expr_str: &str,
    eps: f64,
) -> Vec<f64> {
    let orig_val = eval
        .env
        .get(var_name)
        .cloned()
        .expect("Variable must exist in environment");
    let elements = orig_val.flatten_elements();
    let num_elements = elements.len();
    let shape = orig_val.shape();

    let loss_prog = parse_math_code(loss_expr_str).expect("Loss expression parse failed");
    let mut num_grads = Vec::with_capacity(num_elements);

    for i in 0..num_elements {
        // 1. Evaluate f(x + eps)
        let mut plus_elements = elements.clone();
        plus_elements[i] += eps;
        eval.env.insert(
            var_name.to_string(),
            reconstruct_math_value(&plus_elements, &shape),
        );
        let f_plus = eval
            .execute_program_value(&loss_prog)
            .expect("f_plus evaluation failed")
            .expect("Loss must return a value")
            .as_scalar()
            .expect("Loss must evaluate to a scalar");

        // 2. Evaluate f(x - eps)
        let mut minus_elements = elements.clone();
        minus_elements[i] -= eps;
        eval.env.insert(
            var_name.to_string(),
            reconstruct_math_value(&minus_elements, &shape),
        );
        let f_minus = eval
            .execute_program_value(&loss_prog)
            .expect("f_minus evaluation failed")
            .expect("Loss must return a value")
            .as_scalar()
            .expect("Loss must evaluate to a scalar");

        // 3. Central difference: (f(x+eps) - f(x-eps)) / (2 * eps)
        let grad_i = (f_plus - f_minus) / (2.0 * eps);
        num_grads.push(grad_i);
    }

    // Restore original variable in environment
    eval.env.insert(var_name.to_string(), orig_val);
    num_grads
}

fn reconstruct_math_value(elements: &[f64], shape: &[usize]) -> MathValue {
    match shape.len() {
        0 => MathValue::Scalar(elements.first().copied().unwrap_or(0.0)),
        1 => MathValue::Vector(elements.to_vec()),
        2 => {
            let rows = shape[0];
            let cols = shape[1];
            let mut mat = Vec::with_capacity(rows);
            for r in 0..rows {
                let start = r * cols;
                let end = (start + cols).min(elements.len());
                mat.push(elements[start..end].to_vec());
            }
            MathValue::Matrix(mat)
        }
        _ => MathValue::new_tensor(shape.to_vec(), elements.to_vec()).unwrap(),
    }
}

// ----------------------------------------------------------------------------
// TEST 1: Matrix Multiplication Gradient Audit (Y = X @ W)
// ----------------------------------------------------------------------------
#[test]
fn test_audit_matmul_analytical_vs_numerical_gradient() {
    let setup_code = r#"
        X = [[1.5, -2.0], [0.5, 3.0]]
        W = [[0.8, -1.2], [2.1, 0.4]]
    "#;
    let setup_prog = parse_math_code(setup_code).unwrap();
    let mut eval = MathEvaluator::new();
    eval.execute_program(&setup_prog).unwrap();

    // Loss = sum(X @ W)
    let loss_code = "sum(X @ W)";
    let eps = 1e-5;

    // Numerical gradients
    let num_grad_x = compute_numerical_gradient(&mut eval, "X", loss_code, eps);
    let num_grad_w = compute_numerical_gradient(&mut eval, "W", loss_code, eps);

    // Analytical gradients via ∇
    let grad_prog = parse_math_code("gX = ∇(sum(X @ W), X); gW = ∇(sum(X @ W), W)").unwrap();
    eval.execute_program(&grad_prog).unwrap();

    let an_grad_x = eval.env.get("gX").unwrap().flatten_elements();
    let an_grad_w = eval.env.get("gW").unwrap().flatten_elements();

    for i in 0..num_grad_x.len() {
        let diff = (an_grad_x[i] - num_grad_x[i]).abs();
        assert!(
            diff < 1e-4,
            "MatMul ∇_X mismatch at {}: analytical={}, numerical={}, diff={}",
            i, an_grad_x[i], num_grad_x[i], diff
        );
    }

    for i in 0..num_grad_w.len() {
        let diff = (an_grad_w[i] - num_grad_w[i]).abs();
        assert!(
            diff < 1e-4,
            "MatMul ∇_W mismatch at {}: analytical={}, numerical={}, diff={}",
            i, an_grad_w[i], num_grad_w[i], diff
        );
    }
}

// ----------------------------------------------------------------------------
// TEST 2: RMSNorm Gradient Audit (N(X, gamma))
// ----------------------------------------------------------------------------
#[test]
fn test_audit_rmsnorm_analytical_vs_numerical_gradient() {
    let setup_code = r#"
        X = [[1.2, 0.8, -1.5, 0.4], [0.3, -0.9, 1.1, 2.0]]
        γ = [1.0, 1.0, 1.0, 1.0]
    "#;
    let setup_prog = parse_math_code(setup_code).unwrap();
    let mut eval = MathEvaluator::new();
    eval.execute_program(&setup_prog).unwrap();

    let loss_code = "sum(rmsnorm(X, γ))";
    let eps = 1e-5;

    let num_grad_x = compute_numerical_gradient(&mut eval, "X", loss_code, eps);

    let grad_prog = parse_math_code("gX = ∇(sum(rmsnorm(X, γ)), X)").unwrap();
    eval.execute_program(&grad_prog).unwrap();

    let an_grad_x = eval.env.get("gX").unwrap().flatten_elements();

    for i in 0..num_grad_x.len() {
        let diff = (an_grad_x[i] - num_grad_x[i]).abs();
        assert!(
            diff < 1e-4,
            "RMSNorm ∇_X mismatch at {}: analytical={}, numerical={}, diff={}",
            i, an_grad_x[i], num_grad_x[i], diff
        );
    }
}

// ----------------------------------------------------------------------------
// TEST 3: Mamba-2 State-Space Duality (SSD) Recurrent Scan Gradient Audit
// ----------------------------------------------------------------------------
#[test]
fn test_audit_mamba2_ssd_scan_analytical_vs_numerical_gradient() {
    let setup_code = r#"
        X = [[1.0, -0.5], [0.8, 1.2], [-0.3, 0.7]]
        decay_dt = [0.85, 0.9]
    "#;
    let setup_prog = parse_math_code(setup_code).unwrap();
    let mut eval = MathEvaluator::new();
    eval.execute_program(&setup_prog).unwrap();

    let loss_code = "sum(M(X, decay_dt))";
    let eps = 1e-5;

    let num_grad_x = compute_numerical_gradient(&mut eval, "X", loss_code, eps);

    let grad_prog = parse_math_code("gX = ∇(sum(M(X, decay_dt)), X)").unwrap();
    eval.execute_program(&grad_prog).unwrap();

    let an_grad_x = eval.env.get("gX").unwrap().flatten_elements();

    for i in 0..num_grad_x.len() {
        let diff = (an_grad_x[i] - num_grad_x[i]).abs();
        assert!(
            diff < 1e-4,
            "Mamba-2 SSD ∇_X mismatch at {}: analytical={}, numerical={}, diff={}",
            i, an_grad_x[i], num_grad_x[i], diff
        );
    }
}

// ----------------------------------------------------------------------------
// TEST 4: GRPO Reasoning Policy Loss Gradient Audit (Λ_g)
// ----------------------------------------------------------------------------
#[test]
fn test_audit_grpo_loss_analytical_vs_numerical_gradient() {
    let setup_code = r#"
        logp = [-0.5, -1.2, -0.8, -0.3]
        ref = [-0.6, -1.0, -0.9, -0.4]
        adv = [1.5, -0.8, 0.5, -1.2]
    "#;
    let setup_prog = parse_math_code(setup_code).unwrap();
    let mut eval = MathEvaluator::new();
    eval.execute_program(&setup_prog).unwrap();

    let loss_code = "Λ_g(logp, ref, adv, 0.2, 0.04)";
    let eps = 1e-5;

    let num_grad_logp = compute_numerical_gradient(&mut eval, "logp", loss_code, eps);

    let grad_prog = parse_math_code("g_logp = ∇(Λ_g(logp, ref, adv, 0.2, 0.04), logp)").unwrap();
    eval.execute_program(&grad_prog).unwrap();

    let an_grad_logp = eval.env.get("g_logp").unwrap().flatten_elements();

    for i in 0..num_grad_logp.len() {
        let diff = (an_grad_logp[i] - num_grad_logp[i]).abs();
        assert!(
            diff < 1e-4,
            "GRPO ∇_logp mismatch at {}: analytical={}, numerical={}, diff={}",
            i, an_grad_logp[i], num_grad_logp[i], diff
        );
    }
}

// ----------------------------------------------------------------------------
// TEST 5: Mixture of Experts (MoE) Routing Gradient Audit (Ξ_m)
// ----------------------------------------------------------------------------
#[test]
fn test_audit_moe_routing_analytical_vs_numerical_gradient() {
    let setup_code = r#"
        E0 = [[1.0, 0.2], [-0.1, 0.8]]
        E1 = [[0.5, -0.3], [0.4, 1.2]]
        experts = (E0, E1)
        W_gate = [[1.0, -0.5], [-0.5, 1.0]]
        X = [0.8, 0.6]
    "#;
    let setup_prog = parse_math_code(setup_code).unwrap();
    let mut eval = MathEvaluator::new();
    eval.execute_program(&setup_prog).unwrap();

    let loss_code = "sum(Ξ_m(X, W_gate, experts, 2))";
    let eps = 1e-5;

    let num_grad_x = compute_numerical_gradient(&mut eval, "X", loss_code, eps);

    let grad_prog = parse_math_code("gX = ∇(sum(Ξ_m(X, W_gate, experts, 2)), X)").unwrap();
    eval.execute_program(&grad_prog).unwrap();

    let an_grad_x = eval.env.get("gX").unwrap().flatten_elements();

    for i in 0..num_grad_x.len() {
        let diff = (an_grad_x[i] - num_grad_x[i]).abs();
        assert!(
            diff < 1e-4,
            "MoE ∇_X mismatch at {}: analytical={}, numerical={}, diff={}",
            i, an_grad_x[i], num_grad_x[i], diff
        );
    }
}

// ----------------------------------------------------------------------------
// TEST 6: Adversarial Numerical Stability & Catastrophic Cancellation Audit
// ----------------------------------------------------------------------------
#[test]
fn test_audit_adversarial_neumaier_sum_and_flash_softmax() {
    // 1. Neumaier Compensated Summation: Catastrophic Cancellation Test
    // In standard IEEE-754 FP64: 1.0 + 1e16 - 1e16 + 1.0 collapses to 0.0 or 1.0 due to mantissa overflow.
    // Neumaier compensated summation must capture and preserve the exact 2.0 result!
    let neumaier_code = r#"
        adversarial_vec = [1.0, 1e16, 1.0, -1e16]
        res = ∑_k(adversarial_vec)
    "#;
    let prog = parse_math_code(neumaier_code).unwrap();
    let mut eval = MathEvaluator::new();
    eval.execute_program(&prog).unwrap();

    let res = eval.env.get("res").unwrap().as_scalar().unwrap();
    assert_eq!(
        res, 2.0,
        "Neumaier summation must produce exactly 2.0 on catastrophic cancellation, got {}",
        res
    );

    // 2. FlashAttention Online Softmax Large Logits / Extreme Values
    // Standard naive softmax(10000.0) produces exp(10000.0) -> +inf -> NaN.
    // FlashAttention-2 online softmax subtracts running max and must remain 100% NaN-free!
    let flash_code = r#"
        Q = [[10000.0, 0.0]]
        K = [[10000.0, 0.0]]
        V = [[42.0, 99.0]]
        out = Ψ(Q, K, V, 1.0)
    "#;
    let flash_prog = parse_math_code(flash_code).unwrap();
    eval.execute_program(&flash_prog).unwrap();

    let out_val = eval.env.get("out").unwrap().flatten_elements();
    assert!(
        !out_val[0].is_nan() && !out_val[0].is_infinite(),
        "FlashAttention output must not be NaN or Inf with huge logits: got {}",
        out_val[0]
    );
    assert!(
        (out_val[0] - 42.0).abs() < 1e-4,
        "FlashAttention expected 42.0, got {}",
        out_val[0]
    );
    assert!(
        (out_val[1] - 99.0).abs() < 1e-4,
        "FlashAttention expected 99.0, got {}",
        out_val[1]
    );
}

