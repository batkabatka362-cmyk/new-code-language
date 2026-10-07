use cronc::{compile_to_c23, compile_source, compile_to_metal, compile_to_ptx, compile_native_binary};
use std::fs;
use std::process::Command;

#[test]
fn test_tensor_abstraction_compilation_and_ptx() {
    let source = r#"
    .MODULE TensorTest
    fn test_tensor() -> f32 {
        let a: vec8f = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let b: vec8f = simd_splat(2.5);
        let c: vec8f = simd_splat(1.0);
        let fma_out: vec8f = simd_fma(a, b, c);
        let dot: f32 = simd_dot(fma_out, b);
        return dot;
    }
    "#;

    // 1. Test PTX compilation
    let ptx = compile_to_ptx(source).expect("PTX compilation failed");
    assert!(ptx.contains(".version 7.5"), "PTX header missing");
    assert!(ptx.contains(".target sm_80"), "PTX target missing");
    assert!(ptx.contains("fma_out") || ptx.contains("fma.rn.f32") || ptx.contains("test_tensor"), "PTX kernel missing");

    // 2. Test Metal compilation
    let metal = compile_to_metal(source).expect("Metal compilation failed");
    assert!(metal.contains("#include <metal_stdlib>"), "Metal stdlib missing");
    assert!(metal.contains("test_tensor"), "Metal function missing");

    // 3. Test .cl VLIW compilation
    let cl = compile_source(source).expect(".cl compilation failed");
    assert!(cl.contains("B0001:"), "VLIW bundles missing");
}

#[test]
fn test_transformer_attention_metal_and_ptx() {
    let source = r#"
    .MODULE TransformerTest
    fn rmsnorm_step(x: vec8f, weight: vec8f) -> vec8f {
        let sq: vec8f = x * x;
        let sum_sq: f32 = simd_reduce_sum(sq);
        let inv_rms: f32 = 1.0 / (sum_sq * 0.125 + 0.00001);
        let inv_vec: vec8f = simd_splat(inv_rms);
        return x * inv_vec * weight;
    }

    fn attention_score(q: vec8f, k: vec8f, scale: f32) -> f32 {
        return simd_dot(q, k) * scale;
    }
    "#;

    let ptx = compile_to_ptx(source).expect("PTX compilation failed");
    assert!(ptx.contains("rmsnorm_step"), "rmsnorm missing in PTX");
    assert!(ptx.contains("attention_score"), "attention_score missing in PTX");

    let metal = compile_to_metal(source).expect("Metal compilation failed");
    assert!(metal.contains("rmsnorm_step"), "rmsnorm missing in Metal");
    assert!(metal.contains("attention_score"), "attention_score missing in Metal");
}

