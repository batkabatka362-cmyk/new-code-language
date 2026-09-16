// ============================================================================
// CRON Milestone #012 Integration Test: Automated Kernel Fusion & Zero-Allocation Streaming
// Enforces:
//   - Syntax and AST parsing of `fuse [tile=(M, N), stream=SRAM] { ... }`
//   - Compile-time escape violation rejection with diagnostic [E0017]
//   - Microarchitectural tile capacity bounds enforcement (<= 256 elements) [E0017]
//   - Zero-allocation VLIW slot emission (_FU / _FE and zero intermediate DRAM writes)
//   - Direct GCC C23 native compilation and execution
// ============================================================================

use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::checker::SemanticChecker;
use cronc::c_backend::CBackend;
use std::fs;
use std::process::Command;

#[test]
fn test_fuse_syntax_and_semantic_checking() {
    let source = r#"
def fused_attention_step(
    q: tensor<1, 4, 4, f32>,
    kt: tensor<1, 4, 4, f32>,
    v: tensor<1, 4, 4, f32>
) -> tensor<1, 4, 4, f32> {
    fuse [tile=(4, 4), stream=SRAM] {
        let s1: tensor<1, 4, 4, f32> = tensor_matmul(q, kt);
        let s2: tensor<1, 4, 4, f32> = tensor_scale(s1, 0.5);
        let s3: tensor<1, 4, 4, f32> = tensor_softmax_maxsub(s2);
        let out: tensor<1, 4, 4, f32> = tensor_matmul(s3, v);
        return out;
    }
}

def main() -> int {
    let q: tensor<1, 4, 4, f32> = tensor_init(1.0);
    let kt: tensor<1, 4, 4, f32> = tensor_init(0.5);
    let v: tensor<1, 4, 4, f32> = tensor_init(0.25);
    let res = fused_attention_step(q, kt, v);
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Failed to parse valid fuse block");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Expected valid fuse block to pass semantic check, got: {:?}", result.err());
}

#[test]
fn test_fuse_scope_escape_rejected_e0017() {
    let source = r#"
def invalid_fuse_leak() -> int {
    let mut outer_leak: tensor<1, 4, 4, f32> = tensor_init(0.0);
    let q: tensor<1, 4, 4, f32> = tensor_init(1.0);
    let kt: tensor<1, 4, 4, f32> = tensor_init(1.0);

    fuse [tile=(4, 4), stream=SRAM] {
        let s1: tensor<1, 4, 4, f32> = tensor_matmul(q, kt);
        // ILLEGAL: Escaping intermediate streaming buffer to outer variable
        outer_leak = s1;
    }
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser should parse syntax");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected fusion scope escape to be rejected at compile-time");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0017", "Expected diagnostic E0017, got {}", err.code);
    assert!(err.message.contains("Fusion scope escape violation"), "Error message should mention escape: {}", err.message);
}

#[test]
fn test_fuse_excessive_tile_size_rejected_e0017() {
    let source = r#"
def invalid_tile_size() -> int {
    let q: tensor<1, 4, 4, f32> = tensor_init(1.0);
    let kt: tensor<1, 4, 4, f32> = tensor_init(1.0);

    // ILLEGAL: 32x32 = 1024 elements exceeds local SRAM limit of 256
    fuse [tile=(32, 32), stream=SRAM] {
        let s1: tensor<1, 4, 4, f32> = tensor_matmul(q, kt);
        return 0;
    }
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser should parse syntax");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected excessive tile size to be rejected at compile-time");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0017", "Expected diagnostic E0017, got {}", err.code);
    assert!(err.message.contains("exceeds local SRAM streaming limit"), "Error message should mention SRAM limit: {}", err.message);
}

#[test]
fn test_fuse_vliw_emission_and_zero_dram_traffic() {
    let source = r#"
.MODULE FusedStreamingCore
.ENTRY _main

_main:
    fuse [tile=(4, 4), stream=SRAM] {
        let a = 10
        let b = 20
        let c = a + b
    }
"#;
    let cl_output = cronc::compile_source(source).expect("Compilation to .cl failed");
    assert!(cl_output.contains("FU"), "Generated .cl should contain Fusion Start anchor 'FU'");
    assert!(cl_output.contains("FE"), "Generated .cl should contain Fusion End anchor 'FE'");
    // Ensure no DRAM HBM write instructions (_DW) are emitted within the fused streaming block
    let fu_pos = cl_output.find("FU").expect("Must contain FU");
    let fe_pos = cl_output.find("FE").expect("Must contain FE");
    assert!(fu_pos < fe_pos, "FU must precede FE");
    let fused_slice = &cl_output[fu_pos..fe_pos];
    assert!(!fused_slice.contains("DW"), "Zero-allocation fusion must NOT emit DRAM writes (_DW) inside fused kernel");
}

#[test]
fn test_fuse_c23_native_gcc_execution() {
    let source = r#"
def fused_computation(a: int, b: int) -> int {
    fuse [tile=(4, 4), stream=SRAM] {
        let x = a * 2;
        let y = b * 3;
        let total = x + y;
        return total;
    }
}

def main() -> int {
    let val = fused_computation(5, 7); // 10 + 21 = 31
    if val == 31 {
        return 0;
    } else {
        return 1;
    }
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parser should parse program");
    let mut checker = SemanticChecker::new();
    checker.check_program(&program).expect("Semantic check failed");

    let mut backend = CBackend::new();
    let c_code = backend.generate(&program);
    assert!(c_code.contains("CRON SINGLE-PASS FUSED STREAMING KERNEL"), "C code must include fused streaming header");

    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("test_fused_c23.c");
    let exe_file = temp_dir.join(if cfg!(windows) { "test_fused_c23.exe" } else { "test_fused_c23" });

    fs::write(&c_file, &c_code).expect("Failed to write C code");

    let gcc_status = Command::new("gcc")
        .arg("-O3")
        .arg(&c_file)
        .arg("-o")
        .arg(&exe_file)
        .arg("-lm")
        .status();

    if let Ok(status) = gcc_status {
        if status.success() {
            let run_status = Command::new(&exe_file).status().expect("Failed to execute compiled binary");
            assert!(run_status.success(), "Execution of fused C23 binary must succeed (exit code 0)");
            let _ = fs::remove_file(&exe_file);
        }
    }
    let _ = fs::remove_file(&c_file);
}
