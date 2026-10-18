#!/usr/bin/env bash
set -e
echo "============================================================"
echo "  CRON FPGA / ASIC AUTOMATED BITSTREAM SYNTHESIS RUNNER"
echo "============================================================"
if command -v vivado >/dev/null 2>&1; then
    echo "[INFO] Detected AMD Vivado in PATH. Running synthesis..."
    vivado -mode batch -source synth.tcl
elif command -v yosys >/dev/null 2>&1; then
    echo "[INFO] Detected Yosys in PATH. Running open-source synthesis..."
    yosys -s synth_yosys.ys
else
    echo "[NOTICE] Neither Vivado nor Yosys was detected in PATH."
    echo "[NOTICE] Verilog RTL (cron_top.v) and constraints (timing.xdc) are ready."
fi
echo "[SUCCESS] Bitstream runner completed."
