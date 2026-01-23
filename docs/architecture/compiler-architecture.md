# VelinScript Compiler Architektur

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

Der VelinScript Compiler folgt einer modularen Pass-basierten Architektur, die maximale Flexibilität und Erweiterbarkeit bietet.

## Core Architektur

### VelinCompiler Core

Der `VelinCompiler` ist das Herzstück des Compilers und orchestriert alle Compiler-Passes.

**Struktur:**
- `compiler/src/compiler/mod.rs` - VelinCompiler Struct
- `compiler/src/compiler/config.rs` - CompilerConfig mit Feature Flags
- `compiler/src/compiler/context.rs` - CompilationContext für globalen Zustand
- `compiler/src/compiler/error.rs` - Core Error System
- `compiler/src/compiler/pass.rs` - Pass Trait Definition

**Features:**
- Pass-basierte Architektur
- Konfigurierbare Feature Flags
- Globaler Compilation Context
- Fehler-Aggregation

### Pass-System

Der Compiler verwendet ein Pass-System, bei dem jeder Pass eine spezifische Aufgabe erfüllt:

1. **AutoFixPass** - Automatische Fehlerkorrektur
2. **ParserPass** - Parsing und Modul-Auflösung
3. **DesugaringPass** ✅ (Version 3.0.1) - Syntaktischer Zucker Transformation (z.B. try-catch zu Result)
4. **CodeOrderingPass** ✅ (Neu in 3.1.0) - Automatische Code-Sortierung basierend auf Abhängigkeiten
5. **AISemanticPass** (optional) - KI-basierte Semantik-Analyse
6. **AIBugDetectionPass** (optional) - KI-basierte Bug-Erkennung
7. **TypeCheckPass** - Type Checking (mit verbesserter Type-Inference)
8. **ParallelizationAnalyzer** - Automatische Parallelisierung (Neu in 3.1)
9. **AICodeGenerationPass** (optional) - KI-basierte Code-Generierung
10. **AICodeReviewPass** (optional) - Reviewt AI-generierten Code auf Sicherheit und Qualität
11. **AISandboxPass** (optional) - Validiert AI-generierten Code in isolierter Sandbox
12. **AIOptimizationPass** (optional) - KI-basierte Optimierung
13. **CodegenPass** - Code-Generierung (Multi-Target, mit IR-Unterstützung)
    - **IR-Pipeline** (Standard): AST → IR → Optimized IR → Target Code
    - **Legacy-Pipeline** (optional): AST → Target Code (direkt)

**Implementierung:**
- `compiler/src/passes/autofix.rs`
- `compiler/src/passes/parser.rs`
- `compiler/src/passes/desugar.rs` ✅ (Version 3.0.1) - Syntaktischer Zucker Transformation
- `compiler/src/passes/code_order.rs` ✅ (Neu in 3.1.0) - Automatische Code-Sortierung
- `compiler/src/passes/ai_semantic.rs` (neu in 3.0)
- `compiler/src/passes/ai_bug_detection.rs` (neu in 3.0)
- `compiler/src/passes/type_check.rs` - Mit verbesserter Type-Inference
- `compiler/src/type_checker/checker.rs` - Type-Inference System (Neu in 3.1.0)
- `compiler/src/optimizer/parallelization.rs` (Neu in 3.1)
- `compiler/src/passes/ai_codegen.rs` (neu in 3.0)
- `compiler/src/passes/ai_code_review.rs` (neu in 3.1) - Als Pass implementiert
- `compiler/src/passes/ai_sandbox.rs` (neu in 3.1) - Als Pass implementiert
- `compiler/src/passes/ai_optimization.rs` (neu in 3.0)
- `compiler/src/passes/codegen.rs`
- `compiler/src/compiler/orchestrator.rs` ✅ (Neu in 3.1.0) - Build Orchestration

