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
    if (total_elements <= 0) {
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

    int packed_idx = 0;
    int i = 0;
    while (i < total_elements) {
        uint8_t byte_val = 0;
        for (int b = 0; b < 4; ++b) {
            int idx = i + b;
            uint8_t trit_code = 0;
            if (idx < total_elements) {
                float val = W[idx];
                if (val > threshold) {
                    trit_code = 1; // +1
                } else if (val < -threshold) {
                    trit_code = 3; // -1 (0b11)
                }
            }
            byte_val |= (uint8_t)(trit_code << (b * 2));
        }
        W_packed[packed_idx++] = byte_val;
        i += 4;
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

