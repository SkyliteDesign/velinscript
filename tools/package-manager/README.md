# VelinScript Package Manager (velin-pkg)

Der VelinScript Package Manager verwaltet Dependencies und Packages für VelinScript-Projekte.

## Installation

```bash
cd tools/package-manager
cargo build --release
```

## Verwendung

### Projekt initialisieren

```bash
velin-pkg init [projektname]
```

Erstellt eine `velin.toml` Datei mit Projekt-Konfiguration.

### Dependency hinzufügen

```bash
velin-pkg add github.com/user/repo [--version ^1.0.0]
```

**Beispiel:**
```bash
velin-pkg add github.com/example/database-lib --version ^1.2.0
```

### Dependency entfernen

```bash
velin-pkg remove github.com/user/repo
```

### Dependencies installieren

```bash
velin-pkg install
```

Installiert alle Dependencies aus `velin.toml` in das `vendor/` Verzeichnis.

### Dependencies aktualisieren

```bash
# Prüfe auf verfügbare Updates
velin-pkg update

# Update alle Dependencies
velin-pkg update --all

# Update mit Breaking Changes erlauben
velin-pkg update --all --allow-breaking

# Update spezifisches Package
velin-pkg update github.com/user/repo
```

### Dependencies auflisten

```bash
velin-pkg list
```

Zeigt alle installierten Dependencies mit Versionen.

### Package veröffentlichen

```bash
velin-pkg publish 1.0.0
```

### Dependencies auditieren

```bash
velin-pkg audit
```

Prüft Dependencies auf bekannte Vulnerabilities.

## velin.toml Format

```toml
[package]
name = "my-project"
version = "0.1.0"

[dependencies]
github.com/example/database = "^1.0.0"
github.com/example/auth = "~1.2.0"
```

## velin.lock

Die `velin.lock` Datei speichert exakte Versionen für reproduzierbare Builds. Sie wird automatisch generiert bei `velin-pkg install`.

## Best Practices

1. **Version Constraints** - Verwende SemVer Constraints (`^`, `~`)
2. **Lock File committen** - Committe `velin.lock` für reproduzierbare Builds
3. **Regelmäßige Updates** - Prüfe regelmäßig auf Updates mit `velin-pkg update`
4. **Breaking Changes** - Teste Breaking Changes gründlich vor dem Update
