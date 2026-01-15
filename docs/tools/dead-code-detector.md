# Dead Code Detector

Der VelinScript Dead Code Detector findet automatisch ungenutzten Code in deinem Projekt.

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
