# ============================================================================
# CRON 256-Core 4D-Torus Neuromorphic Photonic Processor Constraints (XDC)
# Target Architecture: AMD Xilinx UltraScale+ (XCU250) / Versal AI Core
# Operating Frequency: 1.2 GHz (Period: 0.833 ns)
# ============================================================================

# 1. Primary System Clock (1.2 GHz)
create_clock -period 0.833 -name sys_clk -waveform {0.000 0.416} [get_ports clk]

# 2. Clock Uncertainty and Jitter Margins
set_clock_uncertainty 0.050 [get_clocks sys_clk]

# 3. Input & Output Timing Constraints (Matched to Photonic Transceiver Interface)
set_input_delay -clock sys_clk -max 0.200 [get_ports rst_n]
set_input_delay -clock sys_clk -min 0.050 [get_ports rst_n]
set_input_delay -clock sys_clk -max 0.200 [get_ports enable]
set_input_delay -clock sys_clk -min 0.050 [get_ports enable]

set_output_delay -clock sys_clk -max 0.250 [get_ports halted]
set_output_delay -clock sys_clk -min 0.050 [get_ports halted]
set_output_delay -clock sys_clk -max 0.250 [get_ports sentry_alert]
set_output_delay -clock sys_clk -min 0.050 [get_ports sentry_alert]

# 4. Multi-Cycle Path Constraints (Optical GEMM Phase Settling: 2 cycles relaxed)
set_multicycle_path -setup 2 -from [get_cells -hierarchical -filter {NAME =~ *optical_accum*}]
set_multicycle_path -hold 1  -from [get_cells -hierarchical -filter {NAME =~ *optical_accum*}]

# 5. False Paths (Asynchronous Hardware Reset Deassertion)
set_false_path -from [get_ports rst_n] -to [get_pins -hierarchical -filter {NAME =~ *CLR || NAME =~ *PRE}]

# 6. Physical I/O Standards & Drive Strengths
set_property IOSTANDARD LVCMOS18 [get_ports clk]
set_property IOSTANDARD LVCMOS18 [get_ports rst_n]
set_property IOSTANDARD LVCMOS18 [get_ports enable]
set_property IOSTANDARD LVCMOS18 [get_ports halted]
set_property IOSTANDARD LVCMOS18 [get_ports sentry_alert]

# 7. High-Fanout Reset Buffer Optimization
set_property BUFFER_TYPE BUFG [get_ports clk]
