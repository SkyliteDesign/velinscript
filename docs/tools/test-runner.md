# VelinScript Test Runner

Der Test Runner führt Unit- und Integrationstests aus, unterstützt Assertions, Mocking und Coverage-Reports.

## Wofür ist der Test Runner ideal?

Der Test Runner ist ideal für:
- ✅ **Automatisierte Test-Ausführung** - Führt alle Tests mit einem Befehl aus
- ✅ **CI/CD-Integration** - Perfekt für kontinuierliche Integration
- ✅ **Coverage-Messung** - Misst Code-Coverage für Qualitätssicherung
- ✅ **Test-Organisation** - Unterstützt Unit- und Integration-Tests getrennt
- ✅ **Mocking** - Ermöglicht isolierte Tests mit Mock-Objekten
- ✅ **Regressions-Tests** - Verhindert, dass bestehende Features brechen

## Wofür ist der Test Runner NICHT gedacht?

Der Test Runner ist NICHT gedacht für:
- ❌ **Manuelle Tests** - Für interaktive, manuelle Tests nutzen Sie den Debugger oder REPL
- ❌ **Performance-Tests** - Für Performance-Messungen nutzen Sie den Benchmark Runner
- ❌ **Security-Tests** - Für Security-Checks nutzen Sie den Security Scanner
- ❌ **Code-Analyse** - Für statische Code-Analyse nutzen Sie den Linter
- ❌ **Live-Debugging** - Für Runtime-Inspection nutzen Sie den Runtime Inspector

## Installation

Das Tool ist Teil der VelinScript Toolchain. Baue es mit:

```bash
cd tools/test-runner
cargo build --release
```

## Verwendung

### Alle Tests ausführen

```bash
velin-test run
```

Führt alle Tests im aktuellen Verzeichnis aus.

### Spezifische Datei testen

```bash
velin-test run path/to/test.velin
```

### Nur Unit-Tests

```bash
velin-test run --unit
```

### Nur Integration-Tests

```bash
velin-test run --integration
```

### Mit Coverage-Report

```bash
velin-test run --coverage
```

Generiert einen Coverage-Report mit Zeilen- und Funktions-Coverage.

### Mit Mocking

```bash
velin-test run --mock
```

Aktiviert Mocking-Framework für Tests.

### Verbose Output

```bash
velin-test run --verbose
```

Zeigt detaillierte Informationen über jeden Test.

## Features

### @test Annotationen

Tests werden mit `@test` Decorator markiert:

```velin
@test
fn testAdd() {
    let result = add(2, 3);
    assert(result == 5);
}

@test
fn testUserCreation() {
    let user = createUser("John", "john@example.com");
    assert(user.name == "John");
}
```

### @before und @after

Setup und Teardown mit `@before` und `@after`:

```velin
@before
fn setup() {
    db.connect();
}

@after
fn teardown() {
    db.disconnect();
}

@test
fn testQuery() {
    let result = db.query("SELECT * FROM users");
    assert(result.len() > 0);
}
```

### Assertions

Unterstützte Assertions:

- `assert(condition)` - Prüft Bedingung
- `assert_eq(a, b)` - Prüft Gleichheit
- `assert_ne(a, b)` - Prüft Ungleichheit
- `assert_true(value)` - Prüft auf true
- `assert_false(value)` - Prüft auf false

### Rust-Tests

Der Test Runner führt auch bestehende Rust-Tests aus:

```bash
velin-test run
# Führt sowohl VelinScript- als auch Rust-Tests aus
```

### Coverage-Reports

Mit `--coverage` werden Coverage-Reports generiert:

```
📈 Coverage-Report:
  Zeilen-Coverage: 87.5%
  Funktionen-Coverage: 92.3%
```

## Beispiel-Output

```
🧪 Führe Tests aus...

🔍 Teste: tests/unit/main_test.velin
  ✓ testAdd
  ✓ testUserCreation

📊 Test-Ergebnisse:
  ✓ Bestanden: 2
  ✗ Fehlgeschlagen: 0
  ⏭️  Übersprungen: 0

📈 Coverage-Report:
  Zeilen-Coverage: 87.5%
  Funktionen-Coverage: 92.3%
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Test Runner                                │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-test run --coverage                           │
│                                                         │
│  🧪 Führe Tests aus...                                 │
│                                                         │
│  🔍 Teste: tests/unit/main_test.velin                  │
│    ✓ testAdd                                           │
│    ✓ testUserCreation                                  │
│                                                         │
│  📊 Test-Ergebnisse:                                    │
│    ✓ Bestanden: 2                                      │
│    ✗ Fehlgeschlagen: 0                                 │
│    ⏭️  Übersprungen: 0                                 │
│                                                         │
│  📈 Coverage-Report:                                   │
│    Zeilen-Coverage: 87.5%                              │
│    Funktionen-Coverage: 92.3%                           │
│                                                         │
│  ✓ Alle Tests bestanden!                                │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Integration

### CI/CD

```yaml
# .github/workflows/test.yml
- name: Run Tests
  run: |
    cd tools/test-runner
    cargo build --release
    ./target/release/velin-test run --coverage
```

### VS Code Extension

Das Tool kann in VS Code Extensions integriert werden:

```typescript
import { exec } from 'child_process';

exec('velin-test run --json', (error, stdout) => {
  if (error) {
    console.error(error);
    return;
  }
  const results = JSON.parse(stdout);
  // Zeige Test-Ergebnisse
});
```

## Best Practices

1. **Test-Organisation** - Organisiere Tests in `tests/unit/` und `tests/integration/`
2. **Isolierte Tests** - Jeder Test sollte unabhängig sein
3. **Mocking** - Nutze Mocking für externe Dependencies
4. **Coverage** - Strebe nach hoher Test-Coverage (>80%)

## Troubleshooting

### Tests werden nicht gefunden

- Prüfe, ob Tests im `tests/` Verzeichnis sind
- Prüfe, ob Tests mit `@test` markiert sind
- Prüfe Datei-Pfade und Berechtigungen

### Assertions schlagen fehl

- Prüfe Assertion-Syntax
- Prüfe, ob Werte korrekt sind
- Nutze `--verbose` für detaillierte Fehlermeldungen

## Weitere Ressourcen

- [Tools Übersicht](TOOLS_ÜBERSICHT.md)
- [Wann nutze ich was?](../wann-nutze-ich-was.md)
- [Testing Guide](../../docs/guides/testing.md)

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 0.1.0
