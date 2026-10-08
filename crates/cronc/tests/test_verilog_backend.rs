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

#[test]
fn test_verilog_torus_256_top_synthesis() {
    let cl_code = r#"
    ; 4D Sphere Geodesic 256-Core Microcode
    B0000: `A00010=0A `Q00030=15 `g00020=23 `w00040=37
    B0001: `C0FA11+02 `R0AK31+14 `g0KF21*21 `015541+31
    B0002: `A3:y14+09 `Q3y?34+16 `g3++24^22 `w3UU44!30
    "#;

    let top_rtl = cronc::verilog_backend::generate_torus_256_top_verilog(cl_code, "sphere_torus_256_top")
        .expect("256-Core Torus RTL synthesis failed");

    assert!(top_rtl.contains("module sphere_torus_256_top ("));
    assert!(top_rtl.contains("output wire        global_halted"));
    assert!(top_rtl.contains("output wire [31:0] total_aggregate_ops"));
    assert!(top_rtl.contains("output wire [31:0] core_0_acc_0"));
    assert!(top_rtl.contains("output wire [31:0] core_255_acc_0"));
    assert!(top_rtl.contains("module noc_router_4d_node"));
    assert!(top_rtl.contains("module sphere_torus_256_top_core"));
    assert!(top_rtl.contains("genvar cw, cz, cy, cx;"));
    assert!(top_rtl.contains("localparam E_ID = (cw * 64) + (cz * 16) + (cy * 4) + ((cx + 1) % 4);"));
    assert!(top_rtl.contains("localparam W_ID = (cw * 64) + (cz * 16) + (cy * 4) + ((cx + 3) % 4);"));
    assert!(top_rtl.contains("localparam N_ID = (cw * 64) + (cz * 16) + (((cy + 1) % 4) * 4) + cx;"));
    assert!(top_rtl.contains("localparam S_ID = (cw * 64) + (cz * 16) + (((cy + 3) % 4) * 4) + cx;"));

    let tb_rtl = cronc::verilog_backend::generate_torus_256_top_testbench("sphere_torus_256_top");
    assert!(tb_rtl.contains("module tb_sphere_torus_256_top;"));
    assert!(tb_rtl.contains("sphere_torus_256_top dut ("));
    assert!(tb_rtl.contains("Silicon Cores:              256 Cores (4x4x4x4 Toroidal Mesh)"));
    assert!(tb_rtl.contains("Total Aggregate 4D Ops:"));
    assert!(tb_rtl.contains("$finish(0);"));
}

