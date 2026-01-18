# Wann nutze ich was? - VelinScript Entscheidungshilfe

Diese Dokumentation hilft Ihnen dabei, das richtige Tool oder Feature für Ihre spezifische Aufgabe zu finden.

---

## Inhaltsverzeichnis

1. [Entscheidungsfluss-Diagramm](#entscheidungsfluss-diagramm)
2. [Entwicklung & Code-Qualität](#entwicklung--code-qualität)
3. [Debugging & Entwicklung](#debugging--entwicklung)
4. [Code-Generierung & Automatisierung](#code-generierung--automatisierung)
5. [Security & Sicherheit](#security--sicherheit)
6. [Package Management](#package-management)
7. [Intelligence Features](#intelligence-features)
8. [API-Entwicklung](#api-entwicklung)
9. [KI & Machine Learning](#ki--machine-learning)

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
   │ Debugger│          │ AutoFix  │          │ client   │
   └─────────┘          └──────────┘          └──────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
   ┌─────────┐          ┌──────────┐          ┌──────────┐
   │ Security│          │ Package  │          │ Docs     │
   │ Scanner │          │ Manager  │          │ Generator│
   └─────────┘          └──────────┘          └──────────┘
```

---

## Entwicklung & Code-Qualität

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
velin-compiler insight -i main.velin
```

**Wann NICHT:**
- ❌ Bei sehr kleinen Projekten
- ❌ Bei experimentellem Code

---

## API-Entwicklung

### Wann nutze ich welche Decorators?

#### `@Auth` - Authentifizierung
**Nutzen Sie `@Auth`, wenn:**
- ✅ Endpunkte geschützt werden sollen
- ✅ JWT-Token validiert werden müssen
- ✅ Benutzer-Authentifizierung erforderlich ist

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
| **Debugger** | Bugs analysieren | Bei komplexen Fehlern |
| **Hot Reload** | Development Server | Während der Entwicklung |
| **LSP** | IDE-Unterstützung | Immer (automatisch) |
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
3. **Linter** vor jedem Commit ausführen
4. **Formatter** auf Save aktivieren

### Vor einem Commit

1. `velin-lint check` - Code-Qualität prüfen
2. `velin format` - Code formatieren
3. `velin check` - Type Checking
4. `velin-security scan` - Security prüfen (optional)

### Vor einem Release

1. `velin-lint check` - Code-Qualität
2. `velin-security scan` - Security-Vulnerabilities
3. `velin-security audit` - Dependency-Audit
4. `velin-api-doc generate` - API-Dokumentation
5. `velin-compiler insight` - Code-Analyse

### Bei neuen Features

1. `velin generate api/crud` - Boilerplate generieren
2. Code schreiben mit LSP-Unterstützung
3. `velin-hot-reload` - Entwicklung mit Hot Reload
4. `velin-debugger` - Bei komplexen Bugs
5. Tests schreiben und ausführen

---

## Häufige Szenarien

### "Ich möchte eine neue API entwickeln"

1. `velin generate api --name MyAPI --path /api/v1/myapi`
2. Code mit LSP-Unterstützung schreiben
3. `velin-hot-reload --server` starten
4. `velin-api-doc generate` für Dokumentation

### "Ich habe einen Bug, den ich nicht verstehe"

1. `velin-debugger start` oder F5 in VS Code
2. Breakpoints setzen
3. Schritt für Schritt debuggen
4. Variablen inspizieren

### "Ich möchte Code-Qualität sicherstellen"

1. `velin-lint check --fix` - Automatische Fixes
2. `velin format` - Code formatieren
3. `velin-compiler insight` - Code-Analyse
4. Manuelle Verbesserungen

### "Ich möchte Security prüfen"

1. `velin-security scan` - Code scannen
2. `velin-security audit` - Dependencies prüfen
3. Gefundene Vulnerabilities beheben
4. In CI/CD integrieren

---

## Fazit

VelinScript bietet eine umfassende Toolchain für moderne API-Entwicklung. Die meisten Tools sind **automatisch aktiv** (LSP, VS Code Extension) oder werden **bei Bedarf** genutzt (Debugger, Security Scanner).

**Goldene Regel:** Nutzen Sie die Tools, die Ihren Workflow verbessern, aber lassen Sie sich nicht von zu vielen Tools ablenken. Starten Sie mit den Basics (LSP, Hot Reload, Linter) und erweitern Sie nach Bedarf.

---

## Weitere Ressourcen

- [Tools Dokumentation](tools/) - Detaillierte Dokumentation aller Tools
- [Getting Started Guide](guides/getting-started.md) - Erste Schritte
- [VS Code Extension](tools/vscode-extension.md) - IDE-Integration
- [API Dokumentation](api/) - API-Referenz
