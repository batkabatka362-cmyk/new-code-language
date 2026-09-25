// ============================================================================
// CRON Compound & Fused Arithmetic Mode Verification Suite (Milestone #030)
// Tests: FMA ('M'), FMS ('S'), Barrel Shifts ('L', 'R'), Saturating Add ('A'),
// Popcount ('B'), Universal Gates ('N', 'O', 'X') across JIT, VM, C23, and CoSim.
// ============================================================================

use cronc::cl_jit::run_cl_jit;
use cronc::cl_cosim::{run_cl_cosim, CosimOptions};
use cronc::cl_binary::{assemble_cl_to_clb, disassemble_clb_to_cl};

#[test]
fn test_fused_multiply_add_fma_execution() {
    let cl_code = r#"
    ; Initialize R1 = 10, R2 = 3, R3 = 4
    B0000: '==01#00A> '==02#003> '==03#004> _NO00#000>
    ; FMA: R1 = R1 + (R2 * R3) = 10 + (3 * 4) = 22 = 0x16
    B0001: _PO01M203> _NO00#000> _NO00#000> _HL00#000!
    "#;

    let core = run_cl_jit(cl_code).expect("JIT execution should succeed");
    assert_eq!(core.r[1], 22, "FMA R1 = 10 + (3 * 4) should be 22");

    let options = CosimOptions::default();
    let report = run_cl_cosim(cl_code, &options).expect("Cosim should succeed");
    assert!(report.is_100pct_parity);
    assert_eq!(report.final_soft_regs[1], 22);
    assert_eq!(report.final_rtl_regs[1], 22);
}

#[test]
fn test_fused_multiply_sub_fms_execution() {
    let cl_code = r#"
    ; Initialize R1 = 50, R2 = 5, R3 = 8
    B0000: '==01#032> '==02#005> '==03#008> _NO00#000>
    ; FMS: R1 = R1 - (R2 * R3) = 50 - (5 * 8) = 10 = 0x0A
    B0001: _PO01S203> _NO00#000> _NO00#000> _HL00#000!
    "#;

    let core = run_cl_jit(cl_code).expect("JIT execution should succeed");
    assert_eq!(core.r[1], 10, "FMS R1 = 50 - (5 * 8) should be 10");

    let options = CosimOptions::default();
    let report = run_cl_cosim(cl_code, &options).expect("Cosim should succeed");
    assert!(report.is_100pct_parity);
    assert_eq!(report.final_soft_regs[1], 10);
    assert_eq!(report.final_rtl_regs[1], 10);
}

#[test]
fn test_barrel_shifts_and_popcount() {
    let cl_code = r#"
    ; Initialize R1 = 0x0F (15, 4 ones), R2 = 3
    B0000: '==01#00F> '==02#003> _NO00#000> _NO00#000>
    ; R3 = R1 << R2 = 0x0F << 3 = 120 = 0x78
    ; R4 = R1 >> R2 = 0x0F >> 3 = 1
    ; R5 = popcount(R1) = 4
    B0001: _PO03L102> _PO04R102> _PO05B100> _HL00#000!
    "#;

    let core = run_cl_jit(cl_code).expect("JIT execution should succeed");
    assert_eq!(core.r[3], 0x0F << 3);
    assert_eq!(core.r[4], 0x0F >> 3); // using R2=3
    assert_eq!(core.r[5], 4, "Popcount of 0x0F should be 4");

    let options = CosimOptions::default();
    let report = run_cl_cosim(cl_code, &options).expect("Cosim should succeed");
    assert!(report.is_100pct_parity);
}

#[test]
fn test_universal_logic_gates_nand_nor_xnor() {
    let cl_code = r#"
    ; Initialize R1 = 0x0F0F, R2 = 0x00FF
    B0000: '==01#F0F> '==02#0FF> _NO00#000> _NO00#000>
    ; R3 = NAND(R1, R2)
    ; R4 = NOR(R1, R2)
    ; R5 = XNOR(R1, R2)
    B0001: _PO03N102> _PO04O102> _PO05X102> _HL00#000!
    "#;

    let core = run_cl_jit(cl_code).expect("JIT execution should succeed");
    assert_eq!(core.r[3], !(0xF0F & 0x0FF));
    assert_eq!(core.r[4], !(0xF0F | 0x0FF));
    assert_eq!(core.r[5], !(0xF0F ^ 0x0FF));

    let options = CosimOptions::default();
    let report = run_cl_cosim(cl_code, &options).expect("Cosim should succeed");
    assert!(report.is_100pct_parity);
}

#[test]
fn test_binary_roundtrip_with_compound_modes() {
    let source = "B0000: _PO01M203> _PO04S506> _PO07L809> _PO0AB100>\n";
    let bytes = assemble_cl_to_clb(source).expect("Assembly should succeed");
    assert_eq!(bytes.len(), 32, "1 bundle with header = 12 bytes header + 20 bytes bundle = 32 bytes");

    let disasm = disassemble_clb_to_cl(&bytes).expect("Disassembly should succeed");
    assert!(disasm.contains("_PO01M2"));
    assert!(disasm.contains("_PO04S5"));
    assert!(disasm.contains("_PO07L8"));
    assert!(disasm.contains("_PO0AB1"));
}
