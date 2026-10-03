#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <omp.h>
#define M 1024
#define K 1024
#define N 1024
void gemm(const float* restrict A, const int8_t* restrict B, float* restrict C) {
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
    float* A = malloc(M*K*4); int8_t* B = malloc(K*N); float* C = malloc(M*N*4);
    gemm(A,B,C);
    struct timespec t0, t1; clock_gettime(CLOCK_MONOTONIC, &t0);
    for(int r=0; r<10; r++) gemm(A,B,C);
    clock_gettime(CLOCK_MONOTONIC, &t1);
    printf('%.2f us\n', ((t1.tv_sec-t0.tv_sec)*1e6 + (t1.tv_nsec-t0.tv_nsec)*1e-3)/10.0);
}