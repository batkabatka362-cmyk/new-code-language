// ============================================================================
// CRON Hardware-Level Constrained Token Sampler Test
// Tests strict JSON grammar enforcement and logit masking for Zero-Hallucination output.
// ============================================================================

use cronc::constrained_sampler::{ConstrainedSampler, GrammarMode, JsonState};

#[test]
fn test_strict_json_grammar_state_transitions() {
    let mut sampler = ConstrainedSampler::new(GrammarMode::StrictJson);
    assert_eq!(sampler.state, JsonState::ExpectObjectStart);

    let json_sequence = "{\"agent\": \"cron-sss\", \"latency_us\": 42, \"verified\": true}";

    for ch in json_sequence.chars() {
        assert!(
            sampler.is_char_valid(ch),
            "Character '{}' should be valid in state {:?}",
            ch,
            sampler.state
        );
        sampler
            .consume_char(ch)
            .expect("State transition should succeed");
    }

    assert!(sampler.is_completed(), "Grammar should be in completed state");
    assert_eq!(sampler.depth, 0);
    assert_eq!(sampler.emitted_text, json_sequence);
}

#[test]
fn test_logit_masking_blocks_invalid_syntax() {
    let sampler = ConstrainedSampler::new(GrammarMode::StrictJson);
    assert_eq!(sampler.state, JsonState::ExpectObjectStart);

    let vocab_chars = vec!['{', 'a', '1', ':', '"', '}'];
    let mut logits = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    // At ExpectObjectStart, only '{' is valid
    sampler.mask_logits(&vocab_chars, &mut logits);

    assert_eq!(logits[0], 1.0, "'{{' must be preserved");
    assert_eq!(logits[1], f32::NEG_INFINITY, "'a' must be masked to -inf");
    assert_eq!(logits[2], f32::NEG_INFINITY, "'1' must be masked to -inf");
    assert_eq!(logits[3], f32::NEG_INFINITY, "':' must be masked to -inf");
    assert_eq!(logits[4], f32::NEG_INFINITY, "'\"' must be masked to -inf");
    assert_eq!(logits[5], f32::NEG_INFINITY, "'}}' must be masked to -inf");
}
