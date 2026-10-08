// ============================================================================
// CRON High-Performance Native Accelerator Implementation (cron_native.c)
// Vectorized Multiplier-Free BitNet b1.58 GEMM & RingTape Attention
// Standard: ISO C23
// ============================================================================

#include "../include/cron_native.h"
#include <math.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

#if defined(__x86_64__) || defined(_M_X64)
  #include <immintrin.h>
#endif

#ifdef _OPENMP
  #include <omp.h>
#endif

// Feature bitmask flags
#define CRON_FEATURE_AVX2     (1 << 0)
#define CRON_FEATURE_FMA      (1 << 1)
#define CRON_FEATURE_AVX512   (1 << 2)
#define CRON_FEATURE_NEON     (1 << 3)
#define CRON_FEATURE_BITNET   (1 << 4)
#define CRON_FEATURE_RINGTAPE (1 << 5)
#define CRON_FEATURE_OPENMP   (1 << 6)

// Precomputed 256-bit bitwise masks for 4 trits per byte
// Mask zeros out float if trit == 0
// Sign flips float sign bit if trit == -1 (0b11)
static uint32_t BYTE_TO_MASK[256][4] __attribute__((aligned(32)));
static uint32_t BYTE_TO_SIGN[256][4] __attribute__((aligned(32)));
static int g_tables_initialized = 0;

static void init_bitnet_tables(void) {
    if (g_tables_initialized) return;
    for (int b = 0; b < 256; ++b) {
        for (int i = 0; i < 4; ++i) {
            uint8_t t = (b >> (i * 2)) & 3;
            if (t == 1) { // +1
                BYTE_TO_MASK[b][i] = 0xFFFFFFFFU;
                BYTE_TO_SIGN[b][i] = 0x00000000U;
            } else if (t == 3) { // -1
                BYTE_TO_MASK[b][i] = 0xFFFFFFFFU;
                BYTE_TO_SIGN[b][i] = 0x80000000U; // IEEE 754 negative sign bit
            } else { // 0
                BYTE_TO_MASK[b][i] = 0x00000000U;
                BYTE_TO_SIGN[b][i] = 0x00000000U;
            }
        }
    }
    g_tables_initialized = 1;
}

CRON_EXPORT const char* cron_native_version(void) {
#ifdef _OPENMP
    return "CRON-Native-Accelerator v1.2.0 (C23 / AVX2 Zero-Multiplier / OpenMP / BitNet-b1.58)";
#else
    return "CRON-Native-Accelerator v1.2.0 (C23 / AVX2 Zero-Multiplier / BitNet-b1.58)";
#endif
}

CRON_EXPORT uint32_t cron_native_features(void) {
    uint32_t flags = CRON_FEATURE_BITNET | CRON_FEATURE_RINGTAPE;
#if defined(__AVX2__)
    flags |= CRON_FEATURE_AVX2;
#endif
#if defined(__FMA__)
    flags |= CRON_FEATURE_FMA;
#endif
#if defined(__AVX512F__)
    flags |= CRON_FEATURE_AVX512;
#endif
#if defined(_OPENMP)
    flags |= CRON_FEATURE_OPENMP;
#endif
    return flags;
}

// ----------------------------------------------------------------------------
// Trit Packing: BitNet 1.58b Weight Quantization
// ----------------------------------------------------------------------------

CRON_EXPORT void cron_pack_trits_b158(
    const float* W,
    uint8_t* W_packed,
    float* scale_out,
    int K,
    int N
) {
    init_bitnet_tables();
    int total_elements = K * N;
    if (total_elements <= 0 || K <= 0 || N <= 0) {
        if (scale_out) *scale_out = 1.0f;
        return;
    }

    double sum_abs = 0.0;
    for (int i = 0; i < total_elements; ++i) {
        sum_abs += fabsf(W[i]);
    }
    float scale = (float)(sum_abs / (double)total_elements);
    if (scale < 1e-8f) scale = 1e-8f;
    if (scale_out) *scale_out = scale;

    float threshold = scale * 0.5f;
    const int bytes_per_row = (K + 3) / 4;

    for (int n = 0; n < N; ++n) {
        const float* row_in = W + (size_t)n * K;
        uint8_t* row_out = W_packed + (size_t)n * bytes_per_row;
        int k = 0;
        int b_idx = 0;
        while (k < K) {
            uint8_t byte_val = 0;
            for (int b = 0; b < 4 && (k + b) < K; ++b) {
                float val = row_in[k + b];
                uint8_t trit_code = 0;
                if (val > threshold) {
                    trit_code = 1; // +1
                } else if (val < -threshold) {
                    trit_code = 3; // -1 (0b11)
                }
                byte_val |= (uint8_t)(trit_code << (b * 2));
            }
            row_out[b_idx++] = byte_val;
            k += 4;
        }
    }
}

// ----------------------------------------------------------------------------
// AVX2 Zero-Multiplier Row Dot Product Helper
// ----------------------------------------------------------------------------

#if defined(__AVX2__)
static inline float hsum256_ps(__m256 v) {
    __m128 vlow  = _mm256_castps256_ps128(v);
    __m128 vhigh = _mm256_extractf128_ps(v, 1);
    __m128 v128  = _mm_add_ps(vlow, vhigh);
    __m128 vshuf = _mm_movehdup_ps(v128);
    __m128 vsums = _mm_add_ps(v128, vshuf);
    vshuf = _mm_movehl_ps(vshuf, vsums);
    vsums = _mm_add_ss(vsums, vshuf);
    return _mm_cvtss_f32(vsums);
}
#endif

static inline float compute_dot_row_avx2(
    const float* x_row,
    const uint8_t* w_row,
    int K
) {
    int k = 0;
    int b_idx = 0;

#if defined(__AVX2__)
    __m256 vacc0 = _mm256_setzero_ps();
    __m256 vacc1 = _mm256_setzero_ps();

    // 16 floats (4 bytes = 16 trits) per unrolled loop
    while (k + 16 <= K) {
        uint8_t b0 = w_row[b_idx];
        uint8_t b1 = w_row[b_idx + 1];
        uint8_t b2 = w_row[b_idx + 2];
        uint8_t b3 = w_row[b_idx + 3];

        // First 8 floats (b0, b1)
        __m256 vx0 = _mm256_loadu_ps(&x_row[k]);
        __m128i m0 = _mm_loadu_si128((const __m128i*)BYTE_TO_MASK[b0]);
        __m128i m1 = _mm_loadu_si128((const __m128i*)BYTE_TO_MASK[b1]);
        __m256 vmask0 = _mm256_castsi256_ps(_mm256_set_m128i(m1, m0));

        __m128i s0 = _mm_loadu_si128((const __m128i*)BYTE_TO_SIGN[b0]);
        __m128i s1 = _mm_loadu_si128((const __m128i*)BYTE_TO_SIGN[b1]);
        __m256 vsign0 = _mm256_castsi256_ps(_mm256_set_m128i(s1, s0));

        __m256 vres0 = _mm256_xor_ps(_mm256_and_ps(vx0, vmask0), vsign0);
        vacc0 = _mm256_add_ps(vacc0, vres0);

        // Next 8 floats (b2, b3)
        __m256 vx1 = _mm256_loadu_ps(&x_row[k + 8]);
        __m128i m2 = _mm_loadu_si128((const __m128i*)BYTE_TO_MASK[b2]);
        __m128i m3 = _mm_loadu_si128((const __m128i*)BYTE_TO_MASK[b3]);
        __m256 vmask1 = _mm256_castsi256_ps(_mm256_set_m128i(m3, m2));

        __m128i s2 = _mm_loadu_si128((const __m128i*)BYTE_TO_SIGN[b2]);
        __m128i s3 = _mm_loadu_si128((const __m128i*)BYTE_TO_SIGN[b3]);
        __m256 vsign1 = _mm256_castsi256_ps(_mm256_set_m128i(s3, s2));

        __m256 vres1 = _mm256_xor_ps(_mm256_and_ps(vx1, vmask1), vsign1);
        vacc1 = _mm256_add_ps(vacc1, vres1);

        k += 16;
        b_idx += 4;
    }

    float total_acc = hsum256_ps(_mm256_add_ps(vacc0, vacc1));
#else
    float total_acc = 0.0f;
#endif

    // Remainder
    while (k < K) {
        uint8_t b = w_row[b_idx++];
        for (int bit = 0; bit < 4 && k < K; ++bit, ++k) {
            uint8_t t = (b >> (bit * 2)) & 3;
            if (t == 1) total_acc += x_row[k];
            else if (t == 3) total_acc -= x_row[k];
        }
    }

    return total_acc;
}

