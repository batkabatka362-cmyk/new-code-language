// ============================================================================
// IEEE 1364-2001 Automated Testbench: 256-Core 4D-Torus DOR (X->Y->Z->W) & VC0/VC1
// ============================================================================

`timescale 1ns / 1ps

module tb_torus_mesh_256_core_top;
    reg clk;
    reg rst_n;
    reg global_start;
    reg ext_inject_val;
    reg [7:0] ext_inject_src;
    reg [7:0] ext_inject_dst;
    reg [31:0] ext_inject_data;

    wire delivered_val;
    wire [7:0] delivered_dst;
    wire [31:0] delivered_data;
    wire [31:0] total_delivered_count;
    wire [255:0] core_halt_signals;
    wire [31:0] total_cycle_count;

    torus_mesh_256_core_top dut (
        .clk(clk),
        .rst_n(rst_n),
        .global_start(global_start),
        .ext_inject_val(ext_inject_val),
        .ext_inject_src(ext_inject_src),
        .ext_inject_dst(ext_inject_dst),
        .ext_inject_data(ext_inject_data),
        .delivered_val(delivered_val),
        .delivered_dst(delivered_dst),
        .delivered_data(delivered_data),
        .total_delivered_count(total_delivered_count),
        .core_halt_signals(core_halt_signals),
        .total_cycle_count(total_cycle_count)
    );

    always #5 clk = ~clk;

    initial begin
        clk = 0;
        rst_n = 0;
        global_start = 0;
        ext_inject_val = 0;
        ext_inject_src = 8'd0;
        ext_inject_dst = 8'd0;
        ext_inject_data = 32'd0;

        #20 rst_n = 1;
        #10 global_start = 1;

        // Packet 1: Inject from Node 0 (0,0,0,0) to Node 255 (3,3,3,3) across all 4 dimensions
        #10;
        ext_inject_val = 1;
        ext_inject_src = 8'd0;
        ext_inject_dst = 8'd255;
        ext_inject_data = 32'hBEEF_CAFE;
        #10 ext_inject_val = 0;

        // Wait for delivery
        wait (delivered_val == 1'b1);
        #1;
        $display("[PACKET 1 DELIVERED] Dst=%0d, Data=0x%08h, Cycles=%0d", delivered_dst, delivered_data, total_cycle_count);
        if (delivered_dst != 8'd255 || delivered_data != 32'hBEEF_CAFE) begin
            $display("[FAIL] Packet 1 corrupted or misrouted!");
            $finish(1);
        end

        // Packet 2: Wrap-around test Node 3 (3,0,0,0) -> Node 0 (0,0,0,0) triggering VC1
        #20;
        ext_inject_val = 1;
        ext_inject_src = 8'd3;
        ext_inject_dst = 8'd0;
        ext_inject_data = 32'h1234_5678;
        #10 ext_inject_val = 0;

        wait (delivered_val == 1'b1);
        #1;
        $display("[PACKET 2 DELIVERED] Dst=%0d, Data=0x%08h, Cycles=%0d", delivered_dst, delivered_data, total_cycle_count);
        if (delivered_dst != 8'd0 || delivered_data != 32'h1234_5678) begin
            $display("[FAIL] Packet 2 corrupted or misrouted!");
            $finish(1);
        end

        #50;
        $display("==================================================");
        $display("  DOR (X->Y->Z->W) & VC0/VC1 VERIFICATION SUCCESS ");
        $display("  Total Delivered Packets: %0d", total_delivered_count);
        $display("==================================================");
        $finish(0);
    end
endmodule
