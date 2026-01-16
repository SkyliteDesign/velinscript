# 🚀 VelinScript 2.0

**Eine moderne Programmiersprache für KI-APIs, die zu Rust kompiliert.**

*Entwickelt von [skylite.Design](https://skylite.design) | Erfahren Sie mehr auf [birdapi.de](https://birdapi.de) | Support im [Forum](https://forum.birdapi.de)*

---

## 🎯 VelinScript 2

VelinScript 2.0 ist die nächste Generation einer speziell für KI-API-Entwicklung optimierten Programmiersprache. Sie kombiniert die Einfachheit moderner Sprachen mit der Performance von Rust und bietet eine umfassende Toolchain für professionelle API-Entwicklung.

### ✨  Kernpunkte

1. **🎯 KI-First Design** - Native Unterstützung für Machine Learning, LLM-Integration und Vector Databases ✅
2. **⚡ Rust-Performance** - Kompiliert zu nativem Rust-Code für maximale Geschwindigkeit und Sicherheit ✅
3. **🔒 Velin Security** - Eingebaute Security-Features von Anfang an (Auth, Rate Limiting, Validation) ✅
4. **🛠️ Developer Excellence** - Vollständige Toolchain mit Linter, Formatter, Hot Reload und mehr ✅

---

## 🤖 KI & Machine Learning Features

VelinScript 2.0 bietet native Unterstützung für moderne KI- und ML-Workflows:

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
- **Advanced Optimizer**: ✅ Vollständig aktiviert - Function Inlining, Loop Optimizations, Dead Code Elimination, Constant Folding
- **Type Safety**: Starke Typisierung mit Type Inference für bessere Entwicklererfahrung

### Standard Library

- **API Standard Library**: Eingebaute Funktionen für REST-API-Entwicklung
- **Database Integration**: Native Support für Datenbankoperationen (SeaORM, SQL)
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

- **Code Formatter**: Vollständige Formatierung von VelinScript-Code
  - Konfigurierbare Formatierungsregeln
  - Unterstützung für alle Language Features
  - CLI-Integration (`velin format`)

- **Documentation Generator (velin-api-doc)**: 
  - JSDoc-Parsing für `///` Kommentare
  - HTML-Export
  - Interactive Docs (Swagger UI)
  - OpenAPI 3.0 Integration

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

- Syntax-Highlighting
- IntelliSense Support
- Error Diagnostics
- Code Formatting
- Debugger Integration (DAP)
  - Breakpoints Management
  - Variable Inspection
  - Call Stack Navigation
  - Watch Expressions

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

---

## 🏛️ Architektur & Design

### Modulare Architektur

VelinScript 2.0 folgt einer klaren, modularen Architektur für maximale Wartbarkeit und Skalierbarkeit:

```
velinscript/
├── compiler/              # Compiler Implementation
│   ├── parser/            # Parser & Lexer
│   ├── type_checker/      # Type Checking
│   ├── codegen/           # Code Generation
│   ├── optimizer/         # Advanced Optimizer
│   └── stdlib/            # Standard Library
├── tools/                 # Entwickler-Tools
│   ├── lsp/              # Language Server Protocol
│   ├── vscode-extension/  # VS Code Extension
│   ├── package-manager/   # Package Manager
│   ├── security-scanner/  # Security Scanner
│   ├── debugger/          # DAP Debugger Server
│   ├── linter/            # Linter Tool
│   ├── api-doc-generator/ # API Documentation Generator
│   └── hot-reload/        # Hot Reload Tool
├── docs/                  # Dokumentation
└── examples/              # Beispiel-Projekte
```

### Design-Prinzipien

1. **Einfachheit**: Klare, lesbare Syntax
2. **Type Safety**: Starke Typisierung mit Type Inference
3. **API-First**: Built-in Support für REST APIs
4. **Security First**: Security-Features von Anfang an
5. **KI/ML Ready**: Native Unterstützung für KI/ML-Integration

---

## ⚠️ Reifegrad & Status

**Aktueller Status: Beta (Production-Ready Features verfügbar)**

VelinScript 2.0 ist in aktiver Entwicklung. Der Compiler-Kern (Parser, Type Checker, Code Generator) ist funktionsfähig. Die Standard Library ist größtenteils nutzbar und getestet (~25+ Funktionen registriert und validiert).

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
- **Status**: ~35+ Funktionen von 36 Modulen sind im Type Checker registriert und vollständig getestet ✅
- **Neu registriert**: 
  - Rate Limiting Decorator (`@RateLimit`) mit vollständiger Validierung ✅
  - DateTime Module (`datetime.now()`, `datetime.format()`, `datetime.parse()`, etc.) ✅
  - Regex Module (`regex.find()`, `regex.replace()`, `regex.findAll()`, etc.) ✅
  - Crypto Module (`crypto.sha256()`, `crypto.uuid()`, `crypto.base64Encode()`, etc.) ✅

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
- **Standard Library**: ✅ Alle wichtigen Funktionen registriert und getestet
- **Developer Tools**: ✅ Alle Tools kompilieren ohne Warnungen, vollständig funktionsfähig
- **Code-Qualität**: ✅ Alle Warnungen behoben (unused imports, dead code, unreachable code)
- **Type Inference**: ✅ Verbessert für Konstruktor-Aufrufe ohne explizite Typen

#### Mittelfristig
- **ML/LLM-Integration**: ✅ Vollständig implementiert - Echte API-Calls für OpenAI, Anthropic, Google Gemini
- **Vector Database Integration**: ✅ Vollständig implementiert - Pinecone, Weaviate, Qdrant Support
- **Security-Framework**: ✅ Vollständig implementiert - JWT/OAuth2, RBAC, Rate Limiting
- **Hot Reloading**: ✅ Vollständig implementiert - File Watching, Watch Mode, Server Mode
- **Vollständige Tool-Integration**: ✅ Alle Tools vollständig funktionsfähig

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

---

## 📝 Beispiel

```velin
// Einfache API-Funktion
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript 2.0! 🚀";
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

# OpenAPI Specification generieren
velin-compiler.exe open-api -i <datei> -o <output>

# Code generieren (Boilerplate, CRUD, etc.)
velin-compiler.exe generate <typ> --name <name>
```

### Package Manager (velin-pkg)

```bash
# Projekt initialisieren
velin-pkg init [name]

# Dependency hinzufügen
velin-pkg add github.com/user/repo [--version ^1.0.0]

# Dependencies installieren
velin-pkg install

# Dependencies aktualisieren
velin-pkg update [package]
```

### Security Scanner (velin-security)

```bash
# Code auf Security-Vulnerabilities scannen
velin-security scan [path] [--format json|html|text]

# Dependencies auf Vulnerabilities prüfen
velin-security audit [--config velin.toml]
```

**Features:**
- Code-Scanning auf Security-Vulnerabilities
- Dependency Vulnerability Scanner
- CVE Database Integration (NVD API)
- GitHub Security Advisories
- OSV (Open Source Vulnerabilities) API

### Debugger (velin-debugger)

```bash
# DAP Server starten
velin-debugger start [--port 4711]
```

**Features:**
- DAP (Debug Adapter Protocol) Server
- Breakpoints Management
- Variable Inspection
- Call Stack Navigation
- VS Code Integration

### Linter (velin-lint)

```bash
# Code auf Linter-Probleme prüfen
velin-lint check [path] [--fix] [--json]
```

### API Documentation Generator (velin-api-doc)

```bash
# Generiert OpenAPI Dokumentation
velin-api-doc generate -i main.velin -o openapi.json [--format json|yaml|markdown|html] [--interactive]
```

### Hot Reload (velin-hot-reload)

```bash
# Überwacht Dateien und kompiliert bei Änderungen
velin-hot-reload --watch [--directory .]

# Startet Development Server mit Hot Reload
velin-hot-reload --server [--port 3000]
```

---

## 📚 Dokumentation

- **[Getting Started Guide](docs/guides/getting-started.md)** - Schritt-für-Schritt Anleitung
- **[Language Specification](docs/language/specification.md)** - Vollständige Sprachspezifikation
- **[API Dokumentation](docs/api/)** - API-Referenz
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

---

## 🌟 Showcase: Custom Recommender

Ein **production-ready Beispiel** für ein hybrides Recommendation System, das die volle Power von VelinScript 2.0 demonstriert:

### 🎯 [Custom Recommender](examples/custom-recommender/) - Hybrid Recommendation System

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

- **🌐 Website**: [birdapi.de](https://birdapi.de)
- **📚 Dokumentation**:[https://velinscript.birdapi.de/]
- **💬 Forum & Support**: [forum.birdapi.de](https://forum.birdapi.de)
- **🏢 Entwickelt von**: [skylite.Design](https://skylite.design)
- **📦 GitHub**: https://github.com/SkyliteDesign/velinscript
- **🐛 Issues**: https://github.com/SkyliteDesign/velinscript/issues
- **💡 Discussions**: https://github.com/SkyliteDesign/velinscript/discussions

---

## 🙏 Danksagungen

VelinScript 2.0 wird von der Community entwickelt und verbessert. Vielen Dank an alle Contributors!

---

**Made with ❤️ by [skylite.Design](https://skylite.design)**

*Erfahren Sie mehr auf [birdapi.de](https://birdapi.de) | Support im [Forum](https://forum.birdapi.de)*