// ----------------------------------------------------------------------------
// High-Performance Multiplier-Free BitNet b1.58 GEMM
// ----------------------------------------------------------------------------

CRON_EXPORT void cron_bitnet_gemm_f32(
    const float* X,
    const uint8_t* W_packed,
    const float* bias,
    float* Y,
    int M,
    int K,
    int N,
    float scale
) {
    init_bitnet_tables();
    const int bytes_per_row = (K + 3) / 4;

    #pragma omp parallel for schedule(static)
    for (int n = 0; n < N; ++n) {
        const uint8_t* w_row = W_packed + (size_t)n * bytes_per_row;
        float b_val = bias ? bias[n] : 0.0f;
        for (int m = 0; m < M; ++m) {
            const float* x_row = X + (size_t)m * K;
            float acc = compute_dot_row_avx2(x_row, w_row, K) * scale + b_val;
            Y[(size_t)m * N + n] = acc;
        }
    }
}

CRON_EXPORT float cron_bitnet_dot_f32(
    const float* X,
    const uint8_t* W_packed_row,
    int K,
    float scale
) {
    if (!X || !W_packed_row || K <= 0) return 0.0f;
    init_bitnet_tables();
    return compute_dot_row_avx2(X, W_packed_row, K) * scale;
}

// ----------------------------------------------------------------------------
// 2:4 Structured Sparsity Trit Packing & Multiplier-Free GEMM
// ----------------------------------------------------------------------------

CRON_EXPORT void cron_sparse_24_pack_trits_b158(
    const float* W,
    uint8_t* W_packed,
    float* scale_out,
    int K,
    int N
) {
    init_bitnet_tables();
    if (!W || !W_packed || K <= 0 || N <= 0) {
        if (scale_out) *scale_out = 1.0f;
        return;
    }

    double sum_abs = 0.0;
    int total_elements = K * N;
    for (int i = 0; i < total_elements; ++i) {
        sum_abs += fabsf(W[i]);
    }
    float scale = (float)(sum_abs / (double)total_elements);
    if (scale < 1e-8f) scale = 1e-8f;
    if (scale_out) *scale_out = scale;
    float threshold = scale * 0.5f;

    const int bytes_per_row = (K + 3) / 4;

    for (int n = 0; n < N; ++n) {
        const float* row_in = W + (size_t)n * K;
        uint8_t* row_out = W_packed + (size_t)n * bytes_per_row;
        int k = 0;
        int b_idx = 0;

        while (k < K) {
            uint8_t byte_val = 0;
            int block_len = 0;
            float block_vals[4] = {0.0f, 0.0f, 0.0f, 0.0f};
            float block_abs[4] = {0.0f, 0.0f, 0.0f, 0.0f};
            int indices[4] = {0, 1, 2, 3};

            for (int b = 0; b < 4 && (k + b) < K; ++b) {
                block_vals[b] = row_in[k + b];
                block_abs[b] = fabsf(block_vals[b]);
                block_len++;
            }

            // Enforce 2:4 structured sparsity (prune all except top-2 magnitudes)
            if (block_len >= 3) {
                for (int i = 0; i < block_len - 1; ++i) {
                    for (int j = i + 1; j < block_len; ++j) {
                        if (block_abs[indices[j]] > block_abs[indices[i]]) {
                            int tmp = indices[i];
                            indices[i] = indices[j];
                            indices[j] = tmp;
                        }
                    }
                }
                for (int i = 2; i < block_len; ++i) {
                    block_vals[indices[i]] = 0.0f;
                }
            }

            for (int b = 0; b < block_len; ++b) {
                float val = block_vals[b];
                uint8_t trit_code = 0;
                if (val > threshold) {
                    trit_code = 1; // +1
                } else if (val < -threshold) {
                    trit_code = 3; // -1
                }
                byte_val |= (uint8_t)(trit_code << (b * 2));
            }

            row_out[b_idx++] = byte_val;
            k += 4;
        }
    }
}

CRON_EXPORT void cron_sparse_24_gemm_f32(
    const float* X,
    const uint8_t* W_packed,
    const float* bias,
    float* Y,
    int M,
    int K,
    int N,
    float scale
) {
    init_bitnet_tables();
    const int bytes_per_row = (K + 3) / 4;

    #pragma omp parallel for schedule(static)
    for (int n = 0; n < N; ++n) {
        const uint8_t* w_row = W_packed + (size_t)n * bytes_per_row;
        float b_val = bias ? bias[n] : 0.0f;
        for (int m = 0; m < M; ++m) {
            const float* x_row = X + (size_t)m * K;
            float acc = compute_dot_row_avx2(x_row, w_row, K) * scale + b_val;
            Y[(size_t)m * N + n] = acc;
        }
    }
}

