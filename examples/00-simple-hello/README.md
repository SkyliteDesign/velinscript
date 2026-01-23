# 00-simple-hello - Einfachstes Hello World

**Das einfachste Beispiel für absolute Anfänger.**

## Was macht dieses Beispiel?

Ein einziger API-Endpoint, der "Hello, World!" zurückgibt.

## Code

```velin
@GET("/hello")
fn hello(): string {
    return "Hello, World!";
}
```

## Kompilieren

```bash
velin compile -i main.velin -o main.rs
```

## Ausführen

```bash
cargo build --release
cargo run --release
```

## Testen

```bash
curl http://localhost:8080/hello
```

**Erwartete Antwort:**
```
Hello, World!
```

## Nächste Schritte

- **[01-hello-api](../01-hello-api/)** - Etwas mehr Features
- **[Getting Started Guide](../../docs/guides/getting-started.md)** - Vollständige Anleitung
