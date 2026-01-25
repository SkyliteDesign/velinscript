# Vollständiger Dokumentations-Index

**Version:** 3.1.0  
**Letzte Aktualisierung:** 2026-02-02

---

## Übersicht

Dieser Index listet alle Dokumentationen nach Themenbereichen auf.

---

## 🏗️ Architektur & Compiler

### Core-Architektur
- ✅ **[Compiler Architecture](architecture/compiler-architecture.md)** - Pass-System und Core
- ✅ **[Pass-Verlauf](architecture/pass-verlauf.md)** - Detaillierte Erklärung aller Passes
- ✅ **[Fehlerbehandlung & Lösungsvorschläge](architecture/error-handling.md)** ✅ (Neu in 3.1.0) - Umfassendes Fehlerbehandlungssystem
- ✅ **[Passes-Übersicht](architecture/passes-uebersicht.md)** - Vollständige Übersicht aller Passes
- ✅ **[Pass-Dokumentation Mapping](architecture/pass-dokumentation-mapping.md)** - Welche Doku für welchen Pass

### Spezifische Passes
- ✅ **[ParserPass](architecture/parser-pass.md)** - Parsing & Modul-Auflösung
- ✅ **[DesugaringPass](architecture/desugaring-pass.md)** - Syntaktischer Zucker Transformation
- ✅ **[CodeOrderingPass](architecture/code-ordering-pass.md)** - Automatische Code-Sortierung

### Compiler-Features
- ✅ **[IR Representation](architecture/ir-representation.md)** - Intermediate Representation
- ✅ **[Multi-Target Compilation](architecture/multi-target-compilation.md)** - 8 Zielsprachen Support
- ✅ **[Parallelization](architecture/parallelization.md)** - Automatische Parallelisierung
- ✅ **[System Generation](architecture/system-generation.md)** - Boilerplate-freie System-Generierung
- ✅ **[Code Generation](architecture/code-generation.md)** - Codegen-System
- ✅ **[Type Inference](architecture/type-inference.md)** - Automatische Type-Inference System
- ✅ **[Code Ordering](architecture/code-ordering.md)** - Automatische Code-Sortierung
- ✅ **[Borrow Checker](architecture/borrow-checker.md)** - Ownership & Borrowing System
- ✅ **[Module Resolution](architecture/module-resolution.md)** - Wie Module aufgelöst werden
- ✅ **[Framework Integration](architecture/framework-integration.md)** - Multi-Framework Support
- ✅ **[Prompt Optimizer](architecture/prompt-optimizer.md)** - 90%+ Token-Ersparnis
- ✅ **[AI Compiler Passes](architecture/ai-compiler-passes.md)** - KI-basierte Code-Analyse

---

## 📖 Sprache & Syntax

### Syntax-Grundlagen
- ✅ **[Language Specification](language/specification.md)** - Vollständige Sprachspezifikation
- ✅ **[Basics](language/basics.md)** - Grundlagen der Sprache
- ✅ **[Tutorial 1: Basics](guides/tutorial-1-basics.md)** - Variablen, Funktionen, Structs

### Erweiterte Sprachfeatures
- ✅ **[Closures & Lambdas](guides/tutorial-closures.md)** - Lambda Functions
- ✅ **[Pattern Matching](guides/tutorial-pattern-matching.md)** - Erweiterte Pattern Matching
- ✅ **[Collections](guides/tutorial-collections.md)** - Collections Library
- ✅ **[String Interpolation](guides/tutorial-string-interpolation.md)** - Format-Strings
- ✅ **[Type Inference](guides/tutorial-type-inference.md)** - Type-Inference & Code Ordering

---

## 🔐 Sicherheit

### Security-Features
- ✅ **[Security Guide](guides/security.md)** - Security-Best-Practices
- ✅ **[Tutorial 3: Security](guides/tutorial-3-security.md)** - Security-Features
- ✅ **[Input Sanitization](guides/tutorial-5-validation.md)** - Input-Validierung (enthält Sanitization)
- ✅ **[Verschlüsselung](api/standard-library.md)** - In Standard Library (crypto, encryption, tls Module)
- ✅ **[Security Scanner](tools/security-scanner.md)** - Security-Vulnerabilities scannen

