// ============================================================================
// Test Suite: CRON-VM 16-Character 256-Register Architecture & 3-Address Ops
// Verifies:
//   - Full 256-register residency across Bank 0..15 without SRAM spilling
//   - FM (Fused Multiply-Add) 3-Address: R_dest = R_src1 * R_src2 + Imm16 & accumulator
//   - MX (Max / Min / ReLU / Clamp) 3-Address
//   - SW (Vector Swizzle / Rotation / ByteSwap) 3-Address
//   - FP (In-Slot 16-bit Float FP16 IEEE 754 arithmetic)
// ============================================================================

use cron_vm::core_engine::CoreEngine;

#[test]
fn test_vm_256_register_full_residency() {
    let mut vm = CoreEngine::new(0);

    // Write to Register 0x2A (Bank 2, Reg 10) and Register 0x8F (Bank 8, Reg 15)
    // 16-character format: '=0' (prefix + op), '2A' (dest 0x2A), '#' (mode), '00' (src1), '00' (src2), '007B' (imm 123), '0' (crc), '>' (term)
    vm.execute_slot("'=02A#0000007B0>");
    vm.execute_slot("'=08F#000001C80>");

    assert_eq!(vm.read_reg_256(0x2A), 123, "R[0x2A] must be 123");
    assert_eq!(vm.read_reg_256(0x8F), 456, "R[0x8F] must be 456");

    // 3-Address Addition across distant register banks:
    // R[0xFE] = R[0x2A] + R[0x8F]
    // Prefix='_', Op="PO", Dest="FE", Mode='+', Src1="2A", Src2="8F", Imm="0000", Term='>'
    vm.execute_slot("_POFE+2A8F00000>");

    assert_eq!(vm.read_reg_256(0xFE), 123 + 456, "R[0xFE] must equal R[0x2A] + R[0x8F] = 579");
}

#[test]
fn test_vm_3address_fma_fm() {
    let mut vm = CoreEngine::new(0);

    // Set R1 = 7, R2 = 9
    vm.write_reg_256(1, 7);
    vm.write_reg_256(2, 9);

    // FM with mode '+': R3 = R1 * R2 + 0x0005 (7 * 9 + 5 = 68)
    // Slot: _FM03+010200050>
    vm.execute_slot("_FM03+010200050>");
    assert_eq!(vm.read_reg_256(3), 68, "R3 should equal 7 * 9 + 5 = 68");

    // FM with mode '*': Accumulator FMA: R3 += R1 * R2 (68 + 63 = 131)
    vm.execute_slot("_FM03*010200000>");
    assert_eq!(vm.read_reg_256(3), 131, "R3 should accumulate to 131");
}

#[test]
fn test_vm_3address_max_min_relu_clamp_mx() {
    let mut vm = CoreEngine::new(0);

    vm.write_reg_256(1, 150);
    vm.write_reg_256(2, 280);

    // Max: R3 = max(R1, R2) = 280
    vm.execute_slot("_MX03>010200000>");
    assert_eq!(vm.read_reg_256(3), 280, "Max should be 280");

    // Min: R4 = min(R1, R2) = 150
    vm.execute_slot("_MX04<010200000>");
    assert_eq!(vm.read_reg_256(4), 150, "Min should be 150");

    // ReLU: R5 = ReLU(signed(-50)) = 0
    vm.write_reg_256(5, (-50i32) as u32);
    vm.execute_slot("_MX06+050000000>");
    assert_eq!(vm.read_reg_256(6), 0, "ReLU of negative number should be 0");

    // ReLU: R6 = ReLU(120) = 120
    vm.write_reg_256(5, 120);
    vm.execute_slot("_MX06+050000000>");
    assert_eq!(vm.read_reg_256(6), 120, "ReLU of positive number should be 120");

    // Clamp: R7 = clamp(R1=150, lower=100, upper=200) = 150
    vm.write_reg_256(10, 100);
    vm.execute_slot("_MX07#010A00C80>"); // 0x00C8 = 200
    assert_eq!(vm.read_reg_256(7), 150, "Clamped value within bounds should be 150");
}

#[test]
fn test_vm_3address_swizzle_and_rotate_sw() {
    let mut vm = CoreEngine::new(0);

    vm.write_reg_256(1, 0x1234_5678);
    vm.write_reg_256(2, 8); // shift by 8 bits

    // Rotate Right by 8: 0x12345678 ror 8 = 0x78123456
    vm.execute_slot("_SW03R010200000>");
    assert_eq!(vm.read_reg_256(3), 0x7812_3456, "Rotate right by 8 should match");

    // Rotate Left by 8: 0x12345678 rol 8 = 0x34567812
    vm.execute_slot("_SW04L010200000>");
    assert_eq!(vm.read_reg_256(4), 0x3456_7812, "Rotate left by 8 should match");

    // Byte Swap: 0x12345678 swap = 0x78563412
    vm.execute_slot("_SW05B010000000>");
    assert_eq!(vm.read_reg_256(5), 0x7856_3412, "Byte swap should match");
}

#[test]
fn test_vm_in_slot_fp16_arithmetic_fp() {
    let mut vm = CoreEngine::new(0);

    // 1.0 in FP16 bits = 0x3C00
    // 2.5 in FP16 bits = 0x4100
    // 1.0 + 2.5 = 3.5 in FP16 bits = 0x4300
    vm.write_reg_256(1, 0x3C00);
    vm.write_reg_256(2, 0x4100);

    // R3 = R1 + R2 in FP16
    vm.execute_slot("_FP03+010200000>");
    assert_eq!(vm.read_reg_256(3), 0x4300, "1.0 + 2.5 in FP16 should be 3.5 (0x4300)");

    // In-Slot immediate FP16 multiplication:
    // R4 = R1 * Imm(2.0 = 0x4000) -> 1.0 * 2.0 = 2.0 (0x4000)
    vm.execute_slot("_FP04*010040000>");
    assert_eq!(vm.read_reg_256(4), 0x4000, "1.0 * 2.0 in FP16 should be 2.0 (0x4000)");
}