#[test]
fn test_snn_surrogate_autograd_c23_execution() {
    let source = r#"
    .MODULE SNNAutogradTest
    fn fast_sigmoid_grad(u: f32, u_th: f32, alpha: f32) -> f32 {
        let diff = u - u_th;
        let abs_diff = if diff < 0.0 { -diff } else { diff };
        let denom = 1.0 + (alpha * abs_diff);
        return 1.0 / (denom * denom);
    }

    fn main() -> i32 {
        let syn_current: f32 = 1.2;
        let threshold: f32 = 1.0;
        let alpha: f32 = 2.0;

        // Gradient at U = 1.2: |1.2 - 1.0| = 0.2, denom = 1.0 + 2.0*0.2 = 1.4, grad = 1 / (1.4^2) = 0.5102
        let grad_val = fast_sigmoid_grad(syn_current, threshold, alpha);

        // Scale by 1000: ~510
        let scaled: i32 = (grad_val * 1000.0) as i32;
        return scaled;
    }
    "#;

    let c_code = compile_to_c23(source).expect("C23 compilation failed");
    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("test_snn_autograd.c");
    let exe_path = temp_dir.join("test_snn_autograd.exe");

    fs::write(&c_path, &c_code).expect("Failed to write C file");

    let status = Command::new("gcc")
        .args([
            "-std=c2x",
            "-O3",
            c_path.to_str().unwrap(),
            "-o",
            exe_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to run GCC");

    assert!(status.success(), "GCC compilation failed");

    let output = Command::new(&exe_path)
        .output()
        .expect("Failed to execute SNN autograd binary");

    // Return code is 510
    let code = output.status.code().unwrap_or(-1);
    assert_eq!(code, 510, "Expected surrogate gradient return code 510, got {}", code);
}

const COGNITIVE_TRANSFORMER_SNN_SRC: &str = r#"
.MODULE cron.example.transformer_snn

fn fast_sigmoid_derivative(u: f32, u_th: f32, alpha: f32) -> f32 {
    let diff = u - u_th;
    let abs_diff = if diff < 0.0 { -diff } else { diff };
    let denom = 1.0 + (alpha * abs_diff);
    return 1.0 / (denom * denom);
}

fn rmsnorm(x: vec8f, weight: vec8f, eps: f32) -> vec8f {
    let sq: vec8f = x * x;
    let sum_sq: f32 = simd_reduce_sum(sq);
    let mean_sq = sum_sq * 0.125;
    let inv_rms = 1.0 / (mean_sq + eps);
    let inv_vec = simd_splat(inv_rms);
    let normalized = x * inv_vec;
    return normalized * weight;
}

fn attention_head(x: vec8f, w_q: vec8f, w_k: vec8f, w_v: vec8f, scale: f32) -> vec8f {
    let q: vec8f = x * w_q;
    let k: vec8f = x * w_k;
    let v: vec8f = x * w_v;
    let qk_dot: f32 = simd_dot(q, k) * scale;
    let score_vec: vec8f = simd_splat(qk_dot);
    return v * score_vec;
}

fn lif_forward(u_membrane: f32, syn_input: f32, decay: f32, threshold: f32) -> f32 {
    let u_next = (u_membrane * decay) + syn_input;
    if u_next >= threshold {
        return 1.0;
    } else {
        return 0.0;
    }
}

fn main() -> i32 {
    let x: vec8f = [1.0, 1.5, 2.0, 0.5, 3.0, 2.5, 1.0, 0.5];
    let norm_w: vec8f = simd_splat(1.0);
    let w_q: vec8f = simd_splat(0.5);
    let w_k: vec8f = simd_splat(0.5);
    let w_v: vec8f = simd_splat(2.0);

    let x_norm = rmsnorm(x, norm_w, 0.00001);
    let attn_out = attention_head(x_norm, w_q, w_k, w_v, 0.3535);
    let x_residual = x + attn_out;

    let syn_weights: vec8f = [0.2, 0.4, 0.1, 0.5, 0.3, 0.6, 0.2, 0.1];
    let syn_current: f32 = simd_dot(x_residual, syn_weights) + 0.1;

    let spike = lif_forward(0.2, syn_current, 0.8, 1.0);

    let surrogate_grad = fast_sigmoid_derivative(syn_current, 1.0, 2.0);
    let grad_vec = simd_splat(surrogate_grad);
    let weight_grads = x_residual * grad_vec;

    let _grad_sum: f32 = simd_reduce_sum(weight_grads);
    let result_code: i32 = if spike > 0.5 { 1 } else { 0 };

    return result_code;
}
"#;

#[test]
fn test_end_to_end_cognitive_transformer_snn_native() {
    let source = COGNITIVE_TRANSFORMER_SNN_SRC;

    let temp_dir = std::env::temp_dir();
    let exe_path = temp_dir.join("test_e2e_cognitive.exe");

    let res = compile_native_binary(source, &exe_path, &["-mavx2"]);
    assert!(res.is_ok(), "Standalone native binary compilation failed: {:?}", res.err());

    let output = Command::new(&exe_path)
        .output()
        .expect("Failed to run standalone cognitive binary");

    // Exit code should be 1 (spike emitted)
    let code = output.status.code().unwrap_or(-1);
    assert_eq!(code, 1, "Expected exit code 1 (spike emitted), got {}", code);
}
