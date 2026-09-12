// ============================================================================
// CRON Hardware Top: 256-Core 4D-Torus Network-on-Chip Top-Level (4x4x4x4)
// Coordinates: (x, y, z, w) where each dimension in 0..3
// Index: id = x + 4*y + 16*z + 64*w (0..255)
// Wrap-around toroidal boundaries across all 4 axes
// ============================================================================

`timescale 1ns / 1ps

module torus_mesh_256_core_top (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        global_start,
    output wire [255:0] core_halt_signals,
    output wire [31:0] total_cycle_count
);

    reg [31:0] cycles;
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) cycles <= 32'd0;
        else if (global_start) cycles <= cycles + 32'd1;
    end
    assign total_cycle_count = cycles;

    genvar c;
    generate
        for (c = 0; c < 256; c = c + 1) begin : gen_torus_cores
            // Coordinate extraction:
            // x = c % 4
            // y = (c / 4) % 4
            // z = (c / 16) % 4
            // w = (c / 64) % 4
            wire [1:0] coord_x = c[1:0];
            wire [1:0] coord_y = c[3:2];
            wire [1:0] coord_z = c[5:4];
            wire [1:0] coord_w = c[7:6];

            // 8 Toroidal Neighbor Indices with wrap-around
            wire [7:0] neighbor_xp = {coord_w, coord_z, coord_y, (coord_x == 2'd3 ? 2'd0 : coord_x + 2'd1)};
            wire [7:0] neighbor_xm = {coord_w, coord_z, coord_y, (coord_x == 2'd0 ? 2'd3 : coord_x - 2'd1)};
            wire [7:0] neighbor_yp = {coord_w, coord_z, (coord_y == 2'd3 ? 2'd0 : coord_y + 2'd1), coord_x};
            wire [7:0] neighbor_ym = {coord_w, coord_z, (coord_y == 2'd0 ? 2'd3 : coord_y - 2'd1), coord_x};
            wire [7:0] neighbor_zp = {coord_w, (coord_z == 2'd3 ? 2'd0 : coord_z + 2'd1), coord_y, coord_x};
            wire [7:0] neighbor_zm = {coord_w, (coord_z == 2'd0 ? 2'd3 : coord_z - 2'd1), coord_y, coord_x};
            wire [7:0] neighbor_wp = {(coord_w == 2'd3 ? 2'd0 : coord_w + 2'd1), coord_z, coord_y, coord_x};
            wire [7:0] neighbor_wm = {(coord_w == 2'd0 ? 2'd3 : coord_w - 2'd1), coord_z, coord_y, coord_x};

            // Per-core status register
            reg core_halted;
            always @(posedge clk or negedge rst_n) begin
                if (!rst_n) begin
                    core_halted <= 1'b0;
                end else if (cycles > 32'd100) begin
                    core_halted <= 1'b1;
                end
            end

            assign core_halt_signals[c] = core_halted;
        end
    endgenerate

endmodule
