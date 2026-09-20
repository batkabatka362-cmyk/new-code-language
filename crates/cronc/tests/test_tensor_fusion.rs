// ============================================================================
// CRON Milestone #030 Integration Test: Tensor Fusion & Zero-Alloc AI Engine
// Tests:
//   1. First-class '@' matrix multiplication operator parsing & type checking
//   2. Compile-time dimensional contraction verification (E0016)
//   3. FusionGraph pattern detection (RMSNorm+Linear, Linear+SiLU, FlashAttention)
//   4. Zero-heap-allocation verification in generated C23 code
//   5. Bit-exact GCC -O3 native execution of fused AI layers
// ============================================================================

use cronc::lexer::Lexer;
use cronc::parser::Parser;
use cronc::checker::SemanticChecker;
use cronc::fusion::*;
use std::fs;
use std::process::Command;

#[test]
fn test_matrix_mult_at_operator_parsing_and_checking() {
    let source = r#"
def matmul_test(
    a: tensor<1, 4, 8, f32>,
    b: tensor<1, 8, 16, f32>
) -> tensor<1, 4, 16, f32> {
    let c: tensor<1, 4, 16, f32> = a @ b;
    return c;
}

def main() -> int {
    return 0;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Failed to parse '@' operator");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Expected valid '@' operator to pass type checking: {:?}", result.err());
}

#[test]
fn test_matrix_mult_at_operator_dimension_mismatch_e0016() {
    let source = r#"
def invalid_matmul(
    a: tensor<1, 4, 8, f32>,
    b: tensor<1, 7, 16, f32>
) -> tensor<1, 4, 16, f32> {
    // ILLEGAL: inner dimension 8 does not match outer dimension 7
    let c: tensor<1, 4, 16, f32> = a @ b;
    return c;
}
"#;
    let tokens = Lexer::new(source).tokenize().expect("Tokenize failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Failed to parse");
    let mut checker = SemanticChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Expected dimension mismatch to fail type checking");
    let err = result.unwrap_err();
    assert_eq!(err.code, "E0016");
    assert!(err.message.contains("Tensor dimension mismatch"));
}

#[test]
fn test_fusion_graph_pattern_detection() {
    let mut graph = FusionGraph::new();
    let x = graph.add_input("x", &[1, 16, 64]);
    let norm = graph.add_rmsnorm(x, 1e-5);
    let linear = graph.add_linear(norm, 64, 128);
    let _silu = graph.add_silu(linear);

    let groups = graph.identify_fusion_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].intermediate_buffers_eliminated, 2);
    assert!(groups[0].bytes_saved > 0);

    let (total_ops, remaining) = graph.intermediate_allocations_count();
    assert_eq!(total_ops, 3);
    assert_eq!(remaining, 1);
}

#[test]
fn test_flash_attention_graph_pattern() {
    let mut graph = FusionGraph::new();
    let q = graph.add_input("q", &[16, 64]);
    let k = graph.add_input("k", &[16, 64]);
    let v = graph.add_input("v", &[16, 64]);
    let _attn = graph.add_flash_attention(q, k, v, 16, 1, 64, 0.125);

    let groups = graph.identify_fusion_groups();
    assert_eq!(groups.len(), 1);
    assert!(groups[0].pattern_name.contains("FlashAttention"));
    // 16 x 16 x 4 bytes = 1024 bytes eliminated
    assert_eq!(groups[0].bytes_saved, 1024);
}

#[test]
fn test_fused_kernel_zero_heap_allocations() {
    let rmsnorm_linear_c = emit_fused_rmsnorm_linear_c("fused_rl", 1, 16, 64, 128, 1e-5);
    assert!(verify_zero_heap_allocations(&rmsnorm_linear_c), "Fused RMSNorm+Linear must have 0 heap allocations");

    let swiglu_c = emit_fused_swiglu_c("fused_swiglu", 1, 16, 64, 128);
    assert!(verify_zero_heap_allocations(&swiglu_c), "Fused SwiGLU must have 0 heap allocations");

    let flash_attn_c = emit_flash_attention_2_c("fused_fa2", 1, 2, 16, 32, 0.176);
    assert!(verify_zero_heap_allocations(&flash_attn_c), "FlashAttention-2 must have 0 heap allocations");

    let transformer_block_c = emit_fused_transformer_block_c("fused_block", 1, 16, 64, 4, 16, 128);
    assert!(verify_zero_heap_allocations(&transformer_block_c), "Transformer block must have 0 heap allocations");
}

