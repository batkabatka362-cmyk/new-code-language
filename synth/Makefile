# ============================================================================
# CRON Hardware Toolchain — Synthesis Makefile
# ============================================================================

.PHONY: all vivado yosys clean help

all: help

help:
	@echo "CRON Hardware Synthesis Automation"
	@echo "Targets:"
	@echo "  make vivado  - Run AMD Xilinx UltraScale+ FPGA batch synthesis flow"
	@echo "  make yosys   - Run Open-source Yosys + ABC gate-level mapping flow"
	@echo "  make clean   - Clean synthesis artifacts and reports"

vivado:
	vivado -mode batch -nojournal -nolog -source vivado_synth.tcl

yosys:
	yosys -s yosys_synth.tcl

clean:
	rm -rf output_vivado output_yosys *.log *.jou .Xil
