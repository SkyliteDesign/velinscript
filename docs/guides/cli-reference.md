# CLI-Referenz - Vollständige Befehlsübersicht

**Version:** 3.1.0  
**Status:** ✅ Vollständige CLI-Referenz

---

## Übersicht

Diese Referenz dokumentiert alle verfügbaren CLI-Befehle von VelinScript mit allen Parametern und Optionen.

**Basis-Befehl:** `velin`

---

## Hauptbefehle

### `velin compile` - Kompilierung

Kompiliert eine VelinScript-Datei zu einer Zielsprache.

**Syntax:**
```bash
velin compile -i <input> [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei (.velin) | **Erforderlich** |
| `--output` | `-o` | Pfad | Ausgabe-Datei | Auto (basierend auf target) |
| `--target` | | String | Ziel-Sprache | `rust` |
| `--framework` | | String | Web Framework | Auto |
| `--no-type-check` | | Flag | Überspringe Type Checking | `false` |
| `--show-code` | | Flag | Zeige generierten Code in Konsole | `false` |
| `--autofix` | | Flag | Automatische Fehlerkorrektur | `false` |
| `--ai-semantic` | | Flag | KI-Semantik-Analyse aktivieren | `false` |
| `--ai-bug-detection` | | Flag | KI-Bug-Erkennung aktivieren | `false` |
| `--ai-codegen` | | Flag | KI-Code-Generierung aktivieren | `false` |
| `--ai-optimization` | | Flag | KI-Optimierung aktivieren | `false` |
| `--ai-provider` | | String | AI Provider (openai, anthropic, gemini, local) | - |
| `--ai-api-key` | | String | AI API Key | - |

**Ziel-Sprachen (`--target`):**
- `rust` (Standard)
- `php`
- `python`
- `typescript` / `ts`
- `javascript` / `js`
- `go` / `golang`
- `java`
- `csharp` / `cs`

**Web Frameworks (`--framework`):**
- `axum` (Rust, Standard)
- `actix` (Rust)
- `laravel` (PHP)
- `symfony` (PHP)
- `fastapi` (Python)
- `flask` (Python)
- `express` (TypeScript/JavaScript)
- `nestjs` (TypeScript)

**Beispiele:**
```bash
# Einfache Kompilierung
velin compile -i main.velin

# Mit Ausgabe-Datei
velin compile -i main.velin -o output.rs

# Für PHP
velin compile -i main.velin --target php

# Mit AutoFix
velin compile -i main.velin --autofix

# Mit KI-Features
velin compile -i main.velin \
  --ai-semantic \
  --ai-bug-detection \
  --ai-codegen \
  --ai-provider openai \
  --ai-api-key $OPENAI_API_KEY

# Code in Konsole anzeigen
velin compile -i main.velin --show-code

