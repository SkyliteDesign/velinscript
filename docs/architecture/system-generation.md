# System-Generierung (Version 3.1)

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

Der SystemGenerator erkennt High-Level APIs und generiert vollständige, produktionsreife Systeme automatisch - ohne Boilerplate-Code.

## SystemGenerator

**Datei:** `compiler/src/codegen/system_generator.rs`

**Funktionalität:**
- Erkennt API-Typ automatisch (Chatbot, Database, Auth, REST, Custom)
- Analysiert System-Anforderungen
- Generiert vollständige Komponenten
- Erstellt Integration-Code
- Generiert Deployment-Configs

## API-Typ-Erkennung

Der Generator erkennt automatisch den API-Typ durch Analyse des Codes:

**Erkannte API-Typen:**
- **Chatbot**: Erkennt `llm.chat()`, `llm.generate()`, `@llm.*` Calls
- **Database**: Erkennt `db.find()`, `db.save()`, `db.findAll()` Calls
- **Authentication**: Erkennt `auth.login()`, `auth.verifyToken()`, `@Auth` Decorator
- **REST**: Erkennt `@GET`, `@POST`, `@PUT`, `@DELETE` Decorators
- **Custom**: Fallback für andere APIs

**Beispiele:**

```velin
// Wird als Chatbot erkannt
@GET("/api/chat")
fn chat(message: string): string {
    return await @llm.analyze(message);
}
```

```velin
// Wird als Database API erkannt
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

```velin
// Wird als Auth API erkannt
@POST("/api/login")
fn login(username: string, password: string): Result<Token, string> {
    return auth.login(username, password);
}
```

## Generierte Komponenten

### Basis-Komponenten (immer)
- **Server**: Axum/Actix-Web Server
- **Routing**: Automatisches Routing aus Decorators
- **Error Handling**: Strukturierte Fehlerbehandlung
- **Logging**: Structured Logging mit Tracing

### Optionale Komponenten
- **Authentication**: JWT/OAuth2 (wenn `needs_auth`)
- **Rate Limiting**: Redis-basiert (wenn `needs_rate_limit`)
- **AI Client**: LLM-Integration via `AIClient` (OpenAI, Anthropic) (wenn `needs_ai`)
- **Database**: Connection Pool & Auth-Integration (wenn `needs_database`)
- **Caching**: Redis Cache (wenn `needs_caching`)

### 3. Automatische Routing-Generierung

Der Generator analysiert die System-Anforderungen (`Requirements`) und erstellt automatisch den passenden Routing-Code für den Webserver (z.B. Axum).

```rust
// Generierter Code (Beispiel)
pub fn add_routes(router: Router) -> Router {
    router
        .route("/health", get(|| async { "OK" }))
        .route("/chat", post(crate::handlers::chat))
        .route("/login", post(crate::handlers::login))
        .route("/register", post(crate::handlers::register))
        .route("/items", get(crate::handlers::list_items))
        .route("/items", post(crate::handlers::create_item))
}
```

### 4. AI Client Integration (Neu in 3.1)

Wenn `needs_ai` erkannt wird (z.B. durch `llm.chat()`), generiert der SystemGenerator automatisch einen `AIClient`, der mit echten LLM-APIs (OpenAI, etc.) kommuniziert.
Der Code nutzt `reqwest` für HTTP-Calls und verarbeitet Umgebungsvariablen (`OPENAI_API_KEY`).

```rust
// Generierter Code (Auszug)
let client = AIClient::new(LLMProvider::OpenAI, api_key);
let response = client.generate(&message).await?;
```

### 5. Database Integration (Neu in 3.1)

Wenn `needs_database` erkannt wird, generiert das System automatisch einen `Database` Connection Pool (via `sqlx`) und integriert diesen in die Handler.

- **Login-Handler**: Enthält vorbereiteten Code für Datenbank-Abfragen zur Authentifizierung.
- **CRUD-Handler**: Generiert Beispiel-Handler (`list_items`, `create_item`) mit Datenbank-Zugriff.

> **Hinweis**: Der generierte Datenbank-Code enthält Sicherheits-Best-Practices (z.B. Prepared Statements Hinweise), muss aber für das spezifische Schema angepasst werden.

### 6. Deployment-Konfiguration
- **Dockerfile**: Multi-stage Build
- **docker-compose.yml**: Mit Dependencies (Redis, Postgres)
- **Kubernetes**: Manifests mit Auto-Scaling
- **Helm Charts**: Für komplexe Deployments

## Component Templates

**Datei:** `compiler/src/codegen/templates/`

### APIServerTemplate
Generiert vollständigen Server mit:
- Axum oder Actix-Web
- Middleware-Stack
- CORS-Konfiguration
- Health-Check Endpoint

### AuthTemplate
Generiert Authentication mit:
- JWT Token Generation/Validation
- OAuth2 Integration
- Middleware für geschützte Routes

### RateLimitTemplate
Generiert Rate Limiting mit:
- Fixed Window
- Sliding Window
- Token Bucket

### AIClientTemplate
Generiert LLM-Client mit:
- OpenAI/Anthropic/Gemini Support
- Chat Completion
- Text Generation

### DeploymentTemplate
Generiert Infrastructure-as-Code:
- Dockerfile
- docker-compose.yml
- Kubernetes Manifests
- Serverless Configs

## Verwendung

```rust
use velin_compiler::codegen::system_generator::{SystemGenerator, APICall};

let generator = SystemGenerator::new(llm_client);
let api_call = APICall {
    name: "chatbot".to_string(),
    args: vec!["prompt".to_string()],
};

let system = generator.generate_system(&api_call)?;

// System enthält:
// - system.components: Alle generierten Komponenten
// - system.integration_code: Integration-Code
// - system.deployment_config: Deployment-Config
```

## Infrastructure Generator

**Datei:** `compiler/src/codegen/infrastructure.rs`

Generiert Infrastructure-as-Code basierend auf Deployment-Plan:

- **Local**: Keine Infrastructure
- **CloudSingle**: Dockerfile + docker-compose.yml
- **CloudMulti**: Kubernetes + Helm Charts
- **Serverless**: Lambda + API Gateway

## Beispiel

**Input:**
```velin
@GET("/api/chat")
fn chat(message: string): string {
    return llm.chat(message);
}
```

**Generiertes System:**
- API Server (Axum)
- Routing
- AI Client (LLM Integration)
- Authentication (JWT)
- Rate Limiting
- Error Handling
- Logging
- Dockerfile
- docker-compose.yml

## Best Practices

1. **API-Typ klar definieren**: Nutze aussagekräftige Funktionsnamen
2. **Requirements spezifizieren**: Decorators helfen bei der Erkennung
3. **Deployment-Plan prüfen**: Generierte Configs sollten reviewt werden
4. **Komponenten anpassen**: Templates können erweitert werden

---

## Siehe auch

- [KI-Compiler-Passes](./ai-compiler-passes.md)
- [Infrastructure as Code](./infrastructure.md)
- [Component Templates](../api/templates.md)
