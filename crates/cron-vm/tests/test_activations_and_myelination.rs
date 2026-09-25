use cron_vm::core_engine::CoreEngine;
use cron_vm::simulator::Simulator;

#[test]
fn test_hardware_activations_gelu_sigmoid_tanh_relu() {
    let mut core = CoreEngine::new(0);

    // Test ReLU (RL): Positive value should pass through, negative should clamp to 0
    // Slot format: _RL 0 5 $ 1 0 0 > -> op=RL, dest_bank=0, dest_reg=5, mode=$, src=1, parity=0, imm=0
    core.registers[1] = 100;
    core.execute_slot("_RL05$100>");
    assert_eq!(core.registers[5], 100, "ReLU of 100 should be 100");

    core.registers[1] = (-50i32) as u32;
    core.execute_slot("_RL05$100>");
    assert_eq!(core.registers[5], 0, "ReLU of -50 should clamp to 0");

    // Test Sigmoid (SI): x = 0 (in Q16.16 is 0) -> Sigmoid(0) = 0.5 -> 32768 in Q16.16
    core.registers[2] = 0;
    core.execute_slot("_SI06$200>");
    let sig_val = core.registers[6];
    assert!((sig_val as i64 - 32768).abs() <= 10, "Sigmoid(0) in Q16.16 should be ~32768, got {}", sig_val);

    // Test Tanh (TA): x = 0 -> Tanh(0) = 0
    core.registers[3] = 0;
    core.execute_slot("_TA07$300>");
    assert_eq!(core.registers[7], 0, "Tanh(0) should be 0");

    // Test GELU (GE): x = 0 -> GELU(0) = 0
    core.registers[4] = 0;
    core.execute_slot("_GE08$400>");
    assert_eq!(core.registers[8], 0, "GELU(0) should be 0");
}

#[test]
fn test_epigenetic_axon_myelination_in_simulator() {
    let mut sim = Simulator::new();
    let cl_code = r#"
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0001: _LI0C$005> _CD05$050> _OD0E$0E0> _GE08$040>
B0002: _HL00#000! _NO00#000> _NO00#000> _NO00#000>
"#;

    sim.load_machine_code(cl_code);
    sim.run();

    assert_eq!(sim.stats.axon_myelinated_bundles, 1, "Bundle 1 should reach myelination threshold");
    assert!(sim.stats.epigenetic_morph_cycles_saved > 0, "Myelinated bundle should save cycles");
}
