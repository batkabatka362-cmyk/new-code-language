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
