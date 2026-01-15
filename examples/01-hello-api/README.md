# 01-hello-api

**Einstieg in 3 Minuten.**

## Zweck

- Syntax zeigen
- API starten
- Keine KI, kein Overhead

## Inhalt

- 1 Datei `main.velin`
- 2 Endpunkte
- kein Setup-Wahnsinn

## Installation

```bash
# VelinScript installieren (falls noch nicht geschehen)
# Siehe: https://github.com/velinscript/velinscript#installation
```

## Kompilieren

```bash
cd examples/01-hello-api
velin compile -i main.velin -o main.rs
```

## Ausführen

```bash
# Kompiliertes Rust-Programm ausführen
cargo run --release
```

Die API läuft dann auf `http://localhost:8080`

## Testen

```bash
# Ping testen
curl http://localhost:8080/ping

# Hello testen
curl "http://localhost:8080/hello?name=Welt"
```

## Was du lernst

- Wie man HTTP-Endpunkte mit `@GET` definiert
- Wie man Query-Parameter verwendet
- Wie man String-Interpolation nutzt (`{name}`)
- Wie einfach VelinScript ist

## Nächste Schritte

- **KI testen?** → Siehe `02-llm-chat`
- **Automatisierung?** → Siehe `03-automation-pipeline`
- **Volles System?** → Siehe `04-custom-recommender`
