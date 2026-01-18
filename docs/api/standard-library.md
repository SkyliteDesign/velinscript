# Standard Library API Reference

VelinScript provides a rich standard library for common tasks. Version 2.5 includes 50+ modules with over 150+ functions.

**Neu in Version 2.5**: 13 neue Module mit 117+ Funktionen hinzugefügt ✅

## Table of Contents

- [String](#string)
- [Math](#math)
- [Date](#date)
- [FileSystem](#filesystem)
- [LLM](#llm)
- [Embedding](#embedding)
- [Agent](#agent)
- [Process](#process)
- [Sandbox](#sandbox)
- [Rollback](#rollback)
- [HTTP](#http)
- [WebSocket](#websocket)
- [Utils](#utils)
- [Logging](#logging)
- [Config](#config)
- [Flow](#flow)
- [AutoDoc](#autodoc)
- [Pipeline](#pipeline)

## String

Global object: `string`

**Neu in Version 2.5** ✅

Erweiterte String-Manipulation für Textverarbeitung.

- `split(text: string, delimiter: string) -> List<string>` - Teilt einen String an einem Delimiter
- `join(list: List<string>, delimiter: string) -> string` - Verbindet eine Liste von Strings
- `replace(text: string, old: string, new: string) -> string` - Ersetzt Teilstrings
- `trim(text: string) -> string` - Entfernt Whitespace am Anfang und Ende
- `slugify(text: string) -> string` - Konvertiert Text zu URL-freundlichem Slug
- `to_int(text: string) -> Result<number, string>` - Konvertiert String zu Integer
- `to_float(text: string) -> Result<number, string>` - Konvertiert String zu Float
- `capitalize(text: string) -> string` - Macht ersten Buchstaben groß
- `lowercase(text: string) -> string` - Konvertiert zu Kleinbuchstaben
- `uppercase(text: string) -> string` - Konvertiert zu Großbuchstaben
- `starts_with(text: string, prefix: string) -> boolean` - Prüft ob String mit Prefix beginnt
- `ends_with(text: string, suffix: string) -> boolean` - Prüft ob String mit Suffix endet

**Beispiel:**
```velin
let parts = string.split("hello,world,test", ",");
let joined = string.join(parts, "-");
let slug = string.slugify("Hello World!"); // "hello-world"
```

## Math

Global object: `math`

**Neu in Version 2.5** ✅

Mathematische Utilities für Berechnungen.

- `clamp(value: number, min: number, max: number) -> number` - Begrenzt Wert auf Bereich
- `lerp(a: number, b: number, t: number) -> number` - Lineare Interpolation
- `round_to(value: number, decimals: number) -> number` - Rundet auf Dezimalstellen
- `random_range(min: number, max: number) -> number` - Zufällige Zahl im Bereich
- `min(a: number, b: number) -> number` - Minimum von zwei Werten
- `max(a: number, b: number) -> number` - Maximum von zwei Werten
- `abs(value: number) -> number` - Absoluter Wert
- `floor(value: number) -> number` - Abrunden
- `ceil(value: number) -> number` - Aufrunden

**Beispiel:**
```velin
let clamped = math.clamp(150, 0, 100); // 100
let random = math.random_range(1, 10);
let rounded = math.round_to(3.14159, 2); // 3.14
```

## Date

Global object: `date`

**Neu in Version 2.5** ✅

Erweiterte Datum- und Zeit-Operationen.

- `add_days(timestamp: number, days: number) -> number` - Fügt Tage hinzu
- `add_hours(timestamp: number, hours: number) -> number` - Fügt Stunden hinzu
- `add_minutes(timestamp: number, minutes: number) -> number` - Fügt Minuten hinzu
- `format_relative(timestamp: number) -> string` - Formatierung als relativer Zeit (z.B. "vor 2 Stunden")
- `is_weekend(timestamp: number) -> boolean` - Prüft ob Wochenende
- `is_weekday(timestamp: number) -> boolean` - Prüft ob Wochentag

**Beispiel:**
```velin
let now = datetime.now();
let tomorrow = date.add_days(now, 1);
let relative = date.format_relative(now); // "vor 2 Stunden"
let weekend = date.is_weekend(now);
```

## FileSystem

Global object: `fs`

**Neu in Version 2.5** ✅

Dateisystem-Operationen für Datei-Management.

- `read_json(path: string) -> Result<any, string>` - Liest JSON-Datei
- `write_json(path: string, value: any) -> Result<(), string>` - Schreibt JSON-Datei
- `copy(source: string, dest: string) -> Result<(), string>` - Kopiert Datei/Verzeichnis
- `move_file(source: string, dest: string) -> Result<(), string>` - Verschiebt Datei
- `get_size(path: string) -> Result<number, string>` - Gibt Dateigröße zurück
- `is_empty(path: string) -> boolean` - Prüft ob Verzeichnis leer ist
- `exists(path: string) -> boolean` - Prüft ob Pfad existiert
- `mkdir(path: string) -> Result<(), string>` - Erstellt Verzeichnis
- `list_files(path: string) -> Result<List<string>, string>` - Listet Dateien auf

**Beispiel:**
```velin
let data = fs.read_json("config.json");
fs.write_json("output.json", { key: "value" });
fs.copy("source.txt", "dest.txt");
let size = fs.get_size("file.txt");
```

## LLM

Global object: `llm`

**Neu in Version 2.5** ✅

KI/LLM-Integration für Textverarbeitung und -generierung.

**Hinweis**: Diese Funktionen benötigen einen konfigurierten LLMClient. Siehe [ML Tutorial](tutorial-7-ml.md) für Details.

- `summarize(text: string) -> Result<string, string>` - Erstellt Zusammenfassung
- `classify(text: string, categories: List<string>) -> Result<string, string>` - Klassifiziert Text
- `extract_entities(text: string) -> Result<List<Map<string, string>>, string>` - Extrahiert Entitäten
- `generate(title: string, style?: string) -> Result<string, string>` - Generiert Text
- `translate(text: string, target_lang: string) -> Result<string, string>` - Übersetzt Text
- `sentiment(text: string) -> Result<string, string>` - Analysiert Sentiment
- `complete(prompt: string, max_tokens?: number) -> Result<string, string>` - Vervollständigt Prompt
- `embed(text: string) -> Result<List<number>, string>` - Erstellt Embedding
- `chat(messages: List<any>) -> Result<string, string>` - Chat-Kompletion

**Beispiel:**
```velin
let client = LLMClient.new("openai");
let summary = await llm.summarize("Long text here...");
let sentiment = await llm.sentiment("I love this product!");
```

## Embedding

Global object: `embedding`

- `compare(a: List<number>, b: List<number>) -> number`
- `similarity(a: List<number>, b: List<number>) -> number`
- `cluster(list: List<List<number>>, k: number) -> Result<List<List<List<number>>>, string>`
- `normalize(embedding: List<number>) -> List<number>`
- `distance(a: List<number>, b: List<number>) -> number`
- `find_nearest(query: List<number>, candidates: List<List<number>>, k: number) -> List<List<number>>`
- `average(embeddings: List<List<number>>) -> List<number>`
- `dimension(embedding: List<number>) -> number`

## Agent

Global object: `agent`

- `create(name: string) -> Agent`
- `think(context: string) -> Result<string, string>`
- `memory.store(key: string, value: any) -> Result<(), string>`
- `memory.search(query: string) -> Result<List<any>, string>`
- `memory.get(key: string) -> Result<any, string>`
- `memory.delete(key: string) -> Result<(), string>`
- `task.run(description: string) -> Result<any, string>`
- `task.plan(goal: string) -> Result<List<string>, string>`
- `task.execute(plan: List<string>) -> Result<any, string>`

## Process

Global object: `process`

- `spawn(command: string, args: List<string>) -> Result<number, string>`
- `kill(pid: number) -> Result<(), string>`
- `restart(pid: number) -> Result<(), string>`
- `status(pid: number) -> Result<any, string>`
- `list() -> List<any>`
- `wait(pid: number) -> Result<number, string>`
- `get_output(pid: number) -> Result<string, string>`
- `is_running(pid: number) -> boolean`
- `get_memory(pid: number) -> Result<number, string>`

## Sandbox

Global object: `sandbox`

- `build(project_path: string) -> Result<(), string>`
- `test(project_path: string) -> Result<(), string>`
- `validate(code: string) -> Result<(), string>`
- `run(code: string) -> Result<any, string>`
- `lint(code: string) -> Result<List<string>, string>`
- `format(code: string) -> Result<string, string>`
- `check_types(code: string) -> Result<(), string>`
- `optimize(code: string) -> Result<string, string>`

## Rollback

Global object: `rollback`

- `list_snapshots() -> List<any>`
- `delete_snapshot(snapshot_id: string) -> Result<(), string>`
- `compare(snapshot1: string, snapshot2: string) -> Result<any, string>`
- `get_info(snapshot_id: string) -> Result<any, string>`
- `auto_snapshot(interval_seconds: number) -> Result<(), string>`

## HTTP

Global object: `http`

- `patch(url: string, body: any) -> Result<HttpResponse, string>`
- `head(url: string) -> Result<HttpResponse, string>`
- `options(url: string) -> Result<HttpResponse, string>`
- `set_timeout(client: HttpClient, ms: number) -> HttpClient`
- `set_headers(client: HttpClient, headers: any) -> HttpClient`

## WebSocket

Global object: `websocket`

- `connect(url: string) -> Result<WebSocket, string>`
- `send(ws: WebSocket, message: string) -> Result<(), string>`
- `receive(ws: WebSocket) -> Result<string, string>`
- `close(ws: WebSocket) -> Result<(), string>`
- `is_connected(ws: WebSocket) -> boolean`
- `ping(ws: WebSocket) -> Result<(), string>`
- `subscribe(ws: WebSocket, topic: string) -> Result<(), string>`
- `on_message(ws: WebSocket, callback: fn) -> Result<(), string>`

## Utils

Global object: `utils`

- `uuid() -> string`
- `sleep(ms: number) -> void`
- `retry(fn: fn, times: number) -> Result<any, string>`
- `debounce(fn: fn, ms: number) -> fn`
- `throttle(fn: fn, ms: number) -> fn`
- `memoize(fn: fn) -> fn`
- `timeout(fn: fn, ms: number) -> Result<any, string>`
- `parallel(tasks: List<fn>) -> List<Result<any, string>>`
- `cache(key: string, fn: fn) -> any`

## Logging

Global object: `log`

- `info(message: string) -> void`
- `warn(message: string) -> void`
- `error(message: string) -> void`
- `debug(message: string) -> void`
- `trace(message: string) -> void`
- `set_level(level: string) -> void`
- `with_context(key: string, value: string) -> Logger`
- `to_file(path: string) -> Result<(), string>`
- `json(message: string, data: any) -> void`

## Config

Global object: `config`

- `get_env(key: string) -> Result<string, string>`
- `get_or_default(key: string, default: string) -> string`
- `load_dotenv() -> Result<(), string>`

## Flow

Global object: `flow`

**Decorator**: `@Flow`

**Neu in Version 2.5** ✅

VelinFlow Runtime für transaktionales Flow-Management mit automatischem State-Tracking, Snapshots und Rollback.

**Features:**
- Automatisches State-Tracking (Pending, Running, Completed, Failed, Compensating, Compensated)
- Input-Snapshot-Management für Rollback
- Automatisches Commit bei Erfolg
- Automatisches Rollback mit Compensation-Logic bei Fehler
- Logging der Ausführungsdauer und Status
- Self-Healing durch Compensation-Hooks

**Verfügbare Funktionen:**
- `flow.snapshot_input(input: any) -> void`: Manuelles Aufzeichnen eines Input-Snapshots

**Beispiel:**
```velin
@Flow
@POST("/orders")
fn createOrder(input: OrderInput): OrderResult {
    flow.snapshot_input(input);
    // Automatisches State-Tracking
    // Automatisches Rollback bei Fehler
    return processOrder(input);
}
```

## AutoDoc

**Decorator**: `@VelinAutoDoc`

**Neu in Version 2.5** ✅

Automatische Dokumentationsgenerierung aus `///` Doc-Comments.

**Features:**
- Erfasst `///` Doc-Comments als First-Class-Citizens im AST
- Generiert strukturierte JSON-Dokumentation
- Extrahiert Typ-Signaturen, Parameter und Return-Types
- Erstellt `llm_prompt_context` für KI-gestützte Dokumentationsgenerierung
- Unterstützt Funktionen, Structs und Module

**Usage**: Place above any function or struct.

**Beispiel:**
```velin
/// Erstellt einen neuen Benutzer
/// 
/// @param name - Der Name des Benutzers
/// @returns Ein User-Objekt mit generierter ID
@VelinAutoDoc
fn createUser(name: string): User {
    // ...
}
```

**Output**: Generates a JSON structure containing:
  - Signatures, parameters, return types.
  - Extracted doc comments (`///`).
  - `llm_prompt_context` field optimized for AI explanation generation.

## Pipeline

**Decorator**: `@VelinPipeline`

**Neu in Version 2.5** ✅

Pipeline-Optimizer für automatische Parallelisierung von unabhängigen async Operationen.

**Features:**
- Analysiert Datenabhängigkeiten zwischen Statements
- Erkennt automatisch unabhängige async Operationen
- Optimiert sequentielle Aufrufe zu parallelen Ausführungsgruppen
- Generiert automatisch `tokio::join!` für unabhängige Operationen
- Verbessert Performance durch Parallelisierung

**Usage**: Place above a `mod` or `fn`.

**Beispiel:**
```velin
@VelinPipeline
async fn loadDashboard() {
    // Werden automatisch parallel ausgeführt
    let user = await getUser();
    let stats = await getStats();
    return { user, stats };
}
```

**Effect**:
  - Analyzes data dependencies between statements.
  - Automatically identifies independent async operations.
  - Optimizes sequential calls into parallel execution groups (`tokio::join!`).
