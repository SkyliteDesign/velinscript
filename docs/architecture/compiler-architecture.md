# VelinScript Compiler Architektur

**Version:** 2.5.0  
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
3. **TypeCheckPass** - Type Checking
4. **CodegenPass** - Code-Generierung

**Implementierung:**
- `compiler/src/passes/autofix.rs`
- `compiler/src/passes/parser.rs`
- `compiler/src/passes/type_check.rs`
- `compiler/src/passes/codegen.rs`

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

## Code-Generierung

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

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 2.5.0
