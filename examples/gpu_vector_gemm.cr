.MODULE GpuVectorGemm

// Vector Fused Multiply-Add Tensor Kernel
def gemm_fma_kernel(a: [f32], b: [f32], c: [f32], n: u32) {
    let mut i = 0
    while i < n {
        let x = 1.25
        let w = 2.50
        let bias = 0.75
        let out = fma(x, w, bias)
        i = i + 1
    }
}

let n = 1024
let step = 1
