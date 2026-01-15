# 🔌 VelinScript Plugin-Entwicklung

Eine vollständige Anleitung zum Erstellen von Plugins und Tools für VelinScript 2.0.

---

## 📋 Inhaltsverzeichnis

1. [Überblick](#überblick)
2. [Plugin-Typen](#plugin-typen)
3. [Rust Tool Plugin](#rust-tool-plugin)
4. [VS Code Extension](#vs-code-extension)a
5. [LSP Extension](#lsp-extension)
6. [Best Practices](#best-practices)
7. [Beispiele](#beispiele)

---

## 🎯 Überblick

VelinScript unterstützt verschiedene Arten von Plugins:

- **Rust Tool Plugins**: CLI-Tools, die als separate Binaries kompiliert werden
- **VS Code Extensions**: Editor-Integration für VS Code
- **LSP Extensions**: Language Server Protocol Erweiterungen

Alle Plugins können auf die VelinScript Compiler-API zugreifen, um Code zu parsen, zu analysieren und zu transformieren.

---

## 🔧 Plugin-Typen

### 1. Rust Tool Plugin

Ein eigenständiges CLI-Tool, das als Binary kompiliert wird. Beispiele:
- `velin-lint` - Code-Qualitätsprüfung
- `velin-security` - Security-Scanner
- `velin-dead-code` - Dead Code Detector

**Vorteile:**
- ✅ Hohe Performance (native Rust)
- ✅ Direkter Zugriff auf Compiler-API
- ✅ Einfache Distribution als Binary
- ✅ Kann in CI/CD integriert werden

### 2. VS Code Extension

Eine TypeScript-basierte Extension für VS Code. Beispiele:
- Syntax Highlighting
- Code Completion
- Debugger Integration

**Vorteile:**
- ✅ Direkte Editor-Integration
- ✅ Benutzerfreundliche UI
- ✅ Schnelle Entwicklung

### 3. LSP Extension

Erweitert den Language Server Protocol Server. Beispiele:
- Custom Code Actions
- Erweiterte Hover-Informationen
- Custom Diagnostics

**Vorteile:**
- ✅ Funktioniert mit allen LSP-kompatiblen Editoren
- ✅ Zentralisierte Logik
- ✅ Konsistente Erfahrung

---

## 🦀 Rust Tool Plugin

### Schritt 1: Projekt-Struktur erstellen

```bash
# Im tools/ Verzeichnis
cd tools
mkdir my-plugin
cd my-plugin
```

### Schritt 2: Cargo.toml erstellen

```toml
[package]
name = "velin-my-plugin"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "velin-my-plugin"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
velin-compiler = { path = "../../compiler" }
walkdir = "2.0"  # Optional, für Verzeichnis-Traversierung
```

### Schritt 3: Grundstruktur (src/main.rs)

```rust
// VelinScript My Plugin
// Beschreibung deines Plugins

mod analyzer;  // Deine Plugin-Logik

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use velin_compiler::parser::parser::Parser as VelinParser;

#[derive(Parser)]
#[command(name = "velin-my-plugin")]
#[command(about = "VelinScript My Plugin - Beschreibung", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Hauptbefehl deines Plugins
    Analyze {
        /// Eingabe-Datei oder Verzeichnis
        #[arg(short, long, default_value = ".")]
        input: PathBuf,
        
        /// Optionale Flag
        #[arg(short, long)]
        verbose: bool,
        
        /// JSON-Output
        #[arg(short, long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { input, verbose, json } => {
            analyze_command(input, verbose, json)
        }
    }
}

fn analyze_command(input: PathBuf, verbose: bool, json: bool) -> Result<()> {
    println!("🔍 Analysiere: {}\n", input.display());
    
    // Lade VelinScript-Dateien
    let files = collect_velin_files(&input)?;
    
    if files.is_empty() {
        eprintln!("Keine VelinScript-Dateien gefunden");
        return Ok(());
    }
    
    let mut results = Vec::new();
    
    // Analysiere jede Datei
    for file in &files {
        if verbose {
            println!("📝 Analysiere: {}", file.display());
        }
        
        let content = std::fs::read_to_string(file)?;
        
        // Parse VelinScript Code
        let program = VelinParser::parse(&content)
            .map_err(|e| anyhow::anyhow!("Parse error: {}", e.message))?;
        
        // Führe deine Analyse durch
        let analysis_result = analyzer::analyze(&program, file)?;
        results.push(analysis_result);
    }
    
    // Output generieren
    if json {
        let json_output = serde_json::json!({
            "results": results,
            "total": results.len()
        });
        println!("{}", serde_json::to_string_pretty(&json_output)?);
    } else {
        // Text-Output
        for result in &results {
            println!("{}: {}", result.file.display(), result.message);
        }
    }
    
    Ok(())
}

fn collect_velin_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) == Some("velin") {
            files.push(path.clone());
        }
    } else if path.is_dir() {
        for entry in walkdir::WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("velin") {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    
    Ok(files)
}
```

### Schritt 4: Plugin-Logik (src/analyzer.rs)

```rust
use anyhow::Result;
use std::path::PathBuf;
use velin_compiler::parser::ast::Program;

pub struct AnalysisResult {
    pub file: PathBuf,
    pub message: String,
    pub severity: Severity,
}

pub enum Severity {
    Info,
    Warning,
    Error,
}

pub fn analyze(program: &Program, file: &PathBuf) -> Result<AnalysisResult> {
    // Deine Analyse-Logik hier
    // Beispiel: Zähle Funktionen
    
    let function_count = program.items.iter()
        .filter(|item| matches!(item, velin_compiler::parser::ast::Item::Function(_)))
        .count();
    
    Ok(AnalysisResult {
        file: file.clone(),
        message: format!("Gefunden: {} Funktionen", function_count),
        severity: Severity::Info,
    })
}
```

### Schritt 5: Kompilieren und Testen

```bash
# Kompilieren
cargo build --release

# Binary ist jetzt verfügbar unter:
# target/release/velin-my-plugin

# Testen
./target/release/velin-my-plugin analyze -i examples/
```

### Schritt 6: Integration in VelinScript

Falls du möchtest, dass dein Plugin in der Haupt-CLI verfügbar ist, kannst du es als Subcommand hinzufügen:

```rust
// In compiler/src/cli.rs
#[derive(Subcommand)]
pub enum Commands {
    // ... bestehende Commands ...
    
    /// Mein Custom Plugin
    MyPlugin {
        #[command(subcommand)]
        subcommand: MyPluginCommands,
    },
}
```

---

## 🎨 VS Code Extension

### Schritt 1: Projekt-Struktur erstellen

```bash
cd tools
mkdir my-vscode-extension
cd my-vscode-extension
npm init -y
```

### Schritt 2: package.json konfigurieren

```json
{
  "name": "velin-my-extension",
  "displayName": "VelinScript My Extension",
  "description": "Meine Custom VelinScript Extension",
  "version": "0.1.0",
  "publisher": "velinscript",
  "engines": {
    "vscode": "^1.60.0"
  },
  "categories": [
    "Other"
  ],
  "activationEvents": [
    "onLanguage:velin"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "velin.myCommand",
        "title": "My Custom Command"
      }
    ]
  },
  "scripts": {
    "compile": "tsc -p ./",
    "watch": "tsc -watch -p ./"
  },
  "dependencies": {
    "vscode": "^1.60.0"
  },
  "devDependencies": {
    "@types/node": "^18.0.0",
    "@types/vscode": "^1.60.0",
    "typescript": "^5.0.0"
  }
}
```

### Schritt 3: TypeScript Extension (src/extension.ts)

```typescript
import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export function activate(context: vscode.ExtensionContext) {
    console.log('VelinScript My Extension ist jetzt aktiv!');
    
    // Command registrieren
    const disposable = vscode.commands.registerCommand(
        'velin.myCommand',
        async () => {
            const editor = vscode.window.activeTextEditor;
            
            if (!editor || editor.document.languageId !== 'velin') {
                vscode.window.showWarningMessage(
                    'Bitte öffne eine .velin Datei'
                );
                return;
            }
            
            const document = editor.document;
            const filePath = document.fileName;
            
            // Führe dein Tool aus
            try {
                const { stdout, stderr } = await execAsync(
                    `velin-my-plugin analyze -i "${filePath}"`
                );
                
                // Zeige Ergebnis
                const outputChannel = vscode.window.createOutputChannel(
                    'VelinScript My Plugin'
                );
                outputChannel.appendLine(stdout);
                outputChannel.show();
                
                vscode.window.showInformationMessage(
                    'Analyse erfolgreich!'
                );
            } catch (error: any) {
                vscode.window.showErrorMessage(
                    `Fehler: ${error.message}`
                );
            }
        }
    );
    
    context.subscriptions.push(disposable);
}

export function deactivate() {
    console.log('VelinScript My Extension wurde deaktiviert');
}
```

### Schritt 4: TypeScript konfigurieren (tsconfig.json)

```json
{
  "compilerOptions": {
    "module": "commonjs",
    "target": "ES2020",
    "outDir": "out",
    "lib": ["ES2020"],
    "sourceMap": true,
    "rootDir": "src",
    "strict": true
  },
  "exclude": ["node_modules", ".vscode-test"]
}
```

### Schritt 5: Kompilieren und Testen

```bash
# Dependencies installieren
npm install

# Kompilieren
npm run compile

# In VS Code testen
# 1. F5 drücken (Start Debugging)
# 2. Extension Development Host öffnet sich
# 3. Command Palette (Ctrl+Shift+P)
# 4. "My Custom Command" ausführen
```

### Schritt 6: Package erstellen

```bash
# VSIX Package erstellen
npm install -g vsce
vsce package
```

---

## 🔌 LSP Extension

### Schritt 1: LSP Handler erstellen

```rust
// In tools/lsp/src/handlers/

use lsp_types::{
    CodeAction, CodeActionKind, CodeActionParams, Command,
    Diagnostic, DiagnosticSeverity, Position, Range,
};
use velin_compiler::parser::parser::Parser;

pub fn handle_custom_code_action(
    params: CodeActionParams,
    document: &str,
) -> Vec<CodeAction> {
    let mut actions = Vec::new();
    
    // Parse VelinScript Code
    if let Ok(program) = Parser::parse(document) {
        // Analysiere Code und erstelle Code Actions
        // Beispiel: Quick Fix für häufige Fehler
        
        let action = CodeAction {
            title: "Fix: Add missing return type".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: None,
            edit: None,
            command: Some(Command {
                title: "Fix".to_string(),
                command: "velin.fix".to_string(),
                arguments: None,
            }),
            is_preferred: Some(true),
            disabled: None,
            data: None,
        };
        
        actions.push(action);
    }
    
    actions
}

pub fn handle_custom_diagnostics(document: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    // Parse und analysiere Code
    if let Ok(program) = Parser::parse(document) {
        // Erstelle Custom Diagnostics
        // Beispiel: Warnung bei zu langen Funktionen
        
        for item in &program.items {
            if let velin_compiler::parser::ast::Item::Function(func) = item {
                // Prüfe Funktion-Länge
                // ... deine Logik ...
                
                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 0 },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(lsp_types::NumberOrString::String(
                        "custom-warning".to_string()
                    )),
                    source: Some("velin-my-plugin".to_string()),
                    message: "Funktion ist zu lang".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                };
                
                diagnostics.push(diagnostic);
            }
        }
    }
    
    diagnostics
}
```

### Schritt 2: In LSP Server integrieren

```rust
// In tools/lsp/src/server.rs

use handlers::custom::{handle_custom_code_action, handle_custom_diagnostics};

// In der Code Action Handler-Funktion
fn handle_code_action(params: CodeActionParams) -> Result<Vec<CodeAction>> {
    let document = get_document(&params.text_document.uri)?;
    
    // Standard LSP Actions
    let mut actions = standard_code_actions(&params, &document)?;
    
    // Custom Actions hinzufügen
    let custom_actions = handle_custom_code_action(params, &document);
    actions.extend(custom_actions);
    
    Ok(actions)
}

// In der Diagnostics-Funktion
fn publish_diagnostics(uri: &Url) -> Result<Vec<Diagnostic>> {
    let document = get_document(uri)?;
    
    // Standard Diagnostics
    let mut diagnostics = standard_diagnostics(&document)?;
    
    // Custom Diagnostics hinzufügen
    let custom_diagnostics = handle_custom_diagnostics(&document);
    diagnostics.extend(custom_diagnostics);
    
    Ok(diagnostics)
}
```

---

## ✅ Best Practices

### 1. Fehlerbehandlung

```rust
use anyhow::{Context, Result};

fn process_file(path: &PathBuf) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read: {}", path.display()))?;
    
    // ... Verarbeitung ...
    
    Ok(())
}
```

### 2. Logging

```rust
use tracing::{info, warn, error};

fn analyze(program: &Program) -> Result<()> {
    info!("Starting analysis");
    
    // ... Analyse ...
    
    if found_issues {
        warn!("Found {} issues", count);
    }
    
    Ok(())
}
```

### 3. Performance

```rust
// Verwende Parallelisierung für große Projekte
use rayon::prelude::*;

files.par_iter().for_each(|file| {
    // Parallele Verarbeitung
});
```

### 4. Konfiguration

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PluginConfig {
    enabled: bool,
    max_issues: usize,
    rules: Vec<String>,
}

fn load_config() -> Result<PluginConfig> {
    let config_path = std::env::current_dir()?
        .join("velin-plugin.toml");
    
    if config_path.exists() {
        let content = std::fs::read_to_string(config_path)?;
        let config: PluginConfig = toml::from_str(&content)?;
        Ok(config)
    } else {
        Ok(PluginConfig::default())
    }
}
```

### 5. Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyzer() {
        let code = r#"
            fn test(): string {
                return "hello";
            }
        "#;
        
        let program = Parser::parse(code).unwrap();
        let result = analyzer::analyze(&program, &PathBuf::from("test.velin")).unwrap();
        
        assert_eq!(result.message, "Gefunden: 1 Funktionen");
    }
}
```

---

## 📚 Beispiele

### Beispiel 1: Complexity Analyzer

```rust
// src/analyzer.rs

pub fn analyze_complexity(program: &Program) -> Result<Vec<ComplexityIssue>> {
    let mut issues = Vec::new();
    
    for item in &program.items {
        if let Item::Function(func) = item {
            let complexity = calculate_complexity(&func.body);
            
            if complexity > 10 {
                issues.push(ComplexityIssue {
                    function: func.name.clone(),
                    complexity,
                    suggestion: "Funktion in kleinere Funktionen aufteilen".to_string(),
                });
            }
        }
    }
    
    Ok(issues)
}

fn calculate_complexity(body: &Block) -> usize {
    // Zähle if, while, for, match Statements
    // ... Implementierung ...
    0
}
```

### Beispiel 2: Dependency Analyzer

```rust
// src/analyzer.rs

pub fn analyze_dependencies(program: &Program) -> Result<DependencyGraph> {
    let mut graph = DependencyGraph::new();
    
    for item in &program.items {
        if let Item::Function(func) = item {
            let dependencies = extract_function_calls(&func.body);
            graph.add_node(&func.name, dependencies);
        }
    }
    
    Ok(graph)
}
```

### Beispiel 3: Code Metrics Plugin

```rust
// src/main.rs

#[derive(Subcommand)]
enum Commands {
    /// Zeigt Code-Metriken
    Metrics {
        #[arg(short, long)]
        input: PathBuf,
        
        #[arg(short, long)]
        format: Option<String>,
    },
}

fn metrics_command(input: PathBuf, format: Option<String>) -> Result<()> {
    let files = collect_velin_files(&input)?;
    let mut metrics = CodeMetrics::new();
    
    for file in &files {
        let content = std::fs::read_to_string(file)?;
        let program = Parser::parse(&content)?;
        
        metrics.analyze(&program);
    }
    
    match format.as_deref() {
        Some("json") => {
            println!("{}", serde_json::to_string_pretty(&metrics)?);
        }
        _ => {
            println!("📊 Code-Metriken:");
            println!("  Funktionen: {}", metrics.function_count);
            println!("  Structs: {}", metrics.struct_count);
            println!("  Enums: {}", metrics.enum_count);
            println!("  Durchschnittliche Funktion-Länge: {}", metrics.avg_function_length);
        }
    }
    
    Ok(())
}
```

---

## 🚀 Veröffentlichung

### Rust Tool Plugin

1. **Repository erstellen**
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   ```

2. **Cargo.toml Metadata hinzufügen**
   ```toml
   [package]
   name = "velin-my-plugin"
   version = "0.1.0"
   authors = ["Dein Name <email@example.com>"]
   license = "MIT"
   repository = "https://github.com/username/velin-my-plugin"
   description = "Beschreibung deines Plugins"
   ```

3. **Auf crates.io veröffentlichen** (optional)
   ```bash
   cargo publish
   ```

### VS Code Extension

1. **VSIX Package erstellen**
   ```bash
   vsce package
   ```

2. **Auf Marketplace veröffentlichen**
   ```bash
   vsce publish
   ```

---

## 📖 Weitere Ressourcen

- [VelinScript Compiler API](https://github.com/SkyliteDesign/velinscript/tree/main/compiler)
- [Clap Documentation](https://docs.rs/clap/)
- [VS Code Extension API](https://code.visualstudio.com/api)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)

---

## 💡 Tipps

1. **Starte klein**: Beginne mit einem einfachen Plugin und erweitere es schrittweise
2. **Nutze bestehende Tools**: Schaue dir `velin-lint` oder `velin-security` als Referenz an
3. **Teste gründlich**: Erstelle Tests für deine Plugin-Logik
4. **Dokumentiere**: Erstelle eine README mit Beispielen
5. **Community**: Teile dein Plugin in der Community

---

**Viel Erfolg beim Plugin-Entwickeln! 🚀**
