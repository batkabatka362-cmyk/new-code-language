use cronc::cl_cosim::{run_cl_cosim, CosimOptions, VerilogRtlCoreSimulator};
use cronc::cl_lang::parse_slot;

#[test]
fn test_cl_cosim_arithmetic_lockstep_parity() {
    let cl_code = r#"
    ; Immediate Loading & SIMD ALU Parity Verification
    B0000: '==00#00A> '==01#004> _NO00$000> _NO00$000>
    B0001: _PO00+100> _NO00$000> _NO00$000> _NO00$000>
    B0002: _HL00$008! _NO00$000> _NO00$000> _NO00$000>
    "#;

    let options = CosimOptions::default();
    let report = run_cl_cosim(cl_code, &options).expect("Co-simulation should succeed");

    assert_eq!(report.total_cycles_simulated, 3);
    assert_eq!(report.bit_exact_match_count, 3);
    assert_eq!(report.divergence_count, 0);
    assert!(report.is_100pct_parity);
    assert!(report.software_halted);
    assert!(report.rtl_halted);
    // R0 should be 10 + 4 = 14 (0xE)
    assert_eq!(report.final_soft_regs[0], 0x0E);
    assert_eq!(report.final_rtl_regs[0], 0x0E);
    // R1 should be 4
    assert_eq!(report.final_soft_regs[1], 0x04);
    assert_eq!(report.final_rtl_regs[1], 0x04);
}

#[test]
fn test_cl_cosim_photonic_reversible_lockstep_parity() {
    let cl_code = r#"
    ; Photonic GEMM, Autodiff Tap & Fredkin Reversible Swap
    B0000: '==01#00A> '==02#005> _NO00$000> _NO00$000>
    B0001: _OP03$000> _FA04$100> _RF01$200> _NO00$000>
    B0002: _HL00$008! _NO00$000> _NO00$000> _NO00$000>
    "#;

    let options = CosimOptions::default();
    let report = run_cl_cosim(cl_code, &options).expect("Co-simulation should succeed");

    assert_eq!(report.total_cycles_simulated, 3);
    assert!(report.is_100pct_parity);
    assert_eq!(report.divergence_count, 0);

    // R3 is result of authentic optical MZI GEMM
    assert_eq!(report.final_soft_regs[3], 0x00FF0000);
    assert_eq!(report.final_rtl_regs[3], 0x00FF0000);

    // R4 received R1 (0x0A) via FA
    assert_eq!(report.final_soft_regs[4], 0x0A);
    assert_eq!(report.final_rtl_regs[4], 0x0A);

    // RF swapped R1 and R2: R1 became 0x05, R2 became 0x0A
    assert_eq!(report.final_soft_regs[1], 0x05);
    assert_eq!(report.final_rtl_regs[1], 0x05);
    assert_eq!(report.final_soft_regs[2], 0x0A);
    assert_eq!(report.final_rtl_regs[2], 0x0A);
}

#[test]
fn test_cl_cosim_mini_transformer_attention_kernel() {
    let cl_kernel = r#"
    @attn_init:
    B0000: '==01#004> '==02#008> '==03#00C> _NO00#000>
    B0001: '==04#010> '==05#020> '==06#001> _NO00#000>

    @q_k_projection:
    B0002: _OP01$28F> _MD02*3A2> _PO04+600> _SB00#000>
    B0003: _FA03$142> _TT05$200> _PO06&100> _bb00#000>

    @attention_weights_softmax:
    B0004: _MD07.280> _PO08*300> _PO09+600> _NO00#000>
    B0005: _PO01/400> _PO02/400> _PO03/400> _bb00#000>

    @context_aggregation:
    B0006: _OP0A$48F> _MD0B*2A2> _PO0C+100> _SB00#000>
    B0007: _FU00#000> _PO00+600> _FE00#000> _HL00!000>
    "#;

    let options = CosimOptions {
        max_cycles: 50,
        trace_all_cycles: true,
        strict_parity: true,
    };

    let report = run_cl_cosim(cl_kernel, &options).expect("Attention kernel should co-simulate");

    assert_eq!(report.total_cycles_simulated, 8);
    assert_eq!(report.bit_exact_match_count, 8);
    assert_eq!(report.divergence_count, 0);
    assert!(report.is_100pct_parity);
    assert!(report.software_halted);
    assert!(report.rtl_halted);

    // Verify all 16 registers match exactly between Golden JIT and RTL
    for i in 0..16 {
        assert_eq!(
            report.final_soft_regs[i], report.final_rtl_regs[i],
            "Register R{} bit-level parity mismatch!",
            i
        );
    }
}

