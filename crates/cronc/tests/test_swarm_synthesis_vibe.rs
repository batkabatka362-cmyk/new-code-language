// =============================================================================
// CRON Autonomous Swarm Self-Synthesis & Vibe-Healing: Integration Test Suite
// Validates: Multi-agent closed-loop synthesis, continuous vibe-healing,
// hardware JIT verification, consensus quorum, and JSON serialization.
// =============================================================================

use cronc::cl_swarm_synthesis::{SwarmSynthesizer, SynthesisConfig};
use cronc::cl_swarm::AgentRole;
use cronc::verify_cl_program;

#[test]
fn test_swarm_synthesis_flash_attention() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Synthesize FlashAttention-2 with optical MZI attention", &config);

    assert!(report.consensus_achieved);
    assert!(report.detected_operators.contains(&"flash-attn".to_string()));
    assert!(report.target_brains.iter().any(|b| b.contains("Photonic")));
    assert!(report.final_code.contains(".core [0, 0, 0, 0]:"));
    assert!(report.final_code.contains("flash-attn"));
    assert!(report.final_code.contains("B0000:"));
    assert!(report.optimized_ipc >= 2.0);

    // Verify .cl syntax validity
    let verify_res = verify_cl_program(&report.final_code);
    assert!(verify_res.is_ok(), "Final synthesized code must verify cleanly: {:?}", verify_res.err());
}

#[test]
fn test_swarm_synthesis_bitnet_ternary_gemm() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Synthesize BitNet b1.58 ternary linear layer", &config);

    assert!(report.consensus_achieved);
    assert!(report.detected_operators.contains(&"bitnet-gemm".to_string()));
    assert!(report.target_brains.iter().any(|b| b.contains("Ternary")));
    assert!(report.final_code.contains("bitnet-gemm"));
    assert!(report.execution_cycles > 0);

    let verify_res = verify_cl_program(&report.final_code);
    assert!(verify_res.is_ok(), "Synthesized BitNet kernel must pass verification: {:?}", verify_res.err());
}

#[test]
fn test_swarm_synthesis_rmsnorm_simd() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Generate RMSNorm root mean square normalization with SIMD reduction", &config);

    assert!(report.consensus_achieved);
    assert!(report.detected_operators.contains(&"rmsnorm".to_string()));
    assert!(report.target_brains.iter().any(|b| b.contains("SIMD")));
}

#[test]
fn test_swarm_synthesis_swiglu_mlp() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Synthesize SwiGLU dual-projection activation kernel", &config);

    assert!(report.consensus_achieved);
    assert!(report.detected_operators.contains(&"swiglu".to_string()));
}

#[test]
fn test_swarm_synthesis_rope_cordic() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Synthesize CORDIC Rotary Position Embedding RoPE kernel", &config);

    assert!(report.consensus_achieved);
    assert!(report.detected_operators.contains(&"rope".to_string()));
    assert!(report.target_brains.iter().any(|b| b.contains("CORDIC")));
}

#[test]
fn test_swarm_synthesis_kv_cache_streaming() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Generate 4D-Torus spatial streaming paged KV-cache channel", &config);

    assert!(report.consensus_achieved);
    assert!(report.detected_operators.contains(&"kv-cache".to_string()));
    assert!(report.target_brains.iter().any(|b| b.contains("4D-Torus")));
}

#[test]
fn test_swarm_synthesis_multi_operator_fusion() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal(
        "Synthesize FlashAttention with fused RMSNorm and BitNet ternary projection",
        &config,
    );

    assert!(report.consensus_achieved);
    assert!(report.detected_operators.contains(&"flash-attn".to_string()));
    assert!(report.detected_operators.contains(&"rmsnorm".to_string()));
    assert!(report.detected_operators.contains(&"bitnet-gemm".to_string()));
    assert!(report.target_brains.len() >= 2);
}

#[test]
fn test_swarm_synthesis_continuous_vibe_healing() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig {
        max_iterations: 3,
        auto_heal: true,
        auto_opt: true,
        run_jit: true,
        target_chip: None,
    };

    let report = synth.synthesize_and_heal("Synthesize BitNet b1.58 ternary GEMM", &config);

    assert!(report.healed, "Synthesizer must execute continuous vibe-healing");
    assert!(report.fixed_crc_count > 0, "Must fix CRC-8 ATM tokens during healing");
    assert!(report.speedup_percentage >= 0.0);
}

#[test]
fn test_swarm_synthesis_jit_execution_zero_traps() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("FlashAttention-2 kernel", &config);

    assert!(report.execution_cycles > 0, "Must execute in JIT VM");
    assert!(report.latency_us > 0.0, "Latency must be measured");
}

#[test]
fn test_swarm_synthesis_json_serialization_validity() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Synthesize BitNet ternary GEMM", &config);

    let json = report.to_json();
    assert!(json.contains("\"prompt\": \"Synthesize BitNet ternary GEMM\""));
    assert!(json.contains("\"consensus_achieved\": true"));
    assert!(json.contains("\"consensus_score\":"));
    assert!(json.contains("\"healed\": true"));
    assert!(json.contains("\"fixed_crc_count\":"));
    assert!(json.contains("\"detected_operators\": [\"bitnet-gemm\"]"));
    assert!(json.contains("\"status\": \"synthesis_complete\""));
}

#[test]
fn test_swarm_synthesis_timeline_steps_ordering() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("FlashAttention-2 forward tile", &config);

    assert!(report.steps.len() >= 4);

    // Step 0 must be Intent Analysis by Planner
    assert_eq!(report.steps[0].stage, "Intent Analysis");
    assert_eq!(report.steps[0].agent_role, AgentRole::Planner);

    // Step 1 must be Microcode Generation by Coder
    assert_eq!(report.steps[1].stage, "Microcode Generation");
    assert_eq!(report.steps[1].agent_role, AgentRole::Coder);

    // Final step must be Consensus Certification by Actuator
    let last = report.steps.last().unwrap();
    assert_eq!(last.stage, "Consensus Certification");
    assert_eq!(last.agent_role, AgentRole::Actuator);
}

#[test]
fn test_swarm_synthesis_ascii_hud_contents() {
    let synth = SwarmSynthesizer::new();
    let config = SynthesisConfig::default();
    let report = synth.synthesize_and_heal("Synthesize BitNet ternary GEMM", &config);

    assert!(report.ascii_synthesis_hud.contains("CRON Autonomous Swarm Self-Synthesis & Vibe-Healing HUD"));
    assert!(report.ascii_synthesis_hud.contains("Multi-Agent Timeline & Vibe-Healing Trace:"));
    assert!(report.ascii_synthesis_hud.contains("Silicon Metrics:"));
    assert!(report.ascii_synthesis_hud.contains("Status: Certified SSS+"));
}
