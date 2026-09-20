// ============================================================================
// CRON Interactive AI Terminal & Chat Engine Tests (test_interactive_chat.rs)
// ============================================================================

use cron_cli::chat::{ChatConfig, ChatSession};

#[test]
fn test_chat_session_unconstrained_turn() {
    let config = ChatConfig {
        model_name: "cron-bitnet-7b".to_string(),
        num_layers: 8,
        page_size_mb: 8,
        k_speculative: 4,
        grammar_mode: "unconstrained".to_string(),
        weights_path: None,
    };

    let mut session = ChatSession::new(config);
    let mut out = Vec::new();

    let telemetry = session.execute_turn("Calculate 4D distance to core 12", &mut out)
        .expect("Execution turn should succeed");

    let response_str = String::from_utf8(out).expect("Valid UTF-8 output");
    assert!(response_str.contains("CRON cognitive engine processed"));
    assert!(response_str.contains("across 8 layers with 8 MB resident set"));

    assert_eq!(session.turns_count, 1);
    assert!(session.total_tokens_emitted > 0);
    assert!(telemetry.prompt_tokens > 0);
    assert_eq!(telemetry.active_ram_mb, 8);
    assert!(telemetry.effective_speedup >= 1.0);
    assert!(telemetry.elapsed_ms >= 0.0);
}

#[test]
fn test_chat_session_multiple_turns_bounded_memory() {
    let config = ChatConfig {
        model_name: "cron-bitnet-7b".to_string(),
        num_layers: 16,
        page_size_mb: 16,
        k_speculative: 4,
        grammar_mode: "unconstrained".to_string(),
        weights_path: None,
    };

    let mut session = ChatSession::new(config);

    for i in 1..=5 {
        let mut out = Vec::new();
        let prompt = format!("Step {}: evaluate attention matrix", i);
        let telemetry = session.execute_turn(&prompt, &mut out).unwrap();

        assert_eq!(session.turns_count, i);
        assert_eq!(telemetry.active_ram_mb, 16);
        assert!(telemetry.tokens_per_sec > 0.0);
    }

    assert_eq!(session.turns_count, 5);
    assert!(session.total_tokens_emitted >= 15);
}

#[test]
fn test_chat_session_json_constrained_mode() {
    let config = ChatConfig {
        model_name: "cron-bitnet-7b".to_string(),
        num_layers: 4,
        page_size_mb: 4,
        k_speculative: 2,
        grammar_mode: "json".to_string(),
        weights_path: None,
    };

    let mut session = ChatSession::new(config);
    let mut out = Vec::new();

    let telemetry = session.execute_turn("Provide status in JSON format", &mut out)
        .expect("JSON turn should succeed");

    let response_str = String::from_utf8(out).expect("Valid UTF-8 output");
    let trimmed = response_str.trim();

    assert!(trimmed.starts_with('{') && trimmed.ends_with('}'));
    assert!(trimmed.contains("\"model\": \"cron-bitnet-7b\""));
    assert!(trimmed.contains("\"response\": \"Executed in 16MB RAM\""));
    assert_eq!(telemetry.active_ram_mb, 4);
}
