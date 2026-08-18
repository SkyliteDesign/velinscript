# Velisch

**Die Notizzettel-Programmiersprache für APIs.**

Du beschreibst möglichst klar *was* gebaut werden soll.  
Der Compiler (**VelinScript**, Befehl `velin`) übernimmt möglichst viel technische Umsetzung.

| Name | Rolle |
|------|--------|
| Velisch | Sprache / Produkt |
| VelinScript | Compiler / Core |
| velisch.info | Einstieg / Docs / Showcase |

## Umfang 3.5.1

VelinScript 3.5.1 konzentriert sich auf den stabilen API-Entwicklungsweg mit Rust/Axum (`velin run`, Port 8080).

Weitere Funktionen wie zusätzliche Target-Runtimes, JWT-Validierung, AI-Sandbox-Ausführung und weitere Runtime-Module befinden sich außerhalb dieses Stable-Umfangs.

## Warum Velisch?

- Kurze, deklarative API-Beschreibung (`@GET`, `@Auth`, …)
- Stabiler Compile-/Laufzeitpfad: **Rust/Axum**
- VelinScript 3.5 unterstützt mehrere Zielsprachen; weitere Targets sind Entwicklungs-/Experimentalstatus

## 5-Minuten-Quickstart

```velin
@GET("/hello")
fn hello(): string {
    return "Hello, World!";
}
```

```bash
velin run main.velin
# GET http://127.0.0.1:8080/hello
```

Oder nur Code erzeugen: `velin compile -i main.velin -o main.rs --show-code`.

Erwartung: Axum-Imports, `create_router()`, Route `GET /hello`.

## Dokumentation

- User-Guides und Tutorials: dieses Produkt-Repo (Velisch)
- Compiler-Architektur / IR / Passes: [VelinScript](https://github.com/SkyliteDesign/velinscript)

## Downloads

Binary-Releases kommen aus dem **VelinScript**-Release (Tag `v3.5.1`, sobald gepusht).  
`v3.5.0-ga` bleibt der vorherige Compiler-Stand.

## GitHub

- Produkt / Community: Velisch (dieses Repo)
- Compiler-Core: https://github.com/SkyliteDesign/velinscript

## Community

- Forum: https://forum.birdapi.de/forum/
- Discussions / Issues: über die jeweiligen GitHub-Repos

---

*Vorlage — Inhalt nach Velisch-Repo kopieren. Kein Push ohne Freigabe.*