**Siehe auch:** 
- [Pass-Verlauf & Funktionsweise](./pass-verlauf.md) ✅ (Neu in 3.1.0) - Detaillierte Erklärung aller Passes
- [ParserPass](./parser-pass.md) ✅ (Neu in 3.1.0) - Parsing & Modul-Auflösung
- [DesugaringPass](./desugaring-pass.md) ✅ (Neu in 3.1.0) - Syntaktischer Zucker Transformation
- [CodeOrderingPass](./code-ordering-pass.md) ✅ (Neu in 3.1.0) - Automatische Code-Sortierung
- [Passes-Übersicht](./passes-uebersicht.md) ✅ (Neu in 3.1.0) - Vollständige Übersicht aller Passes
- [Pass-Dokumentation Mapping](./pass-dokumentation-mapping.md) ✅ (Neu in 3.1.0) - Welche Doku für welchen Pass
- [Module Resolution](./module-resolution.md) - Modul-Auflösung
- [KI-Compiler-Passes](./ai-compiler-passes.md)
- [Type Inference](./type-inference.md) ✅ (Neu in 3.1.0)
- [Code Ordering](./code-ordering.md) ✅ (Neu in 3.1.0)
- [IR-Repräsentation](./ir-representation.md) (Neu in 3.0.1)
- [Borrow Checker](./borrow-checker.md) (Neu in 3.0.1)
- [Prompt Optimizer](./prompt-optimizer.md) (Neu in 3.0.1)

---

## Modul-Auflösung

**Status:** ✅ Vollständig implementiert

Der `ParserPass` implementiert rekursive Modul-Auflösung:

### Features

- **Automatische Modul-Erkennung**: Erkennt `use module_name` Statements
- **Rekursive Auflösung**: Lädt Module rekursiv (Module können andere Module importieren)
- **Datei-Suche**: Sucht automatisch nach `.velin` Dateien im gleichen Verzeichnis
- **AST-Merging**: Fügt geparste Module in den globalen AST ein

### Beispiel

```velin
// main.velin
use models;
use services;

@GET("/api/users")
fn getUsers(): List<User> {
    return services.getUsers();
}
```

```velin
// models.velin
struct User {
    id: string,
    name: string,
}
```

```velin
// services.velin
use models;

fn getUsers(): List<User> {
    return db.findAll(User);
}
```

Der Compiler löst automatisch alle Module auf und der Type Checker kennt alle Definitionen.

---

## AutoFix Engine

**Status:** ✅ Vollständig implementiert

Die AutoFix Engine korrigiert automatisch häufige Syntax-Fehler vor dem Parsing.

### Implementierte Fixes

1. **Unbalanced Braces**: Behebt unausgeglichene Klammern (`{`, `[`, `(`)
2. **Function Signatures**: Korrigiert fehlende Klammern in Funktionssignaturen
3. **Generic Types**: Repariert unvollständige Generic-Typen (`List<T` → `List<T>`)

### Aktivierung

```bash
# Beim Kompilieren
velin compile -i main.velin --autofix

# Beim Prüfen
velin check -i main.velin --autofix
```

### Implementierung

- `compiler/src/autofix/mod.rs` - AutoFixer Logic
- `compiler/src/autofix/report.rs` - AutoFixReport
- `compiler/src/passes/autofix.rs` - AutoFixPass Integration

---

## Framework-Integration

**Status:** ✅ Vollständig implementiert

Der Compiler unterstützt automatische Framework-Erkennung und Codegen für Axum und Actix-Web.

### Framework Selector

**Implementierung:** `compiler/src/codegen/framework.rs`

**Features:**
- Automatische Framework-Erkennung
- Config-basierte Auswahl
- Decorator-basierte Auswahl
- Framework-spezifische Code-Generierung

### Unterstützte Frameworks

1. **Axum** (Default)
   - Moderne, async-first Architektur
   - Type-safe Routing
   - Automatische Request/Response-Handling

2. **Actix-Web**
   - Production-Ready Framework
   - Hohe Performance
   - Umfangreiche Middleware-Unterstützung

### Code-Generierung

Der Compiler generiert automatisch:
- Framework-spezifische Imports
- Router/App-Initialisierung
- Handler-Signaturen
- Middleware-Integration

---

## Error Handling & Observability

**Status:** ✅ Vollständig implementiert

### Global Error Handler

**Implementierung:** `compiler/src/codegen/rust.rs`

Der Compiler generiert automatisch einen globalen Error Handler:

```rust
struct AppError(anyhow::Error);

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // Konvertiert Fehler in saubere JSON-Responses
    }
}
```

### Structured Logging

**Features:**
- Automatisches `#[tracing::instrument]` auf allen async Handlern
- Request-ID Tracking
- Latenz-Messung
- Context-Propagation

### Implementierung

- Alle generierten Handler werden automatisch mit `tracing::instrument` versehen
- Fehler werden in strukturierte JSON-Responses umgewandelt
- Kein stilles Scheitern - alle Fehler werden geloggt

---

## Intermediate Representation (IR)

**Status:** ✅ Vollständig implementiert (Neu in 3.0.1)

**Implementierung:** `compiler/src/ir/`

