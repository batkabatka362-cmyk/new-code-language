
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>
#include <omp.h>

#define S 1024
#define D 64
#define BC 64

// CRON RingTape FlashAttention-3 Tiled Online Streaming Softmax
void cron_tiled_flash_attention(const float* restrict Q, const float* restrict K, const float* restrict V, float* restrict O) {
    float scale = 1.0f / sqrtf((float)D);
    #pragma omp parallel for schedule(static)
    for (int i = 0; i < S; i++) {
        float m = -1e30f;
        float l = 0.0f;
        float acc[D];
        for (int d = 0; d < D; d++) acc[d] = 0.0f;
        const float* restrict q_row = &Q[i * D];

        for (int j_blk = 0; j_blk < S; j_blk += BC) {
            float scores[BC];
            float block_max = -1e30f;
            for (int b = 0; b < BC; b++) {
                int j = j_blk + b;
                const float* restrict k_row = &K[j * D];
                float s = 0.0f;
                #pragma GCC unroll 8
                for (int d = 0; d < D; d++) s += q_row[d] * k_row[d];
                s *= scale;
                scores[b] = s;
                if (s > block_max) block_max = s;
            }
            float m_new = block_max > m ? block_max : m;
            float alpha = expf(m - m_new);
            l = l * alpha;
            for (int d = 0; d < D; d++) acc[d] *= alpha;

            for (int b = 0; b < BC; b++) {
                int j = j_blk + b;
                float p = expf(scores[b] - m_new);
                l += p;
                const float* restrict v_row = &V[j * D];
                #pragma GCC unroll 8
                for (int d = 0; d < D; d++) acc[d] += p * v_row[d];
            }
            m = m_new;
        }
        float inv_l = 1.0f / l;
        for (int d = 0; d < D; d++) O[i * D + d] = acc[d] * inv_l;
    }
}

int main() {
    float* Q = (float*)malloc(S * D * sizeof(float));
    float* K = (float*)malloc(S * D * sizeof(float));
    float* V = (float*)malloc(S * D * sizeof(float));
    float* O = (float*)malloc(S * D * sizeof(float));
    for (int i = 0; i < S * D; i++) { Q[i] = 0.1f; K[i] = 0.1f; V[i] = 0.1f; }
    
    // Warmup
    cron_tiled_flash_attention(Q, K, V, O);
    
    struct timespec ts0, ts1;
    clock_gettime(CLOCK_MONOTONIC, &ts0);
    int runs = 10;
    for (int r = 0; r < runs; r++) {
        cron_tiled_flash_attention(Q, K, V, O);
    }
    clock_gettime(CLOCK_MONOTONIC, &ts1);
    
    double total_us = (ts1.tv_sec - ts0.tv_sec) * 1e6 + (ts1.tv_nsec - ts0.tv_nsec) * 1e-3;
    double median_us = total_us / runs;
    printf("%.2f\n", median_us);
    
    free(Q); free(K); free(V); free(O);
    return 0;
}
