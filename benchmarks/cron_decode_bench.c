
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define DIM 512

// Sub-microsecond Single Token Autoregressive Contraction with Contiguous Row Stride
void cron_decode_token(const float* restrict x, const int8_t* restrict W, float* restrict y) {
    for (int col = 0; col < DIM; col++) y[col] = 0.0f;
    for (int p = 0; p < DIM; p++) {
        float x_val = x[p];
        const int8_t* restrict W_row = &W[p * DIM];
        #pragma GCC ivdep
        for (int col = 0; col < DIM; col++) {
            y[col] += x_val * (float)W_row[col];
        }
    }
}

int main() {
    float* x = (float*)malloc(DIM * sizeof(float));
    int8_t* W = (int8_t*)malloc(DIM * DIM * sizeof(int8_t));
    float* y = (float*)malloc(DIM * sizeof(float));
    for (int i = 0; i < DIM; i++) x[i] = 1.0f;
    for (int i = 0; i < DIM * DIM; i++) W[i] = (i % 3) - 1;
    
    // Warmup
    for (int i = 0; i < 20; i++) cron_decode_token(x, W, y);
    
    struct timespec ts0, ts1;
    clock_gettime(CLOCK_MONOTONIC, &ts0);
    int runs = 1000;
    for (int r = 0; r < runs; r++) {
        cron_decode_token(x, W, y);
    }
    clock_gettime(CLOCK_MONOTONIC, &ts1);
    
    double total_us = (ts1.tv_sec - ts0.tv_sec) * 1e6 + (ts1.tv_nsec - ts0.tv_nsec) * 1e-3;
    double median_us = total_us / runs;
    printf("%.4f\n", median_us);
    
    free(x); free(W); free(y);
    return 0;
}
