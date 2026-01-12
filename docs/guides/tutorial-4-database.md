# Tutorial 4: Database Integration

Lerne, wie du Datenbanken mit VelinScript verwendest.

## Database Funktionen

### Entity finden

```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

### Alle Entities finden

```velin
@GET("/api/users")
fn getUsers(): List<User> {
    return db.findAll(User);
}
```

### Entity speichern

```velin
@POST("/api/users")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
    };
    return db.save(user);
}
```

### Entity löschen

```velin
@DELETE("/api/users/:id")
fn deleteUser(id: string): void {
    db.delete(User, id);
}
```

## Vollständiges CRUD Beispiel

```velin
struct User {
    id: string,
    name: string,
    email: string,
    createdAt: string,
}

@GET("/api/users")
fn getUsers(): List<User> {
    return db.findAll(User);
}

@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}

@POST("/api/users")
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
        createdAt: getCurrentTimestamp(),
    };
    return db.save(user);
}

@PUT("/api/users/:id")
fn updateUser(id: string, name: string, email: string): User {
    let user = db.find(User, id);
    user.name = name;
    user.email = email;
    return db.save(user);
}

@DELETE("/api/users/:id")
fn deleteUser(id: string): void {
    db.delete(User, id);
}
```

## Best Practices

1. **Immer Typen angeben** für Entities
2. **Error Handling** für nicht gefundene Entities
3. **Validation** vor dem Speichern
4. **Transactions** für mehrere Operationen (geplant)

## Nächste Schritte

- [API Documentation](../api/) - Vollständige API-Referenz
- [Language Specification](../language/specification.md) - Sprach-Spezifikation
