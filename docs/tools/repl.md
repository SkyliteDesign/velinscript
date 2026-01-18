# VelinScript REPL

Das REPL (Read-Eval-Print Loop) ist eine interaktive Shell zum Testen von VelinScript-Code in Echtzeit.

## Installation

Das Tool ist Teil der VelinScript Toolchain. Baue es mit:

```bash
cd tools/repl
cargo build --release
```

## Verwendung

### REPL starten

```bash
velin-repl
```

Startet die interaktive Shell.

### Datei laden

```bash
velin-repl --file main.velin
```

Lädt eine Datei in den REPL-Kontext.

## Features

### Einfache Ausdrücke

Einfache mathematische und logische Ausdrücke werden direkt interpretiert:

```velin
velin> 2 + 3
5

velin> sqrt(16)
4.0

velin> "Hello" + " " + "World"
Hello World
```

### Komplexe Code-Blöcke

Komplexe Code-Blöcke werden kompiliert und ausgeführt:

```velin
velin> fn add(a: number, b: number): number {
    return a + b;
}
Code kompiliert erfolgreich

velin> add(5, 3)
8
```

### Befehle

- `:help` oder `:h` - Zeigt Hilfe
- `:history` - Zeigt Command-History
- `:clear` - Löscht Bildschirm
- `exit` oder `quit` - Beendet REPL
- `CTRL-C` - Beendet REPL
- `CTRL-D` - Beendet REPL

### History

Der REPL speichert die Command-History automatisch. Nutze die Pfeiltasten zum Navigieren.

## Beispiel-Session

```
VelinScript REPL
Tippe 'exit' oder 'quit' zum Beenden

velin> 2 + 2
4

velin> let x = 10
Evaluiert: let x = 10

velin> x * 2
20

velin> :help
Verfügbare Befehle:
  :help, :h     - Zeigt diese Hilfe
  :history       - Zeigt Command-History
  :clear         - Löscht Bildschirm
  exit, quit     - Beendet REPL

velin> exit
Auf Wiedersehen!
```

## Integration

### VS Code Extension

Das REPL kann in VS Code Extensions integriert werden:

```typescript
import { exec } from 'child_process';

const repl = exec('velin-repl', (error, stdout) => {
  if (error) {
    console.error(error);
    return;
  }
  console.log(stdout);
});
```

## Best Practices

1. **Schnelle Tests** - Nutze REPL für schnelle Code-Tests
2. **Prototyping** - Teste Ideen interaktiv
3. **Debugging** - Debugge Code-Ausdrücke live

## Troubleshooting

### REPL startet nicht

- Prüfe, ob Rust installiert ist
- Prüfe, ob alle Dependencies installiert sind

### Code wird nicht ausgeführt

- Prüfe Syntax
- Nutze `:help` für verfügbare Befehle

## Weitere Ressourcen

- [Tools Übersicht](TOOLS_ÜBERSICHT.md)
- [Wann nutze ich was?](../wann-nutze-ich-was.md)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 0.1.0
