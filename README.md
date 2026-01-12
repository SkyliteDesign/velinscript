# 🚀 VelinScript

Eine moderne Programmiersprache für KI-APIs, die zu Rust kompiliert.

## 📖 Was ist VelinScript?

VelinScript ist eine speziell für die Entwicklung von KI-APIs entwickelte Programmiersprache. Sie kombiniert die Einfachheit moderner Sprachen mit der Performance von Rust.

### ✨ Hauptmerkmale

- **🎯 KI-optimiert**: Eingebaute Features für Machine Learning, LLM-Integration und Vector Databases
- **⚡ High Performance**: Kompiliert zu nativem Rust-Code für maximale Geschwindigkeit
- **🔒 Security First**: Eingebaute Security-Features (Authentication, Rate Limiting, Input Validation)
- **📦 Standard Library**: Umfangreiche Standard-Bibliothek mit API-Funktionen, Caching, Logging und mehr
- **🛠️ Developer Experience**: Syntax-Highlighting, Auto-Completion, Formatting und mehr

## 🚀 Schnellstart

### Voraussetzungen

- **Rust** (Version 1.70 oder höher)
  - Installation: [rustup.rs](https://rustup.rs/)
  - Oder: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Installation

```bash
# Repository klonen
git clone https://github.com/SkyliteDesign/velinscript.git
cd velinscript

# Compiler bauen
cd compiler
cargo build --release

# Binary ist jetzt verfügbar unter:
# compiler/target/release/velin-compiler.exe (Windows)
# compiler/target/release/velin-compiler (Linux/Mac)
```

### Erste Schritte

```bash
# Neues Projekt erstellen
velin-compiler.exe init my-project

# In das Projekt-Verzeichnis wechseln
cd my-project

# Projekt kompilieren
velin-compiler.exe compile -i main.velin

# Code prüfen
velin-compiler.exe check -i main.velin

# Code formatieren
velin-compiler.exe format -i main.velin
```

## 📝 Beispiel

```velin
// Einfache API-Funktion
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript! 🚀";
}

// Mit Parametern
@POST("/api/users")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
    };
    return user;
}

// Struct-Definition
struct User {
    id: string,
    name: string,
    email: string,
}
```

## 🛠️ Verfügbare Befehle

```bash
# Kompilieren
velin-compiler.exe compile -i <datei> -o <output>

# Code prüfen (Parsing & Type Checking)
velin-compiler.exe check -i <datei>

# Code formatieren
velin-compiler.exe format -i <datei>

# Informationen anzeigen
velin-compiler.exe info -i <datei>

# Neues Projekt initialisieren
velin-compiler.exe init <projektname>

# Code generieren (Boilerplate, CRUD, etc.)
velin-compiler.exe generate <typ> --name <name>

# Tests ausführen
velin-compiler.exe test

# Konfiguration verwalten
velin-compiler.exe config init
velin-compiler.exe config show
velin-compiler.exe config validate

# Cache-Management
velin-compiler.exe cache stats
velin-compiler.exe cache clear

# Health Check
velin-compiler.exe health

# Backup-Management
velin-compiler.exe backup create
velin-compiler.exe backup list
velin-compiler.exe backup restore <id>

# Rollback-Management
velin-compiler.exe rollback begin
velin-compiler.exe rollback commit
velin-compiler.exe rollback rollback

# Serialization
velin-compiler.exe serialize json-to-yaml <input> <output>
velin-compiler.exe serialize yaml-to-json <input> <output>
```

## 📚 Dokumentation

- **[Vollständige Dokumentation](DOKUMENTATION.md)** - Umfassende Dokumentation aller Features
- **[Getting Started Guide](docs/guides/getting-started.md)** - Schritt-für-Schritt Anleitung
- **[API Dokumentation](docs/api/)** - API-Referenz
- **[Beispiele](examples/)** - Beispiel-Projekte

## 🏗️ Projekt-Struktur

```
velinscript/
├── compiler/          # Compiler Implementation
│   ├── src/
│   │   ├── parser/    # Parser & Lexer
│   │   ├── type_checker/  # Type Checking
│   │   ├── codegen/   # Code Generation
│   │   └── stdlib/    # Standard Library
│   └── Cargo.toml
├── tools/             # Entwickler-Tools
│   ├── lsp/          # Language Server Protocol
│   └── vscode-extension/  # VS Code Extension
├── docs/              # Dokumentation
├── examples/          # Beispiel-Projekte
└── README.md
```

## 🤝 Beitragen

Wir freuen uns über Beiträge! Bitte lese [CONTRIBUTING.md](CONTRIBUTING.md) für Details.

### Entwicklung

```bash
# Repository klonen
git clone https://github.com/SkyliteDesign/velinscript.git
cd velinscript

# Compiler bauen
cd compiler
cargo build

# Tests ausführen
cargo test

# Code formatieren
cargo fmt

# Linter ausführen
cargo clippy
```

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](LICENSE) für Details.

## 🔗 Links

- **GitHub**: https://github.com/SkyliteDesign/velinscript
- **Issues**: https://github.com/SkyliteDesign/velinscript/issues
- **Discussions**: https://github.com/SkyliteDesign/velinscript/discussions
- **ForumBirdApi**: https://forum.birdapi.de

## 🙏 Danksagungen

VelinScript wird von der Community entwickelt und verbessert. Vielen Dank an alle Contributors!

---

**Made with ❤️ by the VelinScript Community**