### Authentication & Authorization
- ✅ **[Tutorial 6: Authentication](guides/tutorial-6-authentication.md)** - Auth-Systeme
- ✅ **[OAuth2](guides/tutorial-oauth2.md)** - OAuth2-Integration
- ✅ **[Auth & Rollen](api/standard-library.md)** - In Standard Library (auth, oauth2 Module)

### Rate Limiting
- ✅ **[Rate Limiting](api/standard-library.md)** - In Standard Library (rate_limit Module)

---

## 🚀 Performance

- ✅ **[Performance](architecture/parallelization.md)** - Automatische Parallelisierung
- ✅ **[Profiler](tools/profiler.md)** - Performance-Profiling
- ✅ **[Benchmark Runner](tools/benchmark-runner.md)** - Benchmark-Ausführung

---

## 🤖 KI & Machine Learning

### LLM Integration
- ✅ **[LLM Integration](guides/tutorial-7-ml.md)** - Machine Learning & LLM-Integration
- ✅ **[AI/ML Guide](guides/ai-ml.md)** - KI & Machine Learning
- ✅ **[API-Keys Setup](guides/api-keys-setup.md)** - 🔑 API-Keys Konfiguration

### ML Training
- ✅ **[ML Training](guides/tutorial-ml-training.md)** - ML Model Training

### Vektor-DBs
- ✅ **[Vektor-Datenbanken](guides/vektor-datenbanken.md)** ✅ (Neu in 3.1.0) - Semantische Suche & RAG

---

## 📚 Standardbibliothek

- ✅ **[Standard Library](api/standard-library.md)** - Vollständige API-Referenz (50+ Module, 200+ Funktionen)
- ✅ **[Decorators](api/decorators.md)** - Alle verfügbaren Decorators
- ✅ **[Frameworks](api/frameworks.md)** - Framework-Integration
- ✅ **[OpenAPI](api/openapi.md)** - OpenAPI-Support
- ✅ **[Test Module](api/test_module.md)** - Testing-API

---

## 🔧 Modulsystem

- ✅ **[Module Resolution](architecture/module-resolution.md)** - Wie Module aufgelöst werden
- ✅ **[Auto-Imports](guides/auto-imports.md)** - Automatische Imports

---

## 🛠️ Tools & CLI

### CLI & Prozesse
- ✅ **[Getting Started](guides/getting-started.md)** - CLI-Befehle
- ✅ **[CLI-Referenz](guides/cli-reference.md)** ✅ (Neu in 3.1.0) - Vollständige CLI-Referenz
- ✅ **[Befehle (CLI)](README.md)** - In Haupt-README erwähnt

### Entwickler-Tools
- ✅ **[VS Code Extension](tools/vscode-extension.md)** - IDE-Integration
- ✅ **[LSP](tools/lsp.md)** - Language Server Protocol
- ✅ **[Debugger](tools/debugger.md)** - DAP Debugger Server
- ✅ **[Linter](tools/linter.md)** - Code-Qualitätsanalyse
- ✅ **[Formatter](tools/formatter.md)** - Code-Formatierung
- ✅ **[Hot Reload](tools/hot-reload.md)** - Automatisches Neuladen
- ✅ **[REPL](tools/repl.md)** - Read-Eval-Print-Loop

### Code-Generierung
- ✅ **[Code Generation](tools/code-generation.md)** - Code-Generierung
- ✅ **[Library Generator](tools/library-generator.md)** - Automatische Bibliotheks-Generierung
- ✅ **[API Doc Generator](tools/api-doc-generator.md)** - API-Dokumentationsgenerator

### Testing & Qualität
- ✅ **[Test Runner](tools/test-runner.md)** - Test-Ausführung
- ✅ **[Dead Code Detector](tools/dead-code-detector.md)** - Ungenutzten Code finden
- ✅ **[Security Scanner](tools/security-scanner.md)** - Security-Vulnerabilities scannen
- ✅ **[Profiler](tools/profiler.md)** - Performance-Profiling
- ✅ **[Benchmark Runner](tools/benchmark-runner.md)** - Benchmark-Ausführung

