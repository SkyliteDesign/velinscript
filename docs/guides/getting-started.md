# Getting Started mit VelinScript

Willkommen bei VelinScript! Diese Anleitung führt dich durch die ersten Schritte.

**Velisch** = Sprache / Produkt · **VelinScript** = Compiler (`velin`) · Einstieg: [velisch.info](https://velisch.info).

VelinScript 3.5.0 konzentriert sich auf den stabilen API-Entwicklungsweg mit Rust/Axum. Weitere Funktionen wie zusätzliche Target-Runtimes, erweiterte Authentifizierung, AI-Sandbox-Ausführung und weitere Runtime-Module befinden sich außerhalb dieses 3.5.0-Stable-Umfangs.

## Installation

### Voraussetzungen

- Rust 1.70+ (für Compiler-Entwicklung)
- Git

### Installation

#### Option 1: Aus Source bauen

```bash
# Repository klonen
git clone https://github.com/SkyliteDesign/velinscript.git
cd velinscript

# Compiler bauen
cd compiler
cargo build --release

# Binary: compiler/target/release/velin
# Optional: In PATH einbinden
export PATH=$PATH:$(pwd)/target/release
velin --version
```

#### Option 2: Installation Script

```bash
# Auf Linux/macOS:
curl -sSL https://velisch.dev/install | sh

# Auf Windows (PowerShell):
curl -sSL https://velisch.dev/install | bash
```

## Dein erstes Programm

### 1. Projekt initialisieren

```bash
# Beide Befehle funktionieren:
velin init my-first-api
# oder
velin new my-first-api

cd my-first-api
```

Dies erstellt:
- `main.velin` - Hauptdatei
- `README.md` - Projekt-Dokumentation

### 2. Erste Funktion schreiben

Öffne `main.velin` und schreibe:

```velin
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript! 🚀";
}
```

### 3. Kompilieren

```bash
velin compile -i main.velin -o main.rs
```

Dies erstellt `main.rs` mit dem kompilierten Rust-Code.

**Optional: Andere Ziel-Sprachen**

Du kannst auch PHP oder Python Code generieren:

```bash
# Für PHP
velin compile -i main.velin -o main.php --target php

# Für Python
velin compile -i main.velin -o main.py --target python
```

### 4. Type Checking

```bash
velin check -i main.velin
```

Prüft den Code auf Type-Fehler.

## Grundlagen

### Variablen

```velin
let name = "John";        // Type Inference: string
let age = 30;             // Type Inference: number
let active: boolean = true; // Expliziter Typ
let mut counter = 0;      // Mutable Variable
```

### Funktionen

```velin
fn greet(name: string): string {
    return "Hello, " + name;
}

// Ohne Parameter
fn getVersion(): string {
    return "0.1.0";
}

// Ohne Rückgabewert
fn printMessage(msg: string): void {
    // ...
}
```

### Structs

```velin
struct User {
    id: string,
    name: string,
    email: string,
}
```

### API Endpoints

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}

