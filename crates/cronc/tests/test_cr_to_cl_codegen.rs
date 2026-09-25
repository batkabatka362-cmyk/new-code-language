use cronc::codegen::Codegen;
use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::cl_lang;

#[test]
fn test_cr_to_cl_sagi_lowering() {
    let source = r#"
    fn test_sagi_pipeline() -> i64 {
        let rotor_res = optical_clifford_rotate(10);
        let bcm_res = step_bcm_weight(5);
        let quench = evaluate_quench(1);
        let msg = torus_dor_broadcast(42);
        let trit_acc = trit_zero_mul_accumulate(1, 100, 0);
        let pred_w = step_predictive_synapse(10, 2, 5, 64);
        pred_w
    }
    "#;

    let tokens = Lexer::new(source).tokenize().expect("Lexing failed");
    let program = Parser::new(tokens).parse_program().expect("Parsing failed");

    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);

    assert!(!cl_output.is_empty(), "Generated .cl assembly must not be empty");
    assert!(cl_output.contains("B0001:"), "Must contain cycle 1 bundle");

    // Verify all bundles and slots parse with cl_lang without syntax errors
    let lines: Vec<&str> = cl_output.lines().collect();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('B') || trimmed.starts_with('b') {
            if let Some((_b_prefix, slots_part)) = trimmed.split_once(':') {
                for slot_str in slots_part.split_whitespace() {
                    let slot = cl_lang::parse_slot(slot_str);
                    assert!(slot.is_ok(), "Slot {} in line '{}' must parse cleanly: {:?}", slot_str, line, slot.err());
                }
            }
        }
    }
}

#[test]
fn test_cr_to_cl_predictive_and_ternary_expressions() {
    let source = r#"
    fn compute_brain_pass() -> i64 {
        let e = evaluate_local_error(10);
        let w = step_predictive_synapse(100, e, 4, 32);
        let acc = trit_zero_mul_accumulate(1, 50, 0);
        w + acc
    }
    "#;

    let tokens = Lexer::new(source).tokenize().expect("Lexing failed");
    let program = Parser::new(tokens).parse_program().expect("Parsing failed");

    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);

    assert!(cl_output.contains("B0001:"));
}
