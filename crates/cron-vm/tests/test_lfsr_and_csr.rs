// ============================================================================
// Test: Galois LFSR PRNG, NoC Poll & CSR Performance Counters
// Verifies: _RN LFSR pseudorandom generation, _PL non-blocking poll,
//           CSR read (CYCLE_CNT, STALL_CNT, PRED_EXEC_CNT, VEC_BURST_CNT)
// Milestones: #042, #180
// ============================================================================

use cron_vm::core_engine::CoreEngine;

// ======================== LFSR PRNG Tests ========================

#[test]
fn test_lfsr_produces_nonzero_values() {
    let mut core = CoreEngine::new(0);

    // Execute _RN slot to advance LFSR and write to R1
    core.execute_slot("_RN01$0000>");

    // LFSR output should be non-zero
    assert_ne!(core.registers[1], 0, "LFSR should produce non-zero values");
}

#[test]
fn test_lfsr_produces_distinct_values() {
    let mut core = CoreEngine::new(0);
    let mut values = Vec::new();

    for i in 0..10 {
        let val = core.advance_lfsr();
        assert!(!values.contains(&val), "LFSR cycle {} produced duplicate value {:#010X}", i, val);
        values.push(val);
    }

    // All 10 values should be unique
    assert_eq!(values.len(), 10);
}

#[test]
fn test_lfsr_polynomial_consistency() {
    let mut core = CoreEngine::new(0);
    let initial = core.lfsr_state;

    // Advance many times
    for _ in 0..1000 {
        core.advance_lfsr();
    }

    // After 1000 steps, state should differ from initial
    assert_ne!(core.lfsr_state, initial, "LFSR state should have changed after 1000 advances");
    // And should still be non-zero (Galois LFSR with proper polynomial never reaches 0)
    assert_ne!(core.lfsr_state, 0, "Galois LFSR should never reach zero state");
}

// ======================== NoC Poll Tests ========================

#[test]
fn test_noc_poll_empty_fifo() {
    let mut core = CoreEngine::new(0);

    // Poll with empty FIFO — should write 0
    core.execute_slot("_PL01$0000>");
    assert_eq!(core.registers[1], 0, "Empty NoC FIFO should poll as 0");
}

#[test]
fn test_noc_poll_nonempty_fifo() {
    let mut core = CoreEngine::new(0);

    // Push a packet
    core.noc_push(0xDEAD_BEEF);

    // Poll — should write 1 (packet ready)
    core.execute_slot("_PL01$0000>");
    assert_eq!(core.registers[1], 1, "Non-empty NoC FIFO should poll as 1");
}

#[test]
fn test_noc_push_pop_fifo_ordering() {
    let mut core = CoreEngine::new(0);

    core.noc_push(0xAAAA);
    core.noc_push(0xBBBB);
    core.noc_push(0xCCCC);

    assert_eq!(core.noc_pop(), Some(0xAAAA));
    assert_eq!(core.noc_pop(), Some(0xBBBB));
    assert_eq!(core.noc_pop(), Some(0xCCCC));
    assert_eq!(core.noc_pop(), None);
}

// ======================== CSR Performance Counter Tests ========================

#[test]
fn test_csr_cycle_cnt_increments() {
    let mut core = CoreEngine::new(0);

    assert_eq!(core.read_csr(0), 0, "CSR 0 (CYCLE_CNT) should start at 0");

    // Execute several slots (each increments csr_cycle_cnt)
    core.execute_slot("_NO00#000>");
    core.execute_slot("_NO00#000>");
    core.execute_slot("_NO00#000>");

    assert_eq!(core.read_csr(0), 3, "CSR 0 (CYCLE_CNT) should be 3 after 3 slots");
}

#[test]
fn test_csr_pred_exec_cnt_increments() {
    let mut core = CoreEngine::new(0);

    // Execute predicated operations (PO)
    core.registers[4] = 10;
    core.execute_slot("_PO06$411>");  // ADD: R6 = R6 + R4
    core.execute_slot("_PO06$412>");  // SUB: R6 = R6 - R4

    assert_eq!(core.read_csr(2), 2, "CSR 2 (PRED_EXEC_CNT) should be 2");
}

#[test]
fn test_csr_read_via_rc_slot() {
    let mut core = CoreEngine::new(0);

    // Execute a few operations to build up counters
    core.execute_slot("_NO00#000>");
    core.execute_slot("_NO00#000>");

    // Read CSR 0 (CYCLE_CNT) into R5 via _RC slot with mode 'C'
    // _RC05C0_0> means dest=R5, mode='C', src=0 (CSR ID 0)
    core.execute_slot("_RC05C0_0>");

    // CSR read captures the value at the time of execution (2 from prior NOPs)
    // then the RC slot itself also increments the counter
    assert_eq!(core.registers[5], 2, "R5 should contain CYCLE_CNT at time of read (2)");
}

#[test]
fn test_csr_all_registers_readable() {
    let mut core = CoreEngine::new(0);

    // Manually set CSR values
    core.csr_cycle_cnt = 100;
    core.csr_stall_cnt = 5;
    core.csr_pred_exec_cnt = 42;
    core.csr_vec_burst_cnt = 7;

    assert_eq!(core.read_csr(0), 100);
    assert_eq!(core.read_csr(1), 5);
    assert_eq!(core.read_csr(2), 42);
    assert_eq!(core.read_csr(3), 7);
    assert_eq!(core.read_csr(4), 0, "Invalid CSR ID should return 0");
}