# Ohne Type Checking (schneller)
velin compile -i main.velin --no-type-check
```

---

### `velin check` - Code-Prüfung

Prüft eine VelinScript-Datei auf Fehler (nur Parsing & Type Checking, keine Code-Generierung).

**Syntax:**
```bash
velin check -i <input> [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei (.velin) | **Erforderlich** |
| `--autofix` | | Flag | Automatische Fehlerkorrektur | `false` |

**Beispiele:**
```bash
# Code prüfen
velin check -i main.velin

# Mit AutoFix
velin check -i main.velin --autofix
```

---

### `velin format` - Code-Formatierung

Formatiert eine VelinScript-Datei.

**Syntax:**
```bash
velin format -i <input> [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei (.velin) | **Erforderlich** |
| `--in-place` | | Flag | Überschreibe die Datei | `false` |

**Beispiele:**
```bash
# Formatierung anzeigen (ohne Datei zu ändern)
velin format -i main.velin

# Datei direkt formatieren
velin format -i main.velin --in-place
```

---

### `velin info` - Datei-Informationen

Zeigt Informationen über eine VelinScript-Datei.

**Syntax:**
```bash
velin info -i <input>
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei (.velin) | **Erforderlich** |

**Beispiele:**
```bash
velin info -i main.velin
```

---

### `velin init` / `velin new` - Projekt initialisieren

Initialisiert ein neues VelinScript-Projekt.

**Syntax:**
```bash
velin init [NAME] [OPTIONS]
velin new [NAME] [OPTIONS]
```

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `NAME` | String | Projekt-Name | - |
| `--current-dir` | Flag | Erstelle im aktuellen Verzeichnis | `false` |

**Beispiele:**
```bash
# Neues Projekt erstellen
velin init my-project

# Im aktuellen Verzeichnis
velin init --current-dir

# Alias
velin new my-api
```

---

### `velin serve` / `velin run` - Development-Server

Startet einen Development-Server (kompiliert und startet die API).

**Syntax:**
```bash
velin serve [OPTIONS]
velin run [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei (.velin) | Auto (main.velin) |
| `--port` | `-p` | Zahl | Port | `8080` |
| `--host` | | String | Host | `localhost` |
| `--watch` | `-w` | Flag | Watch-Mode (automatisches Neuladen) | `false` |

**Beispiele:**
```bash
# Server starten
velin serve

# Mit Port
velin serve -p 3000

# Mit Watch-Mode
velin serve --watch

# Alias
velin run -p 8080 --watch
```

---

### `velin open-api` - OpenAPI-Generierung

Generiert OpenAPI-Spezifikation aus VelinScript-Code.

**Syntax:**
```bash
velin open-api -i <input> [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei (.velin) | **Erforderlich** |
| `--output` | `-o` | Pfad | Ausgabe-Datei (.json oder .yaml) | Auto |

**Beispiele:**
```bash
# OpenAPI generieren
velin open-api -i main.velin

# Mit Ausgabe-Datei
velin open-api -i main.velin -o api.yaml
```

---

### `velin generate` - Code-Generierung

Generiert Code (Boilerplate, CRUD, etc.).

**Syntax:**
```bash
velin generate <TYPE> [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `TYPE` | String | Art der Generierung (api, crud, test, client) | **Erforderlich** |
| `--name` | `-n` | String | Name/Modell für die Generierung | - |
| `--fields` | `-f` | String | Felder (für CRUD) | - |
| `--path` | `-p` | String | Pfad (für API) | - |
| `--openapi` | | Pfad | OpenAPI Datei (für Client) | - |
| `--language` | `-l` | String | Ausgabe-Sprache (für Client) | - |
| `--output` | `-o` | Pfad | Ausgabe-Datei | - |

**Generierungs-Typen:**

1. **`api`** - API Boilerplate
   ```bash
   velin generate api --name User --path /api/users
   ```

2. **`crud`** - CRUD-Operationen
   ```bash
   velin generate crud --name Product --fields "id:string,name:string,price:number"
   ```

3. **`test`** - Test-Stubs
   ```bash
   velin generate test --name UserService
   ```

4. **`client`** - API-Client
   ```bash
   velin generate client --openapi api.json --language typescript
   ```

**Beispiele:**
```bash
# API generieren
velin generate api -n User -p /api/users

# CRUD generieren
velin generate crud -n Product -f "id:string,name:string,price:number"

# Client generieren
velin generate client --openapi api.json -l typescript
```

---

### `velin test` - Tests ausführen

Führt Tests aus (Unit + Integration).

**Syntax:**
```bash
velin test [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--directory` | `-d` | Pfad | Test-Verzeichnis | Auto |
| `--unit` | | Flag | Nur Unit Tests | `false` |
| `--integration` | | Flag | Nur Integration Tests | `false` |
| `--verbose` | `-v` | Flag | Verbose Output | `false` |

**Beispiele:**
```bash
# Alle Tests
velin test

# Nur Unit Tests
velin test --unit

# Nur Integration Tests
velin test --integration

# Verbose
velin test --verbose
```

---

## Konfigurationsbefehle

### `velin config` - Config-Verwaltung

Verwaltet `velin.config.json`.

**Syntax:**
```bash
velin config <SUBCOMMAND>
```

**Subcommands:**

#### `velin config init`

Initialisiert `velin.config.json`.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `--example` | Flag | Verwende Beispiel-Config | `false` |

**Beispiele:**
```bash
velin config init
velin config init --example
```

#### `velin config validate`

Validiert `velin.config.json`.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--file` | `-f` | Pfad | Config-Datei | `velin.config.json` |

**Beispiele:**
```bash
velin config validate
velin config validate -f custom.config.json
```

#### `velin config show`

Zeigt Config-Werte.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--file` | `-f` | Pfad | Config-Datei | `velin.config.json` |

**Beispiele:**
```bash
velin config show
velin config show -f custom.config.json
```

---

## Cache-Befehle

### `velin cache` - Cache-Management

Verwaltet den Compiler-Cache.

**Syntax:**
```bash
velin cache <SUBCOMMAND>
```

**Subcommands:**

#### `velin cache stats`

Zeigt Cache-Statistiken.

**Beispiele:**
```bash
velin cache stats
```

#### `velin cache clear`

Leert Cache.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `PATTERN` | String | Pattern für Keys | - |

**Beispiele:**
```bash
# Alles leeren
velin cache clear

# Mit Pattern
velin cache clear "compiled:*"
```

#### `velin cache warm`

Wärmt Cache (lädt häufig verwendete Daten vor).

**Beispiele:**
```bash
velin cache warm
```

---

## Backup & Rollback

### `velin backup` - Backup-Management

Verwaltet Backups.

**Syntax:**
```bash
velin backup <SUBCOMMAND>
```

**Subcommands:**

#### `velin backup create`

Erstellt ein Backup.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--strategy` | `-s` | String | Backup-Strategie (full, incremental) | `full` |
| `--destination` | `-d` | String | Ziel-Verzeichnis | Auto |
| `--compression` | `-c` | String | Komprimierung (gzip, zip, none) | `gzip` |

**Beispiele:**
```bash
velin backup create
velin backup create -s incremental -c zip
```

#### `velin backup restore`

Stellt ein Backup wieder her.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `BACKUP_ID` | String | Backup-ID | **Erforderlich** |
| `--destination` | `-d` | String | Ziel-Verzeichnis | Auto |

**Beispiele:**
```bash
velin backup restore backup-123
velin backup restore backup-123 -d /restore/path
```

#### `velin backup list`

Listet alle Backups auf.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--directory` | `-d` | String | Verzeichnis mit Backups | Auto |

**Beispiele:**
```bash
velin backup list
velin backup list -d /backups
```

#### `velin backup delete`

Löscht ein Backup.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `BACKUP_ID` | String | Backup-ID | **Erforderlich** |
| `--directory` | `-d` | String | Verzeichnis mit Backups | Auto |

**Beispiele:**
```bash
velin backup delete backup-123
```

#### `velin backup verify`

Verifiziert ein Backup.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `BACKUP_ID` | String | Backup-ID | **Erforderlich** |
| `--directory` | `-d` | String | Verzeichnis mit Backups | Auto |

**Beispiele:**
```bash
velin backup verify backup-123
```

---

### `velin rollback` - Rollback-Management

Verwaltet Rollbacks und Versionen.

**Syntax:**
```bash
velin rollback <SUBCOMMAND>
```

**Subcommands:**

#### `velin rollback begin`

Beginnt eine Transaktion.

**Beispiele:**
```bash
velin rollback begin
```

#### `velin rollback commit`

Committet eine Transaktion.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `TRANSACTION_ID` | String | Transaktions-ID | **Erforderlich** |

**Beispiele:**
```bash
velin rollback commit tx-123
```

#### `velin rollback rollback`

Rollback einer Transaktion.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `TRANSACTION_ID` | String | Transaktions-ID | **Erforderlich** |

**Beispiele:**
```bash
velin rollback rollback tx-123
```

#### `velin rollback create-version`

Erstellt eine Version.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `DESCRIPTION` | String | Beschreibung | **Erforderlich** |

**Beispiele:**
```bash
velin rollback create-version "Version 1.0.0"
```

#### `velin rollback to-version`

Rollback zu einer Version.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `VERSION_ID` | String | Version-ID | **Erforderlich** |

**Beispiele:**
```bash
velin rollback to-version v1.0.0
```

#### `velin rollback list-versions`

Listet alle Versionen auf.

**Beispiele:**
```bash
velin rollback list-versions
```

#### `velin rollback create-snapshot`

Erstellt einen Snapshot.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `DESCRIPTION` | String | Beschreibung | **Erforderlich** |

**Beispiele:**
```bash
velin rollback create-snapshot "Pre-deployment snapshot"
```

#### `velin rollback to-snapshot`

Rollback zu einem Snapshot.

**Parameter:**

| Parameter | Typ | Beschreibung | Standard |
|-----------|-----|--------------|----------|
| `SNAPSHOT_ID` | String | Snapshot-ID | **Erforderlich** |

**Beispiele:**
```bash
velin rollback to-snapshot snap-123
```

#### `velin rollback list-snapshots`

Listet alle Snapshots auf.

**Beispiele:**
```bash
velin rollback list-snapshots
```

---

## Serialization-Tools

### `velin serialize` - Serialization-Tools

Konvertiert und validiert JSON/YAML.

**Syntax:**
```bash
velin serialize <SUBCOMMAND>
```

**Subcommands:**

#### `velin serialize json-to-yaml`

Konvertiert JSON zu YAML.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei | **Erforderlich** |
| `--output` | `-o` | Pfad | Ausgabe-Datei | **Erforderlich** |

**Beispiele:**
```bash
velin serialize json-to-yaml -i config.json -o config.yaml
```

#### `velin serialize yaml-to-json`

Konvertiert YAML zu JSON.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--input` | `-i` | Pfad | Eingabe-Datei | **Erforderlich** |
| `--output` | `-o` | Pfad | Ausgabe-Datei | **Erforderlich** |

**Beispiele:**
```bash
velin serialize yaml-to-json -i config.yaml -o config.json
```

#### `velin serialize validate-json`

Validiert JSON.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--file` | `-f` | Pfad | Datei | **Erforderlich** |

**Beispiele:**
```bash
velin serialize validate-json -f data.json
```

#### `velin serialize validate-yaml`

Validiert YAML.

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--file` | `-f` | Pfad | Datei | **Erforderlich** |

**Beispiele:**
```bash
velin serialize validate-yaml -f config.yaml
```

---

## Health Check

### `velin health` - Health Check

Prüft die Gesundheit eines Endpoints.

**Syntax:**
```bash
velin health [OPTIONS]
```

**Parameter:**

| Parameter | Kurzform | Typ | Beschreibung | Standard |
|-----------|----------|-----|--------------|----------|
| `--url` | `-u` | String | Endpoint-URL | - |
| `--verbose` | `-v` | Flag | Zeige detaillierte Metriken | `false` |

**Beispiele:**
```bash
velin health -u http://localhost:8080/health
velin health -u http://localhost:8080/health --verbose
```

---

## Hilfe & Version

### `velin --help` / `velin -h`

Zeigt Hilfe-Informationen.

**Beispiele:**
```bash
velin --help
velin compile --help
velin config --help
```

### `velin --version` / `velin -V`

Zeigt Version.

**Beispiele:**
```bash
velin --version
```

---

## Umgebungsvariablen

### AI-Provider-Konfiguration

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# Anthropic
export ANTHROPIC_API_KEY="sk-ant-..."

# Google Gemini
export GEMINI_API_KEY="..."
```

### Compiler-Konfiguration

```bash
# Cache-Verzeichnis
export VELIN_CACHE_DIR="/path/to/cache"

# Log-Level
export VELIN_LOG_LEVEL="debug"
```

---

## Konfigurationsdatei

### `velin.config.json`

```json
{
  "compiler": {
    "target": "rust",
    "framework": "axum",
    "autofix": true,
    "type_check": true
  },
  "ai": {
    "provider": "openai",
    "api_key": "${OPENAI_API_KEY}"
  },
  "cache": {
    "enabled": true,
    "directory": ".velin/cache"
  }
}
```

---

## Best Practices

### Entwicklung

1. **Verwende `velin serve --watch`** für Development
2. **Nutze `velin check`** vor Commits
3. **Verwende `velin format --in-place`** für konsistente Formatierung

### Production

1. **Nutze `velin compile`** mit expliziten Targets
2. **Aktiviere `--ai-codegen`** nur wenn nötig
3. **Verwende `velin backup create`** vor Deployments

---

## Siehe auch

- [Getting Started](getting-started.md) - Erste Schritte
- [API-Keys Setup](api-keys-setup.md) - API-Keys Konfiguration
- [Tools-Übersicht](../tools/TOOLS_ÜBERSICHT.md) - Alle Tools

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
