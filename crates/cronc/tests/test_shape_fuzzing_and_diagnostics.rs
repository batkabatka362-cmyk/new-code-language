// ============================================================================
// CRON Systematic Debugging & Verification: Phase D4
// Shape Checker Fuzzing, Unicode Parsing & Circular Graph Diagnostics
//
// Verifies:
//   1. Shape Checker Fuzzing: Dimension mismatches rejected with clean diagnostics (0 panics)
//   2. Unicode & Greek Calculus Parsing: Robust lexing of ∇_W, ∑_k, Ψ_g, Ξ_m, M
//   3. Circular Dependency Graph Detection: Clean cycle diagnostic errors (A -> B -> A)
// ============================================================================

use cronc::cl_math::{parse_math_code, MathEvaluator, MathValue};

#[test]
fn test_shape_checker_fuzzing_matrix_mismatch() {
    // 1. Matrix multiplication inner dimension mismatch: [16, 32] @ [64, 16]
    let cl_mismatch = r#"
F(A: f32[16, 32], B: f32[64, 16]) = A @ B
"#;

    let prog = parse_math_code(cl_mismatch).expect("Parsing function signature should succeed");
    let shape_res = prog.verify_shapes();
    assert!(shape_res.is_err(), "verify_shapes must detect dimension mismatch");
    let errs = shape_res.unwrap_err();
    let combined_err = errs.join("; ");
    println!("Matrix mismatch detected cleanly: {}", combined_err);
    assert!(combined_err.contains("Matrix multiplication shape mismatch (@)"));
    assert!(combined_err.contains("inner dim 32"));
    assert!(combined_err.contains("outer dim 64"));
    println!("  -> [PASS] Shape checker rejected [16, 32] @ [64, 16] without panic.");
}

#[test]
fn test_shape_checker_gqa_head_divisibility_rejection() {
    // 2. GQA with non-divisible heads: H_q = 5, H_kv = 2
    let eval = MathEvaluator::new();
    let q = MathValue::Matrix(vec![vec![1.0; 20]; 2]); // seq=2, dim=20
    let k = MathValue::Matrix(vec![vec![1.0; 8]; 2]);
    let v = MathValue::Matrix(vec![vec![1.0; 8]; 2]);

    let res = eval.eval_gqa_attn(&q, &k, &v, 5, 2, None, true);
    assert!(res.is_err(), "eval_gqa_attn must reject H_q=5, H_kv=2");
    let err = res.unwrap_err();
    println!("GQA head mismatch detected cleanly: {}", err);
    assert!(err.contains("must be divisible by n_kv_heads"));
    println!("  -> [PASS] GQA checker rejected H_q=5, H_kv=2 with clean error.");
}

#[test]
fn test_unicode_and_subscript_parsing_fuzzing() {
    // 3. Dense Unicode mathematical symbols and subscripts
    let cl_unicode = r#"
loss(X, W, γ, α) = ∇_W(X @ W) + ∑_k(γ) + Ψ_g(X, X, X, 4, 2) + Ξ_m(X, W, 2) + M(X, α)
"#;

    let prog_res = parse_math_code(cl_unicode);
    assert!(prog_res.is_ok(), "Parser must cleanly handle dense Greek and subscript tokens: {:?}", prog_res.err());
    let prog = prog_res.unwrap();
    assert_eq!(prog.items.len(), 1);
    println!("  -> [PASS] Unicode tokens (∇_W, ∑_k, Ψ_g, Ξ_m, M, γ, α) parsed flawlessly.");
}

#[test]
fn test_circular_dependency_graph_detection() {
    // 4. Circular graph: A depends on B, B depends on C, C depends on A
    let cl_circular = r#"
A = B + 1.0
B = C * 2.0
C = A - 3.0
"#;

    let prog = parse_math_code(cl_circular).expect("Parser should parse expressions");
    let cycle_res = prog.check_circular_dependencies();
    assert!(cycle_res.is_err(), "check_circular_dependencies must catch 3-cycle");
    let cycle_err = cycle_res.unwrap_err();
    println!("Cycle detected cleanly: {}", cycle_err);
    assert!(cycle_err.contains("Circular dependency detected in computational graph"));
    println!("  -> [PASS] Circular graph A -> B -> C -> A detected without recursion blowup.");

    // 5. Valid Acyclic DAG: X -> Y -> Z
    let cl_acyclic = r#"
X = 1.0
Y = X + 2.0
Z = Y * 3.0
"#;
    let prog_ok = parse_math_code(cl_acyclic).expect("Parser should parse DAG");
    assert!(prog_ok.check_circular_dependencies().is_ok(), "Acyclic DAG must pass cycle check");
    println!("  -> [PASS] Acyclic DAG validated successfully.");
}

