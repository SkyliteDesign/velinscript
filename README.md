# 🚀 VelinScript 3.5.1

<div align="center">

```ascii
██╗   ██╗███████╗██╗     ██╗███╗   ██╗    ███████╗ ██████╗██████╗ ██╗██████╗ ████████╗
██║   ██║██╔════╝██║     ██║████╗  ██║    ██╔════╝██╔════╝██╔══██╗██║██╔══██╗╚══██╔══╝
██║   ██║█████╗  ██║     ██║██╔██╗ ██║    ███████╗██║     ██████╔╝██║██████╔╝   ██║   
╚██╗ ██╔╝██╔══╝  ██║     ██║██║╚██╗██║    ╚════██║██║     ██╔══██╗██║██╔══██╗   ██║   
 ╚████╔╝ ███████╗███████╗██║██║ ╚████║    ███████║╚██████╗██║  ██║██║██║  ██║   ██║   
  ╚═══╝  ╚══════╝╚══════╝╚═╝╚═╝  ╚═══╝    ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝   ╚═╝   
                                                                                        
                    V E L I N S C R I P T  3.5.1
                    Velisch - Eine moderne Programmiersprache für KI-APIs
```

**Velisch** ist die Sprache. **VelinScript** ist der Compiler. Einstieg und Docs: [velisch.info](https://velisch.info) (bzw. lokale Produktsite).

Schreibe kurze API-Beschreibungen mit Decorators wie `@GET` und `@Auth`. Der Compiler (`velin`) erzeugt daraus Rust-Code mit Axum-Router und Handlern.

### Umfang von 3.5.1

VelinScript 3.5.1 konzentriert sich auf den stabilen API-Entwicklungsweg mit Rust/Axum. Starten: `velin run main.velin` (Port 8080).

VelinScript 3.5 unterstützt mehrere Zielsprachen. Der stabile Laufzeitpfad in dieser Version ist Rust/Axum. Weitere Targets befinden sich im Entwicklungs- bzw. Experimentalstatus.

Weitere Funktionen wie zusätzliche Target-Runtimes, erweiterte Authentifizierung, AI-Sandbox-Ausführung und weitere Runtime-Module befinden sich außerhalb dieses 3.5.1-Stable-Umfangs.

Quick Start: [QUICK_START.md](QUICK_START.md) · Guides: [docs/guides/](docs/guides/) · Beispiele: [examples/](examples/)

[![Version](https://img.shields.io/badge/version-3.5.1-blue?style=for-the-badge&logo=rust)](https://github.com/SkyliteDesign/velinscript)
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

## 🎯 VelinScript 3.5.1 - API-Entwicklung mit Rust/Axum

VelinScript (Velisch) ist eine moderne, speziell für KI-API-Entwicklung optimierte Programmiersprache. Sie kombiniert die Einfachheit moderner Sprachen mit der Performance von Rust und bietet eine umfassende Toolchain für professionelle API-Entwicklung.

### 🆕 Neu in Version 3.5.1

- ✅ **`velin run`** — kompiliert nach Axum und startet den Server (Port **8080**)
- ✅ **HTTP-Matrix** — GET/POST/PUT/PATCH/DELETE mit Query, Path und Body; fehlender Pflicht-Query → **400**
- ✅ **`@Role` + `X-Role`** — 403 bei falscher/fehlender Rolle; `@Auth` weiter Header-Präsenz (**401**)
- ✅ **Interpolation** — IR-Rust `format!`; Simple Quotes ohne Interpolation
- ✅ **`.vel`-Lese-Alias** — offizielle Endung bleibt `.velin`

3.5.0 brachte die `velin`-CLI, HTTP → Rust/Axum und gezielt `@Auth`. Details: [CHANGELOG.md](CHANGELOG.md).

### ✨ Kernpunkte

<div align="center">

| Feature | Beschreibung | Status 3.5.1 |
|---------|-------------|--------------|
| ⚡ **Rust/Axum APIs** | Stabiler Laufzeitpfad für HTTP/`@Auth` | Stable |
| 🌐 **Weitere Targets** | Codegenerierung, Entwicklungs-/Experimentalstatus | Experimental |
| 🔒 **Security** | Header-`@Auth`, Secret-Scan, Validation | Stable (Basis) |
| 🛠️ **Toolchain** | `velin` CLI, Formatter, Check, LSP/VS Code | Stable |
| 🧠 **Type Inference / Ordering** | Typableitung und Code-Ordnung | Stable |

</div>

---

## 🤖 KI & Machine Learning Features

VelinScript 3.5.1 bietet im Stable-Pfad Rust/Axum-HTTP. Native KI-/ML-Workflows liegen außerhalb dieses Umfangs:

### LLM Integration

- **LLMClient**: Native Unterstützung für OpenAI, Anthropic, Google Gemini und lokale LLMs
- **Embedding Generation**: Automatische Embedding-Erstellung für Vector Search
- **Chat Completion**: Einfache Integration von Chat-Funktionalitäten
- **Streaming Support**: Echtzeit-Streaming von LLM-Responses
- **Prompt Optimizer**: Token-Reduktion für LLM-Prompts (Nutzen abhängig vom Prompt)

### Vector Databases

- **VectorDB Support**: Native Integration für Pinecone, Weaviate, Qdrant
- **Semantic Search**: Embedding-basierte Ähnlichkeitssuche
- **RAG (Retrieval Augmented Generation)**: Vollständige RAG-Implementierung
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
    let client = LLMClient.new(LLMProvider::OpenAI, apiKey);
    let response = await client.complete({
        model: "gpt-4",
        messages: [{ role: "user", content: message }]
    });
    return response.content;
}
```

### Beispiel: Vector Search & RAG

```velin
@POST("/api/search")
fn search(query: string): List<Document> {
    let llmClient = LLMClient.new(LLMProvider::OpenAI, apiKey);
    let vectorDB = VectorDB.new(VectorDBProvider::Pinecone, connectionString);
    
    // Embedding generieren
    let queryEmbedding = llmClient.embed(query);
    
    // Semantische Suche
    let results = vectorDB.search("documents", queryEmbedding, 10);
    
    return results.map(|r| db.find(Document, r.id));
}
```

---

## 🏗️ Core Features

### Performance & Compilation

- **Multi-Target Compilation**: Kompiliert zu mehreren Zielsprachen (HTTP-Referenz: Rust/Axum)
  - **Rust** (Default) - High-Performance, Type-Safe
  - **PHP** - Laravel, Symfony Support
  - **Python** - FastAPI, Flask Support
  - **TypeScript/JavaScript** - Express, NestJS Support
  - **Go** - Native Go Code Generation
  - **Java** - Spring Boot Support
  - **C#** - ASP.NET Core Support
- **Native Rust Compilation**: VelinScript kompiliert zu optimiertem Rust-Code
- **Zero-Cost Abstractions**: Moderne Sprachfeatures ohne Performance-Einbußen
- **Advanced Optimizer**: ✅ Vollständig aktiviert - Function Inlining, Loop Unrolling, Dead Code Elimination, Constant Folding
- **IR Representation**: SSA-Format für optimierte Code-Generierung
- **Borrow Checker**: Ownership & Borrowing System für Memory Safety
- **Type Safety**: Starke Typisierung mit Type Inference für bessere Entwicklererfahrung

### Type System

- **Type Inference**: Automatische Type-Inference mit Member-Access und Result-Type-Auflösung
- **Code Ordering**: Automatische Sortierung von Funktionen, Typen und Blöcken basierend auf Abhängigkeiten
- **Result<T, E>**: Explizite Fehlerbehandlung ohne Exceptions
- **Traits & Interfaces**: Polymorphismus und Code-Wiederverwendung
- **Generics mit Constraints**: Type-safe generische Programmierung
- **Pattern Matching**: Erweiterte Pattern Matching mit Guards, Range Patterns und Destructuring

### Standard Library

- Umfangreiche Modulbibliothek (siehe [Standard Library](docs/api/standard-library.md)); Status der Module reicht von nutzbar bis experimentell
- **API / HTTP**: Decorators und Codegen für REST (Rust/Axum Default-Pfad)
- **Auth (Basis)**: `@Auth` prüft den `Authorization`-Header (kein vollständiges JWT-Produkt)
- **ML / LLM**: experimentell / scaffold — nicht als stabilen Standardpfad behandeln
- **Collections, Logging, Metrics**: je nach Modul Template oder partiell nutzbar

**Liste und Ehrlichkeit:** [Standard Library API Reference](docs/api/standard-library.md)

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
  - Erweiterte Regel-Architektur

- **AutoFix Engine**: ✅
  - Automatische Fehlerkorrektur während der Kompilierung
  - Behebt unausgeglichene Klammern automatisch
  - Korrigiert fehlende Funktionssignaturen
  - Repariert unvollständige Generic-Typen
  - Aktivierbar mit `--autofix` Flag

- **Code Formatter**: Vollständige Formatierung von VelinScript-Code
  - Konfigurierbare Formatierungsregeln
  - Unterstützung für alle Language Features
  - CLI-Integration (`velin format`)

- **Documentation Generator (velin-api-doc)**: 
  - JSDoc-Parsing für `///` Kommentare
  - HTML-Export
  - Interactive Docs (Swagger UI)
  - OpenAPI 3.0 Integration

- **Code Generation Tools**: ✅
  - **Boilerplate Generator**: Automatische API- und CRUD-Code-Generierung
  - **Client Generator**: Generiert TypeScript/JavaScript/Rust Clients aus OpenAPI
  - **Framework Selector**: Automatische Erkennung und Codegen für alle unterstützten Frameworks

- **VelinAutoDoc**: ✅
  - Automatische Dokumentationsgenerierung aus `///` Doc-Comments
  - Strukturierte JSON-Exporte mit API-Dokumentation
  - LLM-freundliche Kontextinformationen für KI-gestützte Dokumentation
  - Integration mit `@VelinAutoDoc` Decorator

- **VelinAutoTest**: ✅
  - Automatische Test-Generierung für Funktionen mit `@VelinAutoTest`
  - Generiert Mock-Daten basierend auf Funktionsparametern
  - Erstellt Test-Stubs mit grundlegenden Assertions

- **VelinInsight**: ✅
  - Code-Analyse und Qualitätsprüfung
  - Erkennt ungenutzte Structs
  - Identifiziert komplexe Funktionen
  - Findet redundante Datenbank-Queries

- **VelinPipeline**: ✅
  - Pipeline-Optimizer für Datenfluss-Analyse
  - Automatische Erkennung parallelisierbarer async Blöcke
  - Codegen-Optimierung mit `tokio::join!` für unabhängige Operationen

- **Hot Reload (velin-hot-reload)**: 
  - Automatisches Neuladen bei Dateiänderungen
  - File System Watching
  - Watch-Mode und Server-Mode

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
- **Commands**: compile, check, format, generate, test, serve (Scaffold), backup, openapi, config
- **LSP Integration**: Language Server Protocol für erweiterte IDE-Features

**Siehe:** [VS Code Extension Dokumentation](docs/tools/vscode-extension.md)

### CLI-Referenz

**Vollständige CLI-Referenz verfügbar:** [CLI-Referenz](docs/guides/cli-reference.md)

**Hauptbefehle:**
- `velin run` - Kompilieren nach Axum und Server starten (Default-Port 8080)
- `velin compile` - Kompilierung mit Multi-Target Support
- `velin check` - Code-Prüfung (Parsing & Type Checking)
- `velin format` - Code-Formatierung
- `velin serve` - Schreibt nur ein Axum-Scaffold unter `.velin/serve-scaffold` (kein Server)
- `velin generate` - Code-Generierung (API, CRUD, Client, system)
- `velin test` - Parse-Check von `.velin`-Testdateien
- `velin config` - Config-Verwaltung
- `velin backup` - Sichert Dateien unter `.velin/backup`
- `velin cache` / `health` / `rollback` - nicht implementiert (kein Fake-Erfolg)

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
- **VelinFlow Runtime**: ✅
  - Automatisches State-Tracking für transaktionale Flows
  - Input-Snapshot-Management
  - Automatisches Rollback/Commit bei Erfolg/Fehler
  - Compensation-Logic für Self-Healing
  - Integration mit `@Flow` Decorator

---

## 🏛️ Architektur & Design

### Modulare Architektur

VelinScript 3.5.1 folgt einer klaren, modularen Architektur für maximale Wartbarkeit und Skalierbarkeit:

```
velinscript/
├── compiler/                    # Compiler Implementation (Rust)
│   ├── src/
│   │   ├── compiler/            # Orchestrierung & Kontext
│   │   │   ├── orchestrator.rs  # Multi-File Dependency Management
│   │   │   ├── context.rs       # Shared Compilation Metadata
│   │   │   ├── config.rs        # Compiler Configuration
│   │   │   ├── pass.rs          # Pass Trait Definition
│   │   │   ├── language.rs      # Language Identity Validation
│   │   │   └── error.rs         # Compiler Errors
│   │   ├── passes/              # Compiler Passes (Pipeline)
│   │   │   ├── autofix.rs       # Pass 1: Syntax Auto-Correction
│   │   │   ├── parser.rs        # Pass 2: AST & Module Resolution
│   │   │   ├── desugar.rs       # Pass 2.5: Desugaring
│   │   │   ├── code_order.rs    # Pass 3: Dependency-based Sorting
│   │   │   ├── type_check.rs    # Pass 4: Type Inference & Checking
│   │   │   ├── codegen.rs       # Pass 5: Code Generation
│   │   │   ├── ai_semantic.rs   # AI: Semantic Analysis
│   │   │   ├── ai_bug_detection.rs  # AI: Bug Detection
│   │   │   ├── ai_codegen.rs    # AI: Code Generation
│   │   │   ├── ai_optimization.rs   # AI: Optimization
│   │   │   ├── ai_code_review.rs    # AI: Code Review
│   │   │   └── ai_sandbox.rs    # AI: Sandbox Testing
│   │   ├── parser/              # Parser Module (separate)
│   │   │   ├── lexer.rs         # Lexical Analysis
│   │   │   ├── parser.rs        # Syntax Parser
│   │   │   └── ast.rs           # Abstract Syntax Tree
│   │   ├── type_checker/        # Type System (separate)
│   │   │   ├── checker.rs       # Type Checker Implementation
│   │   │   ├── environment.rs   # Type Environment
│   │   │   └── errors.rs        # Type Errors
│   │   ├── borrow/              # Ownership & Lifetimes (separate)
│   │   │   ├── checker.rs       # Borrow Checker
│   │   │   ├── ownership.rs     # Ownership Rules
│   │   │   └── lifetime.rs      # Lifetime Analysis
│   │   ├── ir/                  # Intermediate Representation
│   │   │   ├── ir.rs            # IR Definition
│   │   │   ├── builder.rs       # IR Builder
│   │   │   ├── optimizer.rs     # IR Optimizer
│   │   │   └── validator.rs     # IR Validator
│   │   ├── codegen/             # Multi-Target Generatoren
│   │   │   ├── rust.rs          # Rust Backend
│   │   │   ├── typescript.rs    # TS/Express/NestJS Backend
│   │   │   ├── java.rs          # Java/Spring Backend
│   │   │   ├── csharp.rs        # C#/ASP.NET Backend
│   │   │   ├── python.rs        # Python Backend
│   │   │   ├── go.rs            # Go Backend
│   │   │   ├── php.rs           # PHP Backend
│   │   │   ├── javascript.rs    # JavaScript Backend
│   │   │   ├── openapi.rs       # OpenAPI Generator
│   │   │   ├── framework.rs     # Framework Detection
│   │   │   ├── infrastructure.rs # Infrastructure Code
│   │   │   ├── distributed.rs   # Distributed Systems
│   │   │   ├── system_generator.rs # System Generation
│   │   │   ├── autodoc.rs       # Auto Documentation
│   │   │   ├── autotest.rs      # Auto Test Generation
│   │   │   ├── boilerplate.rs   # Boilerplate Generation
│   │   │   ├── client.rs        # Client Generation
│   │   │   ├── ir_codegen.rs    # IR-based Codegen
│   │   │   ├── traits.rs        # Codegen Traits
│   │   │   └── templates/       # Code Templates
│   │   │       ├── api_server.rs
│   │   │       ├── auth.rs
│   │   │       ├── deployment.rs
│   │   │       ├── rate_limit.rs
│   │   │       └── ai_client.rs
│   │   ├── optimizer/           # Parallelisierung & Performance
│   │   │   ├── pipeline.rs      # Pipeline Optimization
│   │   │   ├── parallelization.rs # Parallelization Analysis
│   │   │   ├── profiling.rs     # Performance Profiling
│   │   │   └── learning.rs      # Learning-based Optimization
│   │   ├── analysis/            # Code Analysis
│   │   │   ├── insight.rs       # Code Insights
│   │   │   └── mod.rs
│   │   ├── autofix/             # Auto-Fix Module
│   │   │   ├── mod.rs
│   │   │   └── report.rs        # Fix Reports
│   │   ├── formatter/           # Code Formatter
│   │   │   ├── formatter.rs     # Formatting Logic
│   │   │   ├── config.rs        # Format Config
│   │   │   └── mod.rs
│   │   ├── prompt/              # AI Prompt Management
│   │   │   ├── optimizer.rs     # Prompt Optimization
│   │   │   ├── sanitizer.rs     # Prompt Sanitization
│   │   │   └── mod.rs
│   │   ├── error/               # Error Handling
│   │   │   └── suggestions.rs   # Error Suggestions
│   │   ├── stdlib/              # Standard Library (VelinScript)
│   │   │   ├── ml.rs            # LLM & AI Module
│   │   │   ├── llm.rs           # LLM Integration
│   │   │   ├── embedding.rs     # Embeddings
│   │   │   ├── nlp.rs           # NLP
│   │   │   ├── net.rs           # HTTP & Networking
│   │   │   ├── http.rs          # HTTP Client/Server
│   │   │   ├── http_client.rs   # HTTP Client
│   │   │   ├── websocket.rs     # WebSocket
│   │   │   ├── sys.rs           # System & Metrics
│   │   │   ├── metrics.rs       # Metrics
│   │   │   ├── logging.rs       # Logging
│   │   │   ├── tracing.rs       # Tracing
│   │   │   ├── process.rs       # Process Management
│   │   │   ├── env.rs           # Environment
│   │   │   ├── database.rs      # Database
│   │   │   ├── mongodb.rs       # MongoDB
│   │   │   ├── redis.rs         # Redis
│   │   │   ├── seaorm.rs        # SeaORM
│   │   │   ├── cache.rs         # Caching
│   │   │   ├── queue.rs         # Queue
│   │   │   ├── auth.rs          # Authentication
│   │   │   ├── oauth2.rs        # OAuth2
│   │   │   ├── security.rs      # Security
│   │   │   ├── encryption.rs    # Encryption
│   │   │   ├── crypto.rs        # Cryptography
│   │   │   ├── tls.rs           # TLS
│   │   │   ├── privacy.rs       # Privacy
│   │   │   ├── vault.rs         # Vault
│   │   │   ├── validation.rs    # Validation
│   │   │   ├── collections.rs   # Collections
│   │   │   ├── string.rs        # String Utils
│   │   │   ├── math.rs          # Math
│   │   │   ├── date.rs          # Date
│   │   │   ├── datetime.rs      # DateTime
│   │   │   ├── json.rs          # JSON
│   │   │   ├── yaml.rs          # YAML
│   │   │   ├── csv.rs           # CSV
│   │   │   ├── regex.rs         # Regex
│   │   │   ├── url.rs           # URL
│   │   │   ├── path.rs          # Path
│   │   │   ├── fileio.rs        # File I/O
│   │   │   ├── fs.rs            # Filesystem
│   │   │   ├── encoding.rs      # Encoding
│   │   │   ├── serialization.rs # Serialization
│   │   │   ├── template.rs      # Templates
│   │   │   ├── stream.rs       # Streams
│   │   │   ├── iterators.rs    # Iterators
│   │   │   ├── async_ops.rs    # Async Operations
│   │   │   ├── result.rs       # Result Types
│   │   │   ├── utils.rs        # Utilities
│   │   │   ├── extensions.rs   # Extensions
│   │   │   ├── flow.rs         # Control Flow
│   │   │   ├── workflow.rs     # Workflows
│   │   │   ├── scheduler.rs    # Scheduler
│   │   │   ├── event_bus.rs    # Event Bus
│   │   │   ├── api.rs          # API Utils
│   │   │   ├── config.rs       # Configuration
│   │   │   ├── fixtures.rs     # Test Fixtures
│   │   │   ├── mocks.rs        # Mocks
│   │   │   ├── testing.rs      # Testing
│   │   │   ├── test_module.rs  # Test Module
│   │   │   ├── sandbox.rs      # Sandbox
│   │   │   ├── audit.rs        # Audit
│   │   │   ├── alerting.rs     # Alerting
│   │   │   ├── backup.rs       # Backup
│   │   │   ├── rollback.rs     # Rollback
│   │   │   ├── rate_limit.rs   # Rate Limiting
│   │   │   ├── smtp.rs         # SMTP
│   │   │   ├── actix.rs        # Actix Framework
│   │   │   ├── axum.rs         # Axum Framework
│   │   │   ├── agent.rs        # Agent Framework
│   │   │   └── mod.rs
│   │   ├── cli.rs               # CLI Interface
│   │   ├── lib.rs               # Library Root
│   │   └── main.rs              # Binary Entry Point
│   └── Cargo.toml
├── tools/                       # CLI, LSP & Dev-Tools
│   ├── cli/                     # Command Line Interface
│   ├── lsp/                     # Language Server Protocol
│   ├── ide/                     # IDE Integration
│   ├── repl/                    # REPL
│   ├── debugger/                # Debugger
│   ├── profiler/                # Profiler
│   ├── linter/                  # Linter
│   ├── test-runner/             # Test Runner
│   ├── benchmark-runner/        # Benchmark Runner
│   ├── dependency-graph/        # Dependency Graph
│   ├── dead-code-detector/      # Dead Code Detector
│   ├── bundle-analyzer/         # Bundle Analyzer
│   ├── security-scanner/        # Security Scanner
│   ├── runtime-inspector/       # Runtime Inspector
│   ├── hot-reload/              # Hot Reload
│   ├── library-generator/       # Library Generator
│   ├── api-doc-generator/       # API Doc Generator
│   ├── package-manager/         # Package Manager
│   ├── example-plugin/          # Example Plugin
│   └── vscode-extension/        # VS Code Extension
├── docs/                        # Dokumentation
├── examples/                    # Beispiel-Projekte
├── tests/                       # Tests
└── README.md
```

**Siehe:** [Compiler Architektur Dokumentation](docs/architecture/compiler-architecture.md)

### Compiler Passes

VelinScript verwendet ein Pass-basiertes System mit 13 Compiler-Passes:

1. **AutoFixPass** - Automatische Fehlerkorrektur
2. **ParserPass** - Lexikalische und Syntax-Analyse
3. **DesugaringPass** - Syntaktischer Zucker → Core-Syntax
4. **CodeOrderingPass** - Automatische Sortierung basierend auf Abhängigkeiten
5. **AISemanticPass** - KI-Semantik-Analyse
6. **AIBugDetectionPass** - KI-Bug-Erkennung
7. **TypeCheckPass** - Type Checking & Inference
8. **ParallelizationAnalyzer** - Parallelisierungs-Analyse
9. **AICodeGenerationPass** - KI-Code-Generierung
10. **AICodeReviewPass** - KI-Code-Review
11. **AISandboxPass** - Statische Validierung KI-generierten Codes (keine Runtime-Execution)
12. **AIOptimizationPass** - KI-Optimierung
13. **CodegenPass** - Multi-Target Code-Generierung

**Siehe:** [Pass-Verlauf Dokumentation](docs/architecture/pass-verlauf.md)

### Design-Prinzipien

1. **Einfachheit**: Klare, lesbare Syntax
2. **Type Safety**: Starke Typisierung mit Type Inference
3. **API-First**: Built-in Support für REST APIs
4. **Security First**: Security-Features von Anfang an
5. **KI/ML Ready**: Native Unterstützung für KI/ML-Integration
6. **Multi-Target**: Mehrere Zielsprachen (HTTP-APIs empfohlen mit Rust/Axum)
7. **Performance**: Zero-Cost Abstractions und Advanced Optimizer

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
$ cd velinscript
```

</details>

<details>
<summary><b>🔨 Schritt 2: Compiler bauen</b></summary>

```bash
$ cd compiler
$ cargo build --release
```

</details>

<details>
<summary><b>🎯 Schritt 3: Neues Projekt erstellen</b></summary>

```bash
$ velin init my-project
$ cd my-project
```

</details>

<details>
<summary><b>✅ Schritt 4: Code prüfen</b></summary>

```bash
$ velin check -i main.velin
```

</details>

<details>
<summary><b>🔧 Schritt 5: Code kompilieren</b></summary>

```bash
# Zu Rust (Default)
$ velin compile -i main.velin

# Zu PHP
$ velin compile -i main.velin --target php

# Zu Python
$ velin compile -i main.velin --target python
```

</details>

---

## 📝 Beispiel

```velin
// Einfache API-Funktion
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript 3.5.1!";
}

// Mit Parametern, Validation und Rate Limiting
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

// Pattern Matching
match (result) {
    Ok(value) => return value,
    Err(error) => return 0,
}
```

---

## 📚 Dokumentation

### Getting Started

- **[Getting Started Guide](docs/guides/getting-started.md)** - Schritt-für-Schritt Anleitung
- **[Language Specification](docs/language/specification.md)** - Vollständige Sprachspezifikation
- **[Dokumentations-Index](docs/DOKUMENTATIONS-INDEX.md)** ✅ - Vollständiger Index aller Dokumentationen

### Architektur

- **[Compiler Architektur](docs/architecture/compiler-architecture.md)** - Pass-System und Core
- **[Pass-Verlauf](docs/architecture/pass-verlauf.md)** - Alle 13 Compiler-Passes
- **[Type Inference](docs/architecture/type-inference.md)** - Type-Inference System
- **[Code Ordering](docs/architecture/code-ordering-pass.md)** - Automatische Code-Sortierung
- **[IR Representation](docs/architecture/ir-representation.md)** - SSA-Format
- **[Borrow Checker](docs/architecture/borrow-checker.md)** - Ownership & Borrowing
- **[Code-Generierung](docs/architecture/code-generation.md)** - Multi-Target Codegen

### Guides & Tutorials

- **[Pattern Matching](docs/guides/tutorial-pattern-matching.md)** - Erweiterte Pattern Matching
- **[Closures](docs/guides/tutorial-closures.md)** - Lambda Functions
- **[Collections](docs/guides/tutorial-collections.md)** - Collections Library
- **[ML & LLM](docs/guides/tutorial-7-ml.md)** - Machine Learning & LLM Integration
- **[Vektor-Datenbanken](docs/guides/vektor-datenbanken.md)** ✅ - Semantische Suche & RAG
- **[Type Inference](docs/guides/tutorial-type-inference.md)** - Type-Inference Tutorial
- **[CLI-Referenz](docs/guides/cli-reference.md)** ✅ - Vollständige CLI-Referenz

### Tools

- **[VS Code Extension](docs/tools/vscode-extension.md)** - IDE-Integration
- **[Auto-Repair](docs/tools/auto-repair.md)** - AutoFix Engine
- **[Security Scanner](docs/tools/security-scanner.md)** - Security-Tools

**Vollständige Dokumentation:** [docs/README.md](docs/README.md)

---

## 🌟 Showcase-Beispiele

### 🎯 [Ultimate Showcase](examples/05-ultimate-showcase/) - Alle Features

Das ultimative Beispiel, das Features aus der VelinScript-3.5-Linie demonstriert:

- **📚 VelinAutoDoc**: Automatische Dokumentationsgenerierung
- **⚡ VelinPipeline**: Automatische Parallelisierung
- **🔄 @Flow**: Transaktionales Flow-Management
- **🏗️ Modulare Architektur**: Saubere Trennung
- **🔒 Security**: Auth, RBAC, Validation
- **🤖 KI-Integration**: ML-Modelle nahtlos integriert

### 🎯 [Custom Recommender](examples/custom-recommender/) - Hybrid Recommendation System

Production-ready Beispiel für ein intelligentes Recommendation System:

- **🤖 Hybrid Algorithmus**: Embedding-basierte + Collaborative Filtering
- **🔍 Vector Database Integration**: Pinecone, Weaviate, Qdrant
- **🧠 LLM-Integration**: OpenAI, Anthropic, Google Gemini
- **🔒 Security**: API Key Auth, Rate Limiting, CORS
- **📊 Umfassende API**: 5 Endpoints für Recommendations

**Weitere Beispiele:** [examples/](examples/)

---

## ⚠️ Reifegrad & Status

**Aktueller Status: 3.5.1 (Rust/Axum-HTTP-Pfad getestet)**

VelinScript 3.5.1: Der Compiler-Kern und der Default-Pfad IR → Rust/Axum (Hello, Query, Auth/Role, `velin run`) sind nachweisbar nutzbar. Weitere Targets und viele Stdlib-Module sind experimentell oder partiell — siehe Standard-Library-Doku.

### Was zuverlässig getestet ist

- Parser für grundlegende Syntax (Funktionen, Structs, Decorators)
- Type Checker für typische Hello/API-Fälle
- Rust/Axum HTTP GET/POST/PUT/PATCH/DELETE inkl. Query/Path/Body; fehlender Pflicht-Query → 400
- `@Auth` Header-Präsenz (401 ohne Header); `@Role` + `X-Role` (403 bei falscher/fehlender Rolle)
- String-Interpolation (`format!` in Rust); Simple Quotes ohne Interpolation
- CLI: `run` (Prozess + Port + HTTP), `compile`, `check`, `format`, `init`/`new`, `generate system` (Scaffold)
- Multi-Target: Code-Emit (kein gleichwertiger Runtime-Pfad außer Rust)
- VS Code: Sprache `.velin`; LSP Parse- und Type/Security-Diagnostics

**Beta-Status empfohlen.** Stdlib nicht pauschal als „Production-Grade“ behandeln.

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

**VelinScript 3.5.1 wird von der Community entwickelt und verbessert. Vielen Dank an alle Contributors!**

[![Contributors](https://img.shields.io/github/contributors/SkyliteDesign/velinscript?style=for-the-badge&logo=github)](https://github.com/SkyliteDesign/velinscript/graphs/contributors)

</div>

---

<div align="center">

**Made with ❤️ by [skylite.Design](https://skylite.design)**

*Erfahren Sie mehr auf [birdapi.de](https://birdapi.de) | Support im [Forum](https://forum.birdapi.de/forum/) | Dokumentation auf [velinscript.birdapi.de](https://velinscript.birdapi.de/)*

</div>
