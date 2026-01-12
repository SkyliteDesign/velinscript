# Changelog

Alle wichtigen Änderungen an VelinScript werden in dieser Datei dokumentiert.

Das Format basiert auf [Keep a Changelog](https://keepachangelog.com/de/1.0.0/),
und dieses Projekt hält sich an [Semantic Versioning](https://semver.org/lang/de/).

## [Unreleased]

### Geplant
- LSP Server
- Documentation Generator
- Package Manager
- Erweiterte Pattern Matching
- Generic Constraints

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
