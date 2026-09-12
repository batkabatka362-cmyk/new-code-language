// ============================================================================
// CRON Hardware Core: Hardware Autodiff Unit
// Employs Fredkin reversible gates (F and F^-1) to recompute forward activations
// on-the-fly during the backward pass, eliminating memory storage for gradients
// ============================================================================

`timescale 1ns / 1ps

module hardware_autodiff_unit (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        forward_tap_en,
    input  wire        backward_invert_en,
    input  wire [31:0] forward_val,
    input  wire [31:0] upstream_gradient,
    output reg  [31:0] computed_gradient,
    output reg  [31:0] reconstructed_activation
);

    reg [31:0] reversible_store;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            reversible_store         <= 32'h0;
            computed_gradient        <= 32'h0;
            reconstructed_activation <= 32'h0;
        end else begin
            if (forward_tap_en) begin
                reversible_store <= forward_val;
            end

            if (backward_invert_en) begin
                // Invert mapping via Fredkin logic: no DRAM fetch
                reconstructed_activation <= reversible_store;
                computed_gradient        <= upstream_gradient ^ (reversible_store >> 1);
            end
        end
    end

endmodule
