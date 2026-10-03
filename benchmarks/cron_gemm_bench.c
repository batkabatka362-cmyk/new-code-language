
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <omp.h>

#define M 1024
#define K 1024
#define N 1024

// CRON BitNet b1.58 Multi-Core Contraction with AVX2/AVX-512 Vector FMA
void cron_ternary_gemm(const float* restrict A, const int8_t* restrict B, float* restrict C) {
    #pragma omp parallel for schedule(static)
    for (int i = 0; i < M; i++) {
        float* restrict C_row = &C[i * N];
        for (int j = 0; j < N; j++) C_row[j] = 0.0f;
        for (int p = 0; p < K; p++) {
            float a_val = A[i * K + p];
            const int8_t* restrict B_row = &B[p * N];
            #pragma GCC ivdep
            for (int j = 0; j < N; j++) {
                C_row[j] += a_val * (float)B_row[j];
            }
        }
    }
}

int main() {
    float* A = (float*)malloc(M * K * sizeof(float));
    int8_t* B = (int8_t*)malloc(K * N * sizeof(int8_t));
    float* C = (float*)malloc(M * N * sizeof(float));
    for (int i = 0; i < M * K; i++) A[i] = 1.0f;
    for (int i = 0; i < K * N; i++) B[i] = (i % 3) - 1;
    
    // Warmup
    cron_ternary_gemm(A, B, C);
    
    struct timespec ts0, ts1;
    clock_gettime(CLOCK_MONOTONIC, &ts0);
    int runs = 10;
    for (int r = 0; r < runs; r++) {
        cron_ternary_gemm(A, B, C);
    }
    clock_gettime(CLOCK_MONOTONIC, &ts1);
    
    double total_us = (ts1.tv_sec - ts0.tv_sec) * 1e6 + (ts1.tv_nsec - ts0.tv_nsec) * 1e-3;
    double median_us = total_us / runs;
    printf("%.2f\n", median_us);
    
    free(A); free(B); free(C);
    return 0;
}
