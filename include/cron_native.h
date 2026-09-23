// ============================================================================
// CRON High-Performance Native Accelerator C-FFI Interface (cron_native.h)
// Target: x86_64 AVX2/AVX-512 / ARM64 NEON Host Acceleration
// Standard: ISO C23
// ============================================================================

#ifndef CRON_NATIVE_H
#define CRON_NATIVE_H

#include <stdint.h>
#include <stddef.h>

#ifdef _WIN32
  #define CRON_EXPORT __declspec(dllexport)
#else
  #define CRON_EXPORT __attribute__((visibility("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Quantize and pack FP32 weight matrix into BitNet 1.58b ternary format {-1, 0, +1}.
 * 4 trits are packed per byte (2 bits each: 00=0, 01=+1, 11=-1, 10=reserved).
 * 
 * @param W          Input FP32 weight matrix of shape [K, N] (row-major)
 * @param W_packed   Output packed byte array of size (K * N + 3) / 4
 * @param scale_out  Pointer to output scaling factor: scale = mean(abs(W))
 * @param K          Inner dimension (input features)
 * @param N          Outer dimension (output features)
 */
CRON_EXPORT void cron_pack_trits_b158(
    const float* W,
    uint8_t* W_packed,
    float* scale_out,
    int K,
    int N
);

/**
 * Multiplier-free BitNet b1.58 Ternary GEMM:
 * Computes Y = X * W_ternary^T * scale + bias
 * 
 * Uses branchless 2-bit trit unpacking and unrolled vector accumulation.
 * 
 * @param X          Input activations of shape [M, K] (FP32)
 * @param W_packed   Packed ternary weight matrix of shape [N, K] (row-major, N rows of K elements)
 * @param bias       Optional bias vector of shape [N] (can be NULL)
 * @param Y          Output matrix of shape [M, N] (FP32)
 * @param M          Batch / Sequence dimension
 * @param K          Input features
 * @param N          Output features
 * @param scale      Ternary dequantization scale factor
 */
CRON_EXPORT void cron_bitnet_gemm_f32(
    const float* X,
    const uint8_t* W_packed,
    const float* bias,
    float* Y,
    int M,
    int K,
    int N,
    float scale
);

/**
 * RingTape O(1) Streaming FlashAttention Kernel:
 * Circular sliding-window multi-head attention without dynamic heap reallocation.
 * 
 * Computes: Out = softmax(Q * K_tape^T / sqrt(D)) * V_tape
 * 
 * @param Q          Query tensor [B, H, S, D]
 * @param K_tape     Circular Key tape buffer [B, H, Capacity, D]
 * @param V_tape     Circular Value tape buffer [B, H, Capacity, D]
 * @param Out        Output attention tensor [B, H, S, D]
 * @param B          Batch size
 * @param H          Number of attention heads
 * @param S          Query sequence length
 * @param D          Head dimension
 * @param capacity   Ring buffer capacity (max sequence window)
 * @param tape_head  Current circular pointer index (0 .. capacity-1)
 */
CRON_EXPORT void cron_ringtape_attention_f32(
    const float* Q,
    const float* K_tape,
    const float* V_tape,
    float* Out,
    int B,
    int H,
    int S,
    int D,
    int capacity,
    int tape_head
);

/**
 * Fast Vectorized Root Mean Square Normalization (RMSNorm)
 * Y = (X / sqrt(mean(X^2) + eps)) * W
 * 
 * @param X    Input tensor [M, D]
 * @param W    Weight scale vector [D]
 * @param Y    Output tensor [M, D]
 * @param M    Batch / token count
 * @param D    Hidden dimension
 * @param eps  Epsilon numerical stability constant (e.g. 1e-6)
 */
CRON_EXPORT void cron_fast_rmsnorm_f32(
    const float* X,
    const float* W,
    float* Y,
    int M,
    int D,
    float eps
);

/**
 * Return library version string and hardware capability flags.
 */
CRON_EXPORT const char* cron_native_version(void);
CRON_EXPORT uint32_t cron_native_features(void);

/**
 * Fused Single-Pass RMSNorm + Linear Projection:
 * Computes Y = (X * inv_rms * gamma) * W without writing normalized tensor to DRAM.
 */
