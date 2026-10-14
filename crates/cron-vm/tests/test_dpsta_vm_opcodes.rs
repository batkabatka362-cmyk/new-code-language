use cron_vm::core_engine::CoreEngine;
use cron_vm::debugger::disassemble_slot;

#[test]
fn test_dpsta_4tact_discrete_transition_in_vm() {
    let mut core = CoreEngine::new(0);
    // Input token in R2 = 42 ('K')
    core.registers[2] = 42;
    // Execute _DP01$200>: dest=R1, src=R2
    core.execute_slot("_DP01$200>");

    assert_eq!(core.dpsta_transitions_count, 1, "DPSTA transitions counter must increment");
    let state = core.registers[1];
    assert!(state < 94, "DPSTA state must remain within 94-state bounds, was {}", state);
    assert_eq!(core.dpsta_current_state, state as u8, "Core current state must match dest reg");

    // Second transition with another token in R2 = 15 ('0')
    core.registers[2] = 15;
    core.execute_slot("_DP01$200>");
    assert_eq!(core.dpsta_transitions_count, 2);
    let state2 = core.registers[1];
    assert!(state2 < 94);
    assert_ne!(state, state2, "Successive transitions must produce distinct states");
}

#[test]
fn test_htgm_attractor_resonance_in_vm() {
    let mut core = CoreEngine::new(0);
    // Write a pattern into attractor ring slot 0: _HT02=100> (mode '=', imm=0)
    core.registers[1] = 0xAAAA_5555;
    core.execute_slot("_HT02=100>");
    assert_eq!(core.htgm_ring_memory[0], 0xAAAA_5555);
    assert_eq!(core.htgm_resonances_count, 1);

    // Query with matching pattern
    core.registers[1] = 0xAAAA_5555;
    core.execute_slot("_HT03$100>");
    assert_eq!(core.htgm_resonances_count, 2);
    let result = core.registers[3];
    let matched_slot = (result >> 16) as usize;
    let score = (result & 0xFFFF) as u32;
    assert_eq!(matched_slot, 0, "Must match slot 0");
    assert_eq!(score, 32, "Exact match must yield max score of 32 bits");
}

#[test]
fn test_clifford_4d_rotor_rotation_in_vm() {
    let mut core = CoreEngine::new(0);
    // Pack 4D vector [10, 20, 30, 40] into R2
    core.registers[2] = 10 | (20 << 8) | (30 << 16) | (40 << 24);

    // Identity rotation (imm=0): _CR01$200>
    core.execute_slot("_CR01$200>");
    assert_eq!(core.registers[1], core.registers[2], "Imm 0 must be identity rotation");
    assert_eq!(core.clifford_rotations_count, 1);

    // Plane 0-1 rotation (imm=1): _CR01$201>
    core.execute_slot("_CR01$201>");
    assert_eq!(core.clifford_rotations_count, 2);
    let r0 = (core.registers[1] & 0xFF) as u8;
    let r1 = ((core.registers[1] >> 8) & 0xFF) as u8;
    // Expected: -v1 = -20 = 236 (u8), v0 = 10
    assert_eq!(r0, (-20i8) as u8);
    assert_eq!(r1, 10);
}

#[test]
fn test_tropical_semi_ring_arbiter_in_vm() {
    let mut core = CoreEngine::new(0);
    core.registers[1] = 50;  // current max
    core.registers[2] = 40;  // candidate base
    // Tropical step with imm=4 -> candidate = 40 + 4 = 44 < 50
    core.execute_slot("_TS01$204>"); // imm=4 -> 40 + 4 = 44 < 50
    assert_eq!(core.registers[1], 50, "50 > 44, must keep 50");

    core.registers[2] = 45;
    core.execute_slot("_TS01$208>"); // imm=8 -> 45 + 8 = 53 > 50
    assert_eq!(core.registers[1], 53, "53 > 50, must update to 53");
    assert_eq!(core.tropical_arbiter_count, 2);
}

#[test]
fn test_subbyte_ternary_pack_and_unpack_in_vm() {
    let mut core = CoreEngine::new(0);
    // Pack 4 signed bytes into trits using PK
    core.registers[1] = 0x20_E0_20_00; // trits: +1, -1, +1, 0
    core.execute_slot("_PK02$100>");
    let packed = core.registers[2];
    assert_ne!(packed, 0);

    // Unpack using UP: _UP03$200>
    core.execute_slot("_UP03$200>");
    let unpacked = core.registers[3];
    assert_ne!(unpacked, 0);
}

#[test]
fn test_dpsta_disassembler_mnemonics() {
    let slot_dp = disassemble_slot("_DP01$200>");
    assert_eq!(slot_dp.opcode, "DP");
    assert!(slot_dp.mnemonic.contains("DPSTA_STEP"));

    let slot_ht = disassemble_slot("_HT02=100>");
    assert_eq!(slot_ht.opcode, "HT");
    assert!(slot_ht.mnemonic.contains("HTGM_RESON"));

    let slot_cr = disassemble_slot("_CR03$201>");
    assert_eq!(slot_cr.opcode, "CR");
    assert!(slot_cr.mnemonic.contains("CLIFFORD_ROT"));

    let slot_tr = disassemble_slot("_TS01$208>");
    assert_eq!(slot_tr.opcode, "TS");
    assert!(slot_tr.mnemonic.contains("TROPICAL_MAX"));

    let slot_up = disassemble_slot("_UP02$100>");
    assert_eq!(slot_up.opcode, "UP");
    assert!(slot_up.mnemonic.contains("TERNARY_UNPACK"));
}

#[test]
fn test_cortical_column_stdp_simulator_execution() {
    use cron_vm::simulator::Simulator;

    let source = r#"
.core [0, 0, 0, 0]:
@cortical_column_0_init:
B0000: '==01#000> '==02#004> '==03#008> '==04#00C>
B0001: _LY00$000> _LY01$001> _LY02$002> _LY03$003>

@cortical_column_0_stdp_feedforward:
B0002: _DP01$200> _HT02$100> _CR03$201> _TS04$208>
B0003: _HB01$200> _ST02#400> _HP00$000> _SB00#000>
B0004: _UP01$200> _PO02+100> _NO00#000> _HL00$008!
"#;

    let mut sim = Simulator::new();
    sim.load_machine_code(source);
    assert_eq!(sim.instructions.len(), 5);

    // Step through execution until halt
    let mut steps = 0;
    while !sim.cores[0].is_halted && steps < 10 {
        sim.step();
        steps += 1;
    }

    assert!(sim.cores[0].is_halted, "Core 0 must halt after B0004");
    assert!(sim.cores[0].dpsta_transitions_count >= 1, "Must execute DPSTA transition");
    assert!(sim.cores[0].htgm_resonances_count >= 1, "Must execute HTGM resonance");
    assert!(sim.cores[0].clifford_rotations_count >= 1, "Must execute Clifford rotation");
    assert!(sim.cores[0].tropical_arbiter_count >= 1, "Must execute Tropical arbiter");
    assert!(sim.cores[0].stdp_updates_count >= 1, "Must execute STDP updates");
}
