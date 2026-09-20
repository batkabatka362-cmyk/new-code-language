// ============================================================================
// CRON VS Code Extension Entry Point
// Connects to 'cron lsp' language server daemon via stdio.
// ============================================================================

const vscode = require('vscode');
const path = require('path');

let client;

function activate(context) {
    const outputChannel = vscode.window.createOutputChannel('CRON Language Server');
    outputChannel.appendLine('[CRON Extension] Activating CRON Cognitive Language extension...');

    try {
        const { LanguageClient, TransportKind } = require('vscode-languageclient/node');
        const config = vscode.workspace.getConfiguration('cron');
        const serverPath = config.get('lsp.serverPath') || 'cron';

        const serverOptions = {
            command: serverPath,
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
        outputChannel.appendLine('[CRON Extension] LanguageClient started successfully.');
    } catch (err) {
        outputChannel.appendLine('[CRON Extension] Running in standalone syntax mode: ' + err.message);
    }

    // Register convenience commands
    context.subscriptions.push(
        vscode.commands.registerCommand('cron.buildC23', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cr')) {
                vscode.window.showErrorMessage('Please open a .cr file to compile.');
                return;
            }
            const term = vscode.window.createTerminal('CRON Build');
            term.show();
            term.sendText(`cron c23 "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.runNative', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cr')) {
                vscode.window.showErrorMessage('Please open a .cr file to run.');
                return;
            }
            const term = vscode.window.createTerminal('CRON Native Run');
            term.show();
            term.sendText(`cron native "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.healCl', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file to heal.');
                return;
            }
            const term = vscode.window.createTerminal('CRON Auto-Heal .cl');
            term.show();
            term.sendText(`cron cl-heal "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.optCl', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file to optimize.');
                return;
            }
            const term = vscode.window.createTerminal('CRON Super-Optimize .cl');
            term.show();
            term.sendText(`cron cl-opt "${editor.document.fileName}" --level 2`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('cron.runJitCl', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || !editor.document.fileName.endsWith('.cl')) {
                vscode.window.showErrorMessage('Please open a .cl file to execute.');
                return;
            }
            const term = vscode.window.createTerminal('CRON JIT .cl');
            term.show();
            term.sendText(`cron cl-run "${editor.document.fileName}"`);
        })
    );
}

function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

module.exports = {
    activate,
    deactivate
};
