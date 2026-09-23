//! End-to-End Cross-Engine Integration Test: CRON (.cl & .cr) <-> SAGI 6-Brain Engine
//! Target: 256-Core 4D-Torus Neuromorphic Photonic Silicon Bridge

use cronc::*;
use cronc::cl_kernel::synthesize_kernel;
use cronc::cl_jit::{execute_cl_on_core, ClJitCore};

#[test]
fn test_sagi_cross_engine_golden_v99_synthesis() {
    // 1. Synthesize SAGI V99 Metaplastic Kernel directly from CRON compiler
    let kernel_code = synthesize_kernel("sagi-metaplastic-v99", 64, 16)
        .expect("Failed to synthesize sagi-metaplastic-v99 kernel");

    assert!(!kernel_code.is_empty());
    assert!(kernel_code.contains("@sagi_brain_init:"));
    assert!(kernel_code.contains("@clifford_spacetime_rotation:"));
    assert!(kernel_code.contains("@bcm_metaplastic_update:"));
    assert!(kernel_code.contains("@onsager_thermal_quench:"));

    // 2. Execute natively on ClJitCore simulator
    let mut core = ClJitCore::new();
    let exec_res = execute_cl_on_core(&kernel_code, &mut core);
    assert!(exec_res.is_ok(), "JIT Execution error: {:?}", exec_res.err());
    assert!(core.cycle_count > 0);
    println!("✓ SAGI V99 Kernel executed in {} hardware cycles on Core 0.", core.cycle_count);
}

#[test]
fn test_sagi_6brain_mailbox_ipc_end_to_end() {
    let mut bridge = SagiBrainBridge::new();

    // Send cognitive directives between Brain 4 (ALife) -> Brain 3 (MCTS) -> Brain 1 (Quantum Torus)
    let payload = [512, 256, 128, 64, 32, 16, 8, 4, 2, 1, 0, 0, 0, 0, 0, 0];
    let seq1 = bridge.dispatch_directive(
        SagiBrainKind::ALifeOrganism,
        SagiBrainKind::MctsEngine,
        0x01A0,
        payload,
    );
    let seq2 = bridge.dispatch_directive(
        SagiBrainKind::MctsEngine,
        SagiBrainKind::QuantumTorus,
        0x02B0,
        payload,
    );

    assert_eq!(seq1, 1);
    assert_eq!(seq2, 2);
    assert_eq!(bridge.mailboxes.len(), 2);

    // Verify 4D-Torus routing between core (0,0,0,0) and (2,3,1,2)
    let mut mesh = Torus4DMesh::new(4, 4, 4, 4);
    let packet = TorusMicroPacket {
        src: Torus4DCoord::new(0, 0, 0, 0),
        dst: Torus4DCoord::new(2, 3, 1, 2),
        tag: 0x5A61, // "SAGI"
        data: [0x01A0_CAFE, 0x02B0_BEEF],
        hops_taken: 0,
    };

    let routed = mesh.route_packet(packet);
    assert_eq!(routed.hops_taken, 6);
    println!("✓ SAGI Micro-packet successfully routed across 4D Torus in {} hops.", routed.hops_taken);
}

#[test]
fn test_sagi_thermodynamics_and_metacognitive_safety() {
    let mut thermo = ThermodynamicOptimizer::new(16, 300.0);
    let mut metacog = MetacognitiveEngine::new();

    // Verify thermal quench under low flux
    let fluxes = [0.01; 16];
    let quenched = thermo.balance_thermal_flux(&fluxes);
    assert_eq!(quenched, 16);
    assert!(thermo.total_energy_saved_pj > 0.0);

    // Metacognitive soundness validation
    let (ok, healed_val) = metacog.verify_and_heal(100, 5, "EntropyConservation", 12.5);
    assert!(ok);
    assert_eq!(healed_val, 12.5);

    println!("✓ SAGI Zero-Power Quenching saved {:.2} pJ of Landauer energy.", thermo.total_energy_saved_pj);
}
