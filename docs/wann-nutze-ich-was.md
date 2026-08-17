# Wann nutze ich was? - VelinScript Entscheidungshilfe

Diese Dokumentation hilft Ihnen dabei, das richtige Tool oder Feature für Ihre spezifische Aufgabe zu finden.

---

## Inhaltsverzeichnis

1. [Entscheidungsfluss-Diagramm](#entscheidungsfluss-diagramm)
2. [Entwicklung & Code-Qualität](#entwicklung--code-qualität)
3. [Debugging & Entwicklung](#debugging--entwicklung)
4. [Testing & Qualitätssicherung](#testing--qualitätssicherung)
5. [Performance & Optimierung](#performance--optimierung)
6. [Code-Generierung & Automatisierung](#code-generierung--automatisierung)
7. [Security & Sicherheit](#security--sicherheit)
8. [Package Management](#package-management)
9. [Intelligence Features](#intelligence-features)
10. [API-Entwicklung](#api-entwicklung)
11. [KI & Machine Learning](#ki--machine-learning)

---

## Entscheidungsfluss-Diagramm

```
┌─────────────────────────────────────────────────────────────┐
│                    VELINSCRIPT TOOLCHAIN                     │
│                    Entscheidungshilfe                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
        ┌─────────────────────────────────────────┐
        │  Was möchten Sie tun?                   │
        └─────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
   ┌─────────┐          ┌──────────┐          ┌──────────┐
   │ Code    │          │ Code     │          │ Code     │
   │ schreiben│         │ prüfen   │         │ generieren│
   └─────────┘          └──────────┘          └──────────┘
        │                     │                     │
        ▼                     ▼                     ▼
   ┌─────────┐          ┌──────────┐          ┌──────────┐
   │ LSP     │          │ Linter   │          │ generate │
   │ Hot Reload│        │ Formatter│          │ api/crud │
   │ REPL    │          │ AutoFix  │          │ client   │
   │ Debugger│          │ Deps Graph│         │ library  │
   │ Inspector│         │ Bundle   │          │ generator│
   └─────────┘          └──────────┘          └──────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
   ┌─────────┐          ┌──────────┐          ┌──────────┐
   │ Test    │          │ Security │          │ Package  │
   │ Runner  │          │ Scanner  │          │ Manager  │
   │ Profiler│          │          │          │          │
   │ Benchmark│         │          │          │          │
   └─────────┘          └──────────┘          └──────────┘
```

---

## Entwicklung & Code-Qualität

### Wann nutze ich **Automatic Code Ordering**? ✅ (Neu in 3.1.0)

**Nutzen Sie Automatic Code Ordering, wenn:**
- ✅ Sie Code in natürlicher Reihenfolge schreiben möchten (ohne über Abhängigkeiten nachzudenken)
- ✅ Sie große Projekte mit vielen Abhängigkeiten haben
- ✅ Sie Multi-File-Projekte mit komplexen Modul-Abhängigkeiten entwickeln
- ✅ Sie sicherstellen möchten, dass Code in der korrekten Reihenfolge generiert wird

**Automatisch aktiviert:**
- ✅ Code wird automatisch nach dem Desugaring-Pass sortiert
- ✅ Keine manuelle Konfiguration nötig
- ✅ Funktioniert für Single-File und Multi-File-Projekte

**Beispiel:**
```velin
// Sie können Code in beliebiger Reihenfolge schreiben:
fn processUser(user: User) {
    return user.name.toUpperCase();
}

struct User {
    name: string;
    email: string;
}

// Der Compiler sortiert automatisch zu:
struct User {
    name: string;
    email: string;
}

fn processUser(user: User) {
    return user.name.toUpperCase();
}
```

**Siehe auch:**
- [Code Ordering Dokumentation](../architecture/code-ordering.md)
- [Type Inference Tutorial](../guides/tutorial-type-inference.md)

---

### Wann nutze ich **Type::Any Member-Access**? ✅ (Neu in 3.1.0)

**Nutzen Sie Type::Any Member-Access, wenn:**
- ✅ Sie mit dynamischen Daten arbeiten (z.B. JSON, API-Responses)
- ✅ Sie flexible Member-Zugriffe benötigen
- ✅ Sie automatische Type-Inference basierend auf Member-Namen nutzen möchten

**Automatisch aktiviert:**
- ✅ Type-Inference basierend auf Member-Namen (z.B. `length` → `Number`, `startsWith` → `Boolean`)
- ✅ Unterstützung für String-, List- und Map-ähnliche Methoden
- ✅ Fallback zu `Type::Any` für unbekannte Member (kein Fehler)

**Beispiel:**
```velin
fn processData(data: any) {
    // Automatische Type-Inference
    if (data.startsWith("http://")) {
        // data.startsWith() → Boolean (automatisch inferiert)
        return data.toUpperCase(); // → String
    }
    
    if (data.length > 0) {
        // data.length → Number
        return data.trim(); // → String
    }
}
```

**Siehe auch:**
- [Type Inference Dokumentation](../architecture/type-inference.md)
- [Type Inference Tutorial](../guides/tutorial-type-inference.md)

---

### Wann nutze ich den **Linter** (`velin-lint`)?

**Nutzen Sie den Linter, wenn:**
- ✅ Sie Code-Qualität sicherstellen möchten
- ✅ Sie ungenutzte Variablen/Funktionen finden wollen
- ✅ Sie Code-Komplexität analysieren möchten
- ✅ Sie vor einem Commit prüfen wollen
- ✅ Sie CI/CD-Pipelines einrichten

**Beispiel:**
```bash
# Vor jedem Commit
velin-lint check

# Mit Auto-Fix
velin-lint check --fix

# Nur bestimmte Regeln
velin-lint check --rules unused-variable --rules long-function
```

**Wann NICHT:**
- ❌ Während des aktiven Schreibens (stört den Flow)
- ❌ Bei sehr kleinen, experimentellen Dateien

---

### Wann nutze ich den **Formatter** (`velin format`)?

**Nutzen Sie den Formatter, wenn:**
- ✅ Sie Code konsistent formatieren möchten
- ✅ Sie Code von anderen übernehmen
- ✅ Sie vor einem Commit formatieren wollen
- ✅ Sie Format-on-Save aktivieren möchten

**Beispiel:**
```bash
# Einzelne Datei formatieren
velin format -i main.velin

# Format on Save (VS Code Extension)
# Automatisch aktiviert
```

**Wann NICHT:**
- ❌ Bei bereits perfekt formatiertem Code (unnötig)

---

### Wann nutze ich **AutoFix** (`--autofix`)?

**Nutzen Sie AutoFix, wenn:**
- ✅ Sie häufige Syntax-Fehler automatisch beheben möchten
- ✅ Unausgeglichene Klammern vorhanden sind
- ✅ Fehlende Funktionssignaturen korrigiert werden sollen
- ✅ Generic-Typen repariert werden müssen

**Beispiel:**
```bash
# Mit Kompilierung
velin compile -i main.velin --autofix

# Mit Code-Prüfung
velin check -i main.velin --autofix
```

**Wann NICHT:**
- ❌ Bei komplexen logischen Fehlern (AutoFix kann diese nicht beheben)
- ❌ Wenn Sie die Fehler manuell verstehen möchten

---

### Wann nutze ich den **Dependency Graph** (`velin-deps`)?

**Nutzen Sie den Dependency Graph, wenn:**
- ✅ Sie Modul-Abhängigkeiten visualisieren möchten
- ✅ Sie zirkuläre Imports finden wollen
- ✅ Sie die Projekt-Struktur verstehen möchten
- ✅ Sie Refactoring planen und Abhängigkeiten analysieren müssen
- ✅ Sie Onboarding für neue Teammitglieder unterstützen wollen

**Beispiel:**
```bash
# Dependency-Graph generieren
velin-deps graph

# Nur zirkuläre Abhängigkeiten anzeigen
velin-deps graph --circular

# JSON-Output für CI/CD
velin-deps graph --format json --output deps.json
```

**Wann NICHT:**
- ❌ Bei sehr kleinen Projekten ohne Module
- ❌ Bei einmaligen Skripten ohne Imports

---

### Wann nutze ich den **Bundle Analyzer** (`velin-bundle`)?

**Nutzen Sie den Bundle Analyzer, wenn:**
- ✅ Sie Bundle-Größe optimieren möchten
- ✅ Sie Tree-Shaking-Potenzial identifizieren wollen
- ✅ Sie Code-Splitting-Strategien planen
- ✅ Sie ungenutzten Code finden möchten
- ✅ Sie vor einem Release die Bundle-Größe prüfen wollen

**Beispiel:**
```bash
# Bundle analysieren
velin-bundle analyze

# Mit Tree-Shaking-Analyse
velin-bundle analyze --tree-shaking

# Code-Splitting-Vorschläge
velin-bundle analyze --code-splitting
```

**Wann NICHT:**
- ❌ Bei sehr kleinen Projekten
- ❌ Bei experimentellem Code (zu früh)

---

## Debugging & Entwicklung

## Debugging & Entwicklung

### Wann nutze ich den **Debugger** (`velin-debugger`)?

**Nutzen Sie den Debugger, wenn:**
- ✅ Sie komplexe Bugs analysieren müssen
- ✅ Sie Variablen zur Laufzeit inspizieren wollen
- ✅ Sie den Programmablauf Schritt für Schritt verfolgen möchten
- ✅ Sie Breakpoints setzen und Code pausieren wollen
- ✅ Sie Call Stacks analysieren müssen

**Beispiel:**
```bash
# Debugger Server starten
velin-debugger start --port 4711

# In VS Code: F5 drücken
# Breakpoints setzen und debuggen
```

**Wann NICHT:**
- ❌ Bei einfachen Print-Statements (zu aufwendig)
- ❌ Bei sehr einfachen Fehlern (Linter reicht)

---

### Wann nutze ich **Hot Reload** (`velin-hot-reload`)?

**Nutzen Sie Hot Reload, wenn:**
- ✅ Sie während der Entwicklung schnell Feedback brauchen
- ✅ Sie einen Development Server betreiben
- ✅ Sie kontinuierlich Code ändern und testen
- ✅ Sie API-Endpunkte entwickeln und sofort testen wollen

**Beispiel:**
```bash
# Watch Mode (nur kompilieren)
velin-hot-reload --watch

# Dev Server Mode (kompilieren + ausführen)
velin-hot-reload --server --run-command "velin run main.velin"
```

**Wann NICHT:**
- ❌ Bei Production-Builds (nur für Development)
- ❌ Bei einmaligen Kompilierungen

---

### Wann nutze ich den **LSP Server**?

**Der LSP Server ist automatisch aktiv, wenn:**
- ✅ Sie VS Code mit der VelinScript Extension nutzen
- ✅ Sie Auto-Completion benötigen
- ✅ Sie "Go to Definition" nutzen wollen
- ✅ Sie Hover-Informationen sehen möchten
- ✅ Sie Error Highlighting im Editor brauchen

**Keine manuelle Konfiguration nötig!** Die VS Code Extension startet den LSP automatisch.

**Alternative IDEs:**
- Neovim (mit lspconfig)
- Emacs (mit lsp-mode)
- Vim (mit vim-lsp)

---

### Wann nutze ich den **REPL** (`velin-repl`)?

**Nutzen Sie den REPL, wenn:**
- ✅ Sie Code schnell testen möchten
- ✅ Sie Ausdrücke interaktiv evaluieren wollen
- ✅ Sie Prototyping betreiben
- ✅ Sie API-Funktionen live testen möchten
- ✅ Sie Debugging mit interaktiven Experimenten kombinieren wollen

**Beispiel:**
```bash
# REPL starten
velin-repl

# Datei in REPL laden
velin-repl --file main.velin

# Im REPL:
velin> 2 + 3
5
velin> sqrt(16)
4.0
```

**Wann NICHT:**
- ❌ Bei komplexen, mehrteiligen Programmen (nutzen Sie stattdessen Tests)
- ❌ Bei Production-Code (REPL ist für Experimente)

---

### Wann nutze ich den **Runtime Inspector** (`velin-inspect`)?

**Nutzen Sie den Runtime Inspector, wenn:**
- ✅ Sie Variablen zur Laufzeit inspizieren möchten
- ✅ Sie Memory-Usage überwachen wollen
- ✅ Sie State während der Ausführung analysieren müssen
- ✅ Sie Live-Debugging ohne Breakpoints durchführen möchten
- ✅ Sie Performance-Probleme während der Ausführung identifizieren wollen

**Beispiel:**
```bash
# Code inspizieren
velin-inspect inspect main.velin --variables

# Memory-Usage anzeigen
velin-inspect inspect main.velin --memory

# Watch-Mode (kontinuierliche Überwachung)
velin-inspect inspect main.velin --watch
```

**Wann NICHT:**
- ❌ Bei statischer Code-Analyse (nutzen Sie Linter)
- ❌ Bei sehr einfachen Programmen (unnötig)

---

## Testing & Qualitätssicherung

### Wann nutze ich den **Test Runner** (`velin-test`)?

**Nutzen Sie den Test Runner, wenn:**
- ✅ Sie Unit-Tests ausführen möchten
- ✅ Sie Integration-Tests durchführen wollen
- ✅ Sie Test-Coverage messen möchten
- ✅ Sie vor einem Commit alle Tests prüfen wollen
- ✅ Sie CI/CD-Pipelines mit Tests einrichten
- ✅ Sie Mocking für Tests benötigen

**Beispiel:**
```bash
# Alle Tests ausführen
velin-test run

# Nur Unit-Tests
velin-test run --unit

# Mit Coverage-Report
velin-test run --coverage

# Mit Mocking
velin-test run --mock
```

**Test-Syntax:**
```velin
@test
fn testAdd() {
    let result = add(2, 3);
    assert(result == 5);
}

@before
fn setup() {
    db.connect();
}

@after
fn teardown() {
    db.disconnect();
}
```

**Wann NICHT:**
- ❌ Bei sehr einfachen, selbsterklärenden Funktionen (optional)
- ❌ Bei experimentellem Code (zu früh)

**Best Practice:** Führen Sie Tests vor jedem Commit aus!

---

## Performance & Optimierung

### Wann nutze ich den **Profiler** (`velin-profile`)?

**Nutzen Sie den Profiler, wenn:**
- ✅ Sie Performance-Probleme identifizieren möchten
- ✅ Sie CPU-Auslastung analysieren wollen
- ✅ Sie Memory-Leaks finden müssen
- ✅ Sie Bottlenecks in Ihrem Code lokalisieren wollen
- ✅ Sie Flame Graphs für visuelle Analyse benötigen

**Beispiel:**
```bash
# CPU-Profiling
velin-profile cpu main.velin

# CPU-Profiling mit Flame Graph
velin-profile cpu main.velin --flamegraph

# Memory-Profiling
velin-profile memory main.velin --output memory-report.json
```

**Wann NICHT:**
- ❌ Bei sehr einfachen, schnellen Funktionen (unnötig)
- ❌ Bei experimentellem Code (zu früh)

**Best Practice:** Profilen Sie immer Release-Builds, nicht Debug-Builds!

---

### Wann nutze ich den **Benchmark Runner** (`velin-bench`)?

**Nutzen Sie den Benchmark Runner, wenn:**
- ✅ Sie Performance-Metriken messen möchten
- ✅ Sie Performance-Regressionen erkennen wollen
- ✅ Sie verschiedene Implementierungen vergleichen müssen
- ✅ Sie vor einem Release Performance prüfen wollen
- ✅ Sie statistisch signifikante Performance-Daten benötigen

**Beispiel:**
```bash
# Benchmarks ausführen
velin-bench run

# Mit mehr Iterationen für Genauigkeit
velin-bench run --iterations 1000

# Mit Vergleich zu vorherigen Runs
velin-bench run --compare

# Ergebnisse speichern
velin-bench run --output benchmark.json
```

**Benchmark-Syntax:**
```velin
@benchmark
fn benchmarkSort() {
    let data = generateLargeArray(10000);
    sort(data);
}
```

**Wann NICHT:**
- ❌ Bei sehr einfachen Operationen (Messfehler zu groß)
- ❌ Bei nicht-kritischen Code-Pfaden

**Best Practice:** Führen Sie Benchmarks regelmäßig durch, um Performance-Regressionen früh zu erkennen!

---

## Code-Generierung & Automatisierung

## Code-Generierung & Automatisierung

### Wann nutze ich **Code Generation** (`velin generate`)?

**Nutzen Sie Code Generation, wenn:**
- ✅ Sie neue API-Endpunkte erstellen (`generate api`)
- ✅ Sie CRUD-Operationen benötigen (`generate crud`)
- ✅ Sie Boilerplate-Code vermeiden wollen
- ✅ Sie schnell Prototypen erstellen möchten
- ✅ Sie Client-Code aus OpenAPI generieren wollen (`generate client`)

**Beispiel:**
```bash
# API-Endpunkt generieren
velin generate api --name Products --path /api/v1/products

# Vollständiges CRUD-Modul
velin generate crud --name User --fields "id:string,email:string"

# TypeScript Client aus OpenAPI
velin generate client --openapi api.json --language typescript
```

**Wann NICHT:**
- ❌ Bei sehr speziellen, einmaligen Implementierungen
- ❌ Wenn Sie den generierten Code nicht verstehen

---

### Wann nutze ich den **API Doc Generator** (`velin-api-doc`)?

**Nutzen Sie den API Doc Generator, wenn:**
- ✅ Sie OpenAPI-Spezifikationen generieren möchten
- ✅ Sie interaktive API-Dokumentation (Swagger UI) brauchen
- ✅ Sie API-Dokumentation für Frontend-Teams erstellen
- ✅ Sie Client-Code generieren wollen

**Beispiel:**
```bash
# OpenAPI 3.0 generieren
velin-api-doc generate -i main.velin -o openapi.json --format json --interactive
```

**Wann NICHT:**
- ❌ Bei internen, nicht-dokumentierten APIs
- ❌ Bei sehr einfachen, einmaligen Endpunkten

---

### Wann nutze ich den **Bibliotheks-Generator** (`velin-library-generator`)?

**Nutzen Sie den Bibliotheks-Generator, wenn:**
- ✅ Sie neue Standardbibliotheks-Module für VelinScript erstellen möchten
- ✅ Sie konsistente Module-Struktur für die Standardbibliothek benötigen
- ✅ Sie Zeit bei der Erstellung von Standardbibliotheks-Modulen sparen wollen
- ✅ Sie automatische Integration in Type Checker, Code Generator und Tests brauchen
- ✅ Sie automatische Dokumentations- und Test-Generierung benötigen
- ✅ Sie verschiedene Modul-Typen (Utility, Service, Data Structure) erstellen möchten

**Beispiel:**
```bash
# Interaktiver Modus
cd tools/library-generator
cargo run -- generate --interactive

# Mit YAML-Konfiguration
cargo run -- generate --config my-library.yaml

# Validierung
cargo run -- validate --config my-library.yaml
```

**Generierte Komponenten:**
- Modul-Datei (`compiler/src/stdlib/{name}.rs`)
- Automatische `mod.rs` Integration
- Type Checker Integration
- Code Generator Integration
- Unit Tests (`compiler/tests/{name}_test.rs`)
- Dokumentation (`docs/api/{name}.md`)

**Wann NICHT:**
- ❌ Bei Anwendungs-Code (nutzen Sie normale VelinScript-Dateien)
- ❌ Bei externen Bibliotheken (nutzen Sie den Package Manager)
- ❌ Bei sehr speziellen, einmaligen Modulen (manuelle Implementierung besser)
- ❌ Bei sehr einfachen Utility-Funktionen (Overhead zu groß)

**Best Practice:** Nutzen Sie den Bibliotheks-Generator für alle neuen Standardbibliotheks-Module, um Konsistenz und Vollständigkeit sicherzustellen!

---

## Security & Sicherheit

### Wann nutze ich den **Security Scanner** (`velin-security`)?

**Nutzen Sie den Security Scanner, wenn:**
- ✅ Sie vor einem Release Security-Checks durchführen
- ✅ Sie CI/CD-Pipelines mit Security-Scanning einrichten
- ✅ Sie Dependencies auf Vulnerabilities prüfen wollen
- ✅ Sie SQL Injection, XSS, etc. erkennen möchten
- ✅ Sie Hardcoded Secrets finden wollen

**Beispiel:**
```bash
# Code scannen
velin-security scan

# Dependencies auditieren
velin-security audit

# HTML-Report generieren
velin-security scan --format html
```

**Wann NICHT:**
- ❌ Während des aktiven Schreibens (stört den Flow)
- ❌ Bei experimentellem Code (zu früh)

**Best Practice:** Integrieren Sie Security Scanning in Ihre CI/CD-Pipeline!

---

## Package Management

### Wann nutze ich den **Package Manager** (`velin-pkg`)?

**Nutzen Sie den Package Manager, wenn:**
- ✅ Sie Dependencies zu Ihrem Projekt hinzufügen möchten
- ✅ Sie Dependencies aktualisieren wollen
- ✅ Sie Dependency-Konflikte lösen müssen
- ✅ Sie Security-Audits für Dependencies durchführen
- ✅ Sie ein neues Projekt initialisieren

**Beispiel:**
```bash
# Dependency hinzufügen
velin-pkg add github.com/user/repo --version ^1.0.0

# Dependencies installieren
velin-pkg install

# Updates prüfen
velin-pkg update

# Security Audit
velin-pkg audit
```

**Wann NICHT:**
- ❌ Bei sehr kleinen Projekten ohne externe Dependencies
- ❌ Bei einmaligen Skripten

---

## Intelligence Features

### Wann nutze ich **VelinAutoDoc**?

**Nutzen Sie VelinAutoDoc, wenn:**
- ✅ Sie automatische Dokumentation aus `///` Doc-Comments generieren möchten
- ✅ Sie Knowledge Bases für RAG/LLM-Systeme erstellen
- ✅ Sie strukturierte JSON-Exporte für API-Dokumentation brauchen
- ✅ Sie LLM-freundliche Kontextinformationen benötigen

**Beispiel:**
```velin
/// Erstellt einen neuen Benutzer
/// @param name Der Name des Benutzers
/// @param email Die E-Mail-Adresse
/// @returns Der erstellte Benutzer
@VelinAutoDoc
fn createUser(name: string, email: string): User {
    // ...
}
```

**Wann NICHT:**
- ❌ Bei sehr einfachen, selbsterklärenden Funktionen
- ❌ Bei privaten, internen Funktionen (optional)

---

### Wann nutze ich **VelinPipeline**?

**Nutzen Sie VelinPipeline, wenn:**
- ✅ Sie asynchrone Operationen optimieren möchten
- ✅ Sie unabhängige async Blöcke parallelisieren wollen
- ✅ Sie Pipeline-Performance verbessern möchten
- ✅ Sie automatische Parallelisierung mit `tokio::join!` brauchen

**Beispiel:**
```velin
@VelinPipeline
async fn fetchData(): Data {
    let user = await fetchUser();      // Unabhängig
    let posts = await fetchPosts();    // Unabhängig
    // Wird automatisch parallelisiert!
    return Data { user, posts };
}
```

**Wann NICHT:**
- ❌ Bei sequenziellen, abhängigen Operationen
- ❌ Bei sehr einfachen, schnellen Operationen

---

### Wann nutze ich **@Flow** (VelinFlow)?

**Nutzen Sie @Flow, wenn:**
- ✅ Sie transaktionale Flows benötigen
- ✅ Sie automatisches Rollback bei Fehlern brauchen
- ✅ Sie State-Tracking für komplexe Operationen benötigen
- ✅ Sie Compensation-Logic für Self-Healing implementieren möchten

**Beispiel:**
```velin
@Flow
fn processOrder(order: Order): Result<Order, Error> {
    let user = db.findUser(order.userId)?;
    let payment = processPayment(order)?;
    let shipment = createShipment(order)?;
    // Automatisches Rollback bei Fehler!
    return Ok(order);
}
```

**Wann NICHT:**
- ❌ Bei einfachen, nicht-transaktionalen Operationen
- ❌ Bei read-only Operationen

---

### Wann nutze ich **VelinInsight**?

**Nutzen Sie VelinInsight, wenn:**
- ✅ Sie Code-Qualität analysieren möchten
- ✅ Sie ungenutzte Structs finden wollen
- ✅ Sie komplexe Funktionen identifizieren möchten
- ✅ Sie redundante Datenbank-Queries finden wollen

**Beispiel:**
```bash
velin insight -i main.velin
```

**Wann NICHT:**
- ❌ Bei sehr kleinen Projekten
- ❌ Bei experimentellem Code

---

## API-Entwicklung

### Wann nutze ich welche Decorators?

#### `@Auth` - Authentifizierung
**Nutzen Sie `@Auth`, wenn:**
- ✅ Endpunkte gezielt geschützt werden sollen (nur markierte Routen)
- ✅ Anfragen ohne Authorization-Header abgewiesen werden sollen (401)
- ✅ Benutzer-Zugang im stabilen 3.5.0-Pfad über Header-Präsenz gesteuert wird

JWT-Tokenvalidierung liegt außerhalb des stabilen 3.5.0-Umfangs (siehe [security.md](guides/security.md)).

```velin
@GET("/api/users")
@Auth
fn getUsers(): List<User> {
    // Nur authentifizierte Benutzer
}
```

#### `@Role` - Rollenbasierte Zugriffskontrolle
**Nutzen Sie `@Role`, wenn:**
- ✅ Bestimmte Rollen erforderlich sind
- ✅ Admin- oder User-Bereiche geschützt werden sollen

```velin
@DELETE("/api/users/:id")
@Auth
@Role("admin")
fn deleteUser(id: string): void {
    // Nur Admins
}
```

#### `@RateLimit` - Rate Limiting
**Nutzen Sie `@RateLimit`, wenn:**
- ✅ API-Endpunkte vor Überlastung geschützt werden sollen
- ✅ DDoS-Schutz benötigt wird
- ✅ Fair Usage sichergestellt werden soll

```velin
@POST("/api/chat")
@RateLimit(requests: 100, window: "1m", strategy: "fixed-window")
fn chat(message: string): string {
    // Max. 100 Requests pro Minute
}
```

#### `@Validate` - Input Validation
**Nutzen Sie `@Validate`, wenn:**
- ✅ User-Input validiert werden muss
- ✅ Datenintegrität sichergestellt werden soll
- ✅ Sicherheitslücken vermieden werden sollen

```velin
@POST("/api/users")
fn createUser(@Validate(email: true) email: string): User {
    // Email wird automatisch validiert
}
```

---

## KI & Machine Learning

### Wann nutze ich **LLMClient**?

**Nutzen Sie LLMClient, wenn:**
- ✅ Sie Chat-Funktionalitäten integrieren möchten
- ✅ Sie Text-Generierung benötigen
- ✅ Sie Embeddings generieren wollen
- ✅ Sie mit OpenAI, Anthropic oder Gemini arbeiten

**Beispiel:**
```velin
let client = LLMClient.new("openai");
let response = await client.complete({
    model: "gpt-4",
    messages: [{ role: "user", content: message }]
});
```

---

### Wann nutze ich **VectorDB**?

**Nutzen Sie VectorDB, wenn:**
- ✅ Sie semantische Suche implementieren möchten
- ✅ Sie Embedding-basierte Empfehlungen brauchen
- ✅ Sie mit Pinecone, Weaviate oder Qdrant arbeiten
- ✅ Sie Ähnlichkeitssuche benötigen

**Beispiel:**
```velin
let results = await vectorDB.search(embeddings, limit: 10);
```

---

### Wann nutze ich **ModelLoader** und **TrainingService**?

**Nutzen Sie diese, wenn:**
- ✅ Sie ML-Modelle laden und verwenden möchten
- ✅ Sie Model Training durchführen wollen
- ✅ Sie ONNX oder TensorFlow Models nutzen
- ✅ Sie Hyperparameter Tuning benötigen

---

## Quick Reference: Tool-Übersicht

| Tool | Wann nutzen? | Häufigkeit |
|------|--------------|------------|
| **Linter** | Code-Qualität prüfen | Vor jedem Commit |
| **Formatter** | Code formatieren | Vor jedem Commit |
| **AutoFix** | Syntax-Fehler beheben | Bei Kompilierungsfehlern |
| **Dependency Graph** | Modul-Abhängigkeiten visualisieren | Bei Refactoring, Onboarding |
| **Bundle Analyzer** | Bundle-Größe optimieren | Vor Releases, bei Performance-Problemen |
| **Debugger** | Bugs analysieren | Bei komplexen Fehlern |
| **Hot Reload** | Development Server | Während der Entwicklung |
| **REPL** | Code interaktiv testen | Während der Entwicklung, Prototyping |
| **Runtime Inspector** | Variablen/Memory inspizieren | Bei Runtime-Problemen |
| **LSP** | IDE-Unterstützung | Immer (automatisch) |
| **Test Runner** | Tests ausführen | Vor jedem Commit, in CI/CD |
| **Profiler** | Performance-Probleme finden | Bei Performance-Issues |
| **Benchmark Runner** | Performance messen | Regelmäßig, vor Releases |
| **Code Generation** | Boilerplate generieren | Bei neuen Features |
| **API Doc Generator** | Dokumentation erstellen | Vor Releases |
| **Security Scanner** | Security prüfen | Vor Releases, in CI/CD |
| **Package Manager** | Dependencies verwalten | Bei neuen Dependencies |
| **VelinAutoDoc** | Auto-Dokumentation | Bei öffentlichen APIs |
| **VelinPipeline** | Performance optimieren | Bei async Operationen |
| **@Flow** | Transaktionen | Bei kritischen Operationen |
| **VelinInsight** | Code-Analyse | Regelmäßig |

---

## Workflow-Empfehlungen

### Tägliche Entwicklung

1. **VS Code Extension** aktivieren (LSP, Syntax Highlighting)
2. **Hot Reload** starten für schnelles Feedback
3. **REPL** für schnelle Code-Tests nutzen
4. **Linter** vor jedem Commit ausführen
5. **Formatter** auf Save aktivieren

### Vor einem Commit

1. `velin-test run` - Tests ausführen
2. `velin-lint check` - Code-Qualität prüfen
3. `velin format` - Code formatieren
4. `velin check` - Type Checking
5. `velin-security scan` - Security prüfen (optional)

### Vor einem Release

1. `velin-test run --coverage` - Tests mit Coverage
2. `velin-lint check` - Code-Qualität
3. `velin-bundle analyze` - Bundle-Größe prüfen
4. `velin-bench run` - Performance-Benchmarks
5. `velin-security scan` - Security-Vulnerabilities
6. `velin-security audit` - Dependency-Audit
7. `velin-api-doc generate` - API-Dokumentation
8. `velin insight` - Code-Analyse (falls verfügbar)

### Bei neuen Features

1. `velin generate api/crud` - Boilerplate generieren
2. Code schreiben mit LSP-Unterstützung
3. `velin-repl` - Schnelle Tests während der Entwicklung
4. `velin-hot-reload` - Entwicklung mit Hot Reload
5. `velin-test run` - Tests schreiben und ausführen
6. `velin-debugger` oder `velin-inspect` - Bei komplexen Bugs
7. `velin-bench run` - Performance prüfen (bei kritischen Pfaden)

---

## Häufige Szenarien

### "Ich möchte eine neue API entwickeln"

1. `velin generate api --name MyAPI --path /api/v1/myapi`
2. Code mit LSP-Unterstützung schreiben
3. `velin-hot-reload --server` starten
4. `velin-api-doc generate` für Dokumentation

### "Ich habe einen Bug, den ich nicht verstehe"

1. `velin-repl` - Code interaktiv testen
2. `velin-inspect inspect --watch` - Runtime-Variablen überwachen
3. `velin-debugger start` oder F5 in VS Code
4. Breakpoints setzen
5. Schritt für Schritt debuggen
6. Variablen inspizieren

### "Ich möchte Code-Qualität sicherstellen"

1. `velin-lint check --fix` - Automatische Fixes
2. `velin format` - Code formatieren
3. `velin insight` - Code-Analyse (falls verfügbar)
4. Manuelle Verbesserungen

### "Ich möchte Security prüfen"

1. `velin-security scan` - Code scannen
2. `velin-security audit` - Dependencies prüfen
3. Gefundene Vulnerabilities beheben
4. In CI/CD integrieren

### "Ich möchte Performance optimieren"

1. `velin-bench run` - Baseline-Performance messen
2. `velin-profile cpu` - CPU-Bottlenecks finden
3. `velin-profile memory` - Memory-Leaks identifizieren
4. `velin-profile cpu --flamegraph` - Flame Graph für visuelle Analyse
5. Optimierungen durchführen
6. `velin-bench run --compare` - Performance-Vergleich

### "Ich möchte Modul-Abhängigkeiten verstehen"

1. `velin-deps graph` - Dependency-Graph generieren
2. `velin-deps graph --circular` - Zirkuläre Abhängigkeiten finden
3. `velin-deps graph --format json` - Für CI/CD-Integration
4. Refactoring planen basierend auf Graph

### "Ich möchte Bundle-Größe optimieren"

1. `velin-bundle analyze` - Aktuelle Bundle-Größe prüfen
2. `velin-bundle analyze --tree-shaking` - Ungenutzten Code finden
3. `velin-bundle analyze --code-splitting` - Code-Splitting-Vorschläge
4. Optimierungen durchführen
5. Erneut analysieren für Vergleich

---

## Fazit

VelinScript bietet eine umfassende Toolchain für moderne API-Entwicklung. Die meisten Tools sind **automatisch aktiv** (LSP, VS Code Extension) oder werden **bei Bedarf** genutzt (Debugger, Security Scanner).

**Goldene Regel:** Nutzen Sie die Tools, die Ihren Workflow verbessern, aber lassen Sie sich nicht von zu vielen Tools ablenken. Starten Sie mit den Basics (LSP, Hot Reload, Linter, Test Runner) und erweitern Sie nach Bedarf.

**Tool-Prioritäten:**
- **Essentiell:** LSP, Hot Reload, Linter, Test Runner, Formatter
- **Wichtig:** Debugger, Security Scanner, Dependency Graph
- **Bei Bedarf:** Profiler, Benchmark Runner, Bundle Analyzer, REPL, Runtime Inspector

---

## 🆕 Neue Standard Library Module (Version 2.6)

### Path - Pfad-Manipulation
**Wann nutzen:** Cross-Platform Pfad-Operationen, Dateisystem-Zugriffe, URL-Pfad-Manipulation

**Beispiel:**
```velin
let full_path = path.join(["dir", "subdir", "file.txt"]);
let dir = path.dirname("/home/user/file.txt");
let filename = path.basename("/home/user/file.txt");
let ext = path.extname("file.txt");
```

### URL - URL-Manipulation
**Wann nutzen:** API-URL-Konstruktion, Query-Parameter-Handling, URL-Validierung

**Beispiel:**
```velin
let parsed = url.parse("https://example.com:8080/path?query=value");
let params = url.parse_query("?name=John&age=30");
let query = url.stringify_query({ name: "John", age: 30 });
```

### Stream - Stream-Verarbeitung
**Wann nutzen:** Große Datenmengen verarbeiten, Real-time Datenverarbeitung, Event-Streaming

**Beispiel:**
```velin
let stream = stream.create();
let mapped = stream.map(stream, (item) => item * 2);
let filtered = stream.filter(stream, (item) => item > 0);
let result = stream.reduce(stream, (acc, item) => acc + item, 0);
```

### Redis - Redis-Integration
**Wann nutzen:** Caching, Session-Management, Pub/Sub, Rate Limiting, Task-Queues

**Beispiel:**
```velin
let client = redis.connect("redis://localhost:6379");
redis.set(client, "key", "value");
redis.hset(client, "user:123", "name", "John");
redis.publish(client, "channel", "message");
```

### Tracing - Distributed Tracing
**Wann nutzen:** Microservices-Tracing, Performance-Analyse, Debugging, Observability

**Beispiel:**
```velin
let span = tracing.start_span("api.request");
tracing.set_attribute(span, "http.method", "GET");
let child = tracing.child_span(span, "database.query");
tracing.end_span(child);
tracing.end_span(span);
```

## Weitere Ressourcen

- [Tools Dokumentation](tools/) - Detaillierte Dokumentation aller Tools
- [Getting Started Guide](guides/getting-started.md) - Erste Schritte
- [VS Code Extension](tools/vscode-extension.md) - IDE-Integration
- [API Dokumentation](api/) - API-Referenz
- [Standard Library](api/standard-library.md) - Vollständige API-Referenz aller Module