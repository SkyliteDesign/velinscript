# 🚀 VelinScript 2.7

<div align="center">

```ascii
██╗   ██╗███████╗██╗     ██╗███╗   ██╗    ███████╗ ██████╗██████╗ ██╗██████╗ ████████╗
██║   ██║██╔════╝██║     ██║████╗  ██║    ██╔════╝██╔════╝██╔══██╗██║██╔══██╗╚══██╔══╝
██║   ██║█████╗  ██║     ██║██╔██╗ ██║    ███████╗██║     ██████╔╝██║██████╔╝   ██║   
╚██╗ ██╔╝██╔══╝  ██║     ██║██║╚██╗██║    ╚════██║██║     ██╔══██╗██║██╔══██╗   ██║   
 ╚████╔╝ ███████╗███████╗██║██║ ╚████║    ███████║╚██████╗██║  ██║██║██║  ██║   ██║   
  ╚═══╝  ╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝    ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝   ╚═╝   
                                                                                        
                    V E L I N S C R I P T  2.7.0
                    Velisch Eine moderne Programmiersprache für KI-APIs
```

**Eine moderne Programmiersprache für KI-APIs mit den Namen (Velisch), die zu Rust kompiliert.**

[![Version](https://img.shields.io/badge/version-2.7.0-blue?style=for-the-badge&logo=rust)](https://github.com/SkyliteDesign/velinscript)
[![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Build](https://github.com/SkyliteDesign/velinscript/workflows/CI/badge.svg?style=for-the-badge)](https://github.com/SkyliteDesign/velinscript/actions)
[![Stars](https://img.shields.io/github/stars/SkyliteDesign/velinscript?style=for-the-badge&logo=github)](https://github.com/SkyliteDesign/velinscript/stargazers)
[![Forks](https://img.shields.io/github/forks/SkyliteDesign/velinscript?style=for-the-badge&logo=github)](https://github.com/SkyliteDesign/velinscript/forks)
[![Issues](https://img.shields.io/github/issues/SkyliteDesign/velinscript?style=for-the-badge&logo=github)](https://github.com/SkyliteDesign/velinscript/issues)
[![Contributors](https://img.shields.io/github/contributors/SkyliteDesign/velinscript?style=for-the-badge&logo=github)](https://github.com/SkyliteDesign/velinscript/graphs/contributors)

</div>

---

## 🔗 Quick Links

<div align="center">

**[📚 Dokumentation](https://velinscript.birdapi.de/)** • **[💬 Forum & Support](https://forum.birdapi.de/forum/)** • **[📦 GitHub](https://github.com/SkyliteDesign/velinscript)** • **[🐛 Issues](https://github.com/SkyliteDesign/velinscript/issues)** • **[💡 Discussions](https://github.com/SkyliteDesign/velinscript/discussions)**

*Die Dokumentation enthält Tutorials, Quiz und interaktive Beispiele!*

</div>

---

## 📊 GitHub Statistics

<div align="center">

![GitHub Stats](https://github-readme-stats.vercel.app/api?username=SkyliteDesign&repo=velinscript&show_icons=true&theme=default&hide_border=true&count_private=true)
![Top Languages](https://github-readme-stats.vercel.app/api/top-langs/?username=SkyliteDesign&repo=velinscript&layout=compact&theme=default&hide_border=true)
![GitHub Streak](https://github-readme-streak-stats.demolab.com/?user=SkyliteDesign&repo=velinscript&theme=default&hide_border=true)

</div>

---

## 🎯 VelinScript(Velisch) 2.7

VelinScript 2.7 ist die nächste Generation einer speziell für KI-API-Entwicklung optimierten Programmiersprache namens Velisch. Sie kombiniert die Einfachheit moderner Sprachen mit der Performance von Rust und bietet eine umfassende Toolchain für professionelle API-Entwicklung.

**Neu in Version 2.6**: 5 neue Standard Library Module (path, url, stream, redis, tracing) mit 50+ Funktionen hinzugefügt ✅  
**Neu in Version 2.7**: Bibliotheks-Generator für automatische Standardbibliotheks-Generierung ✅

### ✨  Kernpunkte

<div align="center">

| Feature | Beschreibung | Status |
|---------|-------------|--------|
| 🎯 **KI-First Design** | Native Unterstützung für Machine Learning, LLM-Integration und Vector Databases | ✅ |
| ⚡ **Rust-Performance** | Kompiliert zu nativem Rust-Code für maximale Geschwindigkeit und Sicherheit | ✅ |
| 🔒 **Velin Security** | Eingebaute Security-Features von Anfang an (Auth, Rate Limiting, Validation) | ✅ |
| 🛠️ **Developer Excellence** | Vollständige Toolchain mit Linter, Formatter, Hot Reload und mehr | ✅ |
| 📚 **Erweiterte Standardbibliothek** | Neue Module für String, Math, Date, FS, Process, Agent, WebSocket und mehr (Neu in 2.5) | ✅ |

</div>

---

## 🤖 KI & Machine Learning Features

VelinScript 2.7 bietet native Unterstützung für moderne KI- und ML-Workflows:

### LLM Integration

- **LLMClient**: Native Unterstützung für OpenAI, Anthropic und lokale LLMs
- **Embedding Generation**: Automatische Embedding-Erstellung für Vector Search
- **Chat Completion**: Einfache Integration von Chat-Funktionalitäten
- **Streaming Support**: Echtzeit-Streaming von LLM-Responses

### Vector Databases

- **VectorDB Support**: Native Integration für Pinecone, Weaviate, Qdrant
- **Semantic Search**: Embedding-basierte Ähnlichkeitssuche
- **Hybrid Search**: Kombination aus Keyword- und Vector-Search
- **Automatic Indexing**: Automatische Indexierung von Embeddings

### Machine Learning

- **ModelLoader**: Laden und Verwenden von ML-Models mit integriertem VelinLogger und Metrics
- **TrainingService**: Framework für Model Training
  - ONNX Runtime Integration
  - TensorFlow Integration
  - Hyperparameter Tuning
  - Model Evaluation & Metrics
  - Model Versioning
  - Integriertes Logging und Performance Monitoring
- **LLMClient**: Native LLM-Integration mit VelinLogger und Metrics
- **VectorDB**: Vector Database Integration mit Monitoring und Error Handling
- **Inference Pipeline**: Optimierte Inferenz-Pipelines
- **Model Versioning**: Versionierung und Management von Models

### Beispiel: LLM Chat API

```velin
@POST("/api/chat")
@Auth
fn chat(message: string): string {
    let client = LLMClient.new("openai");
    let response = await client.complete({
        model: "gpt-4",
        messages: [{ role: "user", content: message }]
    });
    return response.content;
}
```

### Beispiel: Vector Search

```velin
@GET("/api/search")
fn search(query: string): List<Item> {
    let embeddings = await generateEmbeddings(query);
    let results = await vectorDB.search(embeddings, limit: 10);
    return results;
}
```

---

## 🏗️ Core Features

### Performance & Compilation

- **Native Rust Compilation**: VelinScript kompiliert zu optimiertem Rust-Code
- **Zero-Cost Abstractions**: Moderne Sprachfeatures ohne Performance-Einbußen
- **Advanced Optimizer**: ✅ Vollständig aktiviert - Function Inlining, Loop Unrolling, Dead Code Elimination, Constant Folding
- **Type Safety**: Starke Typisierung mit Type Inference für bessere Entwicklererfahrung

### Standard Library

- **API Standard Library**: Eingebaute Funktionen für REST-API-Entwicklung
- **Database Integration**: Native Support für Datenbankoperationen (SeaORM, SQL)
- **Security Library**: Production-Grade Auth mit JWT, OAuth2 und TOTP-Support ✅
- **ML Library**: Echte Integration von OpenAI/Anthropic/Gemini und In-Memory Vector Search ✅
- **Collections Library**: Umfangreiche Collections (List, Map, Set) mit funktionalen Methoden
- **HTTP Client Library**: Vollständige Client-Library für HTTP-Requests
- **Rate Limiting**: Erweiterte Rate Limiting Library mit verschiedenen Strategien (inkl. @RateLimit Decorator) ✅
- **DateTime Library**: Datum- und Zeit-Operationen (now, format, parse, etc.) ✅
- **Regex Library**: Reguläre Ausdrücke für Pattern-Matching (find, replace, match, etc.) ✅
- **Crypto Library**: Kryptografische Funktionen (SHA-256, UUID, Base64, etc.) ✅
- **ML/LLM Library**: Native Unterstützung für Machine Learning und LLM-Integration
- **Vector Database Library**: Integration für Vector Databases (Pinecone, Weaviate, Qdrant)
- **VelinLogger**: Strukturiertes Logging mit Context, JSON-Format und File-Rotation
- **Metrics Framework**: Performance Monitoring mit Counters, Gauges und Histograms
- **VelinError**: Umfassendes Error-Handling mit Context, Stack Traces und Recovery-Mechanismen
- **String Library** (Neu in 2.5): Erweiterte String-Manipulation (split, join, slugify, capitalize, etc.) ✅
- **Math Library** (Neu in 2.5): Mathematische Utilities (clamp, lerp, random_range, round_to, etc.) ✅
- **Date Library** (Neu in 2.5): Erweiterte Datum/Zeit-Funktionen (add_days, is_weekend, format_relative, etc.) ✅
- **FS Library** (Neu in 2.5): Dateisystem-Operationen (read_json, write_json, copy, get_size, etc.) ✅
- **LLM Library** (Neu in 2.5): KI/LLM-Integration (summarize, chat, translate, sentiment, etc.) ✅
- **Embedding Library** (Neu in 2.5): Vector Embedding Utilities (similarity, find_nearest, cluster, etc.) ✅
- **Agent Library** (Neu in 2.5): AI Agent Capabilities (memory.store, task.plan, think, etc.) ✅
- **Process Library** (Neu in 2.5): System-Prozess-Management (spawn, kill, status, get_output, etc.) ✅
- **Sandbox Library** (Neu in 2.5): Code-Ausführung und Validierung (run, lint, test, optimize, etc.) ✅
- **WebSocket Library** (Neu in 2.5): Echtzeit-Kommunikation (connect, send, on_message, etc.) ✅
- **Utils Library** (Neu in 2.5): Utility-Funktionen (uuid, sleep, retry, debounce, throttle, etc.) ✅
- **Log Library** (Neu in 2.5): Erweiterte Logging-Funktionen (trace, to_file, with_context, etc.) ✅
- **Config Library** (Neu in 2.5): Konfiguration und Environment-Management (get_env, load_dotenv) ✅
- **Flow Library** (Neu in 2.5): VelinFlow Runtime mit @Flow Decorator, automatischem Rollback/Commit ✅

### Package Management

- **Integrierter Package Manager** (`velin-pkg`): Dependency Management
- **Automatische Updates**: Dependency Update Checking mit Breaking Change Detection
- **Security Auditing**: Automatische Vulnerability-Erkennung

---

## 🎨 Language Features

### Moderne Sprachkonstrukte

- **Result<T, E>**: Explizite Fehlerbehandlung ohne Exceptions
- **Traits & Interfaces**: Polymorphismus und Code-Wiederverwendung
- **Generics mit Constraints**: Type-safe generische Programmierung
- **Pattern Matching**: Erweiterte Pattern Matching mit Guards, Range Patterns und Destructuring
- **Closure/Lambda Functions**: Funktionale Programmierung mit Type Inference
- **String Interpolation**: Format-Strings mit `{expression}` Syntax

### Erweiterte Pattern Matching

```velin
match (result) {
    Ok(value) if value > 0 => {
        return "positive";
    },
    0..=12 => {
        return "child";
    },
    "pending" | "processing" => {
        return "in progress";
    },
    User { name: "admin", role } => {
        return "admin access";
    },
    _ => {
        return "unknown";
    }
}
```

### Closure/Lambda Functions

```velin
let add = (a: number, b: number) => a + b;
let doubled = list.map((x: number) => x * 2);
let evens = list.filter((x: number) => x % 2 == 0);
```

### Collections Library

```velin
let list = List<number>([1, 2, 3, 4, 5]);
let evens = list.filter((x: number) => x % 2 == 0);
let sum = list.reduce((acc: number, x: number) => acc + x, 0);
let found = list.find((x: number) => x > 3);

let map = Map<string, number>();
map.set("key", 42);
let value = map.get("key");
```

### HTTP Client

```velin
let client = HttpClient.new();
let response = await client.get("https://api.example.com/users");
let data = response.json();
```

### String Interpolation

```velin
let name = "John";
let age = 30;
let message = "Hello, {name}! You are {age} years old.";
// Ergebnis: "Hello, John! You are 30 years old."

let x = 10;
let y = 20;
let result = "Sum: {x + y}";
// Ergebnis: "Sum: 30"
```

---

## 🛠️ Developer Experience & Tools

### Developer Tools

- **Linter (velin-lint)**: Code-Qualitätsanalyse mit Auto-Fix
  - Unused Variables Detection
  - Complexity Analysis
  - Naming Conventions
  - Erweiterbare Regel-Architektur

- **AutoFix Engine** (Neu in 2.5): ✅
  - Automatische Fehlerkorrektur während der Kompilierung
  - Behebt unausgeglichene Klammern automatisch
  - Korrigiert fehlende Funktionssignaturen
  - Repariert unvollständige Generic-Typen
  - Aktivierbar mit `--autofix` Flag
  - CLI-Integration: `velin compile --autofix` oder `velin check --autofix`

- **Code Formatter**: Vollständige Formatierung von VelinScript-Code
  - Konfigurierbare Formatierungsregeln
  - Unterstützung für alle Language Features
  - CLI-Integration (`velin format`)

- **Documentation Generator (velin-api-doc)**: 
  - JSDoc-Parsing für `///` Kommentare
  - HTML-Export
  - Interactive Docs (Swagger UI)
  - OpenAPI 3.0 Integration

- **Code Generation Tools** (Neu in 2.5): ✅
  - **Boilerplate Generator**: Automatische API- und CRUD-Code-Generierung
    - `velin generate api --name User --path /api/users`
    - `velin generate crud --name Product --fields "id:string,name:string,price:number"`
  - **Client Generator**: Generiert TypeScript/JavaScript/Rust Clients aus OpenAPI
    - `velin generate client --openapi api.json --language typescript`
  - **Framework Selector**: Automatische Erkennung und Codegen für Axum/Actix-Web

- **Bibliotheks-Generator** (Neu in 2.7): ✅
  - **Automatische Standardbibliotheks-Generierung**: Erstellt neue Bibliotheks-Module mit vollständiger Integration
    - `velin-library-generator generate --interactive` - Interaktiver Modus
    - `velin-library-generator generate --config library.yaml` - Mit YAML-Konfiguration
    - `velin-library-generator validate --config library.yaml` - Konfiguration validieren
  - **Vollständige Integration**: Automatische Integration in mod.rs, Type Checker, Code Generator
  - **Template-System**: 3 Modul-Typen (Simple Functions, Struct Based, Service Based)
  - **Test-Generierung**: Automatische Unit Tests für alle Funktionen
  - **Dokumentation**: Vollständige API-Dokumentation wird automatisch generiert
  - **Siehe**: [Bibliotheks-Generator Dokumentation](tools/library-generator/README.md)

- **VelinAutoDoc** (Neu in 2.5): ✅
  - Automatische Dokumentationsgenerierung aus `///` Doc-Comments
  - Strukturierte JSON-Exporte mit API-Dokumentation
  - LLM-freundliche Kontextinformationen für KI-gestützte Dokumentation
  - Integration mit `@VelinAutoDoc` Decorator
  - Knowledge Base Generation für RAG/LLM-Systeme

- **VelinAutoTest** (Neu in 2.5): ✅
  - Automatische Test-Generierung für Funktionen mit `@VelinAutoTest`
  - Generiert Mock-Daten basierend auf Funktionsparametern
  - Erstellt Test-Stubs mit grundlegenden Assertions
  - Integration in Codegen-Pipeline

- **VelinInsight** (Neu in 2.5): ✅
  - Code-Analyse und Qualitätsprüfung
  - Erkennt ungenutzte Structs
  - Identifiziert komplexe Funktionen (Statement Count > 20)
  - Findet redundante Datenbank-Queries
  - Integration mit `@VelinInsight` Decorator
  - VS Code Extension Integration

- **VelinPipeline** (Neu in 2.5): ✅
  - Pipeline-Optimizer für Datenfluss-Analyse
  - Automatische Erkennung parallelisierbarer async Blöcke
  - Codegen-Optimierung mit `tokio::join!` für unabhängige Operationen
  - Integration mit `@VelinPipeline` Decorator

- **Hot Reload (velin-hot-reload)**: 
  - Automatisches Neuladen bei Dateiänderungen
  - File System Watching
  - Watch-Mode und Server-Mode

- **Dead Code Detector**: Automatische Erkennung von ungenutztem Code

- **Debugger (velin-debugger)**: DAP Server für Debugging
  - Breakpoints setzen/entfernen
  - Step Over/Into/Out
  - Variable Inspection
  - Call Stack Navigation
  - Watch Expressions
  - VS Code Integration

- **Security Scanner (velin-security)**: 
  - Code-Scanning auf Security-Vulnerabilities
  - Dependency Vulnerability Scanner
  - CVE Database Integration (NVD API)
  - GitHub Security Advisories
  - OSV (Open Source Vulnerabilities) API

### Language Server Protocol (LSP)

- **Auto-Completion**: Intelligente Code-Vervollständigung
- **Go to Definition**: Navigation zu Definitionen
- **Find All References**: Alle Referenzen finden
- **Rename Symbol**: Symbol-Umbenennung
- **Code Actions**: Quick Fixes für häufige Fehler
- **Auto-Import Management**: Automatische Import-Organisierung

### VS Code Extension

**Status:** ✅ Vollständig implementiert

- **Syntax-Highlighting**: Vollständige Unterstützung für alle VelinScript-Features
- **Code Snippets**: Templates für @Flow, @VelinAutoDoc, @VelinPipeline, @VelinAutoTest, @VelinInsight
- **IntelliSense Support**: Auto-Completion, Go to Definition, Find All References
- **Error Diagnostics**: Echtzeit-Fehlererkennung
- **Code Formatting**: Integrierte Formatierung
- **Debugger Integration (DAP)**: ✅
  - Breakpoints Management
  - Variable Inspection
  - Call Stack Navigation
  - Watch Expressions
- **Commands**: 30+ Commands für Code-Generierung, Tests, Config, Backup, Rollback
- **LSP Integration**: Language Server Protocol für erweiterte IDE-Features

**Siehe:** [VS Code Extension Dokumentation](docs/tools/vscode-extension.md)

---

## 🔒 Security und Betrieb

### Security Framework

- **@Auth Decorator**: Automatische Authentifizierung
- **@Role Decorator**: Role-based Access Control
- **Input Validation**: Umfangreiches Validator Framework
- **Security Scanner (velin-security)**: Automatische Vulnerability-Erkennung
- **JWT/OAuth2**: Native Unterstützung für moderne Auth-Protokolle

### Rate Limiting

- **@RateLimit Decorator**: Decorator-basierte Rate Limiting mit Type-Checker-Validierung ✅
  - Unterstützte Argumente: `requests` (number), `window` (string), `strategy` (string), `key` (string, optional)
  - Strategien: `fixed-window`, `sliding-window`, `token-bucket`
- **Fixed Window Strategy**: Einfache Zeitfenster-basierte Begrenzung
- **Sliding Window Strategy**: Gleitende Zeitfenster
- **Token Bucket Strategy**: Token-basierte Rate Limiting
- **Distributed Rate Limiting**: Redis-basierte verteilte Rate Limiting

### Monitoring & Operations

- **Health Checks**: Eingebaute Health-Monitoring mit HealthCheck Framework
- **VelinLogger**: Strukturiertes Logging mit Context, JSON-Format, File-Rotation und Log-Levels
- **Metrics & Performance**: MetricsCollector und PerformanceMonitor für Application Metrics
- **VelinError**: Umfassendes Error-Handling mit Context, Stack Traces, Recovery-Mechanismen und Error Reporting
- **Backup & Rollback**: Transaktions-Management mit Rollback-Support
- **VelinFlow Runtime** (Neu in 2.5): ✅
  - Automatisches State-Tracking für transaktionale Flows
  - Input-Snapshot-Management
  - Automatisches Rollback/Commit bei Erfolg/Fehler
  - Compensation-Logic für Self-Healing
  - Integration mit `@Flow` Decorator

---

## 🏛️ Architektur & Design

### Modulare Architektur

VelinScript 2.7 folgt einer klaren, modularen Architektur für maximale Wartbarkeit und Skalierbarkeit:

```
velinscript/
├── compiler/              # Compiler Implementation
│   ├── parser/            # Parser & Lexer
│   ├── type_checker/      # Type Checking
│   ├── codegen/           # Code Generation
│   │   ├── autodoc.rs     # VelinAutoDoc Generator ✅
│   │   ├── autotest.rs    # VelinAutoTest Generator ✅
│   │   ├── boilerplate.rs # Boilerplate Generator ✅
│   │   ├── client.rs      # Client Generator ✅
│   │   └── framework.rs   # Framework Selector ✅
│   ├── optimizer/         # Advanced Optimizer
│   │   └── pipeline.rs    # VelinPipeline Optimizer ✅
│   ├── passes/            # Compiler Passes
│   │   ├── autofix.rs     # AutoFix Pass ✅
│   │   ├── parser.rs      # Parser Pass mit Modul-Auflösung ✅
│   │   ├── type_check.rs  # Type Check Pass
│   │   └── codegen.rs     # Codegen Pass
│   ├── analysis/          # Code Analysis
│   │   └── insight.rs     # VelinInsight Analyzer ✅
│   ├── autofix/           # AutoFix Engine ✅
│   ├── compiler/          # Core Compiler
│   │   ├── mod.rs         # VelinCompiler Core ✅
│   │   ├── config.rs      # CompilerConfig ✅
│   │   ├── context.rs      # CompilationContext ✅
│   │   └── pass.rs        # Pass Trait ✅
│   └── stdlib/            # Standard Library (50+ Module)
├── tools/                 # Entwickler-Tools
│   ├── lsp/              # Language Server Protocol
│   ├── vscode-extension/  # VS Code Extension ✅
│   ├── package-manager/   # Package Manager
│   ├── security-scanner/  # Security Scanner
│   ├── debugger/          # DAP Debugger Server
│   ├── linter/            # Linter Tool
│   ├── api-doc-generator/ # API Documentation Generator
│   └── hot-reload/        # Hot Reload Tool
├── docs/                  # Dokumentation
│   ├── architecture/      # Architektur-Dokumentation ✅
│   ├── api/               # API-Referenz
│   ├── guides/            # Tutorials
│   └── tools/             # Tools-Dokumentation ✅
└── examples/              # Beispiel-Projekte
    └── 05-ultimate-showcase/ # Ultimate Showcase ✅
```

**Siehe:** [Compiler Architektur Dokumentation](docs/architecture/compiler-architecture.md)

### Design-Prinzipien

1. **Einfachheit**: Klare, lesbare Syntax
2. **Type Safety**: Starke Typisierung mit Type Inference
3. **API-First**: Built-in Support für REST APIs
4. **Security First**: Security-Features von Anfang an
5. **KI/ML Ready**: Native Unterstützung für KI/ML-Integration

---

## ⚠️ Reifegrad & Status

**Aktueller Status: Beta (Production-Ready Features verfügbar)**

VelinScript 2.7 ist in aktiver Entwicklung. Der Compiler-Kern (Parser, Type Checker, Code Generator) ist funktionsfähig. Die Standard Library ist größtenteils nutzbar und getestet (50+ Module mit 150+ Funktionen registriert und validiert).

### ✅ Was funktioniert zuverlässig
- Parser für grundlegende Syntax (Funktionen, Structs, Enums, Decorators)
- Type Checker für einfache Typen und Standard Library Funktionen
- Code Generation zu Rust
- CLI-Befehle (`compile`, `check`, `format`, `init`)
- String-Interpolation
- Collections-Methoden (`List.length()`, `List.join()`, etc.)
- Pattern Matching (grundlegend)

### ⚠️ Was funktioniert mit Einschränkungen

#### Standard Library
- **Funktioniert und getestet**: 
  - Database (`db.find()`, `db.save()`, `db.findAll()`, `db.delete()`) ✅
  - File I/O (`file.read()`, `file.write()`, `file.exists()`) ✅
  - JSON (`json.parse()`, `json.stringify()`) ✅
  - HTTP Client (`HttpClient.new()`, `client.get()`, `client.post()`, etc.) ✅
  - Validation (`Validator.new()`, `validator.required()`, `validator.isValid()`, etc.) ✅
  - Auth (`AuthService.new()`, `authService.generateToken()`, `authService.verifyToken()`, etc.) ✅
  - Security (`@Auth` Decorator mit JWT-Validierung, `@Role` Decorator mit RBAC) ✅
  - Rate Limiting (In-Memory mit Cache, Redis-Support vorbereitet) ✅
  - Logging (`Logger.new()`, `VelinLogger.new()`, `logger.info()`, etc.) ✅
  - Metrics (`MetricsCollector.new()`, `collector.incrementCounter()`, etc.) ✅
  - ML/LLM (`LLMClient.new()`, `ModelLoader.new()`, `TrainingService.new()`, etc.) ✅
- **Status**: ~55+ Module mit über 200+ Funktionen sind im Type Checker registriert und vollständig getestet ✅
- **Neu in Version 2.5 registriert**: 
  - Rate Limiting Decorator (`@RateLimit`) mit vollständiger Validierung ✅
  - DateTime Module (`datetime.now()`, `datetime.format()`, `datetime.parse()`, etc.) ✅
  - Regex Module (`regex.find()`, `regex.replace()`, `regex.findAll()`, etc.) ✅
  - Crypto Module (`crypto.sha256()`, `crypto.uuid()`, `crypto.base64Encode()`, etc.) ✅
  - String Module (`string.split()`, `string.join()`, `string.slugify()`, etc.) ✅
  - Math Module (`math.clamp()`, `math.lerp()`, `math.random_range()`, etc.) ✅
  - Date Module (`date.add_days()`, `date.is_weekend()`, `date.format_relative()`, etc.) ✅
  - FS Module (`fs.read_json()`, `fs.write_json()`, `fs.copy()`, etc.) ✅
  - LLM Module (`llm.summarize()`, `llm.chat()`, `llm.translate()`, etc.) ✅
  - Embedding Module (`embedding.similarity()`, `embedding.find_nearest()`, etc.) ✅
  - Agent Module (`agent.memory.store()`, `agent.task.plan()`, etc.) ✅
  - Process Module (`process.spawn()`, `process.kill()`, `process.status()`, etc.) ✅
  - Sandbox Module (`sandbox.run()`, `sandbox.lint()`, `sandbox.test()`, etc.) ✅
  - WebSocket Module (`websocket.connect()`, `websocket.send()`, etc.) ✅
  - Utils Module (`utils.uuid()`, `utils.sleep()`, `utils.retry()`, etc.) ✅
  - Log Module (`log.trace()`, `log.to_file()`, `log.with_context()`, etc.) ✅
  - Config Module (`config.get_env()`, `config.load_dotenv()`) ✅
  - Flow Module (`@Flow` Decorator, `flow.snapshot_input()`, automatisches Rollback/Commit) ✅
- **Neu in Version 2.6 registriert**:
  - Path Module (`path.join()`, `path.dirname()`, `path.basename()`, `path.extname()`, etc.) ✅
  - URL Module (`url.parse()`, `url.protocol()`, `url.hostname()`, `url.parse_query()`, etc.) ✅
  - Stream Module (`stream.create()`, `stream.map()`, `stream.filter()`, `stream.reduce()`, etc.) ✅
  - Redis Module (`redis.connect()`, `redis.set()`, `redis.get()`, `redis.publish()`, etc.) ✅
  - Tracing Module (`tracing.start_span()`, `tracing.set_attribute()`, `tracing.child_span()`, etc.) ✅

#### ML/LLM Features
- **Funktioniert und getestet**: `LLMClient.new()`, `ModelLoader.new()`, `TrainingService.new()` sind registriert und getestet ✅
- **Funktioniert**: Methoden (`generate()`, `embed()`, `predict()`, `train()`) sind registriert und Type-Checking funktioniert ✅
- **Code Generation**: Alle ML/LLM Funktionen generieren korrekten Rust-Code ✅
- **LLM API Integration**: ✅ Vollständig implementiert mit echten API-Calls für:
  - OpenAI (Chat Completions, Embeddings)
  - Anthropic Claude (Messages API)
  - Google Gemini (Generate Content, Embeddings)
- **Vector Databases**: ✅ Vollständig implementiert für:
  - Pinecone (REST API Integration)
  - Weaviate (REST API & GraphQL Integration)
  - Qdrant (Native Rust Client & REST API Fallback)

#### Developer Tools
- **LSP Server**: ✅ Kompiliert ohne Warnungen, vollständig funktionsfähig
- **Debugger**: ✅ Kompiliert ohne Warnungen, DAP-Server vollständig implementiert
- **Package Manager**: ✅ Kompiliert ohne Warnungen, vollständig funktionsfähig
- **Security Scanner**: ✅ Kompiliert ohne Warnungen, vollständig funktionsfähig
- **API Doc Generator**: ✅ Kompiliert ohne Warnungen, vollständig funktionsfähig
- **Linter**: ✅ Kompiliert ohne Warnungen, vollständig funktionsfähig
- **Dead Code Detector**: ✅ Kompiliert ohne Warnungen, vollständig funktionsfähig
- **Hot Reload**: ✅ Kompiliert ohne Warnungen, vollständig funktionsfähig mit:
  - File Watching (kontinuierliches Monitoring)
  - Watch Mode (automatische Kompilierung bei Änderungen)
  - Server Mode (automatischer Server-Neustart mit Graceful Shutdown)

### 🚧 Was ist geplant

#### Kurzfristig (nächste Versionen)
- **Standard Library**: ✅ Alle wichtigen Funktionen registriert und getestet (50+ Module, 150+ Funktionen)
- **Developer Tools**: ✅ Alle Tools kompilieren ohne Warnungen, vollständig funktionsfähig
- **Code-Qualität**: ✅ Alle Warnungen behoben (unused imports, dead code, unreachable code)
- **Type Inference**: ✅ Verbessert für Konstruktor-Aufrufe ohne explizite Typen
- **VelinAutoDoc**: ✅ Vollständig implementiert - Automatische Dokumentationsgenerierung
- **VelinPipeline**: ✅ Vollständig implementiert - Pipeline-Optimierung mit automatischer Parallelisierung
- **VelinFlow**: ✅ Vollständig implementiert - Transaktionales Flow-Management mit Rollback

#### Mittelfristig
- **ML/LLM-Integration**: ✅ Vollständig implementiert - Echte API-Calls für OpenAI, Anthropic, Google Gemini
- **Vector Database Integration**: ✅ Vollständig implementiert - Pinecone, Weaviate, Qdrant Support
- **Security-Framework**: ✅ Vollständig implementiert - JWT/OAuth2, RBAC, Rate Limiting
- **Hot Reloading**: ✅ Vollständig implementiert - File Watching, Watch Mode, Server Mode
- **Vollständige Tool-Integration**: ✅ Alle Tools vollständig funktionsfähig
- **Standard Library Expansion**: ✅ 18 neue Module mit 167+ Funktionen in Version 2.5-2.6 hinzugefügt
  - Version 2.5: 13 Module (string, math, date, fs, llm, embedding, agent, process, sandbox, websocket, utils, log, config)
  - Version 2.6: 5 Module (path, url, stream, redis, tracing)
- **Bibliotheks-Generator** (Neu in 2.7): ✅ Vollständig implementiert
  - Automatische Generierung neuer Standardbibliotheks-Module
  - Interaktiver Modus und YAML-Konfiguration
  - Vollständige Integration in alle System-Komponenten

#### Langfristig
- **Production-Ready Status**: Beta-Release mit vollständiger Standard Library und getesteten Tools
- **Performance-Optimierung**: ✅ Advanced Optimizer vollständig aktiviert (inkl. LoopOptimization)
- **Dokumentation**: Vollständige API-Dokumentation für alle Features

**Für Production-Use wird Beta-Status empfohlen.**

---

## 🚀 Schnellstart

### Voraussetzungen

- **Rust** (Version 1.70 oder höher)
  - Installation: [rustup.rs](https://rustup.rs/)
  - Oder: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Installation

<details>
<summary><b>📦 Schritt 1: Repository klonen</b></summary>

```bash
$ git clone https://github.com/SkyliteDesign/velinscript.git
Cloning into 'velinscript'...
remote: Enumerating objects: 1234, done.
remote: Counting objects: 100% (1234/1234), done.
remote: Compressing objects: 100% (567/567), done.
remote: Total 1234 (delta 667), reused 1234 (delta 667), pack-reused 0
Receiving objects: 100% (1234/1234), 2.5 MiB | 1.2 MiB/s, done.
Resolving deltas: 100% (667/667), done.

$ cd velinscript
✓ Repository erfolgreich geklont
```

</details>

<details>
<summary><b>🔨 Schritt 2: Compiler bauen</b></summary>

```bash
$ cd compiler
$ cargo build --release
   Compiling velin-compiler v2.7.0
   Compiling dependencies...
   [========================================] 100%
   Finished release [optimized] target(s) in 45.2s

✓ Compiler erfolgreich gebaut
✓ Binary verfügbar unter: target/release/velin-compiler.exe (Windows)
                          target/release/velin-compiler (Linux/Mac)
```

</details>

### Erste Schritte

<details>
<summary><b>🎯 Neues Projekt erstellen</b></summary>

```bash
$ velin-compiler.exe init my-project
✓ Projekt-Verzeichnis erstellt: my-project
✓ Config-Datei generiert: velin.config.json
✓ Beispiel-Code hinzugefügt: main.velin
✓ README.md erstellt

$ cd my-project
✓ Projekt erfolgreich initialisiert
```

</details>

<details>
<summary><b>✅ Code prüfen</b></summary>

```bash
$ velin-compiler.exe check -i main.velin
🔍 Parsing main.velin...
  ✓ Syntax valid
  ✓ 3 Funktionen gefunden
  ✓ 1 Struct definiert

🔍 Type Checking...
  ✓ Alle Typen korrekt
  ✓ Standard Library Funktionen validiert
  ✓ Keine Fehler gefunden

✓ Code ist fehlerfrei!
```

</details>

<details>
<summary><b>🔧 Code kompilieren</b></summary>

```bash
$ velin-compiler.exe compile -i main.velin
🔨 Kompilierung gestartet...
  → Parsing... ✓
  → Type Checking... ✓
  → Code Generation... ✓
  → Optimierung... ✓

📦 Rust-Code generiert: target/main.rs
✓ Kompilierung erfolgreich abgeschlossen
```

</details>

<details>
<summary><b>✨ Code formatieren</b></summary>

```bash
$ velin-compiler.exe format -i main.velin
📝 Formatierung gestartet...
  → Einrückung korrigiert... ✓
  → Leerzeichen normalisiert... ✓
  → Zeilenumbrüche optimiert... ✓

✓ Code erfolgreich formatiert
```

</details>

<details>
<summary><b>🛠️ Code generieren (API, CRUD, Client)</b></summary>

```bash
$ velin-compiler.exe generate api --name User --path /api/users
📝 API-Boilerplate generiert...
  ✓ Endpoint: GET /api/users
  ✓ Endpoint: POST /api/users
  ✓ Endpoint: GET /api/users/:id
  ✓ Endpoint: PUT /api/users/:id
  ✓ Endpoint: DELETE /api/users/:id
  ✓ Struct User erstellt
  ✓ Validierung hinzugefügt

✓ API-Code erfolgreich generiert

$ velin-compiler.exe generate crud --name Product --fields "id:string,name:string,price:number"
📝 CRUD-Operationen generiert...
  ✓ Create Operation
  ✓ Read Operation
  ✓ Update Operation
  ✓ Delete Operation
  ✓ Struct Product erstellt

✓ CRUD-Code erfolgreich generiert
```

</details>

---

## 📝 Beispiel

```velin
// Einfache API-Funktion
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript 2.7! 🚀";
}

// Mit Parametern und Validation
@POST("/api/users")
@Auth
@RateLimit(requests: 100, window: "1m", strategy: "fixed-window")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
        createdAt: datetime.now(),
    };
    return user;
}

// DateTime, Regex und Crypto Beispiele
@GET("/api/timestamp")
fn getTimestamp(): string {
    let now = datetime.now();
    return datetime.formatISO8601(now);
}

@POST("/api/validate-email")
fn validateEmail(email: string): boolean {
    let result = regex.find("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$", email);
    return result.isOk();
}

@POST("/api/hash")
fn hashPassword(password: string): string {
    return crypto.sha256(password);
}

// Struct-Definition
struct User {
    id: string,
    name: string,
    email: string,
}

// Result Type für explizite Fehlerbehandlung
fn parseNumber(input: string): Result<number, string> {
    // ... Parsing-Logik
}

// Traits für Polymorphismus
trait Serialize {
    fn toJson(): string;
}

impl Serialize for User {
    fn toJson(): string {
        // ... Implementation
    }
}
```

---

## 🛠️ Verfügbare Befehle

### Compiler

<details>
<summary><b>🔨 Kompilieren</b></summary>

```bash
$ velin-compiler.exe compile -i main.velin -o target/main.rs
🔨 Kompilierung gestartet...
  → Parsing main.velin... ✓
  → Type Checking... ✓
  → Code Generation... ✓
  → Optimierung... ✓

📦 Rust-Code generiert: target/main.rs
✓ Kompilierung erfolgreich (0.45s)
```

</details>

<details>
<summary><b>✅ Code prüfen (Parsing & Type Checking)</b></summary>

```bash
$ velin-compiler.exe check -i main.velin
🔍 Parsing main.velin...
  ✓ Syntax valid
  ✓ 5 Funktionen gefunden
  ✓ 2 Structs definiert
  ✓ 1 Enum definiert

🔍 Type Checking...
  ✓ Alle Typen korrekt
  ✓ Standard Library Funktionen validiert
  ✓ Keine Fehler gefunden

✓ Code ist fehlerfrei!
```

</details>

<details>
<summary><b>✨ Code formatieren</b></summary>

```bash
$ velin-compiler.exe format -i main.velin
📝 Formatierung gestartet...
  → Einrückung korrigiert... ✓
  → Leerzeichen normalisiert... ✓
  → Zeilenumbrüche optimiert... ✓
  → Decorators formatiert... ✓

✓ Code erfolgreich formatiert
```

</details>

<details>
<summary><b>ℹ️ Informationen anzeigen</b></summary>

```bash
$ velin-compiler.exe info -i main.velin
📊 Projekt-Informationen:
  Datei: main.velin
  Größe: 2.5 KB
  Zeilen: 87
  Funktionen: 5
  Structs: 2
  Enums: 1
  Decorators: 8
  Standard Library Imports: 12
```

</details>

<details>
<summary><b>🚀 Neues Projekt initialisieren</b></summary>

```bash
$ velin-compiler.exe init my-project
✓ Projekt-Verzeichnis erstellt: my-project
✓ Config-Datei generiert: velin.config.json
✓ Beispiel-Code hinzugefügt: main.velin
✓ README.md erstellt
✓ .gitignore erstellt

✓ Projekt erfolgreich initialisiert
```

</details>

<details>
<summary><b>📄 OpenAPI Specification generieren</b></summary>

```bash
$ velin-compiler.exe open-api -i main.velin -o openapi.json
📄 OpenAPI-Spezifikation generiert...
  → Endpoints analysiert... ✓
  → Schemas extrahiert... ✓
  → Security Schemes erkannt... ✓
  → Dokumentation generiert... ✓

✓ OpenAPI 3.0 Spezifikation erstellt: openapi.json
```

</details>

<details>
<summary><b>🛠️ Code generieren (Boilerplate, CRUD, etc.)</b></summary>

```bash
$ velin-compiler.exe generate api --name User --path /api/users
📝 API-Boilerplate generiert...
  ✓ Endpoint: GET /api/users
  ✓ Endpoint: POST /api/users
  ✓ Endpoint: GET /api/users/:id
  ✓ Endpoint: PUT /api/users/:id
  ✓ Endpoint: DELETE /api/users/:id
  ✓ Struct User erstellt
  ✓ Validierung hinzugefügt

✓ API-Code erfolgreich generiert

$ velin-compiler.exe generate crud --name Product --fields "id:string,name:string,price:number"
📝 CRUD-Operationen generiert...
  ✓ Create Operation
  ✓ Read Operation
  ✓ Update Operation
  ✓ Delete Operation
  ✓ Struct Product erstellt

✓ CRUD-Code erfolgreich generiert

$ velin-compiler.exe generate client --openapi api.json --language typescript
📝 TypeScript Client generiert...
  ✓ API Client Klasse erstellt
  ✓ Type Definitions generiert
  ✓ Request/Response Handler

✓ TypeScript Client erfolgreich generiert
```

</details>

<details>
<summary><b>🔧 Mit AutoFix</b></summary>

```bash
$ velin-compiler.exe compile -i main.velin --autofix
🔨 Kompilierung mit AutoFix gestartet...
  → Parsing... ⚠️ Fehler gefunden
  → AutoFix aktiviert...
    ✓ Unausgeglichene Klammer behoben
    ✓ Fehlende Funktionssignatur korrigiert
  → Type Checking... ✓
  → Code Generation... ✓

✓ Kompilierung erfolgreich (AutoFix: 2 Korrekturen)
```

</details>

### Package Manager (velin-pkg)

<details>
<summary><b>📦 Projekt initialisieren</b></summary>

```bash
$ velin-pkg init my-project
✓ Package-Manager initialisiert
✓ velin.toml erstellt
✓ Dependencies-Verzeichnis erstellt

✓ Projekt bereit für Dependencies
```

</details>

<details>
<summary><b>➕ Dependency hinzufügen</b></summary>

```bash
$ velin-pkg add github.com/user/repo --version ^1.0.0
📦 Dependency hinzufügen...
  → Repository gefunden: github.com/user/repo
  → Version ^1.0.0 ausgewählt
  → Abhängigkeiten analysiert... ✓
  → velin.toml aktualisiert... ✓

✓ Dependency erfolgreich hinzugefügt
```

</details>

<details>
<summary><b>⬇️ Dependencies installieren</b></summary>

```bash
$ velin-pkg install
📦 Dependencies installieren...
  → 5 Dependencies gefunden
  → Downloading... [████████████] 100%
  → Kompilierung... ✓
  → Installation... ✓

✓ Alle Dependencies erfolgreich installiert
```

</details>

### Security Scanner (velin-security)

<details>
<summary><b>🔒 Code auf Security-Vulnerabilities scannen</b></summary>

```bash
$ velin-security scan --format text
🔍 Security-Scan gestartet...
  → Code analysiert... ✓
  → Vulnerabilities geprüft... ✓
  → Dependencies gescannt... ✓

✓ Scan abgeschlossen: 0 Vulnerabilities gefunden
```

</details>

<details>
<summary><b>🛡️ Dependencies auf Vulnerabilities prüfen</b></summary>

```bash
$ velin-security audit --config velin.toml
🔍 Dependency Audit gestartet...
  → CVE Database abgefragt... ✓
  → GitHub Security Advisories geprüft... ✓
  → OSV API abgefragt... ✓

✓ Audit abgeschlossen: Alle Dependencies sicher
```

</details>

**Features:**
- Code-Scanning auf Security-Vulnerabilities
- Dependency Vulnerability Scanner
- CVE Database Integration (NVD API)
- GitHub Security Advisories
- OSV (Open Source Vulnerabilities) API

### Debugger (velin-debugger)

<details>
<summary><b>🐛 DAP Server starten</b></summary>

```bash
$ velin-debugger start --port 4711
🐛 Debugger Server gestartet...
  → DAP Server läuft auf Port 4711
  → Breakpoints aktiviert
  → Variable Inspection bereit
  → VS Code Integration aktiv

✓ Debugger bereit für Verbindungen
```

</details>

**Features:**
- DAP (Debug Adapter Protocol) Server
- Breakpoints Management
- Variable Inspection
- Call Stack Navigation
- VS Code Integration

### Linter (velin-lint)

<details>
<summary><b>🔍 Code auf Linter-Probleme prüfen</b></summary>

```bash
$ velin-lint check --fix
🔍 Linter-Analyse gestartet...
  → Unused Variables gefunden: 2
  → Complexity Analysis... ✓
  → Naming Conventions geprüft... ✓
  → Auto-Fix aktiviert...
    ✓ 2 Probleme automatisch behoben

✓ Linter-Analyse abgeschlossen
```

</details>

### API Documentation Generator (velin-api-doc)

<details>
<summary><b>📄 OpenAPI Dokumentation generieren</b></summary>

```bash
$ velin-api-doc generate -i main.velin -o openapi.json --format json --interactive
📄 API-Dokumentation generiert...
  → Endpoints analysiert... ✓
  → Schemas extrahiert... ✓
  → Security Schemes erkannt... ✓
  → OpenAPI 3.0 generiert... ✓
  → Swagger UI vorbereitet... ✓

✓ Dokumentation erstellt: openapi.json
✓ Interactive Docs verfügbar
```

</details>

### Hot Reload (velin-hot-reload)

<details>
<summary><b>🔄 Dateien überwachen</b></summary>

```bash
$ velin-hot-reload --watch --directory .
🔄 Hot Reload aktiviert...
  → Watching: ./
  → File System Monitor gestartet
  → Kompilierung bei Änderungen aktiviert

✓ Hot Reload läuft (Drücke Ctrl+C zum Beenden)
```

</details>

<details>
<summary><b>🚀 Development Server starten</b></summary>

```bash
$ velin-hot-reload --server --port 3000
🚀 Development Server gestartet...
  → Server läuft auf http://localhost:3000
  → Hot Reload aktiviert
  → File Watching aktiv
  → Graceful Shutdown bereit

✓ Server bereit (Drücke Ctrl+C zum Beenden)
```

</details>

### AutoFix Engine

<details>
<summary><b>🔧 Automatische Fehlerkorrektur</b></summary>

```bash
$ velin-compiler.exe check -i main.velin --autofix
🔍 Code-Prüfung mit AutoFix...
  → Parsing... ⚠️ Fehler gefunden
  → AutoFix aktiviert...
    ✓ Unausgeglichene Klammer behoben: Zeile 23
    ✓ Fehlende Funktionssignatur korrigiert: Zeile 45
    ✓ Generic-Typ repariert: List<T → List<T>
  → Type Checking... ✓

✓ Code erfolgreich korrigiert (3 Fixes angewendet)
```

</details>

**Features:**
- Behebt unausgeglichene Klammern (`{`, `[`, `(`)
- Korrigiert fehlende Funktionssignaturen
- Repariert unvollständige Generic-Typen (`List<T` → `List<T>`)
- Iterative Korrekturen (bis zu 5 Durchläufe)

### Code Generation

<details>
<summary><b>📝 API Boilerplate generieren</b></summary>

```bash
$ velin-compiler.exe generate api --name User --path /api/users
📝 API-Boilerplate generiert...
  ✓ Endpoint: GET /api/users
  ✓ Endpoint: POST /api/users
  ✓ Endpoint: GET /api/users/:id
  ✓ Endpoint: PUT /api/users/:id
  ✓ Endpoint: DELETE /api/users/:id
  ✓ Struct User erstellt
  ✓ Validierung hinzugefügt

✓ API-Code erfolgreich generiert
```

</details>

<details>
<summary><b>🔄 CRUD Code generieren</b></summary>

```bash
$ velin-compiler.exe generate crud --name Product --fields "id:string,name:string,price:number"
📝 CRUD-Operationen generiert...
  ✓ Create Operation
  ✓ Read Operation
  ✓ Update Operation
  ✓ Delete Operation
  ✓ Struct Product erstellt

✓ CRUD-Code erfolgreich generiert
```

</details>

### VelinInsight (Code-Analyse)

<details>
<summary><b>🔍 Code-Analyse ausführen</b></summary>

```bash
$ velin-compiler.exe insight -i main.velin
🔍 VelinInsight Analyse gestartet...
  → Unused Structs gescannt... ✓
  → Complex Functions analysiert... ⚠️ 2 komplexe Funktionen gefunden
  → Redundant Queries geprüft... ✓
  → Code-Qualität bewertet... ✓

📊 Analyse-Ergebnisse:
  - Unused Structs: 0
  - Complex Functions: 2 (Statement Count > 20)
  - Redundant Queries: 0
  - Code-Qualität: Gut

✓ Analyse abgeschlossen
```

</details>

**Features:**
- Unused Structs Detection
- Complex Functions Detection (Statement Count > 20)
- Redundant Queries Detection
- Integration mit `@VelinInsight` Decorator

---

## 📚 Dokumentation

- **[Getting Started Guide](docs/guides/getting-started.md)** - Schritt-für-Schritt Anleitung
- **[Language Specification](docs/language/specification.md)** - Vollständige Sprachspezifikation
- **[API Dokumentation](docs/api/)** - API-Referenz
- **[Architektur-Dokumentation](docs/architecture/)** - Compiler-Architektur und Internals:
  - [Compiler Architektur](docs/architecture/compiler-architecture.md) - Pass-System und Core
  - [Modul-Auflösung](docs/architecture/module-resolution.md) - Wie Module aufgelöst werden
  - [Framework-Integration](docs/architecture/framework-integration.md) - Axum/Actix-Web Support
  - [Code-Generierung](docs/architecture/code-generation.md) - Codegen-System
- **[Tools Dokumentation](docs/tools/)** - Entwickler-Tools:
  - [VS Code Extension](docs/tools/vscode-extension.md) - IDE-Integration
  - [Bibliotheks-Generator](docs/tools/library-generator.md) - Automatische Bibliotheks-Generierung (Neu in 2.7) ✅
- **[Plugin Development](docs/guides/plugin-development.md)** - 🔌 Plugin-Entwicklung für VelinScript
- **[Tutorials](docs/guides/)** - Umfassende Tutorials:
  - [Pattern Matching](docs/guides/tutorial-pattern-matching.md) - Erweiterte Pattern Matching
  - [Closures](docs/guides/tutorial-closures.md) - Lambda Functions
  - [Collections](docs/guides/tutorial-collections.md) - Collections Library
  - [HTTP Client](docs/guides/tutorial-http-client.md) - HTTP Client Library
  - [ML & LLM](docs/guides/tutorial-7-ml.md) - Machine Learning & LLM Integration
  - [ML Training](docs/guides/tutorial-ml-training.md) - ML Model Training
  - [String Interpolation](docs/guides/tutorial-string-interpolation.md) - Format-Strings
  - [Debugger](docs/guides/tutorial-debugger.md) - Debugging in VS Code
  - [Intelligence Features](docs/guides/tutorial-8-intelligence.md) - VelinAutoDoc, VelinPipeline, @Flow, VelinInsight

---

## 🌟 Showcase-Beispiele

### 🎯 [Ultimate Showcase](examples/05-ultimate-showcase/) - Alle Features von VelinScript 2.7

**Neu in Version 2.5-2.7** ✅ - Das ultimative Beispiel, das alle neuen Features demonstriert:

Ein vollständiges E-Commerce-Backend, das die volle Leistungsfähigkeit von VelinScript 2.7 zeigt:

#### ✨ Features

- **📚 VelinAutoDoc**: Automatische Dokumentationsgenerierung aus `///` Doc-Comments
- **⚡ VelinPipeline**: Automatische Parallelisierung von unabhängigen async Operationen
- **🔄 @Flow**: Transaktionales Flow-Management mit automatischem Rollback/Commit
- **🏗️ Modulare Architektur**: Saubere Trennung in Models, Services, Security und Intelligence
- **🔒 Production-Ready Security**: Auth, Role-based Access Control, Input Validation
- **🤖 KI-Integration**: ML-Modelle nahtlos integriert
- **📊 Robuste Typisierung**: Starke Typisierung für Compile-Time-Fehlererkennung

#### 🚀 Schnellstart

```bash
# Beispiel-Projekt öffnen
cd examples/05-ultimate-showcase

# Projekt prüfen (Parsing & Type Checking)
velin-compiler check -i main.velin
```

#### 📖 Dokumentation

- **[README](examples/05-ultimate-showcase/README.md)** - Vollständige Projekt-Dokumentation

---

### 🎯 [Custom Recommender](examples/custom-recommender/) - Hybrid Recommendation System

Ein **production-ready Beispiel** für ein hybrides Recommendation System, das die volle Power von VelinScript 2.7 demonstriert:

Ein vollständiges, production-ready Beispiel für ein intelligentes Recommendation System mit **echten VectorDB-Integrationen** (Pinecone, Weaviate, Qdrant) und **echten LLM-API-Calls** (OpenAI, Anthropic, Google Gemini):

#### ✨ Features

- **🤖 Hybrid Algorithmus**: Kombiniert Embedding-basierte Empfehlungen (60%) mit Collaborative Filtering (40%)
- **🔍 Vector Database Integration**: ✅ Echte Integration mit Pinecone, Weaviate oder Qdrant für semantische Ähnlichkeitssuche
- **🧠 LLM-Integration**: ✅ Echte API-Calls zu OpenAI, Anthropic oder Google Gemini für Embedding-Generierung
- **🔒 Production-Ready Security**: 
  - API Key Authentication
  - Rate Limiting (100 requests/minute)
  - CORS Support
  - Input Validation
- **📊 Umfassende API**: 5 Endpoints für Recommendations, Preferences, User History, Feedback und Similar Items
- **🏗️ Modulare Architektur**: 15+ Module für Models, Security, Caching, Logging, Async Operations, WebAssembly Support
- **✅ Vollständige Tests**: Unit- und Integration-Tests inklusive
- **📚 Umfassende Dokumentation**: 
  - API-Dokumentation
  - Security-Guide
  - WebAssembly-Dokumentation
  - Optimierungs-Guide

#### 🚀 Schnellstart

```bash
# Beispiel-Projekt öffnen
cd examples/custom-recommender

# Konfiguration anpassen
cp velin.config.example.json velin.config.json

# Projekt kompilieren
velin-compiler compile -i main.velin

# Tests ausführen
velin-compiler test
```

#### 📖 Dokumentation

- **[README](examples/custom-recommender/README.md)** - Vollständige Projekt-Dokumentation
- **[API Endpoints](examples/custom-recommender/API_ENDPOINTS.md)** - API-Referenz
- **[Security Guide](examples/custom-recommender/SECURITY.md)** - Security-Best-Practices
- **[Optimization Guide](examples/custom-recommender/OPTIMIZATION.md)** - Performance-Optimierungen
- **[WebAssembly](examples/custom-recommender/WASM.md)** - WASM-Integration

#### 💡 Weitere Beispiele

- **[Ultimate Showcase](examples/05-ultimate-showcase/)** - Alle Features 2.5-2.7 (VelinAutoDoc, VelinPipeline, @Flow, Bibliotheks-Generator) ✅
- **[Hello API](examples/01-hello-api/)** - Einfaches Einstiegsbeispiel
- **[LLM Chat](examples/02-llm-chat/)** - Chat-API mit LLM-Integration
- **[Automation Pipeline](examples/03-automation-pipeline/)** - Automatisierungsbeispiel
- **[String Interpolation](examples/string-interpolation-example.velin)** - Format-Strings mit Expression-Interpolation
- **[ML Training](examples/ml-training-example.velin)** - ML Model Training mit ONNX und TensorFlow
- **[Debugger Example](examples/debugger-example.velin)** - Debugging mit Breakpoints und Variable Inspection
- **[LLM Chat API](examples/llm-chat-api.velin)** - Chat-API mit LLM-Integration
- **[ML Sentiment Analysis](examples/ml-sentiment-analysis.velin)** - Sentiment-Analyse mit ML
- **[Vector Search API](examples/vector-search-api.velin)** - Vector Database Integration
- **[Complete API with Auth](examples/complete-api-with-auth.velin)** - Vollständige API mit Authentication
- **[OAuth2 API](examples/oauth2-api.velin)** - OAuth2-Integration

---

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

---

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](LICENSE) für Details.

---

## 🔗 Links & Support

<div align="center">

### 📚 Dokumentation & Lernen

**[🌐 Vollständige Dokumentation](https://velinscript.birdapi.de/)** • **[📖 Getting Started](docs/guides/getting-started.md)** • **[🎓 Tutorials & Quiz](https://velinscript.birdapi.de/)** • **[📋 Language Specification](docs/language/specification.md)**

### 💬 Community & Support

**[💬 Forum & Support](https://forum.birdapi.de/forum/)** • **[🐛 Issues melden](https://github.com/SkyliteDesign/velinscript/issues)** • **[💡 Discussions](https://github.com/SkyliteDesign/velinscript/discussions)** • **[🤝 Contributing](CONTRIBUTING.md)**

### 🔗 Weitere Links

**[🌐 Website](https://birdapi.de)** • **[🏢 skylite.Design](https://skylite.design)** • **[📦 GitHub Repository](https://github.com/SkyliteDesign/velinscript)**

</div>

---

## 🙏 Danksagungen

<div align="center">

**VelinScript 2.7 wird von der Community entwickelt und verbessert. Vielen Dank an alle Contributors!**

[![Contributors](https://img.shields.io/github/contributors/SkyliteDesign/velinscript?style=for-the-badge&logo=github)](https://github.com/SkyliteDesign/velinscript/graphs/contributors)

</div>

---

<div align="center">

**Made with ❤️ by [skylite.Design](https://skylite.design)**

*Erfahren Sie mehr auf [birdapi.de](https://birdapi.de) | Support im [Forum](https://forum.birdapi.de/forum/) | Dokumentation auf [velinscript.birdapi.de](https://velinscript.birdapi.de/)*

</div>
