// ============================================================================
// Test: AOT RAW Hazard Scheduling Pass (Milestone #178)
// Verifies: Register dependency DAG, List Scheduling, RAW/WAW/WAR hazard
//           elimination, independent instruction interleaving into 4-wide bundles.
// ============================================================================

use cronc::scheduler::{AOTHazardScheduler, IRInstruction};
use cronc::codegen::make_slot;

#[test]
fn test_ir_instruction_parsing() {
    // Binary add slot: _PO02$101> -> dest=R2, src1=R1, mode='$'
    let slot_add = make_slot('_', "PO", 2, '$', 1, 1, '>');
    let ir_add = IRInstruction::from_slot(&slot_add);
    assert_eq!(ir_add.dest, Some(2));
    assert!(ir_add.srcs.contains(&1), "Should read R1");
    assert!(ir_add.srcs.contains(&2), "Should read accumulator R2");

    // Pure immediate load: '=10#010.>
    let ir_load = IRInstruction::from_raw("B0001: '=10#010.>");
    assert_eq!(ir_load.dest, Some(1));
    assert!(ir_load.srcs.is_empty(), "Immediate load has no register inputs");

    // Optical GEMM op: _OP041$0E>
    let ir_optical = IRInstruction::from_raw("_OP041$0E>");
    assert!(ir_optical.is_optical, "Should be flagged as optical op");

    // Halt barrier: _HL00#000!>
    let ir_halt = IRInstruction::from_raw("_HL00#000!>");
    assert!(ir_halt.is_barrier, "Should be flagged as barrier");
}

#[test]
fn test_milestone_178_hazard_interleaving() {
    // Recreate the exact test case from Milestone #178 docx:
    // A = 10     (writes R1)
    // B = A + 5  (reads R1, writes R2) - RAW hazard on R1!
    // C = B * 2  (reads R2, writes R3) - RAW hazard on R2!
    // X = 100    (writes R4) - Independent!
    // Y = 200    (writes R5) - Independent!

    let raw_slots = vec![
        make_slot('\'', "=0", 1, '#', 0, 1, '>').raw, // A = 10 (writes R1)
        make_slot('_', "PO", 2, '$', 1, 1, '>').raw,  // B = A + 5 (reads R1, writes R2)
        make_slot('_', "MD", 3, '$', 2, 3, '>').raw,  // C = B * 2 (reads R2, writes R3)
        make_slot('\'', "=0", 4, '#', 0, 4, '>').raw, // X = 100 (writes R4, independent)
        make_slot('\'', "=0", 5, '#', 0, 5, '>').raw, // Y = 200 (writes R5, independent)
    ];

    let scheduler = AOTHazardScheduler::new();
    let bundles = scheduler.schedule_raw_slots(&raw_slots, 1);

    // Unoptimized sequential execution would take 4 bundles due to RAW pipeline stalls.
    // Optimized scheduler interleaves X and Y into Cycle 1, reducing to 3 bundles!
    assert_eq!(bundles.len(), 3, "Expected exactly 3 bundles with instruction interleaving");

    // Bundle 1: should contain A=10 (R1), X=100 (R4), Y=200 (R5), and 1 NOP
    let b1_slots = &bundles[0].slots;
    assert_eq!(b1_slots[0].raw, raw_slots[0], "Slot 0 should be A=10");
    assert_eq!(b1_slots[1].raw, raw_slots[3], "Slot 1 should be interleaved X=100");
    assert_eq!(b1_slots[2].raw, raw_slots[4], "Slot 2 should be interleaved Y=200");
    assert_eq!(b1_slots[3].raw, "_NO00#000>", "Slot 3 should be NOP pad");

    // Bundle 2: should contain B = A + 5 (R2) since R1 is now ready from Cycle 1
    let b2_slots = &bundles[1].slots;
    assert_eq!(b2_slots[0].raw, raw_slots[1], "Slot 0 should be B=A+5");
    assert_eq!(b2_slots[1].raw, "_NO00#000>");

    // Bundle 3: should contain C = B * 2 (R3) since R2 is now ready from Cycle 2
    let b3_slots = &bundles[2].slots;
    assert_eq!(b3_slots[0].raw, raw_slots[2], "Slot 0 should be C=B*2");
}

#[test]
fn test_waw_hazard_prevention() {
    // Two instructions write to R1:
    // 1. R1 = 10
    // 2. R1 = 20
    // They must NOT be scheduled in the same bundle.
    let raw_slots = vec![
        make_slot('\'', "=0", 1, '#', 0, 1, '>').raw,
        make_slot('\'', "=0", 1, '#', 0, 2, '>').raw,
    ];

    let scheduler = AOTHazardScheduler::new();
    let bundles = scheduler.schedule_raw_slots(&raw_slots, 1);

    assert_eq!(bundles.len(), 2, "WAW hazard must force write to next cycle");
    assert_eq!(bundles[0].slots[0].raw, raw_slots[0]);
    assert_eq!(bundles[1].slots[0].raw, raw_slots[1]);
}

#[test]
fn test_optical_structural_hazard() {
    // Two optical operations:
    // Only 1 optical operation can execute per hardware cycle.
    let raw_slots = vec![
        make_slot('_', "OP", 4, '$', 1, 0xE, '>').raw,
        make_slot('_', "OP", 5, '$', 2, 0xE, '>').raw,
    ];

    let scheduler = AOTHazardScheduler::new();
    let bundles = scheduler.schedule_raw_slots(&raw_slots, 1);

    assert_eq!(bundles.len(), 2, "Optical structural hazard must separate MZI ops across cycles");
    assert_eq!(bundles[0].slots[0].raw, raw_slots[0]);
    assert_eq!(bundles[1].slots[0].raw, raw_slots[1]);
}

#[test]
fn test_barrier_isolation() {
    // Barrier instruction (_HL halt) terminates the bundle immediately
    let raw_slots = vec![
        make_slot('\'', "=0", 1, '#', 0, 1, '>').raw,
        make_slot('_', "HL", 0, '#', 0, 0, '!').raw, // Barrier
        make_slot('\'', "=0", 2, '#', 0, 2, '>').raw,
    ];

    let scheduler = AOTHazardScheduler::new();
    let bundles = scheduler.schedule_raw_slots(&raw_slots, 1);

    assert_eq!(bundles.len(), 2, "Barrier must flush bundle");
    // Bundle 1 has R1 = 10 and Halt
    assert_eq!(bundles[0].slots[0].raw, raw_slots[0]);
    assert_eq!(bundles[0].slots[1].raw, raw_slots[1]);
    // Bundle 2 has R2 = 20
    assert_eq!(bundles[1].slots[0].raw, raw_slots[2]);
}
