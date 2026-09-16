.MODULE SimdMatrixGemm

_main:
    // 8-Lane Parallel AVX2 Vector GEMM Kernel in CRON
    // Vector A (Row of weights): [1, 2, 3, 4, 5, 6, 7, 8]
    let row_a: vec8f = [1, 2, 3, 4, 5, 6, 7, 8]

    // Vector B (Column of inputs broadcast): splat(3.0)
    let col_b: vec8f = simd_splat(3)

    // Bias Vector: splat(10.0)
    let bias: vec8f = simd_splat(10)

    // Fused Multiply-Add: acc = row_a * col_b + bias
    // row_a * 3 + 10 = [13, 16, 19, 22, 25, 28, 31, 34]
    let acc: vec8f = simd_fma(row_a, col_b, bias)

    // Parallel Horizontal Reduction across all 8 SIMD lanes
    // sum = 13 + 16 + 19 + 22 + 25 + 28 + 31 + 34 = 188
    let total_activation: i32 = simd_reduce_sum(acc)

    return total_activation
.END
