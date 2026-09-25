use cronc::cl_epigenetic_morph::EpigeneticMorphEngine;
use cronc::cl_reversible_entanglement::ReversibleEntanglementEngine;

#[test]
fn test_reversible_entangled_step_and_time_inversion() {
    let mut engine = ReversibleEntanglementEngine::new(8);
    assert_eq!(engine.current_cycle, 0);

    // Forward reversible steps on R0: +10, +25, +40
    engine.execute_reversible_step(0, 10).unwrap();
    engine.execute_reversible_step(0, 25).unwrap();
    engine.execute_reversible_step(0, 40).unwrap();
    assert_eq!(engine.registers[0].forward_val, 75);
    assert_eq!(engine.current_cycle, 3);

    // Time-Inversion: Step backwards in time without memory checkpoints
    let (reg, val) = engine.step_backward_one_cycle().unwrap();
    assert_eq!(reg, 0);
    assert_eq!(val, 35); // 75 - 40 = 35
    assert_eq!(engine.current_cycle, 2);

    let (reg, val) = engine.step_backward_one_cycle().unwrap();
    assert_eq!(reg, 0);
    assert_eq!(val, 10); // 35 - 25 = 10
    assert_eq!(engine.current_cycle, 1);

    let (reg, val) = engine.step_backward_one_cycle().unwrap();
    assert_eq!(reg, 0);
    assert_eq!(val, 0); // 10 - 10 = 0
    assert_eq!(engine.current_cycle, 0);

    // Thermodynamic Landauer check: 0 bits erased
    let entropy = engine.calculate_landauer_entropy(300.0);
    assert_eq!(entropy.bits_erased, 0);
    assert_eq!(entropy.landauer_dissipated_joules, 0.0);
}

#[test]
fn test_rewind_to_target_cycle() {
    let mut engine = ReversibleEntanglementEngine::new(4);
    for i in 1..=20u32 {
        engine.execute_reversible_step(1, i).unwrap();
    }
    assert_eq!(engine.current_cycle, 20);

    // Rewind back to cycle 5 in one call
    let steps = engine.rewind_to_cycle(5).unwrap();
    assert_eq!(steps, 15);
    assert_eq!(engine.current_cycle, 5);
}

#[test]
fn test_epigenetic_axon_myelination_and_morphing() {
    let mut morph = EpigeneticMorphEngine::new();
    let addr = 0x2000_0010;

    // Baseline unmyelinated: 4 cycles latency
    let lat = morph.step_bundle(addr);
    assert_eq!(lat, 4);

    // Activate 15 times: reaches 25% myelination (3 cycles)
    for _ in 1..15 {
        morph.step_bundle(addr);
    }
    assert_eq!(morph.get_myelin_pct(addr), 25.0);

    // Activate 60 times: reaches 50% myelination (2 cycles)
    for _ in 15..60 {
        morph.step_bundle(addr);
    }
    assert_eq!(morph.get_myelin_pct(addr), 50.0);

    // Activate 100+ times: reaches 100% full insulation (1 cycle fused macro)
    for _ in 60..105 {
        morph.step_bundle(addr);
    }
    assert_eq!(morph.get_myelin_pct(addr), 100.0);
    assert_eq!(morph.total_mutated_macros, 1);
    assert!(morph.cumulative_cycles_saved > 100);
}
