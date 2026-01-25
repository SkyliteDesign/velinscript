# 📚 VelinScript Dokumentation - Zentrale Übersicht

Willkommen zur vollständigen Dokumentation von VelinScript 3.1.0!

---

## 🚀 Schnellstart

**Neu hier?** Starte mit dem [Getting Started Guide](guides/getting-started.md)!

- **[📖 Getting Started](guides/getting-started.md)** - Dein erster Einstieg in VelinScript
- **[📋 Language Specification](language/specification.md)** - Vollständige Sprachspezifikation
- **[🎯 Wann nutze ich was?](wann-nutze-ich-was.md)** - Entscheidungshilfe für Tools und Features
- **[📑 Dokumentations-Index](DOKUMENTATIONS-INDEX.md)** ✅ (Neu in 3.1.0) - Vollständiger Index aller Dokumentationen

---

## 📖 Tutorials & Guides

### Grundlagen
- **[Tutorial 1: Basics](guides/tutorial-1-basics.md)** - Variablen, Funktionen, Structs
- **[Tutorial 2: APIs](guides/tutorial-2-apis.md)** - API-Entwicklung mit VelinScript
- **[Tutorial 3: Security](guides/tutorial-3-security.md)** - Security-Features
- **[Tutorial 4: Database](guides/tutorial-4-database.md)** - Datenbank-Integration
- **[Tutorial 5: Validation](guides/tutorial-5-validation.md)** - Input-Validierung
- **[Tutorial 6: Authentication](guides/tutorial-6-authentication.md)** - Auth-Systeme

### Erweiterte Features
- **[Tutorial 7: ML/LLM](guides/tutorial-7-ml.md)** - Machine Learning & LLM-Integration
- **[Tutorial 8: Intelligence](guides/tutorial-8-intelligence.md)** - VelinAutoDoc, VelinPipeline, @Flow
- **[Type Inference & Code Ordering](guides/tutorial-type-inference.md)** ✅ (Neu in 3.1.0) - Automatische Type-Inference und Code-Sortierung
- **[ML Training](guides/tutorial-ml-training.md)** - ML Model Training
- **[VelinFlow](guides/velin-flow.md)** - Transaktionales Flow-Management
- **[API-Keys Setup](guides/api-keys-setup.md)** - 🔑 API-Keys Konfiguration

### Spezielle Themen
- **[Pattern Matching](guides/tutorial-pattern-matching.md)** - Erweiterte Pattern Matching
- **[Closures](guides/tutorial-closures.md)** - Lambda Functions
- **[Collections](guides/tutorial-collections.md)** - Collections Library
- **[HTTP Client](guides/tutorial-http-client.md)** - HTTP Client Library
- **[String Interpolation](guides/tutorial-string-interpolation.md)** - Format-Strings
- **[Debugger](guides/tutorial-debugger.md)** - Debugging in VS Code
- **[OAuth2](guides/tutorial-oauth2.md)** - OAuth2-Integration
- **[Privacy](guides/tutorial-privacy.md)** - Privacy-Features
- **[SeaORM](guides/tutorial-seaorm.md)** - SeaORM-Integration

### Weitere Guides
- **[Advanced](guides/advanced.md)** - Erweiterte Konzepte
- **[Backend](guides/backend.md)** - Backend-Entwicklung
- **[Security](guides/security.md)** - Security-Best-Practices
- **[AI/ML](guides/ai-ml.md)** - KI & Machine Learning
- **[Auto-Imports](guides/auto-imports.md)** - Automatische Imports
- **[Plugin Development](guides/plugin-development.md)** - Plugin-Entwicklung
- **[CLI-Referenz](guides/cli-reference.md)** ✅ (Neu in 3.1.0) - Vollständige CLI-Referenz
- **[Vektor-Datenbanken](guides/vektor-datenbanken.md)** ✅ (Neu in 3.1.0) - Semantische Suche & RAG

---

## 📚 API-Referenz

### Standardbibliothek
- **[Standard Library](api/standard-library.md)** - Vollständige API-Referenz (50+ Module, 200+ Funktionen)
- **[Decorators](api/decorators.md)** - Alle verfügbaren Decorators
- **[Frameworks](api/frameworks.md)** - Framework-Integration
- **[OpenAPI](api/openapi.md)** - OpenAPI-Support
- **[Test Module](api/test_module.md)** - Testing-API

---

## 🏗️ Architektur

### Core-Architektur
- **[Compiler Architecture](architecture/compiler-architecture.md)** - Pass-System und Core
- **[Pass-Verlauf & Funktionsweise](architecture/pass-verlauf.md)** ✅ (Neu in 3.1.0) - Detaillierte Erklärung aller Passes
- **[Fehlerbehandlung & Lösungsvorschläge](architecture/error-handling.md)** ✅ (Neu in 3.1.0) <- 25.01.2026 -> Extra - Umfassendes Fehlerbehandlungssystem
- **[ParserPass](architecture/parser-pass.md)** ✅ (Neu in 3.1.0) - Parsing & Modul-Auflösung
- **[DesugaringPass](architecture/desugaring-pass.md)** ✅ (Neu in 3.1.0) - Syntaktischer Zucker Transformation
- **[CodeOrderingPass](architecture/code-ordering-pass.md)** ✅ (Neu in 3.1.0) - Automatische Code-Sortierung
- **[Module Resolution](architecture/module-resolution.md)** - Wie Module aufgelöst werden
- **[Code Generation](architecture/code-generation.md)** - Codegen-System
- **[Framework Integration](architecture/framework-integration.md)** - Multi-Framework Support

