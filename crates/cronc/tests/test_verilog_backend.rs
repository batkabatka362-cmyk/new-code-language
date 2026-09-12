use cronc::verilog_backend::{encode_slot_to_u32, generate_testbench, generate_verilog_hdl};
use cronc::cl_lang::parse_slot;

#[test]
fn test_verilog_slot_encoding() {
    let slot = parse_slot("_OP04$10E>").expect("valid slot");
    let word = encode_slot_to_u32(&slot);
    // Opcode OP is 0x01, shifted left 26 bits
    assert_eq!((word >> 26) & 0x3F, 0x01);
    // Dest reg is R4
    assert_eq!((word >> 22) & 0x0F, 4);
    // Src reg is R1
    assert_eq!((word >> 18) & 0x0F, 1);
}

#[test]
fn test_verilog_hdl_generation() {
    let cl_code = r#"
    ; Sample VLIW Module
    B0001: '=00#0A04> '=10#040B> _SH00$0B4> _SP00#008>
    B0002: _OP04$10E> _FA05$420> _BK09$501> _HL00$008!
    "#;

    let hdl = generate_verilog_hdl(cl_code, "test_core").expect("verilog generation");
    assert!(hdl.contains("module test_core ("));
    assert!(hdl.contains("output reg  [31:0] r0"));
    assert!(hdl.contains("output reg         halted"));
    assert!(hdl.contains("output reg         sentry_alert"));
    assert!(hdl.contains("inst_data <= 32'h"));
    assert!(hdl.contains("endmodule"));
}

#[test]
fn test_verilog_testbench_generation() {
    let tb = generate_testbench("test_core");
    assert!(tb.contains("module tb_test_core;"));
    assert!(tb.contains("test_core dut ("));
    assert!(tb.contains("always #5 clk = ~clk;"));
    assert!(tb.contains("rst_n = 1;"));
    assert!(tb.contains("enable = 1;"));
    assert!(tb.contains("$display(\"  Total Cycles Executed: %0d\", total_cycles);"));
    assert!(tb.contains("$finish(0);"));
}

#[test]
fn test_verilog_full_source_synthesis() {
    let source = r#"
    .MODULE EdgeCompute
    .ENTRY _main
    _main:
        let a = 10
        let b = 20
        let c = a + b
    .END
    "#;

    let hdl = cronc::compile_to_verilog(source, "cksl_edge_core").expect("compilation");
    assert!(hdl.contains("module cksl_edge_core"));
    assert!(hdl.contains("inst_data <="));
}
