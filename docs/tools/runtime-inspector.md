# VelinScript Runtime Inspector

Der Runtime Inspector ermöglicht Live-Inspection von Variablen, State und Memory während der Ausführung.

## Wofür ist der Runtime Inspector ideal?

Der Runtime Inspector ist ideal für:
- ✅ **Live-Debugging** - Inspiziert Variablen während der Ausführung ohne Breakpoints
- ✅ **Memory-Monitoring** - Überwacht Memory-Usage in Echtzeit
- ✅ **State-Analyse** - Analysiert Programm-State zur Laufzeit
- ✅ **Watch-Mode** - Kontinuierliche Überwachung von Variablen und Memory
- ✅ **Production-Debugging** - Hilft bei Problemen in laufenden Systemen
- ✅ **Performance-Investigation** - Kombiniert mit Profiler für umfassende Analyse

## Wofür ist der Runtime Inspector NICHT gedacht?

Der Runtime Inspector ist NICHT gedacht für:
- ❌ **Statische Code-Analyse** - Für Code-Qualität nutzen Sie den Linter
- ❌ **Strukturierte Tests** - Für Tests nutzen Sie den Test Runner
- ❌ **Performance-Benchmarks** - Für Benchmarks nutzen Sie den Benchmark Runner
- ❌ **CPU-Profiling** - Für CPU-Analyse nutzen Sie den Profiler
- ❌ **Code-Generierung** - Für Boilerplate nutzen Sie Code Generation

## Installation

Das Tool ist Teil der VelinScript Toolchain. Baue es mit:

```bash
cd tools/runtime-inspector
cargo build --release
```

## Verwendung

### Code inspizieren

```bash
velin-inspect inspect main.velin
```

### Variablen anzeigen

```bash
velin-inspect inspect main.velin --variables
```

### Memory-Usage anzeigen

```bash
velin-inspect inspect main.velin --memory
```

### Watch-Mode

```bash
velin-inspect inspect main.velin --watch
```

Überwacht Code kontinuierlich (CTRL-C zum Beenden).

## Features

### Variable-Inspection

Zeigt alle Variablen im Code:

```
📊 Variablen:
  x: 10
  y: 20
  result: 30
```

### Memory-Monitoring

Zeigt Memory-Usage:

```
💾 Memory-Usage:
  Geschätzte Allokationen: 2048 bytes
  Variablen: 5
```

### Watch-Mode

Kontinuierliche Überwachung:

```
🔍 Runtime Inspector für: main.velin

👀 Watch-Mode aktiviert (CTRL-C zum Beenden)

📊 Variablen:
  x: 10
  y: 20
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Runtime Inspector                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-inspect inspect main.velin --watch            │
│                                                         │
│  🔍 Runtime Inspector für: main.velin                  │
│                                                         │
│  👀 Watch-Mode aktiviert (CTRL-C zum Beenden)           │
│                                                         │
│  📊 Variablen:                                         │
│    x: 10                                               │
│    y: 20                                               │
│    result: 30                                          │
│                                                         │
│  💾 Memory-Usage:                                      │
│    Geschätzte Allokationen: 2048 bytes                 │
│    Variablen: 5                                        │
│    Funktionen: 3                                       │
│                                                         │
│  [Aktualisiert sich alle 2 Sekunden...]                │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Integration

### Debugger-Integration

Der Runtime Inspector integriert sich mit dem Debugger:

```bash
# Starte Debugger
velin-debug main.velin

# In separatem Terminal: Inspector
velin-inspect inspect main.velin --watch
```

## Best Practices

1. **Watch-Mode** - Nutze Watch-Mode für Live-Debugging
2. **Variable-Tracking** - Überwache kritische Variablen
3. **Memory-Monitoring** - Prüfe Memory-Usage regelmäßig

## Troubleshooting

### Inspector findet keine Variablen

- Prüfe Code-Syntax
- Prüfe, ob Variablen korrekt deklariert sind

### Watch-Mode funktioniert nicht

- Prüfe Datei-Berechtigungen
- Prüfe, ob Datei existiert

## Weitere Ressourcen

- [Tools Übersicht](TOOLS_ÜBERSICHT.md)
- [Wann nutze ich was?](../wann-nutze-ich-was.md)
- [Debugger](debugger.md)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 0.1.0
