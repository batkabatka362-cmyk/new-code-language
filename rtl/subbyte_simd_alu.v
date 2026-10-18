// ============================================================================
// CRON Hardware Core: Sub-Byte SIMD ALU (BitNet b1.58 0-DSP Ternary MAC)
// Computes 16 parallel 2-bit ternary dot products in a single cycle
// 2-bit encoding: 2'b00 = 0, 2'b01 = +1, 2'b10 = -1, 2'b11 = reserved (0)
// Hardware Architecture: 0 DSP Multipliers, Balanced 4-Level Popcount Tree
// ============================================================================

`timescale 1ns / 1ps

module subbyte_simd_alu (
    input  wire        clk,
    input  wire        rst_n,
    input  wire [31:0] weights_packed,  // 16x 2-bit ternary weights
    input  wire [31:0] acts_packed,     // 16x 2-bit activations
    output reg  [31:0] dot_product_out
);

    // 1. Bit-slice decomposition for weights and activations
    reg [15:0] w_pos;
    reg [15:0] w_neg;
    reg [15:0] a_bit0;
    reg [15:0] a_bit1;
    integer k;

    always @(*) begin
        for (k = 0; k < 16; k = k + 1) begin
            w_pos[k]  = (weights_packed[k*2 +: 2] == 2'b01);
            w_neg[k]  = (weights_packed[k*2 +: 2] == 2'b10);
            a_bit0[k] = acts_packed[k*2];
            a_bit1[k] = acts_packed[k*2 + 1];
        end
    end

    // 2. Synthesizable 4-Level Balanced Popcount Tree (0-DSP Multiplier-Free)
    function [4:0] popcount16;
        input [15:0] vec;
        reg [1:0] s1 [7:0];
        reg [2:0] s2 [3:0];
        reg [3:0] s3 [1:0];
        integer j;
        begin
            // Level 1: 8 pairs -> 2-bit sums
            for (j = 0; j < 8; j = j + 1) begin
                s1[j] = vec[2*j] + vec[2*j + 1];
            end
            // Level 2: 4 pairs -> 3-bit sums
            for (j = 0; j < 4; j = j + 1) begin
                s2[j] = s1[2*j] + s1[2*j + 1];
            end
            // Level 3: 2 pairs -> 4-bit sums
            for (j = 0; j < 2; j = j + 1) begin
                s3[j] = s2[2*j] + s2[2*j + 1];
            end
            // Level 4: Final 5-bit sum [0..16]
            popcount16 = s3[0] + s3[1];
        end
    endfunction

    // 3. Decomposed bitwise products
    wire [15:0] p0 = w_pos & a_bit0;
    wire [15:0] n0 = w_neg & a_bit0;
    wire [15:0] p1 = w_pos & a_bit1;
    wire [15:0] n1 = w_neg & a_bit1;

    // 4. Popcount sums & Signed difference
    wire [4:0] cnt_p0 = popcount16(p0);
    wire [4:0] cnt_n0 = popcount16(n0);
    wire [4:0] cnt_p1 = popcount16(p1);
    wire [4:0] cnt_n1 = popcount16(n1);

    wire signed [6:0] diff0 = $signed({2'b00, cnt_p0}) - $signed({2'b00, cnt_n0});
    wire signed [6:0] diff1 = $signed({2'b00, cnt_p1}) - $signed({2'b00, cnt_n1});

    // 5. Recombine: (diff1 << 1) + diff0
    wire signed [7:0] dot_comb = (diff1 <<< 1) + diff0;

    // 6. Registered output with synchronous reset
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            dot_product_out <= 32'sd0;
        end else begin
            dot_product_out <= {{24{dot_comb[7]}}, dot_comb};
        end
    end

endmodule

