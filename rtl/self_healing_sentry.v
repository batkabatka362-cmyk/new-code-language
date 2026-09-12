// ============================================================================
// CRON Hardware Core: Self-Healing Sentry Unit
// Monitors per-core thermal dissipation and routing packet loss
// Automatically re-routes traffic along healthy toroidal axes if thresholds breached
// ============================================================================

`timescale 1ns / 1ps

module self_healing_sentry (
    input  wire        clk,
    input  wire        rst_n,
    input  wire [7:0]  thermal_sensor_val,
    input  wire [7:0]  thermal_threshold,
    input  wire [3:0]  fault_axis_mask,      // [3:X, 2:Y, 1:Z, 0:W]
    output reg         alarm_triggered,
    output reg  [2:0]  fallback_route_axis,  // Re-routed healthy neighbor direction
    output reg         core_throttle_en
);

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            alarm_triggered     <= 1'b0;
            fallback_route_axis <= 3'b000;
            core_throttle_en    <= 1'b0;
        end else begin
            if (thermal_sensor_val >= thermal_threshold) begin
                alarm_triggered  <= 1'b1;
                core_throttle_en <= 1'b1;
                
                // Autonomous axis re-routing (select first non-faulty axis)
                if (!fault_axis_mask[0]) fallback_route_axis <= 3'b001; // W
                else if (!fault_axis_mask[1]) fallback_route_axis <= 3'b010; // Z
                else if (!fault_axis_mask[2]) fallback_route_axis <= 3'b011; // Y
                else fallback_route_axis <= 3'b100; // X
            end else begin
                alarm_triggered  <= 1'b0;
                core_throttle_en <= 1'b0;
            end
        end
    end

endmodule