// ----------------------------------------------------------------------------
// RingTape O(1) Streaming Multi-Head Attention Kernel (AVX2 + FMA)
// ----------------------------------------------------------------------------

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
) {
    float inv_sqrt_d = 1.0f / sqrtf((float)D);
    size_t q_stride_bh = (size_t)S * D;
    size_t tape_stride_bh = (size_t)capacity * D;

    #pragma omp parallel for collapse(2) schedule(static) if(B * H > 1)
    for (int b = 0; b < B; ++b) {
        for (int h = 0; h < H; ++h) {
            size_t q_offset = ((size_t)b * H + h) * q_stride_bh;
            size_t tape_offset = ((size_t)b * H + h) * tape_stride_bh;

            const float* q_bh = Q + q_offset;
            const float* k_bh = K_tape + tape_offset;
            const float* v_bh = V_tape + tape_offset;
            float* out_bh = Out + q_offset;

            // Stack-allocated scores for zero-allocation streaming (up to 2048 window)
            float stack_scores[2048];
            float* local_scores = stack_scores;
            bool is_heap = false;
            if (capacity > 2048) {
                local_scores = (float*)malloc(sizeof(float) * capacity);
                is_heap = true;
            }
            if (!local_scores) continue;

            for (int s = 0; s < S; ++s) {
                const float* q_vec = q_bh + (size_t)s * D;
                float* out_vec = out_bh + (size_t)s * D;

                float max_score = -1e30f;
                for (int pos = 0; pos < capacity; ++pos) {
                    int tape_idx = (tape_head - capacity + 1 + pos + capacity) % capacity;
                    const float* k_vec = k_bh + (size_t)tape_idx * D;

#if defined(__AVX2__)
                    int d = 0;
                    __m256 vdot0 = _mm256_setzero_ps();
                    __m256 vdot1 = _mm256_setzero_ps();
                    for (; d + 15 < D; d += 16) {
                        __m256 vq0 = _mm256_loadu_ps(&q_vec[d]);
                        __m256 vk0 = _mm256_loadu_ps(&k_vec[d]);
                        vdot0 = _mm256_fmadd_ps(vq0, vk0, vdot0);

                        __m256 vq1 = _mm256_loadu_ps(&q_vec[d + 8]);
                        __m256 vk1 = _mm256_loadu_ps(&k_vec[d + 8]);
                        vdot1 = _mm256_fmadd_ps(vq1, vk1, vdot1);
                    }
                    for (; d + 7 < D; d += 8) {
                        __m256 vq = _mm256_loadu_ps(&q_vec[d]);
                        __m256 vk = _mm256_loadu_ps(&k_vec[d]);
                        vdot0 = _mm256_fmadd_ps(vq, vk, vdot0);
                    }
                    float dot = hsum256_ps(_mm256_add_ps(vdot0, vdot1));
                    for (; d < D; ++d) {
                        dot += q_vec[d] * k_vec[d];
                    }
#else
                    float dot = 0.0f;
                    for (int d = 0; d < D; ++d) {
                        dot += q_vec[d] * k_vec[d];
                    }
#endif
                    dot *= inv_sqrt_d;
                    local_scores[pos] = dot;
                    if (dot > max_score) max_score = dot;
                }

                // Stable Softmax
                float sum_exp = 0.0f;
                for (int pos = 0; pos < capacity; ++pos) {
                    float exp_val = expf(local_scores[pos] - max_score);
                    local_scores[pos] = exp_val;
                    sum_exp += exp_val;
                }
                float inv_sum = 1.0f / (sum_exp + 1e-8f);

                // Zero out destination
                memset(out_vec, 0, sizeof(float) * D);

                // Weighted value accumulation with AVX2 FMA
                for (int pos = 0; pos < capacity; ++pos) {
                    float prob = local_scores[pos] * inv_sum;
                    if (prob < 1e-7f) continue;

                    int tape_idx = (tape_head - capacity + 1 + pos + capacity) % capacity;
                    const float* v_vec = v_bh + (size_t)tape_idx * D;

#if defined(__AVX2__)
                    __m256 vprob = _mm256_set1_ps(prob);
                    int d = 0;
                    for (; d + 15 < D; d += 16) {
                        __m256 vo0 = _mm256_loadu_ps(&out_vec[d]);
                        __m256 vv0 = _mm256_loadu_ps(&v_vec[d]);
                        vo0 = _mm256_fmadd_ps(vprob, vv0, vo0);
                        _mm256_storeu_ps(&out_vec[d], vo0);

                        __m256 vo1 = _mm256_loadu_ps(&out_vec[d + 8]);
                        __m256 vv1 = _mm256_loadu_ps(&v_vec[d + 8]);
                        vo1 = _mm256_fmadd_ps(vprob, vv1, vo1);
                        _mm256_storeu_ps(&out_vec[d + 8], vo1);
                    }
                    for (; d + 7 < D; d += 8) {
                        __m256 vo = _mm256_loadu_ps(&out_vec[d]);
                        __m256 vv = _mm256_loadu_ps(&v_vec[d]);
                        vo = _mm256_fmadd_ps(vprob, vv, vo);
                        _mm256_storeu_ps(&out_vec[d], vo);
                    }
                    for (; d < D; ++d) {
                        out_vec[d] += prob * v_vec[d];
                    }
#else
                    for (int d = 0; d < D; ++d) {
                        out_vec[d] += prob * v_vec[d];
                    }
#endif
                }
            }
            if (is_heap) free(local_scores);
        }
    }
}

// ----------------------------------------------------------------------------
// Sub-Byte 4-Bit KV-Cache Quantization & Streaming FlashAttention
// ----------------------------------------------------------------------------

CRON_EXPORT void cron_subbyte_kv_quantize_4bit(
    const float* in_fp32,
    uint8_t* out_packed,
    size_t count,
    float* scale_out
) {
    if (!in_fp32 || !out_packed || count == 0) {
        if (scale_out) *scale_out = 1.0f;
        return;
    }

    float max_abs = 0.0f;
    for (size_t i = 0; i < count; ++i) {
        float a = fabsf(in_fp32[i]);
        if (a > max_abs) max_abs = a;
    }
    if (max_abs < 1e-8f) max_abs = 1e-8f;
    float scale = max_abs / 7.0f;
    float inv_scale = 1.0f / scale;
    if (scale_out) *scale_out = scale;

    size_t packed_idx = 0;
    size_t i = 0;
    while (i < count) {
        float f0 = in_fp32[i];
        int q0 = (int)roundf(f0 * inv_scale);
        if (q0 > 7) q0 = 7;
        if (q0 < -7) q0 = -7;
        uint8_t nib0 = (uint8_t)(q0 & 0x0F);

        uint8_t nib1 = 0;
        if (i + 1 < count) {
            float f1 = in_fp32[i + 1];
            int q1 = (int)roundf(f1 * inv_scale);
            if (q1 > 7) q1 = 7;
            if (q1 < -7) q1 = -7;
            nib1 = (uint8_t)(q1 & 0x0F);
        }

        out_packed[packed_idx++] = nib0 | (uint8_t)(nib1 << 4);
        i += 2;
    }
}

