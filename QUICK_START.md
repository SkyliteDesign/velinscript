# 🚀 VelinScript Quick Start

**5 Minuten bis zu deinem ersten API-Endpoint!**

---

## Schritt 1: Projekt erstellen

```bash
# Beide Befehle funktionieren:
velin new my-api
# oder
velin init my-api

cd my-api
```

---

## Schritt 2: Code schreiben

Öffne `main.velin` und schreibe:

```velin
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript! 🚀";
}
```

---

## Schritt 3: Kompilieren

```bash
velin compile -i main.velin -o main.rs
```

---

## Schritt 4: Prüfen

```bash
velin check -i main.velin
```

---

## Schritt 5: Ausführen

```bash
# Rust-Code kompilieren
cd ..
cargo build --release --manifest-path my-api/Cargo.toml

# Ausführen
cargo run --release --manifest-path my-api/Cargo.toml
```

---

## 🎉 Fertig!

Deine API läuft jetzt auf `http://localhost:8080/api/hello`

---

## 🔑 Mit LLM-Features?

1. **API-Key setzen:**
   ```bash
   # Windows
   $env:OPENAI_API_KEY = "sk-..."
   
   # Linux/Mac
   export OPENAI_API_KEY="sk-..."
   ```

2. **Code erweitern:**
   ```velin
   @POST("/api/chat")
   fn chat(message: string): string {
       let llm = LLMClient.new("openai", config.get_env("OPENAI_API_KEY", ""));
       return await llm.generate(message);
   }
   ```

Siehe [API-Keys Setup](docs/guides/api-keys-setup.md) für Details.

---

## 📚 Nächste Schritte

- **[Getting Started Guide](docs/guides/getting-started.md)** - Vollständige Anleitung
- **[Tutorials](docs/guides/)** - Schritt-für-Schritt Tutorials
- **[Beispiele](examples/)** - Code-Beispiele
- **[API-Referenz](docs/api/standard-library.md)** - Alle Funktionen

---

## 🆘 Hilfe

- **[Dokumentations-Übersicht](docs/README.md)** - Alle Dokumente
- **[Häufige Probleme](docs/guides/getting-started.md#häufige-probleme)** - Lösungen
- **[Forum](https://forum.birdapi.de/forum/)** - Community-Support

---

**Version:** 3.1.0
