# ==============================================================================
# CRON 4D-Torus Clock & Timing Constraints (IEEE 1364-2001)
# ==============================================================================

# Primary VLIW Execution Core Clock (1000 MHz)
create_clock -period 1.000 -name clk_core [get_ports clk_core]

# 4D-Torus Dimension-Order Routing NoC Clock (500 MHz)
create_clock -period 2.000 -name clk_noc [get_ports clk_noc]

# Asynchronous Clock Domain Crossing (CDC) False Paths between Core and NoC
set_clock_groups -asynchronous -group [get_clocks clk_core] -group [get_clocks clk_noc]

# PCIe Gen5 x16 Reference Clock (100.0 MHz)
create_clock -period 10.000 -name pcie_ref_clk [get_ports pcie_refclk_p]

# Input / Output Delays for HBM2 & Photonic Transceivers
set_input_delay -clock clk_core -max 0.25 [get_ports {ext_data_in*}]
set_output_delay -clock clk_core -max 0.25 [get_ports {ext_data_out*}]
