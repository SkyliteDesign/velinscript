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
