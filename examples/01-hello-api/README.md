# 01-hello-api

**Einstieg in 3 Minuten.**

## Zweck

- Syntax zeigen
- API starten
- Keine KI, kein Overhead

## Inhalt

- 1 Datei `main.velin` (offizielle Endung; `.vel` ist nur ein Lese-Alias)
- 2 Endpunkte
- kein Setup-Wahnsinn

## Starten

```bash
cd examples/01-hello-api
velin run main.velin
```

Die API läuft auf `http://127.0.0.1:8080`.

`velin serve -i main.velin` schreibt nur ein Scaffold nach `.velin/serve-scaffold/` und startet den Server nicht.

## Testen

```bash
curl http://127.0.0.1:8080/ping

curl "http://127.0.0.1:8080/hello?name=Velin"
```

Erwartet: `ok` bzw. `Hello Velin`.

## Was du lernst

- HTTP-Endpunkte mit `@GET`
- Query-Parameter (`name`)
- String-Interpolation (`{name}`)
