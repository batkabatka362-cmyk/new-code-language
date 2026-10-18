// ============================================================================
// Test Suite: 16-16-16-16 VLIW Machine Code Architecture (64 Bytes / 512-bit)
// Tests:
//   - 16-character slot token parsing and field extraction (Rd, Rs1, Rs2, Imm16, CRC)
//   - CRC-8 ATM token calculation and verification (pack_slot_16_with_crc8)
//   - 4-slot 64-byte bundle program verification and RAW hazard detection on Rs2
//   - Backward compatibility with legacy 10-character slots
// ============================================================================

use cronc::cl_lang::{
    parse_slot, parse_slot_immediate, verify_cl_program, verify_token_crc8,
    pack_slot_16_with_crc8,
};
use cronc::cl_heal::heal_slot_crc;

#[test]
fn test_16char_slot_parsing() {
    // Layout:
    // [0]    '_'
    // [1..2] "PO"
    // [3..4] "01" (R1)
    // [5]    '+'
    // [6..7] "02" (R2)
    // [8..9] "03" (R3)
    // [10..13] "0005" (imm 5)
    // [14]   CRC nibble
    // [15]   '>'
    let token = pack_slot_16_with_crc8('_', "PO", "01", '+', "02", "03", "0005", '>');
    assert_eq!(token.len(), 16, "16-char slot must be exactly 16 ASCII characters");
    assert!(verify_token_crc8(&token), "CRC-8 verification failed for 16-char token: {}", token);

    let parsed = parse_slot(&token).expect("Failed to parse 16-char slot");
    assert_eq!(parsed.prefix, '_');
    assert_eq!(parsed.opcode, "PO");
    assert_eq!(parsed.dest_reg, Some(1));
    assert_eq!(parsed.mode, '+');
    assert_eq!(parsed.src_reg, Some(2));
    assert_eq!(parsed.src2_reg, Some(3));
    assert_eq!(parsed.imm_u16, 5);
    assert_eq!(parsed.terminator, '>');
    assert!(parsed.is_16char);

    let imm_val = parse_slot_immediate(&token);
    assert_eq!(imm_val, Some(5));
}

#[test]
fn test_16char_64byte_bundle_program_verification() {
    // 4 slots x 16 chars = 64 characters = exactly 1 L1 cache line / 512-bit VLIW bundle
    let slot0 = pack_slot_16_with_crc8('\'', "=0", "01", '#', "00", "00", "1234", '>');
    let slot1 = pack_slot_16_with_crc8('\'', "=0", "02", '#', "00", "00", "0056", '>');
    let slot2 = pack_slot_16_with_crc8('_', "PO", "03", '+', "01", "02", "0000", '>');
    let slot3 = pack_slot_16_with_crc8('_', "NO", "00", '#', "00", "00", "0000", '>');

    let program = format!("B0000: {} {} {} {}\n", slot0, slot1, slot2, slot3);
    let report = verify_cl_program(&program).expect("Bundle verification should succeed");
    assert_eq!(report.total_bundles, 1);
    assert_eq!(report.total_slots, 4);
    assert_eq!(report.opcodes_verified, 4);
    assert_eq!(report.strict_crc_verified, 4);
}

#[test]
fn test_16char_raw_hazard_detection_on_src2() {
    // Bundle with RAW hazard: Slot 0 writes to R1, Slot 1 reads R1 as src2!
    let slot0 = pack_slot_16_with_crc8('\'', "=0", "01", '#', "00", "00", "0042", '>');
    let slot1 = pack_slot_16_with_crc8('_', "PO", "05", '+', "03", "01", "0000", '>');
    let slot2 = pack_slot_16_with_crc8('_', "NO", "00", '#', "00", "00", "0000", '>');
    let slot3 = pack_slot_16_with_crc8('_', "NO", "00", '#', "00", "00", "0000", '>');

    let program = format!("B0000: {} {} {} {}\n", slot0, slot1, slot2, slot3);
    let report = verify_cl_program(&program).expect("Program should verify with hazard warnings");
    assert!(!report.hazards.is_empty(), "Expected RAW hazard detected on src2 R1");
    assert!(report.hazards[0].contains("R1 (src2)"));
}

#[test]
fn test_16char_healing() {
    // Provide a corrupted 16-char slot with broken CRC nibble
    let bad_token = "_PO04+01020000?>";
    assert_eq!(bad_token.len(), 16);
    let (healed, was_repaired) = heal_slot_crc(bad_token);
    assert!(was_repaired);
    assert_eq!(healed.len(), 16);
    assert!(verify_token_crc8(&healed), "Healed slot must have valid CRC-8: {}", healed);
}

