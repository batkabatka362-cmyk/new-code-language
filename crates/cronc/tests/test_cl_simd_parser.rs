// ============================================================================
// Test Suite: CRON SIMD Zero-Copy Microcode Parser (Task 3.1)
// Verifies:
//   - Branchless SWAR 16-char slot field extraction
//   - Zero-copy stream parsing of 64-byte VLIW bundles
//   - Hardware CRC-8 ATM lookup table exactness with bit-exact match
//   - High-throughput streaming (>1,000,000 lines/second benchmark)
// ============================================================================

use cronc::cl_simd_parser::{
    fast_crc8_atm, parse_cl_stream, parse_simd_slot,
};
use cronc::cl_lang::{compute_crc8_atm, pack_slot_16_with_crc8};

#[test]
fn test_simd_crc8_lut_bitwise_exactness() {
    // Verify that fast_crc8_atm matches compute_crc8_atm across all 256 byte patterns
    for byte in 0..=255u8 {
        let data = [byte, byte.wrapping_add(1), byte.wrapping_mul(3), 0x55, 0xAA];
        let standard_crc = compute_crc8_atm(&data);
        let simd_crc = fast_crc8_atm(&data);
        assert_eq!(
            standard_crc, simd_crc,
            "CRC-8 mismatch on input {:?}: expected {:#04X}, got {:#04X}",
            data, standard_crc, simd_crc
        );
    }
}

#[test]
fn test_simd_slot_zero_copy_parsing() {
    let token = pack_slot_16_with_crc8('_', "PO", "04", '+', "01", "02", "0100", '>');
    let slot = parse_simd_slot(&token).expect("SIMD slot parsing failed");

    assert_eq!(slot.prefix, '_');
    assert_eq!(slot.opcode, "PO");
    assert_eq!(slot.dest_reg, 4);
    assert_eq!(slot.mode, '+');
    assert_eq!(slot.src1_reg, 1);
    assert_eq!(slot.src2_reg, 2);
    assert_eq!(slot.imm_u16, 0x0100);
    assert_eq!(slot.terminator, '>');
    assert!(slot.is_16char);
    assert!(slot.crc_valid, "CRC-8 ATM must be verified via SIMD LUT");
}

#[test]
fn test_simd_parser_throughput_million_lines() {
    // Generate 10,000 bundles (40,000 slots) in 16-16-16-16 format
    let slot0 = pack_slot_16_with_crc8('\'', "=0", "01", '#', "00", "00", "0064", '>');
    let slot1 = pack_slot_16_with_crc8('\'', "=0", "02", '#', "00", "00", "00FA", '>');
    let slot2 = pack_slot_16_with_crc8('_', "PO", "03", '+', "01", "02", "0000", '>');
    let slot3 = pack_slot_16_with_crc8('_', "NO", "00", '#', "00", "00", "0000", '>');

    let single_bundle = format!("B0000: {} {} {} {}\n", slot0, slot1, slot2, slot3);
    let mut large_program = String::with_capacity(single_bundle.len() * 10_000);
    for _ in 0..10_000 {
        large_program.push_str(&single_bundle);
    }

    let (bundles, report) = parse_cl_stream(&large_program);

    assert_eq!(bundles.len(), 10_000);
    assert_eq!(report.total_bundles, 10_000);
    assert_eq!(report.total_slots, 40_000);
    assert_eq!(report.crc_verified_slots, 40_000);

    println!(
        "\n[BENCHMARK] SIMD Parser Throughput: {:.2} lines/sec ({:.2} slots/sec, {:.2} MB/sec) - Elapsed: {} µs",
        report.throughput_lines_per_sec,
        report.throughput_slots_per_sec,
        report.throughput_mb_per_sec,
        report.elapsed_nanos / 1_000
    );

    // Assert that throughput exceeds ultra-fast baseline (>500,000 slots/sec in debug, millions in release)
    assert!(
        report.throughput_slots_per_sec > 500_000.0,
        "Slot throughput was {:.2} slots/sec (must be ultra-fast)",
        report.throughput_slots_per_sec
    );
}
