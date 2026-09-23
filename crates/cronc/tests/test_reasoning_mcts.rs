// ============================================================================
// Unit Tests for CRON Neuro-Symbolic MCTS & Formal Proof Engine
// (test_reasoning_mcts.rs)
// ============================================================================

use cronc::cl_reasoning::{
    FunctionalUnit, MctsConfig, MctsScheduler, MicroOp, SchedulingState,
};
use cronc::cl_proof::{ProofKind, ProofVerifier};

#[test]
fn test_mcts_initial_state_and_config() {
    let config = MctsConfig::default();
    assert_eq!(config.simulations, 200);
    assert_eq!(config.rollout_depth, 16);
    assert!((config.c_puct - std::f64::consts::SQRT_2).abs() < 0.001);

    let scheduler = MctsScheduler::new(config);
    assert_eq!(scheduler.config.target_ipc, 4.0);
}

#[test]
fn test_mcts_op_classification_and_dag_generation() {
    let scheduler = MctsScheduler::new(MctsConfig::default());
    let ops = scheduler.generate_synthetic_ops("FlashAttention-2 Forward Tile");
    assert!(!ops.is_empty());
    assert!(ops.len() >= 10);

    // Verify functional units span Memory, Tensor, Optic, Alu, Control, Branch
    let units: Vec<FunctionalUnit> = ops.iter().map(|o| o.functional_unit).collect();
    assert!(units.contains(&FunctionalUnit::Memory));
    assert!(units.contains(&FunctionalUnit::TensorSystolic));
    assert!(units.contains(&FunctionalUnit::OpticalMzi));
    assert!(units.contains(&FunctionalUnit::Alu));
    assert!(units.contains(&FunctionalUnit::Control));
}

#[test]
fn test_mcts_ready_ops_and_hazard_avoidance() {
    let scheduler = MctsScheduler::new(MctsConfig::default());
    let ops = vec![
        MicroOp {
            id: 0,
            mnemonic: "SRAM_LD".to_string(),
            functional_unit: FunctionalUnit::Memory,
            latency_cycles: 2,
            dest_reg: Some("R1".to_string()),
            src_regs: vec!["R0".to_string()],
            memory_bank: Some(0),
            raw_text: "SRAM_LD R1, @sram(0) [R0]".to_string(),
        },
        MicroOp {
            id: 1,
            mnemonic: "ADD".to_string(),
            functional_unit: FunctionalUnit::Alu,
            latency_cycles: 1,
            dest_reg: Some("R2".to_string()),
            src_regs: vec!["R1".to_string()], // Depends on R1 (latency 2)
            memory_bank: None,
            raw_text: "ADD R2, R1, 10".to_string(),
        },
    ];

    let mut state = SchedulingState::new(2);
    let ready = scheduler.get_ready_ops(&ops, &state);
    assert_eq!(ready, vec![0], "Op 1 cannot be scheduled in cycle 0 due to RAW dependency on R1");

    // Apply Op 0 in cycle 0
    let actions = scheduler.generate_actions(&ready, &ops);
    scheduler.apply_action(&mut state, &actions[0], &ops);
    assert_eq!(state.current_cycle, 1);

    // In cycle 1, Op 0 has latency 2 (ready at cycle 2), so Op 1 is still NOT ready
    let ready_c1 = scheduler.get_ready_ops(&ops, &state);
    assert_eq!(ready_c1, vec![], "Op 1 still not ready at cycle 1 because R1 is ready at cycle 2");

    // Advance to cycle 2
    state.current_cycle = 2;
    let ready_c2 = scheduler.get_ready_ops(&ops, &state);
    assert_eq!(ready_c2, vec![1], "Op 1 is now ready at cycle 2");
}

#[test]
fn test_mcts_action_generation_multi_slot() {
    let scheduler = MctsScheduler::new(MctsConfig::default());
    let ops = vec![
        MicroOp {
            id: 0,
            mnemonic: "ADD".to_string(),
            functional_unit: FunctionalUnit::Alu,
            latency_cycles: 1,
            dest_reg: Some("R1".to_string()),
            src_regs: vec!["R0".to_string()],
            memory_bank: None,
            raw_text: "ADD R1, R0, 1".to_string(),
        },
        MicroOp {
            id: 1,
            mnemonic: "SRAM_LD".to_string(),
            functional_unit: FunctionalUnit::Memory,
            latency_cycles: 2,
            dest_reg: Some("R2".to_string()),
            src_regs: vec!["R0".to_string()],
            memory_bank: Some(1),
            raw_text: "SRAM_LD R2, @sram(1) [R0]".to_string(),
        },
    ];

    let actions = scheduler.generate_actions(&[0, 1], &ops);
    // Should have single op actions + combined bundle action
    assert!(actions.iter().any(|a| a.selected_op_ids.len() == 2), "Must propose a 2-op bundle");
}

