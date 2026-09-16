use cronc::cl_lang::{
    audit_alphabet_coverage, compute_crc8_atm, get_alphabet_94, pack_slot_with_crc8, parse_slot,
    verify_cl_program, verify_token_crc8, ALPHABET_94_STR,
};

#[test]
fn test_alphabet_94_definition_and_completeness() {
    let alphabet = get_alphabet_94();
    assert_eq!(alphabet.len(), 94, "Must contain exactly 94 characters");
    assert_eq!(alphabet[0], '!', "First character must be '!' (ASCII 33)");
    assert_eq!(alphabet[93], '~', "Last character must be '~' (ASCII 126)");

    assert_eq!(ALPHABET_94_STR.len(), 94);
    assert_eq!(
        ALPHABET_94_STR,
        "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
    );

    // Verify each character is distinct
    let mut unique = std::collections::HashSet::new();
    for &c in &alphabet {
        assert!(unique.insert(c), "Duplicate character found: '{}'", c);
    }
}

#[test]
fn test_pure_machine_agent_100_percent_alphabet_saturation() {
    let cl_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples")
        .join("pure_machine_agent.cl");

    let cl_content = std::fs::read_to_string(&cl_path)
        .expect("Failed to read examples/pure_machine_agent.cl");

    let audit = audit_alphabet_coverage(&cl_content);
    assert_eq!(audit.unique_characters_used, 94, "Must utilize all 94 characters!");
    assert_eq!(audit.coverage_percentage, 100.0, "Coverage must be 100.0%");
    assert!(audit.missing_characters.is_empty(), "Missing characters must be empty: {:?}", audit.missing_characters);

    // Verify Shannon information entropy is significant
    assert!(
        audit.entropy_bits_per_char > 4.5,
        "Entropy must exceed 4.5 bits/char (got {})",
        audit.entropy_bits_per_char
    );

    // Verify VLIW structural validity with 0 hazards
    let report = verify_cl_program(&cl_content).expect("Verification must pass");
    assert_eq!(report.total_bundles, 12);
    assert_eq!(report.total_slots, 48);
    assert_eq!(report.opcodes_verified, 48);
    assert_eq!(report.hazards.len(), 0, "Hazard count must be 0: {:?}", report.hazards);
}

#[test]
fn test_slot_prefixes_and_terminators() {
    // Standard synchronous
    let slot_std = parse_slot("_PO01$200>").unwrap();
    assert_eq!(slot_std.prefix, '_');
    assert_eq!(slot_std.terminator, '>');

    // Speculative
    let slot_spec = parse_slot("'PO01$200?").unwrap();
    assert_eq!(slot_spec.prefix, '\'');
    assert_eq!(slot_spec.terminator, '?');

    // Stochastic / Approximate
    let slot_stoch = parse_slot("~PO01$200;").unwrap();
    assert_eq!(slot_stoch.prefix, '~');
    assert_eq!(slot_stoch.terminator, ';');

    // Spatial Memory Pointer
    let slot_mem = parse_slot("@PO01$200!").unwrap();
    assert_eq!(slot_mem.prefix, '@');
    assert_eq!(slot_mem.terminator, '!');
}

#[test]
fn test_crc8_atm_token_integrity() {
    let crc = compute_crc8_atm(b"_OP04$1E");
    assert_eq!(crc, compute_crc8_atm(b"_OP04$1E"));

    let slot = pack_slot_with_crc8('_', "OP", "04", '$', '1', 'E', '>');
    assert_eq!(slot.len(), 10);
    assert!(verify_token_crc8(&slot), "CRC-8 ATM check must pass for packed slot");

    // Corrupt one character and ensure CRC-8 detects it
    let mut corrupted: Vec<char> = slot.chars().collect();
    corrupted[4] = if corrupted[4] == '4' { '5' } else { '4' };
    let corrupted_slot: String = corrupted.into_iter().collect();
    assert!(
        !verify_token_crc8(&corrupted_slot),
        "Corrupted slot must fail CRC-8 verification"
    );
}

#[test]
fn test_all_coprocessor_opcodes_in_cl() {
    let test_program = r#"
B0001: _HE01$000> _LI02$005> _OD03$000> _CA04$000>
B0002: _AW05$000> _CD06$000> _CS07$004> _PS08$000>
B0003: _TT09$000> _WH0A$000> _TX00$000> _RX0B$000>
B0004: _LF0C$000> _IR0D$000> _DF00$000> _HL00#000!
"#;
    let report = verify_cl_program(test_program).expect("Verification must pass");
    assert_eq!(report.total_bundles, 4);
    assert_eq!(report.total_slots, 16);
    assert_eq!(report.hazards.len(), 0, "Hazard count must be 0: {:?}", report.hazards);
}
