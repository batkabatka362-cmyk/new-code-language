use cronc::cl_math::{parse_math_code, MathEvaluator, MathValue};
use cronc::cl_zero_verbal_lint::ZeroVerbalLinter;
use std::fs;
use std::path::Path;

#[test]
fn test_sovereign_zero_verbal_frontier_model() {
    let model_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/models/sovereign_zero_verbal_frontier.cl");
    let code = fs::read_to_string(&model_path).expect("Must read sovereign_zero_verbal_frontier.cl");

    // 1. Audit: Zero-Verbal Constitution Linter Verification
    let audit_report = ZeroVerbalLinter::audit_source(&code, "sovereign_zero_verbal_frontier.cl");
    println!("{}", audit_report.render_ascii_hud());
    assert!(
        audit_report.is_constitution_compliant,
        "Zero-verbal audit failed with {} violations: {:?}",
        audit_report.violations_count,
        audit_report.diagnostics
    );

    // 2. Parse into Language 1 AST
    let prog = parse_math_code(&code).expect("Parsing sovereign zero-verbal frontier model must succeed");

    // 3. Forward Evaluation via Language 1 Engine
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog).expect("Forward evaluation of frontier model must succeed");

    // Verify forward intermediate states and output
    assert!(eval.env.contains_key("x_1"), "x_1 attention state must exist");
    assert!(eval.env.contains_key("x_2"), "x_2 feedforward state must exist");
    assert!(eval.env.contains_key("L"), "Loss L must exist");

    let l_val = eval.env.get("L").expect("L must be computed");
    if let MathValue::Scalar(loss) = l_val {
        assert!(!loss.is_nan() && *loss >= 0.0, "Loss must be valid non-negative scalar: {}", loss);
    } else {
        panic!("Loss L must be scalar, got {:?}", l_val);
    }

    // 4. Low-Level Silicon VLIW Microcode Lowering (Language 2)
    let vliw_bundles = prog.to_vliw();
    let vliw_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/models/sovereign_zero_verbal_frontier_vliw.cl");
    fs::write(&vliw_path, &vliw_bundles).expect("Must write sovereign_zero_verbal_frontier_vliw.cl");
    println!("=== LANGUAGE 2: 128-BIT VLIW SILICON MICROCODE ===");
    for line in vliw_bundles.lines().take(12) {
        println!("{}", line);
    }
    assert!(!vliw_bundles.is_empty(), "VLIW microcode lowering must generate bundles");
    assert!(
        vliw_bundles.contains("_MD") || vliw_bundles.contains("_PO") || vliw_bundles.contains("_ST"),
        "VLIW bundles must contain tensor and ALU ops"
    );

    // 5. Standalone ISO C23 Lowering
    let c23_code = prog.to_c23();
    assert!(c23_code.contains("CRON_API"), "C23 code must contain CRON_API exports");
}

#[test]
fn test_arbitrary_modular_pipeline_composition_and_autodiff() {
    // Verifies that any arbitrary mathematical formula connects seamlessly via pipelines (->),
    // operator sections ((@ W), (+ b)), and placeholders (_) without rigid scripting.
    let code = r#"
        // 1. Modular morphism definitions
        N_0(z) = z -> Z -> M_01
        K_0(z, W_k) = T_B(z, 2) @ W_k
        A_0(z, W_g) = (z @ W_g) * σ(z @ W_g)

        // 2. Dynamic pipeline chaining of arbitrary morphisms
        x = [1.0, 2.0, 3.0]
        W_k = [[1.0, 0.0], [0.0, 1.0], [0.5, 0.5]]
        b_k = [0.05, -0.05]
        W_o = [[2.0], [1.0]]

        // Pipeline: input -> Norm -> Chebyshev KAN -> Activation -> Dense projection
        y_hat = x -> N_0 -> K_0(W_k) -> (+ b_k) -> m -> (@ W_o)

        // Target and loss
        y = [[0.5], [1.0], [1.5]]
        L = μ((y_hat - y)^2)

        // Reverse-mode analytical VJP autodiff through pipeline
        g_o = ∇_W_o(L)

        // Muon Newton-Schulz spectral descent optimization step
        W_1 = W_o - (0.001 * Ω_μ(g_o))
    "#;

    let audit = ZeroVerbalLinter::audit_source(code, "modular_pipeline.cl");
    println!("{}", audit.render_ascii_hud());
    assert!(audit.is_constitution_compliant, "Modular pipeline must be 100% zero-verbal compliant");

    let prog = parse_math_code(code).expect("Parsing arbitrary modular pipeline must succeed");
    let mut eval = MathEvaluator::new();
    eval.execute_program_value(&prog).expect("Evaluation of arbitrary pipeline must succeed");

    // Assert states exist and are valid
    let y_hat_val = eval.env.get("y_hat").expect("y_hat must be evaluated");
    assert!(matches!(y_hat_val, MathValue::Matrix(_)));
    let l_val = eval.env.get("L").expect("L must be evaluated");
    let g_val = eval.env.get("g_o").expect("g_o must be evaluated");
    let wo_val = eval.env.get("W_1").expect("W_1 must be evaluated");

    if let MathValue::Scalar(loss) = l_val {
        assert!(!loss.is_nan() && *loss >= 0.0, "Loss must be valid scalar: {}", loss);
    } else {
        panic!("Loss L must be scalar");
    }

    if let MathValue::Matrix(g_mat) = g_val {
        assert_eq!(g_mat.len(), 2);
        assert_eq!(g_mat[0].len(), 1);
        for row in g_mat {
            for v in row {
                assert!(!v.is_nan());
            }
        }
    } else {
        panic!("Gradient g_o must be 2x1 Matrix, got {:?}", g_val);
    }

    if let MathValue::Matrix(wo_mat) = wo_val {
        assert_eq!(wo_mat.len(), 2);
        assert_eq!(wo_mat[0].len(), 1);
        for row in wo_mat {
            for v in row {
                assert!(!v.is_nan());
            }
        }
    } else {
        panic!("W_1 must be 2x1 Matrix, got {:?}", wo_val);
    }

    // Level 2 VLIW Lowering
    let vliw = prog.to_vliw();
    assert!(!vliw.is_empty(), "Pipeline must lower to 128-bit VLIW microcode");
    assert!(vliw.contains("_MD") || vliw.contains("_PO") || vliw.contains("_TK"), "Must contain MMA, ALU, or KAN slots");
}
