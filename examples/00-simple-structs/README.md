# 00-simple-structs - Structs Beispiel

**Zeigt grundlegende Struct-Verwendung in VelinScript.**

## Was macht dieses Beispiel?

Demonstriert:
- Struct-Definition
- Struct-Erstellung
- Struct als Rückgabewert

## Code

```velin
struct User {
    id: string,
    name: string,
    email: string,
    age: number,
}

@GET("/user")
fn getUser(): User {
    let user = User {
        id: "123",
        name: "John Doe",
        email: "john@example.com",
        age: 30,
    };
    
    return user;
}
```

## Kompilieren

```bash
velin compile -i main.velin -o main.rs
```

## Nächste Schritte

- **[Tutorial 1: Basics](../../docs/guides/tutorial-1-basics.md)** - Vollständiges Tutorial zu Structs
