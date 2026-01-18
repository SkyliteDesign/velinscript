# Dead Code Detector

Der VelinScript Dead Code Detector findet automatisch ungenutzten Code in deinem Projekt.

## Wofür ist der Dead Code Detector ideal?

Der Dead Code Detector ist ideal für:
- ✅ **Code-Bereinigung** - Findet ungenutzten Code automatisch
- ✅ **Refactoring-Vorbereitung** - Identifiziert Code, der entfernt werden kann
- ✅ **Bundle-Optimierung** - Unterstützt Tree-Shaking durch Dead Code-Entfernung
- ✅ **Code-Wartung** - Hält Codebase sauber und wartbar
- ✅ **CI/CD-Integration** - Kann Dead Code in Pipelines erkennen
- ✅ **Onboarding** - Hilft neuen Entwicklern, ungenutzten Code zu finden

## Wofür ist der Dead Code Detector NICHT gedacht?

Der Dead Code Detector ist NICHT gedacht für:
- ❌ **Code-Qualität** - Für Code-Qualitätsprüfung nutzen Sie den Linter
- ❌ **Bundle-Analyse** - Für detaillierte Bundle-Analyse nutzen Sie den Bundle Analyzer
- ❌ **Performance-Analyse** - Für Performance nutzen Sie den Profiler
- ❌ **Security-Checks** - Für Security nutzen Sie den Security Scanner
- ❌ **Dependency-Analyse** - Für Dependencies nutzen Sie den Dependency Graph

## Installation

Der Dead Code Detector ist Teil des VelinScript Toolchains. Baue ihn mit:

```bash
cd tools/dead-code-detector
cargo build --release
```

## Verwendung

### Basis-Scan

```bash
velin-dead-code scan
```

Scannt das aktuelle Verzeichnis auf Dead Code.

### Spezifisches Verzeichnis scannen

```bash
velin-dead-code scan src/
```

### JSON Report generieren

```bash
velin-dead-code scan --json > dead-code-report.json
```

### Auto-Fix (geplant)

```bash
velin-dead-code scan --fix
```

**Hinweis:** Auto-Fix wird in zukünftigen Versionen unterstützt.

## Was wird erkannt?

Der Dead Code Detector findet:

- **Ungenutzte Funktionen** - Funktionen die nie aufgerufen werden
- **Ungenutzte Variablen** - Variablen die definiert aber nie verwendet werden
- **Ungenutzte Structs/Enums** - Typen die nie referenziert werden
- **Ungenutzte Traits/Impls** - Traits und Implementierungen die nicht verwendet werden
- **Ungenutzte Imports** - `use` Statements für Symbole die nie verwendet werden

## Beispiel-Output

```
VelinScript Dead Code Report
============================
Gefundener Dead Code: 3

[Function] oldFunction
  File: src/main.velin
  Suggestion: Entferne Funktion 'oldFunction' oder verwende sie

[Variable] unusedVar
  File: src/utils.velin
  Suggestion: Entferne Variable 'unusedVar' oder prefixe mit '_'

[Import] unused::module
  File: src/main.velin
  Suggestion: Entferne ungenutzten Import 'module'
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Dead Code Detector                        │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-dead-code scan                                 │
│                                                         │
│  🔍 VelinScript Dead Code Report                       │
│  ============================                           │
│  Gefundener Dead Code: 3                               │
│                                                         │
│  [Function] oldFunction                                 │
│    File: src/main.velin                                │
│    Suggestion: Entferne Funktion 'oldFunction'         │
│                                                         │
│  [Variable] unusedVar                                   │
│    File: src/utils.velin                                │
│    Suggestion: Entferne Variable 'unusedVar'           │
│                                                         │
│  [Import] unused::module                               │
│    File: src/main.velin                                │
│    Suggestion: Entferne ungenutzten Import             │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Best Practices

1. **Regelmäßig scannen** - Integriere Dead Code Detection in deine CI/CD Pipeline
2. **Vor Commits prüfen** - Scanne vor größeren Commits
3. **Vorsicht bei Auto-Fix** - Prüfe Dead Code manuell bevor du ihn entfernst
4. **Public APIs** - Dead Code in öffentlichen APIs könnte von externen Code verwendet werden

## Integration in CI/CD

```yaml
# .github/workflows/dead-code-check.yml
- name: Check for Dead Code
  run: |
    cd tools/dead-code-detector
    cargo build --release
    ./target/release/velin-dead-code scan --json > dead-code.json
    if [ -s dead-code.json ]; then
      echo "Dead Code gefunden!"
      cat dead-code.json
      exit 1
    fi
```
