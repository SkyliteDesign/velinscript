# VelinScript Profiler

Der Profiler führt CPU- und Memory-Profiling durch und generiert Flame Graphs.

## Wofür ist der Profiler ideal?

Der Profiler ist ideal für:
- ✅ **Performance-Bottlenecks** - Findet langsame Code-Stellen
- ✅ **CPU-Analyse** - Identifiziert CPU-intensive Funktionen
- ✅ **Memory-Leaks** - Erkennt Memory-Probleme und übermäßige Allokationen
- ✅ **Flame Graphs** - Visualisiert Performance-Daten für einfache Analyse
- ✅ **Optimierungs-Planung** - Zeigt, wo Optimierungen den größten Effekt haben
- ✅ **Production-Debugging** - Analysiert Performance-Probleme in Production-ähnlichen Umgebungen

## Wofür ist der Profiler NICHT gedacht?

Der Profiler ist NICHT gedacht für:
- ❌ **Statische Code-Analyse** - Für Code-Qualität nutzen Sie den Linter
- ❌ **Unit-Tests** - Für Tests nutzen Sie den Test Runner
- ❌ **Benchmark-Vergleiche** - Für statistische Benchmarks nutzen Sie den Benchmark Runner
- ❌ **Security-Checks** - Für Security-Vulnerabilities nutzen Sie den Security Scanner
- ❌ **Code-Generierung** - Für Boilerplate nutzen Sie Code Generation

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

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Profiler                                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-profile cpu main.velin --flamegraph           │
│                                                         │
│  ⚡ CPU-Profiling für: main.velin                      │
│                                                         │
│  📊 CPU-Profiling-Ergebnisse:                          │
│    Gesamt-Zeit: 125.45ms                               │
│    Funktionen: 5                                        │
│                                                         │
│  🔥 Generiere Flame Graph...                          │
│  ✓ Flame Graph gespeichert: flamegraph.svg            │
│                                                         │
│  [Flame Graph Visualisierung]                          │
│    ████████████ processOrder (45ms)                    │
│    ████████ validatePayment (32ms)                     │
│    ████ createShipment (18ms)                         │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

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
