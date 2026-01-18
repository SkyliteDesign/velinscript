# VelinScript Profiler

Der Profiler führt CPU- und Memory-Profiling durch und generiert Flame Graphs.

## Installation

Das Tool ist Teil der VelinScript Toolchain. Baue es mit:

```bash
cd tools/profiler
cargo build --release
```

## Verwendung

### CPU-Profiling

```bash
velin-profile cpu main.velin
```

### CPU-Profiling mit Flame Graph

```bash
velin-profile cpu main.velin --flamegraph
```

### Memory-Profiling

```bash
velin-profile memory main.velin
```

### Output speichern

```bash
velin-profile memory main.velin --output memory-report.json
```

## Features

### CPU-Profiling

Analysiert CPU-Auslastung und Funktion-Performance:

```
⚡ CPU-Profiling für: main.velin

📊 CPU-Profiling-Ergebnisse:
  Gesamt-Zeit: 125.45ms
  Funktionen: 5
```

### Memory-Profiling

Analysiert Memory-Allokationen:

```
💾 Memory-Profiling für: main.velin

📊 Memory-Profiling-Ergebnisse:
  Gesamt-Allokationen: 10240 bytes
  Peak-Memory: 10240 bytes
  Allokationen: 15
```

### Flame Graphs

Visualisiert CPU-Usage als Flame Graph:

```bash
velin-profile cpu main.velin --flamegraph
```

Generiert `flamegraph.svg` mit visueller Darstellung der Funktion-Performance.

## Integration

### CI/CD

```yaml
# .github/workflows/profile.yml
- name: Profile Code
  run: |
    cd tools/profiler
    cargo build --release
    ./target/release/velin-profile cpu main.velin --flamegraph
```

## Best Practices

1. **Release-Builds** - Profile immer Release-Builds
2. **Repräsentative Workloads** - Nutze realistische Test-Daten
3. **Mehrfache Runs** - Führe Profiling mehrfach durch
4. **Flame Graphs** - Nutze Flame Graphs für visuelle Analyse

## Troubleshooting

### Profiling ist zu langsam

- Reduziere Code-Umfang
- Nutze Sampling statt Instrumentation
- Prüfe System-Last

### Flame Graph wird nicht generiert

- Prüfe Output-Verzeichnis-Berechtigungen
- Prüfe SVG-Generierung

## Weitere Ressourcen

- [Tools Übersicht](TOOLS_ÜBERSICHT.md)
- [Wann nutze ich was?](../wann-nutze-ich-was.md)
- [Benchmark Runner](benchmark-runner.md)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 0.1.0
