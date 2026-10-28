# Forwarder script for backward compatibility
$script = Join-Path $PSScriptRoot "ai\run_chat_and_sleep_daemon.ps1"
& $script @args
exit $LASTEXITCODE
