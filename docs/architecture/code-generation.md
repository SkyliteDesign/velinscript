# Code-Generierung in VelinScript

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

VelinScript bietet umfangreiche Code-Generierung für **8 Zielsprachen** (Rust, PHP, Python, TypeScript, JavaScript, Go, Java, C#), die Entwicklern hilft, Boilerplate-Code zu vermeiden und schnell produktive APIs zu erstellen.

## Multi-Target Generierung

Der Compiler kann Code für verschiedene Zielplattformen und Frameworks generieren:

| Sprache | Frameworks | Besonderheiten | Status |
|---------|------------|----------------|--------|
| **Rust** | Axum, Actix | Async-first, High Performance | ✅ |
| **PHP** | Laravel, Symfony | Controller-Klassen, PSR-Standards | ✅ |
| **Python** | FastAPI, Flask | Pydantic Models, Type Hints | ✅ |
| **TypeScript** | Express, NestJS | Interfaces, Generics, Type Safety | ✅ |
| **JavaScript** | Express, Node.js | ES Modules, Async/Await | ✅ |
| **Go** | Gin, Echo | Goroutines, Error Handling | ✅ |
| **Java** | Spring Boot | POJOs, Lombok Support | ✅ |
| **C#** | ASP.NET Core | File-Scoped Namespaces, PascalCase | ✅ |

**Siehe:** [Multi-Target Compilation Dokumentation](./multi-target-compilation.md) für Details zu allen Targets.

## Boilerplate Generator

**Implementierung:** `compiler/src/codegen/boilerplate.rs`

### API Boilerplate

Generiert vollständige REST-API-Endpunkte:

```bash
velin generate api --name User --path /api/users
```

**Generiert:**
- GET `/api/users` - Liste aller Users
- POST `/api/users` - Neuen User erstellen
- GET `/api/users/:id` - User nach ID
- PUT `/api/users/:id` - User aktualisieren
- DELETE `/api/users/:id` - User löschen

### CRUD Generator

Generiert vollständige CRUD-Operationen für ein Modell:

```bash
velin generate crud --name Product --fields "id:string,name:string,price:number"
```

**Generiert:**
- Struct-Definition
- CRUD-Endpunkte
- Database-Integration
- Validation

## Client Generator

**Implementierung:** `compiler/src/codegen/client.rs`

Generiert TypeScript/JavaScript/Rust Clients aus OpenAPI-Spezifikationen:

```bash
velin generate client --openapi api.json --language typescript
```

**Unterstützte Sprachen:**
- TypeScript
- JavaScript
- Rust

**Features:**
- Automatische Methoden-Generierung aus OpenAPI Paths
- Type-Safe API-Calls
- Error Handling
- Request/Response-Typen

## AutoDoc Generator

**Implementierung:** `compiler/src/codegen/autodoc.rs`

**Status:** ✅ Vollständig implementiert

Generiert strukturierte Dokumentation aus `///` Doc-Comments:

**Features:**
- JSON-Dokumentation
- Knowledge Base Generation für RAG/LLM
- LLM-freundliche Kontextinformationen
- Integration mit `@VelinAutoDoc` Decorator

**Output-Format:**
```json
{
  "project": "VelinProject",
  "version": "1.0.0",
  "items": [...],
  "knowledge_base": [...]
}
```

## AutoTest Generator

**Implementierung:** `compiler/src/codegen/autotest.rs`

**Status:** ✅ Vollständig implementiert

Generiert automatisch Test-Stubs für Funktionen mit `@VelinAutoTest`:

**Features:**
- Mock-Daten-Generierung basierend auf Parametertypen
- Test-Stub-Generierung
- Grundlegende Assertions
- Integration in Codegen-Pipeline

**Generierter Code:**
```rust
#[tokio::test]
async fn test_auto_functionName() {
    let param1 = "mock_param1".to_string();
    let result = functionName(param1).await;
    assert!(result.is_ok(), "Function execution failed");
}
```

## Framework Codegen

**Implementierung:** `compiler/src/codegen/framework.rs`

**Status:** ✅ Vollständig implementiert

### Framework-Erkennung

- Automatische Erkennung aus Config
- Decorator-basierte Erkennung
- Default: Axum

### Code-Generierung

- Framework-spezifische Imports
- Router/App-Initialisierung
- Handler-Signaturen
- Middleware-Integration

## OpenAPI Generator

**Implementierung:** `compiler/src/codegen/openapi.rs`

**Status:** ✅ Vollständig implementiert

Generiert OpenAPI 3.0 Spezifikationen aus VelinScript-Code:

**Features:**
- Automatische Endpoint-Erkennung
- Parameter-Dokumentation
- Request/Response Schemas
- Security Requirements
- Operation IDs

**CLI:**
```bash
velin open-api -i main.velin -o api.json
```

---

## Beispiele

**Siehe:** [Multi-Target Compilation Beispiele](../examples/multi-target-examples.md) für detaillierte Beispiele aller Targets.

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