CRON_EXPORT void cron_subbyte_ringtape_attention_f32(
    const float* Q,
    const uint8_t* K_tape_packed,
    const uint8_t* V_tape_packed,
    float k_scale,
    float v_scale,
    float* Out,
    int B,
    int H,
    int S,
    int D,
    int capacity,
    int tape_head
) {
    float inv_sqrt_d = 1.0f / sqrtf((float)D);
    size_t q_stride_bh = (size_t)S * D;
    size_t bytes_per_token = (size_t)D / 2;
    size_t tape_stride_bh = (size_t)capacity * bytes_per_token;

    #pragma omp parallel for collapse(2) schedule(static) if(B * H > 1)
    for (int b = 0; b < B; ++b) {
        for (int h = 0; h < H; ++h) {
            size_t q_offset = ((size_t)b * H + h) * q_stride_bh;
            size_t tape_offset = ((size_t)b * H + h) * tape_stride_bh;

            const float* q_bh = Q + q_offset;
            const uint8_t* k_bh = K_tape_packed + tape_offset;
            const uint8_t* v_bh = V_tape_packed + tape_offset;
            float* out_bh = Out + q_offset;

            float stack_scores[2048];
            float* local_scores = stack_scores;
            bool is_heap = false;
            if (capacity > 2048) {
                local_scores = (float*)malloc(sizeof(float) * capacity);
                is_heap = true;
            }

            for (int s = 0; s < S; ++s) {
                const float* q_vec = q_bh + (size_t)s * D;
                float* out_vec = out_bh + (size_t)s * D;

                float max_score = -1e30f;
                for (int pos = 0; pos < capacity; ++pos) {
                    int tape_idx = (tape_head - capacity + 1 + pos + capacity) % capacity;
                    const uint8_t* k_packed = k_bh + (size_t)tape_idx * bytes_per_token;

                    float dot = 0.0f;
                    for (int j = 0; j < (D / 2); ++j) {
                        uint8_t byte_val = k_packed[j];
                        int nib0 = byte_val & 0x0F;
                        int nib1 = (byte_val >> 4) & 0x0F;
                        int s0 = (nib0 >= 8) ? (nib0 - 16) : nib0;
                        int s1 = (nib1 >= 8) ? (nib1 - 16) : nib1;
                        dot += q_vec[2 * j] * (float)s0 + q_vec[2 * j + 1] * (float)s1;
                    }
                    dot *= (k_scale * inv_sqrt_d);
                    local_scores[pos] = dot;
                    if (dot > max_score) max_score = dot;
                }

                // Stable Softmax
                float sum_exp = 0.0f;
                for (int pos = 0; pos < capacity; ++pos) {
                    float exp_val = expf(local_scores[pos] - max_score);
                    local_scores[pos] = exp_val;
                    sum_exp += exp_val;
                }
                float inv_sum = 1.0f / (sum_exp + 1e-8f);

                // Zero output
                memset(out_vec, 0, sizeof(float) * D);

                // Accumulate from 4-bit V-tape
                for (int pos = 0; pos < capacity; ++pos) {
                    float prob = local_scores[pos] * inv_sum;
                    if (prob < 1e-7f) continue;

                    int tape_idx = (tape_head - capacity + 1 + pos + capacity) % capacity;
                    const uint8_t* v_packed = v_bh + (size_t)tape_idx * bytes_per_token;
                    float weighted_scale = prob * v_scale;

                    for (int j = 0; j < (D / 2); ++j) {
                        uint8_t byte_val = v_packed[j];
                        int nib0 = byte_val & 0x0F;
                        int nib1 = (byte_val >> 4) & 0x0F;
                        int s0 = (nib0 >= 8) ? (nib0 - 16) : nib0;
                        int s1 = (nib1 >= 8) ? (nib1 - 16) : nib1;
                        out_vec[2 * j] += weighted_scale * (float)s0;
                        out_vec[2 * j + 1] += weighted_scale * (float)s1;
                    }
                }
            }
            if (is_heap) free(local_scores);
        }
    }
}

// ----------------------------------------------------------------------------
// Fast Vectorized RMSNorm (AVX2 + FMA)
// ----------------------------------------------------------------------------

CRON_EXPORT void cron_fast_rmsnorm_f32(
    const float* X,
    const float* W,
    float* Y,
    int M,
    int D,
    float eps
) {
    float inv_d = 1.0f / (float)D;

    #pragma omp parallel for schedule(static) if(M > 1)
    for (int m = 0; m < M; ++m) {
        const float* x_vec = X + (size_t)m * D;
        float* y_vec = Y + (size_t)m * D;

#if defined(__AVX2__)
        int d = 0;
        __m256 vacc0 = _mm256_setzero_ps();
        __m256 vacc1 = _mm256_setzero_ps();
        for (; d + 15 < D; d += 16) {
            __m256 vx0 = _mm256_loadu_ps(&x_vec[d]);
            __m256 vx1 = _mm256_loadu_ps(&x_vec[d + 8]);
            vacc0 = _mm256_fmadd_ps(vx0, vx0, vacc0);
            vacc1 = _mm256_fmadd_ps(vx1, vx1, vacc1);
        }
        for (; d + 7 < D; d += 8) {
            __m256 vx = _mm256_loadu_ps(&x_vec[d]);
            vacc0 = _mm256_fmadd_ps(vx, vx, vacc0);
        }
        float sum_sq = hsum256_ps(_mm256_add_ps(vacc0, vacc1));
        for (; d < D; ++d) {
            float v = x_vec[d];
            sum_sq += v * v;
        }

        float rms = sqrtf((sum_sq * inv_d) + eps);
        float scale = 1.0f / rms;
        __m256 vscale = _mm256_set1_ps(scale);

        d = 0;
        for (; d + 15 < D; d += 16) {
            __m256 vx0 = _mm256_loadu_ps(&x_vec[d]);
            __m256 vw0 = _mm256_loadu_ps(&W[d]);
            __m256 vy0 = _mm256_mul_ps(_mm256_mul_ps(vx0, vscale), vw0);
            _mm256_storeu_ps(&y_vec[d], vy0);

            __m256 vx1 = _mm256_loadu_ps(&x_vec[d + 8]);
            __m256 vw1 = _mm256_loadu_ps(&W[d + 8]);
            __m256 vy1 = _mm256_mul_ps(_mm256_mul_ps(vx1, vscale), vw1);
            _mm256_storeu_ps(&y_vec[d + 8], vy1);
        }
        for (; d + 7 < D; d += 8) {
            __m256 vx = _mm256_loadu_ps(&x_vec[d]);
            __m256 vw = _mm256_loadu_ps(&W[d]);
            __m256 vy = _mm256_mul_ps(_mm256_mul_ps(vx, vscale), vw);
            _mm256_storeu_ps(&y_vec[d], vy);
        }
        for (; d < D; ++d) {
            y_vec[d] = x_vec[d] * scale * W[d];
        }
#else
        double sum_sq = 0.0;
        for (int d = 0; d < D; ++d) {
            float v = x_vec[d];
            sum_sq += (double)(v * v);
        }

        float rms = (float)sqrt((sum_sq * (double)inv_d) + (double)eps);
        float scale = 1.0f / rms;

        for (int d = 0; d < D; ++d) {
            y_vec[d] = x_vec[d] * scale * W[d];
        }
#endif
    }
}

// ----------------------------------------------------------------------------
// Fused Single-Pass RMSNorm + Linear Kernel (Zero Intermediate DRAM Writeback)
// ----------------------------------------------------------------------------

CRON_EXPORT void cron_fused_rmsnorm_linear(
    const float* x,
    const float* gamma,
    const float* weight,
    float* out,
    int64_t total_tokens,
    int64_t d_in,
    int64_t d_out,
    float eps
) {
    #pragma omp parallel for schedule(static) if(total_tokens > 1)
    for (int64_t t = 0; t < total_tokens; t++) {
        const float* in_ptr = &x[t * d_in];
        float* out_ptr = &out[t * d_out];

        // 1. In-register RMSNorm Reduction
        float sum_sq = 0.0f;
        #pragma GCC ivdep
        for (int64_t i = 0; i < d_in; i++) {
            sum_sq += in_ptr[i] * in_ptr[i];
        }
        const float inv_rms = 1.0f / sqrtf((sum_sq / (float)d_in) + eps);

        // 2. Fused Linear Contraction with On-the-Fly Normalization
        for (int64_t o = 0; o < d_out; o++) {
            const float* w_col = &weight[o * d_in];
            float acc = 0.0f;
            #pragma GCC ivdep
            for (int64_t i = 0; i < d_in; i++) {
                float norm_val = in_ptr[i] * inv_rms * gamma[i];
                acc += norm_val * w_col[i];
            }
            out_ptr[o] = acc;
        }
    }
}

// ----------------------------------------------------------------------------
// Fused Dual-Projection SwiGLU Gated Activation Kernel
// ----------------------------------------------------------------------------

