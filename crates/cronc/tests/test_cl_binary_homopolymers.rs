use cronc::cl_binary::{assemble_cl_to_clb, disassemble_clb_to_cl};
use cronc::cl_lang::verify_cl_program;
use cron_vm::Simulator;

#[test]
fn test_round_trip_homopolymer_binary_and_simulation() {
    let cl_code = r#"
    B0001: '=00#0A04> '=0A#0005> _1100$000> _SH00$000>
    B0002: _CC00$000> _POAA$000> _DDA0$0A0> _NO00$000>
    B0003: _EE00$000> _8800$000> _bb00$000> _HL00$000!
    "#;

    // 1. Verify text validity
    let report_orig = verify_cl_program(cl_code).expect("Original text must be valid");
    assert_eq!(report_orig.total_bundles, 3);

    // 2. Assemble to 128-bit binary bytecode
    let clb_bytes = assemble_cl_to_clb(cl_code).expect("Assembly must succeed");
    assert_eq!(clb_bytes.len(), 12 + 3 * 20, "Must be exactly 72 bytes");

    // 3. Disassemble back to .cl text
    let disasm = disassemble_clb_to_cl(&clb_bytes).expect("Disassembly must succeed");
    let report_disasm = verify_cl_program(&disasm).expect("Disassembled code must be valid");
    assert_eq!(report_disasm.total_bundles, 3);
    assert_eq!(report_disasm.total_slots, 12);

    // 4. Simulate disassembled binary
    let mut sim = Simulator::new();
    sim.load_machine_code(&disasm);
    sim.run();

    // Verify all homopolymer states were triggered on Core 0
    assert!(sim.get_core_photonic_pumps(0) >= 1, "Photonic pump 11 must fire");
    assert!(sim.get_core_cache_invalidations(0) >= 1, "Cache invalidation CC must fire");
    assert!(sim.get_core_dma_transfers(0) >= 1, "Direct DMA DD must fire");
    assert_eq!(sim.get_core_dvfs_state(0), 1, "DVFS state EE must be Eco");
    assert!(sim.get_core_arena_resets(0) >= 1, "Arena reset 88 must fire");
    assert_eq!(sim.core_dump(0)[10], 25, "R10 AA auto-accumulation must equal 5*5=25");
}
