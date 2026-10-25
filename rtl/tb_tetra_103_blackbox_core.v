// ============================================================================
// TESTBENCH: Tetra 1.03-Bit Neural Black Box Core
// Verifies 0-DSP Single Cycle Forward Wave Processing & Facet Branching
// ============================================================================

`timescale 1ns / 1ps

module tb_tetra_103_blackbox_core;

    reg         clk;
    reg         rst_n;
    reg         enable;
    reg         mode_step;
    reg  [31:0] act_wave_e0;
    reg  [31:0] act_wave_e1;
    reg  [31:0] act_wave_e2;
    reg  [31:0] act_wave_e3;
    reg  [127:0] weight_v_packed;
    reg  [3:0]   micro_gain;

    wire [31:0] out_wave_e0;
    wire [31:0] out_wave_e1;
    wire [1:0]  facet_branch_out;
    wire [15:0] resonance_energy;
    wire        valid_out;

    // Instantiate Black Box Unit Under Test (UUT)
    tetra_103_blackbox_core uut (
        .clk(clk),
        .rst_n(rst_n),
        .enable(enable),
        .mode_step(mode_step),
        .act_wave_e0(act_wave_e0),
        .act_wave_e1(act_wave_e1),
        .act_wave_e2(act_wave_e2),
        .act_wave_e3(act_wave_e3),
        .weight_v_packed(weight_v_packed),
        .micro_gain(micro_gain),
        .out_wave_e0(out_wave_e0),
        .out_wave_e1(out_wave_e1),
        .facet_branch_out(facet_branch_out),
        .resonance_energy(resonance_energy),
        .valid_out(valid_out)
    );

    // Clock generation (100 MHz = 10ns period)
    always #5 clk = ~clk;

    initial begin
        clk = 0;
        rst_n = 0;
        enable = 0;
        mode_step = 1;
        act_wave_e0 = 32'h0;
        act_wave_e1 = 32'h0;
        act_wave_e2 = 32'h0;
        act_wave_e3 = 32'h0;
        weight_v_packed = 128'h0;
        micro_gain = 4'd8;

        #20 rst_n = 1;
        #10 enable = 1;

        // Cycle 1: DPSTA Character Wave (ASCII 'M' = 77)
        act_wave_e0 = 32'h02C9FB4D ^ 77;
        act_wave_e1 = 32'h55555555 ^ 77;
        weight_v_packed = 128'hA5A5A5A5_5A5A5A5A_F0F0F0F0_0F0F0F0F;
        micro_gain = 4'd12;
        #10;

        // Cycle 2: Next Token Resonance ('o' = 111)
        act_wave_e0 = out_wave_e0 ^ 111;
        act_wave_e1 = out_wave_e1 ^ 111;
        weight_v_packed = 128'h33333333_CCCCCCCC_AAAAAAAA_55555555;
        micro_gain = 4'd10;
        #10;

        // Cycle 3: Context Wave ('n' = 110)
        act_wave_e0 = out_wave_e0 ^ 110;
        act_wave_e1 = out_wave_e1 ^ 110;
        #10;

        enable = 0;
        #20;
        $finish;
    end

endmodule
