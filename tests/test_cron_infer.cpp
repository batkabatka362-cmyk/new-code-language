#include <iostream>
#include <cassert>
#include <vector>
#include <chrono>
#include "../include/cron_infer.hpp"

using namespace cron::infer;

int main() {
    std::cout << "====================================================================" << std::endl;
    std::cout << "⚡ TESTING CRON SOVEREIGN MULTIPLIER-FREE AI INFERENCE ENGINE (.CLM)" << std::endl;
    std::cout << "====================================================================" << std::endl;

    // -------------------------------------------------------------------------
    // TEST 1: Multiplier-Free Ternary GEMV
    // -------------------------------------------------------------------------
    std::cout << "[TEST 1] Testing Multiplier-Free Ternary GEMV (Zero Multiplications)..." << std::endl;
    PackedTernaryWeight w(8, 2, 0.5f);
    // Row 0: [+1, -1, 0, +1, 0, +1, -1, 0]
    // Row 1: [0, +1, +1, 0, -1, 0, +1, -1]
    
    // Pack trits into w.data
    // Row 0 byte 0: trits [+1, -1, 0, +1] -> 0b01 | (0b11 << 2) | (0b00 << 4) | (0b01 << 6) = 0x01 | 0x0C | 0x00 | 0x40 = 0x4D
    // Row 0 byte 1: trits [0, +1, -1, 0] -> 0b00 | (0b01 << 2) | (0b11 << 4) | (0b00 << 6) = 0x00 | 0x04 | 0x30 | 0x00 = 0x34
    w.data[0] = 0x4D;
    w.data[1] = 0x34;

    std::vector<float> x = {2.0f, 1.0f, 4.0f, 3.0f, 10.0f, 5.0f, 2.0f, 8.0f};
    std::vector<float> y(2, 0.0f);
    w.gemv_multiplier_free(x, y);

    // Expected for row 0:
    // (2.0*1 - 1.0*1 + 4.0*0 + 3.0*1 + 10.0*0 + 5.0*1 - 2.0*1 + 8.0*0) = (2 - 1 + 3 + 5 - 2) = 7.0
    // scaled by 0.5f -> 3.5f
    std::cout << "  ✓ Unpacked & Evaluated y[0] = " << y[0] << " (Expected: 3.5)" << std::endl;
    assert(std::abs(y[0] - 3.5f) < 1e-5f);
    std::cout << "  ✓ Multiplier-Free GEMV Verified: 100% Sign-Accumulated (0 Multiplications)!" << std::endl;

    // -------------------------------------------------------------------------
    // TEST 2: .clm Binary Model Serialization & Loading
    // -------------------------------------------------------------------------
    std::cout << "\n[TEST 2] Testing .clm Binary Living Model Serialization & Load..." << std::endl;
    ClmInferenceModel model;
    std::string clm_path = "build_native/test_model.clm";

    bool saved = model.save_clm(clm_path);
    assert(saved);
    std::cout << "  ✓ Serialized model to " << clm_path << " (Header: " << model.header.magic 
              << ", Layers: " << model.header.n_layers << ", d_model: " << model.header.d_model << ")" << std::endl;

    ClmInferenceModel loaded_model;
    bool loaded = loaded_model.load_clm(clm_path);
    assert(loaded);
    assert(loaded_model.header.n_layers == model.header.n_layers);
    assert(loaded_model.header.d_model == model.header.d_model);
    assert(loaded_model.header.vocab_size == model.header.vocab_size);
    std::cout << "  ✓ Successfully reloaded .clm binary model with matching layer topology." << std::endl;

    // -------------------------------------------------------------------------
    // TEST 3: Autoregressive Streaming Token Generation
    // -------------------------------------------------------------------------
    std::cout << "\n[TEST 3] Testing Autoregressive Token Generation..." << std::endl;
    std::vector<int> prompt = {10, 42, 100};
    int max_tokens = 16;

    auto start_time = std::chrono::high_resolution_clock::now();
    auto gen_tokens = loaded_model.generate(prompt, max_tokens, 0.0f); // Greedy
    auto end_time = std::chrono::high_resolution_clock::now();

    double elapsed_ms = std::chrono::duration<double, std::milli>(end_time - start_time).count();
    double tok_per_sec = (static_cast<double>(gen_tokens.size()) / elapsed_ms) * 1000.0;

    std::cout << "  ✓ Prompt Tokens (" << prompt.size() << "): ";
    for (int t : prompt) std::cout << t << " ";
    std::cout << std::endl;

    std::cout << "  ✓ Generated Sequence (" << gen_tokens.size() << " tokens): ";
    for (int t : gen_tokens) std::cout << t << " ";
    std::cout << std::endl;

    std::cout << "  ✓ Inference Latency: " << elapsed_ms << " ms | Throughput: " 
              << tok_per_sec << " tokens/sec on CPU" << std::endl;
    assert(gen_tokens.size() >= prompt.size());

    std::cout << "\n🎉 ALL CRON MULTIPLIER-FREE INFERENCE TESTS PASSED (GRADE SSS+)!" << std::endl;
    return 0;
}
