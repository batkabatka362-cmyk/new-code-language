# ==============================================================================
# CRON Master Ecosystem Regression Audit (PowerShell)
# ==============================================================================
$ErrorActionPreference = "Stop"

Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host "  CRON SOVEREIGN ECOSYSTEM: MASTER REGRESSION AUDIT (PHASES D1 - D5)" -ForegroundColor Cyan
Write-Host "================================================================================" -ForegroundColor Cyan

python scripts/audit_ecosystem.py
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Ecosystem audit failed." -ForegroundColor Red
    exit $LASTEXITCODE
}

