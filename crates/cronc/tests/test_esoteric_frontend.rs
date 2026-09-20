use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::checker::SemanticChecker;
use cronc::codegen::Codegen;

#[test]
fn test_tape_ring_buffer_parsing_and_codegen() {
    let source = r#"
.MODULE TapeTest
.ENTRY _main

def _main() -> i32 {
    tape kv_cache: RingTape[Float16, 512]
    let token: u32 = 99
    kv_cache << token
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check should pass");

    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);

    // Verify Brainfuck Tape Pointer Init and Write instructions in emitted .cl
    assert!(cl_output.contains("TI"), "Expected tape init opcode TI");
    assert!(cl_output.contains("TW"), "Expected tape write opcode TW");
}

#[test]
fn test_inline_vliw_asm_embedding() {
    let source = r#"
.MODULE InlineVliwTest
.ENTRY _main

def _main() -> i32 {
    __vliw_asm__ {
        "B0000: '==01#00A> _OP01$28F> _NO00#000> _NO00#000>",
        "B0001: _TW00#100> _NO00#000> _NO00#000> _HL00#000!"
    }
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check should pass");

    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);

    assert!(cl_output.contains("'=="), "Expected ternary constant slot");
    assert!(cl_output.contains("_OP"), "Expected optical MZI slot");
    assert!(cl_output.contains("_TW"), "Expected tape write slot");
}

#[test]
fn test_systolic_wavefront_block() {
    let source = r#"
.MODULE SystolicTest
.ENTRY _main

def _main() -> i32 {
    @systolic(mesh: [8, 8], topology: Torus4D)
    block wavefront {
        flow A -> EAST;
        flow B -> SOUTH;
        let acc: u32 = 42
    }
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check should pass");

    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);

    // Directional routing slots for Befunge systolic execution
    assert!(cl_output.contains("DE"), "Expected East routing DE opcode");
    assert!(cl_output.contains("DS"), "Expected South routing DS opcode");
}

#[test]
fn test_prolog_rule_declaration() {
    let source = r#"
.MODULE PrologRuleTest
.ENTRY _main

rule attention_causal(i, j) :-
    i <= j;

def _main() -> i32 {
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check should pass");

    // Codegen emission includes unification slot
    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);
    assert!(cl_output.contains("UN"), "Expected Unification UN opcode");
}

#[test]
fn test_c23_transpilation_esoteric() {
    let source = r#"
.MODULE C23EsotericTest
.ENTRY _main

rule test_rule(a, b) :- a < b;

def _main() -> i32 {
    tape my_tape: RingTape[u32, 128]
    let val: u32 = 77
    my_tape << val

    @systolic(mesh: [4, 4], topology: Torus4D)
    block flow_core {
        flow In -> NORTH;
        let step: u32 = 1
    }

    __vliw_asm__ {
        "B0000: _NO00#000> _NO00#000> _NO00#000> _HL00#000!"
    }

    return 0
}
"#;

    let c_code = cronc::compile_to_c23(source).expect("Transpilation should succeed");

    assert!(c_code.contains("uint32_t my_tape_buf[128] = {0};"));
    assert!(c_code.contains("size_t my_tape_ptr = 0;"));
    assert!(c_code.contains("my_tape_buf[my_tape_ptr] = val;"));
    assert!(c_code.contains("// [CRON Befunge Systolic Wavefront Block]"));
    assert!(c_code.contains("// [CRON Prolog Neuro-Symbolic Rule: test_rule]"));
    assert!(c_code.contains("// [CRON VLIW Inline Asm Block]"));
}

#[test]
fn test_esoteric_semantic_errors() {
    // Tape capacity must be > 0
    let invalid_tape_source = r#"
.MODULE ErrTest
.ENTRY _main

def _main() -> i32 {
    tape bad_tape: RingTape[u32, 0]
    return 0
}
"#;

    let mut lexer = Lexer::new(invalid_tape_source);
    let tokens = lexer.tokenize().expect("Lexing should succeed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut checker = SemanticChecker::new();
    let err = checker.check_program(&program);
    assert!(err.is_err(), "Zero capacity tape should fail semantic check");
    let msg = format!("{:?}", err.unwrap_err());
    assert!(msg.contains("E0022") || msg.contains("Capacity must be > 0"));
}
