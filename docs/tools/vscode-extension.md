# VS Code Extension für VelinScript

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

Die VS Code Extension bietet vollständige IDE-Unterstützung für VelinScript mit Syntax-Highlighting, IntelliSense, Debugging und Code-Generierung.

## Wofür ist die VS Code Extension ideal?

Die VS Code Extension ist ideal für:
- ✅ **Vollständige IDE-Erfahrung** - All-in-One-Lösung für VelinScript-Entwicklung
- ✅ **Multi-Target Support** - Entwicklung für Rust, TypeScript, Python, PHP, Java, C#
- ✅ **Syntax-Highlighting** - Farbige Syntax-Hervorhebung für bessere Lesbarkeit
- ✅ **IntelliSense** - Auto-Completion, Hover-Info, Go-to-Definition
- ✅ **Debugging** - Integrierter Debugger mit Breakpoints
- ✅ **Code-Generierung** - Templates und Code-Snippets
- ✅ **Format-on-Save** - Automatische Code-Formatierung

## Wofür ist die VS Code Extension NICHT gedacht?

Die VS Code Extension ist NICHT gedacht für:
- ❌ **CLI-Tools** - Für Command-Line-Tools nutzen Sie die separaten Tools
- ❌ **CI/CD-Integration** - Für Pipelines nutzen Sie die CLI-Tools direkt
- ❌ **Alternative IDEs** - Für andere IDEs nutzen Sie den LSP direkt
- ❌ **Headless-Umgebungen** - Für Server ohne GUI nutzen Sie CLI-Tools

## Features

### Multi-Target Support (Neu in 3.1)

**Status:** ✅ Vollständig implementiert

Die Extension unterstützt nun die Entwicklung für verschiedene Zielsprachen:
- **Framework Detection**: Automatische Erkennung des verwendeten Frameworks (z.B. Laravel, FastAPI, Spring)
- **Target Configuration**: Einstellung des Targets via `velin.config.json` oder VS Code Settings
- **Syntax Highlighting**: Erweiterte Unterstützung für Framework-spezifische Decorators (`@NestJS`, `@Spring`, `@AspNet`)

### Syntax-Highlighting

**Status:** ✅ Vollständig implementiert

Unterstützt alle VelinScript-Features:
- Keywords (fn, let, return, etc.)
- Decorators (@GET, @POST, @Auth, @Flow, @VelinAutoDoc, @Express, @Spring, etc.)
- Types (string, number, boolean, List, Result, etc.)
- String Interpolation
- Doc-Comments (`///`)

**Datei:** `tools/vscode-extension/syntaxes/velin.tmLanguage.json`

### Code Snippets

**Status:** ✅ Vollständig implementiert

**Verfügbare Snippets:**

1. **velin-flow** - Template für `@Flow` Funktionen
2. **velin-autodoc** - Template für Funktionen mit `@VelinAutoDoc`
3. **velin-autotest** - Template für `@VelinAutoTest`
4. **velin-pipeline** - Template für `@VelinPipeline` Module
5. **velin-insight** - Template für `@VelinInsight` Module
6. **ts-express** - TypeScript Express Endpoint
7. **java-spring** - Java Spring Controller
8. **csharp-aspnet** - C# ASP.NET Controller

**Datei:** `tools/vscode-extension/snippets/velin.json`

### Commands

**Status:** ✅ Vollständig implementiert

#### Compiler Commands

- `velin.compile` - Kompiliert eine VelinScript Datei
- `velin.check` - Prüft eine VelinScript Datei
- `velin.format` - Formatiert eine VelinScript Datei

#### Code Generation Commands

