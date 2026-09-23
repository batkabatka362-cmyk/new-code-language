//! Comprehensive Superintelligence Test Suite for the 6 Pillars of .cl & SAGI Integration

use cronc::*;

#[test]
fn test_pillar1_metaplasticity_bcm_and_stdp() {
    let mut engine = MetaplasticEngine::new(4, BcmConfig::default());
    engine.add_synapse(0, 1, 100);
    engine.add_synapse(1, 2, 200);

    // Initial state
    assert_eq!(engine.synapses.len(), 2);
    assert_eq!(engine.synapses[0].weight, 100);

    // Run plasticity cycle with high pre/post activity
    let pre_acts = vec![512, 512, 0];
    let post_acts = vec![0, 600, 600];
    let modified = engine.execute_plasticity_cycle(1, &pre_acts, &post_acts);

    assert_eq!(modified, 2);
    assert!(engine.total_plasticity_events > 0);
}

#[test]
fn test_pillar2_thermodynamics_and_zero_power_quench() {
    let mut thermo = ThermodynamicOptimizer::new(8, 300.0);
    assert_eq!(thermo.tiles.len(), 8);

    // Low flux on tiles 0..4 should quench them into zero-power state
    let fluxes = vec![0.01, 0.02, 0.03, 0.04, 0.50, 0.80, 0.90, 0.95];
    let quenched = thermo.balance_thermal_flux(&fluxes);

    assert_eq!(quenched, 4);
    assert!(thermo.tiles[0].is_thermally_quenched);
    assert!(!thermo.tiles[7].is_thermally_quenched);
    assert!(thermo.total_energy_saved_pj > 0.0);
}

#[test]
fn test_pillar3_clifford_cl40_and_optical_mzi() {
    // 16-Blade Cl(4,0) multivector
    let v1 = Multivector16::from_vector(1.0, 0.0, 0.0, 0.0); // e0
    let v2 = Multivector16::from_vector(0.0, 1.0, 0.0, 0.0); // e1

    // Geometric product e0 * e1 = e01 (bivector)
    let prod = v1.geometric_product(&v2);
    assert_eq!(prod.blades[5], 1.0); // bivector blade e01

    // Optical MZI Phase Rotor
    let rotor = MziPhaseRotor::new(std::f32::consts::FRAC_PI_2, 0.0);
    let (out0, out1) = rotor.transform_optical_field((1.0, 0.0), (0.0, 0.0));
    assert!(out0.0.abs() < 1e-4);
    assert!((out1.0 - 1.0).abs() < 1e-4);
}

#[test]
fn test_pillar4_sagi_6brain_bridge() {
    let mut bridge = SagiBrainBridge::new();
    assert_eq!(bridge.active_brains.len(), 6);

    let seq = bridge.dispatch_directive(
        SagiBrainKind::ALifeOrganism,
        SagiBrainKind::MctsEngine,
        0x01A0,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    );

    assert_eq!(seq, 1);
    assert_eq!(bridge.mailboxes.len(), 1);
    assert_eq!(bridge.mailboxes[0].command_opcode, 0x01A0);

    // Update homeostasis
    bridge.update_homeostasis(128, 64);
    assert!(bridge.neuromodulators.dopamine > 512);
}

#[test]
fn test_pillar5_4d_torus_lockless_dor_routing() {
    let mut mesh = Torus4DMesh::new(4, 4, 4, 4); // 256 cores
    assert_eq!(mesh.total_nodes, 256);

    let src = Torus4DCoord::new(0, 0, 0, 0);
    let dst = Torus4DCoord::new(2, 3, 1, 2);

    let dist = src.torus_distance(&dst, (4, 4, 4, 4));
    assert_eq!(dist, 2 + 1 + 1 + 2); // 2 + 1 + 1 + 2 = 6

    let packet = TorusMicroPacket {
        src,
        dst,
        tag: 42,
        data: [0xCAFE_BABE, 0xDEAD_BEEF],
        hops_taken: 0,
    };

    let routed = mesh.route_packet(packet);
    assert_eq!(routed.hops_taken, dist);
    assert_eq!(mesh.routed_packets_count, 1);
}

#[test]
fn test_pillar6_metacognitive_verification_and_healing() {
    let mut metacog = MetacognitiveEngine::new();

    // Normal value
    let (ok1, val1) = metacog.verify_and_heal(1, 0, "WeightStability", 100.0);
    assert!(ok1);
    assert_eq!(val1, 100.0);

    // Out-of-bounds overflow value -> Auto-clamped
    let (ok2, val2) = metacog.verify_and_heal(2, 1, "WeightStability", 99999.0);
    assert!(!ok2);
    assert_eq!(val2, 8192.0);
    assert_eq!(metacog.total_heals_applied, 1);
    assert_eq!(metacog.anomaly_log.len(), 1);
}
