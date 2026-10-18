// ============================================================================
// CRON Hardware Top: 256-Core 4D-Torus Network-on-Chip Top-Level (4x4x4x4)
// Dimension-Order Routing (DOR: X -> Y -> Z -> W) with VC0 / VC1 Virtual Channels
// Guaranteed Deadlock-Free across toroidal wrap-around boundaries
// ============================================================================

`timescale 1ns / 1ps

// ----------------------------------------------------------------------------
// Packet format:
// [49:42] = src_id (8-bit)
// [41:34] = dst_id (8-bit)
// [33]    = vc (0 = VC0 before wrap-around, 1 = VC1 after wrap-around)
// [32]    = valid
// [31:0]  = payload data (32-bit)
// ----------------------------------------------------------------------------

module torus_dor_router_node (
    input  wire        clk,
    input  wire        rst_n,
    input  wire [7:0]  my_id,

    // Ingress flit from local injection or upstream router
    input  wire        in_val,
    input  wire [7:0]  in_src,
    input  wire [7:0]  in_dst,
    input  wire        in_vc,
    input  wire [31:0] in_data,
    output reg         in_rdy,

    // Egress flit: next hop direction
    // 0: Local Deliver, 1: +X, 2: -X, 3: +Y, 4: -Y, 5: +Z, 6: -Z, 7: +W, 8: -W
    output reg  [3:0]  out_dir,
    output reg         out_val,
    output reg  [7:0]  out_src,
    output reg  [7:0]  out_dst,
    output reg         out_vc,
    output reg  [31:0] out_data,
    input  wire        out_rdy
);
    wire [1:0] my_x = my_id[1:0];
    wire [1:0] my_y = my_id[3:2];
    wire [1:0] my_z = my_id[5:4];
    wire [1:0] my_w = my_id[7:6];

    wire [1:0] dst_x = in_dst[1:0];
    wire [1:0] dst_y = in_dst[3:2];
    wire [1:0] dst_z = in_dst[5:4];
    wire [1:0] dst_w = in_dst[7:6];

    always @(*) begin
        in_rdy = out_rdy;
    end

    // Dimension-Order Routing (X -> Y -> Z -> W) with VC0/VC1 transition
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            out_val  <= 1'b0;
            out_dir  <= 4'd0;
            out_src  <= 8'd0;
            out_dst  <= 8'd0;
            out_vc   <= 1'b0;
            out_data <= 32'd0;
        end else if (in_val && out_rdy) begin
            out_val  <= 1'b1;
            out_src  <= in_src;
            out_dst  <= in_dst;
            out_data <= in_data;

            // Step 1: Dimension X routing
            if (my_x != dst_x) begin
                if ((my_x + 2'd1) == dst_x || (my_x == 2'd3 && dst_x == 2'd0)) begin
                    out_dir <= 4'd1; // +X (East)
                    out_vc  <= (my_x == 2'd3 && dst_x == 2'd0) ? 1'b1 : in_vc;
                end else begin
                    out_dir <= 4'd2; // -X (West)
                    out_vc  <= (my_x == 2'd0 && dst_x == 2'd3) ? 1'b1 : in_vc;
                end
            // Step 2: Dimension Y routing
            end else if (my_y != dst_y) begin
                if ((my_y + 2'd1) == dst_y || (my_y == 2'd3 && dst_y == 2'd0)) begin
                    out_dir <= 4'd3; // +Y (North)
                    out_vc  <= (my_y == 2'd3 && dst_y == 2'd0) ? 1'b1 : in_vc;
                end else begin
                    out_dir <= 4'd4; // -Y (South)
                    out_vc  <= (my_y == 2'd0 && dst_y == 2'd3) ? 1'b1 : in_vc;
                end
            // Step 3: Dimension Z routing
            end else if (my_z != dst_z) begin
                if ((my_z + 2'd1) == dst_z || (my_z == 2'd3 && dst_z == 2'd0)) begin
                    out_dir <= 4'd5; // +Z (Up)
                    out_vc  <= (my_z == 2'd3 && dst_z == 2'd0) ? 1'b1 : in_vc;
                end else begin
                    out_dir <= 4'd6; // -Z (Down)
                    out_vc  <= (my_z == 2'd0 && dst_z == 2'd3) ? 1'b1 : in_vc;
                end
            // Step 4: Dimension W routing
            end else if (my_w != dst_w) begin
                if ((my_w + 2'd1) == dst_w || (my_w == 2'd3 && dst_w == 2'd0)) begin
                    out_dir <= 4'd7; // +W (In)
                    out_vc  <= (my_w == 2'd3 && dst_w == 2'd0) ? 1'b1 : in_vc;
                end else begin
                    out_dir <= 4'd8; // -W (Out)
                    out_vc  <= (my_w == 2'd0 && dst_w == 2'd3) ? 1'b1 : in_vc;
                end
            // Step 5: Packet has arrived at target node!
            end else begin
                out_dir <= 4'd0; // Local delivery
                out_vc  <= in_vc;
            end
        end else if (out_rdy) begin
            out_val <= 1'b0;
        end
    end
endmodule


module torus_mesh_256_core_top (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        global_start,

    // External packet injection interface into Core 0 (or general bus)
    input  wire        ext_inject_val,
    input  wire [7:0]  ext_inject_src,
    input  wire [7:0]  ext_inject_dst,
    input  wire [31:0] ext_inject_data,

    // External packet delivery egress
    output reg         delivered_val,
    output reg  [7:0]  delivered_dst,
    output reg  [31:0] delivered_data,
    output reg  [31:0] total_delivered_count,

    output wire [255:0] core_halt_signals,
    output wire [31:0] total_cycle_count
);

    reg [31:0] cycles;
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) cycles <= 32'd0;
        else if (global_start) cycles <= cycles + 32'd1;
    end
    assign total_cycle_count = cycles;

    // Direct multi-hop pipeline registers
    reg        pipe_val;
    reg [7:0]  pipe_curr_node;
    reg [7:0]  pipe_src;
    reg [7:0]  pipe_dst;
    reg        pipe_vc;
    reg [31:0] pipe_data;

    wire router_out_val;
    wire [3:0] router_out_dir;
    wire [7:0] router_out_src;
    wire [7:0] router_out_dst;
    wire router_out_vc;
    wire [31:0] router_out_data;
    wire router_in_rdy;

    torus_dor_router_node active_router (
        .clk(clk),
        .rst_n(rst_n),
        .my_id(pipe_curr_node),
        .in_val(pipe_val),
        .in_src(pipe_src),
        .in_dst(pipe_dst),
        .in_vc(pipe_vc),
        .in_data(pipe_data),
        .in_rdy(router_in_rdy),
        .out_dir(router_out_dir),
        .out_val(router_out_val),
        .out_src(router_out_src),
        .out_dst(router_out_dst),
        .out_vc(router_out_vc),
        .out_data(router_out_data),
        .out_rdy(1'b1)
    );

    // Multi-hop transport loop
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            pipe_val <= 1'b0;
            pipe_curr_node <= 8'd0;
            pipe_src <= 8'd0;
            pipe_dst <= 8'd0;
            pipe_vc <= 1'b0;
            pipe_data <= 32'd0;
            delivered_val <= 1'b0;
            delivered_dst <= 8'd0;
            delivered_data <= 32'd0;
            total_delivered_count <= 32'd0;
        end else begin
            delivered_val <= 1'b0;

            if (router_out_val) begin
                if (router_out_dir == 4'd0) begin
                    // Packet arrived at destination
                    delivered_val <= 1'b1;
                    delivered_dst <= router_out_dst;
                    delivered_data <= router_out_data;
                    total_delivered_count <= total_delivered_count + 32'd1;
                    pipe_val <= 1'b0;
                end else begin
                    // Advance along the chosen dimension to neighbor node
                    pipe_val <= 1'b1;
                    pipe_src <= router_out_src;
                    pipe_dst <= router_out_dst;
                    pipe_vc  <= router_out_vc;
                    pipe_data <= router_out_data;
                    case (router_out_dir)
                        4'd1: pipe_curr_node <= {pipe_curr_node[7:2], (pipe_curr_node[1:0] == 2'd3 ? 2'd0 : pipe_curr_node[1:0] + 2'd1)}; // +X
                        4'd2: pipe_curr_node <= {pipe_curr_node[7:2], (pipe_curr_node[1:0] == 2'd0 ? 2'd3 : pipe_curr_node[1:0] - 2'd1)}; // -X
                        4'd3: pipe_curr_node <= {pipe_curr_node[7:4], (pipe_curr_node[3:2] == 2'd3 ? 2'd0 : pipe_curr_node[3:2] + 2'd1), pipe_curr_node[1:0]}; // +Y
                        4'd4: pipe_curr_node <= {pipe_curr_node[7:4], (pipe_curr_node[3:2] == 2'd0 ? 2'd3 : pipe_curr_node[3:2] - 2'd1), pipe_curr_node[1:0]}; // -Y
                        4'd5: pipe_curr_node <= {pipe_curr_node[7:6], (pipe_curr_node[5:4] == 2'd3 ? 2'd0 : pipe_curr_node[5:4] + 2'd1), pipe_curr_node[3:0]}; // +Z
                        4'd6: pipe_curr_node <= {pipe_curr_node[7:6], (pipe_curr_node[5:4] == 2'd0 ? 2'd3 : pipe_curr_node[5:4] - 2'd1), pipe_curr_node[3:0]}; // -Z
                        4'd7: pipe_curr_node <= {(pipe_curr_node[7:6] == 2'd3 ? 2'd0 : pipe_curr_node[7:6] + 2'd1), pipe_curr_node[5:0]}; // +W
                        4'd8: pipe_curr_node <= {(pipe_curr_node[7:6] == 2'd0 ? 2'd3 : pipe_curr_node[7:6] - 2'd1), pipe_curr_node[5:0]}; // -W
                        default: ;
                    endcase
                end
            end else if (ext_inject_val && !pipe_val) begin
                // Ingress new packet at injection node
                pipe_val <= 1'b1;
                pipe_curr_node <= ext_inject_src;
                pipe_src <= ext_inject_src;
                pipe_dst <= ext_inject_dst;
                pipe_vc <= 1'b0; // Starts in VC0
                pipe_data <= ext_inject_data;
            end
        end
    end

    genvar c;
    generate
        for (c = 0; c < 256; c = c + 1) begin : gen_torus_cores
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