**Architektur:**
```
Source Code → AST → IR → Optimized IR → Target Code
```

### IR-Struktur

Die IR verwendet SSA (Single Static Assignment) Format für optimierte Code-Generierung:

- **IRModule** - Haupt-IR-Struktur mit Functions, Structs, Enums
- **IRFunction** - Funktionen in IR-Format mit SSA-Body
- **IRBlock** - Blocks mit Instructions und Control Flow
- **IRInstruction** - SSA-Instructions (Add, Subtract, Call, etc.)
- **IRValue** - Werte in SSA-Format (Constant, Variable, Temporary)

### IR Builder

**Implementierung:** `compiler/src/ir/builder.rs`

**Features:**
- Vollständige AST → IR Konvertierung
- SSA-Format-Generierung
- Ownership-Information für Borrow-Checker
- Control Flow Graph (CFG) Erstellung

### IR Optimizer

**Implementierung:** `compiler/src/ir/optimizer.rs`

**Features:**
- **Dead Code Elimination** - Entfernt ungenutzte Variablen und Instructions
- **Constant Folding** - Faltet konstante Ausdrücke zur Compile-Zeit
- **Function Inlining** - Inlined kleine Funktionen direkt
- **Loop Optimizations** - Loop Unrolling, Invariant Code Motion

### IR Validator

**Implementierung:** `compiler/src/ir/validator.rs`

**Features:**
- SSA-Format-Validierung
- Block-Referenz-Validierung
- Typ-Konsistenz-Prüfung

### IR Code Generator

**Implementierung:** `compiler/src/codegen/ir_codegen.rs`

**Features:**
- IR → Rust Code Generation
- IR → PHP Code Generation (geplant)
- IR → Python Code Generation (geplant)
- Multi-Target Support

### Integration

Die IR ist vollständig in die Compiler-Pipeline integriert:

1. **AST → IR** - IRBuilder konvertiert AST zu IR
2. **IR-Optimierungen** - IROptimizer optimiert IR
3. **IR-Validierung** - IRValidator prüft IR auf Korrektheit
4. **IR → Target Code** - IRCodeGenerator generiert Target-Code

**Aktivierung:**
- Standardmäßig aktiviert in `CodegenPass`
- Kann mit `CodegenPass::with_ir(false)` deaktiviert werden (Legacy-Modus)

---

## Code-Generierung

### IR-basierte Code-Generierung

**Status:** ✅ Vollständig implementiert (Neu in 3.0.1)

**Implementierung:** `compiler/src/codegen/ir_codegen.rs`

**Features:**
- Optimierte Code-Generierung über IR
- Multi-Target Support (Rust, PHP, Python)
- SSA-basierte Optimierungen

### Boilerplate Generator

**Status:** ✅ Vollständig implementiert

**Implementierung:** `compiler/src/codegen/boilerplate.rs`

**Features:**
- API Boilerplate Generation
- CRUD Code Generation
- Test Boilerplate Generation

### Client Generator

**Status:** ✅ Vollständig implementiert

**Implementierung:** `compiler/src/codegen/client.rs`

**Features:**
- TypeScript Client Generation aus OpenAPI
- JavaScript Client Generation
- Rust Client Generation

### AutoDoc Generator

**Status:** ✅ Vollständig implementiert

**Implementierung:** `compiler/src/codegen/autodoc.rs`

**Features:**
- JSON-Dokumentation aus `///` Doc-Comments
- Knowledge Base Generation für RAG/LLM
- LLM-freundliche Kontextinformationen

### AutoTest Generator

**Status:** ✅ Vollständig implementiert

**Implementierung:** `compiler/src/codegen/autotest.rs`

**Features:**
- Automatische Test-Stub-Generierung
- Mock-Daten-Generierung
- Integration mit `@VelinAutoTest` Decorator

---

## Pipeline-Optimierung

**Status:** ✅ Vollständig implementiert

**Implementierung:** `compiler/src/optimizer/pipeline.rs`

**Features:**
- Datenabhängigkeits-Analyse
- Automatische Erkennung parallelisierbarer async Blöcke
- Codegen-Optimierung mit `tokio::join!`
- Integration mit `@VelinPipeline` Decorator

---

## Insight Engine

**Status:** ✅ Vollständig implementiert

**Implementierung:** `compiler/src/analysis/insight.rs`

**Features:**
- Unused Structs Detection
- Complex Functions Detection (Statement Count > 20)
- Redundant Queries Detection
- Integration mit `@VelinInsight` Decorator

---

