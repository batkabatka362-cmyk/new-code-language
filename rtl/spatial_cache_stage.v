// ============================================================================
// CRON Hardware Core: Spatial Cache Staging & Halo Blending Unit
// Double-buffers neighbor halo boundaries across 4D Torus dimensions
// Enables 0-wait halo blending directly into local scratchpad memory
// ============================================================================

`timescale 1ns / 1ps

module spatial_cache_stage (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        prefetch_en,
    input  wire        blend_en,
    input  wire [2:0]  neighbor_dir,     // 0..7 for 8 toroidal neighbors
    input  wire [31:0] incoming_halo_data,
    input  wire [31:0] local_tile_data,
    output reg  [31:0] blended_tile_out,
    output reg         stage_ready
);

    reg [31:0] staging_buffer [0:7];

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            blended_tile_out <= 32'h0;
            stage_ready      <= 1'b0;
        end else begin
            if (prefetch_en) begin
                staging_buffer[neighbor_dir] <= incoming_halo_data;
                stage_ready                  <= 1'b1;
            end

            if (blend_en) begin
                // Weighted halo blend (averaging boundary entries)
                blended_tile_out <= (local_tile_data + staging_buffer[neighbor_dir]) >> 1;
            end
        end
    end

endmodule
