// ============================================================================
// CRON Hardware Core: Sub-Byte SIMD ALU (Ternary 1.58b MAC)
// Computes 16 parallel 2-bit ternary dot products in a single cycle
// 2-bit encoding: 2'b00 = 0, 2'b01 = +1, 2'b10 = -1, 2'b11 = reserved
// ============================================================================

`timescale 1ns / 1ps

module subbyte_simd_alu (
    input  wire        clk,
    input  wire        rst_n,
    input  wire [31:0] weights_packed,  // 16x 2-bit ternary weights
    input  wire [31:0] acts_packed,     // 16x 2-bit activations
    output reg  [31:0] dot_product_out
);

    reg signed [31:0] accumulator;
    integer i;

    always @(*) begin
        accumulator = 32'sd0;
        for (i = 0; i < 16; i = i + 1) begin
            case (weights_packed[i*2 +: 2])
                2'b01: accumulator = accumulator + acts_packed[i*2 +: 2];
                2'b10: accumulator = accumulator - acts_packed[i*2 +: 2];
                default: ; // 0 does not alter accumulator
            endcase
        end
    end

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            dot_product_out <= 32'h0;
        end else begin
            dot_product_out <= accumulator;
        end
    end

endmodule
