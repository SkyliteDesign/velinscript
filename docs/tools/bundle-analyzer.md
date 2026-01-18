# VelinScript Bundle Analyzer

Das Bundle Analyzer Tool analysiert Bundle-Größe, Tree-Shaking-Potenzial und Code-Splitting-Möglichkeiten in VelinScript-Projekten.

## Installation

Das Tool ist Teil der VelinScript Toolchain. Baue es mit:

```bash
cd tools/bundle-analyzer
cargo build --release
```

## Verwendung

### Bundle analysieren

```bash
velin-bundle analyze
```

Analysiert das aktuelle Verzeichnis auf Bundle-Größe und Potenzial für Optimierungen.

### Spezifisches Verzeichnis analysieren

```bash
velin-bundle analyze src/
```

### Tree-Shaking-Potenzial anzeigen

```bash
velin-bundle analyze --tree-shaking
```

Zeigt ungenutzten Code, der durch Tree-Shaking entfernt werden könnte.

### Code-Splitting-Vorschläge

```bash
velin-bundle analyze --code-splitting
```

Zeigt große Dateien, die aufgeteilt werden könnten.

### JSON-Output

```bash
velin-bundle analyze --json
```

Generiert strukturierte JSON-Daten für weitere Verarbeitung.

### Report in Datei speichern

```bash
velin-bundle analyze --output report.txt
velin-bundle analyze --json --output report.json
```

## Features

### Bundle-Größen-Analyse

Das Tool analysiert alle `.velin` Dateien und berechnet:

- Gesamt-Zeilen Code
- Anzahl Funktionen, Structs, Enums
- Datei-Größen pro Datei
- Gesamt-Statistiken

**Beispiel-Output:**

```
📦 Bundle-Analyse Report
==================================================

## Übersicht

Dateien: 15
Gesamt-Zeilen: 3240
Funktionen: 87
Structs: 23
Enums: 5

## Datei-Größen

  src/main.velin:
    Zeilen: 450, Funktionen: 12, Structs: 3, Enums: 1
  src/models.velin:
    Zeilen: 320, Funktionen: 8, Structs: 5, Enums: 0
```

### Tree-Shaking-Potenzial

Identifiziert ungenutzten Code, der entfernt werden könnte:

```bash
velin-bundle analyze --tree-shaking
```

**Beispiel-Output:**

```
## Tree-Shaking-Potenzial

Ungenutzte Funktionen: 12
Ungenutzte Structs: 3
Ungenutzte Enums: 1
Potenzielle Einsparungen: 14.29%
```

### Code-Splitting-Vorschläge

Zeigt große Dateien, die aufgeteilt werden könnten:

```bash
velin-bundle analyze --code-splitting
```

**Beispiel-Output:**

```
## Code-Splitting-Vorschläge

Große Dateien (könnten aufgeteilt werden):
  - src/main.velin (450 Zeilen)
  - src/services.velin (380 Zeilen)
  - src/utils.velin (320 Zeilen)
```

## JSON-Format

Strukturierte Daten für CI/CD und weitere Verarbeitung:

```json
{
  "total_files": 15,
  "total_lines": 3240,
  "total_functions": 87,
  "total_structs": 23,
  "total_enums": 5,
  "file_sizes": [
    {
      "file": "src/main.velin",
      "lines": 450,
      "functions": 12,
      "structs": 3,
      "enums": 1
    }
  ],
  "tree_shaking_potential": {
    "unused_functions_count": 12,
    "unused_structs_count": 3,
    "unused_enums_count": 1,
    "potential_savings_percent": 14.29
  }
}
```

## Integration

### CI/CD

```yaml
# .github/workflows/bundle-check.yml
- name: Analyze Bundle
  run: |
    cd tools/bundle-analyzer
    cargo build --release
    ./target/release/velin-bundle analyze --json --output bundle.json
    # Prüfe auf große Bundle-Größe
    if jq '.total_lines > 10000' bundle.json; then
      echo "Bundle zu groß!"
      exit 1
    fi
```

### VS Code Extension

```typescript
import { exec } from 'child_process';

exec('velin-bundle analyze --json', (error, stdout) => {
  if (error) {
    console.error(error);
    return;
  }
  const analysis = JSON.parse(stdout);
  // Zeige Bundle-Statistiken
});
```

## Best Practices

1. **Regelmäßige Analyse** - Führe Bundle-Analysen regelmäßig durch
2. **Tree-Shaking nutzen** - Entferne ungenutzten Code
3. **Code-Splitting** - Teile große Dateien in kleinere Module
4. **Monitoring** - Überwache Bundle-Größe im Zeitverlauf

## Troubleshooting

### Tool findet keine Dateien

- Prüfe, ob `.velin` Dateien im angegebenen Verzeichnis existieren
- Prüfe Datei-Pfade und Berechtigungen

### Ungenaue Tree-Shaking-Analyse

Die Tree-Shaking-Analyse basiert auf einer vereinfachten String-basierten Analyse. Für präzisere Ergebnisse sollte der vollständige AST analysiert werden.

## Weitere Ressourcen

- [Tools Übersicht](TOOLS_ÜBERSICHT.md)
- [Wann nutze ich was?](../wann-nutze-ich-was.md)
- [Dead Code Detector](dead-code-detector.md)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 0.1.0
