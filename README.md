# VelinScript 1.9.3 
# Skylite.Deisgn - Birdapi.de
#

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![GitHub Actions](https://img.shields.io/badge/GitHub_Actions-enabled-blue.svg)](https://github.com/features/actions)

Eine moderne Programmiersprache für KI-APIs. Kompiliert zu Rust für maximale Performance.

> **Hinweis:** Dieses Projekt befindet sich in aktiver Entwicklung. Die API kann sich noch ändern.

## Installation

```bash
# Rust installieren (falls noch nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compiler bauen
cd compiler
cargo build --release

# Binary ist dann in: compiler/target/release/velin-compiler
```

## Quick Start

```velin
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript! 🚀";
}
```

## CLI Commands - Vollständige Übersicht

### Kompilieren

```bash
# Basis-Kompilierung
velin compile -i main.velin -o main.rs

# Kompilierung ohne Type Checking
velin compile -i main.velin --no-type-check

# Generierten Code in Konsole anzeigen
velin compile -i main.velin --show-code
```

### Prüfen (nur Type Checking)

```bash
velin check -i main.velin
```

### Formatieren

```bash
# Datei formatieren
velin format -i main.velin

# Datei formatieren und überschreiben
velin format -i main.velin --in-place
```

### Informationen anzeigen

```bash
velin info -i main.velin
```

### Projekt initialisieren

```bash
# Neues Projekt erstellen
velin init my-project

# Im aktuellen Verzeichnis initialisieren
velin init --current-dir
```

### OpenAPI Specification generieren

```bash
# OpenAPI Spec generieren
velin openapi -i main.velin -o api.json

# Automatische Ausgabe-Datei (main.openapi.json)
velin openapi -i main.velin
```

### Code-Generierung

```bash
# API-Endpoint generieren
velin generate api --name UserAPI --path /api/users

# CRUD-Operationen generieren
velin generate crud --name User --fields "id:string,name:string,email:string"

# Test-Template generieren
velin generate test --name UserTest

# Client-Code generieren (TypeScript)
velin generate client --openapi api.json --language typescript --output ./client

# Client-Code generieren (JavaScript)
velin generate client --openapi api.json --language javascript --output ./client

# Mit Ausgabe-Datei
velin generate api --name ProductAPI --output product_api.velin
```

### Tests ausführen

```bash
# Alle Tests
velin test

# Nur Unit Tests
velin test --unit

# Nur Integration Tests
velin test --integration

# Verbose Output
velin test --verbose

# Tests aus spezifischem Verzeichnis
velin test --directory ./tests
```

### Config-Management

```bash
# Config initialisieren
velin config init

# Config mit Beispiel initialisieren
velin config init --example

# Config validieren
velin config validate

# Spezifische Config-Datei validieren
velin config validate --file velin.config.json

# Config anzeigen
velin config show

# Spezifische Config-Datei anzeigen
velin config show --file velin.config.json
```

### Cache-Management

```bash
# Cache-Statistiken anzeigen
velin cache stats

# Cache leeren
velin cache clear

# Cache mit Pattern leeren
velin cache clear "user:*"

# Cache wärmen
velin cache warm
```

### Health Check

```bash
# Health Check (Standard: http://localhost:8080/health)
velin health

# Health Check mit URL
velin health --url http://localhost:8080/health

# Detaillierte Metriken
velin health --url http://localhost:8080/metrics --verbose
```

### Backup-Management

```bash
# Backup erstellen
velin backup create --strategy full --destination ./backups

# Backup mit Kompression erstellen
velin backup create --strategy full --destination ./backups --compression gzip

# Backup wiederherstellen
velin backup restore backup-123

# Backup wiederherstellen zu spezifischem Ziel
velin backup restore backup-123 --destination ./restore

# Backup-Liste anzeigen
velin backup list

# Backup-Liste aus spezifischem Verzeichnis
velin backup list --directory ./backups

# Backup löschen
velin backup delete backup-123

# Backup verifizieren
velin backup verify backup-123

# Backup verifizieren aus spezifischem Verzeichnis
velin backup verify backup-123 --directory ./backups
```

### Rollback-Management

```bash
# Transaktion beginnen
velin rollback begin

# Transaktion committen
velin rollback commit tx-123

# Transaktion rollback
velin rollback rollback tx-123

# Version erstellen
velin rollback create-version "Vor Update"

# Rollback zu Version
velin rollback to-version v-123

# Versionen auflisten
velin rollback list-versions

# Snapshot erstellen
velin rollback create-snapshot "Production State"

# Rollback zu Snapshot
velin rollback to-snapshot snap-123

# Snapshots auflisten
velin rollback list-snapshots
```

### Serialization-Tools

```bash
# JSON zu YAML konvertieren
velin serialize json-to-yaml -i config.json -o config.yaml

# YAML zu JSON konvertieren
velin serialize yaml-to-json -i config.yaml -o config.json

# JSON validieren
velin serialize validate-json -f data.json

# YAML validieren
velin serialize validate-yaml -f config.yaml
```

## Language Features

### Datentypen

- **Primitive Types:**
  - `string` - Zeichenketten
  - `number` - Zahlen (Integer & Float)
  - `boolean` - Wahrheitswerte
  - `void` - Kein Rückgabewert
  - `null` - Null-Wert

- **Komplexe Types:**
  - `List<T>` - Listen/Arrays
  - `Option<T>` - Optionale Werte
  - `Result<T, E>` - Fehlerbehandlung
  - Custom Types (Structs, Enums)

### Variablen & Funktionen

```velin
// Variablen mit Type Inference
let name = "John";
let age = 30;
let active: boolean = true;

// Mutable Variablen
let mut counter = 0;
counter = counter + 1;

// Funktionen
fn greet(name: string): string {
    return "Hello, " + name;
}

// Async Funktionen
async fn fetchData(): string {
    return await http.get("https://api.example.com/data");
}

// Public Funktionen
pub fn publicFunction(): void {
    // ...
}
```

### Structs & Enums

```velin
// Structs
struct User {
    id: string,
    name: string,
    email: string,
}

// Enums
enum Status {
    Active,
    Inactive,
    Pending,
}

// Enums mit Daten
enum Result {
    Success(string),
    Error(string),
}
```

### Kontrollstrukturen

```velin
// If/Else
if condition {
    // ...
} else {
    // ...
}

// For-Loops
for item in items {
    // ...
}

// While-Loops
while condition {
    // ...
}

// Match (Pattern Matching)
match value {
    case "option1" => { /* ... */ }
    case "option2" => { /* ... */ }
    _ => { /* default */ }
}
```

### Decorators

VelinScript unterstützt umfangreiche Decorators:

#### HTTP Decorators
- `@GET(path)` - GET Endpoint
- `@POST(path)` - POST Endpoint
- `@PUT(path)` - PUT Endpoint
- `@DELETE(path)` - DELETE Endpoint
- `@PATCH(path)` - PATCH Endpoint

#### Security Decorators
- `@Auth` - Authentifizierung erforderlich
- `@Role("admin")` - Rollen-basierte Zugriffskontrolle
- `@JWT` - JWT Token Validierung
- `@OAuth2` - OAuth2 Integration
- `@APIKey` - API Key Authentifizierung

#### Framework Decorators
- `@Axum` - Axum Framework verwenden
- `@Actix` - Actix-Web Framework verwenden

#### Validation Decorators
- `@Validate` - Input Validation aktivieren
- `@Summary("Beschreibung")` - OpenAPI Summary
- `@Description("Detaillierte Beschreibung")` - OpenAPI Description
- `@Tag("users")` - OpenAPI Tag

#### Performance Decorators
- `@Cache(ttl: "5m", key: "user:{id}")` - Caching
- `@SEO(title: "...", description: "...")` - SEO-Metadaten

#### ML/AI Decorators
- `@AI(model: "sentiment")` - AI/ML Integration

#### Testing Decorators
- `@test` - Test-Funktion

## Standard Library Funktionen

### Database Operations

```velin
// Find by ID
let user = db.find(User, "123");

// Find All
let users = db.findAll(User);

// Save (Insert/Update)
let saved = db.save(user);

// Update
let updated = db.update(User, "123", user);

// Delete
db.delete(User, "123");

// Query Builder
let results = db.query("SELECT * FROM users WHERE active = true");

// Transactions
db.transaction({
    db.save(user1);
    db.save(user2);
});
```

### HTTP Operations

```velin
// HTTP Request
let request = HttpRequest::new("GET", "/api/users");
request.header("Authorization", "Bearer token");
let response = http.send(request);

// HTTP Response
let response = HttpResponse::ok(json);
let error = HttpResponse::bad_request("Invalid input");
```

### Validation

```velin
let mut validator = Validator::new();
validator.required("name", Some(&name));
validator.min_length("name", &name, 3);
validator.max_length("name", &name, 50);
validator.email("email", &email);
validator.pattern("phone", &phone, r"^\d+$", "Invalid phone");
validator.min("age", age, 18);
validator.max("age", age, 100);
validator.range("score", score, 0, 100);

if !validator.is_valid() {
    return validator.errors();
}
```

### File I/O

```velin
// Datei lesen
let content = file.read("data.txt");

// Datei schreiben
file.write("output.txt", content);

// Datei anhängen
file.append("log.txt", "New entry");

// Datei existiert?
if file.exists("config.json") {
    // ...
}

// Verzeichnis erstellen
file.createDirectory("./data");

// Verzeichnis lesen
let files = file.readDirectory("./data");
```

### JSON Operations

```velin
// JSON parsen
let data = json.parse(jsonString);

// JSON stringify
let jsonString = json.stringify(data);

// JSON validieren
if json.isValid(jsonString) {
    // ...
}
```

### Crypto & Security

```velin
// SHA-256 Hash
let hash = crypto.sha256("password");

// MD5 Hash
let md5 = crypto.md5("data");

// UUID generieren
let id = crypto.uuid();

// Passwort hashen
let salt = crypto.generateSalt();
let hash = crypto.hashPassword("password", &salt);

// Passwort verifizieren
if crypto.verifyPassword("password", &salt, &hash) {
    // ...
}

// Base64 Encode/Decode
let encoded = crypto.base64Encode("data");
let decoded = crypto.base64Decode(encoded);
```

### DateTime Operations

```velin
// Aktuelles Datum/Zeit
let now = datetime.now();
let today = datetime.today();

// Formatieren
let formatted = datetime.format(now, "YYYY-MM-DD HH:mm:ss");

// Parsen
let date = datetime.parse("2024-01-01", "YYYY-MM-DD");

// Berechnungen
let tomorrow = datetime.addDays(now, 1);
let diff = datetime.diff(date1, date2);
```

### Regex Operations

```velin
// Pattern matchen
if regex.match("^[0-9]+$", input) {
    // ...
}

// Finden
let matches = regex.find("\\d+", text);

// Ersetzen
let replaced = regex.replace("old", "new", text);
```

### Logging

```velin
// Log-Level
logging.info("Information");
logging.debug("Debug message");
logging.warn("Warning");
logging.error("Error occurred");
```

### Async Operations

```velin
// Parallel ausführen
let results = async.parallel([
    async.fetch("url1"),
    async.fetch("url2"),
    async.fetch("url3"),
]);

// Timeout
let result = async.timeout(operation, 5000);

// Retry
let result = async.retry(operation, 3);
```

### Iterator Operations

```velin
// Map
let doubled = list.map(x => x * 2);

// Filter
let filtered = list.filter(x => x > 10);

// Reduce
let sum = list.reduce((acc, x) => acc + x, 0);

// Group By
let grouped = list.groupBy(x => x.category);

// Sorted
let sorted = list.sorted();
```

### ML/AI Operations

```velin
// Model laden
let model = ml.loadModel("sentiment.model");

// Prediction
let result = model.predict(input);

// LLM Client
let client = ml.createLLMClient("openai", apiKey);
let response = client.chat(messages);

// Vector DB
let vectorDB = ml.createVectorDB("pinecone");
vectorDB.upsert(vectors);
let results = vectorDB.query(queryVector, topK: 10);
```

## Features

### Core Features

- ✅ **Parser & Lexer** - Vollständige VelinScript Syntax
- ✅ **Type Checker** - Type Inference & Type Safety
- ✅ **Code Generator** - Transformation zu Rust
- ✅ **Error Handling** - Detaillierte Fehlermeldungen mit Source Context
- ✅ **Formatter** - Code-Formatierung

### CLI Features

- ✅ **Compile** - Kompilierung zu Rust
- ✅ **Check** - Type Checking
- ✅ **Format** - Code-Formatierung
- ✅ **Info** - Code-Analyse
- ✅ **Init** - Projekt-Initialisierung
- ✅ **OpenAPI** - OpenAPI Spec Generierung
- ✅ **Generate** - Code-Generierung (API, CRUD, Tests, Clients)
- ✅ **Test** - Test-Ausführung
- ✅ **Config** - Config-Management
- ✅ **Cache** - Cache-Management
- ✅ **Health** - Health Checks
- ✅ **Backup** - Backup-Management
- ✅ **Rollback** - Rollback-Management
- ✅ **Serialize** - Serialization-Tools

### Framework Features

- ✅ **HTTP Framework** - Axum & Actix-Web Support
- ✅ **Path Parameters** - Automatische Extraktion (`/api/users/:id`)
- ✅ **Query Parameters** - Automatische Query-Parameter-Unterstützung
- ✅ **Request Body** - JSON Body Parsing
- ✅ **Error Responses** - Automatische 400/401/403/404/500 Responses
- ✅ **Middleware** - CORS, Auth, Role Middleware

### Database Features

- ✅ **ORM Support** - SeaORM & sqlx
- ✅ **CRUD Operations** - find, findAll, save, update, delete
- ✅ **Query Builder** - db.query() für Custom Queries
- ✅ **Transactions** - db.transaction() Support
- ✅ **Entity Generation** - Automatische Entity-Generierung

### Security Features

- ✅ **Authentication** - @Auth Decorator
- ✅ **Authorization** - @Role Decorator
- ✅ **JWT** - JWT Token Validation
- ✅ **OAuth2** - OAuth2 Integration
- ✅ **OIDC** - OpenID Connect Support
- ✅ **API Keys** - API Key Authentication
- ✅ **TLS** - TLS/SSL Support
- ✅ **Vault** - HashiCorp Vault Integration

### Validation Features

- ✅ **Input Validation** - Automatische Parameter-Validierung
- ✅ **Validator Methods** - required, min_length, max_length, email, pattern, min, max, range, custom
- ✅ **Error Collection** - Strukturierte Fehlermeldungen
- ✅ **Framework Integration** - Automatische Error-Responses

### OpenAPI Features

- ✅ **Spec Generation** - Vollständige OpenAPI 3.0 Specs
- ✅ **Components/Schemas** - Automatische Schema-Generierung
- ✅ **Security Schemes** - JWT, OAuth2, API Key
- ✅ **Error Responses** - Vollständige Error-Response-Definitionen
- ✅ **Decorator Metadata** - @Summary, @Description, @Tag Support
- ✅ **Path Parameters** - Korrekte Path-Parameter-Erkennung

### Testing Features

- ✅ **Test Decorator** - @test für Test-Funktionen
- ✅ **Assert Functions** - assert(), assert_eq(), assert_ne()
- ✅ **Test Runner** - Unit & Integration Tests

### ML/AI Features

- ✅ **Model Loading** - ML Model Support
- ✅ **LLM Integration** - OpenAI, Anthropic, Local LLMs
- ✅ **Vector DB** - Pinecone, Weaviate, Qdrant Support
- ✅ **Training** - Model Training Support

### Performance Features

- ✅ **Optimizer** - Dead Code Elimination
- ✅ **Constant Folding** - Compile-Time Optimierungen
- ✅ **Function Inlining** - Performance-Optimierungen
- ✅ **Caching** - @Cache Decorator

## Warum VelinScript?

- 🚀 **Schnell** - Kompiliert zu Rust für maximale Performance
- 🔒 **Sicher** - Security-Features von Anfang an
- 🤖 **KI-Ready** - Native Unterstützung für KI/ML-Integration
- 📝 **Einfach** - Klare, lesbare Syntax
- 🛠️ **Praktisch** - Built-in Support für REST APIs
- 🔧 **Produktiv** - Code-Generierung, OpenAPI, Testing out-of-the-box

## Beispiel

**VelinScript:**
```velin
struct User {
    id: string,
    name: string,
    email: string,
}

@GET("/api/users/:id")
@Auth
fn getUser(id: string): User {
    return db.find(User, id);
}

@POST("/api/users")
@Validate
fn createUser(name: string, email: string): User {
    let mut validator = Validator::new();
    validator.required("name", Some(&name));
    validator.email("email", &email);
    
    if !validator.is_valid() {
        return validator.errors();
    }
    
    let user = User {
        id: crypto.uuid(),
        name: name,
        email: email,
    };
    
    return db.save(user);
}
```

**Wird zu Rust:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[get("/api/users/:id")]
async fn get_user(Path(id): Path<String>) -> impl IntoResponse {
    // Auth middleware check
    let user = db.find::<User>(id).await;
    (StatusCode::OK, Json(user)).into_response()
}

#[post("/api/users")]
async fn create_user(Json(payload): Json<CreateUserPayload>) -> impl IntoResponse {
    // Validation
    let mut validator = Validator::new();
    validator.required("name", Some(&payload.name));
    validator.email("email", &payload.email);
    
    if !validator.is_valid() {
        return (StatusCode::BAD_REQUEST, Json(validator.errors())).into_response();
    }
    
    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        name: payload.name,
        email: payload.email,
    };
    
    let saved = db.save(user).await;
    (StatusCode::CREATED, Json(saved)).into_response()
}
```

## Projektstruktur

```
velinscript/
├── compiler/          # Rust Compiler
│   ├── src/
│   │   ├── parser/    # Parser & Lexer
│   │   ├── type_checker/  # Type Checking
│   │   ├── codegen/   # Code Generation
│   │   ├── stdlib/    # Standard Library
│   │   └── cli.rs     # CLI Commands
│   └── tests/         # Compiler Tests
├── tools/             # Developer Tools
│   ├── lsp/           # Language Server
│   └── vscode-extension/  # VSCode Extension
├── docs/              # Dokumentation
│   ├── language/      # Language Specification
│   ├── api/           # API Documentation
│   └── guides/        # Tutorials & Guides
├── examples/          # Beispiel-Projekte
└── tests/            # Integration Tests
```

## Entwicklung

```bash
# Compiler bauen
cd compiler
cargo build

