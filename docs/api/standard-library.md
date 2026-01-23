# Standard Library API Reference

VelinScript provides a rich standard library for common tasks. Version 3.1.0 includes 50+ modules with over 200+ functions.

**Neu in Version 2.5**: 13 neue Module mit 117+ Funktionen hinzugefügt ✅  
**Neu in Version 2.6**: 5 neue Module mit 50+ Funktionen hinzugefügt ✅  
**Neu in Version 2.7**: 17 neue Module mit 120+ Funktionen hinzugefügt ✅  
**Neu in Version 3.0.1**: IR, Borrow Checker, Prompt Optimizer ✅  
**Neu in Version 3.1.0**: Multi-Target Compilation, erweiterte Parallelisierung ✅

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
- [Path](#path)
- [URL](#url)
- [Stream](#stream)
- [Redis](#redis)
- [Tracing](#tracing)
- [Encoding](#encoding)
- [Queue](#queue)
- [MongoDB](#mongodb)
- [SMTP](#smtp)
- [CSV](#csv)
- [YAML](#yaml)
- [Audit](#audit)
- [Encryption](#encryption)
- [Alerting](#alerting)
- [NLP](#nlp)
- [Workflow](#workflow)
- [Scheduler](#scheduler)
- [Event Bus](#event-bus)
- [Fixtures](#fixtures)
- [Mocks](#mocks)
- [Template](#template)
- [Env](#env)
- [Metrics](#metrics)
- [Cache](#cache)

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

## Path

Global object: `path`

**Neu in Version 2.6** ✅

Pfad-Manipulation für Cross-Platform Dateisystem-Operationen.

- `join(parts: List<string>) -> string` - Verbindet Pfad-Komponenten
- `dirname(path: string) -> string` - Gibt Verzeichnisname zurück
- `basename(path: string) -> string` - Gibt Dateiname zurück
- `extname(path: string) -> string` - Gibt Dateiendung zurück
- `normalize(path: string) -> string` - Normalisiert Pfad (entfernt `..` und `.`)
- `resolve(path: string) -> Result<string, string>` - Konvertiert zu absolutem Pfad
- `relative(from: string, to: string) -> string` - Gibt relativen Pfad zurück
- `is_absolute(path: string) -> boolean` - Prüft ob Pfad absolut ist
- `separator() -> string` - Gibt Pfad-Trennzeichen zurück (`/` oder `\`)

**Beispiel:**
```velin
let full_path = path.join(["dir", "subdir", "file.txt"]);
let dir = path.dirname("/home/user/file.txt"); // "/home/user"
let filename = path.basename("/home/user/file.txt"); // "file.txt"
let ext = path.extname("file.txt"); // ".txt"
let normalized = path.normalize("dir/../other/./file.txt"); // "other/file.txt"
let is_abs = path.is_absolute("/home/user"); // true
```

## URL

Global object: `url`

**Neu in Version 2.6** ✅

URL-Manipulation und -Parsing für API-Integration.

- `parse(url: string) -> Result<Url, string>` - Parst URL-String
- `protocol(url: Url) -> string` - Gibt Protokoll zurück (z.B. "https:")
- `hostname(url: Url) -> string` - Gibt Hostname zurück
- `port(url: Url) -> number` - Gibt Port zurück
- `pathname(url: Url) -> string` - Gibt Pfad zurück
- `search(url: Url) -> string` - Gibt Query-String zurück
- `hash(url: Url) -> string` - Gibt Fragment zurück
- `format(components: any) -> string` - Erstellt URL aus Komponenten
- `parse_query(query: string) -> any` - Parst Query-String zu Map
- `stringify_query(params: any) -> string` - Konvertiert Map zu Query-String

**Beispiel:**
```velin
let parsed = url.parse("https://example.com:8080/path?query=value#fragment");
let protocol = url.protocol(parsed); // "https:"
let hostname = url.hostname(parsed); // "example.com"
let port = url.port(parsed); // 8080
let params = url.parse_query("?name=John&age=30");
let query = url.stringify_query({ name: "John", age: 30 }); // "name=John&age=30"
```

## Stream

Global object: `stream`

**Neu in Version 2.6** ✅

Stream-Verarbeitung für große Datenmengen und Real-time Datenverarbeitung.

- `create() -> Stream` - Erstellt neuen Stream
- `map(stream: Stream, mapper: fn) -> Stream` - Transformiert Stream-Elemente
- `filter(stream: Stream, predicate: fn) -> Stream` - Filtert Stream-Elemente
- `reduce(stream: Stream, reducer: fn, initial: any) -> any` - Reduziert Stream
- `batch(stream: Stream, size: number) -> Stream` - Gruppiert Elemente in Batches
- `buffer(stream: Stream, size: number) -> Stream` - Puffert Stream-Elemente
- `merge(stream1: Stream, stream2: Stream) -> Stream` - Verbindet zwei Streams
- `zip(stream1: Stream, stream2: Stream) -> Stream` - Kombiniert zwei Streams

**Beispiel:**
```velin
let stream = stream.create();
let mapped = stream.map(stream, (item) => item * 2);
let filtered = stream.filter(stream, (item) => item > 0);
let result = stream.reduce(stream, (acc, item) => acc + item, 0);
let batched = stream.batch(stream, 100);
```

## Redis

Global object: `redis`

**Neu in Version 2.6** ✅

Redis-Integration für Caching, Session-Management und Pub/Sub.

- `connect(url: string) -> Result<RedisClient, string>` - Verbindet zu Redis
- `set(client: RedisClient, key: string, value: string, ttl?: number) -> Result<void, string>` - Setzt Wert
- `get(client: RedisClient, key: string) -> Result<Option<string>, string>` - Holt Wert
- `delete(client: RedisClient, key: string) -> Result<boolean, string>` - Löscht Wert
- `hset(client: RedisClient, hash: string, field: string, value: string) -> Result<void, string>` - Setzt Hash-Feld
- `hget(client: RedisClient, hash: string, field: string) -> Result<Option<string>, string>` - Holt Hash-Feld
- `hgetall(client: RedisClient, hash: string) -> Result<Map<string, string>, string>` - Holt alle Hash-Felder
- `lpush(client: RedisClient, list: string, value: string) -> Result<void, string>` - Fügt links zur Liste hinzu
- `rpush(client: RedisClient, list: string, value: string) -> Result<void, string>` - Fügt rechts zur Liste hinzu
- `lpop(client: RedisClient, list: string) -> Result<Option<string>, string>` - Entfernt links von Liste
- `llen(client: RedisClient, list: string) -> Result<number, string>` - Gibt Listenlänge zurück
- `sadd(client: RedisClient, set: string, member: string) -> Result<void, string>` - Fügt zu Set hinzu
- `sismember(client: RedisClient, set: string, member: string) -> Result<boolean, string>` - Prüft Set-Mitgliedschaft
- `smembers(client: RedisClient, set: string) -> Result<List<string>, string>` - Gibt alle Set-Mitglieder zurück
- `publish(client: RedisClient, channel: string, message: string) -> Result<void, string>` - Veröffentlicht Nachricht

**Beispiel:**
```velin
let client = redis.connect("redis://localhost:6379");
redis.set(client, "key", "value");
let value = redis.get(client, "key");
redis.hset(client, "user:123", "name", "John");
let name = redis.hget(client, "user:123", "name");
redis.publish(client, "channel", "message");
```

## Tracing

Global object: `tracing`

**Neu in Version 2.6** ✅

Distributed Tracing für Microservices und Performance-Analyse.

- `start_span(name: string) -> Span` - Startet neuen Span
- `set_attribute(span: Span, key: string, value: string) -> void` - Setzt Span-Attribut
- `child_span(parent: Span, name: string) -> Span` - Erstellt Child-Span
- `end_span(span: Span) -> void` - Beendet Span
- `export(format: string) -> Result<void, string>` - Exportiert Tracing-Daten

**Beispiel:**
```velin
let span = tracing.start_span("api.request");
tracing.set_attribute(span, "http.method", "GET");
tracing.set_attribute(span, "http.url", "/api/users");
let child = tracing.child_span(span, "database.query");
// ... Operation ...
tracing.end_span(child);
tracing.end_span(span);
```

## Encoding

Global object: `encoding`

**Neu in Version 2.7** ✅

Zeichenkodierung und -dekodierung für verschiedene Formate.

- `base64_encode(input: string) -> string` - Kodiert String zu Base64
- `base64_decode(input: string) -> Result<string, string>` - Dekodiert Base64 zu String
- `url_encode(input: string) -> string` - URL-kodiert String
- `url_decode(input: string) -> string` - URL-dekodiert String
- `hex_encode(input: string) -> string` - Kodiert String zu Hex
- `hex_decode(input: string) -> Result<string, string>` - Dekodiert Hex zu String
- `is_valid_utf8(bytes: List<number>) -> boolean` - Prüft UTF-8-Validität
- `fix_utf8(bytes: List<number>) -> List<number>` - Repariert ungültige UTF-8-Bytes

**Beispiel:**
```velin
let encoded = encoding.base64_encode("Hello World");
let decoded = encoding.base64_decode(encoded);
let url_encoded = encoding.url_encode("Hello World!");
```

## Queue

Global object: `queue`

**Neu in Version 2.7** ✅

Warteschlangen für Task- und Event-Verarbeitung.

- `create(capacity?: number) -> Queue` - Erstellt neue Queue
- `enqueue(queue: Queue, item: any) -> void` - Fügt Element hinzu
- `dequeue(queue: Queue) -> Option<any>` - Entfernt und gibt Element zurück
- `peek(queue: Queue) -> Option<any>` - Gibt Element zurück ohne Entfernung
- `size(queue: Queue) -> number` - Anzahl Elemente
- `is_empty(queue: Queue) -> boolean` - Prüft ob leer
- `is_full(queue: Queue) -> boolean` - Prüft ob voll
- `priority(compare: fn(any, any) -> number) -> PriorityQueue` - Erstellt Priority Queue
- `bounded(capacity: number) -> Queue` - Erstellt Queue mit fester Kapazität

**Beispiel:**
```velin
let queue = queue.create(100);
queue.enqueue(queue, "task1");
let item = queue.dequeue(queue);
```

## MongoDB

Global object: `mongodb`

**Neu in Version 2.7** ✅

MongoDB-Integration für NoSQL-Datenbanken.

- `connect(url: string) -> Result<MongoClient, string>` - Verbindet zu MongoDB
- `database(client: MongoClient, name: string) -> Database` - Gibt Datenbank zurück
- `collection(db: Database, name: string) -> Collection` - Gibt Collection zurück
- `insert_one(collection: Collection, doc: any) -> Result<string, string>` - Fügt Dokument hinzu
- `find(collection: Collection, filter: any) -> Result<List<any>, string>` - Findet Dokumente
- `find_one(collection: Collection, filter: any) -> Result<Option<any>, string>` - Findet ein Dokument
- `update_one(collection: Collection, filter: any, update: any) -> Result<boolean, string>` - Aktualisiert Dokument
- `delete_one(collection: Collection, filter: any) -> Result<boolean, string>` - Löscht Dokument
- `aggregate(collection: Collection, pipeline: List<any>) -> Result<List<any>, string>` - Aggregation
- `create_index(collection: Collection, keys: any, unique: boolean) -> Result<void, string>` - Erstellt Index

**Beispiel:**
```velin
let client = mongodb.connect("mongodb://localhost:27017");
let db = mongodb.database(client, "mydb");
let collection = mongodb.collection(db, "users");
let result = mongodb.insert_one(collection, { name: "John", age: 30 });
```

## SMTP

Global object: `smtp`

**Neu in Version 2.7** ✅

E-Mail-Versand über SMTP.

- `connect(config: any) -> Result<SmtpClient, string>` - Verbindet zu SMTP-Server
- `send(mailer: SmtpClient, email: any) -> Result<void, string>` - Sendet E-Mail
- `template(template_path: string, data: any) -> Result<string, string>` - Rendert E-Mail-Template

**Beispiel:**
```velin
let client = smtp.connect({ host: "smtp.example.com", port: 587, username: "user", password: "pass" });
smtp.send(client, { from: "sender@example.com", to: ["recipient@example.com"], subject: "Hello", body: "Body" });
```

## CSV

Global object: `csv`

**Neu in Version 2.7** ✅

CSV-Verarbeitung für Datenimport/Export.

- `read(path: string, has_header: boolean) -> Result<List<Map<string, string>>, string>` - Liest CSV-Datei
- `write(path: string, rows: List<Map<string, string>>, headers?: List<string>) -> Result<void, string>` - Schreibt CSV-Datei
- `parse(csv_string: string) -> List<List<string>>` - Parst CSV-String
- `stringify(rows: List<List<string>>, headers: List<string>) -> string` - Konvertiert zu CSV-String
- `validate(path: string, schema: any) -> Result<boolean, string>` - Validiert CSV gegen Schema

**Beispiel:**
```velin
let rows = csv.read("data.csv", true);
csv.write("output.csv", rows, ["name", "email", "age"]);
```

## YAML

Global object: `yaml`

**Neu in Version 2.7** ✅

YAML-Verarbeitung für Konfigurationsdateien.

- `parse(yaml_string: string) -> Result<any, string>` - Parst YAML-String
- `parse_file(path: string) -> Result<any, string>` - Parst YAML-Datei
- `stringify(value: any) -> Result<string, string>` - Konvertiert zu YAML-String
- `write_file(path: string, value: any) -> Result<void, string>` - Schreibt YAML-Datei
- `validate(path: string, schema: any) -> Result<boolean, string>` - Validiert YAML gegen Schema

**Beispiel:**
```velin
let data = yaml.parse_file("config.yaml");
yaml.write_file("output.yaml", { name: "John", age: 30 });
```

## Audit

Global object: `audit`

**Neu in Version 2.7** ✅

Audit-Logging für Compliance und Sicherheit.

- `log(log_data: any) -> Result<void, string>` - Erstellt Audit-Log-Eintrag
- `query(filters: any) -> Result<List<any>, string>` - Abfragt Audit-Logs
- `export(format: string, filters: any) -> Result<string, string>` - Exportiert Audit-Logs

**Beispiel:**
```velin
audit.log({ action: "user.login", user_id: "123", ip_address: "192.168.1.1" });
let logs = audit.query({ user_id: "123", action: "user.login" });
```

## Encryption

Global object: `encryption`

**Neu in Version 2.7** ✅

Erweiterte Verschlüsselung für sichere Datenverarbeitung.

- `aes_encrypt(data: string, key: string) -> Result<string, string>` - AES-Verschlüsselung
- `aes_decrypt(encrypted: string, key: string) -> Result<string, string>` - AES-Entschlüsselung
- `rsa_generate_keypair(bits: number) -> Result<any, string>` - Generiert RSA-Schlüsselpaar
- `rsa_encrypt(data: string, public_key: string) -> Result<string, string>` - RSA-Verschlüsselung
- `rsa_decrypt(encrypted: string, private_key: string) -> Result<string, string>` - RSA-Entschlüsselung
- `fernet_generate_key() -> string` - Generiert Fernet-Schlüssel
- `fernet_encrypt(data: string, key: string) -> Result<string, string>` - Fernet-Verschlüsselung
- `fernet_decrypt(encrypted: string, key: string) -> Result<string, string>` - Fernet-Entschlüsselung
- `generate_key(algorithm: string) -> Result<string, string>` - Generiert Schlüssel für Algorithmus
- `store_key(key_id: string, key: string, vault: string) -> Result<void, string>` - Speichert Schlüssel
- `retrieve_key(key_id: string) -> Result<string, string>` - Ruft Schlüssel ab

**Beispiel:**
```velin
let encrypted = encryption.aes_encrypt("secret data", "my-key");
let decrypted = encryption.aes_decrypt(encrypted, "my-key");
let keypair = encryption.rsa_generate_keypair(2048);
```

## Alerting

Global object: `alerting`

**Neu in Version 2.7** ✅

Alerting-System für System-Überwachung.

- `create_rule(rule: any) -> any` - Erstellt Alert-Regel
- `check(metric: string, value: number, rules: List<any>) -> List<any>` - Prüft Metriken gegen Regeln
- `trigger(alert: any) -> Result<void, string>` - Löst Alert aus
- `history(filters: any) -> Result<List<any>, string>` - Abfragt Alert-Historie

**Beispiel:**
```velin
let rule = alerting.create_rule({ condition: ">", threshold: 0.05 });
let triggered = alerting.check("error_rate", 0.08, [rule]);
```

## NLP

Global object: `nlp`

**Neu in Version 2.7** ✅

Natural Language Processing für Textanalyse.

- `tokenize(text: string) -> List<string>` - Tokenisiert Text
- `sentiment(text: string) -> string` - Analysiert Sentiment (positive/negative/neutral)
- `ner(text: string) -> List<any>` - Named Entity Recognition
- `keywords(text: string, count: number) -> List<string>` - Extrahiert Keywords
- `similarity(text1: string, text2: string) -> number` - Berechnet Text-Ähnlichkeit
- `summarize(text: string, sentences: number) -> string` - Erstellt Zusammenfassung

**Beispiel:**
```velin
let tokens = nlp.tokenize("Hello world");
let sentiment = nlp.sentiment("I love this product!"); // "positive"
let keywords = nlp.keywords("This is a great article about technology", 5);
```

## Workflow

Global object: `workflow`

**Neu in Version 2.7** ✅

Workflow-Engine für Business-Process-Management.

- `create(definition: any) -> any` - Erstellt Workflow
- `start(workflow: any) -> any` - Startet Workflow
- `execute_step(workflow: any, step_id: string) -> any` - Führt Schritt aus
- `get_status(workflow: any) -> string` - Gibt Workflow-Status zurück
- `get_history(workflow: any) -> List<any>` - Gibt Workflow-Historie zurück
- `complete(workflow: any) -> any` - Markiert Workflow als abgeschlossen
- `fail(workflow: any, error: string) -> any` - Markiert Workflow als fehlgeschlagen

**Beispiel:**
```velin
let workflow = workflow.create({ name: "order_processing", steps: [...] });
workflow.start(workflow);
workflow.execute_step(workflow, "validate_order");
```

## Scheduler

Global object: `scheduler`

**Neu in Version 2.7** ✅

Task-Scheduling für periodische Jobs.

- `schedule(task: any, cron: string) -> any` - Plant Task mit Cron-Expression
- `schedule_interval(task: any, interval: string) -> any` - Plant Task mit Intervall
- `cancel(task_id: string) -> any` - Bricht Task ab
- `list() -> List<any>` - Listet alle Tasks
- `get(task_id: string) -> any` - Gibt Task zurück
- `enable(task_id: string) -> any` - Aktiviert Task
- `disable(task_id: string) -> any` - Deaktiviert Task

**Beispiel:**
```velin
scheduler.schedule({ name: "daily_backup" }, "0 0 * * *");
scheduler.schedule_interval({ name: "health_check" }, "1h");
```

## Event Bus

Global object: `event_bus`

**Neu in Version 2.7** ✅

Event-Bus für Event-Driven-Architektur.

- `create() -> EventBus` - Erstellt Event-Bus
- `publish(bus: EventBus, topic: string, event: any) -> Result<void, string>` - Veröffentlicht Event
- `subscribe(bus: EventBus, topic: string) -> Result<void, string>` - Abonniert Topic
- `unsubscribe(subscription: any) -> Result<void, string>` - Kündigt Abonnement
- `get_history(bus: EventBus, topic: string, limit: number) -> List<any>` - Gibt Event-Historie zurück

**Beispiel:**
```velin
let bus = event_bus.create();
event_bus.publish(bus, "user.created", { user_id: "123" });
event_bus.subscribe(bus, "user.created");
```

## Fixtures

Global object: `fixtures`

**Neu in Version 2.7** ✅

Test-Fixtures für Test-Daten-Generierung.

- `create(template: any) -> any` - Erstellt Fixture aus Template
- `create_many(template: any, count: number) -> List<any>` - Erstellt mehrere Fixtures
- `factory(name: string, builder: any) -> any` - Registriert Factory
- `build(factory: any, overrides: any) -> any` - Erstellt Fixture mit Factory

**Beispiel:**
```velin
let user = fixtures.create({ name: "{{random_string}}", email: "{{random_email}}" });
let users = fixtures.create_many({ name: "User" }, 10);
```

## Mocks

Global object: `mocks`

**Neu in Version 2.7** ✅

Erweiterte Mocks für Unit-Testing.

- `mock(original: any, mock: any) -> any` - Erstellt Mock
- `spy(target: any) -> any` - Erstellt Spy
- `verify(spy: any, expected_calls: List<any>) -> any` - Verifiziert Aufrufe
- `reset(spy: any) -> any` - Setzt Spy zurück
- `stub(return_value: any) -> any` - Erstellt Stub

**Beispiel:**
```velin
let mock_db = mocks.mock(db, { find: fn() -> User { ... } });
let spy = mocks.spy(http_client);
mocks.verify(spy, [{ method: "GET", url: "/api/users" }]);
```

## Template

Global object: `template`

**Neu in Version 2.7** ✅

Template-Engine für HTML- und Dokument-Generierung.

- `render(template: string, data: any) -> string` - Rendert Template
- `render_file(path: string, data: any) -> Result<string, string>` - Rendert Template-Datei
- `partial(partial_path: string, data: any) -> Result<string, string>` - Rendert Partial
- `cache(template: string, cache_key: string) -> string` - Cached Template

**Beispiel:**
```velin
let html = template.render("Hello {{name}}!", { name: "John" });
let rendered = template.render_file("welcome.html", { user: user });
```

## Env

Global object: `env`

**Neu in Version 2.7** ✅

Erweiterte Umgebungsvariablen mit Validierung.

- `load(path: string) -> Result<void, string>` - Lädt .env-Datei
- `get(key: string, default?: string) -> string` - Gibt Umgebungsvariable zurück
- `get_number(key: string, default: number) -> number` - Gibt Zahl zurück
- `get_bool(key: string, default: boolean) -> boolean` - Gibt Boolean zurück
- `set(key: string, value: string) -> void` - Setzt Umgebungsvariable
- `validate(schema: any) -> Result<void, string>` - Validiert Umgebungsvariablen
- `get_secret(key: string, vault: string) -> Result<string, string>` - Ruft Secret ab

**Beispiel:**
```velin
env.load(".env");
let db_host = env.get("DB_HOST", "localhost");
let db_port = env.get_number("DB_PORT", 5432);
env.validate({ "DB_HOST": { required: true, type: "string" } });
```

## Metrics

Global object: `metrics`

**Neu in Version 3.0** ✅

Performance Monitoring und Metriken-Sammlung.

- `increment(name: string, labels?: string) -> void` - Erhöht Counter
- `gauge(name: string, value: number, labels?: string) -> void` - Setzt Gauge
- `histogram(name: string, value: number, labels?: string) -> void` - Zeichnet Histogramm auf

**Beispiel:**
```velin
metrics.increment("api.requests");
metrics.gauge("api.active_connections", 42);
metrics.histogram("api.response_time", 150.5);
```

## Cache

Global object: `cache`

**Neu in Version 3.0** ✅

Caching-System für Performance-Optimierung.

- `get(key: string) -> Option<any>` - Holt Wert aus Cache
- `set(key: string, value: any) -> void` - Setzt Wert in Cache
- `remove(key: string) -> boolean` - Löscht Wert aus Cache
- `clear() -> void` - Leert gesamten Cache
- `exists(key: string) -> boolean` - Prüft ob Key existiert
- `size() -> number` - Gibt Anzahl Einträge zurück

**Beispiel:**
```velin
let cached = cache.get("user:123");
if (cached.isSome()) {
    return cached.unwrap();
}
let user = db.find(User, "123");
cache.set("user:123", user);
```
