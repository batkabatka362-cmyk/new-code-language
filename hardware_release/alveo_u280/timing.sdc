# ==============================================================================
# Synopsys Design Constraints (SDC) for ASIC (OpenLane Sky130) & Quartus
# ==============================================================================
create_clock -period 1.000 -name clk [get_ports clk]
set_clock_uncertainty 0.05 [get_clocks clk]
set_input_delay -max 0.20 -clock clk [all_inputs]
set_output_delay -max 0.20 -clock clk [all_outputs]
