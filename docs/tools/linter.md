# VelinScript Linter

Der VelinScript Linter analysiert Code auf Qualität, Best Practices und potenzielle Probleme.

## Wofür ist der Linter ideal?

Der Linter ist ideal für:
- ✅ **Code-Qualitätssicherung** - Findet Code-Smells und Qualitätsprobleme automatisch
- ✅ **Best Practices** - Enforced Coding-Standards und Konventionen
- ✅ **Ungenutzten Code** - Identifiziert ungenutzte Variablen, Funktionen und Imports
- ✅ **Komplexitäts-Analyse** - Erkennt zu komplexe Funktionen und verschachtelte Strukturen
- ✅ **Pre-Commit-Checks** - Perfekt für Git Hooks und CI/CD-Pipelines
- ✅ **Team-Konsistenz** - Stellt sicher, dass alle Teammitglieder den gleichen Standards folgen

## Wofür ist der Linter NICHT gedacht?

Der Linter ist NICHT gedacht für:
- ❌ **Syntax-Fehler** - Für Syntax-Korrektur nutzen Sie AutoFix
- ❌ **Type-Checking** - Für Type-Validation nutzen Sie den Compiler (`velin check`)
- ❌ **Security-Checks** - Für Security-Vulnerabilities nutzen Sie den Security Scanner
- ❌ **Performance-Analyse** - Für Performance-Optimierung nutzen Sie den Profiler
- ❌ **Runtime-Debugging** - Für Live-Debugging nutzen Sie den Debugger oder Runtime Inspector

## Installation

Der Linter ist Teil des VelinScript Toolchains. Baue ihn mit:

```bash
cd tools/linter
cargo build --release
```

## Verwendung

### Basis-Check

```bash
velin-lint check
```

Analysiert das aktuelle Verzeichnis auf Linter-Probleme.

### Spezifisches Verzeichnis prüfen

```bash
velin-lint check src/
```

### Spezifische Datei prüfen

```bash
velin-lint check main.velin
```

### JSON-Output

```bash
velin-lint check --json > lint-report.json
```

### Auto-Fix (geplant)

```bash
velin-lint check --fix
```

**Hinweis:** Auto-Fix wird in zukünftigen Versionen unterstützt.

### Bestimmte Regeln ausführen

```bash
velin-lint check --rules unused-variable --rules long-function
```

## Verfügbare Regeln

### Code-Qualität

- **unused-variable** - Erkennt ungenutzte Variablen
- **unused-function** - Erkennt ungenutzte Funktionen
- **unused-import** - Erkennt ungenutzte Imports
- **long-function** - Warnt bei zu langen Funktionen (>50 Zeilen)
- **complex-function** - Warnt bei zu komplexen Funktionen (hohe Zyklomatische Komplexität)
- **deeply-nested** - Warnt bei zu tief verschachteltem Code (>4 Ebenen)

### Best Practices

- **naming-convention** - Prüft Namenskonventionen (camelCase für Funktionen, PascalCase für Structs)
- **missing-docs** - Warnt bei fehlender Dokumentation für öffentliche Funktionen
- **magic-numbers** - Warnt bei Magic Numbers (sollten als Konstanten definiert werden)
- **error-handling** - Prüft auf fehlende Error-Handling

### Performance

- **inefficient-loop** - Erkennt ineffiziente Schleifen
- **unnecessary-clone** - Erkennt unnötige Clone-Operationen
- **large-struct** - Warnt bei sehr großen Structs (>100 Felder)

## Beispiel-Output

```
🔍 Analysiere 5 Datei(en)...

📊 Gefundene Probleme: 3

src/main.velin:15:3 [warning] unused-variable: Ungenutzte Variable: 'temp'
  💡 Vorschlag: Entferne die Variable 'temp' oder verwende sie

src/utils.velin:42:1 [warning] long-function: Funktion 'processData' ist zu lang (67 Zeilen)
  💡 Vorschlag: Teile die Funktion in kleinere Funktionen auf

src/api.velin:8:1 [info] missing-docs: Öffentliche Funktion 'createUser' hat keine Dokumentation
  💡 Vorschlag: Füge /// Doc-Comment hinzu
```

### JSON-Output Format

```json
{
  "issues": [
    {
      "file": "src/main.velin",
      "line": 15,
      "column": 3,
      "severity": "warning",
      "rule": "unused-variable",
      "message": "Ungenutzte Variable: 'temp'",
      "suggestion": "Entferne die Variable 'temp' oder verwende sie"
    }
  ],
  "total": 1
}
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Linter                                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-lint check                                     │
│                                                         │
│  🔍 Analysiere 5 Datei(en)...                          │
│                                                         │
│  📊 Gefundene Probleme: 3                              │
│                                                         │
│  src/main.velin:15:3 [warning] unused-variable         │
│    Ungenutzte Variable: 'temp'                          │
│    💡 Vorschlag: Entferne die Variable 'temp'          │
│                                                         │
│  src/utils.velin:42:1 [warning] long-function          │
│    Funktion 'processData' ist zu lang (67 Zeilen)      │
│    💡 Vorschlag: Teile die Funktion auf                │
│                                                         │
│  src/api.velin:8:1 [info] missing-docs                 │
│    Öffentliche Funktion 'createUser' hat keine Docs    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Konfiguration

Erstelle eine `.velinlintrc.json` Datei im Projekt-Root:

```json
{
  "rules": {
    "unused-variable": "error",
    "long-function": "warning",
    "missing-docs": "info"
  },
  "ignore": [
    "target/**",
    "vendor/**"
  ],
  "max-function-length": 50,
  "max-complexity": 10,
  "max-nesting-depth": 4
}
```

## Integration in CI/CD

```yaml
# .github/workflows/lint.yml
- name: Lint Code
  run: |
    cd tools/linter
    cargo build --release
    ./target/release/velin-lint check --json > lint-report.json
    if [ -s lint-report.json ]; then
      cat lint-report.json
      exit 1
    fi
```

## Best Practices

1. **Regelmäßig linten** - Integriere Linting in deine CI/CD Pipeline
2. **Vor Commits prüfen** - Führe Linting vor größeren Commits aus
3. **Regeln anpassen** - Konfiguriere Regeln nach Projekt-Bedarf
4. **Schrittweise einführen** - Beginne mit Warnungen, erhöhe später auf Errors
5. **Team-Konsens** - Diskutiere Regeln mit dem Team

## VS Code Integration

Der Linter ist automatisch in der VS Code Extension integriert. Probleme werden direkt im Editor angezeigt.

## Troubleshooting

### Zu viele Warnungen

- Passe die Konfiguration an (`.velinlintrc.json`)
- Ignoriere bestimmte Dateien oder Verzeichnisse
- Beginne mit weniger strengen Regeln

### Falsch-positive Ergebnisse

- Melde Issues im Repository
- Verwende `// velin-lint-disable-next-line` für spezifische Zeilen
- Verwende `// velin-lint-disable` für ganze Dateien

### Performance-Probleme

- Verwende `--rules` um nur bestimmte Regeln auszuführen
- Ignoriere große Verzeichnisse (z.B. `target/`, `vendor/`)
- Führe Linting nur auf geänderten Dateien aus
