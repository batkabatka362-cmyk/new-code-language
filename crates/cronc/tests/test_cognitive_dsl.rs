use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::checker::SemanticChecker;
use cronc::codegen::Codegen;

#[test]
fn test_cognitive_dsl_brain_syntax() {
    let source = r#"
.MODULE CognitiveTest_V88
.ENTRY _main

brain Imagination {
    let reality = 100
    fork reality
    let step = 1
    let action = 2
    simulate step with action
    let risk = 80
    if risk > 64 then abort
}

_main:
    brain MetacognitiveSentry [thermal_thresh=160] {
        let sensor = 42
        fork sensor
        simulate sensor
        if sensor == 0 then abort
    }
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    assert_eq!(program.brains.len(), 2);
    assert_eq!(program.brains[0].name, "Imagination");
    assert_eq!(program.brains[0].body.len(), 7);
    assert_eq!(program.brains[1].name, "MetacognitiveSentry");

    // Semantic checking
    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check should pass");

    // Codegen emission
    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);

    assert!(!cl_output.is_empty());
    // Should contain Sentry watchdog slot for brain
    assert!(cl_output.contains("_SH"));
    // Should contain Superposition slot for fork
    assert!(cl_output.contains("_SW"));
    // Should contain Prediction simulation slot
    assert!(cl_output.contains("_PS"));
    // Should contain Abort / Halt slot
    assert!(cl_output.contains("_HL"));
}

#[test]
fn test_cognitive_dsl_page_320_verbatim() {
    let source = r#"
.MODULE Page320_CognitiveDSL
.ENTRY _main

brain Imagination {
    fork 1
    simulate 2 with 3
    if 100 > 64 then abort
}

_main:
    let lin token = 99
    let res = consume(token)
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check should pass");

    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);
    assert!(cl_output.contains("B0001:"));
}