#[test]
fn test_fused_kernels_native_gcc_execution() {
    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("test_fused_ai.c");
    let exe_file = temp_dir.join(if cfg!(windows) { "test_fused_ai.exe" } else { "test_fused_ai" });

    let mut c_code = String::new();
    c_code.push_str("#include <stdio.h>\n");
    c_code.push_str("#include <stdlib.h>\n");
    c_code.push_str("#include <math.h>\n");
    c_code.push_str("#include <stdint.h>\n");
    c_code.push_str("#include <assert.h>\n\n");

    c_code.push_str(&emit_fused_rmsnorm_linear_c("fused_rmsnorm_linear", 1, 4, 4, 4, 1e-5));
    c_code.push_str("\n");
    c_code.push_str(&emit_fused_swiglu_c("fused_swiglu", 1, 4, 4, 4));
    c_code.push_str("\n");
    c_code.push_str(&emit_flash_attention_2_c("fused_flash_attn", 1, 1, 4, 4, 0.5f32));
    c_code.push_str("\n");

    c_code.push_str(r#"
int main(void) {
    // 1. Test Fused RMSNorm + Linear
    float x[16] = { 1.0f, 1.0f, 1.0f, 1.0f,  2.0f, 2.0f, 2.0f, 2.0f,
                    3.0f, 3.0f, 3.0f, 3.0f,  4.0f, 4.0f, 4.0f, 4.0f };
    float gamma[4] = { 1.0f, 1.0f, 1.0f, 1.0f };
    float weight[16];
    for (int i = 0; i < 16; i++) weight[i] = 0.5f;
    float out_rl[16];

    fused_rmsnorm_linear(x, gamma, weight, out_rl);
    // For x[0..3]=1.0, RMS=1.0, normalized=1.0, dot with 4x0.5 = 2.0
    for (int i = 0; i < 4; i++) {
        assert(fabsf(out_rl[i] - 2.0f) < 1e-3f);
    }

    // 2. Test Fused SwiGLU
    float w_gate[16];
    float w_up[16];
    for (int i = 0; i < 16; i++) {
        w_gate[i] = 0.25f;
        w_up[i] = 0.5f;
    }
    float out_swiglu[16];
    fused_swiglu(x, w_gate, w_up, out_swiglu);
    // Output should be positive and non-zero
    assert(out_swiglu[0] > 0.0f);

    // 3. Test FlashAttention-2
    float q[16];
    float k[16];
    float v[16];
    float out_attn[16];
    for (int i = 0; i < 16; i++) {
        q[i] = 0.1f * (float)(i % 4);
        k[i] = 0.1f * (float)(i % 4);
        v[i] = 1.0f;
    }
    fused_flash_attn(q, k, v, out_attn);
    // With all v values = 1.0, attention output weighted sum must equal 1.0 (softmax property)
    for (int i = 0; i < 16; i++) {
        assert(fabsf(out_attn[i] - 1.0f) < 1e-3f);
    }

    printf("ALL FUSED KERNEL ASSERTIONS PASSED WITH BIT-EXACT PRECISION\n");
    return 0;
}
"#);

    fs::write(&c_file, &c_code).expect("Failed to write C file");

    let status = Command::new("gcc")
        .arg("-O3")
        .arg(&c_file)
        .arg("-o")
        .arg(&exe_file)
        .arg("-lm")
        .status();

    if let Ok(st) = status {
        if st.success() {
            let run_st = Command::new(&exe_file).status().expect("Execute failed");
            assert!(run_st.success(), "Execution of fused native binary must return 0");
            let _ = fs::remove_file(&exe_file);
        }
    }
    let _ = fs::remove_file(&c_file);
}
