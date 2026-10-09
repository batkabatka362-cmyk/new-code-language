// ============================================================================
// CRON Matrix 4.1: Synthesizable 4D Spherical Geodesic Cortex Hardware Core
// Standard: IEEE 1364-2001 Verilog HDL
// Target: 256-Core 4D-Torus S^3 x Z100 Hard Silicon
// ============================================================================

module matrix4_cortex_top #(
    parameter DATA_WIDTH = 128,
    parameter DEPTH_WIDTH = 7,
    parameter SURFACE_WIDTH = 7,
    parameter ROM_DEPTH = 5
)(
    input  wire                   clk,
    input  wire                   rst_n,
    input  wire                   exec_en,
    output reg  [DEPTH_WIDTH-1:0] w0, w1, w2, w3,
    output reg  [SURFACE_WIDTH-1:0] x0, x1, x2, x3,
    output reg  [SURFACE_WIDTH-1:0] z0, z1, z2, z3,
    output reg  [7:0]             vol0, vol1, vol2, vol3,
    output reg  [1:0]             spin0, spin1, spin2, spin3,
    output reg                    hamiltonian_ok,
    output reg                    collision_deflected,
    output reg                    halt_flag
);

    // 128-Bit Instruction ROM
    reg [DATA_WIDTH-1:0] rom [0:ROM_DEPTH-1];
    reg [7:0] pc;

    initial begin
        $readmemh("matrix4_rom.hex", rom);
    end

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            pc <= 8'd0;
            w0 <= 7'd0; x0 <= 7'd0; z0 <= 7'd0; vol0 <= 8'd34; spin0 <= 2'd0;
            w1 <= 7'd0; x1 <= 7'd0; z1 <= 7'd0; vol1 <= 8'd33; spin1 <= 2'd0;
            w2 <= 7'd0; x2 <= 7'd0; z2 <= 7'd0; vol2 <= 8'd33; spin2 <= 2'd0;
            w3 <= 7'd0; x3 <= 7'd0; z3 <= 7'd0; vol3 <= 8'd0;  spin3 <= 2'd0;
            hamiltonian_ok <= 1'b1;
            collision_deflected <= 1'b0;
            halt_flag <= 1'b0;
        end else if (exec_en && !halt_flag) begin
            // 0-Anchor Hamiltonian Verification
            hamiltonian_ok <= ((vol0 + vol1 + vol2 + vol3) == 8'd100);

            // Braid Deflector Hardware Collision Avoidance (|W_i - W_j| >= 1)
            if ((x0 == x1) && (z0 == z1) && (w0 == w1)) begin
                w0 <= w0 + 7'd1;
                collision_deflected <= 1'b1;
            end else begin
                collision_deflected <= 1'b0;
            end

            // Cycle Advance
            if (pc < ROM_DEPTH - 1) begin
                pc <= pc + 8'd1;
            end else begin
                halt_flag <= 1'b1;
            end
        end
    end

endmodule
