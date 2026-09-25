use cronc::cl_self_compiler::SelfRewritingJitEngine;
use cronc::cl_quantum_zeno::QuantumHypothesisRegister;
use cronc::cl_reversible_thermo::{ReversibleState3, ReversiblePipelineVerifier};
use cronc::cl_spatiotemporal::SpatiotemporalStreamer;
use cronc::cl_macro::build_valid_slot;

#[test]
fn test_live_self_rewriting_jit() {
    let slot0 = build_valid_slot("==00#010", "");
    let slot1 = build_valid_slot("_AD01000", "");
    let nop = build_valid_slot("__NOP0000", "");

    let bundles = vec![
        [slot0, slot1, nop.clone(), nop.clone()],
    ];

    let mut jit = SelfRewritingJitEngine::from_bundles(&bundles);
    let patched = jit.hot_patch_slot(0, 1, "_OP", "010000");
    assert!(patched);
    assert_eq!(jit.mutation_log.len(), 1);
    assert_eq!(jit.total_cycles_saved, 3);

    let mutated_cl = jit.emit_mutated_cl();
    assert!(mutated_cl.contains("B0000:"));
    assert!(mutated_cl.contains("_OP"));
}

#[test]
fn test_quantum_zeno_hypothesis_convergence() {
    let mut reg = QuantumHypothesisRegister::new_hadamard_superposition();

    // All qubits in 50/50 superposition initially
    assert!((reg.qubits[0].prob_zero() - 0.5).abs() < 1e-3);

    // Quantum Zeno drive toward target pattern 0xAAAA (1010101010101010)
    let target = 0xAAAAu16;
    let collapsed = reg.converge_zeno(target, 20);

    assert_eq!(collapsed, target, "Quantum Zeno should drive register exactly to target attractor");
}

#[test]
fn test_thermodynamic_reversible_gates() {
    let state = ReversibleState3::new(true, true, false);

    // Fredkin gate with A=1 swaps B and C: (1, 1, 0) -> (1, 0, 1)
    let fredkin_res = state.fredkin();
    assert_eq!(fredkin_res, ReversibleState3::new(true, false, true));

    // Toffoli gate with A=1, B=1 flips C: (1, 1, 0) -> (1, 1, 1)
    let toffoli_res = state.toffoli();
    assert_eq!(toffoli_res, ReversibleState3::new(true, true, true));

    // Verifier proves 0 bits erased and 0 Joules dissipated
    let mut verifier = ReversiblePipelineVerifier::new();
    let _ = verifier.step_reversible(state, "fredkin");
    let _ = verifier.step_reversible(state, "toffoli");

    assert_eq!(verifier.total_operations, 2);
    assert_eq!(verifier.bits_erased, 0);
    assert_eq!(verifier.dissipated_joules, 0.0);
}

#[test]
fn test_spatiotemporal_multimodal_streamer() {
    let mut streamer = SpatiotemporalStreamer::new();

    // Ingest visual event
    streamer.ingest_event(10, 20, 0, 0.5, 1);
    // Ingest auditory tonotopic spike
    streamer.ingest_event(0, 0, 440, 1.2, 1);

    assert_eq!(streamer.active_events.len(), 2);
    assert!(streamer.temporal_coherence_index > 0.90);

    let cl_code = streamer.compile_snapshot_to_cl(0);
    assert!(cl_code.contains("B0000:"));
    assert!(cl_code.contains("_OP"));
}
