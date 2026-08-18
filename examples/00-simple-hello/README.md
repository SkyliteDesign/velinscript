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
velin run main.velin
```

Die API läuft auf `http://127.0.0.1:8080`. `velin serve -i main.velin` schreibt nur ein Scaffold nach `.velin/serve-scaffold/` und startet den Server nicht.

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
