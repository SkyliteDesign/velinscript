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

## Ausführen (empfohlen)

```bash
# Compiler bauen
cargo build --release -p velin-compiler

# Scaffold erzeugen und starten
velin serve -i main.velin
cd .velin/serve-scaffold
cargo run
```

Alternativ nur den Rust-Code erzeugen:

```bash
velin compile -i main.velin -o main.rs
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
