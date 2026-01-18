# VelinScript Dependency Graph

Das Dependency Graph Tool visualisiert Modul-Abhängigkeiten und erkennt zirkuläre Imports in VelinScript-Projekten.

## Installation

Das Tool ist Teil der VelinScript Toolchain. Baue es mit:

```bash
cd tools/dependency-graph
cargo build --release
```

## Verwendung

### Dependency-Graph generieren

```bash
velin-deps graph
```

Generiert einen Dependency-Graph für das aktuelle Verzeichnis.

### Spezifisches Verzeichnis analysieren

```bash
velin-deps graph src/
```

### Output-Format wählen

```bash
# DOT-Format (Graphviz)
velin-deps graph --format dot

# JSON-Format
velin-deps graph --format json

# SVG-Format (Standard, mit Hinweis auf Graphviz)
velin-deps graph --format svg
```

### Nur zirkuläre Abhängigkeiten anzeigen

```bash
velin-deps graph --circular
```

Zeigt nur Module, die in zirkulären Abhängigkeiten involviert sind.

### Output in Datei speichern

```bash
velin-deps graph --output dependencies.dot
velin-deps graph --format json --output deps.json
```

## Features

### Modul-Abhängigkeiten erkennen

Das Tool analysiert alle `use` Statements in `.velin` Dateien und erstellt einen Graphen der Abhängigkeiten:

```velin
// main.velin
use models;
use services;

// models.velin
use types;

// services.velin
use models;
```

Der Graph zeigt: `main → models`, `main → services`, `models → types`, `services → models`

### Zirkuläre Import-Erkennung

Das Tool erkennt automatisch zirkuläre Abhängigkeiten:

```velin
// a.velin
use b;

// b.velin
use a;  // ⚠️ Zirkuläre Abhängigkeit!
```

**Beispiel-Output:**

```
⚠️  1 zirkuläre Abhängigkeit(en) gefunden:

  a → b → a
```

### Visualisierung

#### DOT-Format

Das DOT-Format kann mit Graphviz visualisiert werden:

```bash
velin-deps graph --format dot --output deps.dot
dot -Tsvg deps.dot -o deps.svg
```

#### JSON-Format

Strukturierte Daten für weitere Verarbeitung:

```json
{
  "nodes": ["main", "models", "services"],
  "edges": [
    ["main", "models"],
    ["main", "services"]
  ],
  "circular_dependencies": [],
  "total_nodes": 3,
  "total_edges": 2,
  "circular_count": 0
}
```

## Beispiel-Output

### Standard-Output

```
🔍 Analysiere Dependencies...

✓ Keine zirkulären Abhängigkeiten gefunden

# Um SVG zu generieren, installiere Graphviz und führe aus:
# dot -Tsvg -o output.svg <(velin-deps graph --format dot)

digraph Dependencies {
  rankdir=LR;
  node [shape=box];

  "main" -> "models";
  "main" -> "services";
  "models" -> "types";
}
```

### Mit zirkulären Abhängigkeiten

```
🔍 Analysiere Dependencies...

⚠️  1 zirkuläre Abhängigkeit(en) gefunden:

  a → b → a

digraph Dependencies {
  rankdir=LR;
  node [shape=box];

  "a" -> "b";
  "b" -> "a";
}
```

## Integration

### CI/CD

```yaml
# .github/workflows/deps-check.yml
- name: Check Dependencies
  run: |
    cd tools/dependency-graph
    cargo build --release
    ./target/release/velin-deps graph --format json > deps.json
    # Prüfe auf zirkuläre Abhängigkeiten
    if jq '.circular_count > 0' deps.json; then
      echo "Zirkuläre Abhängigkeiten gefunden!"
      exit 1
    fi
```

### VS Code Extension

Das Tool kann in VS Code Extensions integriert werden:

```typescript
import { exec } from 'child_process';

exec('velin-deps graph --format json', (error, stdout) => {
  if (error) {
    console.error(error);
    return;
  }
  const deps = JSON.parse(stdout);
  // Visualisiere Dependencies
});
```

## Best Practices

1. **Regelmäßige Checks** - Führe Dependency-Analysen regelmäßig durch
2. **Zirkuläre Abhängigkeiten vermeiden** - Refaktoriere Code, um Zyklen zu vermeiden
3. **Klare Modul-Struktur** - Organisiere Module hierarchisch
4. **Dokumentation** - Dokumentiere Abhängigkeiten in README

## Troubleshooting

### Tool findet keine Dependencies

- Prüfe, ob `.velin` Dateien im angegebenen Verzeichnis existieren
- Prüfe, ob `use` Statements korrekt formatiert sind
- Prüfe Datei-Pfade und Berechtigungen

### Graphviz nicht installiert

Für SVG-Generierung benötigst du Graphviz:

```bash
# Ubuntu/Debian
sudo apt-get install graphviz

# macOS
brew install graphviz

# Windows
choco install graphviz
```

### Zirkuläre Abhängigkeiten beheben

1. Identifiziere die betroffenen Module
2. Refaktoriere gemeinsame Abhängigkeiten in ein separates Modul
3. Verwende Dependency Injection oder Interfaces

## Weitere Ressourcen

- [Modul-Auflösung](../../docs/architecture/module-resolution.md)
- [Tools Übersicht](TOOLS_ÜBERSICHT.md)
- [Wann nutze ich was?](../wann-nutze-ich-was.md)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 0.1.0
