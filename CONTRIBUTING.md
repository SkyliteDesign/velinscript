# Contributing to VelinScript

Vielen Dank für dein Interesse an VelinScript! Wir freuen uns über Contributions.

**Aktuelle Version**: 2.5.0

**Neu in Version 2.5**: 
- 13 neue Standard Library Module (string, math, date, fs, llm, embedding, agent, process, sandbox, websocket, utils, log, config, flow)
- VelinAutoDoc für automatische Dokumentationsgenerierung
- VelinPipeline für automatische Performance-Optimierung
- VelinFlow Runtime für transaktionales Flow-Management

## Wie man beiträgt

### Bug Reports

1. Prüfe, ob der Bug bereits gemeldet wurde
2. Erstelle ein Issue mit:
   - Beschreibung des Problems
   - Schritte zur Reproduktion
   - Erwartetes Verhalten
   - Tatsächliches Verhalten
   - VelinScript Version
   - Beispiel-Code (falls möglich)

### Feature Requests

1. Prüfe, ob das Feature bereits vorgeschlagen wurde
2. Erstelle ein Issue mit:
   - Beschreibung des Features
   - Use Case
   - Vorschlag für Implementation

### Code Contributions

1. **Fork das Repository**
2. **Erstelle einen Branch**
   ```bash
   git checkout -b feature/mein-feature
   ```

3. **Mache deine Änderungen**
   - Folge dem Code-Style
   - Schreibe Tests
   - Aktualisiere Dokumentation

4. **Commits**
   - Verwende aussagekräftige Commit-Messages
   - Ein Commit pro logischer Änderung

5. **Push und Pull Request**
   ```bash
   git push origin feature/mein-feature
   ```

## Code-Style

### Rust Code

- Folgt Rust Standard-Style
- `cargo fmt` für Formatierung
- `cargo clippy` für Linting

### VelinScript Code

- 4 Spaces für Einrückung
- camelCase für Funktionen und Variablen
- PascalCase für Typen
- snake_case für Dateinamen

### Kommentare

- Kommentare auf Deutsch oder Englisch
- Dokumentiere öffentliche APIs
- Erkläre komplexe Logik

## Testing

### Unit Tests

```bash
cd compiler
cargo test
```

### Integration Tests

```bash
cargo test --test integration_test
```

### Beispiel-Tests

```bash
velin compile -i examples/hello.velin
velin check -i examples/hello.velin
```

## Pull Request Process

1. **Beschreibung**
   - Was wurde geändert?
   - Warum wurde es geändert?
   - Wie wurde es getestet?

2. **Checklist**
   - [ ] Code kompiliert
   - [ ] Tests bestehen
   - [ ] Dokumentation aktualisiert
   - [ ] Code-Style eingehalten

3. **Review**
   - Warte auf Review
   - Beantworte Fragen
   - Mache Änderungen falls nötig

## Development Setup

```bash
# Repository klonen
git clone https://github.com/SkyliteDesign/velinscript.git
cd velinscript

# Compiler bauen
cd compiler
cargo build

# Tests ausführen
cargo test

# Beispiel kompilieren
cargo run -- compile -i ../examples/hello.velin
```

## Projekt-Struktur

- `compiler/` - Compiler Implementation
- `docs/` - Dokumentation
- `examples/` - Beispiel-Projekte
- `tests/` - Tests

## Fragen?

- GitHub Issues für Fragen
- Diskussionen in Issues
- Code Reviews in Pull Requests

## Code of Conduct

Wir verpflichten uns, eine freundliche und respektvolle Community zu sein.

## License

Durch Contributions stimmst du zu, dass dein Code unter der MIT-Lizenz lizenziert wird.