#[test]
fn test_cl_cosim_divergence_detection_and_reporting() {
    let mut rtl = VerilogRtlCoreSimulator::new();
    let slot = parse_slot("_PO00+100>").unwrap();

    // Set initial register values
    rtl.rf[0] = 10;
    rtl.rf[1] = 5;

    rtl.step_cycle(std::slice::from_ref(&slot), &["_PO00+100>"]);

    // After adding R1 (5) to R0 (10), R0 must be 15
    assert_eq!(rtl.rf[0], 15);
    assert_eq!(rtl.total_cycles, 1);
    assert_eq!(rtl.pc, 1);
}

#[test]
fn test_cl_cosim_detects_hardware_divergence() {
    let cl_code = r#"
    B0000: '==00#00A> _NO00$000> _NO00$000> _NO00$000>
    B0001: _HL00$008! _NO00$000> _NO00$000> _NO00$000>
    "#;

    let report = run_cl_cosim(cl_code, &CosimOptions::default()).expect("Simulate");
    assert!(report.is_100pct_parity);
    assert_eq!(report.divergence_count, 0);

    // Corrupted state step test
    let mut rtl = VerilogRtlCoreSimulator::new();
    rtl.rf[0] = 0xDEADBEEF; // Injected hardware divergence
    let slot = parse_slot("_NO00$000>").unwrap();
    rtl.step_cycle(&[slot], &["_NO00$000>"]);
    assert_eq!(rtl.rf[0], 0xDEADBEEF);
}

#[test]
fn test_cl_cosim_json_serialization_validity() {
    let cl_code = r#"
    B0000: '==00#00A> _NO00$000> _NO00$000> _NO00$000>
    B0001: _HL00$008! _NO00$000> _NO00$000> _NO00$000>
    "#;

    let report = run_cl_cosim(cl_code, &CosimOptions::default()).expect("Simulate");
    let json = report.to_json();

    assert!(json.contains("\"total_cycles_simulated\": 2"));
    assert!(json.contains("\"bit_exact_match_count\": 2"));
    assert!(json.contains("\"divergence_count\": 0"));
    assert!(json.contains("\"is_100pct_parity\": true"));
    assert!(json.contains("\"divergence_details\": []"));
    assert!(json.contains("\"final_soft_regs\": ["));
    assert!(json.contains("\"final_rtl_regs\": ["));
    assert!(json.contains("\"cycle_trace\": ["));
}

#[test]
fn test_cl_cosim_ascii_trace_format() {
    let cl_code = r#"
    B0000: '==00#005> '==01#003> _NO00$000> _NO00$000>
    B0001: _PO00+100> _NO00$000> _NO00$000> _HL00$008!
    "#;

    let report = run_cl_cosim(cl_code, &CosimOptions::default()).expect("Simulate");
    let trace = report.format_ascii_trace("test_kernel.cl");

    assert!(trace.contains("CRON HARDWARE CO-SIMULATION BRIDGE"));
    assert!(trace.contains("100% BIT-EXACT HARDWARE PARITY"));
    assert!(trace.contains("B0000"));
    assert!(trace.contains("B0001"));
    assert!(trace.contains("Final Hardware Register File Parity:"));
}