CRON_EXPORT void cron_fused_rmsnorm_linear(
    const float* x,
    const float* gamma,
    const float* weight,
    float* out,
    int64_t total_tokens,
    int64_t d_in,
    int64_t d_out,
    float eps
);

/**
 * Fused Dual-Projection SwiGLU Gated Activation:
 * Computes Y = SiLU(X * W_gate) * (X * W_up) with in-register activation.
 */
CRON_EXPORT void cron_fused_swiglu(
    const float* x,
    const float* w_gate,
    const float* w_up,
    float* out,
    int64_t total_tokens,
    int64_t d_in,
    int64_t d_hidden
);

/**
 * FlashAttention-2 Online Softmax Attention Forward:
 * Computes attention context with online streaming softmax in O(N) memory without DRAM N^2 allocation.
 */
CRON_EXPORT void cron_flash_attention_2(
    const float* q,
    const float* k,
    const float* v,
    float* out,
    int64_t batch,
    int64_t heads,
    int64_t seq,
    int64_t d,
    float sm_scale
);

/**
 * Full Fused Transformer Layer Forward Block:
 * Fused Attention + Residual + RMSNorm + SwiGLU + Residual in 0 heap allocations.
 */
CRON_EXPORT void cron_fused_transformer_block(
    float* x,
    const float* gamma1,
    const float* w_q,
    const float* w_k,
    const float* w_v,
    const float* w_out,
    const float* gamma2,
    const float* w_gate,
    const float* w_up,
    const float* w_down,
    int64_t B,
    int64_t S,
    int64_t D,
    int64_t H,
    int64_t HD,
    int64_t FF
);

/**
 * 4D Clifford Algebra Cl(4,0) Full Geometric Product: C = A * B.
 * 16-blade multivector multiplication with branchless prefix-XOR POPCNT Cayley table.
 *
 * @param a  Input multivector A [16 blades]
 * @param b  Input multivector B [16 blades]
 * @param c  Output multivector C [16 blades]
 */
CRON_EXPORT void cron_c140_geometric_product(
    const float* a,
    const float* b,
    float* c
);

/**
 * 4D Clifford Algebra Cl(4,0) Rotor Sandwich Product: X' = R * X * ~R.
 *
 * @param rotor_8  Even rotor [scalar, e12, e13, e14, e23, e24, e34, e1234]
 * @param x_16     Input multivector X [16 blades]
 * @param out_16   Output rotated multivector X' [16 blades]
 */
CRON_EXPORT void cron_c140_rotor_sandwich(
    const float* rotor_8,
    const float* x_16,
    float* out_16
);

/**
 * Vectorized 4D Rotor Vector Rotation: v' = R * v * ~R for array of 4D vectors.
 * Uses exact branchless SO(4) quadratic forms and AVX2 FMA vectorization.
 *
 * @param rotor_8   Even rotor [scalar, e12, e13, e14, e23, e24, e34, e1234]
 * @param v_in      Array of 4D vectors [count * 4]
 * @param v_out     Output array of rotated 4D vectors [count * 4]
 * @param count     Number of 4D vectors
 */
CRON_EXPORT void cron_c140_vector_rotate_4d(
    const float* rotor_8,
    const float* v_in,
    float* v_out,
    int64_t count
);

/**
 * High-Throughput Spacetime Tensor Folding Batch:
 * Applies an array of rotors to fold/rotate batches of 4D vectors,
 * replacing heavy W * x GEMM with memory-free geometric rotations.
 *
 * @param x_in         Input tensor [num_vectors * 4]
 * @param rotors_8     Array of rotors [num_rotors * 8]
 * @param x_out        Output folded tensor [num_vectors * 4]
 * @param num_vectors  Total number of 4D vectors
 * @param num_rotors   Number of rotors (cycled over vectors)
 */
CRON_EXPORT void cron_c140_tensor_folding_batch(
    const float* x_in,
    const float* rotors_8,
    float* x_out,
    int64_t num_vectors,
    int64_t num_rotors
);

#ifdef __cplusplus
}
#endif

#endif // CRON_NATIVE_H
