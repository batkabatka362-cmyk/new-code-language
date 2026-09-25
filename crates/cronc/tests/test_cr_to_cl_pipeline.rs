use cronc::compile_source;

#[test]
fn test_cr_high_level_to_cl_vliw_pipeline() {
    let cr_source = r#"
    .MODULE SagiHighLevelPipeline
    .ENTRY _main
    _main:
        let a: i32 = 10
        let b: i32 = 20
        let trits = pack_trits(a)
        let mac_res = ternary_mac(a, b)
        let plastic_res = stdp_update(a, b)
        send_noc(1, mac_res)
        noc_barrier()
    .END
    "#;

    let cl_result = compile_source(cr_source);
    assert!(cl_result.is_ok(), "Failed to compile .cr source: {:?}", cl_result.err());

    let cl_code = cl_result.unwrap();
    println!("Compiled .CL:\n{}", cl_code);

    assert!(cl_code.contains("B0001:"));
    assert!(cl_code.contains("_PK")); // Packed trits opcode
    assert!(cl_code.contains("_MD")); // Ternary MAC opcode
    assert!(cl_code.contains("_ST")); // STDP update opcode
    assert!(cl_code.contains("_TX")); // NoC send opcode
    assert!(cl_code.contains("_SY")); // NoC barrier opcode
}

#[test]
fn test_cr_clifford_and_compound_modes() {
    let cr_source = r#"
    .MODULE CliffordCompoundPipeline
    .ENTRY _main
    _main:
        let x: i32 = 42
        let y: i32 = 100
        let fma_val = fma(x, y)
        let pop_val = popcount(x)
        let sat_val = saturating_add(x, y)
        let rotor_val = clifford_rotor_sandwich(x, y)
    .END
    "#;

    let cl_result = compile_source(cr_source);
    assert!(cl_result.is_ok(), "Failed to compile .cr clifford/compound: {:?}", cl_result.err());

    let cl_code = cl_result.unwrap();
    println!("Compiled .CL Compound:\n{}", cl_code);

    assert!(cl_code.contains("B0001:"));
    assert!(cl_code.contains("_MM")); // FMA / FMS
    assert!(cl_code.contains("_PO")); // Popcount / Saturating add
}