### Weitere Tools
- ✅ **[Package Manager](tools/package-manager.md)** - Dependency Management
- ✅ **[Runtime Inspector](tools/runtime-inspector.md)** - Runtime-Analyse
- ✅ **[Dependency Graph](tools/dependency-graph.md)** - Abhängigkeits-Graph
- ✅ **[Bundle Analyzer](tools/bundle-analyzer.md)** - Bundle-Analyse
- ✅ **[Backup & Rollback](tools/backup-rollback.md)** - Backup-Management
- ✅ **[Auto Repair](tools/auto-repair.md)** - Automatische Reparatur
- ✅ **[TOOLS_ÜBERSICHT](tools/TOOLS_ÜBERSICHT.md)** - Vollständige Übersicht aller Tools

---

## 🔌 Plugin-Entwicklung

- ✅ **[Plugin Development](guides/plugin-development.md)** - Plugin-Entwicklung
- ✅ **[Example Plugin](tools/example-plugin.md)** - Beispiel-Plugin

---

## 📦 Paketmanagement

- ✅ **[Package Manager](tools/package-manager.md)** - Dependency Management

---

## 📋 Tutorials & Guides

### Grundlagen
- ✅ **[Getting Started](guides/getting-started.md)** - Dein erster Einstieg
- ✅ **[Tutorial 1: Basics](guides/tutorial-1-basics.md)** - Variablen, Funktionen, Structs
- ✅ **[Tutorial 2: APIs](guides/tutorial-2-apis.md)** - API-Entwicklung
- ✅ **[Tutorial 3: Security](guides/tutorial-3-security.md)** - Security-Features
- ✅ **[Tutorial 4: Database](guides/tutorial-4-database.md)** - Datenbank-Integration
- ✅ **[Tutorial 5: Validation](guides/tutorial-5-validation.md)** - Input-Validierung
- ✅ **[Tutorial 6: Authentication](guides/tutorial-6-authentication.md)** - Auth-Systeme
- ✅ **[Tutorial 7: ML/LLM](guides/tutorial-7-ml.md)** - Machine Learning & LLM-Integration
- ✅ **[Tutorial 8: Intelligence](guides/tutorial-8-intelligence.md)** - VelinAutoDoc, VelinPipeline, @Flow

### Spezielle Themen
- ✅ **[Pattern Matching](guides/tutorial-pattern-matching.md)** - Erweiterte Pattern Matching
- ✅ **[Closures](guides/tutorial-closures.md)** - Lambda Functions
- ✅ **[Collections](guides/tutorial-collections.md)** - Collections Library
- ✅ **[HTTP Client](guides/tutorial-http-client.md)** - HTTP Client Library
- ✅ **[String Interpolation](guides/tutorial-string-interpolation.md)** - Format-Strings
- ✅ **[Debugger](guides/tutorial-debugger.md)** - Debugging in VS Code
- ✅ **[OAuth2](guides/tutorial-oauth2.md)** - OAuth2-Integration
- ✅ **[Privacy](guides/tutorial-privacy.md)** - Privacy-Features
- ✅ **[SeaORM](guides/tutorial-seaorm.md)** - SeaORM-Integration
- ✅ **[Type Inference](guides/tutorial-type-inference.md)** - Type-Inference & Code Ordering
- ✅ **[ML Training](guides/tutorial-ml-training.md)** - ML Model Training
- ✅ **[VelinFlow](guides/velin-flow.md)** - Transaktionales Flow-Management

### Weitere Guides
- ✅ **[Advanced](guides/advanced.md)** - Erweiterte Konzepte
- ✅ **[Backend](guides/backend.md)** - Backend-Entwicklung
- ✅ **[Security](guides/security.md)** - Security-Best-Practices
- ✅ **[AI/ML](guides/ai-ml.md)** - KI & Machine Learning
- ✅ **[Auto-Imports](guides/auto-imports.md)** - Automatische Imports
- ✅ **[Plugin Development](guides/plugin-development.md)** - Plugin-Entwicklung

---

## 💡 Beispiele

- ✅ **[Multi-Target Examples](examples/multi-target-examples.md)** - Beispiele für alle 8 Zielsprachen
- ✅ **[AI Smart Home](examples/08-ai-smart-home.md)** - Smart Home Beispiel

---

## 📊 Status-Übersicht

### ✅ Vollständig dokumentiert