# Tests ausführen
cargo test

# Beispiel kompilieren
cargo run -- compile -i ../examples/hello.velin
```

## Status

**(MVP) - Abgeschlossen:**
- ✅ Parser & Lexer
- ✅ AST & Parser
- ✅ Type Checker (mit Module-Support, Pattern Matching, Member Access)
- ✅ Code Generator
- ✅ CLI Tool (15+ Commands)
- ✅ Standard Library (20+ Module)
- ✅ Security Framework
- ✅ Testing Framework
- ✅ Error Handling
- ✅ Database ORM erweitern (query(), transaction(), Entity-Generierung)
- ✅ HTTP Framework vollständig (Path/Query-Parameter, Middleware, Error-Responses)
- ✅ OpenAPI Integration (Components/Schemas, Security-Schemes, vollständige Spec)
- ✅ Input Validation (Regex, min/max, custom, Framework-Integration)

**Geplant:**
- LSP Server (in Entwicklung)
- Documentation Generator
- Package Manager
- AI/ML Framework (Basis vorhanden)

## Contributing

Wir freuen uns über Contributions! Siehe [CONTRIBUTING.md](CONTRIBUTING.md) für Details.

## License

MIT License - Siehe [LICENSE](LICENSE) für Details.

## Community & Tools

### 🛠️ Developer Tools

- **VSCode Extension** - Syntax Highlighting, IntelliSense, Code Completion
  - Installieren: [Marketplace](https://marketplace.visualstudio.com/items?itemName=velinscript.velinscript) (bald verfügbar)
  - Features: Syntax Highlighting, Snippets, Error Detection
  - Repository: [tools/vscode-extension/](tools/vscode-extension/)

- **LSP Server** - Language Server Protocol für IDE-Integration
  - Unterstützt: VSCode, Vim, Emacs, und andere LSP-kompatible Editoren
  - Features: Auto-Completion, Go-to-Definition, Hover-Informationen
  - Repository: [tools/lsp/](tools/lsp/)

### 📚 Ressourcen

- 📖 **Dokumentation:** [docs/](docs/)
- 💻 **Beispiele:** [examples/](examples/)
- 📋 **Language Specification:** [docs/language/specification.md](docs/language/specification.md)
- 🗺️ **Roadmap:** Siehe [CHANGELOG.md](CHANGELOG.md)

### 🤝 Beitragen

- 🐛 **Bug melden:** [GitHub Issues](https://github.com/SkyliteDesign/velinscript/issues)
- 💡 **Feature vorschlagen:** [GitHub Discussions](https://github.com/SkyliteDesign/velinscript/discussions)
- 🔧 **Code beitragen:** Siehe [CONTRIBUTING.md](CONTRIBUTING.md)
- 💬 **Diskutieren:** [GitHub Discussions](https://github.com/SkyliteDesign/velinscript/discussions)
- 💬 **Diskutieren:** [Forum BirdAPI Velin Core Discussions](https://forum.birdapi.de/forum/)

### 📦 Installation & Setup


- **Compiler bauen:** Siehe [Installation](#installation)
- **VSCode Extension:** Installationsanleitung in [tools/vscode-extension/README.md](tools/vscode-extension/README.md)
- **LSP Server:** Setup-Anleitung in [tools/lsp/README.md](tools/lsp/README.md)

---

**Entwickelt mit ❤️ für die KI-API-Community**
