# Clock Timing Constraints for attention
create_clock -period 1.000 -name sys_clk [get_ports clk]
set_input_delay -clock sys_clk 0.200 [all_inputs -no_clocks]
set_output_delay -clock sys_clk 0.200 [all_outputs]