#[test]
fn test_mcts_synthesizer_flash_attention_convergence() {
    let mut scheduler = MctsScheduler::new(MctsConfig {
        simulations: 100,
        rollout_depth: 12,
        ..Default::default()
    });

    let res = scheduler.synthesize("FlashAttention-2 forward tile", None);
    assert!(res.optimized_ipc >= 1.25, "MCTS must achieve IPC >= 1.25, got {:.2}", res.optimized_ipc);
    assert!(res.total_cycles < res.total_ops, "Bundling must compress cycle count");
    assert!(res.speedup_pct > 0.0);
    assert!(res.tree_node_count > 10);
    assert!(!res.cl_code.is_empty());
}

#[test]
fn test_mcts_custom_input_code_scheduling() {
    let mut scheduler = MctsScheduler::new(MctsConfig {
        simulations: 50,
        rollout_depth: 8,
        ..Default::default()
    });

    let raw_code = r#"
        ADD R1, R0, 4
        SRAM_LD R2, @sram(0) [R0]
        ADD R3, R0, 8
        MZI_ATTN V1, R1, Lambda0
        CRC_AUTH R1, 0x55
    "#;

    let res = scheduler.synthesize("Custom Pipeline", Some(raw_code));
    assert_eq!(res.total_ops, 5);
    assert!(res.optimized_ipc > 1.0);
}

#[test]
fn test_mcts_json_serialization() {
    let mut scheduler = MctsScheduler::new(MctsConfig {
        simulations: 30,
        ..Default::default()
    });

    let res = scheduler.synthesize("BitNet Ternary GEMM", None);
    let json = res.to_json();
    assert!(json.contains("\"prompt\": \"BitNet Ternary GEMM\""));
    assert!(json.contains("\"optimized_ipc\":"));
    assert!(json.contains("\"speedup_pct\":"));
    assert!(json.contains("\"total_cycles\":"));
}

#[test]
fn test_proof_verifier_deadlock_freedom_lemma() {
    let verifier = ProofVerifier::new();
    let code = "B0000: SRAM_LD R1, @sram(0) [R0] | SYS_MATMUL V1, R1, R2 | CRC_AUTH R1, 0x12 | NOP\n";
    let cert = verifier.verify_kernel(code, "Systolic GEMM");

    assert!(cert.is_certified);
    assert_eq!(cert.total_lemmas, 6);
    assert_eq!(cert.passed_lemmas, 6);

    let dor_lemma = cert.lemmas.iter().find(|l| l.kind == ProofKind::DeadlockFreedom).unwrap();
    assert!(dor_lemma.verified);
    assert!(dor_lemma.formal_statement.contains("Cycles(CDG) = ∅"));
    assert!(dor_lemma.deduction_steps[1].contains("Dally-Seitz"));
}

#[test]
fn test_proof_verifier_wcet_and_landauer_bounds() {
    let verifier = ProofVerifier::new();
    let code = r#"
        B0000: SRAM_LD R1, @sram(0) [R0] | NOP | NOP | NOP
        B0001: MZI_ATTN V1, R1, Lambda0  | NOP | NOP | NOP
        B0002: STORE @sram(1) [R0], V1   | NOP | NOP | NOP
    "#;
    let cert = verifier.verify_kernel(code, "Optical Attention Tile");

    assert!(cert.max_cycles_bound >= 12);
    assert!(cert.landauer_dissipation_pj > 0.0);
    assert!(cert.landauer_dissipation_pj < 1.0, "Dissipation must be well below 50 pJ budget");

    let landauer_lemma = cert.lemmas.iter().find(|l| l.kind == ProofKind::ThermodynamicEntropy).unwrap();
    assert!(landauer_lemma.verified);
}

#[test]
fn test_proof_certificate_ascii_badge_and_json() {
    let verifier = ProofVerifier::new();
    let code = "B0000: ADD R1, R0, 1 | NOP | NOP | NOP\n";
    let cert = verifier.verify_kernel(code, "Increment Loop");

    let badge = cert.render_ascii_badge();
    assert!(badge.contains("CRON FORMAL MATHEMATICAL PROOF CERTIFICATE"));
    assert!(badge.contains("MATHEMATICALLY PROVEN & TAPE-OUT CERTIFIED"));

    let json = cert.to_json();
    assert!(json.contains("\"certificate_id\":"));
    assert!(json.contains("\"is_certified\": true"));
    assert!(json.contains("\"lemmas\":"));
}
