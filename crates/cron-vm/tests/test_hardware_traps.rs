// ============================================================================
// Test: Hardware Traps & Fault Handling (Milestone #000, #181)
// Verifies: div-by-zero trap, MCAUSE/MEPC registers, ISR vector 0x0004,
//           MRET (_RT) return from trap, shadow checkpoint bank
// ============================================================================

use cron_vm::core_engine::CoreEngine;

#[test]
fn test_div_by_zero_triggers_trap() {
    let mut core = CoreEngine::new(0);

    // Set up: R6 = 100, R4 = 0 (divisor is zero)
    core.registers[6] = 100;
    core.registers[4] = 0;

    // Execute: PO R6 with imm=4 (division), src=R4
    // _PO06$4_4> means dest=R6, mode=$, src=4, imm=4
    core.execute_slot("_PO06$444>");

    // Verify trap was triggered
    assert_eq!(core.mcause, 0x0001, "MCAUSE should be 0x0001 (Div-by-Zero)");
    assert!(core.in_trap, "Core should be in trap state");
    assert_eq!(core.trap_count, 1, "Trap count should be 1");
    assert_eq!(core.trap_handler_addr, 0x0004, "Trap handler should be at 0x0004");
}

#[test]
fn test_mret_clears_trap_state() {
    let mut core = CoreEngine::new(0);

    // Manually trigger a trap
    core.trigger_trap(0x0001);
    assert!(core.in_trap);
    assert_eq!(core.mcause, 0x0001);

    // Execute MRET (_RT)
    core.execute_slot("_RT00$0000>");

    // Verify trap state is cleared
    assert!(!core.in_trap, "in_trap should be false after MRET");
    assert_eq!(core.mcause, 0, "MCAUSE should be cleared after MRET");
}

#[test]
fn test_trap_preserves_mepc() {
    let mut core = CoreEngine::new(0);
    core.cycle_count = 42;

    // Trigger trap
    core.trigger_trap(0x0001);

    // MEPC should capture the faulting cycle
    assert_eq!(core.mepc, 42, "MEPC should be the faulting cycle (42)");
}

#[test]
fn test_multiple_traps_increment_counter() {
    let mut core = CoreEngine::new(0);

    core.trigger_trap(0x0001);
    core.in_trap = false; // simulate handler clearing
    core.trigger_trap(0x0002);
    core.in_trap = false;
    core.trigger_trap(0x0001);

    assert_eq!(core.trap_count, 3, "Should have 3 total traps");
}

#[test]
fn test_shadow_checkpoint_save_restore() {
    let mut core = CoreEngine::new(0);

    // Set up register state
    core.registers[1] = 0xAAAA;
    core.registers[2] = 0xBBBB;
    core.registers[3] = 0xCCCC;

    // Save checkpoint: _RC00#000.!>  (imm=0, mode='!')
    core.execute_slot("_RC00!0_0>");

    // Modify registers
    core.registers[1] = 0x1111;
    core.registers[2] = 0x2222;
    core.registers[3] = 0x3333;

    // Restore checkpoint: _RC00#001.!> (imm=1, mode='!')
    core.execute_slot("_RC00!0_1>");

    // Verify registers are restored
    assert_eq!(core.registers[1], 0xAAAA, "R1 should be restored");
    assert_eq!(core.registers[2], 0xBBBB, "R2 should be restored");
    assert_eq!(core.registers[3], 0xCCCC, "R3 should be restored");
}
