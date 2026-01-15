# Changelog

Alle wichtigen Änderungen an VelinScript werden in dieser Datei dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/de/1.0.0/),
und dieses Projekt hält sich an [Semantic Versioning](https://semver.org/lang/de/).

## [Unreleased]

### Geplant
- Macros System
- Compile-time Evaluation

## [0.2.0] - 2024-XX-XX

### Added
- **Erweiterte Pattern Matching**
  - Pattern Guards (`if condition` in Match Arms)
  - Range Patterns (`0..=12`, `13..19`)
  - Struct Destructuring mit Literal-Matching (`User { name: "admin" }`)
  - Wildcard Patterns (`_`)
  - Or Patterns (`"pending" | "processing"`)
  - Enum Variant Patterns (`Status::Active`)

- **Closure/Lambda Functions**
  - Lambda-Syntax: `(params) => expression`
  - Type Inference für Lambda-Parameter
  - Capture von Variablen aus äußerem Scope
  - Integration mit Collections Library

- **Collections Library**
  - `List<T>` mit funktionalen Methoden: filter, map, reduce, find, contains, indexOf, sort, reverse, chunk, slice
  - `Map<K, V>` mit Methoden: keys, values, entries, get, set, delete, has, size
  - `Set<T>` mit Methoden: add, remove, has, size, union, intersection, difference
  - Parallelisierung für große Collections

- **HTTP Client Library**
  - `HttpClient` Klasse für Client-seitige HTTP-Requests
  - Methoden: GET, POST, PUT, DELETE, PATCH
  - Response-Parsing: json(), text(), status()
  - Error Handling mit Retry-Logik
  - Header-Management

- **Rate Limiting Library**
  - Fixed Window Strategy
  - Sliding Window Strategy
  - Token Bucket Strategy
  - Distributed Rate Limiting (Redis-basiert)
  - Decorator-basierte Integration (`@RateLimit`)

- **Code Formatter (vollständig)**
  - Vollständige Formatierung von VelinScript-Code
  - Unterstützung für alle Language Features
  - Konfigurierbare Formatierungsregeln
  - CLI-Integration (`velin format`)

- **Linter (velin-lint)**
  - Code-Qualitätsanalyse
  - Regeln: Unused Variables, Unused Imports, Complexity, Naming Conventions
  - Erweiterbare Regel-Architektur
  - CLI-Tool für Standalone-Nutzung

- **Documentation Generator (velin-api-doc)**
  - JSDoc-Parsing für `///` Kommentare
  - HTML-Export
  - Interactive Docs (Swagger UI)
  - OpenAPI 3.0 Integration

- **Hot Reload (velin-hot-reload)**
  - Automatisches Neuladen bei Dateiänderungen
  - File System Watching
  - Watch-Mode und Server-Mode
  - Integration mit Compiler

- **Advanced Optimizer**
  - Function Inlining (mit Heuristik für Rekursion)
  - Loop Optimizations (while (false) Elimination)
  - Dead Code Elimination
  - Constant Folding

### Fixed
- Range Pattern Tokenisierung: `read_number()` erkennt jetzt korrekt `..=` als Range-Operator
- Enum-Parsing: Newlines werden vor Enum-Varianten korrekt übersprungen
- Mehrzeilige Funktionsaufrufe: Newlines werden in Argument-Listen korrekt behandelt
- Pattern Matching: Alle Pattern-Typen (Range, Or, Wildcard, Struct Destructuring) funktionieren korrekt

## [0.1.1] - 2024-XX-XX

### Added
- **Autonome Funktionen**
  - Auto-Import Management im LSP Server
  - Auto-Fix für häufige Fehler (Code Actions)
  - Dead Code Detector Tool (velin-dead-code)
  - Automatische Dependency Updates im Package Manager
  - API Documentation Generator (velin-api-doc) für OpenAPI/Swagger

- **LSP Server Verbesserungen**
  - References Provider (Find All References)
  - Rename Symbol Support
  - Code Actions (Quick Fixes)
  - Import-Organisierung

- **Package Manager (velin-pkg)**
  - Dependency Update Checking
  - Automatische Updates mit `--all` Flag
  - Breaking Change Detection

- **Dead Code Detector**
  - Automatische Erkennung von ungenutztem Code
  - JSON Report Support
  - Scan für ganze Verzeichnisse

- **API Documentation Generator**
  - OpenAPI 3.0 JSON/YAML Generation
  - Markdown Dokumentation
  - Automatische Schema-Extraktion aus Structs/Enums
  - Security Schemes aus @Auth Decorators

## [0.1.0] - 2024-XX-XX

### Added
- **Parser & Lexer**
  - Vollständiger Lexer mit allen Tokens
  - Unterstützung für Keywords, Decorators, Literals, Operators
  - Kommentare (single-line und multi-line)
  - Error Handling mit Zeilen/Spalten

- **AST & Parser**
  - Vollständiger Parser für VelinScript
  - AST-Strukturen für alle Language Features
  - Unterstützt: Functions, Structs, Enums, Decorators, Modules
  - Error Handling mit Source Context

- **Type Checker**
  - Type Inference für Variablen
  - Type Checking für alle Expressions
  - Function Signatures
  - Scoping mit Environment
  - Error Reporting mit Location

- **Code Generator**
  - Transformation zu Rust Code
  - Decorators → Rust Attributes
  - Types → Rust Types
  - Standard Library Funktionen (db.find, etc.)
  - Security-Decorators unterstützt
  - Test-Decorators unterstützt

- **CLI Tool**
  - `velin compile` - Kompiliert VelinScript zu Rust
  - `velin check` - Prüft Code (Parsing + Type Checking)
  - `velin format` - Formatiert Code (Platzhalter)
  - `velin info` - Zeigt Informationen über Code
  - `velin init` - Initialisiert neues Projekt

- **Standard Library**
  - API Standard Library (api.rs)
  - Database Standard Library (database.rs)
  - Security Standard Library (security.rs)
  - Testing Standard Library (testing.rs)

- **Security Framework**
  - @Auth Decorator → AuthMiddleware
  - @Role Decorator → RoleMiddleware
  - Security Standard Library vorhanden
  - Code Generator unterstützt Security-Decorators

- **Testing Framework**
  - @test Decorator unterstützt
  - assert() Funktionen → assert!() / assert_eq!() / assert_ne!()
  - Test-Beispiele vorhanden

- **Dokumentation**
  - Language Specification
  - Getting Started Guide
  - API Documentation
  - Tutorials (Basics, APIs, Security, Database)
  - Beispiele

- **CI/CD**
  - GitHub Actions für CI
  - GitHub Actions für Releases
  - Automatische Tests

- **Installation**
  - install.sh für Linux/macOS
  - install.ps1 für Windows

- **OpenAPI Integration**
  - Automatische OpenAPI Spec Generierung
  - CLI Befehl `velin openapi`
  - Extraktion von Endpoints, Parameters, Security

- **HTTP Framework**
  - HttpRequest/HttpResponse Klassen
  - Error Responses (400, 401, 403, 404, 500)
  - Header und Query Parameter Management

- **Input Validation**
  - Validator Framework
  - Methoden: required, min_length, max_length, email, pattern
  - Fehlersammlung und -berichterstattung

- **JWT/OAuth2**
  - AuthService für JWT Token Management
  - OAuth2Provider für OAuth2 Integration
  - Role-based Access Control

- **ML/LLM Framework**
  - ModelLoader für ML Models
  - LLMClient (OpenAI, Anthropic, Local)
  - VectorDB Support (Pinecone, Weaviate, Qdrant)
  - TrainingService für Model Training

- **Performance Optimizer**
  - Dead Code Elimination
  - Constant Folding
  - Function Inlining
  - Loop Optimization
  - Benchmark Framework

### Changed
- Error Handling verbessert mit Zeilen/Spalten und Source Context

### Fixed
- (Noch keine Bug Fixes - erste Version)

## [0.0.1] - 2024-XX-XX

### Added
- Initiales Projekt-Setup
- Basis-Projektstruktur
