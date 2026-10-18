// ============================================================================
// Test Suite: CRON-VM 16-Character VLIW Slot Hardware Execution Engine
// Verifies:
//   - 3-Address Code execution: R_dest = R_src1 [op] R_src2 in _PO
//   - 16-bit Immediate loading in '=0'
//   - 3-Address Sub-Byte Ternary MAC in _MD: R_dest = R_src1 (dot) R_src2
//   - 16-bit Scratchpad SRAM offset in _LD and _SR
//   - 100% Bitwise match & backward compatibility for 10-char slots
// ============================================================================

use cron_vm::core_engine::CoreEngine;

#[test]
fn test_vm_16char_3address_addition() {
    let mut vm = CoreEngine::new(0);

    // B0000: Load R1 = 100, R2 = 250
    // '=001#00000064E> -> R1 = 0x64 (100)
    // '=002#000000FAE> -> R2 = 0xFA (250)
    vm.execute_slot("B0000: '=001#00000064E> '=002#000000FAE> _NO00#000000000> _NO00#000000000>");
    // Execute individual slots
    vm.execute_slot("'=001#000000640>");
    assert_eq!(vm.registers[1], 100, "R1 should be 100");

    vm.execute_slot("'=002#000000FA0>");
    assert_eq!(vm.registers[2], 250, "R2 should be 250");

    // 3-Address Addition: R3 = R1 + R2
    // prefix='_', op="PO", dest="03", mode='+', src1="01", src2="02", imm="0000", term='>'
    // token: _PO03+010200000>
    vm.execute_slot("_PO03+010200000>");
    assert_eq!(vm.registers[3], 350, "R3 should equal R1 + R2 = 100 + 250 = 350");
}

#[test]
fn test_vm_16char_3address_subtraction() {
    let mut vm = CoreEngine::new(0);

    vm.registers[1] = 500;
    vm.registers[2] = 120;

    // 3-Address Subtraction: R4 = R1 - R2
    vm.execute_slot("_PO04-010200000>");
    assert_eq!(vm.registers[4], 380, "R4 should equal R1 - R2 = 500 - 120 = 380");
}

#[test]
fn test_vm_16char_16bit_immediate_arithmetic() {
    let mut vm = CoreEngine::new(0);
    vm.registers[1] = 1000;

    // R2 = R1 + 0x0500 (1280) via immediate mode (src2=00, imm=0500)
    vm.execute_slot("_PO02+010005000>");
    assert_eq!(vm.registers[2], 1000 + 1280, "R2 should equal R1 + 0x0500 = 2280");
}

#[test]
fn test_vm_16char_sram_load_store_16bit_offset() {
    let mut vm = CoreEngine::new(0);
    vm.registers[1] = 0xCAFEBABE; // Data word
    vm.registers[2] = 0;          // Base address

    // Store R1 to SRAM at base R2 with 16-bit offset 0x0100 (256): _SR01#020001000>
    vm.execute_slot("_SR01#020001000>");

    // Clear R3 and load back from SRAM at base R2 with 16-bit offset 0x0100: _LD03#020001000>
    vm.registers[3] = 0;
    vm.execute_slot("_LD03#020001000>");
    assert_eq!(vm.registers[3], 0xCAFEBABE, "R3 should load stored value 0xCAFEBABE from offset 0x0100");
}

#[test]
fn test_vm_16char_ternary_mac_3address() {
    let mut vm = CoreEngine::new(0);

    // BitNet b1.58 ternary packed weights:
    // 0x55555555 = all +1 trits
    vm.registers[1] = 0x5555_5555;
    vm.registers[2] = 0x5555_5555;

    // 3-Address Ternary Dot Product MAC: R5 = R1 (dot) R2
    // Dot product of 16 identical (+1) elements = 16
    vm.execute_slot("_MD05.010200000>");
    assert_eq!(vm.registers[5], 16, "R5 should equal dot product = 16");
}

#[test]
fn test_vm_16char_ternary_mac_in_slot_weights() {
    let mut vm = CoreEngine::new(0);

    // BitNet b1.58 activations in R1: 0x55555555 (16x +1 trits)
    vm.registers[1] = 0x5555_5555;

    // In-Slot 16-Bit Ternary MAC: R6 = R1 (dot) [In-slot 16-bit weights 0x5555]
    // prefix='_', op="MD", dest="06", mode='.', src1="01", src2="00", imm="5555", term='>'
    // token: _MD06.010055550>
    vm.execute_slot("_MD06.010055550>");
    assert_eq!(vm.registers[6], 16, "R6 should compute dot product with in-slot 16-bit weights = 16");
}

