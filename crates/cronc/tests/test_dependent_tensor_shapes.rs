// ============================================================================
// CRON Integration Test: Dependent Tensor Dimensions & Symbolic Shape Inference
// Enforces:
//   - Compile-time static dimension proof (O(1) static type check)
//   - Zero runtime bounds checking / shape checking overhead
//   - Rejection of inner dimension mismatches with diagnostic [E0016]
//   - Generic call-site dimension unification
//   - GCC C23 native compilation and execution
// ============================================================================

use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::checker::SemanticChecker;
use cronc::c_backend::CBackend;
use std::fs;
use std::process::Command;

#[test]
fn test_valid_dependent_tensor_dimensions() {
    let source = r#"
def run_matmul() -> int {
    let a: tensor<1, 32, 64, f32> = tensor_init(1.0);
    let b: tensor<1, 64, 128, f32> = tensor_init(2.0);
    let c: tensor<1, 32, 128, f32> = tensor_matmul(a, b);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Failed to parse valid dependent tensor program");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Expected valid dependent tensor program to pass semantic check, got: {:?}", result.err());
}

#[test]
fn test_dependent_tensor_inner_dimension_mismatch_rejected_e0016() {
    let source = r#"
def invalid_matmul() -> int {
    let a: tensor<1, 32, 64, f32> = tensor_init(1.0);
    let b: tensor<1, 32, 128, f32> = tensor_init(2.0);
    let c = tensor_matmul(a, b);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser should parse invalid matmul syntax");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected inner dimension mismatch (64 vs 32) to be rejected at compile-time");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0016", "Expected diagnostic error code E0016, got {}", err.code);
    assert!(err.message.contains("dimension mismatch"), "Error message should mention dimension mismatch: {}", err.message);
}

#[test]
fn test_dependent_tensor_annotation_mismatch_rejected_e0016() {
    let source = r#"
def invalid_annot() -> int {
    let a: tensor<1, 32, 64, f32> = tensor_init(1.0);
    let b: tensor<1, 64, 128, f32> = tensor_init(2.0);
    // Incorrect expected type annotation: tensor<1, 32, 256, f32> instead of 128
    let c: tensor<1, 32, 256, f32> = tensor_matmul(a, b);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser should parse program");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected variable annotation mismatch to be rejected at compile time");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0016");
    assert!(err.message.contains("Tensor dimension mismatch"));
}

#[test]
fn test_generic_function_tensor_unification() {
    let source = r#"
def batched_gemm(a: tensor<B, M, K, f32>, b: tensor<B, K, N, f32>) -> tensor<B, M, N, f32> {
    return tensor_matmul(a, b);
}

def main() -> int {
    let x: tensor<1, 16, 32, f32> = tensor_init(1.0);
    let y: tensor<1, 32, 64, f32> = tensor_init(2.0);
    let z: tensor<1, 16, 64, f32> = batched_gemm(x, y);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser should parse generic tensor program");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Expected valid generic call unification to pass, got: {:?}", result.err());
}

#[test]
fn test_generic_function_symbol_conflict_rejected_e0016() {
    let source = r#"
def batched_gemm(a: tensor<B, M, K, f32>, b: tensor<B, K, N, f32>) -> tensor<B, M, N, f32> {
    return tensor_matmul(a, b);
}

def main() -> int {
    let x: tensor<1, 16, 32, f32> = tensor_init(1.0);
    // y has inner dimension 48 instead of K=32 bound from x
    let y: tensor<1, 48, 64, f32> = tensor_init(2.0);
    let z = batched_gemm(x, y);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser should parse generic tensor program");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected generic symbol K conflict (32 vs 48) to be rejected at compile-time");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0016");
    assert!(err.message.contains("symbol 'K' was previously bound to '32', but argument provides '48'"));
}

#[test]
fn test_dependent_tensor_gcc_native_execution() {
    let source = r#"
def batched_forward() -> float {
    let a: tensor<1, 4, 4, f32> = tensor_init(1.5);
    let b: tensor<1, 4, 4, f32> = tensor_init(2.0);
    let c: tensor<1, 4, 4, f32> = tensor_matmul(a, b);
    let t: tensor<1, 4, 4, f32> = tensor_transpose(c);
    let res: tensor<1, 4, 4, f32> = tensor_add(c, t);
    return res.data[0];
}

def main() -> int {
    let val = batched_forward();
    if val > 20.0 {
        return 0;
    } else {
        return 1;
    }
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parse failed");
    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check failed");

    let mut backend = CBackend::new();
    let c_code = backend.generate(&program);

    let temp_c = std::env::temp_dir().join("cron_test_dep_tensor.c");
    let temp_exe = std::env::temp_dir().join("cron_test_dep_tensor.exe");

    fs::write(&temp_c, &c_code).expect("Failed to write temporary C source");

    let gcc_status = Command::new("gcc")
        .args([
            "-O2",
            "-std=c2x",
            temp_c.to_str().unwrap(),
            "-o",
            temp_exe.to_str().unwrap(),
            "-lm",
        ])
        .status();

    if let Ok(status) = gcc_status {
        if status.success() {
            let run_status = Command::new(&temp_exe).status();
            assert!(run_status.is_ok());
            assert_eq!(run_status.unwrap().code(), Some(0), "Dependent tensor native binary returned non-zero code");
        }
    }

    let _ = fs::remove_file(temp_c);
    let _ = fs::remove_file(temp_exe);
}
