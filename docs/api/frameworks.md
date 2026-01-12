# Framework-Auswahl: Axum vs Actix-Web

VelinScript unterstützt beide modernen Rust HTTP-Frameworks.

## Framework-Auswahl

### Via Config

In `velin.config.json`:

```json
{
  "framework": "axum"
}
```

oder

```json
{
  "framework": "actix"
}
```

### Via Decorator

```velin
@Axum
@GET("/api/hello")
fn hello(): string {
    return "Hello, Axum!";
}
```

```velin
@Actix
@GET("/api/hello")
fn hello(): string {
    return "Hello, Actix!";
}
```

## Axum (Empfohlen für 2026)

**Vorteile:**
- Eng mit Tokio-Ökosystem verzahnt
- Type-safe Extractors
- Moderne API-Design
- Gute Performance

**Verwendung:**
```velin
@Axum
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

## Actix-Web (Performance-König)

**Vorteile:**
- Maximale Requests pro Sekunde
- Sehr niedrige Latenz
- Langjährig stabil
- Umfangreiches Middleware-System

**Verwendung:**
```velin
@Actix
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

## Default

Wenn kein Framework angegeben wird, verwendet VelinScript standardmäßig **Axum** (empfohlen für 2026).