## VelinFlow Runtime

**Status:** ✅ Vollständig implementiert

**Implementierung:** `compiler/src/stdlib/flow.rs`

**Features:**
- Automatisches State-Tracking
- Input-Snapshot-Management
- Automatisches Rollback/Commit
- Compensation-Logic
- Integration mit `@Flow` Decorator

---

## Zusammenfassung

Alle geplanten Features aus den `.trae/documents` sind implementiert:

✅ **VelinCompiler Core** - Pass-basierte Architektur  
✅ **Modul-Auflösung** - Rekursive Import-Auflösung  
✅ **AutoFix Engine** - Automatische Fehlerkorrektur  
✅ **Framework-Integration** - Axum/Actix-Web Support  
✅ **Error Handling** - Global Error Handler + Structured Logging  
✅ **Code-Generierung** - Boilerplate, Client, AutoDoc, AutoTest  
✅ **Pipeline-Optimierung** - Automatische Parallelisierung  
✅ **Insight Engine** - Code-Analyse  
✅ **VelinFlow Runtime** - Transaktionales Flow-Management  
✅ **Ultimate Showcase** - Vollständiges Beispiel-Projekt  
✅ **Multi-Target Backend** - PHP und Python Support

---

## Version 3.1 Features

### Multi-Target Compilation

**Status:** ✅ Vollständig implementiert

Der Compiler wurde erweitert, um neben Rust auch Code für andere Zielsprachen zu generieren. Dies ermöglicht die Nutzung von VelinScript in bestehenden PHP- oder Python-Umgebungen.

**Architektur:**
- **CodeGenerator Trait**: Abstrahierte Schnittstelle für Code-Generierung (`compiler/src/codegen/traits.rs`)
- **TargetLanguage Enum**: Definition der Ziel-Sprachen (Rust, PHP, Python)
- **Dynamische Dispatch**: Der `CodegenPass` wählt den passenden Generator basierend auf der `--target` Option.

**Unterstützte Targets:**

1. **Rust** (Default) ✅
   - High-Performance, Type-Safe
   - Nutzt Axum/Tokio Stack
   - Volle Feature-Unterstützung

2. **PHP** (`--target php`) ✅
   - Generiert modernes PHP 8.2+
   - Strict Types (`declare(strict_types=1)`)
   - PSR-konformer Code
   - Automatische Umsetzung von Velin-Typen zu PHP-Typen (z.B. `List<T>` -> `array`)

3. **Python** (`--target python`) ✅
   - Generiert Python 3.10+ Code
   - Nutzung von Type Hints (`typing.*`)
   - Data Classes für Structs
   - Async/Await Support für asynchrone Funktionen

4. **TypeScript** (`--target typescript` oder `--target ts`) ✅
   - Generiert TypeScript 5.0+ Code
   - Interfaces, Klassen, Generics (`List<T>` → `T[]`)
   - Async/Await Support

5. **JavaScript** (`--target javascript` oder `--target js`) ✅
   - Generiert modernen JavaScript Code (ES2020+)
   - Async/Await Support
   - ES Modules

6. **Go** (`--target go` oder `--target golang`) ✅
   - Generiert Go 1.20+ Code
   - Structs und Interfaces
   - Goroutines für Async

7. **Java** (`--target java`) ✅
   - Generiert Java 17+ Code (Spring Boot kompatibel)
   - POJOs mit Gettern/Settern
   - Lombok Support (optional)

8. **C#** (`--target csharp` oder `--target cs`) ✅
   - Generiert C# 10+ Code (ASP.NET Core kompatibel)
   - File-Scoped Namespaces
   - PascalCase-Konventionen

**CLI Nutzung:**
```bash
velin compile -i main.velin --target rust    # Default
velin compile -i main.velin --target php
velin compile -i main.velin --target python
velin compile -i main.velin --target typescript
velin compile -i main.velin --target javascript
velin compile -i main.velin --target go
velin compile -i main.velin --target java
velin compile -i main.velin --target csharp
```

**Siehe:** [Multi-Target Compilation Dokumentation](./multi-target-compilation.md) für Details zu allen Targets.

---

## Version 3.0 Features

### KI-Compiler-Passes

**Status:** ✅ Vollständig implementiert

VelinScript 3.0 führt revolutionäre KI-basierte Compiler-Passes ein:
- **AISemanticPass**: Code-Semantik-Analyse mit LLM
- **AIBugDetectionPass**: Proaktive Bug-Erkennung
- **AICodeGenerationPass**: Automatische Code-Generierung
- **AIOptimizationPass**: KI-basierte Code-Optimierung

