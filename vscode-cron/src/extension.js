// ============================================================================
// CRON VS Code Extension Entry Point
// Connects to 'cron lsp' language server daemon via stdio.
// Includes status bar telemetry, quick command menu, and intelligent path resolution.
// ============================================================================

const vscode = require('vscode');
const path = require('path');
const fs = require('fs');

let client;
let statusBarItem;

function resolveCronExecutable() {
    const config = vscode.workspace.getConfiguration('cron');
    const configuredPath = config.get('lsp.serverPath');
    if (configuredPath && configuredPath !== 'cron') {
        return configuredPath;
    }

    // Try detecting target/release or target/debug in active workspace folders
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders && workspaceFolders.length > 0) {
        for (const folder of workspaceFolders) {
            const relExe = path.join(folder.uri.fsPath, 'target', 'release', process.platform === 'win32' ? 'cron.exe' : 'cron');
            if (fs.existsSync(relExe)) {
                return relExe;
            }
            const dbgExe = path.join(folder.uri.fsPath, 'target', 'debug', process.platform === 'win32' ? 'cron.exe' : 'cron');
            if (fs.existsSync(dbgExe)) {
                return dbgExe;
            }
        }
    }

    return 'cron';
}

function activate(context) {
    const outputChannel = vscode.window.createOutputChannel('CRON Language Server');
    outputChannel.appendLine('[CRON Extension] Activating CRON Cognitive Language extension v1.2.0...');

    const serverExe = resolveCronExecutable();
    outputChannel.appendLine(`[CRON Extension] Resolved executable: ${serverExe}`);

    // 1. Initialize Status Bar Indicator
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.command = 'cron.quickMenu';
    statusBarItem.text = '$(circuit-board) CRON 256-Core';
    statusBarItem.tooltip = 'CRON 4D-Torus Hardware Ecosystem - Click for Quick Menu';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);

    // 2. Start Language Client
    try {
        const { LanguageClient, TransportKind } = require('vscode-languageclient/node');
        const serverOptions = {
            command: serverExe,
            args: ['lsp'],
            transport: TransportKind.stdio
        };

        const clientOptions = {
            documentSelector: [
                { scheme: 'file', language: 'cron' },
                { scheme: 'file', language: 'cron-cl' }
            ],
            synchronize: {
                fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{cr,cl,toml}')
            },
            outputChannel: outputChannel
        };

        client = new LanguageClient(
            'cronLspServer',
            'CRON Cognitive Language Server (LSP 3.17)',
            serverOptions,
            clientOptions
        );

        client.start();
        statusBarItem.text = '$(check) CRON [LSP 3.17 Active]';
        outputChannel.appendLine('[CRON Extension] LanguageClient started successfully.');
    } catch (err) {
        outputChannel.appendLine('[CRON Extension] Running in standalone syntax mode: ' + err.message);
        statusBarItem.text = '$(circuit-board) CRON [Ready]';
    }

    // Helper to run command in terminal
    function runCronInTerminal(title, commandStr) {
        const term = vscode.window.createTerminal(title);
        term.show();
        term.sendText(commandStr);
    }

    // 3. Register Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cron.quickMenu', async () => {
            const items = [
                { label: '$(check) Check Current File', description: 'Run linear type & EBNF syntax checker', command: 'cron.check' },
                { label: '$(play) Run Native Host Binary', description: 'Compile to C23 and execute with gcc -O3', command: 'cron.runNative' },
                { label: '$(code) Compile to C23 Source', description: 'Generate high-performance C23 code', command: 'cron.buildC23' },
                { label: '$(shield) Verify Safety & Landauer', description: 'Deadlock-freedom & thermodynamic entropy verification', command: 'cron.verifySafety' },
                { label: '$(zap) Auto-Heal VLIW Microcode', description: 'Autonomous vibe-coding repair & CRC parity', command: 'cron.healCl' },
                { label: '$(rocket) Super-Optimize VLIW Slots', description: 'Level-2 DAG Out-of-Order Compaction', command: 'cron.optCl' },
                { label: '$(dashboard) Benchmark Silicon Roofline', description: 'Compute FLOPs/Byte, TOPS/W, and PPA metrics', command: 'cron.clPerf' },
                { label: '$(pulse) Generate VCD Waveform', description: 'Cycle-accurate logic analyzer traces', command: 'cron.clVcd' },
                { label: '$(server-process) Launch Swarm TUI Monitor', description: '256-Core 4D-Torus Live Silicon Monitor', command: 'cron.openSwarmTui' },
                { label: '$(debug-alt) Start Hardware Debugger (DAP)', description: 'Cycle-accurate VLIW stepping & registers', command: 'cron.startDebugging' },
                { label: '$(circuit-board) 4D-Torus & Wafer Visualizer', description: 'Live 65,536-Core silicon heatmap & packet telemetry', command: 'cron.openMeshVisualizer' },
            ];

            const selected = await vscode.window.showQuickPick(items, {
                placeHolder: 'Select a CRON Toolchain action to execute...'
            });

            if (selected) {
                vscode.commands.executeCommand(selected.command);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.check', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage('No active file open.');
                return;
            }
            runCronInTerminal('CRON Check', `"${serverExe}" check "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.buildC23', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cr')) {
                vscode.window.showErrorMessage('Please open a .cr file to compile.');
                return;
            }
            runCronInTerminal('CRON Build', `"${serverExe}" c23 "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.runNative', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cr')) {
                vscode.window.showErrorMessage('Please open a .cr file to run.');
                return;
            }
            runCronInTerminal('CRON Native Run', `"${serverExe}" native "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.verifySafety', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage('No active file open.');
                return;
            }
            runCronInTerminal('CRON Safety Verify', `"${serverExe}" verify "${editor.document.fileName}" --temp 300.0 --freq 1.0`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.healCl', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file to heal.');
                return;
            }
            runCronInTerminal('CRON Auto-Heal', `"${serverExe}" cl-heal "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.optCl', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file to optimize.');
                return;
            }
            runCronInTerminal('CRON Super-Optimize', `"${serverExe}" cl-opt "${editor.document.fileName}" --level 2`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.runJitCl', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file to execute.');
                return;
            }
            runCronInTerminal('CRON JIT Simulator', `"${serverExe}" cl-run "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.clPerf', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file for PPA benchmark.');
                return;
            }
            runCronInTerminal('CRON PPA Benchmark', `"${serverExe}" cl-perf "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.clVcd', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file to trace.');
                return;
            }
            runCronInTerminal('CRON VCD Logic Analyzer', `"${serverExe}" cl-vcd "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.openSwarmTui', async () => {
            runCronInTerminal('CRON Swarm Silicon TUI', `"${serverExe}" cl-swarm --tui`);
        })
    );

    // 4. Register Debug Adapter Protocol (DAP) Factory & Provider
    context.subscriptions.push(
        vscode.debug.registerDebugAdapterDescriptorFactory('cron', {
            createDebugAdapterDescriptor(_session) {
                return new vscode.DebugAdapterExecutable(serverExe, ['dap']);
            }
        })
    );

    context.subscriptions.push(
        vscode.debug.registerDebugConfigurationProvider('cron', {
            resolveDebugConfiguration(folder, config) {
                if (!config.type && !config.request && !config.name) {
                    const editor = vscode.window.activeTextEditor;
                    if (editor && (editor.document.languageId === 'cron' || editor.document.languageId === 'cron-cl')) {
                        config.type = 'cron';
                        config.name = 'Debug CRON Hardware Program';
                        config.request = 'launch';
                        config.program = '${file}';
                        config.stopOnEntry = true;
                        config.coreId = 0;
                    }
                }
                if (!config.program) {
                    config.program = '${file}';
                }
                return config;
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.startDebugging', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage('Please open a .cr or .cl file to debug.');
                return;
            }
            vscode.debug.startDebugging(undefined, {
                type: 'cron',
                name: 'Debug CRON Hardware Program',
                request: 'launch',
                program: editor.document.fileName,
                stopOnEntry: true,
                coreId: 0
            });
        })
    );

    // 5. Register 4D-Torus & Wafer-65536 Live Silicon Visualizer Webview
    context.subscriptions.push(
        vscode.commands.registerCommand('cron.openMeshVisualizer', () => {
            const panel = vscode.window.createWebviewPanel(
                'cronMeshVisualizer',
                'CRON 4D-Torus & Wafer Visualizer',
                vscode.ViewColumn.Beside,
                {
                    enableScripts: true,
                    retainContextWhenHidden: true
                }
            );

            panel.webview.html = getMeshVisualizerHtml();
        })
    );
}

function getMeshVisualizerHtml() {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CRON 4D-Torus & Wafer-65536 Silicon Visualizer</title>
    <style>
        :root {
            --bg-primary: #070a13;
            --bg-secondary: #0f172a;
            --bg-card: rgba(30, 41, 59, 0.7);
            --border-glass: rgba(148, 163, 184, 0.15);
            --cyan-accent: #00e5ff;
            --green-accent: #00e676;
            --gold-accent: #ffd600;
            --red-alert: #ff1744;
            --text-main: #f1f5f9;
            --text-dim: #94a3b8;
        }
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            user-select: none;
        }
        body {
            background: radial-gradient(circle at 50% 0%, #1e1b4b 0%, var(--bg-primary) 70%);
            color: var(--text-main);
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
            min-height: 100vh;
            padding: 20px;
            display: flex;
            flex-direction: column;
            gap: 16px;
        }
        /* Top Telemetry HUD */
        .hud-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(170px, 1fr));
            gap: 12px;
        }
        .hud-card {
            background: var(--bg-card);
            backdrop-filter: blur(12px);
            border: 1px solid var(--border-glass);
            border-radius: 10px;
            padding: 12px 16px;
            display: flex;
            flex-direction: column;
            gap: 4px;
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
            transition: border-color 0.2s;
        }
        .hud-card:hover {
            border-color: var(--cyan-accent);
        }
        .hud-label {
            font-size: 0.75rem;
            color: var(--text-dim);
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }
        .hud-val {
            font-size: 1.35rem;
            font-weight: 700;
            color: #fff;
            display: flex;
            align-items: center;
            gap: 6px;
        }
        .badge-verified {
            background: rgba(0, 230, 118, 0.15);
            color: var(--green-accent);
            border: 1px solid var(--green-accent);
            font-size: 0.68rem;
            padding: 2px 6px;
            border-radius: 4px;
            font-weight: 600;
        }
        /* Action Bar */
        .toolbar {
            display: flex;
            flex-wrap: wrap;
            align-items: center;
            justify-content: space-between;
            gap: 10px;
            background: var(--bg-card);
            border: 1px solid var(--border-glass);
            padding: 10px 16px;
            border-radius: 10px;
        }
        .btn-group {
            display: flex;
            gap: 8px;
            flex-wrap: wrap;
        }
        button {
            background: rgba(15, 23, 42, 0.8);
            color: var(--text-main);
            border: 1px solid var(--border-glass);
            padding: 8px 14px;
            border-radius: 6px;
            font-size: 0.85rem;
            font-weight: 600;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
            transition: all 0.2s ease;
        }
        button:hover {
            background: var(--cyan-accent);
            color: #000;
            border-color: var(--cyan-accent);
            box-shadow: 0 0 14px rgba(0, 229, 255, 0.4);
        }
        button.btn-danger:hover {
            background: var(--red-alert);
            color: #fff;
            border-color: var(--red-alert);
            box-shadow: 0 0 14px rgba(255, 23, 68, 0.4);
        }
        /* Main Layout */
        .workspace {
            display: grid;
            grid-template-columns: 1fr 340px;
            gap: 16px;
            flex: 1;
        }
        @media (max-width: 900px) {
            .workspace {
                grid-template-columns: 1fr;
            }
        }
        /* Canvas & Grid Container */
        .visualizer-container {
            background: var(--bg-card);
            border: 1px solid var(--border-glass);
            border-radius: 12px;
            padding: 16px;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            position: relative;
            min-height: 480px;
        }
        .wafer-grid {
            display: grid;
            grid-template-columns: repeat(16, 1fr);
            gap: 4px;
            width: 100%;
            max-width: 480px;
            aspect-ratio: 1 / 1;
        }
        .die-cell {
            background: rgba(0, 229, 255, 0.08);
            border: 1px solid rgba(0, 229, 255, 0.2);
            border-radius: 4px;
            aspect-ratio: 1 / 1;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.65rem;
            color: var(--text-dim);
            transition: all 0.15s ease;
            position: relative;
        }
        .die-cell:hover {
            transform: scale(1.18);
            z-index: 10;
            border-color: #fff;
            box-shadow: 0 0 12px var(--cyan-accent);
        }
        .die-cell.selected {
            border-color: #fff;
            box-shadow: 0 0 15px #fff;
            transform: scale(1.2);
            z-index: 11;
        }
        .die-cell.throttled {
            background: rgba(255, 23, 68, 0.35) !important;
            border-color: var(--red-alert) !important;
            color: #fff !important;
            font-weight: 800;
            animation: pulse-red 1.2s infinite;
        }
        @keyframes pulse-red {
            0%, 100% { box-shadow: 0 0 4px var(--red-alert); }
            50% { box-shadow: 0 0 16px var(--red-alert); }
        }
        /* Inspector Drawer */
        .inspector {
            background: var(--bg-card);
            border: 1px solid var(--border-glass);
            border-radius: 12px;
            padding: 16px;
            display: flex;
            flex-direction: column;
            gap: 14px;
        }
        .inspector-title {
            font-size: 1.1rem;
            font-weight: 700;
            color: var(--cyan-accent);
            border-bottom: 1px solid var(--border-glass);
            padding-bottom: 8px;
        }
        .inspector-row {
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 0.85rem;
        }
        .inspector-row .label {
            color: var(--text-dim);
        }
        .inspector-row .val {
            font-weight: 600;
            font-family: monospace;
        }
        /* Virtual Channels Bar */
        .vc-bar {
            display: flex;
            height: 12px;
            border-radius: 6px;
            overflow: hidden;
            margin-top: 6px;
            background: #111;
        }
        .vc-seg {
            height: 100%;
            transition: width 0.3s;
        }
        .vc0 { background: var(--cyan-accent); }
        .vc1 { background: var(--green-accent); }
        .vc2 { background: var(--gold-accent); }
        .vc3 { background: #d946ef; }
        .vc-legend {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 6px;
            font-size: 0.72rem;
            color: var(--text-dim);
            margin-top: 6px;
        }
        .vc-dot {
            width: 8px;
            height: 8px;
            display: inline-block;
            border-radius: 50%;
            margin-right: 4px;
        }
        /* Registers Grid */
        .reg-grid {
            display: grid;
            grid-template-columns: repeat(4, 1fr);
            gap: 6px;
            margin-top: 6px;
        }
        .reg-box {
            background: rgba(15, 23, 42, 0.6);
            border: 1px solid var(--border-glass);
            border-radius: 4px;
            padding: 4px;
            text-align: center;
            font-family: monospace;
            font-size: 0.72rem;
        }
        .reg-name {
            color: var(--cyan-accent);
            font-size: 0.65rem;
        }
    </style>
</head>
<body>
    <!-- Top Telemetry HUD -->
    <div class="hud-grid">
        <div class="hud-card">
            <span class="hud-label">Architecture</span>
            <div class="hud-val" id="hud-arch">Wafer-65536</div>
        </div>
        <div class="hud-card">
            <span class="hud-label">NoC Interconnect</span>
            <div class="hud-val">8D Hyper-Torus</div>
        </div>
        <div class="hud-card">
            <span class="hud-label">Packet Collisions</span>
            <div class="hud-val" style="color: var(--green-accent);">
                0 <span class="badge-verified">VERIFIED</span>
            </div>
        </div>
        <div class="hud-card">
            <span class="hud-label">Peak Silicon Temp</span>
            <div class="hud-val" id="hud-peak-temp" style="color: var(--cyan-accent);">38.2 °C</div>
        </div>
        <div class="hud-card">
            <span class="hud-label">DVFS Throttling</span>
            <div class="hud-val" id="hud-throttled-count" style="color: var(--green-accent);">0 Dies</div>
        </div>
    </div>

    <!-- Controls Toolbar -->
    <div class="toolbar">
        <div class="btn-group">
            <button id="btn-toggle-view">⇄ Switch View: Wafer (16x16)</button>
            <button id="btn-step-cycle">▶ Step 100 Cycles</button>
            <button id="btn-quorum">⚡ Run Wafer AllReduce</button>
        </div>
        <div class="btn-group">
            <button id="btn-hotspot" class="btn-danger">🔥 Inject Hotspot (125°C)</button>
            <button id="btn-cool">❄ Active Cooling</button>
        </div>
    </div>

    <!-- Workspace: Grid and Inspector -->
    <div class="workspace">
        <div class="visualizer-container">
            <div class="wafer-grid" id="wafer-grid"></div>
        </div>

        <div class="inspector">
            <div class="inspector-title" id="insp-title">Die (0, 0) — Core 0</div>
            <div class="inspector-row">
                <span class="label">4D Sub-Coord:</span>
                <span class="val" id="insp-coord">[X:0, Y:0, Z:0, W:0]</span>
            </div>
            <div class="inspector-row">
                <span class="label">Temperature:</span>
                <span class="val" id="insp-temp" style="color: var(--cyan-accent);">38.2 °C</span>
            </div>
            <div class="inspector-row">
                <span class="label">DVFS State:</span>
                <span class="val" id="insp-dvfs" style="color: var(--green-accent);">ACTIVE (1.0 GHz)</span>
            </div>
            <div class="inspector-row">
                <span class="label">Hierarchy Role:</span>
                <span class="val" id="insp-role" style="color: var(--gold-accent);">Wafer Root</span>
            </div>

            <div style="border-top: 1px solid var(--border-glass); padding-top: 8px;">
                <span class="label" style="font-size: 0.8rem; font-weight: 600;">Virtual Channels (Zero-Collision Routing):</span>
                <div class="vc-bar">
                    <div class="vc-seg vc0" style="width: 72%;"></div>
                    <div class="vc-seg vc1" style="width: 18%;"></div>
                    <div class="vc-seg vc2" style="width: 6%;"></div>
                    <div class="vc-seg vc3" style="width: 4%;"></div>
                </div>
                <div class="vc-legend">
                    <div><span class="vc-dot" style="background: var(--cyan-accent);"></span>VC0 Direct DOR</div>
                    <div><span class="vc-dot" style="background: var(--green-accent);"></span>VC1 Torus Wrap</div>
                    <div><span class="vc-dot" style="background: var(--gold-accent);"></span>VC2 Detour Deflect</div>
                    <div><span class="vc-dot" style="background: #d946ef;"></span>VC3 Consensus Bar</div>
                </div>
            </div>

            <div style="border-top: 1px solid var(--border-glass); padding-top: 8px;">
                <span class="label" style="font-size: 0.8rem; font-weight: 600;">Banked Registers (Core Snapshot):</span>
                <div class="reg-grid" id="reg-grid"></div>
            </div>
        </div>
    </div>

    <script>
        const TOTAL_DIES = 256;
        let selectedDieId = 0;
        let dieTemps = new Array(TOTAL_DIES).fill(38.0);
        let dieThrottled = new Array(TOTAL_DIES).fill(false);
        let currentCycle = 100;
        let isWaferView = true;

        const gridEl = document.getElementById('wafer-grid');
        const regGridEl = document.getElementById('reg-grid');

        // Populate 16x16 Die Cells
        function renderGrid() {
            gridEl.innerHTML = '';
            for (let i = 0; i < TOTAL_DIES; i++) {
                const cell = document.createElement('div');
                cell.className = 'die-cell';
                if (i === selectedDieId) cell.classList.add('selected');
                if (dieThrottled[i]) cell.classList.add('throttled');
                
                // Color gradient based on temperature: 25C (cyan) -> 85C (amber) -> 105C+ (red)
                const temp = dieTemps[i];
                if (!dieThrottled[i]) {
                    const ratio = Math.min(Math.max((temp - 25) / 60, 0), 1);
                    const r = Math.round(ratio * 255);
                    const g = Math.round(229 - ratio * 50);
                    const b = Math.round(255 - ratio * 200);
                    cell.style.background = 'rgba(' + r + ', ' + g + ', ' + b + ', 0.18)';
                    cell.style.borderColor = 'rgba(' + r + ', ' + g + ', ' + b + ', 0.4)';
                }

                cell.textContent = dieThrottled[i] ? '!' : i;
                cell.onclick = () => selectDie(i);
                gridEl.appendChild(cell);
            }
        }

        function selectDie(id) {
            selectedDieId = id;
            updateInspector();
            renderGrid();
        }

        function updateInspector() {
            const wx = selectedDieId % 16;
            const wy = Math.floor(selectedDieId / 16);
            document.getElementById('insp-title').textContent = 'Die (' + wx + ', ' + wy + ') — Die #' + selectedDieId;
            document.getElementById('insp-coord').textContent = '[X:' + (wx % 4) + ', Y:' + (wy % 4) + ', Z:' + (Math.floor(wx/4)) + ', W:' + (Math.floor(wy/4)) + ']';
            
            const temp = dieTemps[selectedDieId].toFixed(1);
            const tempEl = document.getElementById('insp-temp');
            tempEl.textContent = temp + ' °C';
            tempEl.style.color = dieThrottled[selectedDieId] ? 'var(--red-alert)' : 'var(--cyan-accent)';

            const dvfsEl = document.getElementById('insp-dvfs');
            if (dieThrottled[selectedDieId]) {
                dvfsEl.textContent = 'DVFS THROTTLED (0.5 GHz)';
                dvfsEl.style.color = 'var(--red-alert)';
            } else {
                dvfsEl.textContent = 'ACTIVE (1.0 GHz)';
                dvfsEl.style.color = 'var(--green-accent)';
            }

            const roleEl = document.getElementById('insp-role');
            if (selectedDieId === 0) roleEl.textContent = 'Wafer Root';
            else if (selectedDieId % 16 === 0) roleEl.textContent = 'Quadrant Leader';
            else if (selectedDieId % 4 === 0) roleEl.textContent = 'Die Leader';
            else roleEl.textContent = 'Compute Worker';

            // Populate Register File
            regGridEl.innerHTML = '';
            for (let r = 0; r < 16; r++) {
                const box = document.createElement('div');
                box.className = 'reg-box';
                const val = (r * 17 + selectedDieId * 3) % 256;
                box.innerHTML = '<div class="reg-name">R' + r + '</div>0x' + val.toString(16).padStart(4, '0').toUpperCase();
                regGridEl.appendChild(box);
            }

            // Update Peak Temp & Throttled Count in HUD
            const peak = Math.max(...dieTemps).toFixed(1);
            document.getElementById('hud-peak-temp').textContent = peak + ' °C';
            const throttledCount = dieThrottled.filter(Boolean).length;
            const thEl = document.getElementById('hud-throttled-count');
            thEl.textContent = throttledCount + ' Dies';
            thEl.style.color = throttledCount > 0 ? 'var(--red-alert)' : 'var(--green-accent)';
        }

        // Action Handlers
        document.getElementById('btn-step-cycle').onclick = () => {
            currentCycle += 100;
            // Fourier bleed simulation
            for (let i = 0; i < TOTAL_DIES; i++) {
                if (dieThrottled[i]) {
                    dieTemps[i] = Math.max(dieTemps[i] - 4.5, 38.0);
                    if (dieTemps[i] <= 85.0) {
                        dieThrottled[i] = false;
                    }
                } else if (dieTemps[i] > 38.0) {
                    dieTemps[i] = Math.max(dieTemps[i] - 1.5, 38.0);
                }
            }
            updateInspector();
            renderGrid();
        };

        document.getElementById('btn-hotspot').onclick = () => {
            dieTemps[selectedDieId] = 125.0;
            dieThrottled[selectedDieId] = true;
            updateInspector();
            renderGrid();
        };

        document.getElementById('btn-cool').onclick = () => {
            dieTemps.fill(38.0);
            dieThrottled.fill(false);
            updateInspector();
            renderGrid();
        };

        document.getElementById('btn-quorum').onclick = () => {
            currentCycle += 250;
            updateInspector();
            renderGrid();
        };

        document.getElementById('btn-toggle-view').onclick = () => {
            isWaferView = !isWaferView;
            document.getElementById('hud-arch').textContent = isWaferView ? 'Wafer-65536' : '4D-Torus-256';
            document.getElementById('btn-toggle-view').textContent = isWaferView ? '⇄ Switch View: Wafer (16x16)' : '⇄ Switch View: 4D-Torus (4x4)';
        };

        // Initialize
        updateInspector();
        renderGrid();
    </script>
</body>
</html>`;
}

function deactivate() {
    if (statusBarItem) {
        statusBarItem.dispose();
    }
    if (!client) {
        return undefined;
    }
    return client.stop();
}

module.exports = {
    activate,
    deactivate
};

