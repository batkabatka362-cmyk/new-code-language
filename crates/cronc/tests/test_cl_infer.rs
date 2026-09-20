use cronc::{
    generate_tokens, synthesize_transformer_cl, transformer_forward_step,
    verify_cl_program, run_cl_jit,
    KVCache, TransformerConfig,
};

#[test]
fn test_transformer_forward_pass_logits() {
    let config = TransformerConfig {
        vocab_size: 128,
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 4,
        head_dim: 8,
        intermediate_dim: 64,
        max_seq_len: 64,
        is_ternary_bitnet: true,
        enable_2_4_sparsity: true,
    };

    let mut kv_cache = KVCache::new(config.num_layers, config.max_seq_len, config.hidden_dim);
    let logits = transformer_forward_step(65, 0, &config, &mut kv_cache);

    assert_eq!(logits.len(), 128);
    for &l in &logits {
        assert!(!l.is_nan(), "Logit must not be NaN");
        assert!(!l.is_infinite(), "Logit must be finite");
    }
}

#[test]
fn test_autoregressive_generation_kv_cache() {
    let config = TransformerConfig {
        vocab_size: 128,
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 2,
        head_dim: 16,
        intermediate_dim: 64,
        max_seq_len: 64,
        is_ternary_bitnet: true,
        enable_2_4_sparsity: true,
    };

    let res = generate_tokens("CRON", 6, 0.0, &config);

    assert_eq!(res.prompt, "CRON");
    assert_eq!(res.telemetry.prompt_tokens, 4);
    assert_eq!(res.telemetry.generated_tokens, 6);
    assert_eq!(res.telemetry.total_tokens, 10);
    assert_eq!(res.token_ids.len(), 10);
    assert!(res.telemetry.time_to_first_token_us > 0.0);
    assert!(res.telemetry.inter_token_latency_us > 0.0);
    assert!(res.telemetry.tokens_per_second > 0.0);
    assert!(res.telemetry.energy_per_token_pj > 0.0);
}

#[test]
fn test_cl_transformer_microcode_synthesis_and_jit() {
    let config = TransformerConfig {
        vocab_size: 64,
        hidden_dim: 16,
        num_layers: 2,
        num_heads: 2,
        head_dim: 8,
        intermediate_dim: 32,
        max_seq_len: 32,
        is_ternary_bitnet: true,
        enable_2_4_sparsity: true,
    };

    let cl_code = synthesize_transformer_cl(&config);
    assert!(cl_code.contains("CRON COMPLETE LLM TRANSFORMER INFERENCE MICRO-KERNEL"));

    // Verify .cl syntax and CRC-8 integrity
    let cl_rep = verify_cl_program(&cl_code);
    assert!(cl_rep.is_ok(), "Synthesized transformer microcode must be valid .cl: {:?}", cl_rep.err());

    // Execute via JIT engine
    let jit_res = run_cl_jit(&cl_code);
    assert!(jit_res.is_ok(), "JIT execution failed: {:?}", jit_res.err());
}

#[test]
fn test_cl_infer_reports_and_json() {
    let config = TransformerConfig::default();
    let res = generate_tokens("AI", 4, 0.7, &config);

    // Test ASCII dashboard
    let ascii = res.render_ascii_dashboard();
    assert!(ascii.contains("CRON 4D-TORUS LLM TRANSFORMER INFERENCE DASHBOARD"));
    assert!(ascii.contains("Time to First Token"));
    assert!(ascii.contains("Inference Throughput"));

    // Test JSON serialization
    let json = res.to_json();
    assert!(json.contains("\"prompt\": \"AI\""));
    assert!(json.contains("\"total_tokens\":"));
    assert!(json.contains("\"ttft_us\":"));
    assert!(json.contains("\"tokens_per_second\":"));
}
