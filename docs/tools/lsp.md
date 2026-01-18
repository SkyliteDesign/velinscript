# VelinScript Language Server (LSP)

Der VelinScript Language Server implementiert das Language Server Protocol (LSP) für vollständige IDE-Unterstützung.

## Installation

Der LSP Server ist Teil des VelinScript Toolchains. Baue ihn mit:

```bash
cd tools/lsp
cargo build --release
```

## Features

### Auto-Completion

Vollständige Code-Vervollständigung für:
- Funktionen und Methoden
- Structs und Enums
- Variablen und Konstanten
- Standardbibliothek-Funktionen
- Decorators

### Go-to-Definition

Springe zur Definition von:
- Funktionen
- Structs/Enums
- Variablen
- Imports

### Hover-Informationen

Zeigt beim Hovern über Code:
- Funktions-Signaturen
- Dokumentation
- Typ-Informationen
- Parameter-Details

### Error Highlighting

Zeigt Compiler-Fehler direkt im Editor:
- Syntax-Fehler
- Type-Errors
- Unused Variables
- Missing Imports

### Code Formatting

Automatische Code-Formatierung:
- Format on Save
- Format Selection
- Format Document

### Refactoring

Unterstützte Refactorings:
- Rename Symbol
- Extract Function
- Extract Variable
- Inline Variable

### Find All References

Findet alle Referenzen zu:
- Funktionen
- Variablen
- Structs/Enums
- Imports

### Code Actions

Verfügbare Code Actions:
- Quick Fix für Compiler-Fehler
- Import-Organisierung
- Unused Code entfernen
- Add Missing Documentation

## Verwendung

### Manueller Start

```bash
velin-lsp
```

Der LSP Server kommuniziert über stdin/stdout mit dem Client.

### VS Code Integration

Der LSP Server wird automatisch von der VS Code Extension verwendet. Keine manuelle Konfiguration nötig.

### Andere IDEs

Jeder LSP-kompatible Editor kann den Server verwenden:

#### Neovim

```lua
-- init.lua
local lspconfig = require('lspconfig')
lspconfig.velin.setup({
  cmd = {'velin-lsp'},
  filetypes = {'velin'},
  root_dir = lspconfig.util.root_pattern('velin.toml', '.git'),
})
```

#### Emacs (lsp-mode)

```elisp
(require 'lsp-mode)
(add-to-list 'lsp-language-id-configuration '(velin-mode . "velin"))
(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection "velin-lsp")
  :activation-fn (lsp-activate-on "velin")
  :server-id 'velin))
```

#### Vim (vim-lsp)

```vim
if executable('velin-lsp')
  au User lsp_setup call lsp#register_server({
    \ 'name': 'velin-lsp',
    \ 'cmd': {server_info->['velin-lsp']},
    \ 'whitelist': ['velin'],
    \ })
endif
```

## LSP Features im Detail

### Text Document Synchronization

- **DidOpen** - Dokument wird geöffnet
- **DidChange** - Dokument wurde geändert
- **DidClose** - Dokument wurde geschlossen
- **DidSave** - Dokument wurde gespeichert

### Completion

```json
{
  "label": "string.split",
  "kind": 2,
  "detail": "fn(string, string) -> List<string>",
  "documentation": "Teilt einen String an einem Delimiter"
}
```

### Hover

```json
{
  "contents": {
    "kind": "markdown",
    "value": "```velin\nfn split(text: string, delimiter: string) -> List<string>\n```\n\nTeilt einen String an einem Delimiter"
  },
  "range": {
    "start": {"line": 5, "character": 10},
    "end": {"line": 5, "character": 16}
  }
}
```

### Diagnostics

```json
{
  "diagnostics": [
    {
      "range": {
        "start": {"line": 10, "character": 5},
        "end": {"line": 10, "character": 12}
      },
      "severity": 1,
      "message": "Variable 'unused' wird nicht verwendet",
      "source": "velin-compiler"
    }
  ]
}
```

### Code Actions

```json
{
  "title": "Remove unused variable",
  "kind": "quickfix",
  "edit": {
    "changes": {
      "file:///path/to/file.velin": [
        {
          "range": {
            "start": {"line": 10, "character": 0},
            "end": {"line": 10, "character": 20}
          },
          "newText": ""
        }
      ]
    }
  }
}
```

## Konfiguration

### VS Code Settings

```json
{
  "velin.lsp.path": "velin-lsp",
  "velin.lsp.trace": "off",
  "velin.lsp.logLevel": "info"
}
```

### LSP Initialization Options

```json
{
  "compiler": {
    "path": "velin-compiler",
    "features": ["axum", "sea-orm"]
  },
  "formatting": {
    "enabled": true,
    "formatOnSave": true
  },
  "diagnostics": {
    "enabled": true,
    "level": "all"
  }
}
```

## Performance

### Caching

Der LSP Server cached:
- Parsed ASTs
- Type-Informationen
- Completion-Listen
- Symbol-Tabellen

### Incremental Updates

- Nur geänderte Dateien werden neu geparst
- Type-Checking nur für betroffene Module
- Optimierte Symbol-Resolution

## Troubleshooting

### LSP Server startet nicht

- Prüfe, ob `velin-lsp` im PATH ist
- Prüfe LSP-Logs in VS Code (Output Panel)
- Prüfe Firewall-Einstellungen

### Auto-Completion funktioniert nicht

- Prüfe, ob Datei als `.velin` erkannt wird
- Prüfe LSP-Status in VS Code (Status Bar)
- Starte LSP Server neu

### Fehler werden nicht angezeigt

- Prüfe LSP-Diagnostics-Einstellungen
- Prüfe, ob Compiler korrekt konfiguriert ist
- Prüfe LSP-Logs

### Performance-Probleme

- Reduziere Anzahl geöffneter Dateien
- Deaktiviere nicht benötigte Features
- Prüfe System-Ressourcen

## Entwicklung

Der LSP Server nutzt `velin-compiler` als Dependency:

```toml
[dependencies]
velin-compiler = { path = "../../compiler" }
```

### Erweitern

Um neue LSP-Features hinzuzufügen:

1. Implementiere Handler in `src/handlers/`
2. Registriere Handler in `src/server.rs`
3. Teste mit LSP-kompatiblem Client

## Testing

```bash
cd tools/lsp
cargo test
```

Der LSP Server hat umfangreiche Tests für alle Features.

## Best Practices

1. **LSP-kompatible Editoren** - Verwende LSP-kompatible Editoren für beste Erfahrung
2. **Regelmäßige Updates** - Halte LSP Server auf neuestem Stand
3. **Logging** - Aktiviere Logging für Debugging
4. **Performance** - Überwache LSP-Performance bei großen Projekten
5. **Konfiguration** - Passe LSP-Einstellungen nach Bedarf an