**Siehe:** [KI-Compiler-Passes](./ai-compiler-passes.md)

### System-Generierung

**Status:** ✅ Vollständig implementiert

Boilerplate-freie System-Generierung:
- **SystemGenerator**: Erkennt High-Level APIs und generiert vollständige Systeme
- **Component Templates**: Wiederverwendbare Templates für System-Komponenten
- **Infrastructure Generator**: Infrastructure-as-Code Generation

**Siehe:** [System-Generierung](./system-generation.md)

### Automatische Parallelisierung

**Status:** ✅ Vollständig implementiert

- **ParallelizationAnalyzer**: Analysiert Datenabhängigkeiten und parallelisiert Code automatisch
- Unterstützt: Multithreading, GPU, Async, SIMD

### Selbstoptimierung

**Status:** ✅ Vollständig implementiert

- **ProfilingCollector**: Sammelt Laufzeitdaten für Optimierung
- **Learning System**: Lernt aus Optimierungs-Historie

### Verteilte Systeme

**Status:** ✅ Vollständig implementiert

- **DeploymentAnalyzer**: Analysiert und generiert Deployment-Pläne
- **InfrastructureGenerator**: Generiert Docker, Kubernetes, Serverless Configs

---

---

## Version 3.0.1 Features

### Intermediate Representation (IR)

**Status:** ✅ Vollständig implementiert

Die IR ist eine echte Intermediate Representation zwischen AST und Code-Generierung:

- **SSA-Format** - Single Static Assignment für optimierte Code-Generierung
- **IR-Optimierungen** - Dead Code Elimination, Constant Folding, Function Inlining
- **Multi-Target** - IR kann zu verschiedenen Ziel-Sprachen generiert werden
- **Vollständig integriert** - Standardmäßig in CodegenPass aktiviert

**Vorteile:**
- Mehrfache Optimierungen auf IR-Ebene
- Unabhängigkeit von Ziel-Sprache
- Bessere Code-Qualität
- Einfacheres Debugging

**Siehe:** [IR-Dokumentation](#intermediate-representation-ir)

---

---

## Version 3.0.1 Features

### Borrow Checker (Ownership & Borrowing)

**Status:** ✅ Vollständig implementiert

Der Borrow Checker implementiert ein Ownership-System ähnlich Rust:

- **Ownership-Tracking** - Verfolgt wer einen Wert besitzt
- **Borrow-Checks** - Prüft ob Referenzen gültig sind
- **Lifetime-Analyse** - Analysiert wie lange Werte leben
- **Memory-Safety** - Verhindert Use-After-Free, Double-Free

**Implementierung:** `compiler/src/borrow/`

**Features:**
- Use-After-Move Erkennung
- Multiple Mutable Borrows Erkennung
- Lifetime-Validierung
- Immutable Borrow Mutation Erkennung

**Integration:**
- Automatisch in `TypeCheckPass` integriert
- Prüft IR-Code nach Type-Checking

**Siehe:** [Borrow Checker Dokumentation](#borrow-checker-ownership--borrowing)

### Prompt Optimizer (90%+ Token-Ersparnis)

**Status:** ✅ Vollständig implementiert

Der Prompt Optimizer komprimiert LLM-Prompts automatisch:

- **Prompt-Kürzung** - Entfernt redundante Wörter
- **Token-Optimierung** - Verwendet Variablen statt Text
- **System-Prompt-Caching** - Wiederverwendung von System-Prompts
- **Kompakte Syntax** - `@llm.analyze(text)` statt langer Prompts

**Implementierung:** `compiler/src/prompt/`

**Beispiel:**
```velin
// Vorher (120 Tokens):
let prompt = "Bitte analysiere den folgenden Text, fasse ihn zusammen...";
let response = await llm.generate(prompt);

// Nachher (5-10 Tokens):
let response = await @llm.analyze(text);
```

**Ersparnis:** 90-95% Tokens

**Unterstützte Methoden:**
- `@llm.analyze(text)` - Text analysieren
- `@llm.summarize(text)` - Text zusammenfassen
- `@llm.extract(text, pattern)` - Informationen extrahieren
- `@llm.evaluate(text)` - Text bewerten
- `@llm.translate(text, lang)` - Text übersetzen
- `@llm.sentiment(text)` - Sentiment analysieren

**Siehe:** [Prompt Optimizer Dokumentation](#prompt-optimizer-90-token-ersparnis)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