#[test]
fn test_vm_16char_65536_unconditional_jump() {
    let mut vm = CoreEngine::new(0);

    // Unconditional jump to 0xFFFF (65,535): _JP00#0000FFFF0>
    vm.execute_slot("_JP00#0000FFFF0>");
    assert_eq!(vm.branch_target, Some(0xFFFF), "Branch target should resolve to 65,535 (0xFFFF)");
}

#[test]
fn test_vm_16char_65536_conditional_branches() {
    let mut vm = CoreEngine::new(0);

    // Test BZ (Branch if Zero) on R1 == 0 to 0x8000 (32,768)
    vm.registers[1] = 0;
    vm.execute_slot("_BZ01#000080000>");
    assert_eq!(vm.branch_target, Some(0x8000), "Branch target should resolve to 32,768 when R1 == 0");

    // Clear branch target and set R1 != 0
    vm.branch_target = None;
    vm.registers[1] = 42;
    vm.execute_slot("_BZ01#000080000>");
    assert_eq!(vm.branch_target, None, "Branch should NOT be taken when R1 != 0");

    // Test BN (Branch if Non-Zero) on R1 != 0 to 0x4000 (16,384)
    vm.execute_slot("_BN01#000040000>");
    assert_eq!(vm.branch_target, Some(0x4000), "Branch target should resolve to 16,384 when R1 != 0");
}

#[test]
fn test_vm_16char_65536_subroutine_call_and_return() {
    let mut vm = CoreEngine::new(0);
    vm.cycle_count = 100;

    // Subroutine call to 0x1234: _JS00#000012340>
    vm.execute_slot("_JS00#000012340>");
    assert_eq!(vm.branch_target, Some(0x1234), "Call target should be 0x1234");
    assert_eq!(vm.ras.last().copied(), Some(101), "Return address 101 must be saved on RAS");

    // Return from subroutine: _RT00#000000000>
    vm.branch_target = None;
    vm.execute_slot("_RT00#000000000>");
    assert_eq!(vm.branch_target, Some(101), "Return from subroutine should target saved address 101");
}

#[test]
fn test_vm_16char_flat_64kb_sram_addressing() {
    let mut vm = CoreEngine::new(0);
    vm.registers[1] = 0x1234_5678; // Word to store
    vm.registers[2] = 0;          // Base address

    // Store word to max offset 0x3FFC (16380 word boundary within 64KB SRAM): _SR01#02003FFC0>
    vm.execute_slot("_SR01#02003FFC0>");

    // Clear R3 and load back: _LD03#02003FFC0>
    vm.registers[3] = 0;
    vm.execute_slot("_LD03#02003FFC0>");
    assert_eq!(vm.registers[3], 0x1234_5678, "Loaded value must match 0x12345678 from flat 64KB SRAM");
}

#[test]
fn test_vm_variable_width_power_gating_simulation() {
    use cron_vm::simulator::Simulator;

    let mut sim = Simulator::new();
    let cl_code = r#"
// Bundle 0: 2 active slots, 2 power-gated slots (saving 2x 120 uW)
B0000: '=001#0000000A0> '=002#000000140>
// Bundle 1: 1 active slot, 3 power-gated slots (saving 3x 120 uW)
B0001: _PO03+010200000>
// Bundle 2: 4 active slots (0 power-gated slots)
B0002: _NO00#000000000> _NO00#000000000> _NO00#000000000> _HL00#00000000!>
"#;
    sim.load_machine_code(cl_code);
    assert_eq!(sim.instructions.len(), 3);
    assert_eq!(sim.instructions[0].slots.len(), 2, "Bundle 0 has 2 slots");
    assert_eq!(sim.instructions[1].slots.len(), 1, "Bundle 1 has 1 slot");
    assert_eq!(sim.instructions[2].slots.len(), 4, "Bundle 2 has 4 slots");

    while sim.step() {}

    // R3 = R1 + R2 = 10 + 20 = 30
    assert_eq!(sim.cores[0].registers[3], 30);
    // Bundle 0 had 2 gated, Bundle 1 had 3 gated -> total power gated slots = 5
    assert_eq!(sim.stats.power_gated_slots, 5, "Must record exactly 5 power-gated inactive slots");
    assert_eq!(sim.stats.total_energy_saved_uw, 5 * 120, "Must record 600 uW total energy saved via power-gating");
}


