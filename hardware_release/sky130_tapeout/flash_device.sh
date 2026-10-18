#!/usr/bin/env bash
set -e
echo "============================================================"
echo "  CRON FPGA 1-CLICK PHYSICAL HARDWARE FLASH PROGRAMMER"
echo "  Target: OpenLane SkyWater 130nm ASIC Tapeout (Open-Source Silicon)"
echo "============================================================"

if command -v openFPGALoader >/dev/null 2>&1; then
    echo "[INFO] Detected openFPGALoader in PATH."
    echo "[FLASHING] Executing: openFPGALoader cron_top.bit"
    openFPGALoader cron_top.bit
elif command -v iceprog >/dev/null 2>&1; then
    echo "[INFO] Detected iceprog in PATH."
    echo "[FLASHING] Executing: iceprog cron_top.bin"
    iceprog cron_top.bin
elif command -v ecpprog >/dev/null 2>&1; then
    echo "[INFO] Detected ecpprog in PATH."
    echo "[FLASHING] Executing: ecpprog cron_top.bit"
    ecpprog cron_top.bit
else
    echo "[NOTICE] No physical FPGA programmer (openFPGALoader, iceprog, ecpprog) was detected in PATH."
    echo "[NOTICE] On Linux/macOS, install via: sudo apt-get install openfpgaloader OR brew install openfpgaloader"
    echo "[NOTICE] Manual flash command: openFPGALoader cron_top.bit"
fi
echo "[SUCCESS] Flash script finished."