### Erweiterte Features
- **[Type Inference](architecture/type-inference.md)** ✅ (Neu in 3.1.0) - Automatische Type-Inference System
- **[Code Ordering](architecture/code-ordering.md)** ✅ (Neu in 3.1.0) - Automatische Code-Sortierung
- **[Multi-Target Compilation](architecture/multi-target-compilation.md)** - 8 Zielsprachen Support
- **[IR-Repräsentation](architecture/ir-representation.md)** - Intermediate Representation
- **[Borrow Checker](architecture/borrow-checker.md)** - Ownership & Borrowing System
- **[Parallelization](architecture/parallelization.md)** - Automatische Parallelisierung
- **[Prompt Optimizer](architecture/prompt-optimizer.md)** - 90%+ Token-Ersparnis

### KI-Features
- **[AI Compiler Passes](architecture/ai-compiler-passes.md)** - KI-basierte Code-Analyse
- **[System Generation](architecture/system-generation.md)** - Boilerplate-freie System-Generierung

---

## 🛠️ Tools

### Entwickler-Tools
- **[VS Code Extension](tools/vscode-extension.md)** - IDE-Integration
- **[LSP](tools/lsp.md)** - Language Server Protocol
- **[Debugger](tools/debugger.md)** - DAP Debugger Server
- **[Linter](tools/linter.md)** - Code-Qualitätsanalyse
- **[Formatter](tools/formatter.md)** - Code-Formatierung
- **[Hot Reload](tools/hot-reload.md)** - Automatisches Neuladen

### Code-Generierung
- **[Code Generation](tools/code-generation.md)** - Code-Generierung
- **[Library Generator](tools/library-generator.md)** - Automatische Bibliotheks-Generierung
- **[API Doc Generator](tools/api-doc-generator.md)** - API-Dokumentationsgenerator
- **[Boilerplate Generator](tools/boilerplate-generator.md)** - Boilerplate-Generierung

### Testing & Qualität
- **[Test Runner](tools/test-runner.md)** - Test-Ausführung
- **[Dead Code Detector](tools/dead-code-detector.md)** - Ungenutzten Code finden
- **[Security Scanner](tools/security-scanner.md)** - Security-Vulnerabilities scannen
- **[Profiler](tools/profiler.md)** - Performance-Profiling
- **[Benchmark Runner](tools/benchmark-runner.md)** - Benchmark-Ausführung

### Weitere Tools
- **[Package Manager](tools/package-manager.md)** - Dependency Management
- **[REPL](tools/repl.md)** - Read-Eval-Print-Loop
- **[Runtime Inspector](tools/runtime-inspector.md)** - Runtime-Analyse
- **[Dependency Graph](tools/dependency-graph.md)** - Abhängigkeits-Graph
- **[Bundle Analyzer](tools/bundle-analyzer.md)** - Bundle-Analyse
- **[Backup & Rollback](tools/backup-rollback.md)** - Backup-Management
- **[Auto Repair](tools/auto-repair.md)** - Automatische Reparatur

### Tools-Übersicht
- **[TOOLS_ÜBERSICHT](tools/TOOLS_ÜBERSICHT.md)** - Vollständige Übersicht aller Tools

---

## 💡 Beispiele

### Code-Beispiele
- **[Multi-Target Examples](examples/multi-target-examples.md)** - Beispiele für alle 8 Zielsprachen
- **[AI Smart Home](examples/08-ai-smart-home.md)** - Smart Home Beispiel

### Projekt-Beispiele
Siehe [examples/](../../examples/) für vollständige Beispiel-Projekte:
- **[System Diagnosis](system-diagnose.md)** ✅ (Neu in 3.1.0) - Umfassendes System Monitoring und Security Checks (2,372 LoC)
- **[01-hello-api](../../examples/01-hello-api/)** - Einfaches Einstiegsbeispiel
- **[02-llm-chat](../../examples/02-llm-chat/)** - LLM-Integration
- **[05-ultimate-showcase](../../examples/05-ultimate-showcase/)** - Alle Features
- **[Custom Recommender](../../examples/custom-recommender/)** - Production-Ready Recommendation System

---

## 🔍 Sprache

### Sprachspezifikation
- **[Language Specification](language/specification.md)** - Vollständige Sprachspezifikation
- **[Basics](language/basics.md)** - Grundlagen der Sprache

---

## 📋 Entscheidungshilfe

**[Wann nutze ich was?](wann-nutze-ich-was.md)** - Umfassende Entscheidungshilfe für:
- Entwicklung & Code-Qualität
- Debugging & Entwicklung
- Testing & Qualitätssicherung
- Performance & Optimierung
- Code-Generierung & Automatisierung
- Security & Sicherheit
- Package Management
- Intelligence Features
- API-Entwicklung
- KI & Machine Learning

---

## 🆘 Hilfe & Support

- **[Getting Started](guides/getting-started.md)** - Erste Schritte
- **[Häufige Probleme](guides/getting-started.md#häufige-probleme)** - Lösungen für häufige Probleme
- **[Forum & Support](https://forum.birdapi.de/forum/)** - Community-Support
- **[GitHub Issues](https://github.com/SkyliteDesign/velinscript/issues)** - Bug Reports
- **[Discussions](https://github.com/SkyliteDesign/velinscript/discussions)** - Diskussionen

---

## 📝 Beitragen

- **[CONTRIBUTING.md](../../CONTRIBUTING.md)** - Wie man beiträgt
- **[Plugin Development](guides/plugin-development.md)** - Plugin-Entwicklung

---

**Letzte Aktualisierung:** 2026-02-02  
**Version:** 3.1.0