- `velin.generate.mlFunction` - Generiert ML-Funktion
- `velin.generate.modelLoader` - Generiert ModelLoader
- `velin.generate.aiEndpoint` - Generiert AI-Endpoint
- `velin.generate.responses` - Generiert Response-Typen
- `velin.generate.errors` - Generiert Error-Typen
- `velin.generate.logging` - Generiert Logging-Code
- `velin.generate.cache` - Generiert Cache-Code
- `velin.generate.health` - Generiert Health-Check
- `velin.generate.async` - Generiert Async-Code
- `velin.generate.security` - Generiert Security-Code
- `velin.generate.backup` - Generiert Backup-Code
- `velin.generate.rollback` - Generiert Rollback-Code
- `velin.generate.serialization` - Generiert Serialization-Code

#### Intelligence Commands

- `velin.insight` - Führt Code-Analyse aus (VelinInsight)
- `velin.autodoc` - Generiert automatische Dokumentation (VelinAutoDoc)

#### Test Commands

- `velin.test` - Führt Tests aus
- `velin.test.unit` - Führt nur Unit-Tests aus
- `velin.test.integration` - Führt nur Integration-Tests aus

#### Config Commands

- `velin.config.init` - Initialisiert velin.config.json
- `velin.config.validate` - Validiert velin.config.json

#### Backup & Rollback Commands

- `velin.backup.create` - Erstellt ein Backup
- `velin.backup.restore` - Stellt ein Backup wieder her
- `velin.backup.list` - Listet alle Backups auf
- `velin.rollback.begin` - Beginnt eine Transaktion
- `velin.rollback.commit` - Committet eine Transaktion
- `velin.rollback.rollback` - Rollback einer Transaktion

#### Serialization Commands

- `velin.serialize.jsonToYaml` - Konvertiert JSON zu YAML
- `velin.serialize.yamlToJson` - Konvertiert YAML zu JSON

### Debugger Integration

**Status:** ✅ Vollständig implementiert

- DAP (Debug Adapter Protocol) Support
- Breakpoints Management
- Variable Inspection
- Call Stack Navigation
- Watch Expressions

**Konfiguration:**
```json
{
  "velin.debugger.path": "velin-debugger",
  "velin.debugger.port": 4711
}
```

### LSP Integration

**Status:** ✅ Vollständig implementiert

- Auto-Completion
- Go to Definition
- Find All References
- Rename Symbol
- Code Actions
- Hover Documentation
- Signature Help

**Konfiguration:**
```json
{
  "velin.lsp.path": "velin-lsp"
}
```

## Installation

### Aus Source

```bash
cd tools/vscode-extension
npm install
npm run compile
code --install-extension velinscript-0.1.0.vsix
```

### Aus Marketplace

*(Später verfügbar)*

## Verwendung

### Code Snippets verwenden

1. Öffne eine `.velin` Datei
2. Tippe den Snippet-Präfix (z.B. `velin-flow`)
3. Drücke `Tab` oder `Enter`
4. Der Snippet wird eingefügt

### Commands ausführen

1. Öffne die Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`)
2. Tippe `VelinScript:`
3. Wähle den gewünschten Command

### Debugging

1. Erstelle eine `.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "velin",
      "request": "launch",
      "name": "Debug VelinScript",
      "program": "${workspaceFolder}/main.velin"
    }
  ]
}
```

2. Setze Breakpoints
3. Starte Debugging (`F5`)

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VS Code mit VelinScript Extension                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  [Editor]                                               │
│    fn processOrder(order: Order) {                     │
│        let user = db.findUser(order.userId)?;          │
│        let payment = processPayment(order)?;            │
│        return Ok(order);                                │
│    }                                                    │
│                                                         │
│  [Syntax Highlighting aktiv]                            │
│  [Auto-Completion aktiv]                                │
│  [Error Highlighting: Keine Fehler]                     │
│                                                         │
│  [Status Bar]                                           │
│    VelinScript ✓ | LSP: Ready | Debug: Ready           │
│                                                         │
│  [Command Palette]                                      │
│    VelinScript: Compile                                 │
│    VelinScript: Generate API                            │
│    VelinScript: Run Tests                              │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 2.5.0
