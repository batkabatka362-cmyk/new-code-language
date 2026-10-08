#include <iostream>
#include <cassert>
#include <vector>
#include "../include/cron_engine.hpp"

int main() {
    std::cout << "====================================================================" << std::endl;
    std::cout << "🚀 TESTING CRON PURE C++20 AI INFERENCE ENGINE (GRADE SSS+)" << std::endl;
    std::cout << "====================================================================" << std::endl;

    cron::ModelConfig cfg;
    cfg.vocab_size = 1000;
    cfg.d_model = 128;
    cfg.n_heads = 4;
    cfg.n_layers = 2;
    cfg.mlp_dim = 256;
    cfg.max_seq_len = 512;
    cfg.num_experts = 8;
    cfg.top_k_experts = 2;

    // -------------------------------------------------------------------------
    // TEST 1: Transformer Block Construction & Single Token Forward Pass
    // -------------------------------------------------------------------------
    std::cout << "[TEST 1] Testing cron::TransformerBlock forward pass..." << std::endl;
    cron::TransformerBlock block(cfg);

    std::vector<float> token_in(cfg.d_model, 0.5f);
    std::vector<float> token_out(cfg.d_model, 0.0f);

    block.forward_token(token_in, token_out);

    float norm_sq = 0.0f;
    for (float v : token_out) {
        assert(!std::isnan(v));
        assert(!std::isinf(v));
        norm_sq += v * v;
    }
    assert(norm_sq > 0.0f);
    std::cout << "  ✓ Test 1 Passed: TransformerBlock forward executed cleanly (Norm: " << std::sqrt(norm_sq) << ")." << std::endl;

    // -------------------------------------------------------------------------
    // TEST 2: Full Inference Engine Construction & Autoregressive Step
    // -------------------------------------------------------------------------
    std::cout << "[TEST 2] Testing cron::InferenceEngine step_token & sampling..." << std::endl;
    cron::InferenceEngine engine(cfg);

    std::vector<float> logits(cfg.vocab_size, 0.0f);
    engine.step_token(token_in, logits);

    int sampled_token = engine.sample_argmax(logits);
    assert(sampled_token >= 0 && sampled_token < cfg.vocab_size);
    std::cout << "  ✓ Test 2 Passed: Full engine step computed logits, sampled token = " << sampled_token << "." << std::endl;

    // -------------------------------------------------------------------------
    // TEST 3: Streaming Autoregressive Token Generation Benchmark
    // -------------------------------------------------------------------------
    std::cout << "[TEST 3] Benchmarking C++20 streaming generation..." << std::endl;
    auto bench = engine.benchmark_generation(50);
    std::cout << "  ✓ Tokens Generated     : " << bench.tokens_generated << std::endl;
    std::cout << "  ✓ Total Time           : " << bench.total_time_ms << " ms" << std::endl;
    std::cout << "  ✓ Throughput           : " << bench.tokens_per_sec << " tokens/sec" << std::endl;
    std::cout << "  ✓ Latency per Token    : " << bench.time_per_token_ms << " ms" << std::endl;
    std::cout << "  ✓ 4-Bit KV Cache Memory: " << bench.kv_cache_memory_kb << " KB" << std::endl;
    assert(bench.tokens_per_sec > 0.0);
    std::cout << "  ✓ Test 3 Passed: C++ generation benchmark succeeded." << std::endl;

    // -------------------------------------------------------------------------
    // TEST 4: Inline CL Execution on 4D-Torus VM from C++
    // -------------------------------------------------------------------------
    std::cout << "[TEST 4] Testing InlineCL execution on 4D-Torus Silicon..." << std::endl;
    std::string cl_code = R"(
    B0000: '==01#010> '==02#020> '==03#030> '==04#040>
    B0001: _SZ05$081> _SZ06$0B2> _SZ07$0A3> ~PF03@210>
    B0002: _TX05$1B0> _TX06$110> _TX07$170> _PK08`270>
    B0003: _SB00#060> _NO00#000> _NO00#000> _NO00#000>
    )";

    auto telemetry = cron::InlineCL::run_on_torus(cl_code);
    assert(telemetry.success);
    assert(telemetry.active_cores > 0);
    std::cout << "  ✓ Test 4 Passed: Inline CL executed on 256-Core 4D-Torus (Cycles: "
              << telemetry.total_cycles << ", Active Cores: " << telemetry.active_cores << ")." << std::endl;

    std::cout << "\n🎉 ALL CRON C++20 AI INFERENCE ENGINE TESTS PASSED (GRADE SSS+)!" << std::endl;
    return 0;
}
