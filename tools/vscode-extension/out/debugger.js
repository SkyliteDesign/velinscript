"use strict";
// VS Code Debugger Integration for VelinScript
Object.defineProperty(exports, "__esModule", { value: true });
exports.registerDebugger = exports.VelinDebugAdapterDescriptorFactory = void 0;
const vscode = require("vscode");
const child_process = require("child_process");
class VelinDebugAdapterDescriptorFactory {
    createDebugAdapterDescriptor(session, executable) {
        const config = vscode.workspace.getConfiguration('velin');
        const debuggerPath = config.get('debugger.path', 'velin-debugger');
        const port = config.get('debugger.port', 4711);
        // Start DAP server if not already running
        if (!this.server) {
            this.startDAPServer(debuggerPath, port);
        }
        // Return debug adapter descriptor
        return new vscode.DebugAdapterServer(port);
    }
    startDAPServer(debuggerPath, port) {
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
exports.VelinDebugAdapterDescriptorFactory = VelinDebugAdapterDescriptorFactory;
function registerDebugger(context) {
    const factory = new VelinDebugAdapterDescriptorFactory();
    context.subscriptions.push(vscode.debug.registerDebugAdapterDescriptorFactory('velin', factory));
    // Register debug configuration provider
    context.subscriptions.push(vscode.debug.registerDebugConfigurationProvider('velin', {
        provideDebugConfigurations(folder, token) {
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
        resolveDebugConfiguration(folder, config, token) {
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
    }));
}
exports.registerDebugger = registerDebugger;
//# sourceMappingURL=debugger.js.map