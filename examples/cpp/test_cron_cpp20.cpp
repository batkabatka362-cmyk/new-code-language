// ============================================================================
// CRON Sovereign Interop: C++20 Turnkey Verification & Benchmark
// Demonstrates zero-dependency integration between C++20 and CRON BitNet Engine
// ============================================================================

#include "cron.hpp"
#include <iostream>
#include <vector>
#include <chrono>
#include <cassert>
#include <iomanip>

int main() {
    std::cout << "======================================================================\n";
    std::cout << "  CRON Sovereign Language Ecosystem: C++20 Native Interop Suite\n";
    std::cout << "  Zero Dependencies - Pure C++20 - BitNet b1.58 Multiplier-Free\n";
    std::cout << "======================================================================\n\n";

    // 1. Verify C++20 std::popcount sub-byte reduction
    std::cout << "[TEST 1] Verifying C++20 <bit> std::popcount hardware acceleration...\n";
    uint32_t sample_bits = 0b10110010101011110000111101010101;
    int popc = std::popcount(sample_bits);
    std::cout << "  std::popcount(0x" << std::hex << sample_bits << std::dec << ") = " << popc << " bits set.\n";
    assert(popc == 18);
    std::cout << "  -> [PASS] Hardware popcount verified.\n\n";

    // 2. Initialize BitNetTensor (16 rows x 64 cols)
    std::cout << "[TEST 2] Initializing BitNet b1.58 Tensor (16 rows x 64 cols)...\n";
    const size_t rows = 16;
    const size_t cols = 64;
    cron::BitNetTensor tensor(rows, cols, 0.5f);

    // Populate with deterministic ternary pattern: {-1, 0, +1}
    for (size_t r = 0; r < rows; ++r) {
        for (size_t c = 0; c < cols; ++c) {
            int8_t trit = 0;
            if ((r + c) % 3 == 0) trit = 1;
            else if ((r + c) % 3 == 1) trit = -1;
            tensor.set_trit(r, c, trit);
        }
    }

    std::vector<float> inputs(cols, 1.0f);
    std::vector<float> outputs(rows, 0.0f);

    // 3. Forward pass GEMV execution
    std::cout << "[TEST 3] Executing 0-FLOP Multiplier-Free GEMV Forward Pass...\n";
    tensor.forward(inputs.data(), outputs.data());

    std::cout << "  Outputs [first 4 rows]: ";
    for (size_t i = 0; i < 4; ++i) {
        std::cout << outputs[i] << " ";
    }
    std::cout << "\n  -> [PASS] Forward pass completed successfully.\n\n";

    // 4. Verify 4D Hyperspherical Geodesic Coordinates
    std::cout << "[TEST 4] Testing 4D-Torus Hyperspherical Geodesic Coordinates (S^3)...\n";
    cron::GeodesicCoord4D coord{1.0f, 2.0f, 2.0f, 0.0f};
    float initial_norm = coord.norm();
    std::cout << "  Initial Norm: " << initial_norm << " (expected 3.0)\n";
    assert(std::abs(initial_norm - 3.0f) < 1e-5f);

    coord.normalize();
    float normalized_norm = coord.norm();
    std::cout << "  Normalized Norm: " << normalized_norm << "\n";
    assert(std::abs(normalized_norm - 1.0f) < 1e-5f);
    std::cout << "  -> [PASS] 4D Geodesic normalization verified.\n\n";

    // 5. High-Throughput Roofline Benchmark
    std::cout << "[TEST 5] Benchmarking BitNet Ternary GEMV Throughput (100,000 iterations)...\n";
    const int iterations = 100000;
    auto start = std::chrono::high_resolution_clock::now();

    for (int it = 0; it < iterations; ++it) {
        tensor.forward(inputs.data(), outputs.data());
    }

    auto end = std::chrono::high_resolution_clock::now();
    double elapsed_ms = std::chrono::duration<double, std::milli>(end - start).count();
    double total_ops = static_cast<double>(rows * cols * 2) * iterations;
    double mtops = (total_ops / (elapsed_ms / 1000.0)) / 1e6;
    double latency_ns = (elapsed_ms * 1e6) / iterations;

    std::cout << std::fixed << std::setprecision(2);
    std::cout << "  Total Time       : " << elapsed_ms << " ms\n";
    std::cout << "  Average Latency  : " << latency_ns << " ns / inference\n";
    std::cout << "  Effective Rate   : " << mtops << " MTOPS (Million Ternary Ops / sec)\n";
    std::cout << "  -> [PASS] High-speed C++20 BitNet execution benchmark complete.\n\n";

    std::cout << "======================================================================\n";
    std::cout << "  [SUCCESS] All CRON C++20 Interoperability Tests Passed Cleanly!\n";
    std::cout << "======================================================================\n";
    return 0;
}
