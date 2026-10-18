@echo off
echo ============================================================
echo   CRON FPGA 1-CLICK PHYSICAL HARDWARE FLASH PROGRAMMER
echo   Target: OpenLane SkyWater 130nm ASIC Tapeout (Open-Source Silicon)
echo ============================================================

where openFPGALoader >nul 2>nul
if %ERRORLEVEL% equ 0 (
    echo [INFO] Detected openFPGALoader in PATH.
    echo [FLASHING] Executing: openFPGALoader cron_top.bit
    openFPGALoader cron_top.bit
    goto :flash_done
)

where iceprog >nul 2>nul
if %ERRORLEVEL% equ 0 (
    echo [INFO] Detected iceprog in PATH.
    echo [FLASHING] Executing: iceprog cron_top.bin
    iceprog cron_top.bin
    goto :flash_done
)

where ecpprog >nul 2>nul
if %ERRORLEVEL% equ 0 (
    echo [INFO] Detected ecpprog in PATH.
    echo [FLASHING] Executing: ecpprog cron_top.bit
    ecpprog cron_top.bit
    goto :flash_done
)

echo [NOTICE] No physical FPGA programmer (openFPGALoader, iceprog, ecpprog) was detected in PATH.
echo [NOTICE] To install openFPGALoader on Windows:
echo [NOTICE]   winget install openFPGALoader
echo [NOTICE] Manual flash command: openFPGALoader cron_top.bit

:flash_done
echo [SUCCESS] Flash script finished.
