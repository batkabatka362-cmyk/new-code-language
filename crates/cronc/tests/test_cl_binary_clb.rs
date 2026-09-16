use cronc::cl_binary::{
    assemble_cl_to_clb, decode_u32_to_slot, disassemble_clb_to_cl, encode_slot_to_u32, CLB_MAGIC,
    CLB_VERSION,
};
use cronc::cl_lang::{parse_slot, verify_cl_program};
use cron_vm::Simulator;

#[test]
fn test_encode_decode_slot_preserves_semantics() {
    let test_slots = vec![
        "_PO06G400>",
        "_PO0A$000>",
        "_POA0$000>",
        "_POAA$000>",
        "_bb00$000>",
        "_OP041$0E>",
        "_FA054$20>",
        "_SP00#0088>",
        "_SH00$01B4>",
        "'=00#0A04>",
        "'=06#0064>",
        "'=04#000A>",
    ];

    for raw in test_slots {
        let encoded = encode_slot_to_u32(raw);
        let decoded = decode_u32_to_slot(encoded);

        assert_eq!(
            decoded.len(),
            10,
            "Decoded slot '{}' must be exactly 10 characters",
            decoded
        );

        // Verify decoded slot is valid per cl_lang specification
        let parsed = parse_slot(&decoded);
        assert!(
            parsed.is_ok(),
            "Decoded slot '{}' failed validation: {:?}",
            decoded,
            parsed.err()
        );
    }
}

#[test]
fn test_round_trip_assemble_and_disassemble() {
    let original_cl = r#"
    B0001: '=00#0A04> '=01#0005> _SH00$01B4> _SP00#0088>
    B0002: _PO06+400> _POA0$000> _POAA$000> _bb00$000>
    B0003: _OP041$0E> _FA054$20> _HLT_____8! _NO00$000>
    "#;

    // Assemble textual .cl into 128-bit binary bytecode (.clb)
    let binary = assemble_cl_to_clb(original_cl).expect("Assembly must succeed");

    // Header validation: 12 bytes header
    assert!(binary.len() >= 12);
    assert_eq!(&binary[0..4], &CLB_MAGIC, "Must have CLB1 magic header");
    let version = u16::from_be_bytes([binary[4], binary[5]]);
    assert_eq!(version, CLB_VERSION);

    // 3 bundles x 20 bytes + 12 header = 72 bytes total
    let expected_len = 12 + 3 * 20;
    assert_eq!(
        binary.len(),
        expected_len,
        "3 bundles must produce exactly 72 bytes (128-bit payload per bundle)"
    );

    // Disassemble back to .cl
    let disassembled = disassemble_clb_to_cl(&binary).expect("Disassembly must succeed");

    // Verify disassembled code passes syntax and parity validation
    let report = verify_cl_program(&disassembled);
    assert!(
        report.is_ok(),
        "Disassembled program must pass full validation: {:?}",
        report.err()
    );
    let r = report.unwrap();
    assert_eq!(r.total_bundles, 3);
    assert_eq!(r.total_slots, 12);
}

#[test]
fn test_clb_binary_simulation_equivalence() {
    let cl_code = r#"
    B0001: '=00#0A04> '=06#0064> '=04#0032> _SH00$01B4>
    B0002: _PO06G400> _POA0$000> _bb00$000> _NO00$000>
    B0003: '=0A#0005> _POAA$000> _HLT_____8! _NO00$000>
    "#;

    // 1. Run directly from text
    let mut sim_text = Simulator::new();
    sim_text.load_machine_code(cl_code);
    sim_text.run();

    // 2. Assemble to .clb binary, then disassemble and simulate
    let binary = assemble_cl_to_clb(cl_code).expect("Assembly to CLB failed");
    let disasm = disassemble_clb_to_cl(&binary).expect("Disassembly from CLB failed");

    let mut sim_bin = Simulator::new();
    sim_bin.load_machine_code(&disasm);
    sim_bin.run();

    // Verify cycle counts and core 0 state match exactly
    assert_eq!(sim_text.stats.total_cycles, sim_bin.stats.total_cycles);
    let dump_text = sim_text.core_dump(0);
    let dump_bin = sim_bin.core_dump(0);
    assert_eq!(dump_text[6], dump_bin[6], "R6 comparison result must match");
    assert_eq!(dump_text[10], dump_bin[10], "R10 AA accumulation must match");
    assert_eq!(
        sim_text.get_bank_register(0, 10, 0),
        sim_bin.get_bank_register(0, 10, 0),
        "Bank 10 Reg 0 state must match"
    );
}
