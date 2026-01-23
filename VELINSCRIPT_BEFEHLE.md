# 📋 VelinScript - Alle verfügbaren Befehle

Eine vollständige Übersicht aller nützlichen Befehle von VelinScript 3.1.0.

---

## 🔧 Compiler (velin / velin-compiler)

### Grundlegende Befehle

```bash
# Kompiliert eine VelinScript Datei zu Rust (oder andere Zielsprachen)
velin compile -i <datei> [-o <output>] [--no-type-check] [--show-code] [--autofix] [--target <sprache>] [--framework <framework>] [--ai-semantic] [--ai-bug-detection] [--ai-codegen] [--ai-optimization] [--ai-provider <provider>] [--ai-api-key <key>]

# Code prüfen (Parsing & Type Checking)
velin check -i <datei> [--autofix]

# Code formatieren
velin format -i <datei> [--in-place]

# Informationen über eine Datei anzeigen
velin info -i <datei>

# Neues Projekt initialisieren
velin init [<projektname>] [--current-dir]
# oder
velin new [<projektname>] [--current-dir]

# Development-Server starten
velin serve [-i <datei>] [--port <port>] [--host <host>] [--watch]
# oder
velin run [-i <datei>] [--port <port>] [--host <host>] [--watch]

# OpenAPI Specification generieren
velin open-api -i <datei> [-o <output>]
```

### Code-Generierung

```bash
# Generiert Code-Boilerplate
velin generate <typ> [--name <name>] [--output <datei>]

# Verfügbare Typen:
# - api: Generiert API-Endpoint
# - crud: Generiert CRUD-Operationen
# - test: Generiert Test-Template
# - client: Generiert Client-Code (erfordert --openapi)
# - responses: Generiert Responses-Modul
# - errors: Generiert Errors-Modul
# - logging: Generiert Logging-Modul
# - cache: Generiert Cache-Modul
# - health: Generiert Health-Check-Modul
# - async: Generiert Async-Operations-Modul
# - security: Generiert Security-Modul

# Beispiele:
velin generate api --name UserAPI --path /api/users
velin generate crud --name User --fields "id:string,name:string,email:string"
velin generate client --openapi openapi.json --language typescript
```

### Testing

```bash
# Führt Tests aus
velin test [--directory <verzeichnis>] [--unit] [--integration] [--verbose]
```

### Konfiguration

```bash
# Config-Management
velin config init [--example]
velin config validate [--file <datei>]
velin config show [--file <datei>]
```

### Cache-Management

```bash
# Cache-Statistiken anzeigen
velin cache stats

# Cache leeren
velin cache clear [<pattern>]

# Cache aufwärmen
velin cache warm
```

### Health Check

```bash
# Health Check durchführen
velin health [--url <endpoint>] [--verbose]
```

### Backup & Rollback

```bash
# Backup erstellen
velin backup create [--strategy full|incremental] [--destination <pfad>] [--compression gzip|zip|none]

# Backup wiederherstellen
velin backup restore <backup-id> [--destination <pfad>]

# Backups auflisten
velin backup list [--directory <verzeichnis>]

# Backup löschen
velin backup delete <backup-id> [--directory <verzeichnis>]

# Backup verifizieren
velin backup verify <backup-id> [--directory <verzeichnis>]

# Transaktionen
velin rollback begin
velin rollback commit <transaction-id>
velin rollback rollback <transaction-id>

# Versionen
velin rollback create-version <beschreibung>
velin rollback to-version <version-id>
velin rollback list-versions

# Snapshots
velin rollback create-snapshot <beschreibung>
velin rollback to-snapshot <snapshot-id>
velin rollback list-snapshots
```

### Serialisierung

```bash
# JSON zu YAML konvertieren
velin serialize json-to-yaml -i <input> -o <output>

# YAML zu JSON konvertieren
velin serialize yaml-to-json -i <input> -o <output>

# JSON validieren
velin serialize validate-json -f <datei>

# YAML validieren
velin serialize validate-yaml -f <datei>
```

---

## 📦 Package Manager (velin-pkg)

```bash
# Projekt initialisieren
velin-pkg init [name]

# Dependency hinzufügen
velin-pkg add <package> [--version <version>]
# Beispiel: velin-pkg add github.com/user/repo --version ^1.0.0

# Dependency entfernen
velin-pkg remove <package>

# Dependencies installieren
velin-pkg install

# Dependencies aktualisieren
velin-pkg update [<package>] [--all] [--allow-breaking]

# Installierte Packages auflisten
velin-pkg list

# Package veröffentlichen
velin-pkg publish <version>

# Dependencies auf Vulnerabilities prüfen
velin-pkg audit
```

---

## 🔒 Security Scanner (velin-security)

```bash
# Code auf Security-Vulnerabilities scannen
velin-security scan [<path>] [--format json|html|text]

# Dependencies auf Vulnerabilities prüfen
velin-security audit [--config velin.toml]
```

**Features:**
- Code-Scanning auf Security-Vulnerabilities
- Dependency Vulnerability Scanner
- CVE Database Integration (NVD API)
- GitHub Security Advisories
- OSV (Open Source Vulnerabilities) API

---

## 🐛 Debugger (velin-debugger)

```bash
# DAP Server starten
velin-debugger start [--port <port>]
# Standard-Port: 4711
```

**Features:**
- DAP (Debug Adapter Protocol) Server
- Breakpoints Management
- Variable Inspection
- Call Stack Navigation
- VS Code Integration

---

## 🔍 Linter (velin-lint)

