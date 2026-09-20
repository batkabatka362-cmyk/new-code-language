// ============================================================================
// CRON Automated Verification Suite: BPE Tokenizer & Autoregressive Paged Inference
// Test File: test_bpe_and_paged_generation.rs
//
// 100% Pure Rust - Zero External Dependencies
// ============================================================================

use cronc::bpe_tokenizer::{BpeTokenizer, BOS_TOKEN, IM_END_TOKEN, IM_START_TOKEN};
use cronc::cl_infer::{
    generate_tokens_autoregressive_paged, transformer_forward_step_paged,
    KVCache, LayerWeightProvider, TransformerConfig,
};
use cronc::model_importer::ternary::pack_ternary_2bit;

#[test]
fn test_bpe_tokenizer_basic_roundtrip() {
    let tok = BpeTokenizer::from_default_vocab();
    let sample_text = "CRON BitNet 1.58b 4D-Torus neural architecture running in 16 MB RAM";

    let encoded = tok.encode(sample_text);
    assert!(!encoded.is_empty(), "Encoded tokens should not be empty");

    let decoded = tok.decode(&encoded);
    assert_eq!(decoded, sample_text, "BPE roundtrip must be lossless");
}

#[test]
fn test_bpe_tokenizer_byte_fallback() {
    let tok = BpeTokenizer::from_default_vocab();
    // Test with special non-ASCII characters, Mongolian Cyrillic, and emojis
    let text = "Сайн байна уу? 🚀 CRON 1.58b: α + β = γ!";

    let encoded = tok.encode(text);
    assert!(!encoded.is_empty());

    let decoded = tok.decode(&encoded);
    assert_eq!(decoded, text, "Byte-fallback must preserve all UTF-8 characters without UNK");
}

#[test]
fn test_bpe_tokenizer_special_tokens() {
    let tok = BpeTokenizer::from_default_vocab();
    let text = format!("{}{}Hello CRON AI{}", BOS_TOKEN, IM_START_TOKEN, IM_END_TOKEN);

    let encoded = tok.encode(&text);
    assert!(encoded.contains(&tok.bos_token_id()), "Must contain BOS token ID");

    let decoded = tok.decode(&encoded);
    assert_eq!(decoded, text, "Special tokens must roundtrip perfectly");
}

#[test]
fn test_bpe_tokenizer_from_gguf_metadata() {
    let tokens = vec![
        "<unk>".to_string(),
        "<s>".to_string(),
        "</s>".to_string(),
        "CR".to_string(),
        "ON".to_string(),
        "CRON".to_string(),
    ];
    let merges = vec!["CR ON".to_string()];

    let tok = BpeTokenizer::from_gguf_metadata(&tokens, None, Some(&merges))
        .expect("Failed to build tokenizer from GGUF metadata");

    assert!(tok.vocab_size() >= 256);
}

/// Mock layer weight provider supplying packed ternary weights
struct MockPagedWeightProvider {
    packed_data: Vec<u8>,
    scales: Vec<f32>,
}

impl MockPagedWeightProvider {
    fn new(in_dim: usize, out_dim: usize) -> Self {
        // Create deterministic ternary weights {-1, 0, 1}
        let mut ternary = Vec::with_capacity(in_dim * out_dim);
        for i in 0..(in_dim * out_dim) {
            let val = match i % 3 {
                0 => 0i8,
                1 => 1i8,
                _ => -1i8,
            };
            ternary.push(val);
        }

        let packed = pack_ternary_2bit(&ternary);
        let scales = vec![0.5f32; out_dim];

        Self {
            packed_data: packed,
            scales,
        }
    }
}

impl LayerWeightProvider for MockPagedWeightProvider {
    fn get_tensor(&self, _layer_idx: usize, _tensor_name: &str) -> Option<(Vec<u8>, Vec<f32>)> {
        Some((self.packed_data.clone(), self.scales.clone()))
    }
}

#[test]
fn test_transformer_forward_step_paged_with_weights() {
    let tok = BpeTokenizer::from_default_vocab();
    let config = TransformerConfig {
        vocab_size: tok.vocab_size(),
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 2,
        head_dim: 16,
        intermediate_dim: 64,
        max_seq_len: 64,
        is_ternary_bitnet: true,
        enable_2_4_sparsity: true,
    };

    let mut kv_cache = KVCache::new(config.num_layers, config.max_seq_len, config.hidden_dim);
    let weight_provider = MockPagedWeightProvider::new(config.hidden_dim, config.hidden_dim);

    let logits = transformer_forward_step_paged(
        42,
        0,
        &config,
        &mut kv_cache,
        Some(&weight_provider),
    );

    assert_eq!(logits.len(), config.vocab_size, "Logits dimension must match vocabulary size");
    assert!(logits.iter().any(|&v| v != 0.0), "Logits must not be all zeros");
}

#[test]
fn test_autoregressive_paged_generation_streaming() {
    let tok = BpeTokenizer::from_default_vocab();
    let config = TransformerConfig {
        vocab_size: tok.vocab_size(),
        hidden_dim: 32,
        num_layers: 2,
        num_heads: 2,
        head_dim: 16,
        intermediate_dim: 64,
        max_seq_len: 64,
        is_ternary_bitnet: true,
        enable_2_4_sparsity: true,
    };

    let weight_provider = MockPagedWeightProvider::new(config.hidden_dim, config.hidden_dim);

    let mut streamed_tokens = Vec::new();
    let mut streamed_text = String::new();

    let result = generate_tokens_autoregressive_paged(
        "CRON neural prompt",
        8,
        0.7,
        &config,
        &tok,
        Some(&weight_provider),
        |id, text_chunk| {
            streamed_tokens.push(id);
            streamed_text.push_str(text_chunk);
        },
    );

    assert_eq!(result.telemetry.generated_tokens, streamed_tokens.len());
    assert!(!result.generated_text.is_empty());
    assert!(result.telemetry.time_to_first_token_us > 0.0);
    assert!(result.telemetry.tokens_per_second >= 0.0);
    assert!(result.telemetry.kv_cache_occupancy_pct > 0.0);
}
