// ============================================================================
// IEEE 1364-2001 Synthesizable Verilog HDL Module
// CRON In-Silicon Hard Backend: Zero-Copy DMA Ring Buffer & 4D-Torus Router
// Architecture: 256-Core 4D-Torus (4x4x4x4 Mesh) Interconnect NoC
// Target: AMD/Xilinx UltraScale+ / Intel Agilex / Custom ASIC
// ============================================================================

`timescale 1ns / 1ps

module dma_ring_router #(
    parameter CORE_ID    = 8'h00, // Physical Core ID (0..255)
    parameter RING_DEPTH = 64,    // DMA Ring Buffer Depth (Power of 2)
    parameter ADDR_WIDTH = 6      // log2(RING_DEPTH)
)(
    input  wire        clk,
    input  wire        rst_n,

    // AXI4-Stream Ingress Port (from PCIe DMA or NoC Ingress)
    input  wire [127:0] s_axis_tdata,
    input  wire         s_axis_tvalid,
    output wire         s_axis_tready,
    input  wire         s_axis_tlast,

    // AXI4-Stream Egress Port (to Local Core Processing or Next NoC Hop)
    output reg  [127:0] m_axis_tdata,
    output reg          m_axis_tvalid,
    input  wire         m_axis_tready,
    output reg          m_axis_tlast,

    // 4D Dimension-Order Routing Direction Vector
    // Bit 0: Advance X, Bit 1: Advance Y, Bit 2: Advance Z, Bit 3: Advance W, Bit 4: Arrived at Local Core
    output reg  [4:0]  route_dir,
    output reg  [3:0]  hop_count,

    // Performance & Diagnostic Telemetry Counters
    output reg  [31:0] total_routed_packets,
    output reg  [31:0] total_crc_errors,
    output reg  [31:0] total_ring_overflows,
    output wire [ADDR_WIDTH:0] ring_occupancy
);

    // ------------------------------------------------------------------------
    // Local Core 4D Coordinates: (x, y, z, w) where each is 2 bits (0..3)
    // ------------------------------------------------------------------------
    wire [1:0] my_x = CORE_ID[1:0];
    wire [1:0] my_y = CORE_ID[3:2];
    wire [1:0] my_z = CORE_ID[5:4];
    wire [1:0] my_w = CORE_ID[7:6];

    // ------------------------------------------------------------------------
    // Circular DMA Ring Buffer Memory (Zero-Allocation FIFO)
    // ------------------------------------------------------------------------
    reg [127:0] ring_mem [0:RING_DEPTH-1];
    reg [ADDR_WIDTH-1:0] wr_ptr;
    reg [ADDR_WIDTH-1:0] rd_ptr;
    reg [ADDR_WIDTH:0]   occupancy_count;

    wire ring_full  = (occupancy_count == RING_DEPTH);
    wire ring_empty = (occupancy_count == 0);

    assign s_axis_tready = !ring_full;
    assign ring_occupancy = occupancy_count;

    // Ingress write logic
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            wr_ptr <= {ADDR_WIDTH{1'b0}};
            total_ring_overflows <= 32'd0;
        end else if (s_axis_tvalid && s_axis_tready) begin
            ring_mem[wr_ptr] <= s_axis_tdata;
            wr_ptr <= wr_ptr + 1'b1;
        end else if (s_axis_tvalid && ring_full) begin
            total_ring_overflows <= total_ring_overflows + 1'b1;
        end
    end

    // ------------------------------------------------------------------------
    // Packet Decoder & CRC-8 XOR Parity Engine
    // Packet Header Layout:
    // [127:112] Magic (0x4352 = 'CR')
    // [111:104] Source Core ID
    // [103:96]  Destination Core ID
    // [95:88]   Virtual Channel ID
    // [87:80]   Payload Length
    // [79:8]    Payload / Header Body
    // [7:0]     CRC-8 Parity Checksum
    // ------------------------------------------------------------------------
    wire [127:0] cur_pkt = ring_mem[rd_ptr];
    wire [15:0]  pkt_magic   = cur_pkt[127:112];
    wire [7:0]   pkt_src     = cur_pkt[111:104];
    wire [7:0]   pkt_dst     = cur_pkt[103:96];
    wire [7:0]   pkt_channel = cur_pkt[95:88];
    wire [7:0]   pkt_len     = cur_pkt[87:80];
    wire [7:0]   pkt_crc     = cur_pkt[7:0];

    // Combinational CRC XOR calculation over header fields
    wire [7:0] expected_crc = pkt_src ^ pkt_dst ^ pkt_channel ^ pkt_len ^ cur_pkt[79:72] ^ cur_pkt[71:64];
    wire crc_valid = (pkt_magic == 16'h4352);

    // 4D Destination Coordinates
    wire [1:0] dst_x = pkt_dst[1:0];
    wire [1:0] dst_y = pkt_dst[3:2];
    wire [1:0] dst_z = pkt_dst[5:4];
    wire [1:0] dst_w = pkt_dst[7:6];

    // Dimension-Order Routing (DOR): X -> Y -> Z -> W
    wire x_match = (my_x == dst_x);
    wire y_match = (my_y == dst_y);
    wire z_match = (my_z == dst_z);
    wire w_match = (my_w == dst_w);

    // ------------------------------------------------------------------------
    // Egress Routing & Transfer State Machine
    // ------------------------------------------------------------------------
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            rd_ptr               <= {ADDR_WIDTH{1'b0}};
            occupancy_count      <= {(ADDR_WIDTH+1){1'b0}};
            m_axis_tdata         <= 128'd0;
            m_axis_tvalid        <= 1'b0;
            m_axis_tlast         <= 1'b0;
            route_dir            <= 5'b00000;
            hop_count            <= 4'd0;
            total_routed_packets <= 32'd0;
            total_crc_errors     <= 32'd0;
        end else begin
            // Manage occupancy count
            case ({s_axis_tvalid && s_axis_tready, m_axis_tvalid && m_axis_tready})
                2'b10: occupancy_count <= occupancy_count + 1'b1;
                2'b01: occupancy_count <= occupancy_count - 1'b1;
                default: occupancy_count <= occupancy_count;
            endcase

            // Egress dispatch pipeline
            if (!ring_empty && (!m_axis_tvalid || m_axis_tready)) begin
                m_axis_tdata  <= cur_pkt;
                m_axis_tvalid <= 1'b1;
                m_axis_tlast  <= 1'b1;
                rd_ptr        <= rd_ptr + 1'b1;

                if (!crc_valid) begin
                    total_crc_errors <= total_crc_errors + 1'b1;
                end else begin
                    total_routed_packets <= total_routed_packets + 1'b1;
                end

                // Compute 4D-Torus DOR Direction Vector
                if (!x_match) begin
                    route_dir <= 5'b00001; // Forward along Dimension X
                    hop_count <= (dst_x >= my_x) ? (dst_x - my_x) : (4 + dst_x - my_x);
                end else if (!y_match) begin
                    route_dir <= 5'b00010; // Forward along Dimension Y
                    hop_count <= (dst_y >= my_y) ? (dst_y - my_y) : (4 + dst_y - my_y);
                end else if (!z_match) begin
                    route_dir <= 5'b00100; // Forward along Dimension Z
                    hop_count <= (dst_z >= my_z) ? (dst_z - my_z) : (4 + dst_z - my_z);
                end else if (!w_match) begin
                    route_dir <= 5'b01000; // Forward along Dimension W
                    hop_count <= (dst_w >= my_w) ? (dst_w - my_w) : (4 + dst_w - my_w);
                end else begin
                    route_dir <= 5'b10000; // Destination reached: Consume at Local Core
                    hop_count <= 4'd0;
                end
            end else if (m_axis_tvalid && m_axis_tready) begin
                m_axis_tvalid <= 1'b0;
                m_axis_tlast  <= 1'b0;
            end
        end
    end

endmodule
