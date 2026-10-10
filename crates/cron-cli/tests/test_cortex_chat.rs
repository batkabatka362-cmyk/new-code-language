// ============================================================================
// CRON Matrix 4.1 Live 4D Cortical Chat & Online STDP Learning Integration Tests
// File: crates/cron-cli/tests/test_cortex_chat.rs
// ============================================================================

use cronc::cl_topological_lm::TopologicalBrainTransformer;

fn resolve_path(rel: &str) -> std::path::PathBuf {
    let p = std::path::Path::new(rel);
    if p.exists() {
        return p.to_path_buf();
    }
    let p2 = std::path::Path::new("../../").join(rel);
    if p2.exists() {
        return p2;
    }
    let p1 = std::path::Path::new("../").join(rel);
    if p1.exists() {
        return p1;
    }
    p.to_path_buf()
}

#[test]
fn test_cortex_chat_streaming_turn_and_metrics() {
    let mut brain = TopologicalBrainTransformer::new(4, 512);

    let corpus_file = resolve_path("examples/matrix4_cortex_supercomputer/knowledge_corpus.txt");
    if let Ok(corpus_text) = std::fs::read_to_string(&corpus_file) {
        brain.train_sequence_stdp(&corpus_text);
    }

    let initial_tokens = brain.total_tokens_processed;
    assert!(initial_tokens > 0, "Initial tokens should be bootstrapped from corpus");

    // Execute first turn
    let mut emitted_tokens = Vec::new();
    let (response, metrics) = brain.chat_turn_streaming(
        "What is discrete 4D Clifford kinematics?",
        24,
        |tok| {
            emitted_tokens.push(tok.to_string());
            true
        },
    );

    assert_eq!(response.len(), 24);
    assert_eq!(emitted_tokens.len(), 24);
    assert_eq!(metrics.hamiltonian_pct, 100, "Hamiltonian energy must strictly be 100%");
    assert!(metrics.ram_savings_pct > 95.0, "3-Byte KV Cache must achieve >95% RAM compression");
    assert!(brain.total_tokens_processed > initial_tokens, "Live STDP must continuously learn new prompt tokens");

    // Execute second turn to verify multi-turn plasticity
    let prev_learned = brain.total_tokens_processed;
    let (response2, metrics2) = brain.chat_turn_streaming(
        "Explain 0-Anchor energy homeostasis",
        16,
        |_| true,
    );

    assert_eq!(response2.len(), 16);
    assert_eq!(metrics2.hamiltonian_pct, 100);
    assert!(brain.total_tokens_processed > prev_learned, "Brain must continuously adapt without backpropagation");
}

#[test]
fn test_cortex_chat_hud_rendering_and_save_load() {
    let mut brain = TopologicalBrainTransformer::new(4, 512);
    brain.train_sequence_stdp("CRON 4D TOPOLOGICAL QUANTUM SILICON");

    // 1. Telemetry HUD Rendering
    let hud = brain.render_cortex_hud();
    assert!(hud.contains("CRON 4D TOPOLOGICAL CORTICAL MANIFOLD TELEMETRY HUD"));
    assert!(hud.contains("Hamiltonian Energy: 100% (INVARIANT)"));
    assert!(hud.contains("Layer I  (Apical)"));
    assert!(hud.contains("Layer V  (Motor)"));

    // 2. Binary .cl_brain Save and Load Roundtrip
    let temp_brain_path = std::env::temp_dir().join("test_cortex_live_mind.cl_brain");
    brain.save_to_file(&temp_brain_path).expect("Saving brain must succeed");
    assert!(temp_brain_path.exists());

    let loaded = TopologicalBrainTransformer::load_from_file(&temp_brain_path)
        .expect("Loading brain must succeed");

    assert_eq!(loaded.columns.len(), 4);
    assert_eq!(loaded.total_tokens_processed, brain.total_tokens_processed);
    assert_eq!(loaded.render_cortex_hud(), hud);

    let _ = std::fs::remove_file(temp_brain_path);
}
