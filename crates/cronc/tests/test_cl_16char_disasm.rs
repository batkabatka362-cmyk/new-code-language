// ============================================================================
// Test Suite: 16-Character Disassembler & 3-Address VLIW Verification
// Verifies:
//   - Disassembly of 16-character slots to human-readable 3-address assembly
//   - Disassembly of FM (FMA), MX (Max/ReLU), SW (Swizzle/Rotate), FP (FP16)
//   - Verification of 256-register indices (R[0x01]..R[0xFF])
// ============================================================================

use cron_vm::debugger::disassemble_slot;

#[test]
fn test_16char_disassembler_po() {
    let slot = "_PO03+010200000>";
    let dis = disassemble_slot(slot);

    assert_eq!(dis.opcode, "PO");
    assert_eq!(dis.dest_reg, 3);
    assert_eq!(dis.src_reg, 1);
    assert_eq!(dis.mnemonic, "ADD R[0x03], R[0x01], R[0x02]");
}

#[test]
fn test_16char_disassembler_fma() {
    let slot = "_FM05+0102000A0>";
    let dis = disassemble_slot(slot);

    assert_eq!(dis.opcode, "FM");
    assert_eq!(dis.dest_reg, 5);
    assert_eq!(dis.src_reg, 1);
    assert!(dis.mnemonic.contains("FMA R[0x05], R[0x01], R[0x02], #0x000A"));
}

#[test]
fn test_16char_disassembler_mx() {
    let slot = "_MX0A>010200000>";
    let dis = disassemble_slot(slot);

    assert_eq!(dis.opcode, "MX");
    assert_eq!(dis.dest_reg, 0x0A);
    assert!(dis.mnemonic.contains("MAX R[0x0A], R[0x01], R[0x02]"));
}

#[test]
fn test_16char_disassembler_fp16() {
    let slot = "_FP04*010040000>";
    let dis = disassemble_slot(slot);

    assert_eq!(dis.opcode, "FP");
    assert_eq!(dis.dest_reg, 4);
    assert!(dis.mnemonic.contains("FMUL.16 R[0x04], R[0x01], #0x4000"));
}

#[test]
fn test_16char_disassembler_load_imm() {
    let slot = "'=02A#0000007B0>";
    let dis = disassemble_slot(slot);

    assert_eq!(dis.opcode, "LOAD_IMM");
    assert_eq!(dis.dest_reg, 0x2A);
    assert_eq!(dis.imm, 123);
    assert!(dis.mnemonic.contains("LI R[0x2A], #0x007B (123)"));
}
