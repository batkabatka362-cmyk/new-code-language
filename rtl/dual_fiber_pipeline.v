// ============================================================================
// CRON Hardware Core: Dual-Fiber Coroutine Pipeline
// Manages Fiber 0 (Main Execution) and Fiber 1 (Background I/O / Mesh Transfer)
// Switches context in 0 cycles when an execution unit encounters a network stall
// ============================================================================

`timescale 1ns / 1ps

module dual_fiber_pipeline (
    input  wire        clk,
    input  wire        rst_n,
    input  wire        fiber1_spawn,
    input  wire        fiber1_join,
    input  wire        fiber_yield,
    input  wire        noc_stall,
    input  wire [31:0] pc_in_f0,
    input  wire [31:0] pc_in_f1,
    output reg         active_fiber,    // 0: Fiber 0, 1: Fiber 1
    output reg  [31:0] current_pc
);

    reg fiber1_alive;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            active_fiber  <= 1'b0;
            fiber1_alive  <= 1'b0;
            current_pc    <= 32'h0;
        end else begin
            if (fiber1_spawn) begin
                fiber1_alive <= 1'b1;
            end else if (fiber1_join) begin
                fiber1_alive <= 1'b0;
                active_fiber <= 1'b0;
            end

            // Context switch logic: 0-cycle switch if active fiber encounters stall
            if (active_fiber == 1'b0) begin
                if (noc_stall && fiber1_alive) begin
                    active_fiber <= 1'b1;
                    current_pc   <= pc_in_f1;
                end else begin
                    current_pc   <= pc_in_f0;
                end
            end else begin
                if (fiber_yield || !fiber1_alive) begin
                    active_fiber <= 1'b0;
                    current_pc   <= pc_in_f0;
                end else begin
                    current_pc   <= pc_in_f1;
                end
            end
        end
    end

endmodule
