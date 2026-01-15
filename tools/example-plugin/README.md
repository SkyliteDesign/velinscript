# VelinScript Example Plugin

Ein Beispiel-Plugin für VelinScript, das Code-Metriken analysiert.

## Features

- Zählt Funktionen, Structs und Enums
- Berechnet durchschnittliche Funktion-Länge
- Generiert JSON- oder Text-Reports

## Installation

```bash
cd tools/example-plugin
cargo build --release
```

## Verwendung

```bash
# Analysiere ein Verzeichnis
./target/release/velin-example-plugin metrics -i examples/

# JSON-Output
./target/release/velin-example-plugin metrics -i examples/ --format json

# Verbose Output
./target/release/velin-example-plugin metrics -i examples/ --verbose
```

## Beispiel-Output

```
📊 Code-Metriken für examples/:
  Funktionen: 45
  Structs: 12
  Enums: 3
  Durchschnittliche Funktion-Länge: 15 Zeilen
  Gesamt-Zeilen: 1200
```
