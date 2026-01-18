# VelinScript Code Formatter

Der VelinScript Code Formatter sorgt für konsistente Code-Formatierung in Ihren Projekten. Er ist direkt in den Compiler integriert und kann über die CLI oder die VS Code Extension verwendet werden.

## Installation

Der Formatter ist Teil des VelinScript Compilers. Keine separate Installation nötig.

## Verwendung

### CLI-Nutzung

#### Einzelne Datei formatieren

```bash
# Formatiert und gibt das Ergebnis in der Konsole aus
velin format -i main.velin

# Formatiert und überschreibt die Datei
velin format -i main.velin --in-place
```

#### Mehrere Dateien formatieren

```bash
# Mit Shell-Loop
for file in src/**/*.velin; do
    velin format -i "$file" --in-place
done
```

### VS Code Integration

Der Formatter ist automatisch in der VS Code Extension integriert:

- **Format Document**: `Shift+Alt+F` (Windows/Linux) oder `Shift+Option+F` (Mac)
- **Format on Save**: Automatisch aktiviert (konfigurierbar in Settings)
- **Format Selection**: Markieren Sie Code und drücken Sie `Ctrl+K Ctrl+F`

## Formatierungsregeln

### Einrückung

- **Standard**: 4 Leerzeichen
- **Einrückungsstil**: Spaces (keine Tabs)
- **Tab-Breite**: 4 Zeichen

### Zeilenbreite

- **Standard**: 100 Zeichen
- Längere Zeilen werden automatisch umgebrochen

### Leerzeichen

- **Operatoren**: Leerzeichen um binäre Operatoren (`+`, `-`, `*`, `/`, `==`, etc.)
- **Kommas**: Leerzeichen nach Kommas in Listen
- **Funktionsaufrufe**: Keine Leerzeichen zwischen Funktionsname und öffnender Klammer

### Zeilenumbrüche

- **Funktionen**: Leerzeile zwischen Funktionen
- **Structs/Enums**: Leerzeile zwischen Typ-Definitionen
- **Imports**: Gruppierte Imports mit Leerzeilen zwischen Gruppen

## Konfiguration

### velin.toml

Sie können die Formatierung in `velin.toml` anpassen:

```toml
[formatter]
# Einrückungsgröße (Standard: 4)
indent_size = 4

# Einrückungsstil: "spaces" oder "tabs"
indent_style = "spaces"

# Maximale Zeilenbreite (Standard: 100)
line_width = 100

# Tab-Breite (Standard: 4)
tab_width = 4
```

### VS Code Settings

```json
{
  "velin.formatter.enabled": true,
  "velin.formatter.formatOnSave": true,
  "velin.formatter.indentSize": 4,
  "velin.formatter.lineWidth": 100
}
```

## Beispiel

### Vor der Formatierung

```velin
@GET("/api/users")
fn getUsers():List<User>{
let users=db.findAll(User);
return users;
}

@POST("/api/users")
@Auth
fn createUser(name:string,email:string):User{
let user=User{id:generateId(),name:name,email:email,createdAt:datetime.now()};
return user;
}
```

### Nach der Formatierung

```velin
@GET("/api/users")
fn getUsers(): List<User> {
    let users = db.findAll(User);
    return users;
}

@POST("/api/users")
@Auth
fn createUser(name: string, email: string): User {
    let user = User {
        id: generateId(),
        name: name,
        email: email,
        createdAt: datetime.now(),
    };
    return user;
}
```

## Was wird formatiert?

Der Formatter normalisiert:

- ✅ **Einrückung**: Konsistente Einrückung mit Spaces
- ✅ **Leerzeichen**: Um Operatoren, nach Kommas, etc.
- ✅ **Zeilenumbrüche**: Zwischen Funktionen, Structs, etc.
- ✅ **Klammern**: Konsistente Platzierung von `{`, `}`, `(`, `)`, `[`, `]`
- ✅ **Semikolons**: Konsistente Platzierung
- ✅ **Imports**: Gruppierung und Sortierung von `use` Statements

## Integration in CI/CD

### Pre-Commit Hook

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Formatiere alle geänderten .velin Dateien
git diff --cached --name-only --diff-filter=ACM | grep '\.velin$' | while read file; do
    velin format -i "$file" --in-place
    git add "$file"
done
```

### GitHub Actions

```yaml
name: Format Check

on: [push, pull_request]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
      - name: Build Compiler
        run: |
          cd compiler
          cargo build --release
      - name: Check Formatting
        run: |
          for file in $(find . -name "*.velin" -not -path "./target/*"); do
            velin format -i "$file" > formatted.velin
            if ! diff -q "$file" formatted.velin > /dev/null; then
              echo "❌ $file ist nicht formatiert!"
              exit 1
            fi
          done
          echo "✅ Alle Dateien sind korrekt formatiert"
```

## Best Practices

1. **Format on Save aktivieren** - Automatische Formatierung bei jedem Speichern
2. **Pre-Commit Hooks** - Formatierung vor jedem Commit
3. **CI/CD Integration** - Format-Checks in der Pipeline
4. **Team-Konsens** - Einheitliche Formatierungsregeln im Team
5. **Regelmäßige Formatierung** - Vor größeren Commits formatieren

## Troubleshooting

### Formatter überschreibt meine Formatierung

- Der Formatter ist konsistent - wenn er Ihre Formatierung ändert, entspricht sie nicht den Standardregeln
- Passen Sie die Konfiguration in `velin.toml` an, wenn Sie andere Regeln möchten

### Formatierung funktioniert nicht in VS Code

- Prüfen Sie, ob die VS Code Extension installiert ist
- Prüfen Sie die VS Code Settings für `velin.formatter.enabled`
- Starten Sie VS Code neu

### Parsing-Fehler verhindern Formatierung

- Der Formatter benötigt gültigen Code
- Beheben Sie zuerst Parsing-Fehler mit `velin check` oder `--autofix`

## Erweiterte Nutzung

### Formatierung mit AutoFix kombinieren

```bash
# Zuerst AutoFix, dann Formatierung
velin check -i main.velin --autofix
velin format -i main.velin --in-place
```

### Formatierung für ganze Projekte

```bash
# Finde alle .velin Dateien und formatiere sie
find . -name "*.velin" -not -path "./target/*" -exec velin format -i {} --in-place \;
```

## Vergleich mit anderen Formatern

| Feature | VelinScript Formatter | Prettier | rustfmt |
|---------|----------------------|----------|---------|
| VelinScript-spezifisch | ✅ | ❌ | ❌ |
| Konfigurierbar | ✅ | ✅ | ✅ |
| Format on Save | ✅ | ✅ | ✅ |
| CLI-Integration | ✅ | ✅ | ✅ |
| VS Code Extension | ✅ | ✅ | ✅ |

## Weitere Ressourcen

- [VS Code Extension](vscode-extension.md) - IDE-Integration
- [Linter](linter.md) - Code-Qualitätsprüfung
- [AutoFix](auto-repair.md) - Automatische Fehlerkorrektur
