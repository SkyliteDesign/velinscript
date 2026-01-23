# Framework-Übersicht

VelinScript unterstützt moderne Web-Frameworks in **Rust**, **PHP** und **Python**.

## Rust Frameworks

### Axum (Default, Empfohlen)
- **Vorteile:** Eng mit Tokio verzahnt, Type-safe Extractors.
- **Verwendung:** `@Axum` oder `framework: "axum"`

### Actix-Web
- **Vorteile:** Maximale Performance, stabiles Ökosystem.
- **Verwendung:** `@Actix` oder `framework: "actix"`

## PHP Frameworks

### Laravel (Default für PHP)
- **Generierung:** Erstellt `AppController`-Klasse und registriert Routen via `Route::get/post/...`.
- **Vorteile:** Standard für modernes PHP, Eloquent-ready (via Velin ORM).
- **Verwendung:** `@Laravel` oder `framework: "laravel"`

### Symfony
- **Generierung:** Erstellt Controller mit PHP 8 Attributen (`#[Route]`).
- **Vorteile:** Enterprise-Standard, sauber getrennte Komponenten.
- **Verwendung:** `@Symfony` oder `framework: "symfony"`

## Python Frameworks

### FastAPI (Default für Python)
- **Generierung:** Nutzt Pydantic (`BaseModel`) für Typ-Validierung.
- **Vorteile:** Automatische OpenAPI-Doku, Type-Hints, Async-Support.
- **Verwendung:** `@FastAPI` oder `framework: "fastapi"`

### Flask
- **Generierung:** Klassische View-Functions und `app.add_url_rule`.
- **Vorteile:** Einfachheit, riesiges Ökosystem.
- **Verwendung:** `@Flask` oder `framework: "flask"`

## Go Frameworks

### Gin (Default für Go)
- **Generierung:** JSON-Struct-Tags, Gin-Handler mit Kontext (`*gin.Context`).
- **Vorteile:** Performance (Martini-API), Middleware-Support, JSON-Validierung.
- **Verwendung:** `@Gin` oder `framework: "gin"`

## Konfiguration

In `velin.config.json`:

```json
{
  "target": "go",      // "rust", "php", "python", "go", "ts", "java", "csharp"
  "framework": "gin"   // Optional, überschreibt Default
}
```
