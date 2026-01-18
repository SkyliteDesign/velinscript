# API Documentation - Decorators

Decorators sind Metadaten, die Funktionen annotieren und deren Verhalten steuern.

## HTTP Decorators

### @GET

Definiert einen GET-Endpoint.

```velin
@GET("/api/users")
fn getUsers(): List<User> {
    return db.findAll(User);
}
```

**Parameter:**
- `path` (string, required) - URL-Pfad

**Transformiert zu:**
```rust
#[get("/api/users")]
fn get_users() -> Vec<User> {
    // ...
}
```

### @POST

Definiert einen POST-Endpoint.

```velin
@POST("/api/users")
fn createUser(name: string, email: string): User {
    // ...
}
```

### @PUT

Definiert einen PUT-Endpoint.

```velin
@PUT("/api/users/:id")
fn updateUser(id: string, name: string): User {
    // ...
}
```

### @DELETE

Definiert einen DELETE-Endpoint.

```velin
@DELETE("/api/users/:id")
fn deleteUser(id: string): void {
    db.delete(User, id);
}
```

### @PATCH

Definiert einen PATCH-Endpoint.

```velin
@PATCH("/api/users/:id")
fn patchUser(id: string, updates: UserUpdate): User {
    // ...
}
```

## Security Decorators

### @Auth

Erfordert Authentifizierung für den Endpoint.

```velin
@Auth
@GET("/api/profile")
fn getProfile(): User {
    return currentUser();
}
```

**Transformiert zu:**
```rust
#[actix_web::web::middleware(AuthMiddleware)]
fn get_profile() -> User {
    // ...
}
```

### @Role

Erfordert eine bestimmte Rolle.

```velin
@Auth
@Role("admin")
@GET("/api/admin/users")
fn getAdminUsers(): List<User> {
    return db.findAll(User);
}
```

**Parameter:**
- `role` (string, required) - Erforderliche Rolle

**Transformiert zu:**
```rust
#[actix_web::web::middleware(RoleMiddleware::new("admin"))]
fn get_admin_users() -> Vec<User> {
    // ...
}
```

## Performance Decorators

### @Cache

Aktiviert Caching für den Endpoint.

```velin
@Cache(ttl: "5m", key: "user:{id}")
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

**Parameter:**
- `ttl` (string, optional) - Time-to-Live (z.B. "5m", "1h")
- `key` (string, optional) - Cache-Key Pattern

## SEO Decorators

### @SEO

Definiert SEO-Metadaten für den Endpoint.

```velin
@SEO(title: "Product: {name}", description: "{description}")
@GET("/products/:id")
fn getProduct(id: string): Product {
    return db.find(Product, id);
}
```

**Parameter:**
- `title` (string, optional) - SEO Title
- `description` (string, optional) - SEO Description

## AI/ML Decorators

### @AI

Aktiviert KI/ML-Funktionalität.

```velin
@AI(model: "sentiment")
@POST("/api/analyze")
fn analyze(text: string): Sentiment {
    return model.predict(text);
}
```

**Parameter:**
- `model` (string, required) - Model-Name

## Intelligence Decorators

### @Flow

Markiert eine Funktion als transaktionalen "Flow". VelinScript verwaltet automatisch den Zustand, Snapshots und Rollbacks.

**Neu in Version 2.5**: Vollständig implementiertes VelinFlow Runtime System ✅

```velin
@Flow
@POST("/orders")
fn createOrder(input: OrderInput): OrderResult {
    // Automatischer Snapshot des Inputs
    // Automatischer Rollback bei Fehler
    // Automatisches State-Tracking
    flow.snapshot_input(input);
    return processOrder(input);
}
```

**Features:**
- Automatisches State-Tracking (Pending, Running, Completed, Failed, Compensating, Compensated)
- Input-Snapshot-Management für Rollback
- Automatisches Commit bei Erfolg
- Automatisches Rollback mit Compensation-Logic bei Fehler
- Logging der Ausführungsdauer und Status
- Self-Healing durch Compensation-Hooks

**Verfügbare Funktionen:**
- `flow.snapshot_input(input: any) -> void`: Manuelles Aufzeichnen eines Input-Snapshots

### @VelinAutoDoc

Weist den Compiler an, automatisch strukturierte Dokumentation (JSON) für diese Funktion, Struktur oder dieses Modul zu generieren, einschließlich KI-freundlicher Kontextinformationen.

**Neu in Version 2.5**: Vollständig implementiertes VelinAutoDoc System ✅

```velin
/// Erstellt einen neuen Benutzer
/// 
/// @param name - Der Name des Benutzers
/// @returns Ein User-Objekt mit generierter ID
@VelinAutoDoc
fn createUser(name: string): User {
    // ...
}
```

**Features:**
- Erfasst `///` Doc-Comments als First-Class-Citizens im AST
- Generiert strukturierte JSON-Dokumentation
- Extrahiert Typ-Signaturen, Parameter und Return-Types
- Erstellt `llm_prompt_context` für KI-gestützte Dokumentationsgenerierung
- Unterstützt Funktionen, Structs und Module

