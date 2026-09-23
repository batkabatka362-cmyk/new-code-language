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

#[test]
fn test_end_to_end_cognitive_transformer_snn_native() {
    let example_path = "../../examples/cognitive_transformer_snn.cr";
    let source = match fs::read_to_string(example_path) {
        Ok(s) => s,
        Err(_) => fs::read_to_string("examples/cognitive_transformer_snn.cr")
            .expect("Failed to read cognitive_transformer_snn.cr"),
    };

    let temp_dir = std::env::temp_dir();
    let exe_path = temp_dir.join("test_e2e_cognitive.exe");

    let res = compile_native_binary(&source, &exe_path, &["-mavx2"]);
    assert!(res.is_ok(), "Standalone native binary compilation failed: {:?}", res.err());

    let output = Command::new(&exe_path)
        .output()
        .expect("Failed to run standalone cognitive binary");

    // Exit code should be 1 (spike emitted)
    let code = output.status.code().unwrap_or(-1);
    assert_eq!(code, 1, "Expected exit code 1 (spike emitted), got {}", code);
}
