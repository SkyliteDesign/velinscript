# Getting Started mit VelinScript

Willkommen bei VelinScript! Diese Anleitung führt dich durch die ersten Schritte.

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

# Binary ist in: compiler/target/release/velin-compiler
# Optional: In PATH einbinden
export PATH=$PATH:$(pwd)/target/release
```

#### Option 2: Installation Script (später verfügbar)

```bash
curl https://velinscript.dev/install.sh | sh
```

## Dein erstes Programm

### 1. Projekt initialisieren

```bash
velin-compiler init my-first-api
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
velin-compiler compile -i main.velin -o main.rs
```

Dies erstellt `main.rs` mit dem kompilierten Rust-Code.

### 4. Type Checking

```bash
velin-compiler check -i main.velin
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

- **Language Specification:** [../language/specification.md](../language/specification.md)
- **API Documentation:** [../api/](../api/)
- **GitHub Issues:** Für Fragen und Bug Reports

## Häufige Probleme

### "Command not found: velin"

Stelle sicher, dass der Compiler gebaut wurde und im PATH ist:

```bash
cd compiler
cargo build --release
export PATH=$PATH:$(pwd)/target/release
```

### Type Errors

Nutze `velin-compiler check` um Type-Fehler zu finden:

```bash
velin-compiler check -i main.velin
```

### Parsing Errors

Prüfe die Syntax in der Language Specification.
