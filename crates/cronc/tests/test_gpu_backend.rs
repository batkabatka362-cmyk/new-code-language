use cronc::{compile_to_metal, compile_to_ptx};

#[test]
fn test_cuda_ptx_generation_and_headers() {
    let cron_code = r#"
    .MODULE GpuVectorMath

    def vector_add_kernel(a: f32, b: f32, n: u32) -> f32 {
        let res = a + b
        return res
    }
    "#;

    let ptx = compile_to_ptx(cron_code).expect("PTX compilation failed");

    assert!(ptx.contains(".version 7.5"), "Missing PTX version directive");
    assert!(ptx.contains(".target sm_80"), "Missing PTX target directive");
    assert!(ptx.contains(".address_size 64"), "Missing 64-bit address size directive");
    assert!(ptx.contains(".visible .entry vector_add_kernel"), "Missing kernel entry");
    assert!(ptx.contains("%tid.x"), "Missing thread index register");
    assert!(ptx.contains("%ctaid.x"), "Missing block index register");
    assert!(ptx.contains("add.f32"), "Missing float addition instruction");
}

#[test]
fn test_cuda_ptx_fma_intrinsic() {
    let cron_code = r#"
    .MODULE GpuTensorFma

    def gemm_fma_kernel(x: f32, w: f32, bias: f32) -> f32 {
        let out = fma(x, w, bias)
        return out
    }
    "#;

    let ptx = compile_to_ptx(cron_code).expect("PTX compilation failed");

    assert!(ptx.contains("fma.rn.f32"), "Missing hardware FMA PTX instruction");
}

#[test]
fn test_apple_metal_generation() {
    let cron_code = r#"
    .MODULE AppleMetalPipeline

    def metal_tensor_kernel(data: [f32], out: [f32], count: u32) {
        let val = 3.14
        let doubled = val * 2.0
    }
    "#;

    let metal = compile_to_metal(cron_code).expect("Metal compilation failed");

    assert!(metal.contains("#include <metal_stdlib>"), "Missing Metal standard library header");
    assert!(metal.contains("using namespace metal;"), "Missing Metal namespace");
    assert!(metal.contains("kernel void metal_tensor_kernel"), "Missing Metal kernel signature");
    assert!(metal.contains("[[buffer(0)]]"), "Missing Metal buffer attribute 0");
    assert!(metal.contains("[[buffer(1)]]"), "Missing Metal buffer attribute 1");
    assert!(metal.contains("[[thread_position_in_grid]]"), "Missing Metal thread ID attribute");
}

#[test]
fn test_apple_metal_fma_intrinsic() {
    let cron_code = r#"
    .MODULE AppleMetalFma

    def metal_fma_kernel(a: f32, b: f32, c: f32) {
        let acc = fma(a, b, c)
    }
    "#;

    let metal = compile_to_metal(cron_code).expect("Metal compilation failed");

    assert!(metal.contains("fma(a, b, c)"), "Missing Metal fma call");
}
