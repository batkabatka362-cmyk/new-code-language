# ============================================================================
# CRON Hardware Toolchain — Synthesis Makefile
# ============================================================================

.PHONY: all vivado yosys verilator clean help

all: help

help:
	@echo "CRON Hardware Synthesis Automation"
	@echo "Targets:"
	@echo "  make verilator - Run Verilator cycle-accurate C++ co-simulation"
	@echo "  make vivado    - Run AMD Xilinx UltraScale+ FPGA batch synthesis flow"
	@echo "  make yosys     - Run Open-source Yosys + ABC gate-level mapping flow"
	@echo "  make clean     - Clean synthesis artifacts and reports"

verilator:
	verilator --cc --exe --build -j 4 cksl_core.v verilator_testbench.cpp --trace -o sim_cksl_core
	./obj_dir/sim_cksl_core

vivado:
	vivado -mode batch -nojournal -nolog -source vivado_synth.tcl

yosys:
	yosys -s yosys_synth.tcl

clean:
	rm -rf output_vivado output_yosys obj_dir *.vcd *.log *.jou .Xil
