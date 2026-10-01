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

#[test]
fn test_flash_attention_3_tile_forward() {
    use cronc::flash_attention_3_tile_forward;

    let head_dim = 16;
    let num_heads = 4;
    let hidden_dim = head_dim * num_heads; // 64
    let seq_len = 8;
    let tile_size = 4;

    let q = vec![0.5f64; hidden_dim];
    let k_cache = vec![0.25f64; seq_len * hidden_dim];
    let v_cache = vec![0.75f64; seq_len * hidden_dim];

    let out = flash_attention_3_tile_forward(
        &q,
        &k_cache,
        &v_cache,
        head_dim,
        num_heads,
        hidden_dim,
        seq_len,
        tile_size,
        1.0,
    );

    assert_eq!(out.len(), hidden_dim);
    for (i, &v) in out.iter().enumerate() {
        assert!(!v.is_nan(), "FlashAttention-3 out[{}] must not be NaN", i);
        assert!(!v.is_infinite(), "FlashAttention-3 out[{}] must be finite", i);
        assert!(v > 0.0, "Attention output with positive values should be > 0.0");
    }
}

#[test]
fn test_golden_bitnet_weight_provider_and_gguf_export() {
    use cronc::{
        export_synthetic_bitnet_gguf, generate_tokens_autoregressive_paged,
        BpeTokenizer, GgufWeightProvider, GoldenBitNetWeightProvider, LayerWeightProvider,
        TransformerConfig,
    };

    let tokenizer = BpeTokenizer::from_default_vocab();
    let config = TransformerConfig {
        vocab_size: tokenizer.vocab_size(),
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 2,
        head_dim: 16,
        intermediate_dim: 64,
        max_seq_len: 64,
        is_ternary_bitnet: true,
        enable_2_4_sparsity: true,
    };

    let golden = GoldenBitNetWeightProvider::new(
        config.num_layers,
        config.hidden_dim,
        config.intermediate_dim,
        config.vocab_size,
    );

    // Verify tensor existence in GoldenBitNetWeightProvider
    for l in 0..config.num_layers {
        for tname in &["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"] {
            let res = golden.get_tensor(l, tname);
            assert!(res.is_some(), "Layer {} tensor '{}' should exist", l, tname);
            let (packed, scales) = res.unwrap();
            assert!(!packed.is_empty());
            assert!(!scales.is_empty());
        }
    }
    assert!(golden.get_tensor(9999, "lm_head").is_some());

    // Export to synthetic GGUF file
    let tmp_path = std::env::temp_dir().join("test_cron_bitnet.gguf");
    let export_res = export_synthetic_bitnet_gguf(&tmp_path, &config);
    assert!(export_res.is_ok(), "GGUF export failed: {:?}", export_res.err());
    let file_size = export_res.unwrap();
    assert!(file_size > 0);

    // Load back using GgufWeightProvider
    let gguf_prov = GgufWeightProvider::from_file(&tmp_path);
    assert!(gguf_prov.is_ok(), "Failed to reload GGUF: {:?}", gguf_prov.err());
    let prov = gguf_prov.unwrap();

    let mut emitted_tokens = Vec::new();
    let gen_res = generate_tokens_autoregressive_paged(
        "CRON",
        4,
        0.5,
        &config,
        &tokenizer,
        Some(&prov),
        |_id, s| emitted_tokens.push(s.to_string()),
    );

    assert_eq!(gen_res.telemetry.generated_tokens, 4);
    assert!(!gen_res.generated_text.is_empty());
    assert!(!gen_res.generated_text.contains('\u{FFFD}'), "Generated text should have zero UTF-8 corruption");

    // Clean up temporary file
    let _ = std::fs::remove_file(tmp_path);
}

#[test]
fn test_advanced_sampling_parameters() {
    use cronc::{sample_token_advanced, SamplingConfig};

    let logits = vec![1.0, 2.0, 5.0, 4.0, 0.5];

    // Greedy sampling at temp 0.0 -> max logit is index 2 (val 5.0)
    let greedy_cfg = SamplingConfig {
        temperature: 0.0,
        top_p: 1.0,
        top_k: 0,
        repetition_penalty: 1.0,
    };
    let picked = sample_token_advanced(&logits, &[], &greedy_cfg);
    assert_eq!(picked, 2);

    // Repetition penalty should penalize token 2 so token 3 (val 4.0) wins
    let rep_cfg = SamplingConfig {
        temperature: 0.0,
        top_p: 1.0,
        top_k: 0,
        repetition_penalty: 2.0, // reduces logit 5.0 -> 2.5, making 4.0 at index 3 the highest
    };
    let penalized_pick = sample_token_advanced(&logits, &[2], &rep_cfg);
    assert_eq!(penalized_pick, 3);
}

#[test]
fn test_multimodal_vision_audio_autoregressive_inference() {
    use cronc::{
        generate_multimodal_autoregressive, BpeTokenizer, SamplingConfig,
        TransformerConfig, VisionConfig, VisionPatchProcessor,
        AudioConfig, AudioSpectrogramProcessor,
    };

    let config = TransformerConfig {
        vocab_size: 384,
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 4,
        head_dim: 8,
        intermediate_dim: 64,
        max_seq_len: 64,
        is_ternary_bitnet: true,
        enable_2_4_sparsity: true,
    };

    // 1. Synthesize synthetic 32x32 RGB image patches
    let vision_cfg = VisionConfig {
        width: 32,
        height: 32,
        channels: 3,
        patch_size: 16,
        embed_dim: 32,
        is_ternary: true,
    };
    let rgb_image = vec![128u8; 32 * 32 * 3];
    let patches = VisionPatchProcessor::extract_patches(&rgb_image, &vision_cfg);
    let patch_embeddings = VisionPatchProcessor::project_patches(&patches, &vision_cfg);
    assert_eq!(patch_embeddings.len(), 4);

    // 2. Synthesize synthetic audio waveform (1 kHz tone)
    let audio_cfg = AudioConfig {
        sample_rate: 16000,
        fft_size: 256,
        hop_length: 128,
        mel_bands: 16,
        embed_dim: 32,
    };
    let mut audio_samples = Vec::with_capacity(1024);
    for i in 0..1024 {
        audio_samples.push((i as f32 * 0.1).sin());
    }
    let mel_frames = AudioSpectrogramProcessor::compute_mel_spectrogram(&audio_samples, &audio_cfg);
    let audio_embeddings = AudioSpectrogramProcessor::project_audio_frames(&mel_frames, config.hidden_dim);
    assert!(!audio_embeddings.is_empty());

    // 3. Combine visual patches and audio frames as prefix multimodal context
    let mut multimodal_prefix = Vec::new();
    multimodal_prefix.extend(patch_embeddings);
    multimodal_prefix.extend(audio_embeddings.into_iter().take(2));

    let tokenizer = BpeTokenizer::from_default_vocab();
    let sampling = SamplingConfig {
        temperature: 0.0,
        top_p: 1.0,
        top_k: 0,
        repetition_penalty: 1.1,
    };

    let mut emitted = Vec::new();
    let res = generate_multimodal_autoregressive(
        &multimodal_prefix,
        "Sensor input:",
        4,
        &sampling,
        &config,
        &tokenizer,
        None,
        |_id, s| emitted.push(s.to_string()),
    );

    assert_eq!(res.telemetry.generated_tokens, 4);
    assert!(res.telemetry.prompt_tokens >= multimodal_prefix.len());
    assert!(!res.generated_text.contains('\u{FFFD}'));
}


