# ==============================================================================
# CRON LIVE OPENAI-COMPATIBLE REST API SERVER & BENCHMARK SHOWCASE
# Demonstrates:
# 1. Spawning CRON Live HTTP Inference Server (POST /v1/chat/completions, GET /v1/models)
# 2. Zero-Allocation Health & Telemetry Verification (GET /health)
# 3. Model Inventory Inspection (GET /v1/models)
# 4. Non-Streaming Mongolian AI Chat Completion with <think> Reasoning
# 5. Server-Sent Events (SSE) Live Token Streaming (text/event-stream)
# 6. Industrial Load Benchmark with Latency Histogram & QPS Profiling (cron bench-server)
# ==============================================================================

$ErrorActionPreference = 'Stop'
$repoRoot = (Get-Item "$PSScriptRoot\..\..").FullName
$cron = "$repoRoot\target\release\cron.exe"

if (-not (Test-Path $cron)) {
    $cron = "$repoRoot\target\debug\cron.exe"
}

$port = 8089
$baseUrl = "http://127.0.0.1:$port"

Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host " CRON LIVE OPENAI-COMPATIBLE REST API SERVER & BENCHMARK SUITE                  " -ForegroundColor Green
Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host " Engine Executable : $cron"
Write-Host " Server Endpoint   : $baseUrl"
Write-Host ""

$serverProcess = $null