@POST("/api/users")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
    };
    return db.save(user);
}
```

## Nächste Schritte

1. **Tutorial 1: Basics** - [tutorial-1-basics.md](tutorial-1-basics.md)
2. **Tutorial 2: APIs** - [tutorial-2-apis.md](tutorial-2-apis.md)
3. **Tutorial 3: Security** - [tutorial-3-security.md](tutorial-3-security.md)
4. **Tutorial 4: Database** - [tutorial-4-database.md](tutorial-4-database.md)
5. **Tutorial 5: Validation** - [tutorial-5-validation.md](tutorial-5-validation.md)
6. **Tutorial 6: Authentication** - [tutorial-6-authentication.md](tutorial-6-authentication.md)
7. **Tutorial 7: ML Integration** - [tutorial-7-ml.md](tutorial-7-ml.md)
8. **Tutorial 8: Intelligence** - [tutorial-8-intelligence.md](tutorial-8-intelligence.md)

## Entwickler-Tools

- **Auto-Import Management** - [auto-imports.md](auto-imports.md)
- **AutoFix Engine** - Automatische Fehlerkorrektur mit `--autofix` Flag
- **VelinAutoDoc** - Automatische Dokumentationsgenerierung
- **VelinAutoTest** - Automatische Test-Generierung
- **VelinInsight** - Code-Analyse und Qualitätsprüfung
- **VelinPipeline** - Automatische Performance-Optimierung
- **Dead Code Detector** - [../tools/dead-code-detector.md](../tools/dead-code-detector.md)
- **API Documentation Generator** - [../tools/api-doc-generator.md](../tools/api-doc-generator.md)
- **Bibliotheks-Generator** - [../tools/library-generator.md](../tools/library-generator.md) ✅ (Neu in 2.7)
- **VS Code Extension** - [../tools/vscode-extension.md](../tools/vscode-extension.md) ✅
- **Package Manager** - Siehe `tools/package-manager/README.md`

## Beispiele

Siehe [examples/](../../examples/) für vollständige Beispiel-Projekte.

**Empfohlene Beispiele:**
- **[01-hello-api](../../examples/01-hello-api/)** - Einfaches Einstiegsbeispiel
- **[02-llm-chat](../../examples/02-llm-chat/)** - LLM-Integration
- **[05-ultimate-showcase](../../examples/05-ultimate-showcase/)** - Alle Features 2.5 (VelinAutoDoc, VelinPipeline, @Flow) ✅
- **[Custom Recommender](../../examples/custom-recommender/)** - Production-Ready Recommendation System

## Hilfe

- **[Quick Start Guide](../../QUICK_START.md)** - 5 Minuten bis zur ersten API
- **[API-Keys Setup](api-keys-setup.md)** - 🔑 API-Keys konfigurieren
- **[Language Specification](../language/specification.md)** - Vollständige Sprachspezifikation
- **[API Documentation](../api/)** - API-Referenz
- **[Dokumentations-Übersicht](../README.md)** - Alle Dokumente
- **[GitHub Issues](https://github.com/SkyliteDesign/velinscript/issues)** - Fragen und Bug Reports

## Häufige Probleme

### "Command not found: velin"

Stelle sicher, dass der Compiler gebaut wurde und im PATH ist:

```bash
cd compiler
cargo build --release
export PATH=$PATH:$(pwd)/target/release
```

**Hinweis:** Der Binary heißt `velin-compiler` (oder `velin-compiler.exe` auf Windows), aber der Befehl ist `velin`. Stelle sicher, dass der Binary im PATH ist oder erstelle einen Alias/Symlink.

### Type Errors

Nutze `velin check` um Type-Fehler zu finden:

```bash
velin check -i main.velin
```

**Beispiel-Ausgabe:**
```
❌ Type error: Type mismatch at line 12, column 15
📁 Datei: main.velin
📍 Position: Zeile 12, Spalte 15

💡 Did you mean: 'length'?
💡 Beispiel für explizite Typ-Annotation:
   let x: number = 42;
   let name: string = "John";

🔧 Lösungsvorschläge:
   - Prüfe die Typen deiner Variablen
   - Nutze explizite Typ-Annotationen bei Unsicherheit
   - Siehe: docs/guides/tutorial-1-basics.md
```

#### Parsing Errors

Prüfe die Syntax in der Language Specification.

**Beispiel-Ausgabe:**
```
❌ Parse error: Unexpected token at line 5, column 10
📁 Datei: main.velin
📍 Position: Zeile 5, Spalte 10

💡 Did you mean: 'fn'?
💡 Tip: Function declarations use 'fn', not 'function'
   Beispiel: fn myFunction(): string { return "hello"; }

🔧 Lösungsvorschläge:
   - Prüfe auf fehlende oder überflüssige Klammern
   - Nutze 'velin check --autofix' für automatische Korrekturen
   - Siehe: docs/language/specification.md
```

### Weitere Fehlertypen ✅ (Neu in 3.1.0)

Der Compiler bietet jetzt intelligente Lösungsvorschläge für alle Fehlertypen:

- **CodeGen Errors** - Code-Generierungsfehler mit spezifischen Hinweisen
- **IO Errors** - Datei-/IO-Fehler mit Berechtigungs- und Pfad-Hinweisen
- **Validation Errors** - Validierungsfehler mit Feld-spezifischen Tipps
- **Config Errors** - Konfigurationsfehler mit JSON-Syntax-Hilfen
- **Internal Errors** - Interne Compiler-Fehler mit Bug-Report-Hinweisen

**Siehe:** [Fehlerbehandlung & Lösungsvorschläge](../architecture/error-handling.md) ✅ (Neu in 3.1.0) für vollständige Dokumentation.
