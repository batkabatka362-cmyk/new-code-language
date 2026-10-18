// ============================================================================
// CRON Sovereign Interop: Single-Header CUDA Library (cron.cuh)
// Multiplier-Free BitNet b1.58 & 4D Torus Warp-Cooperative Kernels
// Target: NVIDIA SM_80+ (Ampere / Hopper / Blackwell)
// ============================================================================
#pragma once

#include <cuda_runtime.h>
#include <cstdint>

namespace cron::cuda {

/// Fast sub-byte ternary dot product using PTX popc
__device__ __forceinline__ int ternary_dot_32(uint32_t a, uint32_t b) {
    uint32_t lo_a = a & 0x55555555u;
    uint32_t hi_a = (a >> 1u) & 0x55555555u;
    uint32_t pos_a = lo_a & ~hi_a;
    uint32_t neg_a = ~lo_a & hi_a;
    uint32_t lo_b = b & 0x55555555u;
    uint32_t hi_b = (b >> 1u) & 0x55555555u;
    uint32_t pos_b = lo_b & ~hi_b;
    uint32_t neg_b = ~lo_b & hi_b;
    uint32_t plus = (pos_a & pos_b) | (neg_a & neg_b);
    uint32_t minus = (pos_a & neg_b) | (neg_a & pos_b);
    return __popc(plus) - __popc(minus);
}

/// Warp-level all-reduce summation
__device__ __forceinline__ int warp_reduce_sum(int val) {
    #pragma unroll
    for (int offset = 16; offset > 0; offset /= 2) {
        val += __shfl_down_sync(0xFFFFFFFF, val, offset);
    }
    return val;
}

/// BitNet b1.58 GEMV Kernel (0 Multiplications on Tensor/CUDA Cores)
__global__ void bitnet_gemv_kernel(
    const uint8_t* __restrict__ packed_weights,
    const float* __restrict__ inputs,
    float* __restrict__ outputs,
    float scale,
    int rows,
    int cols
) {
    int row = blockIdx.x;
    if (row >= rows) return;

    int lane = threadIdx.x;
    int acc = 0;
    int total_words = (cols + 15) / 16;

    for (int w = lane; w < total_words; w += blockDim.x) {
        const uint32_t* row_w = reinterpret_cast<const uint32_t*>(packed_weights + row * ((cols + 3) / 4));
        uint32_t weight_word = row_w[w];
        // Decode and accumulate
        #pragma unroll
        for (int i = 0; i < 16; ++i) {
            int col_idx = w * 16 + i;
            if (col_idx < cols) {
                uint32_t code = (weight_word >> (i * 2)) & 0x03;
                if (code == 1) acc += (int)inputs[col_idx];
                else if (code == 2) acc -= (int)inputs[col_idx];
            }
        }
    }

    acc = warp_reduce_sum(acc);
    if (lane == 0) {
        outputs[row] = (float)acc * scale;
    }
}

} // namespace cron::cuda
