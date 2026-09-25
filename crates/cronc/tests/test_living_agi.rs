use cronc::cl_hyperdimensional::{HyperVector, HolographicMemoryBank};
use cronc::cl_neuromodulation::NeuromodulationEngine;
use cronc::cl_sleep_engine::SleepReplayEngine;
use cronc::cl_global_workspace::GlobalWorkspaceEngine;
use cronc::cl_agi_orchestrator::LivingAgiMind;

#[test]
fn test_hyperdimensional_holographic_memory() {
    let mut bank = HolographicMemoryBank::new();
    bank.store_pair("capital_of_france", "paris");
    bank.store_pair("capital_of_japan", "tokyo");
    bank.store_pair("capital_of_mongolia", "ulaanbaatar");

    let recall_france = bank.recall("capital_of_france");
    assert!(recall_france.is_some());
    let (val_fr, sim_fr) = recall_france.unwrap();
    assert_eq!(val_fr, "paris");
    assert!(sim_fr > 0.15, "Similarity should be significant: {}", sim_fr);

    let recall_mn = bank.recall("capital_of_mongolia");
    assert!(recall_mn.is_some());
    let (val_mn, _) = recall_mn.unwrap();
    assert_eq!(val_mn, "ulaanbaatar");
}

#[test]
fn test_hypervector_algebraic_operations() {
    let a = HyperVector::from_seed("vector_a");
    let b = HyperVector::from_seed("vector_b");

    // Orthogonality of independent hypervectors
    let sim_ab = a.similarity(&b);
    assert!(sim_ab.abs() < 0.15, "Independent vectors should be nearly orthogonal: {}", sim_ab);

    // Self-similarity
    let sim_aa = a.similarity(&a);
    assert!((sim_aa - 1.0).abs() < 1e-4);

    // Binding and Unbinding
    let bound = a.bind(&b);
    let unbound = bound.bind(&a); // In ternary HDC, unbind with 'a' gives back 'b'
    let sim_unbound_b = unbound.similarity(&b);
    assert!(sim_unbound_b > 0.80, "Unbound vector should closely match original b: {}", sim_unbound_b);

    // Sequence Permutation
    let permuted = a.permute(5);
    let sim_perm = a.similarity(&permuted);
    assert!(sim_perm.abs() < 0.15, "Permuted vector should be orthogonal to original");
}

#[test]
fn test_neuromodulatory_chemical_dynamics() {
    let mut neuro = NeuromodulationEngine::new();
    assert_eq!(neuro.current_state.dopamine, 0.5);

    // High reward event surges dopamine
    neuro.trigger_event(1.0, 0.2, 0.5);
    assert!(neuro.current_state.dopamine > 0.7);

    let tuning = neuro.derive_core_tuning();
    assert!(tuning.stdp_learning_rate_scale > 1.5);
    assert!(tuning.lif_spike_threshold > 0.5);

    // Homeostatic decay over cycles
    for _ in 0..50 {
        neuro.step_homeostasis();
    }
    assert!((neuro.current_state.dopamine - 0.5).abs() < 0.1);
}

#[test]
fn test_episodic_sleep_replay_and_pruning() {
    let mut sleep = SleepReplayEngine::new();
    for i in 0..15 {
        sleep.record_experience("Visual Cortex", [0x0100; 16], (i as f32) / 15.0, i * 100);
    }
    assert_eq!(sleep.episodic_buffer.len(), 15);

    let report = sleep.execute_sleep_cycle(64);
    assert_eq!(report.episodes_replayed, 15);
    assert_eq!(sleep.episodic_buffer.len(), 0); // Buffer consolidated
    assert!(report.synapses_pruned > 0);
    assert!(report.entropy_reduction_joules > 0.0);
}

#[test]
fn test_global_workspace_attention_theater() {
    let mut gw = GlobalWorkspaceEngine::new();
    gw.submit_thought("Visual Cortex", "object_detected_car", 0.4, "Visual car incoming");
    gw.submit_thought("Frontal Cortex", "brake_decision", 0.9, "Critical emergency brake");
    gw.submit_thought("Auditory Cortex", "horn_sound", 0.6, "Auditory horn alert");

    let focus = gw.arbitrate_focus(0.2).expect("Focus arbitration must succeed");
    assert_eq!(focus.winning_origin, "Frontal Cortex");
    assert_eq!(focus.winning_thought, "Critical emergency brake");
    assert!(focus.confidence_score > 0.9);
}

#[test]
fn test_unified_living_agi_mind_cycle() {
    let mut mind = LivingAgiMind::new();

    // Turn 1: Query foundational identity
    let turn1 = mind.process_turn("identity", 0.8, false);
    assert!(turn1.recalled_concept.is_some());
    assert!(turn1.recalled_concept.unwrap().contains("SAGI"));

    // Turn 2: New fact learning via dialogue
    let turn2 = mind.process_turn("mission: explore quantum spacetime", 1.0, false);
    assert!(turn2.focus.confidence_score > 0.5);

    // Turn 3: Flash recall of newly learned fact without backpropagation
    let turn3 = mind.process_turn("mission", 0.5, true);
    assert!(turn3.recalled_concept.is_some());
    assert_eq!(turn3.recalled_concept.unwrap(), "explore quantum spacetime");
    assert!(turn3.sleep_report.is_some(), "Sleep consolidation should execute on request");
}

#[test]
fn test_unified_living_agi_all_six_engines() {
    let mut mind = LivingAgiMind::new();

    // Verify initial homeostatic state
    assert_eq!(mind.homeostasis.state.active_state_name, "AWAKE_EXPLORING");
    assert!((mind.homeostasis.state.energy_level - 1.0).abs() < 1e-4);

    // Turn 1: Process active reasoning turn
    let turn = mind.process_turn("architecture 4D-Torus neuromorphic photonic", 0.9, false);

    // 1. Hopfield attractor detection
    assert!(turn.hopfield_attractor.is_some());
    assert_eq!(turn.hopfield_attractor.unwrap(), "architecture");

    // 2. Active inference action & free energy
    assert!(turn.active_inference_action < 4);

    // 3. Elastic SSM streaming token counter
    assert_eq!(turn.ssm_stream_tokens, 1);

    // 4. Stigmergy 4D-Torus thought node exploration
    assert!(turn.stigmergy_selected_node < 256);

    // 5. Homeostasis energy drain
    assert!(turn.energy_level < 1.0);
    assert!(mind.homeostasis.state.energy_level < 1.0);

    // Turn 2: Long sequence streaming into SSM without memory footprint growth
    for i in 0..100 {
        mind.process_turn(&format!("stream_token_{}", i), 0.1, false);
    }
    assert_eq!(mind.ssm.total_tokens_streamed, 101);
    assert_eq!(mind.ssm.memory_footprint_bytes(), 1088); // Exactly 1088 bytes for 16x8 matrix + 8 alphas!
}

#[test]
fn test_living_agi_compile_to_cl() {
    let mind = LivingAgiMind::new();
    let cl_code = mind.compile_living_mind_to_cl();

    // Check code structure
    assert!(cl_code.contains(".core [0,0,0,0]:"));
    assert!(cl_code.contains(".core [0,0,0,1]:")); // Core 64
    assert!(cl_code.contains(".core [0,0,0,2]:")); // Core 128
    assert!(cl_code.contains(".core [0,0,0,3]:")); // Core 192

    // Check that valid bundles are emitted with slots
    assert!(cl_code.contains("B0000:"));
    assert!(cl_code.contains("B0001:"));
    assert!(cl_code.contains("B0002:"));
}


