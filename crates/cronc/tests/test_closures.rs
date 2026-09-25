use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::checker::SemanticChecker;
use cronc::codegen::Codegen;
use cronc::c_backend::CBackend;
use cronc::compile_source;

#[test]
fn test_closure_parsing_and_checking() {
    let source = r#"
def main() -> i32 {
    let add = |x: i32, y: i32| -> i32 {
        x + y
    };
    let double = |n: i32| n * 2;
    let get_constant = || 42;
    0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing failed");

    assert_eq!(program.functions.len(), 1);
    let main_fn = &program.functions[0];
    assert_eq!(main_fn.body.len(), 4);

    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic checking failed");

    let mut codegen = Codegen::new();
    let cl_output = codegen.generate(&program);
    assert!(!cl_output.is_empty(), "Codegen should emit VLIW bundles");

    let mut c_backend = CBackend::new();
    let c_code = c_backend.generate(&program);
    assert!(c_code.contains("[=]"), "C backend should transpile closures to lambdas");

    let compiled_cl = compile_source(source).expect("compile_source failed");
    assert!(!compiled_cl.is_empty(), "Full compilation should output .cl code");
}

#[test]
fn test_closure_ast_variants() {
    let source = r#"
def compute() -> i32 {
    let f = |a: i32| {
        a + 10
    };
    0
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut checker = SemanticChecker::new();
    assert!(checker.check_program(&program).is_ok());
}
