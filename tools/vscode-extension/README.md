# VelinScript VS Code Extension

VS Code Extension für VelinScript Language Support.

## Features

- ✅ Syntax Highlighting
- ✅ Auto-Completion
- ✅ Go-to-Definition
- ✅ Hover-Informationen
- ✅ Code Formatting
- ✅ Error Highlighting
- ✅ Code-Generierung Commands

## Installation

```bash
# Von VSIX
code --install-extension velinscript-0.1.0.vsix

# Oder aus dem Marketplace (wenn verfügbar)
# Suche nach "VelinScript" im VS Code Marketplace
```

## Verwendung

### Commands

**Compiler Commands:**
- `VelinScript: Compile` - Kompiliert die aktuelle Datei
- `VelinScript: Format Document` - Formatiert die aktuelle Datei
- `VelinScript: Check` - Prüft die aktuelle Datei

**Template Generation:**
- `VelinScript: Generate ML Function Template` - Generiert ML-Funktion Template
- `VelinScript: Generate Model Loader Boilerplate` - Generiert Model Loader Template
- `VelinScript: Generate AI API Endpoint Template` - Generiert AI Endpoint Template
- `VelinScript: Generate Responses Template` - Generiert Responses Module
- `VelinScript: Generate Errors Template` - Generiert Errors Module
- `VelinScript: Generate Logging Template` - Generiert Logging Module
- `VelinScript: Generate Cache Template` - Generiert Cache Module
- `VelinScript: Generate Health Template` - Generiert Health Check Module
- `VelinScript: Generate Async Template` - Generiert Async Operations Module
- `VelinScript: Generate Security Template` - Generiert Security Module
- `VelinScript: Generate Backup Template` - Generiert Backup Template
- `VelinScript: Generate Rollback Template` - Generiert Rollback Template
- `VelinScript: Generate Serialization Template` - Generiert Serialization Template

**Testing:**
- `VelinScript: Run Tests` - Führt alle Tests aus
- `VelinScript: Run Unit Tests` - Führt nur Unit Tests aus
- `VelinScript: Run Integration Tests` - Führt nur Integration Tests aus

**Configuration:**
- `VelinScript: Initialize Config File` - Erstellt velin.config.json
- `VelinScript: Validate Config File` - Validiert velin.config.json

**Backup Management:**
- `VelinScript: Create Backup` - Erstellt ein Backup
- `VelinScript: Restore Backup` - Stellt ein Backup wieder her
- `VelinScript: List Backups` - Zeigt alle Backups an

**Rollback Management:**
- `VelinScript: Begin Transaction` - Startet eine Transaktion
- `VelinScript: Commit Transaction` - Committet eine Transaktion
- `VelinScript: Rollback Transaction` - Führt einen Rollback durch

**Serialization:**
- `VelinScript: Convert JSON to YAML` - Konvertiert JSON zu YAML
- `VelinScript: Convert YAML to JSON` - Konvertiert YAML zu JSON

### LSP Features

- Auto-Completion beim Tippen
- Hover-Informationen über Funktionen/Types
- Go-to-Definition mit F12
- Format on Save (konfigurierbar)

**Neue Completion-Funktionen:**
- Response-Funktionen: `successResponse`, `errorResponse`, `successResponseWithCache`
- Error-Funktionen: `createError`, `createValidationError`, `createNotFoundError`
- Logging-Funktionen: `logRequest`, `logResponse`, `logError`, `logPerformance`
- Cache-Funktionen: `cacheGet`, `cacheSet`, `cacheInvalidate`
- Security-Funktionen: `applySecurityMiddleware`, `sanitizeInput`, `validateApiKey`

### Code Snippets

Verfügbare Snippets (Prefix eingeben und Tab drücken):

- `apiresponse` - Standardisierte API Response
- `trycatch` - Try-Catch mit Error Handling
- `cacheget` - Cache-Abfrage mit Fallback
- `logreq` - Request-Logging initialisieren
- `secure` - Sicherer Endpoint mit Middleware
- `success` - Erfolgreiche Response mit Logging
- `error` - Error Response mit Logging
- `cacheset` - Wert im Cache speichern
- `perf` - Performance-Messung

## Konfiguration

```json
{
  "velin.lsp.path": "velin-lsp",
  "velin.compiler.path": "velin"
}
```

## Entwicklung

```bash
cd tools/vscode-extension
npm install
npm run compile
npm run watch
```

### Verfügbare npm Scripts

- `npm run compile` - Kompiliert TypeScript zu JavaScript
- `npm run watch` - Kompiliert TypeScript im Watch-Modus (automatische Neukompilierung bei Änderungen)
- `npm run vscode:prepublish` - Führt `npm run compile` aus (wird vor dem Publishing ausgeführt)

## License

MIT
