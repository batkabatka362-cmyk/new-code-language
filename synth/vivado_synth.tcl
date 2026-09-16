# ============================================================================
# CRON Hardware Toolchain — Vivado Batch Synthesis Script (Non-Project Mode)
# Target: AMD Xilinx UltraScale+ FPGA (xcu250-figd2104-2L-e) / Versal AI Core
# Generates: Gate-level netlist, timing reports (WNS/TNS), and resource utilization
# ============================================================================

set SCRIPT_DIR [file dirname [file normalize [info script]]]
set PROJ_ROOT  [file normalize "$SCRIPT_DIR/.."]
set OUTPUT_DIR "$SCRIPT_DIR/output_vivado"

file mkdir $OUTPUT_DIR

puts "======================================================================"
puts "  CRON Neuromorphic Core Synthesis Flow (AMD Xilinx Vivado)"
puts "  Target Part: xcu250-figd2104-2L-e"
puts "  Frequency  : 1.2 GHz"
puts "======================================================================"

# 1. Set Device Target
set_part xcu250-figd2104-2L-e

# 2. Read Synthesized Verilog RTL and RTL Library Modules
puts "--> Reading Verilog RTL sources..."
if {[file exists "$OUTPUT_DIR/cksl_core.v"]} {
    read_verilog "$OUTPUT_DIR/cksl_core.v"
} elseif {[file exists "$PROJ_ROOT/cksl_core.v"]} {
    read_verilog "$PROJ_ROOT/cksl_core.v"
}

# Read all RTL library modules
read_verilog [glob -nocomplain "$PROJ_ROOT/rtl/*.v"]

# 3. Read XDC Timing Constraints
puts "--> Reading Physical & Timing Constraints..."
read_xdc "$SCRIPT_DIR/cron_core_constraints.xdc"

# 4. Run RTL Synthesis
puts "--> Running Logic Synthesis (-flatten_hierarchy rebuilt)..."
synth_design -top cksl_core -part xcu250-figd2104-2L-e \
    -flatten_hierarchy rebuilt \
    -mode out_of_context \
    -retiming

# Write post-synthesis checkpoints and reports
write_checkpoint -force "$OUTPUT_DIR/post_synth.dcp"
report_utilization -file "$OUTPUT_DIR/post_synth_utilization.rpt"
report_timing_summary -file "$OUTPUT_DIR/post_synth_timing.rpt"

# 5. Logic Optimization (Opt Design)
puts "--> Running Logic Optimization (opt_design)..."
opt_design -directive Explore

# 6. Placement & Physical Optimization
puts "--> Placing Design (place_design)..."
place_design -directive Explore
phys_opt_design -directive Explore

write_checkpoint -force "$OUTPUT_DIR/post_place.dcp"
report_timing_summary -file "$OUTPUT_DIR/post_place_timing.rpt"

# 7. Routing (Route Design)
puts "--> Routing Design (route_design)..."
route_design -directive Explore
phys_opt_design -directive Explore

write_checkpoint -force "$OUTPUT_DIR/post_route.dcp"

# 8. Generate Final Verification Reports
puts "--> Generating Final Sign-Off Reports..."
report_timing_summary -delay_type min_max -report_unconstrained -check_timing_verbose -file "$OUTPUT_DIR/final_timing_summary.rpt"
report_utilization -hierarchical -file "$OUTPUT_DIR/final_utilization_hierarchical.rpt"
report_power -file "$OUTPUT_DIR/final_power_estimation.rpt"
report_drc -file "$OUTPUT_DIR/final_drc_checks.rpt"

# Write final EDIF / Verilog netlist
write_verilog -force -mode timesim "$OUTPUT_DIR/cksl_core_netlist.v"

puts "======================================================================"
puts "  SYNTHESIS AND IMPLEMENTATION COMPLETE!"
puts "  Outputs saved to: $OUTPUT_DIR"
puts "======================================================================"
