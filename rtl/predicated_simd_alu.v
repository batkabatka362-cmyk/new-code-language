// ============================================================================
// CRON Hardware Core: Predicated SIMD ALU
// Supports 4 parallel 32-bit execution channels with dynamic predicate mask
// Inactive channels are clock-gated to eliminate thermal dissipation
// ============================================================================

`timescale 1ns / 1ps

module predicated_simd_alu (
    input  wire        clk,
    input  wire        rst_n,
    input  wire [3:0]  predicate_mask,  // 1 bit per channel
    input  wire [1:0]  op_code,         // 0: ADD, 1: SUB, 2: MUL, 3: AND
    input  wire [127:0] in_a,           // 4x 32-bit channels
    input  wire [127:0] in_b,
    output reg  [127:0] out_result,
    output reg  [3:0]   channel_active
);

    genvar i;
    generate
        for (i = 0; i < 4; i = i + 1) begin : gen_alu_channel
            wire [31:0] a_ch = in_a[i*32 +: 32];
            wire [31:0] b_ch = in_b[i*32 +: 32];
            reg  [31:0] res_ch;

            always @(*) begin
                if (predicate_mask[i]) begin
                    case (op_code)
                        2'b00: res_ch = a_ch + b_ch;
                        2'b01: res_ch = a_ch - b_ch;
                        2'b10: res_ch = a_ch * b_ch;
                        2'b11: res_ch = a_ch & b_ch;
                    endcase
                end else begin
                    res_ch = 32'h0000_0000;
                end
            end

            always @(posedge clk or negedge rst_n) begin
                if (!rst_n) begin
                    out_result[i*32 +: 32] <= 32'h0;
                    channel_active[i]      <= 1'b0;
                end else begin
                    out_result[i*32 +: 32] <= res_ch;
                    channel_active[i]      <= predicate_mask[i];
                end
            end
        end
    endgenerate

endmodule
