# API Documentation Generator

Eine gute API ist nur so gut wie ihre Dokumentation. VelinScript automatisiert diesen Prozess vollständig, indem es Ihre Code-Struktur, Typen und Kommentare analysiert und daraus eine interaktive OpenAPI (Swagger) Dokumentation generiert.

## Wofür ist der API Doc Generator ideal?

Der API Doc Generator ist ideal für:
- ✅ **Automatische Dokumentation** - Generiert OpenAPI/Swagger aus Code
- ✅ **Frontend-Integration** - Erstellt Dokumentation für Frontend-Teams
- ✅ **Client-Generierung** - Basis für automatische Client-Generierung
- ✅ **API-Versionierung** - Unterstützt Versionierung von APIs
- ✅ **Interaktive Dokumentation** - Swagger UI für API-Exploration
- ✅ **CI/CD-Integration** - Automatische Dokumentations-Updates in Pipelines

## Wofür ist der API Doc Generator NICHT gedacht?

Der API Doc Generator ist NICHT gedacht für:
- ❌ **Code-Qualität** - Für Code-Qualitätsprüfung nutzen Sie den Linter
- ❌ **Security-Checks** - Für Security nutzen Sie den Security Scanner
- ❌ **Performance-Analyse** - Für Performance nutzen Sie den Profiler
- ❌ **Unit-Tests** - Für Tests nutzen Sie den Test Runner
- ❌ **Code-Generierung** - Für Code-Generierung nutzen Sie `velin generate`

---

## Inhaltsverzeichnis

1.  [Grundlagen](#1-grundlagen)
2.  [Nutzung (`velin-api-doc`)](#2-nutzung-velin-api-doc)
3.  [Kommentare schreiben (JSDoc Style)](#3-kommentare-schreiben-jsdoc-style)
4.  [OpenAPI Integration](#4-openapi-integration)
5.  [Hosting der Dokumentation](#5-hosting-der-dokumentation)

---

## 1. Grundlagen

VelinScript nutzt einen "Code-First"-Ansatz. Sie schreiben Ihren Code und annotieren ihn bei Bedarf. Der Generator extrahiert:

*   **Routen:** Aus `@Controller`, `@GET`, `@POST` etc.
*   **Parameter:** Aus `@Body`, `@Query`, `@Path`.
*   **Modelle:** Aus Structs, die in der API verwendet werden.
*   **Beschreibungen:** Aus `///` Doc-Comments.
*   **Validierung:** Aus `@Validate`-Regeln (z.B. min/max werden zu OpenAPI constraints).

---

## 2. Nutzung (`velin-api-doc`)

Der Generator kann als Standalone-Tool oder über `velin open-api` genutzt werden.

```bash
# JSON generieren (für maschinelle Verarbeitung)
velin-api-doc generate -i src/main.velin -o openapi.json --format json

# HTML generieren (für Menschen)
velin-api-doc generate -i src/main.velin -o docs.html --format html

# Interaktiven Modus starten (Swagger UI)
velin-api-doc serve --port 8081
```

**Optionen:**
*   `--title`: Titel der API (überschreibt `velin.toml`).
*   `--version`: Version der API.
*   `--include-private`: Auch nicht-öffentliche Endpunkte dokumentieren.

---

## 3. Kommentare schreiben (JSDoc Style)

VelinScript versteht Markdown in Kommentaren. Nutzen Sie `///` (Triple-Slash) für Dokumentation.

```velin
/// Repräsentiert einen Benutzer im System.
/// 
/// Ein Benutzer kann mehrere Rollen haben und ist eindeutig durch seine ID identifiziert.
struct User {
    /// Die eindeutige UUID des Benutzers.
    /// @example "550e8400-e29b-41d4-a716-446655440000"
    id: string,
    
    /// Der vollständige Anzeigename.
    name: string
}

@Controller("/users")
struct UserController {

    /// Ruft alle Benutzer ab.
    ///
    /// Diese Methode unterstützt Paginierung via `limit` und `offset`.
    ///
    /// @param limit Maximale Anzahl der Ergebnisse (Standard: 20)
    /// @returns Eine Liste von Benutzern
    /// @throws 403 Wenn der Zugriff verweigert wird
    @GET("/")
    fn list(@Query("limit") limit: number = 20): List<User> {
        // ...
    }
}
```

---

## 4. OpenAPI Integration

Der generierte Output entspricht der OpenAPI Specification 3.0 (OAS3). Das bedeutet, Sie können ihn mit dem riesigen Ökosystem an Tools nutzen:

*   **Postman:** Importieren Sie die `openapi.json` direkt in Postman.
*   **Client Generatoren:** Nutzen Sie `openapi-generator` für Java, Python, Go, etc.
*   **API Gateways:** Konfigurieren Sie Kong oder Tyk automatisch.

**Spezifische OpenAPI-Erweiterungen:**
Sie können auch rohe OpenAPI-Felder injizieren:

```velin
@OpenApiExtension("x-amazon-apigateway-auth", { "type": "none" })
@GET("/health")
fn health() { ... }
```

---

## 5. Hosting der Dokumentation

In einer VelinScript-Webanwendung können Sie die Dokumentation direkt einbetten.

```velin
// In Ihrer main.velin
use http
use api_doc

fn main() {
    let app = http.Server.new();
    
    // Aktiviert /docs/swagger und /docs/json
    app.use(api_doc.SwaggerUI({
        path: "/docs",
        title: "My API",
        specFile: "./openapi.json" // Optional: Generiert zur Laufzeit wenn leer
    }));
    
    app.listen(8080);
}
```

Dies ist extrem nützlich für interne APIs, da die Dokumentation immer synchron mit dem deployten Code ist.

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript API Doc Generator                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-api-doc generate -i main.velin                │
│                                                         │
│  📚 Generiere API-Dokumentation...                     │
│                                                         │
│  ✓ 15 Endpunkte gefunden                               │
│  ✓ 8 Models extrahiert                                 │
│  ✓ OpenAPI 3.0 Spezifikation generiert                 │
│                                                         │
│  📄 openapi.json gespeichert                           │
│  🌐 Swagger UI verfügbar unter /docs                   │
│                                                         │
│  [Swagger UI Preview]                                   │
│    GET  /api/users          - Liste aller Benutzer     │
│    POST /api/users          - Erstelle Benutzer        │
│    GET  /api/users/:id      - Hole Benutzer            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```
