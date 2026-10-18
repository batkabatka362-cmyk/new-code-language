// ============================================================================
// CRON VM Integration Test: PRM Step-Level Verifier & GF(2^4) SRAM Swizzling
// Verifies:
//   1. GF(2^4) XOR-Swizzled 16-bank SRAM zero bank collision bijection
//   2. _SR (Store Word) and _LD (Load Word) execution via VM
//   3. _VR (Process Reward Model Verifier) confidence scoring & early pruning
// ============================================================================

use cron_vm::core_engine::CoreEngine;

#[test]
fn test_gf24_sram_swizzling_bijection() {
    let core = CoreEngine::new(0);

    // Verify that for each 16-word row, all 16 bank indices are unique (bijective permutation)
    for row in 0..64 {
        let mut bank_seen = [false; 16];
        for word_in_row in 0..16 {
            let linear_word = row * 16 + word_in_row;
            let swizzled = core.sram_swizzle_word_index(linear_word);
            let bank = swizzled & 0x0F;
            assert!(!bank_seen[bank], "Duplicate bank hit in row {}: bank {}", row, bank);
            bank_seen[bank] = true;
        }
    }
}

#[test]
fn test_sram_swizzled_store_load_roundtrip() {
    let mut core = CoreEngine::new(0);

    // Address in R1 = 0x100, Value in R2 = 0xDEADBEEF
    core.registers[1] = 0x100;
    core.registers[2] = 0xDEADBEEF;

    // Execute _SR02$100> (Store value in R2 to SRAM at address in R1)
    // 10 chars: '_' (0), "SR" (1..2), "02" (3..4: dest bank 0 reg 2 [val]), '$' (5), '1' (6: src reg 1 [addr]), '0' (7), '0' (8), '>' (9)
    core.execute_slot("_SR02$100>");

    // Clear R2 and verify R3 is initially 0
    core.registers[2] = 0;
    core.registers[3] = 0;

    // Execute _LD03$100> (Load word from SRAM at address in R1 into R3)
    core.execute_slot("_LD03$100>");

    assert_eq!(core.registers[3], 0xDEADBEEF, "SRAM store-load roundtrip mismatch");
}

#[test]
fn test_prm_verifier_scoring_and_pruning() {
    let mut core = CoreEngine::new(0);
    core.vr_threshold = 400; // Threshold = 400

    // High entropy step: 16 ones in 32-bit register (balanced bits) in R4
    core.registers[4] = 0x55555555; // 16 ones
    // 10 chars: _VR00$400> (dest = R0, src = R4, mode = '$')
    core.execute_slot("_VR00$400>");
    assert_eq!(core.verifier_register, 992, "High entropy step should score ~992");
    assert_eq!(core.registers[0], 992);
    assert_eq!(core.prm_pruned_steps, 0);

    // Low confidence step: zero step value in R5
    core.registers[5] = 0;
    // 10 chars: _VR00?500> (dest = R0, src = R5, mode = '?')
    core.execute_slot("_VR00?500>"); // '?' mode triggers early pruning check
    assert_eq!(core.verifier_register, 0, "Zero step should score 0");
    assert_eq!(core.prm_pruned_steps, 1, "Early pruning counter must increment when below threshold");
}