#[test]
fn test_mixed_10char_and_16char_backward_compatibility() {
    // Both 10-char and 16-char slots parse and validate seamlessly
    let legacy_slot = "_PO01+203>"; // 10 chars
    let parsed10 = parse_slot(legacy_slot).expect("Legacy 10-char slot must parse");
    assert!(!parsed10.is_16char);
    assert_eq!(parsed10.src2_reg, None);

    let modern_slot = pack_slot_16_with_crc8('_', "PO", "01", '+', "02", "03", "0000", '>'); // 16 chars
    let parsed16 = parse_slot(&modern_slot).expect("Modern 16-char slot must parse");
    assert!(parsed16.is_16char);
    assert_eq!(parsed16.src2_reg, Some(3));
}

#[test]
fn test_variable_width_power_gating_optimizer() {
    use cronc::cl_opt::{optimize_cl_program_advanced, ClOptConfig, ClOptLevel};

    let source = r#"
B0000: '=001#0000000A0> _NO00#000000000> _NO00#000000000> _NO00#000000000>
B0001: '=002#000000140> _NO00#000000000> _NO00#000000000> _NO00#000000000>
B0002: _PO03+010200000> _NO00#000000000> _NO00#000000000> _NO00#000000000>
"#;
    let config = ClOptConfig {
        level: ClOptLevel::Level2,
        enable_peephole: true,
        enable_fusion: true,
        target_ipc: 4.0,
        enable_thermal_guard: false,
        max_consecutive_high_power: 3,
        variable_width_power_gated: true,
    };

    let report = optimize_cl_program_advanced(source, &config).expect("Optimization should succeed");
    assert!(report.power_gated_slots > 0, "Must have power-gated inactive slots");
    assert!(report.estimated_power_saved_watts > 0.0, "Must calculate positive power savings");
    println!("Power-Gating Optimizer Report:\n  Power-gated slots: {}\n  Estimated Power Saved: {:.2}W\nCode:\n{}", 
        report.power_gated_slots, report.estimated_power_saved_watts, report.optimized_code);
}

#[test]
fn test_variable_width_simd_parser_power_gating() {
    use cronc::cl_simd_parser::parse_cl_stream;

    let source = r#"
B0000: '=001#0000000A0> '=002#000000140>
B0001: _PO03+010200000>
B0002: _NO00#000000000> _NO00#000000000> _NO00#000000000> _HL00#00000000!>
"#;
    let (bundles, report) = parse_cl_stream(source);
    assert_eq!(bundles.len(), 3);
    assert_eq!(bundles[0].slot_count, 2);
    assert_eq!(bundles[1].slot_count, 1);
    assert_eq!(bundles[2].slot_count, 4);

    // Bundle 0 has 2 gated, Bundle 1 has 3 gated -> total = 5 gated
    assert_eq!(report.power_gated_slots, 5);
    assert!(report.energy_saved_pj > 0.5, "Energy saved must be tracked");
    println!("SIMD Parser Power-Gating Report: {} slots gated, {:.3} pJ energy saved", report.power_gated_slots, report.energy_saved_pj);
}

#[test]
fn test_16char_65536_jump_and_branch_encodings() {
    let jp_slot = pack_slot_16_with_crc8('_', "JP", "00", '#', "00", "00", "FFFF", '>');
    let bz_slot = pack_slot_16_with_crc8('_', "BZ", "01", '#', "00", "00", "8000", '>');
    let bn_slot = pack_slot_16_with_crc8('_', "BN", "02", '#', "00", "00", "4000", '>');
    let js_slot = pack_slot_16_with_crc8('_', "JS", "00", '#', "00", "00", "1234", '>');

    for slot in &[&jp_slot, &bz_slot, &bn_slot, &js_slot] {
        assert_eq!(slot.len(), 16);
        assert!(verify_token_crc8(slot));
    }

    let parsed_jp = parse_slot(&jp_slot).unwrap();
    assert_eq!(parsed_jp.opcode, "JP");
    assert_eq!(parsed_jp.imm_u16, 0xFFFF);

    let parsed_bz = parse_slot(&bz_slot).unwrap();
    assert_eq!(parsed_bz.opcode, "BZ");
    assert_eq!(parsed_bz.imm_u16, 0x8000);

    let parsed_bn = parse_slot(&bn_slot).unwrap();
    assert_eq!(parsed_bn.opcode, "BN");
    assert_eq!(parsed_bn.imm_u16, 0x4000);

    let parsed_js = parse_slot(&js_slot).unwrap();
    assert_eq!(parsed_js.opcode, "JS");
    assert_eq!(parsed_js.imm_u16, 0x1234);
}