```bash
# Code auf Linter-Probleme prüfen
velin-lint check [<path>] [--fix] [--json] [--rules <regel1,regel2>]
```

**Features:**
- Unused Variables Detection
- Complexity Analysis
- Naming Conventions
- Erweiterbare Regel-Architektur
- Auto-Fix für einfache Probleme

---

## 🧹 Dead Code Detector (velin-dead-code)

```bash
# Code auf Dead Code scannen
velin-dead-code scan [<path>] [--fix] [--json]
```

**Features:**
- Findet ungenutzten Code
- JSON-Report
- (Geplant) Automatisches Entfernen

---

## 📚 API Documentation Generator (velin-api-doc)

```bash
# Generiert OpenAPI Dokumentation
velin-api-doc generate -i <datei> [-o <output>] [--format json|yaml|markdown|html] [--interactive] [--title <titel>] [--version <version>]
```

**Features:**
- JSDoc-Parsing für `///` Kommentare
- HTML-Export
- Interactive Docs (Swagger UI)
- OpenAPI 3.0 Integration
- Unterstützte Formate: JSON, YAML, Markdown, HTML

---

## 🔥 Hot Reload (velin-hot-reload)

```bash
# Überwacht Dateien und kompiliert bei Änderungen
velin-hot-reload --watch [--directory <verzeichnis>] [--compile-command <befehl>]

# Startet Development Server mit Hot Reload
velin-hot-reload --server [--directory <verzeichnis>] [--port <port>] [--run-command <befehl>]
```

**Features:**
- Automatisches Neuladen bei Dateiänderungen
- File System Watching
- Watch-Mode und Server-Mode
- Konfigurierbare Kompilier- und Start-Befehle

---

## 🎯 Häufig verwendete Befehls-Kombinationen

### Neues Projekt erstellen

```bash
# 1. Projekt initialisieren
velin init my-project

# 2. In Projekt wechseln
cd my-project

# 3. Config erstellen
velin config init

# 4. Code kompilieren
velin compile -i main.velin

# 5. Code prüfen
velin check -i main.velin

# 6. Code formatieren
velin format -i main.velin --in-place
```

### Development Workflow

```bash
# 1. Hot Reload starten (in einem Terminal)
velin-hot-reload --watch

# 2. In anderem Terminal: Code entwickeln und testen
velin check -i main.velin
velin test
```

### API-Entwicklung

```bash
# 1. API-Endpoint generieren
velin generate api --name UserAPI --path /api/users

# 2. Code kompilieren
velin compile -i main.velin

# 3. OpenAPI-Dokumentation generieren
velin open-api -i main.velin -o openapi.json

# 4. HTML-Dokumentation generieren
velin-api-doc generate -i main.velin -o docs.html --format html --interactive
```

### Security-Check

```bash
# 1. Code scannen
velin-security scan --format text

# 2. Dependencies prüfen
velin-security audit

# 3. Linter ausführen
velin-lint check --fix
```

---

## 📝 VS Code Extension Commands

Falls die VS Code Extension installiert ist, stehen folgende Commands zur Verfügung:

### Compiler Commands
- `VelinScript: Compile` - Kompiliert die aktuelle Datei
- `VelinScript: Format Document` - Formatiert die aktuelle Datei
- `VelinScript: Check` - Prüft die aktuelle Datei

### Template Generation
- `VelinScript: Generate ML Function Template`
- `VelinScript: Generate Model Loader Boilerplate`
- `VelinScript: Generate AI API Endpoint Template`
- `VelinScript: Generate Responses Template`
- `VelinScript: Generate Errors Template`
- `VelinScript: Generate Logging Template`
- `VelinScript: Generate Cache Template`
- `VelinScript: Generate Health Template`
- `VelinScript: Generate Async Template`
- `VelinScript: Generate Security Template`
- `VelinScript: Generate Backup Template`
- `VelinScript: Generate Rollback Template`
- `VelinScript: Generate Serialization Template`

### Testing
- `VelinScript: Run Tests`
- `VelinScript: Run Unit Tests`
- `VelinScript: Run Integration Tests`

### Configuration
- `VelinScript: Initialize Config File`
- `VelinScript: Validate Config File`

### Analysis & Documentation
- `VelinScript: Generate Insight` - Code-Analyse und Insights
- `VelinScript: Generate AutoDoc` - Automatische Dokumentationserstellung

---

## 💡 Tipps & Tricks

1. **Kompilierung mit Code-Anzeige**: Verwende `--show-code` um den generierten Rust-Code zu sehen
   ```bash
   velin compile -i main.velin --show-code
   ```

2. **Type Checking überspringen**: Für schnelle Tests ohne Type Checking
   ```bash
   velin compile -i main.velin --no-type-check
   ```

3. **Formatierung in-place**: Überschreibt die Datei direkt
   ```bash
   velin format -i main.velin --in-place
   ```

4. **JSON-Output für Linter**: Für CI/CD-Integration
   ```bash
   velin-lint check --json > lint-results.json
   ```

5. **Interaktive API-Dokumentation**: Mit Swagger UI
   ```bash
   velin-api-doc generate -i main.velin -o docs.html --format html --interactive
   ```

---

## 🔗 Weitere Informationen

- **Dokumentation**: Siehe `docs/` Verzeichnis
- **Beispiele**: Siehe `examples/` Verzeichnis
- **Getting Started**: `docs/guides/getting-started.md`
- **Language Specification**: `docs/language/specification.md`

---

**Stand**: VelinScript 3.1.0
