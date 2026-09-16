# ============================================================================
# CRON Hardware Toolchain — Open-Source Yosys Synthesis Script
# Targets: Lattice ECP5 FPGA (Open FPGA Flow) or ASIC Standard Cell Mapping
# ============================================================================

yosys -import

set SCRIPT_DIR [file dirname [file normalize [info script]]]
set PROJ_ROOT  [file normalize "$SCRIPT_DIR/.."]
set OUTPUT_DIR "$SCRIPT_DIR/output_yosys"

file mkdir $OUTPUT_DIR

puts "======================================================================"
puts "  CRON Open-Source Synthesis Flow (Yosys)"
puts "  Target Architecture: 256-Core Neuromorphic Processor Core"
puts "======================================================================"

# 1. Read Verilog sources
if {[file exists "$OUTPUT_DIR/cksl_core.v"]} {
    read_verilog -sv "$OUTPUT_DIR/cksl_core.v"
} elseif {[file exists "$PROJ_ROOT/cksl_core.v"]} {
    read_verilog -sv "$PROJ_ROOT/cksl_core.v"
}

# Read RTL library files
foreach f [glob -nocomplain "$PROJ_ROOT/rtl/*.v"] {
    read_verilog -sv $f
}

# 2. Elaborate hierarchy
hierarchy -check -top cksl_core

# 3. High-level synthesis optimizations
proc
opt
fsm
opt
memory
opt

# 4. Target Synthesis Mapping (Generic or ECP5)
synth -top cksl_core -flatten

# 5. Technology Mapping
dfflegalize -cell $_DFF_P_ 01
abc -g cmos2
opt_clean -purge

# 6. Generate Reports & Netlist
tee -o "$OUTPUT_DIR/yosys_stats.txt" stat
write_verilog "$OUTPUT_DIR/cksl_core_synth.v"
write_json "$OUTPUT_DIR/cksl_core_netlist.json"

puts "======================================================================"
puts "  YOSYS SYNTHESIS COMPLETE!"
puts "  Netlist: $OUTPUT_DIR/cksl_core_synth.v"
puts "======================================================================"
