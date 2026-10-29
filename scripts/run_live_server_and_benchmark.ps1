# Forwarder script for backward compatibility
$script = Join-Path $PSScriptRoot "ai\run_live_server_and_benchmark.ps1"
& $script @args
exit $LASTEXITCODE

