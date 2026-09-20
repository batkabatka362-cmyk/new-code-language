use cronc::{
    generate_vcd_trace, render_ascii_waveform,
    VcdConfig,
};

#[test]
fn test_vcd_header_and_signal_declarations() {
    let cl_code = r#"
    B0000: '==01#00A> '==02#014> '==03#000> '==04#000>
    B0001: _OP01$29F> _MD02*322> _PO04+650> _SB00#060>
    B0002: _bb00#000> _NO00#070> _NO00#070> _HL00$0E8!
    "#;

    let config = VcdConfig::default();
    let report = generate_vcd_trace(cl_code, &config).expect("VCD trace generation must succeed");

    assert!(report.vcd_content.contains("$date"));
    assert!(report.vcd_content.contains("$version"));
    assert!(report.vcd_content.contains("CRON SSS+ Silicon VCD Dumper"));
    assert!(report.vcd_content.contains("$timescale\n   1ns\n$end"));
    assert!(report.vcd_content.contains("$scope module cksl_core $end"));
    assert!(report.vcd_content.contains("$var wire 1"));
    assert!(report.vcd_content.contains("$var wire 32"));
    assert!(report.vcd_content.contains("$upscope $end"));
    assert!(report.vcd_content.contains("$enddefinitions $end"));
    assert!(report.vcd_content.contains("$dumpvars"));

    assert_eq!(report.total_cycles, 3);
    assert!(report.execution_halted);
    assert!(report.total_signals >= 25);
    assert!(report.value_changes_dumped > 0);
}

#[test]
fn test_vcd_value_change_deduplication() {
    let cl_code = r#"
    B0000: '==01#005> '==02#005> '==03#000> '==04#000>
    B0001: _NO00#070> _NO00#070> _NO00#070> _NO00#070>
    B0002: _NO00#070> _NO00#070> _NO00#070> _HL00$0E8!
    "#;

    let config = VcdConfig {
        max_cycles: 10,
        timescale_ns: 1,
        clock_period_ns: 10,
        include_registers: true,
        include_memory: true,
        include_noc: true,
        include_neuromorphic: true,
    };

    let report = generate_vcd_trace(cl_code, &config).expect("Trace generation must succeed");

    // In cycle 1 (B0001), registers do not change because all slots are NOPs.
    // The number of dumped value changes must be significantly lower than dumping every signal every cycle.
    let max_possible_changes = report.total_signals * report.total_cycles * 2;
    assert!(
        report.value_changes_dumped < max_possible_changes,
        "VCD must deduplicate unchanged signals: dumped {}, max possible {}",
        report.value_changes_dumped,
        max_possible_changes
    );

    // Check timestamps exist
    assert!(report.vcd_content.contains("#0"));
    assert!(report.vcd_content.contains("#5"));
    assert!(report.vcd_content.contains("#10"));
    assert!(report.vcd_content.contains("#15"));
}

#[test]
fn test_ascii_waveform_diagram() {
    let cl_code = r#"
    B0000: '==01#00A> '==02#014> '==03#000> '==04#000>
    B0001: _OP01$29F> _MD02*322> _PO04+650> _SB00#060>
    B0002: _bb00#000> _NO00#070> _NO00#070> _HL00$0E8!
    "#;

    let config = VcdConfig::default();
    let report = generate_vcd_trace(cl_code, &config).expect("Trace generation must succeed");

    let ascii = render_ascii_waveform(&report.snapshots, 8);

    assert!(ascii.contains("CRON 256-CORE 4D-TORUS CYCLE-ACCURATE PIPELINE TIMING DIAGRAM"));
    assert!(ascii.contains("Cycle    │"));
    assert!(ascii.contains("clk (HI) │"));
    assert!(ascii.contains("    (LO) │"));
    assert!(ascii.contains("PC       │"));
    assert!(ascii.contains("ALU0     │"));
    assert!(ascii.contains("ALU1     │"));
    assert!(ascii.contains("MEM      │"));
    assert!(ascii.contains("NOC      │"));
    assert!(ascii.contains("SRAM.Bnk │"));
    assert!(ascii.contains("R0       │"));
    assert!(ascii.contains("R1       │"));
    assert!(ascii.contains("NoC.DOR  │"));
    assert!(ascii.contains("Halted   │"));
}

#[test]
fn test_vcd_json_telemetry() {
    let cl_code = r#"
    B0000: '==01#00A> '==02#014> '==03#000> '==04#000>
    B0001: _bb00#000> _NO00#070> _NO00#070> _HL00$0E8!
    "#;

    let config = VcdConfig::default();
    let report = generate_vcd_trace(cl_code, &config).expect("Trace generation must succeed");

    let json = report.to_json();
    assert!(json.contains("\"total_cycles\": 2"));
    assert!(json.contains("\"total_signals\":"));
    assert!(json.contains("\"value_changes_dumped\":"));
    assert!(json.contains("\"execution_halted\": true"));
    assert!(json.contains("\"timeline\":"));
    assert!(json.contains("\"cycle\": 0"));
    assert!(json.contains("\"cycle\": 1"));
}
