# Tutorial 3: Security

Lerne, wie du Security in VelinScript implementierst.

## Authentication

### @Auth Decorator

Der `@Auth` Decorator erfordert Authentifizierung:

```velin
@Auth
@GET("/api/profile")
fn getProfile(): User {
    return currentUser();
}
```

Nur authentifizierte Benutzer können diesen Endpoint aufrufen.

## Authorization

### @Role Decorator

Der `@Role` Decorator erfordert eine bestimmte Rolle:

```velin
@Auth
@Role("admin")
@GET("/api/admin/users")
fn getAdminUsers(): List<User> {
    return db.findAll(User);
}
```

Nur Benutzer mit der Rolle "admin" können diesen Endpoint aufrufen.

## Kombination von Security Decorators

```velin
@Auth
@Role("user")
@GET("/api/users/:id")
fn getUser(id: string): User {
    // Nur authentifizierte Benutzer mit Rolle "user"
    return db.find(User, id);
}

@Auth
@Role("admin")
@GET("/api/admin/stats")
fn getAdminStats(): Stats {
    // Nur authentifizierte Benutzer mit Rolle "admin"
    return calculateStats();
}
```

## Public Endpoints

Endpoints ohne `@Auth` sind öffentlich:

```velin
@GET("/api/public/info")
fn getPublicInfo(): string {
    return "Public information";
}
```

## Best Practices

1. **Immer @Auth verwenden** für geschützte Endpoints
2. **Minimale Rechte** - Nur notwendige Rollen erfordern
3. **Input Validation** - Alle Inputs validieren
4. **Error Handling** - Keine sensiblen Informationen in Fehlermeldungen

## Beispiel: Vollständige Security

```velin
struct User {
    id: string,
    name: string,
    email: string,
    role: string,
}

// Public Endpoint
@GET("/api/public/health")
fn healthCheck(): string {
    return "OK";
}

// Authenticated Endpoint
@Auth
@GET("/api/profile")
fn getProfile(): User {
    return currentUser();
}

// User Role Required
@Auth
@Role("user")
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}

// Admin Role Required
@Auth
@Role("admin")
@GET("/api/admin/users")
fn getAdminUsers(): List<User> {
    return db.findAll(User);
}

@Auth
@Role("admin")
@DELETE("/api/admin/users/:id")
fn deleteUser(id: string): void {
    db.delete(User, id);
}
```

## Nächste Schritte

- [Tutorial 4: Database](tutorial-4-database.md) - Database Integration
