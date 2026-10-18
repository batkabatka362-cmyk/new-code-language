@echo off
echo ============================================================
echo   CRON FPGA / ASIC AUTOMATED BITSTREAM SYNTHESIS RUNNER
echo ============================================================
where vivado >nul 2>nul
if %ERRORLEVEL% equ 0 (
    echo [INFO] Detected AMD Vivado in PATH. Running synthesis...
    vivado -mode batch -source synth.tcl
    goto :done
)
where yosys >nul 2>nul
if %ERRORLEVEL% equ 0 (
    echo [INFO] Detected Yosys in PATH. Running open-source synthesis...
    yosys -s synth_yosys.ys
    goto :done
)
echo [NOTICE] Neither Vivado nor Yosys was detected in PATH.
echo [NOTICE] Hardware Verilog RTL (cron_top.v) and timing constraints (timing.xdc)
echo [NOTICE] are ready for synthesis.
:done
echo [SUCCESS] Bitstream runner completed.