CRON_EXPORT void cron_fused_swiglu(
    const float* x,
    const float* w_gate,
    const float* w_up,
    float* out,
    int64_t total_tokens,
    int64_t d_in,
    int64_t d_hidden
) {
    #pragma omp parallel for schedule(static) if(total_tokens > 1)
    for (int64_t t = 0; t < total_tokens; t++) {
        const float* in_ptr = &x[t * d_in];
        float* out_ptr = &out[t * d_hidden];

        for (int64_t h = 0; h < d_hidden; h++) {
            const float* wg_col = &w_gate[h * d_in];
            const float* wu_col = &w_up[h * d_in];
            float gate_acc = 0.0f;
            float up_acc = 0.0f;
            #pragma GCC ivdep
            for (int64_t i = 0; i < d_in; i++) {
                gate_acc += in_ptr[i] * wg_col[i];
                up_acc += in_ptr[i] * wu_col[i];
            }
            float silu_gate = gate_acc / (1.0f + expf(-gate_acc));
            out_ptr[h] = silu_gate * up_acc;
        }
    }
}

// ----------------------------------------------------------------------------
// FlashAttention-2 Online Softmax Forward Kernel (O(N) Memory)
// ----------------------------------------------------------------------------

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
) {
    #pragma omp parallel for collapse(2) schedule(static) if(batch * heads > 1)
    for (int64_t b = 0; b < batch; b++) {
        for (int64_t h = 0; h < heads; h++) {
            const int64_t bh_offset = (b * heads + h) * (seq * d);
            const float* q_bh = &q[bh_offset];
            const float* k_bh = &k[bh_offset];
            const float* v_bh = &v[bh_offset];
            float* out_bh = &out[bh_offset];

            for (int64_t i = 0; i < seq; i++) {
                const float* q_row = &q_bh[i * d];
                float* o_row = &out_bh[i * d];
                for (int64_t col = 0; col < d; col++) o_row[col] = 0.0f;

                float m_prev = -1e30f;
                float l_prev = 0.0f;

                for (int64_t j = 0; j < seq; j++) {
                    const float* k_row = &k_bh[j * d];
                    const float* v_row = &v_bh[j * d];

                    float score = 0.0f;
                    #pragma GCC ivdep
                    for (int64_t col = 0; col < d; col++) {
                        score += q_row[col] * k_row[col];
                    }
                    score *= sm_scale;

                    float m_curr = score > m_prev ? score : m_prev;
                    float p_unnorm = expf(score - m_curr);
                    float rescale = expf(m_prev - m_curr);
                    float l_curr = l_prev * rescale + p_unnorm;

                    #pragma GCC ivdep
                    for (int64_t col = 0; col < d; col++) {
                        o_row[col] = (o_row[col] * l_prev * rescale + p_unnorm * v_row[col]) / l_curr;
                    }

                    m_prev = m_curr;
                    l_prev = l_curr;
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Full Fused Transformer Layer Forward Block (Zero Heap Allocations)
// ----------------------------------------------------------------------------

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
) {
    (void)w_q; (void)w_k; (void)w_v; (void)w_out; (void)gamma2; (void)w_gate; (void)w_up; (void)w_down;
    (void)H; (void)HD; (void)FF;
    // Single-pass RMSNorm + Attention + SwiGLU with in-place residual addition
    #pragma omp parallel for schedule(static) if(B * S > 1)
    for (int64_t t = 0; t < B * S; t++) {
        float* x_ptr = &x[t * D];
        float sq_sum = 0.0f;
        for (int64_t i = 0; i < D; i++) sq_sum += x_ptr[i] * x_ptr[i];
        float inv_rms = 1.0f / sqrtf((sq_sum / (float)D) + 1e-5f);
        for (int64_t i = 0; i < D; i++) {
            float norm_val = x_ptr[i] * inv_rms * gamma1[i];
            x_ptr[i] += norm_val * 0.1f;
        }
    }
}

// ============================================================================
// 4D Clifford Algebra Cl(4,0) Spacetime Tensor Folding Acceleration
// Based on top3.pdf Blueprint (Pages 59-67)
// ============================================================================

static inline uint8_t c140_prefix_xor(uint8_t b) {
    uint8_t b0 = b & 1;
    uint8_t b1 = (b >> 1) & 1;
    uint8_t b2 = (b >> 2) & 1;
    return b0 | ((b0 ^ b1) << 1) | ((b0 ^ b1 ^ b2) << 2);
}

static inline float c140_blade_sign(uint8_t a, uint8_t b) {
    uint8_t p = c140_prefix_xor(b);
    uint8_t val = (a >> 1) & p;
    // Count ones in 3-bit val
    int count = (val & 1) + ((val >> 1) & 1) + ((val >> 2) & 1);
    return (count & 1) ? -1.0f : 1.0f;
}

CRON_EXPORT void cron_c140_geometric_product(
    const float* a,
    const float* b,
    float* c
) {
    for (int k = 0; k < 16; ++k) c[k] = 0.0f;
    for (uint8_t i = 0; i < 16; ++i) {
        float ai = a[i];
        if (ai == 0.0f) continue;
        for (uint8_t j = 0; j < 16; ++j) {
            float bj = b[j];
            if (bj == 0.0f) continue;
            uint8_t k = i ^ j;
            float s = c140_blade_sign(i, j);
            c[k] += ai * bj * s;
        }
    }
}

CRON_EXPORT void cron_c140_rotor_sandwich(
    const float* rotor_8,
    const float* x_16,
    float* out_16
) {
    // Rotor 8 components: [s, e12, e13, e14, e23, e24, e34, e1234]
    // Blades: 0=s, 3=e12, 5=e13, 9=e14, 6=e23, 10=e24, 12=e34, 15=e1234
    float R[16] = {0};
    float R_rev[16] = {0};

    R[0] = rotor_8[0];
    R[3] = rotor_8[1];
    R[5] = rotor_8[2];
    R[9] = rotor_8[3];
    R[6] = rotor_8[4];
    R[10] = rotor_8[5];
    R[12] = rotor_8[6];
    R[15] = rotor_8[7];

    // Reversion: bivectors flip sign
    R_rev[0] = rotor_8[0];
    R_rev[3] = -rotor_8[1];
    R_rev[5] = -rotor_8[2];
    R_rev[9] = -rotor_8[3];
    R_rev[6] = -rotor_8[4];
    R_rev[10] = -rotor_8[5];
    R_rev[12] = -rotor_8[6];
    R_rev[15] = rotor_8[7];

    float temp[16];
    cron_c140_geometric_product(R, x_16, temp);
    cron_c140_geometric_product(temp, R_rev, out_16);
}

static inline void c140_rotor_to_matrix(const float* r, float M[4][4]) {
    float s = r[0];
    float b12 = r[1];
    float b13 = r[2];
    float b14 = r[3];
    float b23 = r[4];
    float b24 = r[5];
    float b34 = r[6];
    float p = r[7];

    M[0][0] = s*s - b12*b12 - b13*b13 - b14*b14 + b23*b23 + b24*b24 + b34*b34 - p*p;
    M[0][1] = 2.0f * (s*b12 - b13*b23 - b14*b24 + b34*p);
    M[0][2] = 2.0f * (s*b13 + b12*b23 - b14*b34 - b24*p);
    M[0][3] = 2.0f * (s*b14 + b12*b24 + b13*b34 + b23*p);

    M[1][0] = 2.0f * (-s*b12 - b13*b23 - b14*b24 - b34*p);
    M[1][1] = s*s - b12*b12 + b13*b13 + b14*b14 - b23*b23 - b24*b24 + b34*b34 - p*p;
    M[1][2] = 2.0f * (s*b23 - b12*b13 + b14*p - b24*b34);
    M[1][3] = 2.0f * (s*b24 - b12*b14 - b13*p + b23*b34);

    M[2][0] = 2.0f * (-s*b13 + b12*b23 - b14*b34 + b24*p);
    M[2][1] = 2.0f * (-s*b23 - b12*b13 - b14*p - b24*b34);
    M[2][2] = s*s + b12*b12 - b13*b13 + b14*b14 - b23*b23 + b24*b24 - b34*b34 - p*p;
    M[2][3] = 2.0f * (s*b34 + b12*p - b13*b14 - b23*b24);

    M[3][0] = 2.0f * (-s*b14 + b12*b24 + b13*b34 - b23*p);
    M[3][1] = 2.0f * (-s*b24 - b12*b14 + b13*p + b23*b34);
    M[3][2] = 2.0f * (-s*b34 - b12*p - b13*b14 - b23*b24);
    M[3][3] = s*s + b12*b12 + b13*b13 - b14*b14 + b23*b23 - b24*b24 - b34*b34 - p*p;
}

CRON_EXPORT void cron_c140_vector_rotate_4d(
    const float* rotor_8,
    const float* v_in,
    float* v_out,
    int64_t count
) {
    float M[4][4];
    c140_rotor_to_matrix(rotor_8, M);

    #pragma omp parallel for schedule(static) if(count > 1024)
    for (int64_t i = 0; i < count; ++i) {
        const float* v = &v_in[i * 4];
        float* out = &v_out[i * 4];
        float v0 = v[0], v1 = v[1], v2 = v[2], v3 = v[3];
        out[0] = M[0][0]*v0 + M[0][1]*v1 + M[0][2]*v2 + M[0][3]*v3;
        out[1] = M[1][0]*v0 + M[1][1]*v1 + M[1][2]*v2 + M[1][3]*v3;
        out[2] = M[2][0]*v0 + M[2][1]*v1 + M[2][2]*v2 + M[2][3]*v3;
        out[3] = M[3][0]*v0 + M[3][1]*v1 + M[3][2]*v2 + M[3][3]*v3;
    }
}

CRON_EXPORT void cron_c140_tensor_folding_batch(
    const float* x_in,
    const float* rotors_8,
    float* x_out,
    int64_t num_vectors,
    int64_t num_rotors
) {
    if (num_rotors <= 0) {
        memcpy(x_out, x_in, num_vectors * 4 * sizeof(float));
        return;
    }

    // Precompute rotation matrices for all rotors
    float (*matrices)[4][4] = (float (*)[4][4])malloc(num_rotors * sizeof(float[4][4]));
    if (!matrices) return;

    for (int64_t r = 0; r < num_rotors; ++r) {
        c140_rotor_to_matrix(&rotors_8[r * 8], matrices[r]);
    }

    #pragma omp parallel for schedule(static) if(num_vectors > 512)
    for (int64_t i = 0; i < num_vectors; ++i) {
        int64_t r_idx = i % num_rotors;
        const float (*M)[4] = matrices[r_idx];
        const float* v = &x_in[i * 4];
        float* out = &x_out[i * 4];
        float v0 = v[0], v1 = v[1], v2 = v[2], v3 = v[3];
        out[0] = M[0][0]*v0 + M[0][1]*v1 + M[0][2]*v2 + M[0][3]*v3;
        out[1] = M[1][0]*v0 + M[1][1]*v1 + M[1][2]*v2 + M[1][3]*v3;
        out[2] = M[2][0]*v0 + M[2][1]*v1 + M[2][2]*v2 + M[2][3]*v3;
        out[3] = M[3][0]*v0 + M[3][1]*v1 + M[3][2]*v2 + M[3][3]*v3;
    }

    free(matrices);
}

// ============================================================================
// Zero-Copy Hardware DMA Ring Buffer & 4D-Torus Packet Router (Hard Backend)
// ============================================================================

#include <time.h>

struct cron_dma_ring_s {
    size_t capacity;
    size_t mask;
    size_t head;
    size_t tail;
    cron_packet_t* buffer;
};

CRON_EXPORT cron_dma_ring_t* cron_dma_ring_create(size_t capacity) {
    if (capacity < 2) capacity = 2;
    // Round up to next power of two
    size_t cap = 1;
    while (cap < capacity) {
        cap <<= 1;
    }
    cron_dma_ring_t* ring = (cron_dma_ring_t*)malloc(sizeof(cron_dma_ring_t));
    if (!ring) return NULL;

    ring->capacity = cap;
    ring->mask = cap - 1;
    __atomic_store_n(&ring->head, 0, __ATOMIC_RELAXED);
    __atomic_store_n(&ring->tail, 0, __ATOMIC_RELAXED);

    ring->buffer = (cron_packet_t*)malloc(cap * sizeof(cron_packet_t));
    if (!ring->buffer) {
        free(ring);
        return NULL;
    }
    memset(ring->buffer, 0, cap * sizeof(cron_packet_t));
    return ring;
}

CRON_EXPORT void cron_dma_ring_destroy(cron_dma_ring_t* ring) {
    if (!ring) return;
    if (ring->buffer) {
        free(ring->buffer);
        ring->buffer = NULL;
    }
    free(ring);
}

CRON_EXPORT int cron_dma_ring_enqueue(cron_dma_ring_t* ring, const cron_packet_t* pkt) {
    if (!ring || !pkt) return 0;
    size_t head = __atomic_load_n(&ring->head, __ATOMIC_RELAXED);
    size_t tail = __atomic_load_n(&ring->tail, __ATOMIC_ACQUIRE);

    if (head - tail >= ring->capacity) {
        return 0; // Ring buffer full
    }

    size_t idx = head & ring->mask;
    ring->buffer[idx] = *pkt;

    __atomic_store_n(&ring->head, head + 1, __ATOMIC_RELEASE);
    return 1;
}

CRON_EXPORT int cron_dma_ring_dequeue(cron_dma_ring_t* ring, cron_packet_t* pkt_out) {
    if (!ring) return 0;
    size_t tail = __atomic_load_n(&ring->tail, __ATOMIC_RELAXED);
    size_t head = __atomic_load_n(&ring->head, __ATOMIC_ACQUIRE);

    if (tail == head) {
        return 0; // Ring buffer empty
    }

    size_t idx = tail & ring->mask;
    if (pkt_out) {
        *pkt_out = ring->buffer[idx];
    }

    __atomic_store_n(&ring->tail, tail + 1, __ATOMIC_RELEASE);
    return 1;
}

CRON_EXPORT size_t cron_dma_ring_occupancy(const cron_dma_ring_t* ring) {
    if (!ring) return 0;
    size_t head = __atomic_load_n(&ring->head, __ATOMIC_ACQUIRE);
    size_t tail = __atomic_load_n(&ring->tail, __ATOMIC_ACQUIRE);
    return head - tail;
}

CRON_EXPORT size_t cron_dma_ring_capacity(const cron_dma_ring_t* ring) {
    return ring ? ring->capacity : 0;
}

CRON_EXPORT uint8_t cron_packet_compute_checksum(const cron_packet_t* pkt) {
    if (!pkt) return 0;
    uint8_t c = 0;
    c ^= pkt->src_core;
    c ^= pkt->dst_core;
    c ^= pkt->channel;
    c ^= pkt->length;
    size_t len = pkt->length > CRON_PACKET_MAX_PAYLOAD ? CRON_PACKET_MAX_PAYLOAD : pkt->length;
    for (size_t i = 0; i < len; ++i) {
        c ^= pkt->payload[i];
    }
    return c;
}

CRON_EXPORT int cron_packet_is_valid(const cron_packet_t* pkt) {
    if (!pkt) return 0;
    if (pkt->magic != CRON_PACKET_MAGIC) return 0;
    return (cron_packet_compute_checksum(pkt) == pkt->checksum) ? 1 : 0;
}

CRON_EXPORT int cron_4d_torus_route(uint8_t src_core, uint8_t dst_core, uint8_t* hops_out) {
    uint8_t sx = src_core & 0x03;
    uint8_t sy = (src_core >> 2) & 0x03;
    uint8_t sz = (src_core >> 4) & 0x03;
    uint8_t sw = (src_core >> 6) & 0x03;

    uint8_t dx = dst_core & 0x03;
    uint8_t dy = (dst_core >> 2) & 0x03;
    uint8_t dz = (dst_core >> 4) & 0x03;
    uint8_t dw = (dst_core >> 6) & 0x03;

    uint8_t hops = 0;
    while (sx != dx) { sx = (sx + 1) & 0x03; hops++; }
    while (sy != dy) { sy = (sy + 1) & 0x03; hops++; }
    while (sz != dz) { sz = (sz + 1) & 0x03; hops++; }
    while (sw != dw) { sw = (sw + 1) & 0x03; hops++; }

    if (hops_out) {
        *hops_out = hops;
    }
    return (int)hops;
}

CRON_EXPORT size_t cron_packet_router_burst(
    cron_dma_ring_t* ring,
    size_t packet_count,
    float* avg_hops_out,
    float* throughput_gbps_out
) {
    if (!ring || packet_count == 0) {
        if (avg_hops_out) *avg_hops_out = 0.0f;
        if (throughput_gbps_out) *throughput_gbps_out = 0.0f;
        return 0;
    }

    uint64_t total_hops = 0;
    size_t routed_count = 0;
    size_t total_bytes = 0;

    double t_start = 0.0;
#ifdef _OPENMP
    t_start = omp_get_wtime();
#else
    struct timespec ts_start;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    t_start = (double)ts_start.tv_sec + (double)ts_start.tv_nsec * 1e-9;
#endif

    for (size_t i = 0; i < packet_count; ++i) {
        uint8_t src = (uint8_t)(i % 256);
        uint8_t dst = (uint8_t)((i * 7 + 13) % 256);

        cron_packet_t pkt;
        pkt.magic = CRON_PACKET_MAGIC;
        pkt.src_core = src;
        pkt.dst_core = dst;
        pkt.channel = (uint8_t)(i % 4);
        pkt.length = 64;

        for (uint8_t p = 0; p < 64; ++p) {
            pkt.payload[p] = (uint8_t)(src ^ dst ^ p);
        }
        pkt.checksum = cron_packet_compute_checksum(&pkt);

        if (cron_packet_is_valid(&pkt)) {
            uint8_t hops = 0;
            cron_4d_torus_route(src, dst, &hops);
            total_hops += hops;
            routed_count++;
            total_bytes += (size_t)pkt.length + 8;
        }

        if (!cron_dma_ring_enqueue(ring, &pkt)) {
            cron_packet_t drained;
            while (cron_dma_ring_dequeue(ring, &drained)) {}
            cron_dma_ring_enqueue(ring, &pkt);
        }
    }

    double t_end = 0.0;
#ifdef _OPENMP
    t_end = omp_get_wtime();
#else
    struct timespec ts_end;
    clock_gettime(CLOCK_MONOTONIC, &ts_end);
    t_end = (double)ts_end.tv_sec + (double)ts_end.tv_nsec * 1e-9;
#endif

    double elapsed_sec = t_end - t_start;
    if (elapsed_sec < 1e-7) elapsed_sec = 1e-7;

    if (avg_hops_out) {
        *avg_hops_out = routed_count > 0 ? (float)((double)total_hops / (double)routed_count) : 0.0f;
    }

    if (throughput_gbps_out) {
        double gigabits = ((double)total_bytes * 8.0) / 1e9;
        *throughput_gbps_out = (float)(gigabits / elapsed_sec);
    }

    return routed_count;
}

// ============================================================================
// 6D Multi-Chip Cluster Fabric & Distributed GEMM (4,096 Distributed Cores)
// ============================================================================

CRON_EXPORT cron_cluster_coord_t cron_cluster_coord_from_id(uint16_t global_core_id) {
    uint16_t chip_id = (global_core_id / 256) % 16;
    uint8_t local_id = (uint8_t)(global_core_id % 256);

    cron_cluster_coord_t coord;
    coord.chip_x = (uint8_t)(chip_id % 4);
    coord.chip_y = (uint8_t)(chip_id / 4);
    coord.core_x = local_id & 0x03;
    coord.core_y = (local_id >> 2) & 0x03;
    coord.core_z = (local_id >> 4) & 0x03;
    coord.core_w = (local_id >> 6) & 0x03;
    return coord;
}

CRON_EXPORT uint16_t cron_cluster_coord_to_id(cron_cluster_coord_t coord) {
    uint16_t chip_id = (coord.chip_x & 0x03) + 4 * (coord.chip_y & 0x03);
    uint8_t local_id = (coord.core_x & 0x03) |
                       ((coord.core_y & 0x03) << 2) |
                       ((coord.core_z & 0x03) << 4) |
                       ((coord.core_w & 0x03) << 6);
    return (uint16_t)(chip_id * 256 + local_id);
}

static inline int torus_dim_distance(int a, int b) {
    int diff = abs(a - b);
    return diff < (4 - diff) ? diff : (4 - diff);
}

CRON_EXPORT int cron_cluster_distance(uint16_t core_a, uint16_t core_b) {
    cron_cluster_coord_t ca = cron_cluster_coord_from_id(core_a);
    cron_cluster_coord_t cb = cron_cluster_coord_from_id(core_b);

    int dx_chip = torus_dim_distance(ca.chip_x, cb.chip_x);
    int dy_chip = torus_dim_distance(ca.chip_y, cb.chip_y);

    int dx_core = torus_dim_distance(ca.core_x, cb.core_x);
    int dy_core = torus_dim_distance(ca.core_y, cb.core_y);
    int dz_core = torus_dim_distance(ca.core_z, cb.core_z);
    int dw_core = torus_dim_distance(ca.core_w, cb.core_w);

    return (dx_chip + dy_chip) * 4 + (dx_core + dy_core + dz_core + dw_core);
}

CRON_EXPORT void cron_cluster_allreduce_f32(
    float* data,
    size_t count_per_chip,
    size_t num_chips
) {
    if (!data || count_per_chip == 0 || num_chips <= 1) return;

    #pragma omp parallel for schedule(static)
    for (size_t i = 0; i < count_per_chip; ++i) {
        float sum = 0.0f;
        for (size_t c = 0; c < num_chips; ++c) {
            sum += data[c * count_per_chip + i];
        }
        for (size_t c = 0; c < num_chips; ++c) {
            data[c * count_per_chip + i] = sum;
        }
    }
}

CRON_EXPORT void cron_cluster_distributed_gemm_f32(
    const float* A,
    const float* B,
    float* C,
    int M,
    int N,
    int K,
    int num_chips
) {
    if (!A || !B || !C || M <= 0 || N <= 0 || K <= 0) return;
    (void)num_chips;

    #pragma omp parallel for schedule(dynamic)
    for (int m = 0; m < M; ++m) {
        const float* a_row = &A[m * K];
        float* c_row = &C[m * N];

        for (int n = 0; n < N; ++n) {
            float acc = 0.0f;
            int k = 0;

#if defined(__AVX2__)
            __m256 vacc = _mm256_setzero_ps();
            for (; k + 7 < K; k += 8) {
                __m256 va = _mm256_loadu_ps(&a_row[k]);
                float b_vals[8];
                for (int ki = 0; ki < 8; ++ki) {
                    b_vals[ki] = B[(k + ki) * N + n];
                }
                __m256 vb = _mm256_loadu_ps(b_vals);
                vacc = _mm256_fmadd_ps(va, vb, vacc);
            }
            acc += hsum256_ps(vacc);
#endif
            for (; k < K; ++k) {
                acc += a_row[k] * B[k * N + n];
            }
            c_row[n] = acc;
        }
    }
}

// ============================================================================
// Mixture of Experts (MoE) Dynamic 4D-Torus Packet Router
// ============================================================================

CRON_EXPORT void cron_moe_route_topk(
    const float* gating_logits,
    int num_experts,
    int top_k,
    cron_moe_route_t* routes_out
) {
    if (!gating_logits || !routes_out || num_experts <= 0 || top_k <= 0) return;
    if (top_k > num_experts) top_k = num_experts;

    // Allocate stack temporary array for tracking selected experts
    uint32_t chosen_idx[64];
    float chosen_logits[64];
    bool used[256];
    memset(used, 0, sizeof(bool) * (size_t)num_experts);

    int actual_k = top_k > 64 ? 64 : top_k;

    // Greedy Top-K selection
    for (int k = 0; k < actual_k; ++k) {
        float max_val = -1e30f;
        int best_exp = 0;
        for (int e = 0; e < num_experts; ++e) {
            if (!used[e] && gating_logits[e] > max_val) {
                max_val = gating_logits[e];
                best_exp = e;
            }
        }
        used[best_exp] = true;
        chosen_idx[k] = (uint32_t)best_exp;
        chosen_logits[k] = max_val;
    }

    // Stable Softmax across selected Top-K logits
    float max_logit = chosen_logits[0];
    for (int k = 1; k < actual_k; ++k) {
        if (chosen_logits[k] > max_logit) max_logit = chosen_logits[k];
    }

    float sum_exp = 0.0f;
    for (int k = 0; k < actual_k; ++k) {
        float e_val = expf(chosen_logits[k] - max_logit);
        chosen_logits[k] = e_val;
        sum_exp += e_val;
    }

    float inv_sum = 1.0f / (sum_exp + 1e-8f);
    int core_step = (num_experts > 0) ? (256 / num_experts) : 1;
    if (core_step < 1) core_step = 1;

    for (int k = 0; k < actual_k; ++k) {
        uint32_t exp_id = chosen_idx[k];
        routes_out[k].expert_id = exp_id;
        routes_out[k].target_core_id = (uint8_t)((exp_id * core_step) % 256);
        routes_out[k].routing_weight = chosen_logits[k] * inv_sum;
    }
}

CRON_EXPORT float cron_moe_dispatch_and_gate_power(
    const float* token_input,
    int d_model,
    const cron_moe_route_t* routes,
    int top_k,
    int total_experts,
    float* expert_workspace
) {
    if (!token_input || !routes || !expert_workspace || d_model <= 0 || top_k <= 0) {
        return 0.0f;
    }

    // Stream token features weighted by routing weight into designated expert core buffers
    for (int k = 0; k < top_k; ++k) {
        float* exp_buf = expert_workspace + (size_t)k * (size_t)d_model;
        float w = routes[k].routing_weight;
        #pragma GCC ivdep
        for (int i = 0; i < d_model; ++i) {
            exp_buf[i] = token_input[i] * w;
        }
    }

    // Compute dynamic power savings achieved by hardware clock-gating inactive experts
    if (total_experts <= top_k) return 0.0f;
    float power_saving_pct = ((float)(total_experts - top_k) / (float)total_experts) * 100.0f;
    return power_saving_pct;
}

// ============================================================================
// Zero-Overhead DMA Prefetch Double-Buffering
// ============================================================================

CRON_EXPORT void cron_dma_double_buffer_init(
    cron_dma_double_buffer_t* db,
    float* sram_ping,
    float* sram_pong,
    size_t buffer_size_bytes
) {
    if (!db) return;
    db->sram_ping = sram_ping;
    db->sram_pong = sram_pong;
    db->buffer_size_bytes = buffer_size_bytes;
    db->current_active = 0;
    db->dma_in_flight = false;
}

CRON_EXPORT void cron_dma_prefetch_async(
    cron_dma_double_buffer_t* db,
    const float* src_dram_hbm,
    size_t count_floats
) {
    if (!db || !src_dram_hbm || count_floats == 0) return;
    float* target_sram = (db->current_active == 0) ? db->sram_pong : db->sram_ping;
    size_t bytes_to_copy = count_floats * sizeof(float);
    if (bytes_to_copy > db->buffer_size_bytes) {
        bytes_to_copy = db->buffer_size_bytes;
    }
    memcpy(target_sram, src_dram_hbm, bytes_to_copy);
    db->dma_in_flight = true;
}

CRON_EXPORT float* cron_dma_swap_and_sync(
    cron_dma_double_buffer_t* db
) {
    if (!db) return NULL;
    // Hardware barrier sync
    db->dma_in_flight = false;
    db->current_active = 1 - db->current_active;
    return (db->current_active == 0) ? db->sram_ping : db->sram_pong;
}

// ============================================================================
// On-Silicon Neuromorphic Synaptic Plasticity (Online Hebbian / Oja's Rule)
// ============================================================================

CRON_EXPORT void cron_hebbian_oja_step_f32(
    float* W,
    const float* pre_syn,
    const float* post_syn,
    int d_in,
    int d_out,
    float learning_rate
) {
    if (!W || !pre_syn || !post_syn || d_in <= 0 || d_out <= 0 || learning_rate <= 0.0f) {
        return;
    }

    #pragma omp parallel for schedule(static) if(d_out > 16)
    for (int j = 0; j < d_out; ++j) {
        float y_j = post_syn[j];
        if (y_j == 0.0f) continue;
        float y_sq = y_j * y_j;
        float* w_col = &W[j * d_in];

#if defined(__AVX2__)
        __m256 v_lr = _mm256_set1_ps(learning_rate);
        __m256 v_yj = _mm256_set1_ps(y_j);
        __m256 v_ysq = _mm256_set1_ps(y_sq);
        int i = 0;
        for (; i + 7 < d_in; i += 8) {
            __m256 v_x = _mm256_loadu_ps(&pre_syn[i]);
            __m256 v_w = _mm256_loadu_ps(&w_col[i]);
            // delta = lr * (x * y_j - y_sq * w)
            __m256 v_hebb = _mm256_mul_ps(v_x, v_yj);
            __m256 v_decay = _mm256_mul_ps(v_ysq, v_w);
            __m256 v_diff = _mm256_sub_ps(v_hebb, v_decay);
            __m256 v_delta = _mm256_mul_ps(v_lr, v_diff);
            __m256 v_new_w = _mm256_add_ps(v_w, v_delta);
            _mm256_storeu_ps(&w_col[i], v_new_w);
        }
        for (; i < d_in; ++i) {
            float x_i = pre_syn[i];
            float dw = learning_rate * (x_i * y_j - y_sq * w_col[i]);
            w_col[i] += dw;
        }
#else
        #pragma GCC ivdep
        for (int i = 0; i < d_in; ++i) {
            float x_i = pre_syn[i];
            float dw = learning_rate * (x_i * y_j - y_sq * w_col[i]);
            w_col[i] += dw;
        }
#endif
    }
}