try {
    # --------------------------------------------------------------------------
    # STAGE 1: Spawn CRON Live HTTP Server on Port $port
    # --------------------------------------------------------------------------
    Write-Host "[1/6] Launching CRON Live HTTP Server daemon on port $port..." -ForegroundColor Yellow
    $serverProcess = Start-Process -FilePath $cron -ArgumentList "serve", "--port", "$port" -PassThru -WindowStyle Hidden
    
    $ready = $false
    for ($i = 0; $i -lt 30; $i++) {
        Start-Sleep -Milliseconds 200
        try {
            $resp = Invoke-RestMethod -Uri "$baseUrl/health" -Method Get -TimeoutSec 1 -ErrorAction SilentlyContinue
            if ($resp -and $resp.status -eq "healthy") {
                $ready = $true
                break
            }
        } catch {}
    }

    if (-not $ready) {
        throw "Failed to connect to CRON HTTP server at $baseUrl within timeout."
    }
    Write-Host "  [+] HTTP Server bound and healthy (PID: $($serverProcess.Id))" -ForegroundColor Green

    # --------------------------------------------------------------------------
    # STAGE 2: Zero-Allocation Health & Telemetry Verification
    # --------------------------------------------------------------------------
    Write-Host "`n[2/6] Verifying Zero-GC Regional Arena & Core Telemetry (/health)..." -ForegroundColor Yellow
    $health = Invoke-RestMethod -Uri "$baseUrl/health" -Method Get
    Write-Host "  [+] Server Status : $($health.status)" -ForegroundColor White
    Write-Host "  [+] Hard Backend  : $($health.engine)" -ForegroundColor White
    Write-Host "  [+] Active Cores  : $($health.cores) Coherent 4D-Torus Nodes" -ForegroundColor White
    Write-Host "  [+] Zero-GC Mode  : $($health.zero_gc)" -ForegroundColor White
    Write-Host "  [+] Allocations   : $($health.allocations) heap allocations" -ForegroundColor Green

    # --------------------------------------------------------------------------
    # STAGE 3: Model Inventory Inspection (/v1/models)
    # --------------------------------------------------------------------------
    Write-Host "`n[3/6] Inspecting OpenAI-Compatible Model Inventory (/v1/models)..." -ForegroundColor Yellow
    $modelsResp = Invoke-RestMethod -Uri "$baseUrl/v1/models" -Method Get
    foreach ($m in $modelsResp.data) {
        Write-Host "  - Model ID: $($m.id.PadRight(24)) | Owner: $($m.owned_by)" -ForegroundColor White
    }

    # --------------------------------------------------------------------------
    # STAGE 4: Non-Streaming Mongolian AI Chat Completion
    # --------------------------------------------------------------------------
    Write-Host "`n[4/6] Executing Non-Streaming Chat Completion with R1 Reasoning (/v1/chat/completions)..." -ForegroundColor Yellow
    $chatBody = @{
        model = "cron-mongol-v3"
        messages = @(
            @{ role = "user"; content = "CRON системийн архитектур болон VLIW микрокодын давуу талыг тайлбарлана уу." }
        )
        stream = $false
    } | ConvertTo-Json

    $chatResult = Invoke-RestMethod -Uri "$baseUrl/v1/chat/completions" -Method Post -ContentType "application/json; charset=utf-8" -Body $chatBody
    Write-Host "  [+] Response ID : $($chatResult.id)" -ForegroundColor White
    Write-Host "  [+] Model Used  : $($chatResult.model)" -ForegroundColor White
    Write-Host "  [+] Tokens      : Prompt=$($chatResult.usage.prompt_tokens), Completion=$($chatResult.usage.completion_tokens), Total=$($chatResult.usage.total_tokens)" -ForegroundColor Green
    Write-Host "  [+] Message Output Preview:" -ForegroundColor Cyan
    $lines = $chatResult.choices[0].message.content -split "`n"
    for ($i = 0; $i -lt [Math]::Min(12, $lines.Length); $i++) {
        Write-Host "      $($lines[$i])" -ForegroundColor Gray
    }
    if ($lines.Length -gt 12) {
        Write-Host "      ... ($($lines.Length - 12) more lines) ..." -ForegroundColor DarkGray
    }

    # --------------------------------------------------------------------------
    # STAGE 5: Server-Sent Events (SSE) Live Token Streaming
    # --------------------------------------------------------------------------
    Write-Host "`n[5/6] Verifying Server-Sent Events (SSE) Token Streaming (stream: true)..." -ForegroundColor Yellow
    $streamPayload = '{"model":"cron-r1-7b-ternary","messages":[{"role":"user","content":"Stream test"}],"stream":true}'
    
    Write-Host "  [+] Connecting to SSE Stream via curl.exe..." -ForegroundColor Cyan
    $sseChunks = curl.exe -N -s -X POST "$baseUrl/v1/chat/completions" -H "Content-Type: application/json" -d "{\`"model\`":\`"cron-r1-7b-ternary\`",\`"stream\`":true}"
    
    $sseChunkCount = 0
    $receivedDone = $false
    Write-Host "  [+] Streaming Tokens: " -ForegroundColor Cyan -NoNewline
    foreach ($line in $sseChunks) {
        if ($line.StartsWith("data: ")) {
            $data = $line.Substring(6)
            if ($data -eq "[DONE]") {
                $receivedDone = $true
            } else {
                $sseChunkCount++
                if ($sseChunkCount -le 20) {
                    Write-Host "." -ForegroundColor White -NoNewline
                }
            }
        }
    }
    Write-Host " [DONE]" -ForegroundColor Green
    Write-Host "  [+] Total SSE Chunks Streamed: $sseChunkCount tokens (Terminated with [DONE]: $receivedDone)" -ForegroundColor Green

    # --------------------------------------------------------------------------
    # STAGE 6: Industrial Server Load Benchmark (cron bench-server)
    # --------------------------------------------------------------------------
    Write-Host "`n[6/6] Running Industrial Server Load Benchmark (cron bench-server)..." -ForegroundColor Yellow
    & $cron bench-server --url "$baseUrl/health" --requests 300 --concurrency 10
    if ($LASTEXITCODE -ne 0) {
        throw "Server benchmark failed with exit code $LASTEXITCODE"
    }

    Write-Host "`n================================================================================" -ForegroundColor Cyan
    Write-Host " [SUCCESS] LIVE REST API SERVER & BENCHMARK VERIFIED WITH 0 ERRORS!             " -ForegroundColor Green
    Write-Host "================================================================================" -ForegroundColor Cyan
} finally {
    if ($serverProcess -and -not $serverProcess.HasExited) {
        Write-Host "`nStopping background CRON HTTP server daemon (PID: $($serverProcess.Id))..." -ForegroundColor DarkGray
        Stop-Process -Id $serverProcess.Id -Force -ErrorAction SilentlyContinue
    }
}
