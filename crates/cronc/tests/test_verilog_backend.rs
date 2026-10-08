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

#[test]
fn test_verilog_sphere_geodesic_matrix_synthesis() {
    // 4D Sphere Geodesic 10-char slot tokens
    let cl_code = r#"
    ; 4D Sphere Geodesic Matrix Core Microcode
    B0000: `A00010+00 `B00020+10 `C00030+20 `D00040+30
    B0001: `A12310*04 `B12320*14 `C12330*24 `D12340*34
    B0002: `A00010!00 `B00020!10 `C00030!20 `D00040!30
    "#;

    let slot = parse_slot("`A12310*04").expect("valid slot");
    let word = encode_slot_to_u32(&slot);
    // Opcode for 4D Sphere Geodesic is 0x38 (Opcode 6'h38)
    assert_eq!((word >> 26) & 0x3F, 0x38);

    let hdl = generate_verilog_hdl(cl_code, "sphere_matrix_core").expect("verilog generation");
    assert!(hdl.contains("module sphere_matrix_core ("));
    assert!(hdl.contains("output reg  [31:0] sphere_acc_0"));
    assert!(hdl.contains("output reg  [31:0] sphere_acc_1"));
    assert!(hdl.contains("output reg  [31:0] sphere_acc_2"));
    assert!(hdl.contains("output reg  [31:0] sphere_acc_3"));
    assert!(hdl.contains("output reg  [31:0] sphere_curvature_ops"));
    assert!(hdl.contains("6'h38: begin // 4D Sphere Geodesic ALU"));
    assert!(hdl.contains("sphere_acc[sph_head] <= sphere_acc[sph_head] + sph_geo_term;"));

    let tb = generate_testbench("sphere_matrix_core");
    assert!(tb.contains("sphere_matrix_core dut ("));
    assert!(tb.contains(".sphere_acc_0(sphere_acc_0)"));
    assert!(tb.contains("Sphere Acc [0]: %0d"));
}

