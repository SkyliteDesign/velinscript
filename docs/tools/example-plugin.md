# VelinScript Example Plugin

Ein Beispiel-Plugin für VelinScript, das Code-Metriken analysiert. Dieses Plugin dient als Vorlage für die Entwicklung eigener VelinScript-Plugins.

## Wofür ist das Example Plugin ideal?

Das Example Plugin ist ideal für:
- ✅ **Plugin-Entwicklung** - Vorlage für eigene VelinScript-Plugins
- ✅ **Compiler-API-Lernen** - Zeigt, wie die Compiler-API genutzt wird
- ✅ **Code-Metriken** - Analysiert Code-Metriken als Beispiel
- ✅ **Erweiterbarkeit** - Basis für eigene Tool-Entwicklung
- ✅ **Best Practices** - Zeigt Best Practices für Plugin-Entwicklung
- ✅ **Integration** - Beispiel für Tool-Integration in VelinScript

## Wofür ist das Example Plugin NICHT gedacht?

Das Example Plugin ist NICHT gedacht für:
- ❌ **Production-Analyse** - Für detaillierte Code-Analyse nutzen Sie den Linter
- ❌ **Code-Qualität** - Für Code-Qualität nutzen Sie den Linter
- ❌ **Performance-Analyse** - Für Performance nutzen Sie den Profiler
- ❌ **Security-Checks** - Für Security nutzen Sie den Security Scanner
- ❌ **Direkte Nutzung** - Primär als Vorlage, nicht für direkte Nutzung

## Installation

```bash
cd tools/example-plugin
cargo build --release
```

## Verwendung

### Code-Metriken analysieren

```bash
velin-example-plugin metrics -i <verzeichnis>
```

**Beispiel:**
```bash
velin-example-plugin metrics -i examples/
```

### JSON-Output

```bash
velin-example-plugin metrics -i examples/ --format json
```

### Verbose Output

```bash
velin-example-plugin metrics -i examples/ --verbose
```

## Features

### Code-Metriken

- **Funktionen** - Anzahl der Funktionen
- **Structs** - Anzahl der Structs
- **Enums** - Anzahl der Enums
- **Durchschnittliche Funktion-Länge** - Durchschnittliche Zeilen pro Funktion
- **Gesamt-Zeilen** - Gesamte Anzahl Code-Zeilen

### Output-Formate

- **Text** - Menschenlesbares Format (Standard)
- **JSON** - Maschinenlesbares Format

## Beispiel-Output

### Text-Format

```
📊 Code-Metriken für examples/:
  Funktionen: 45
  Structs: 12
  Enums: 3
  Durchschnittliche Funktion-Länge: 15 Zeilen
  Gesamt-Zeilen: 1200
```

### JSON-Format

```json
{
  "functions": 45,
  "structs": 12,
  "enums": 3,
  "average_function_length": 15,
  "total_lines": 1200,
  "files_analyzed": 23
}
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Example Plugin                             │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-example-plugin metrics -i examples/            │
│                                                         │
│  📊 Code-Metriken für examples/:                        │
│    Funktionen: 45                                       │
│    Structs: 12                                          │
│    Enums: 3                                             │
│    Durchschnittliche Funktion-Länge: 15 Zeilen         │
│    Gesamt-Zeilen: 1200                                  │
│                                                         │
│  ✓ Analyse abgeschlossen                               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Plugin-Entwicklung

Dieses Plugin dient als Vorlage für eigene VelinScript-Plugins.

### Projekt-Struktur

```
example-plugin/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    └── metrics.rs
```

### Cargo.toml

```toml
[package]
name = "velin-example-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
velin-compiler = { path = "../../compiler" }
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Plugin-Implementierung

```rust
use velin_compiler::parser::ast::*;
use velin_compiler::parser::Parser;

pub struct Metrics {
    pub functions: usize,
    pub structs: usize,
    pub enums: usize,
    pub total_lines: usize,
}

impl Metrics {
    pub fn analyze_directory(path: &Path) -> Result<Self> {
        // Implementierung
    }
}
```

## Eigene Plugins erstellen

### 1. Projekt-Struktur erstellen

```bash
cargo new --bin my-velin-plugin
cd my-velin-plugin
```

### 2. Dependencies hinzufügen

```toml
[dependencies]
velin-compiler = { path = "../../compiler" }
clap = { version = "4.0", features = ["derive"] }
```

### 3. Plugin-Logik implementieren

```rust
use velin_compiler::parser::ast::*;
use velin_compiler::parser::Parser;

fn main() {
    // Plugin-Logik
}
```

### 4. Build und Test

```bash
cargo build --release
./target/release/my-velin-plugin
```

## Plugin-API

### Parser verwenden

```rust
use velin_compiler::parser::Parser;

let parser = Parser::new();
let program = parser.parse(&code)?;
```

### AST durchlaufen

```rust
for item in &program.items {
    match item {
        Item::Function(func) => {
            // Funktion verarbeiten
        }
        Item::Struct(s) => {
            // Struct verarbeiten
        }
        Item::Enum(e) => {
            // Enum verarbeiten
        }
        _ => {}
    }
}
```

### Type-Checker verwenden

```rust
use velin_compiler::type_checker::TypeChecker;

let mut type_checker = TypeChecker::new();
type_checker.check(&program)?;
```

## Best Practices

1. **Klarer Zweck** - Jedes Plugin sollte einen klaren Zweck haben
2. **Gute Dokumentation** - Dokumentiere alle Features und Optionen
3. **Error Handling** - Behandle Fehler ordentlich
4. **Testing** - Schreibe Tests für Plugin-Funktionalität
5. **Performance** - Optimiere für große Codebases
6. **CLI-Design** - Verwende clap für konsistente CLI

## Integration

### VS Code Extension

Plugins können in VS Code Extensions integriert werden:

```typescript
import { exec } from 'child_process';

exec('velin-example-plugin metrics -i src/', (error, stdout) => {
    if (error) {
        console.error(error);
        return;
    }
    console.log(stdout);
});
```

### CI/CD

```yaml
# .github/workflows/metrics.yml
- name: Code Metrics
  run: |
    cd tools/example-plugin
    cargo build --release
    ./target/release/velin-example-plugin metrics -i src/ --format json > metrics.json
```

## Erweiterte Features

### Custom Metriken

Erweitere das Plugin um eigene Metriken:

```rust
pub struct ExtendedMetrics {
    pub functions: usize,
    pub structs: usize,
    pub complexity: f64,
    pub test_coverage: f64,
}
```

### Filter-Optionen

```bash
velin-example-plugin metrics -i src/ \
  --exclude "test/**" \
  --include "src/**/*.velin"
```

### Export-Funktionen

```bash
velin-example-plugin metrics -i src/ \
  --export csv \
  --output metrics.csv
```

## Troubleshooting

### Plugin kompiliert nicht

- Prüfe `velin-compiler` Dependency-Pfad
- Prüfe Rust-Version (mindestens 1.70)
- Prüfe Feature-Flags

### Plugin findet keine Dateien

- Prüfe Verzeichnis-Pfad
- Prüfe Datei-Extension (.velin)
- Prüfe Berechtigungen

### Performance-Probleme

- Verwende Parallelisierung für große Verzeichnisse
- Cache Parsing-Ergebnisse
- Optimiere AST-Traversierung

## Weitere Ressourcen

- [VelinScript Compiler API](../../compiler/README.md)
- [Plugin Development Guide](../../docs/guides/plugin-development.md)
- [Example Plugins](../../tools/)
