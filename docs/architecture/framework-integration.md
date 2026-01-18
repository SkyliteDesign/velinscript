# Framework-Integration in VelinScript

**Version:** 2.5.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

VelinScript unterstützt automatische Framework-Erkennung und Code-Generierung für moderne Rust HTTP-Frameworks.

## Unterstützte Frameworks

### Axum (Default)

**Status:** ✅ Vollständig unterstützt

- Moderne, async-first Architektur
- Type-safe Routing
- Automatische Request/Response-Handling
- Middleware-Integration

### Actix-Web

**Status:** ✅ Vollständig unterstützt

- Production-Ready Framework
- Hohe Performance
- Umfangreiche Middleware-Unterstützung

## Framework-Erkennung

Der Compiler erkennt das Framework auf drei Arten:

### 1. Config-basiert

```json
// velin.config.json
{
  "framework": "axum"
}
```

### 2. Decorator-basiert

```velin
@Axum
@GET("/api/users")
fn getUsers(): List<User> {
    // ...
}
```

### 3. Default

Wenn nichts angegeben ist, verwendet der Compiler **Axum** als Default.

## Code-Generierung

### Automatische Imports

Der Compiler generiert automatisch Framework-spezifische Imports:

**Axum:**
```rust
use axum::{
    Router, extract::{Path, Query, Json, State},
    routing::{get, post, put, delete},
    response::Response,
    http::StatusCode
};
```

**Actix-Web:**
```rust
use actix_web::{web, HttpResponse, Responder, HttpRequest};
```

### Router-Initialisierung

**Axum:**
```rust
pub fn create_router() -> Router {
    Router::new()
        .route("/api/users", get(get_users_handler))
        .route("/api/users", post(create_user_handler))
}
```

**Actix-Web:**
```rust
pub fn create_app() -> App {
    App::new()
        .route("/api/users", web::get().to(get_users_handler))
        .route("/api/users", web::post().to(create_user_handler))
}
```

### Handler-Signaturen

Der Compiler generiert Framework-spezifische Handler-Signaturen:

**Axum:**
```rust
async fn get_users_handler() -> impl IntoResponse {
    // ...
}
```

**Actix-Web:**
```rust
async fn get_users_handler(req: HttpRequest) -> impl Responder {
    // ...
}
```

## Parameter-Extraktion

### Path-Parameter

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    // ...
}
```

**Axum:**
```rust
async fn get_user_handler(Path(id): Path<String>) -> impl IntoResponse {
    // ...
}
```

**Actix-Web:**
```rust
async fn get_user_handler(req: HttpRequest) -> impl Responder {
    let id = req.match_info().get("id").unwrap();
    // ...
}
```

### Query-Parameter

```velin
@GET("/api/users")
fn getUsers(limit: number): List<User> {
    // ...
}
```

**Axum:**
```rust
async fn get_users_handler(Query(params): Query<GetUsersParams>) -> impl IntoResponse {
    // ...
}
```

### Body-Parameter

```velin
@POST("/api/users")
fn createUser(user: UserInput): User {
    // ...
}
```

**Axum:**
```rust
async fn create_user_handler(Json(user): Json<UserInput>) -> impl IntoResponse {
    // ...
}
```

## Error Handling

### Global Error Handler

Der Compiler generiert automatisch einen globalen Error Handler:

```rust
struct AppError(anyhow::Error);

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // Konvertiert Fehler in saubere JSON-Responses
        // Status Code: 500 für interne Fehler
    }
}
```

### Result-Typen

```velin
@GET("/api/users/:id")
fn getUser(id: string): Result<User, string> {
    // ...
}
```

Der Compiler konvertiert automatisch:
- `Ok(value)` → 200 OK mit JSON-Body
- `Error(message)` → 400 Bad Request oder 500 Internal Server Error

## Middleware-Integration

### Auth Middleware

```velin
@Auth
@GET("/api/profile")
fn getProfile(): User {
    // ...
}
```

Der Compiler generiert automatisch Auth-Middleware-Integration für beide Frameworks.

### Rate Limiting

```velin
@RateLimit(requests: 100, window: "1m")
@GET("/api/users")
fn getUsers(): List<User> {
    // ...
}
```

## Observability

### Structured Logging

Alle Handler werden automatisch mit `#[tracing::instrument]` versehen:

```rust
#[tracing::instrument]
async fn get_users_handler() -> impl IntoResponse {
    // Automatisches Logging:
    // - Request-ID
    // - Latenz
    // - Parameter
}
```

## Implementierung

**Datei:** `compiler/src/codegen/framework.rs`

**Features:**
- `FrameworkSelector::detect_framework()` - Framework-Erkennung
- `FrameworkSelector::generate_imports()` - Import-Generierung
- `FrameworkSelector::generate_app_init()` - Router/App-Initialisierung
- `FrameworkSelector::generate_handler_signature()` - Handler-Signaturen

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 2.5.0
