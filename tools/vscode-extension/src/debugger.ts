// VS Code Debugger Integration for VelinScript

import * as vscode from 'vscode';
import * as net from 'net';
import * as child_process from 'child_process';

export class VelinDebugAdapterDescriptorFactory implements vscode.DebugAdapterDescriptorFactory {
    private server?: net.Server;

    createDebugAdapterDescriptor(
        session: vscode.DebugSession,
        executable: vscode.DebugAdapterExecutable | undefined
    ): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
        const config = vscode.workspace.getConfiguration('velin');
        const debuggerPath = config.get<string>('debugger.path', 'velin-debugger');
        const port = config.get<number>('debugger.port', 4711);

        // Start DAP server if not already running
        if (!this.server) {
            this.startDAPServer(debuggerPath, port);
        }

        // Return debug adapter descriptor
        return new vscode.DebugAdapterServer(port);
    }

    private startDAPServer(debuggerPath: string, port: number) {
        // Start velin-debugger in background
        const process = child_process.spawn(debuggerPath, ['start', '--port', port.toString()], {
            detached: true,
            stdio: 'ignore'
        });

        process.unref();

        // Give server time to start
        setTimeout(() => {
            console.log(`VelinScript Debugger started on port ${port}`);
        }, 1000);
    }

    dispose() {
        if (this.server) {
            this.server.close();
        }
    }
}

export function registerDebugger(context: vscode.ExtensionContext) {
    const factory = new VelinDebugAdapterDescriptorFactory();
    context.subscriptions.push(
        vscode.debug.registerDebugAdapterDescriptorFactory('velin', factory)
    );

    // Register debug configuration provider
    context.subscriptions.push(
        vscode.debug.registerDebugConfigurationProvider('velin', {
            provideDebugConfigurations(
                folder: vscode.WorkspaceFolder | undefined,
                token?: vscode.CancellationToken
            ): vscode.ProviderResult<vscode.DebugConfiguration[]> {
                return [
                    {
                        type: 'velin',
                        request: 'launch',
                        name: 'Debug VelinScript',
                        program: '${workspaceFolder}/main.velin',
                        stopOnEntry: false
                    },
                    {
                        type: 'velin',
                        request: 'attach',
                        name: 'Attach to VelinScript',
                        port: 4711,
                        host: 'localhost'
                    }
                ];
            },
            resolveDebugConfiguration(
                folder: vscode.WorkspaceFolder | undefined,
                config: vscode.DebugConfiguration,
                token?: vscode.CancellationToken
            ): vscode.ProviderResult<vscode.DebugConfiguration> {
                // Validate and resolve debug configuration
                if (!config.type) {
                    config.type = 'velin';
                }
                if (config.request === 'launch' && !config.program) {
                    config.program = '${workspaceFolder}/main.velin';
                }
                if (config.request === 'attach' && !config.port) {
                    config.port = 4711;
                }
                return config;
            }
        })
    );
}
