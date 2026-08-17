# VelinScript Benchmark Runner

Der Benchmark Runner führt Performance-Benchmarks aus mit statistischer Auswertung.

## Wofür ist der Benchmark Runner ideal?

Der Benchmark Runner ist ideal für:
- ✅ **Performance-Messung** - Misst Ausführungszeiten mit statistischer Genauigkeit
- ✅ **Regression-Erkennung** - Erkennt Performance-Verschlechterungen über Zeit
- ✅ **Implementierungs-Vergleich** - Vergleicht verschiedene Algorithmen oder Implementierungen
- ✅ **CI/CD-Integration** - Kann Performance-Regressionen in Pipelines erkennen
- ✅ **Release-Qualitätssicherung** - Prüft Performance vor Releases
- ✅ **Statistische Analyse** - Bietet Mittelwert, Min, Max, Standardabweichung

## Wofür ist der Benchmark Runner NICHT gedacht?

Der Benchmark Runner ist NICHT gedacht für:
- ❌ **CPU-Profiling** - Für detaillierte CPU-Analyse nutzen Sie den Profiler
- ❌ **Memory-Profiling** - Für Memory-Analyse nutzen Sie den Profiler
- ❌ **Unit-Tests** - Für Funktionalitätstests nutzen Sie den Test Runner
- ❌ **Live-Debugging** - Für Runtime-Inspection nutzen Sie den Runtime Inspector
- ❌ **Code-Qualität** - Für Code-Qualitätsprüfung nutzen Sie den Linter

## Installation

Das Tool ist Teil der VelinScript Toolchain. Baue es mit:

```bash
cd tools/benchmark-runner
cargo build --release
```

## Verwendung

### Benchmarks ausführen

```bash
velin-bench run
```

Führt alle Benchmarks im aktuellen Verzeichnis aus.

### Spezifische Datei benchmarken

```bash
velin-bench run path/to/benchmark.velin
```

### Anzahl Iterationen

```bash
velin-bench run --iterations 1000
```

### Mit Vergleich

```bash
velin-bench run --compare
```

Vergleicht Ergebnisse mit vorherigen Runs.

### Output speichern

```bash
velin-bench run --output results.json
```

### Verbose Output

```bash
velin-bench run --verbose
```

## Features

### @benchmark Annotationen

Benchmarks werden mit `@benchmark` Decorator markiert:

```velin
@benchmark
fn benchmarkSort() {
    let data = generateLargeArray(10000);
    sort(data);
}

@benchmark
fn benchmarkSearch() {
    let data = generateLargeArray(10000);
    search(data, 5000);
}
```

### Statistische Auswertung

Der Benchmark Runner berechnet:

- **Mittelwert** - Durchschnittliche Ausführungszeit
- **Minimum** - Schnellste Ausführung
- **Maximum** - Langsamste Ausführung
- **Standardabweichung** - Streuung der Ergebnisse

## Beispiel-Output

```
⚡ Führe Benchmarks aus...

🔍 Benchmarke: benchmarks/sort.velin

📊 Benchmark-Ergebnisse:
  benchmarkSort: 12.45ms (100 Iterationen)
    Min: 11.23ms, Max: 14.67ms, StdDev: 0.89ms
  benchmarkSearch: 8.32ms (100 Iterationen)
    Min: 7.91ms, Max: 9.12ms, StdDev: 0.34ms
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Benchmark Runner                          │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-bench run --iterations 100                    │
│                                                         │
│  ⚡ Führe Benchmarks aus...                            │
│                                                         │
│  🔍 Benchmarke: benchmarks/sort.velin                  │
│                                                         │
│  📊 Benchmark-Ergebnisse:                              │
│    benchmarkSort: 12.45ms (100 Iterationen)            │
│      Min: 11.23ms, Max: 14.67ms, StdDev: 0.89ms        │
│    benchmarkSearch: 8.32ms (100 Iterationen)          │
│      Min: 7.91ms, Max: 9.12ms, StdDev: 0.34ms          │
│                                                         │
│  ✓ Benchmarks erfolgreich abgeschlossen                │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## JSON-Format

```json
[
  {
    "name": "benchmarkSort",
    "mean_time": 12.45,
    "min_time": 11.23,
    "max_time": 14.67,
    "std_dev": 0.89,
    "iterations": 100
  }
]
```

## Integration

### CI/CD

```yaml
# .github/workflows/benchmark.yml
- name: Run Benchmarks
  run: |
    cd tools/benchmark-runner
    cargo build --release
    ./target/release/velin-bench run --output benchmark.json
```

## Best Practices

1. **Ausreichend Iterationen** - Nutze mindestens 100 Iterationen
2. **Warme Läufe** - Ignoriere erste Iterationen (Warmup)
3. **Konsistente Umgebung** - Führe Benchmarks unter gleichen Bedingungen aus
4. **Regelmäßige Benchmarks** - Überwache Performance im Zeitverlauf

## Troubleshooting

### Benchmarks sind zu langsam

- Reduziere Anzahl Iterationen
- Prüfe System-Last
- Nutze Release-Builds

### Ungenaue Ergebnisse

- Erhöhe Anzahl Iterationen
- Führe Benchmarks mehrfach aus
- Prüfe auf Hintergrund-Prozesse

## Weitere Ressourcen

- [Tools Übersicht](TOOLS_ÜBERSICHT.md)
- [Wann nutze ich was?](../wann-nutze-ich-was.md)
- [Profiler](profiler.md)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 0.1.0
