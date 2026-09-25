use cronc::cl_macro::{parse_clm, MacroCompiler, build_valid_slot};
use cronc::cl_importer::NeuralLayer;
use cronc::cl_lsp::{analyze_cl_source, DiagnosticSeverity};
use cronc::cl_visualizer::export_html5_dashboard;
use cronc::cl_clifford::Multivector4D;

#[test]
fn test_cl_macro_compilation() {
    let clm_src = r#"
        let R0 = 0x100
        let R1 = 0x200
        R2 = R0 + R1
        op_gemm R3, R0, R1
        ternary_mac R4, R0, R1
        stdp_learn R5, R0, R1
        pack_trits R6, R0
        send_noc 0x01, R2
        recv_noc R7
        barrier
    "#;

    let stmts = parse_clm(clm_src).expect("Failed to parse CLM source");
    assert_eq!(stmts.len(), 10);

    let mut compiler = MacroCompiler::new(0);
    compiler.compile_stmts(&stmts);
    let cl_code = compiler.finish();

    assert!(cl_code.contains("B0000:"));
    assert!(cl_code.contains("B0001:"));
    assert!(cl_code.contains("B0002:"));

    // Verify all emitted slots are strictly 10 characters
    for line in cl_code.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(parts.len(), 5); // Label + 4 slots
        for slot in &parts[1..] {
            assert_eq!(slot.len(), 10, "Slot {} length is not 10", slot);
        }
    }
}

#[test]
fn test_cl_importer_ternary_quantization() {
    let layer = NeuralLayer::new_random("test_linear", 32, 16, 12345);
    let trits = layer.quantize_ternary(0.25);
    assert_eq!(trits.len(), 32 * 16);

    let packed = NeuralLayer::pack_16_trits(&trits[..16]);
    assert!(packed > 0);

    let cl_code = layer.compile_to_cl(0, 16);
    assert!(cl_code.contains("B0000:"));
    assert!(cl_code.contains("_MD")); // Contains ternary MAC
    assert!(cl_code.contains("_TX")); // Contains NoC transmission
}

#[test]
fn test_cl_lsp_diagnostics() {
    // 1. Valid program
    let valid_slot = build_valid_slot("==00#042", "");
    let valid_nop = build_valid_slot("__NOP0000", "");
    let valid_prog = format!("B0000: {} {} {} {}\n", valid_slot, valid_nop, valid_nop, valid_nop);

    let diags = analyze_cl_source(&valid_prog);
    assert!(diags.is_empty(), "Expected 0 diagnostics for valid code, got {:?}", diags);

    // 2. Invalid CRC token
    let bad_crc_slot = "==00#0420X";
    let bad_prog = format!("B0000: {} {} {} {}\n", bad_crc_slot, valid_nop, valid_nop, valid_nop);
    let bad_diags = analyze_cl_source(&bad_prog);
    assert_eq!(bad_diags.len(), 1);
    assert_eq!(bad_diags[0].code, "CRC001");

    // 3. WAW Hazard
    let waw_slot1 = build_valid_slot("==05#001", "");
    let waw_slot2 = build_valid_slot("==05#002", "");
    let waw_prog = format!("B0000: {} {} {} {}\n", waw_slot1, waw_slot2, valid_nop, valid_nop);
    let waw_diags = analyze_cl_source(&waw_prog);
    assert_eq!(waw_diags.len(), 1);
    assert_eq!(waw_diags[0].code, "HAZ001");
    assert_eq!(waw_diags[0].severity, DiagnosticSeverity::Warning);
}

#[test]
fn test_cl_visualizer_html_export() {
    let mock_regs = [[0x00FFu32; 16]; 256];
    let html = export_html5_dashboard(&mock_regs, 5000, 1200);

    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("SAGI AGI Neocortical Foundation Engine"));
    assert!(html.contains("256-Core 4D-Torus"));
    assert!(html.contains("CORES"));
}

#[test]
fn test_cl_clifford_4d_rotors() {
    let mut mv = Multivector4D::zero();
    mv.s = 1.0;
    mv.v[0] = 3.0;
    mv.b[0] = 0.5;

    let rotor = Multivector4D::new_rotor_4d(std::f32::consts::PI / 4.0, 0);
    assert!((rotor.s - 0.9238795).abs() < 1e-4);

    let cl_code = mv.compile_rotor_transform_to_cl(&rotor, 0);
    assert!(cl_code.contains("B0000:"));
    assert!(cl_code.contains("_M")); // FMA mode rotor product
}
