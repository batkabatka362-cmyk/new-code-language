// ==============================================================================
// CRON FPGA Physical Hardware UART Transceiver Bridge (cron_uart.v)
// Clock: 25000000 Hz | Baud Rate: 115200 (8-N-1)
// Connects Host PC USB-Serial directly to CRON 4D-Torus Ingress/Egress Interface
// Strictly 0 DSP Multipliers (Pure LUT Counter Logic)
// ==============================================================================

`timescale 1ns / 1ps

module cron_uart #(
    parameter CLK_FREQ_HZ = 25000000,
    parameter BAUD_RATE   = 115200
)(
    input  wire       clk,
    input  wire       rst_n,
    input  wire       uart_rx,
    output reg        uart_tx,
    
    // CRON 4D-Torus Ingress Stream (from Host)
    output reg  [7:0] rx_data,
    output reg        rx_valid,
    input  wire       rx_ready,
    
    // CRON 4D-Torus Egress Stream (to Host)
    input  wire [7:0] tx_data,
    input  wire       tx_valid,
    output reg        tx_ready,
    
    // Hardware Diagnostics & Physical LED Indicators
    output reg        led_heartbeat,
    output reg        led_torus_active
);

    localparam CLKS_PER_BIT = CLK_FREQ_HZ / BAUD_RATE;
    localparam CNT_WIDTH = 20;

    // --- Heartbeat & Activity Diagnostic Counters ---
    reg [24:0] hb_counter;
    reg [19:0] activity_timer;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            hb_counter <= 25'd0;
            led_heartbeat <= 1'b0;
            activity_timer <= 20'd0;
            led_torus_active <= 1'b0;
        end else begin
            hb_counter <= hb_counter + 1'b1;
            led_heartbeat <= hb_counter[24]; // ~0.75 Hz blink at 25MHz

            if (rx_valid || tx_valid) begin
                activity_timer <= 20'd500_000;
                led_torus_active <= 1'b1;
            end else if (activity_timer > 0) begin
                activity_timer <= activity_timer - 1'b1;
                led_torus_active <= 1'b1;
            end else begin
                led_torus_active <= 1'b0;
            end
        end
    end

    // --- UART Receiver (RX) with 3-Stage Synchronizer & Mid-Bit Sampling ---
    reg [2:0] rx_sync;
    always @(posedge clk or negedge rst_n) begin
        if (!rst_n)
            rx_sync <= 3'b111;
        else
            rx_sync <= {rx_sync[1:0], uart_rx};
    end
    wire rx_in = rx_sync[2];

    localparam RX_IDLE  = 3'd0;
    localparam RX_START = 3'd1;
    localparam RX_DATA  = 3'd2;
    localparam RX_STOP  = 3'd3;

    reg [2:0] rx_state;
    reg [CNT_WIDTH-1:0] rx_clk_cnt;
    reg [2:0] rx_bit_idx;
    reg [7:0] rx_byte;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            rx_state   <= RX_IDLE;
            rx_clk_cnt <= 0;
            rx_bit_idx <= 0;
            rx_byte    <= 8'd0;
            rx_data    <= 8'd0;
            rx_valid   <= 1'b0;
        end else begin
            rx_valid <= 1'b0;
            case (rx_state)
                RX_IDLE: begin
                    rx_clk_cnt <= 0;
                    rx_bit_idx <= 0;
                    if (rx_in == 1'b0)
                        rx_state <= RX_START;
                end

                RX_START: begin
                    if (rx_clk_cnt == (CLKS_PER_BIT / 2)) begin
                        if (rx_in == 1'b0) begin
                            rx_clk_cnt <= 0;
                            rx_state   <= RX_DATA;
                        end else begin
                            rx_state <= RX_IDLE;
                        end
                    end else begin
                        rx_clk_cnt <= rx_clk_cnt + 1'b1;
                    end
                end

                RX_DATA: begin
                    if (rx_clk_cnt == (CLKS_PER_BIT - 1)) begin
                        rx_clk_cnt <= 0;
                        rx_byte[rx_bit_idx] <= rx_in;
                        if (rx_bit_idx == 3'd7) begin
                            rx_state <= RX_STOP;
                        end else begin
                            rx_bit_idx <= rx_bit_idx + 1'b1;
                        end
                    end else begin
                        rx_clk_cnt <= rx_clk_cnt + 1'b1;
                    end
                end

                RX_STOP: begin
                    if (rx_clk_cnt == (CLKS_PER_BIT - 1)) begin
                        rx_clk_cnt <= 0;
                        rx_data    <= rx_byte;
                        rx_valid   <= 1'b1;
                        rx_state   <= RX_IDLE;
                    end else begin
                        rx_clk_cnt <= rx_clk_cnt + 1'b1;
                    end
                end

                default: rx_state <= RX_IDLE;
            endcase
        end
    end

    // --- UART Transmitter (TX) ---
    localparam TX_IDLE  = 3'd0;
    localparam TX_START = 3'd1;
    localparam TX_DATA  = 3'd2;
    localparam TX_STOP  = 3'd3;

    reg [2:0] tx_state;
    reg [CNT_WIDTH-1:0] tx_clk_cnt;
    reg [2:0] tx_bit_idx;
    reg [7:0] tx_byte;

    always @(posedge clk or negedge rst_n) begin
        if (!rst_n) begin
            tx_state   <= TX_IDLE;
            tx_clk_cnt <= 0;
            tx_bit_idx <= 0;
            tx_byte    <= 8'd0;
            uart_tx    <= 1'b1;
            tx_ready   <= 1'b1;
        end else begin
            case (tx_state)
                TX_IDLE: begin
                    uart_tx  <= 1'b1;
                    tx_ready <= 1'b1;
                    if (tx_valid && tx_ready) begin
                        tx_byte    <= tx_data;
                        tx_ready   <= 1'b0;
                        tx_clk_cnt <= 0;
                        tx_state   <= TX_START;
                    end
                end

                TX_START: begin
                    uart_tx <= 1'b0;
                    if (tx_clk_cnt == (CLKS_PER_BIT - 1)) begin
                        tx_clk_cnt <= 0;
                        tx_bit_idx <= 0;
                        tx_state   <= TX_DATA;
                    end else begin
                        tx_clk_cnt <= tx_clk_cnt + 1'b1;
                    end
                end

                TX_DATA: begin
                    uart_tx <= tx_byte[tx_bit_idx];
                    if (tx_clk_cnt == (CLKS_PER_BIT - 1)) begin
                        tx_clk_cnt <= 0;
                        if (tx_bit_idx == 3'd7) begin
                            tx_state <= TX_STOP;
                        end else begin
                            tx_bit_idx <= tx_bit_idx + 1'b1;
                        end
                    end else begin
                        tx_clk_cnt <= tx_clk_cnt + 1'b1;
                    end
                end

                TX_STOP: begin
                    uart_tx <= 1'b1;
                    if (tx_clk_cnt == (CLKS_PER_BIT - 1)) begin
                        tx_clk_cnt <= 0;
                        tx_ready   <= 1'b1;
                        tx_state   <= TX_IDLE;
                    end else begin
                        tx_clk_cnt <= tx_clk_cnt + 1'b1;
                    end
                end

                default: tx_state <= TX_IDLE;
            endcase
        end
    end

endmodule
