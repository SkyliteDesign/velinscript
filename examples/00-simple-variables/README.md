# 00-simple-variables - Variablen Beispiel

**Zeigt grundlegende Variablen-Verwendung in VelinScript.**

## Was macht dieses Beispiel?

Demonstriert verschiedene Arten von Variablen:
- Type Inference
- Explizite Typen
- Mutable Variablen

## Code

```velin
@GET("/test")
fn test(): string {
    // Type Inference
    let name = "John";
    let age = 30;
    let active = true;
    
    // Explizite Typen
    let email: string = "john@example.com";
    let score: number = 95.5;
    
    // Mutable Variable
    let mut counter = 0;
    counter = counter + 1;
    
    return "Name: {name}, Age: {age}, Active: {active}, Counter: {counter}";
}
```

## Kompilieren

```bash
velin compile -i main.velin -o main.rs
```

## Nächste Schritte

- **[Tutorial 1: Basics](../../docs/guides/tutorial-1-basics.md)** - Vollständiges Tutorial zu Variablen
