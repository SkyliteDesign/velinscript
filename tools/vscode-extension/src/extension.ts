import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';
import * as path from 'path';
import * as fs from 'fs';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
	const config = vscode.workspace.getConfiguration('velin');
	const lspPath = config.get<string>('lsp.path', 'velin-lsp');
	const compilerPath = config.get<string>('compiler.path', 'velin');

	// Server options
	const serverOptions: ServerOptions = {
		command: lspPath,
		args: []
	};

	// Client options
	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: 'file', language: 'velin' }],
	};

	// Create language client
	client = new LanguageClient(
		'velinLanguageServer',
		'VelinScript Language Server',
		serverOptions,
		clientOptions
	);

	// Start the client
	client.start();

	// Helper function to read template file
	function readTemplate(templateName: string): string {
		const templatePath = path.join(context.extensionPath, 'templates', templateName);
		try {
			return fs.readFileSync(templatePath, 'utf8');
		} catch (error) {
			vscode.window.showErrorMessage(`Template ${templateName} nicht gefunden`);
			return '';
		}
	}

	// Helper function to insert template at cursor position
	async function insertTemplate(templateName: string) {
		const editor = vscode.window.activeTextEditor;
		if (!editor || editor.document.languageId !== 'velin') {
			vscode.window.showWarningMessage('Bitte öffne eine .velin Datei');
			return;
		}

		const template = readTemplate(templateName);
		if (template === '') {
			return;
		}

		const position = editor.selection.active;
		await editor.edit(editBuilder => {
			editBuilder.insert(position, template);
		});

		vscode.window.showInformationMessage(`Template ${templateName} eingefügt`);
	}

	// Register commands
	context.subscriptions.push(
		vscode.commands.registerCommand('velin.compile', async () => {
			const editor = vscode.window.activeTextEditor;
			if (!editor || editor.document.languageId !== 'velin') {
				vscode.window.showWarningMessage('Bitte öffne eine .velin Datei');
				return;
			}

			const document = editor.document;
			const filePath = document.fileName;

			const terminal = vscode.window.createTerminal('VelinScript Compiler');
			terminal.sendText(`${compilerPath} compile -i "${filePath}"`);
			terminal.show();
		}),

		vscode.commands.registerCommand('velin.format', async () => {
			const editor = vscode.window.activeTextEditor;
			if (!editor || editor.document.languageId !== 'velin') {
				vscode.window.showWarningMessage('Bitte öffne eine .velin Datei');
				return;
			}

			const document = editor.document;
			const filePath = document.fileName;

			const terminal = vscode.window.createTerminal('VelinScript Formatter');
			terminal.sendText(`${compilerPath} format -i "${filePath}"`);
			terminal.show();
		}),

		vscode.commands.registerCommand('velin.check', async () => {
			const editor = vscode.window.activeTextEditor;
			if (!editor || editor.document.languageId !== 'velin') {
				vscode.window.showWarningMessage('Bitte öffne eine .velin Datei');
				return;
			}

			const document = editor.document;
			const filePath = document.fileName;

			const terminal = vscode.window.createTerminal('VelinScript Checker');
			terminal.sendText(`${compilerPath} check -i "${filePath}"`);
			terminal.show();
		}),

		// KI-Code-Generierung Commands
		vscode.commands.registerCommand('velin.generate.mlFunction', async () => {
			await insertTemplate('ml-function.velin');
		}),

		vscode.commands.registerCommand('velin.generate.modelLoader', async () => {
			await insertTemplate('model-loader.velin');
		}),

		vscode.commands.registerCommand('velin.generate.aiEndpoint', async () => {
			await insertTemplate('ai-endpoint.velin');
		}),

		// Neue Template-Commands
		vscode.commands.registerCommand('velin.generate.responses', async () => {
			await insertTemplate('responses.velin');
		}),

		vscode.commands.registerCommand('velin.generate.errors', async () => {
			await insertTemplate('errors.velin');
		}),

		vscode.commands.registerCommand('velin.generate.logging', async () => {
			await insertTemplate('logging.velin');
		}),

		vscode.commands.registerCommand('velin.generate.cache', async () => {
			await insertTemplate('cache.velin');
		}),

		vscode.commands.registerCommand('velin.generate.health', async () => {
			await insertTemplate('health.velin');
		}),

		vscode.commands.registerCommand('velin.generate.async', async () => {
			await insertTemplate('async.velin');
		}),

		vscode.commands.registerCommand('velin.generate.security', async () => {
			await insertTemplate('security.velin');
		}),

		// Test-Commands
		vscode.commands.registerCommand('velin.test', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Tests');
			terminal.sendText(`${compilerPath} test`);
			terminal.show();
		}),

		vscode.commands.registerCommand('velin.test.unit', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Unit Tests');
			terminal.sendText(`${compilerPath} test --unit`);
			terminal.show();
		}),

		vscode.commands.registerCommand('velin.test.integration', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Integration Tests');
			terminal.sendText(`${compilerPath} test --integration`);
			terminal.show();
		}),

		// Config-Commands
		vscode.commands.registerCommand('velin.config.init', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Config');
			terminal.sendText(`${compilerPath} config init`);
			terminal.show();
		}),

		vscode.commands.registerCommand('velin.config.validate', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Config');
			terminal.sendText(`${compilerPath} config validate`);
			terminal.show();
		}),

		// Backup Template-Commands
		vscode.commands.registerCommand('velin.generate.backup', async () => {
			await insertTemplate('backup.velin');
		}),

		vscode.commands.registerCommand('velin.generate.rollback', async () => {
			await insertTemplate('rollback.velin');
		}),

		vscode.commands.registerCommand('velin.generate.serialization', async () => {
			await insertTemplate('serialization.velin');
		}),

		// Backup-Commands
		vscode.commands.registerCommand('velin.backup.create', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Backup');
			terminal.sendText(`${compilerPath} backup create`);
			terminal.show();
		}),

		vscode.commands.registerCommand('velin.backup.restore', async () => {
			const backupId = await vscode.window.showInputBox({
				prompt: 'Backup-ID eingeben',
				placeHolder: 'backup-123'
			});
			if (backupId) {
				const terminal = vscode.window.createTerminal('VelinScript Backup');
				terminal.sendText(`${compilerPath} backup restore ${backupId}`);
				terminal.show();
			}
		}),

		vscode.commands.registerCommand('velin.backup.list', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Backup');
			terminal.sendText(`${compilerPath} backup list`);
			terminal.show();
		}),

		// Rollback-Commands
		vscode.commands.registerCommand('velin.rollback.begin', async () => {
			const terminal = vscode.window.createTerminal('VelinScript Rollback');
			terminal.sendText(`${compilerPath} rollback begin`);
			terminal.show();
		}),

		vscode.commands.registerCommand('velin.rollback.commit', async () => {
			const txId = await vscode.window.showInputBox({
				prompt: 'Transaktions-ID eingeben',
				placeHolder: 'tx-123'
			});
			if (txId) {
				const terminal = vscode.window.createTerminal('VelinScript Rollback');
				terminal.sendText(`${compilerPath} rollback commit ${txId}`);
				terminal.show();
			}
		}),

		vscode.commands.registerCommand('velin.rollback.rollback', async () => {
			const txId = await vscode.window.showInputBox({
				prompt: 'Transaktions-ID eingeben',
				placeHolder: 'tx-123'
			});
			if (txId) {
				const terminal = vscode.window.createTerminal('VelinScript Rollback');
				terminal.sendText(`${compilerPath} rollback rollback ${txId}`);
				terminal.show();
			}
		}),

		// Serialization-Commands
		vscode.commands.registerCommand('velin.serialize.jsonToYaml', async () => {
			const input = await vscode.window.showInputBox({
				prompt: 'JSON-Datei eingeben',
				placeHolder: 'config.json'
			});
			if (input) {
				const terminal = vscode.window.createTerminal('VelinScript Serialize');
				terminal.sendText(`${compilerPath} serialize json-to-yaml -i "${input}"`);
				terminal.show();
			}
		}),

		vscode.commands.registerCommand('velin.serialize.yamlToJson', async () => {
			const input = await vscode.window.showInputBox({
				prompt: 'YAML-Datei eingeben',
				placeHolder: 'config.yaml'
			});
			if (input) {
				const terminal = vscode.window.createTerminal('VelinScript Serialize');
				terminal.sendText(`${compilerPath} serialize yaml-to-json -i "${input}"`);
				terminal.show();
			}
		})
	);
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
