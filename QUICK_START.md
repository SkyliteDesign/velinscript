# VelinScript / Velisch Quick Start (3.5.1)

In wenigen Minuten zum ersten laufenden API-Endpoint.

**Velisch** ist die Sprache, **VelinScript** der Compiler (`velin`). Einstieg: [velisch.info](https://velisch.info) · Core: [GitHub velinscript](https://github.com/SkyliteDesign/velinscript).

VelinScript 3.5.1 konzentriert sich auf den stabilen API-Entwicklungsweg mit Rust/Axum. Weitere Funktionen wie zusätzliche Target-Runtimes, erweiterte Authentifizierung, AI-Sandbox-Ausführung und weitere Runtime-Module befinden sich außerhalb dieses Stable-Umfangs.

---

## Schritt 1: Installieren

Windows (Beispiel: lokales Prefix):

```powershell
.\install.ps1 -Prefix "$PWD\bin" -SourceRepo "."
$env:PATH = "$PWD\bin;$env:PATH"
velin --version
```

---

## Schritt 2: Projekt erstellen

```bash
velin new my-api
cd my-api
```

Das erzeugt `main.velin`, `Cargo.toml` und ein lauffähiges `src/main.rs`.

---

## Schritt 3: Code anpassen (optional)

```velin
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript!";
}
```

Nach Änderungen neu kompilieren:

```bash
velin compile -i main.velin -o src/main.rs
```

---

## Schritt 4: Prüfen

```bash
velin check -i main.velin
```

---

## Schritt 5: Starten

```bash
velin run main.velin
```

Dann: `GET http://127.0.0.1:8080/api/hello`.

`velin serve` schreibt nur ein Scaffold und startet keinen Server.

---

## Mehr erfahren

- [Multi-Target](docs/guides/multi-target.md)
- [Getting Started](docs/guides/getting-started.md)
- Beispiele unter `examples/00-simple-hello/`
