// ============================================================================
// IEEE 1364-2001 Synthesizable Verilog HDL Module
// CRON Multi-Chip 5D/6D Inter-Chip Optical Waveguide Crossbar Router
// Architecture: 16-Chip (4x4 2D Mesh) Interconnecting 4,096 Distributed Cores
// Target: AMD/Xilinx UltraScale+ / Intel Agilex / Custom TSMC N5 SiPh ASIC
// ============================================================================

`timescale 1ns / 1ps

module cluster_crossbar_5d #(
    parameter CHIP_ID    = 4'h0, // Physical Chip Socket ID (0..15)
    parameter DATA_WIDTH = 128   // Optical Flit Width (128-bit)
)(
    input  wire                   clk,
    input  wire                   rst_n,

    // Port 0: Intra-Chip Local NoC (To Core [3, 3, 3, 3] Gateway)
    input  wire [DATA_WIDTH-1:0]  local_in_data,
    input  wire                   local_in_valid,
    output wire                   local_in_ready,
    output reg  [DATA_WIDTH-1:0]  local_out_data,
    output reg                    local_out_valid,
    input  wire                   local_out_ready,

    // Port 1: North Optical Waveguide Link (Chip Y - 1)
    input  wire [DATA_WIDTH-1:0]  north_in_data,
    input  wire                   north_in_valid,
    output wire                   north_in_ready,
    output reg  [DATA_WIDTH-1:0]  north_out_data,
    output reg                    north_out_valid,
    input  wire                   north_out_ready,

    // Port 2: South Optical Waveguide Link (Chip Y + 1)
    input  wire [DATA_WIDTH-1:0]  south_in_data,
    input  wire                   south_in_valid,
    output wire                   south_in_ready,
    output reg  [DATA_WIDTH-1:0]  south_out_data,
    output reg                    south_out_valid,
    input  wire                   south_out_ready,

    // Port 3: East Optical Waveguide Link (Chip X + 1)
    input  wire [DATA_WIDTH-1:0]  east_in_data,
    input  wire                   east_in_valid,
    output wire                   east_in_ready,
    output reg  [DATA_WIDTH-1:0]  east_out_data,
    output reg                    east_out_valid,
    input  wire                   east_out_ready,

    // Port 4: West Optical Waveguide Link (Chip X - 1)
    input  wire [DATA_WIDTH-1:0]  west_in_data,
    input  wire                   west_in_valid,
    output wire                   west_in_ready,
    output reg  [DATA_WIDTH-1:0]  west_out_data,
    output reg                    west_out_valid,
    input  wire                   west_out_ready,

    // Optical Network Telemetry & Performance Counters
    output reg  [31:0]            total_cross_chip_packets,
    output reg  [31:0]            total_optical_wavelength_shifts,
    output wire                   link_active
);

    // Current 2D Chip Coordinates (4x4 Grid)
    wire [1:0] my_chip_x = CHIP_ID[1:0];
    wire [1:0] my_chip_y = CHIP_ID[3:2];

    assign local_in_ready = local_out_ready;
    assign north_in_ready = north_out_ready;
    assign south_in_ready = south_out_ready;
    assign east_in_ready  = east_out_ready;
    assign west_in_ready  = west_out_ready;

    assign link_active = local_in_valid | north_in_valid | south_in_valid | east_in_valid | west_in_valid;

    // Header extraction from Local Ingress Packet:
    // [127:124] Target Chip ID (0..15)
    // [123:116] Target Intra-Chip Core ID (0..255)
    wire [3:0] target_chip = local_in_data[127:124];
    wire [1:0] target_x    = target_chip[1:0];
    wire [1:0] target_y    = target_chip[3:2];

    // Dimension-Order Routing (DOR): X-first, then Y
    wire need_east  = (target_x > my_chip_x) || (my_chip_x == 2'b11 && target_x == 2'b00);
    wire need_west  = (target_x < my_chip_x) || (my_chip_x == 2'b00 && target_x == 2'b11);
    wire need_south = (target_x == my_chip_x) && ((target_y > my_chip_y) || (my_chip_y == 2'b11 && target_y == 2'b00));
    wire need_north = (target_x == my_chip_x) && ((target_y < my_chip_y) || (my_chip_y == 2'b00 && target_y == 2'b11));
    wire is_local   = (target_chip == CHIP_ID);

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            local_out_data                  <= {DATA_WIDTH{1'b0}};
            local_out_valid                 <= 1'b0;
            north_out_data                  <= {DATA_WIDTH{1'b0}};
            north_out_valid                 <= 1'b0;
            south_out_data                  <= {DATA_WIDTH{1'b0}};
            south_out_valid                 <= 1'b0;
            east_out_data                   <= {DATA_WIDTH{1'b0}};
            east_out_valid                  <= 1'b0;
            west_out_data                   <= {DATA_WIDTH{1'b0}};
            west_out_valid                  <= 1'b0;
            total_cross_chip_packets        <= 32'd0;
            total_optical_wavelength_shifts <= 32'd0;
        end else begin
            // Local ingress dispatching
            if (local_in_valid && local_in_ready) begin
                total_cross_chip_packets <= total_cross_chip_packets + 1'b1;
                if (need_east) begin
                    east_out_data  <= local_in_data;
                    east_out_valid <= 1'b1;
                end else if (need_west) begin
                    west_out_data  <= local_in_data;
                    west_out_valid <= 1'b1;
                end else if (need_south) begin
                    south_out_data  <= local_in_data;
                    south_out_valid <= 1'b1;
                end else if (need_north) begin
                    north_out_data  <= local_in_data;
                    north_out_valid <= 1'b1;
                end else if (is_local) begin
                    local_out_data  <= local_in_data;
                    local_out_valid <= 1'b1;
                end
            end else begin
                if (east_out_ready)  east_out_valid  <= 1'b0;
                if (west_out_ready)  west_out_valid  <= 1'b0;
                if (south_out_ready) south_out_valid <= 1'b0;
                if (north_out_ready) north_out_valid <= 1'b0;
                if (local_out_ready) local_out_valid <= 1'b0;
            end

            // Optical transit forwarding from ingress directions
            if (east_in_valid && is_local) begin
                local_out_data  <= east_in_data;
                local_out_valid <= 1'b1;
                total_optical_wavelength_shifts <= total_optical_wavelength_shifts + 1'b1;
            end else if (west_in_valid && is_local) begin
                local_out_data  <= west_in_data;
                local_out_valid <= 1'b1;
                total_optical_wavelength_shifts <= total_optical_wavelength_shifts + 1'b1;
            end
        end
    end

endmodule