| Thema | Dokumentation |
|-------|--------------|
| IR Representation | `architecture/ir-representation.md` |
| Multi-Target | `architecture/multi-target-compilation.md` |
| Parallelisierung | `architecture/parallelization.md` |
| System-Generierung | `architecture/system-generation.md` |
| Syntax-Grundlagen | `language/specification.md`, `language/basics.md` |
| Kollektionen | `guides/tutorial-collections.md` |
| Closures & Lambdas | `guides/tutorial-closures.md` |
| Pattern Matching | `guides/tutorial-pattern-matching.md` |
| Auth & Rollen | `guides/tutorial-6-authentication.md`, `api/standard-library.md` |
| Rate Limiting | `api/standard-library.md` |
| Performance | `architecture/parallelization.md`, `tools/profiler.md` |
| Input Sanitization | `guides/tutorial-5-validation.md` |
| Verschlüsselung | `api/standard-library.md` (crypto, encryption Module) |
| Sicherheitsscanner | `tools/security-scanner.md` |
| LLM Integration | `guides/tutorial-7-ml.md`, `guides/ai-ml.md` |
| ML Training | `guides/tutorial-ml-training.md` |
| Standardbibliothek | `api/standard-library.md` |
| Modulsystem | `architecture/module-resolution.md`, `guides/auto-imports.md` |
| CLI & Prozesse | `guides/getting-started.md` (CLI-Befehle) |
| Plugin-Entwicklung | `guides/plugin-development.md` |
| Paketmanagement | `tools/package-manager.md` |

### ⚠️ Teilweise dokumentiert

| Thema | Status | Wo dokumentiert |
|-------|--------|----------------|
| **Vektor-DBs** | ⚠️ | In Standard Library erwähnt, aber keine spezifische Doku |
| **Befehle (CLI)** | ⚠️ | In Getting Started erwähnt, aber keine vollständige CLI-Referenz |

### ✅ Vollständig dokumentiert

- ✅ **Vollständige CLI-Referenz** - [CLI-Referenz](guides/cli-reference.md) ✅
- ✅ **Vektor-DBs spezifische Doku** - [Vektor-Datenbanken](guides/vektor-datenbanken.md) ✅

---

## 📁 Dokumentationsstruktur

```
docs/
├── architecture/        # Compiler-Architektur & Passes
├── guides/             # Tutorials & Anleitungen
├── api/                # API-Referenz
├── tools/              # Tool-Dokumentationen
├── language/           # Sprachspezifikation
├── examples/           # Code-Beispiele
└── README.md           # Haupt-Übersicht
```

---

## 🔍 Schnellsuche

### Nach Thema suchen

- **Architektur:** `docs/architecture/`
- **Tutorials:** `docs/guides/tutorial-*.md`
- **API:** `docs/api/`
- **Tools:** `docs/tools/`
- **Sprache:** `docs/language/`

### Nach Pass suchen

- Siehe: [Pass-Dokumentation Mapping](architecture/pass-dokumentation-mapping.md)

---

## � Beispiel-Projekte & Beispiele

### System-Verwaltung
- ✅ **[System Diagnosis](../examples/system-diagnosis/)** ✅ (Neu in 3.1.0) - Vollständiges System-Diagnose-Tool
  - **API-Dokumentation**: [System-Diagnose-Dokumentation](system-diagnose.md) (911 Zeilen)
  - **Features**:
    - Systemressourcen-Überwachung (CPU, Memory, Disk, Network)
    - Umfassende Sicherheitsprüfungen (15+ Checks)
    - Health-Score-Berechnung (0-100)
    - Automatische Empfehlungen
    - Service-Status-Überwachung
    - Log-Analyse mit Pattern-Erkennung
  - **Module**:
    - `system_diagnosis.velin` - Hauptmodul (975 Zeilen)
    - `security_checks.velin` - Sicherheitsprüfungen (768 Zeilen)
    - `tests/system_diagnosis_test.velin` - Test-Suite (629 Zeilen, 20+ Tests)
  - **Dokumentation**:
    - [README.md](../examples/system-diagnosis/README.md) - Quick Start
    - [ZUSAMMENFASSUNG.md](../examples/system-diagnosis/ZUSAMMENFASSUNG.md) - Implementation Summary

---

## �📝 Empfehlungen

### Fehlende Dokumentationen erstellen

1. **CLI-Referenz** - Vollständige Liste aller CLI-Befehle mit Parametern
2. **Vektor-DBs** - Spezifische Anleitung für Vektor-Datenbanken

---

**Letzte Aktualisierung:** 2026-02-02  
**Version:** 3.1.0