**Output-Format:**
```json
{
  "name": "createUser",
  "signature": "fn(name: string) -> User",
  "doc_comments": ["Erstellt einen neuen Benutzer", ...],
  "parameters": [...],
  "return_type": "User",
  "llm_prompt_context": "..."
}
```

### @VelinAutoTest

**Neu in Version 2.5** ✅

Generiert automatisch Unit-Tests mit Mock-Daten für die markierte Funktion.

```velin
@VelinAutoTest
fn calculateTotal(items: List<Item>): number {
    // Velin erstellt Testfälle mit leeren Listen, großen Listen, etc.
    // ...
}
```

**Features:**
- Automatische Test-Stub-Generierung
- Mock-Daten-Generierung basierend auf Parametertypen
- Generiert Rust-Test-Code mit `#[tokio::test]`
- Grundlegende Assertions
- Integration in Codegen-Pipeline

**Generierter Test:**
```rust
#[tokio::test]
async fn test_auto_calculateTotal() {
    let items = vec![];
    let result = calculateTotal(items).await;
    assert!(result.is_ok(), "Function execution failed");
}
```

### @VelinPipeline

Aktiviert den Pipeline-Optimizer für ein Modul oder eine Funktion. Erkennt unabhängige `await`-Aufrufe und parallelisiert sie automatisch.

**Neu in Version 2.5**: Vollständig implementiertes VelinPipeline System ✅

```velin
@VelinPipeline
async fn loadDashboard() {
    // Werden automatisch parallel ausgeführt
    let user = await getUser();
    let stats = await getStats();
    let recommendations = await getRecommendations();
    return { user, stats, recommendations };
}
```

**Features:**
- Analysiert Datenabhängigkeiten zwischen Statements
- Erkennt automatisch unabhängige async Operationen
- Optimiert sequentielle Aufrufe zu parallelen Ausführungsgruppen
- Generiert automatisch `tokio::join!` für unabhängige Operationen
- Verbessert Performance durch Parallelisierung

**Beispiel-Transformation:**
```velin
// Vorher (sequentiell)
let a = await op1();
let b = await op2(); // Wartet auf op1
let c = await op3(); // Wartet auf op2

// Nachher (parallel mit @VelinPipeline)
let (a, b, c) = tokio::join!(op1(), op2(), op3());
```

### @VelinInsight

**Neu in Version 2.5** ✅

Markiert ein Projekt oder Modul für die tiefgehende Code-Analyse (Code-Qualität, Dead Code, Komplexität).

```velin
@VelinInsight
mod services {
    // VelinInsight analysiert dieses Modul
    // ...
}
```

**Features:**
- Erkennt ungenutzte Structs
- Identifiziert komplexe Funktionen (Statement Count > 20)
- Findet redundante Datenbank-Queries
- Generiert InsightReport mit Empfehlungen
- VS Code Extension Integration

**Output:**
```json
{
  "unused_structs": ["OldStruct"],
  "complex_functions": ["processOrder"],
  "redundant_queries": ["db.find() in loop"]
}
```

## Testing Decorators

### @test

Markiert eine Funktion als Test.

```velin
@test
fn testGetUser() {
    let user = getUser("123");
    assert(user.name == "John");
}
```

**Transformiert zu:**
```rust
#[test]
fn test_get_user() {
    // ...
}
```

## Decorator Kombinationen

Mehrere Decorators können kombiniert werden:

```velin
@Auth
@Role("admin")
@Cache(ttl: "10m")
@GET("/api/admin/stats")
fn getAdminStats(): Stats {
    return calculateStats();
}
```

Die Reihenfolge ist wichtig:
1. Security Decorators (@Auth, @Role)
2. Performance Decorators (@Cache)
3. HTTP Decorators (@GET, @POST, etc.)
4. Andere Decorators (@SEO, @AI)

## OpenAPI Integration

Alle HTTP Decorators werden automatisch in OpenAPI Specifications umgewandelt:

```bash
velin-compiler open-api -i main.velin -o api.json
```

Die generierte Spec enthält:
- Alle Endpoints mit Methoden und Pfaden
- Parameter (Path, Query, Body)
- Request/Response Schemas
- Security Requirements (aus @Auth, @Role)
- Operation IDs

Siehe [OpenAPI Documentation](openapi.md) für Details.
