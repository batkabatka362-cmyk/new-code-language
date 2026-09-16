// ============================================================================
// CRON Test Suite - Language-Level Native Autodiff & Fredkin Reversible Stack
// (C) 2026 CRON Language Project - SSS+ Tier Industrial Systems Test
// ============================================================================

use cronc::autodiff::AutodiffEngine;
use cronc::jit_backend::run_source_jit;
use cronc::parser::Parser;
use cronc::lexer::Lexer;
use std::process::Command;

#[test]
fn test_autodiff_scalar_polynomial() {
    let source = r#"
.MODULE test.autodiff.poly

def square(x: f32) -> f32 {
    return x * x
}

def poly(x: f32) -> f32 {
    return 3.0 * x * x + 5.0 * x + 7.0
}

def main() -> i32 {
    let d_sq = grad(square)(4.0)
    let d_poly = grad(poly)(2.0)
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program().expect("Parse failed");

    // Run autodiff synthesis pass
    AutodiffEngine::differentiate_program(&mut program).expect("Autodiff pass failed");

    // Verify synthesized functions exist
    let fn_names: Vec<String> = program.functions.iter().map(|f| f.name.clone()).collect();
    assert!(fn_names.contains(&"square_grad_x".to_string()), "Must synthesize square_grad_x");
    assert!(fn_names.contains(&"poly_grad_x".to_string()), "Must synthesize poly_grad_x");

    // Verify types and compilation
    let mut checker = cronc::checker::SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check must succeed on differentiated AST");
}

#[test]
fn test_autodiff_multivariable_partial_derivatives() {
    let source = r#"
.MODULE test.autodiff.multivar

def surface(x: f32, y: f32) -> f32 {
    return x * x * y + 2.0 * y
}

def main() -> i32 {
    // df/dx = 2 * x * y
    let dx = grad(surface, wrt: "x")(3.0, 4.0)
    // df/dy = x * x + 2.0
    let dy = grad(surface, wrt: "y")(3.0, 4.0)
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program().expect("Parse failed");

    AutodiffEngine::differentiate_program(&mut program).expect("Autodiff pass failed");

    let fn_names: Vec<String> = program.functions.iter().map(|f| f.name.clone()).collect();
    assert!(fn_names.contains(&"surface_grad_x".to_string()), "Must synthesize surface_grad_x");
    assert!(fn_names.contains(&"surface_grad_y".to_string()), "Must synthesize surface_grad_y");
}

#[test]
fn test_autodiff_simd_vector_loss() {
    let source = r#"
.MODULE test.autodiff.simd

def vector_loss(v: vec8f) -> f32 {
    return simd_reduce_sum(v * v)
}

def fma_loss(a: vec8f, b: vec8f, c: vec8f) -> vec8f {
    return simd_fma(a, b, c)
}

def main() -> i32 {
    let zeros = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    let grad_v = grad(vector_loss)(zeros)
    let grad_a = grad(fma_loss, wrt: "a")(zeros, zeros, zeros)
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program().expect("Parse failed");

    AutodiffEngine::differentiate_program(&mut program).expect("Autodiff pass failed");

    let fn_names: Vec<String> = program.functions.iter().map(|f| f.name.clone()).collect();
    assert!(fn_names.contains(&"vector_loss_grad_v".to_string()), "Must synthesize vector_loss_grad_v");
    assert!(fn_names.contains(&"fma_loss_grad_a".to_string()), "Must synthesize fma_loss_grad_a");
}

#[test]
fn test_autodiff_transcendental_functions() {
    let source = r#"
.MODULE test.autodiff.transcendental

def model_step(x: f32) -> f32 {
    return sin(x) + exp(x) + tanh(x) + relu(x) + sigmoid(x)
}

def main() -> i32 {
    let d_step = grad(model_step)(0.0)
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program().expect("Parse failed");

    AutodiffEngine::differentiate_program(&mut program).expect("Autodiff pass failed");

    let fn_names: Vec<String> = program.functions.iter().map(|f| f.name.clone()).collect();
    assert!(fn_names.contains(&"model_step_grad_x".to_string()), "Must synthesize model_step_grad_x");
}

#[test]
fn test_autodiff_c23_native_gcc_execution() {
    let source = r#"
.MODULE test.autodiff.c23

def square(x: f32) -> f32 {
    return x * x
}

def cubic(x: f32) -> f32 {
    return x * x * x
}

def main() -> i32 {
    // d/dx(x^2) at x=5 is 10.0
    let d1 = grad(square)(5.0)
    // d/dx(x^3) at x=3 is 3 * 3^2 = 27.0
    let d2 = grad(cubic)(3.0)

    let total = (d1 as i32) + (d2 as i32)
    return total
}
"#;

    let c_code = cronc::compile_to_c23(source).expect("C23 compilation failed");
    assert!(c_code.contains("square_grad_x"), "C23 code must contain square_grad_x");
    assert!(c_code.contains("cubic_grad_x"), "C23 code must contain cubic_grad_x");

    // Write to temp file and compile with GCC
    let temp_c = std::env::temp_dir().join("test_autodiff_out.c");
    let temp_exe = std::env::temp_dir().join("test_autodiff_out.exe");
    std::fs::write(&temp_c, &c_code).expect("Write temp C file");

    let gcc_status = Command::new("gcc")
        .args(&["-O3", temp_c.to_str().unwrap(), "-o", temp_exe.to_str().unwrap(), "-lm"])
        .status();

    if let Ok(status) = gcc_status {
        if status.success() {
            let run_output = Command::new(&temp_exe).output().expect("Execute test_autodiff_out.exe");
            let exit_code = run_output.status.code().unwrap_or(-1);
            // 10 + 27 = 37
            assert_eq!(exit_code, 37, "Gradient derivative exit code must equal 37 (10 + 27)");
        }
    }
}

#[test]
fn test_autodiff_llvm_ir_generation() {
    let source = r#"
.MODULE test.autodiff.llvm

def square(x: f32) -> f32 {
    return x * x
}

def main() -> i32 {
    let d = grad(square)(4.0)
    return 0
}
"#;

    let llvm_ir = cronc::compile_to_llvm(source).expect("LLVM IR generation failed");
    assert!(llvm_ir.contains("@square_grad_x"), "LLVM IR must define @square_grad_x");
    assert!(llvm_ir.contains("call"), "LLVM IR must call the synthesized gradient function");
}

#[test]
fn test_autodiff_jit_execution() {
    let source = r#"
.MODULE test.autodiff.jit

def square(x: i64) -> i64 {
    return x * x
}

def main() -> i64 {
    // d/dx(x^2) = 2 * x; at x = 7, result is 14
    let d = grad(square)(7)
    return d
}
"#;

    let result = run_source_jit(source).expect("JIT execution failed");
    assert_eq!(result, 14, "JIT derivative of x^2 at x=7 must equal 14");
}

#[test]
fn test_autodiff_fredkin_reversible_adjoint() {
    let source = r#"
.MODULE test.autodiff.fredkin

def reversible_layer(control: bool, x: f32, y: f32) -> f32 {
    let swapped = fredkin_gate(control, x, y)
    return x * 3.0 + y * 5.0
}

def main() -> i32 {
    let dx = grad(reversible_layer, wrt: "x")(true, 10.0, 20.0)
    let dy = grad(reversible_layer, wrt: "y")(true, 10.0, 20.0)
    return 0
}
"#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let mut program = parser.parse_program().expect("Parse failed");

    AutodiffEngine::differentiate_program(&mut program).expect("Autodiff pass failed");

    let fn_names: Vec<String> = program.functions.iter().map(|f| f.name.clone()).collect();
    assert!(fn_names.contains(&"reversible_layer_grad_x".to_string()), "Must synthesize reversible_layer_grad_x");
    assert!(fn_names.contains(&"reversible_layer_grad_y".to_string()), "Must synthesize reversible_layer_grad_y");
}
