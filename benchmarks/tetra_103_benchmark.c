#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#ifdef _WIN32
#include <windows.h>
static inline uint64_t get_time_ns(void) {
    static LARGE_INTEGER freq;
    static int initialized = 0;
    if (!initialized) {
        QueryPerformanceFrequency(&freq);
        initialized = 1;
    }
    LARGE_INTEGER counter;
    QueryPerformanceCounter(&counter);
    return (uint64_t)((double)counter.QuadPart * 1000000000.0 / (double)freq.QuadPart);
}
#else
#include <time.h>
static inline uint64_t get_time_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC_RAW, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}
#endif

#define NUM_BLOCKS 100000
#define NUM_WARMUP 10
#define NUM_RUNS 50

typedef struct {
    uint32_t edge_planes[6];
    uint8_t  micro_gain;
    uint8_t  pad[7];
} TetraBlock;

// Prevent compiler from optimizing away the loop
__attribute__((noinline))
int64_t benchmark_tetra_dot_product(const TetraBlock* restrict weights, 
                                    const TetraBlock* restrict acts, 
                                    size_t num_blocks) {
    int64_t accumulator = 0;
    for (size_t b = 0; b < num_blocks; b++) {
        uint32_t m0 = ~(weights[b].edge_planes[0] ^ acts[b].edge_planes[0]);
        uint32_t m1 = ~(weights[b].edge_planes[1] ^ acts[b].edge_planes[1]);
        uint32_t m2 = ~(weights[b].edge_planes[2] ^ acts[b].edge_planes[2]);
        uint32_t m3 = ~(weights[b].edge_planes[3] ^ acts[b].edge_planes[3]);
        uint32_t m4 = ~(weights[b].edge_planes[4] ^ acts[b].edge_planes[4]);
        uint32_t m5 = ~(weights[b].edge_planes[5] ^ acts[b].edge_planes[5]);

        int match = __builtin_popcount(m0) + __builtin_popcount(m1) +
                    __builtin_popcount(m2) + __builtin_popcount(m3) +
                    __builtin_popcount(m4) + __builtin_popcount(m5);

        accumulator += (int64_t)(match - 96) << (weights[b].micro_gain >> 1);
    }
    return accumulator;
}

int main(void) {
    printf("========================================================================\n");
    printf("  1.03125-BIT TETRAHEDRON SIMPLEX VLIW BENCHMARK (NANO-SECOND HARDWARE) \n");
    printf("  Theory: 32 Weights + 4-bit Shared Micro-Gain (Gamma in [0..15])       \n");
    printf("========================================================================\n\n");

    size_t num_blocks = NUM_BLOCKS;
    TetraBlock* weights = (TetraBlock*)malloc(num_blocks * sizeof(TetraBlock));
    TetraBlock* acts = (TetraBlock*)malloc(num_blocks * sizeof(TetraBlock));

    // Initialize with varied bit patterns
    for (size_t i = 0; i < num_blocks; i++) {
        weights[i].edge_planes[0] = 0xAAAAAAAA ^ (uint32_t)i;
        weights[i].edge_planes[1] = 0x55555555 + (uint32_t)(i * 3);
        weights[i].edge_planes[2] = 0x33333333 ^ (uint32_t)(i << 2);
        weights[i].edge_planes[3] = 0x0F0F0F0F + (uint32_t)(i * 7);
        weights[i].edge_planes[4] = 0x00FF00FF ^ (uint32_t)(i >> 1);
        weights[i].edge_planes[5] = 0x5A5A5A5A + (uint32_t)(i * 13);
        weights[i].micro_gain = (uint8_t)(i % 16);

        acts[i].edge_planes[0] = 0xFFFFFFFF - (uint32_t)i;
        acts[i].edge_planes[1] = 0x12345678 ^ (uint32_t)(i * 5);
        acts[i].edge_planes[2] = 0x87654321 + (uint32_t)(i << 3);
        acts[i].edge_planes[3] = 0x55AA55AA ^ (uint32_t)(i * 11);
        acts[i].edge_planes[4] = 0xAA55AA55 + (uint32_t)i;
        acts[i].edge_planes[5] = 0x01234567 ^ (uint32_t)(i * 17);
        acts[i].micro_gain = (uint8_t)((i + 3) % 16);
    }

    // Warmup
    for (int w = 0; w < NUM_WARMUP; w++) {
        benchmark_tetra_dot_product(weights, acts, num_blocks);
    }

    printf("[*] Хэмжилтийг эхлүүлж байна: %d блок (%d жин) x %d удаа...\n", 
           (int)num_blocks, (int)(num_blocks * 32), NUM_RUNS);

    uint64_t start_ns = get_time_ns();
    int64_t acc = 0;
    for (int r = 0; r < NUM_RUNS; r++) {
        acc += benchmark_tetra_dot_product(weights, acts, num_blocks);
    }
    uint64_t end_ns = get_time_ns();

    uint64_t total_ns = end_ns - start_ns;
    double total_sec = (double)total_ns / 1e9;
    double total_weights = (double)num_blocks * 32.0 * (double)NUM_RUNS;
    double total_blocks = (double)num_blocks * (double)NUM_RUNS;
    double ns_per_block = (double)total_ns / total_blocks;
    double ns_per_weight = ns_per_block / 32.0;
    double weights_per_sec = total_weights / total_sec;
    double giga_ops = weights_per_sec / 1e9;

    printf("\n--- БОДИТ ҮР ДҮН ---\n");
    printf("Нийт боловсруулсан 1.03-бит жингийн тоо: %.0f жин\n", total_weights);
    printf("Нийт зарцуулсан хугацаа:                 %.4f секунд (%.2f мс)\n", total_sec, total_sec * 1000.0);
    printf("1 ширхэг 32-жингийн Тетраэдр блок:       %.3f нано-секунд\n", ns_per_block);
    printf("Нэг жинд ногдох дундаж хугацаа:          %.4f нано-секунд\n", ns_per_weight);
    printf("Тооцооллын хурд (Throughput):            %.2f САЯ Жин/Секунд (%.3f Giga-Weights/sec)\n", weights_per_sec / 1e6, giga_ops);
    printf("Шалгалтын хяналтын утга (Accumulator):   %lld\n", (long long)acc);
    printf("========================================================================\n");

    free(weights);
    free(acts);
    return 0;
}
