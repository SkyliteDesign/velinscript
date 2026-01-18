# VelinScript Package Manager (velin-pkg)

Der VelinScript Package Manager verwaltet Dependencies und Packages für VelinScript-Projekte.

## Wofür ist der Package Manager ideal?

Der Package Manager ist ideal für:
- ✅ **Dependency-Management** - Verwaltet Projekt-Abhängigkeiten
- ✅ **Version-Resolution** - Löst Dependency-Konflikte automatisch
- ✅ **Security-Audits** - Prüft Dependencies auf Vulnerabilities
- ✅ **Workspace-Management** - Verwaltet Multi-Package-Projekte
- ✅ **Lock-Files** - Stellt reproduzierbare Builds sicher
- ✅ **SemVer-Support** - Unterstützt semantische Versionierung

## Wofür ist der Package Manager NICHT gedacht?

Der Package Manager ist NICHT gedacht für:
- ❌ **Code-Qualität** - Für Code-Qualität nutzen Sie den Linter
- ❌ **Dependency-Visualisierung** - Für Dependency-Graphen nutzen Sie den Dependency Graph
- ❌ **Bundle-Analyse** - Für Bundle-Größen nutzen Sie den Bundle Analyzer
- ❌ **Performance-Analyse** - Für Performance nutzen Sie den Profiler
- ❌ **Code-Generierung** - Für Code-Generierung nutzen Sie `velin generate`

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

**Beispiel:**
```bash
velin-pkg init my-awesome-project
```

### Dependency hinzufügen

```bash
velin-pkg add <source> [--version <version>]
```

**Quellen:**
- GitHub: `github.com/user/repo`
- GitLab: `gitlab.com/user/repo`
- Local: `file:./path/to/package`

**Beispiele:**
```bash
# GitHub Repository
velin-pkg add github.com/example/database-lib --version ^1.2.0

# GitLab Repository
velin-pkg add gitlab.com/example/auth-lib

# Local Package
velin-pkg add file:./local-package
```

**Version Constraints:**
- `^1.2.0` - Compatible Version (>=1.2.0, <2.0.0)
- `~1.2.0` - Patch Version (>=1.2.0, <1.3.0)
- `1.2.0` - Exact Version
- `>=1.2.0` - Minimum Version
- `*` - Latest Version

### Dependency entfernen

```bash
velin-pkg remove <source>
```

**Beispiel:**
```bash
velin-pkg remove github.com/example/database-lib
```

### Dependencies installieren

```bash
velin-pkg install
```

Installiert alle Dependencies aus `velin.toml` in das `vendor/` Verzeichnis.

**Optionen:**
- `--no-lock` - Generiert keine `velin.lock` Datei
- `--offline` - Verwendet nur lokale Caches
- `--update` - Aktualisiert vorhandene Dependencies

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

**Optionen:**
- `--tree` - Zeigt Dependency-Baum
- `--outdated` - Zeigt nur veraltete Dependencies
- `--json` - JSON-Output

### Package veröffentlichen

```bash
velin-pkg publish <version>
```

**Beispiel:**
```bash
velin-pkg publish 1.0.0
```

**Voraussetzungen:**
- `velin.toml` muss korrekt konfiguriert sein
- Git Repository muss vorhanden sein
- Version muss in `velin.toml` übereinstimmen

### Dependencies auditieren

```bash
velin-pkg audit
```

## Screenshot

```
┌─────────────────────────────────────────────────────────┐
│  VelinScript Package Manager                           │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  $ velin-pkg add github.com/user/repo --version ^1.0.0 │
│                                                         │
│  📦 Füge Dependency hinzu...                          │
│  ✓ Dependency hinzugefügt: github.com/user/repo@1.0.0 │
│  ✓ velin.toml aktualisiert                             │
│                                                         │
│  $ velin-pkg install                                    │
│                                                         │
│  📥 Installiere Dependencies...                        │
│  ✓ 5 Dependencies installiert                          │
│  ✓ velin.lock generiert                                │
│                                                         │
│  $ velin-pkg list                                       │
│                                                         │
│  📋 Installierte Dependencies:                         │
│    github.com/user/repo      v1.0.0                    │
│    github.com/user/auth      v2.1.0                    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

Prüft Dependencies auf bekannte Vulnerabilities.

**Optionen:**
- `--json` - JSON-Output
- `--fix` - Versucht automatisch zu fixen

### Cache verwalten

```bash
# Cache anzeigen
velin-pkg cache list

# Cache löschen
velin-pkg cache clean

# Cache-Info
velin-pkg cache info
```

## velin.toml Format

```toml
[package]
name = "my-project"
version = "0.1.0"
description = "Mein tolles Projekt"
authors = ["Author <author@example.com>"]
license = "MIT"
repository = "https://github.com/user/repo"

[dependencies]
github.com/example/database = "^1.0.0"
github.com/example/auth = "~1.2.0"
github.com/example/utils = "1.5.0"

[dev-dependencies]
github.com/example/test-utils = "^0.1.0"

[features]
default = []
axum = []
sea-orm = []
```

### Package-Metadaten

- **name** - Package-Name (erforderlich)
- **version** - Version im SemVer-Format (erforderlich)
- **description** - Kurze Beschreibung
- **authors** - Liste der Autoren
- **license** - Lizenz (MIT, Apache-2.0, etc.)
- **repository** - Git Repository URL
- **homepage** - Homepage URL
- **documentation** - Dokumentations-URL
- **keywords** - Keywords für Package-Suche

### Dependencies

```toml
[dependencies]
# GitHub Repository
github.com/user/repo = "^1.0.0"

# GitLab Repository
gitlab.com/user/repo = "~1.2.0"

# Local Package
local-package = { path = "./local-package" }

# Mit Features
featured-package = { version = "^1.0.0", features = ["feature1"] }

# Optional Dependency
optional-package = { version = "^1.0.0", optional = true }
```

### Dev-Dependencies

Dependencies nur für Development/Testing:

```toml
[dev-dependencies]
github.com/example/test-utils = "^0.1.0"
```

### Features

```toml
[features]
default = ["feature1"]
feature1 = []
feature2 = ["feature1"]
```

## velin.lock

Die `velin.lock` Datei speichert exakte Versionen für reproduzierbare Builds.

**Wichtig:**
- Wird automatisch generiert bei `velin-pkg install`
- Sollte in Git committed werden
- Stellt sicher, dass alle Entwickler gleiche Versionen verwenden

## Workspaces

Für Multi-Package-Projekte:

```toml
[workspace]
members = [
    "package1",
    "package2",
    "package3"
]
```

## Best Practices

1. **Version Constraints** - Verwende SemVer Constraints (`^`, `~`)
2. **Lock File committen** - Committe `velin.lock` für reproduzierbare Builds
3. **Regelmäßige Updates** - Prüfe regelmäßig auf Updates mit `velin-pkg update`
4. **Breaking Changes** - Teste Breaking Changes gründlich vor dem Update
5. **Security Audits** - Führe regelmäßig `velin-pkg audit` aus
6. **Minimale Dependencies** - Füge nur notwendige Dependencies hinzu
7. **Version Pinning** - Pin kritische Dependencies auf exakte Versionen

## Integration in CI/CD

```yaml
# .github/workflows/ci.yml
- name: Install Dependencies
  run: |
    cd tools/package-manager
    cargo build --release
    ./target/release/velin-pkg install

- name: Audit Dependencies
  run: |
    ./target/release/velin-pkg audit
```

## Troubleshooting

### Dependency nicht gefunden

- Prüfe Repository-URL
- Prüfe Internet-Verbindung
- Prüfe Zugriffsrechte (private Repositories)

### Version-Konflikte

```bash
# Zeige Dependency-Baum
velin-pkg list --tree

# Update alle Dependencies
velin-pkg update --all
```

### Lock File Konflikte

```bash
# Regeneriere Lock File
rm velin.lock
velin-pkg install
```

### Cache-Probleme

```bash
# Lösche Cache
velin-pkg cache clean

# Installiere neu
velin-pkg install
```

## Package-Entwicklung

### Lokales Package testen

```bash
# In Package-Verzeichnis
velin-pkg publish --dry-run

# In Consumer-Projekt
velin-pkg add file:./path/to/package
```

### Package-Struktur

```
my-package/
├── velin.toml
├── src/
│   ├── main.velin
│   └── lib.velin
├── README.md
└── LICENSE
```

## Vergleich mit anderen Package Managern

| Feature | velin-pkg | npm | cargo |
|---------|----------|-----|-------|
| Version Constraints | ✅ | ✅ | ✅ |
| Lock File | ✅ | ✅ | ✅ |
| Workspaces | ✅ | ✅ | ✅ |
| Git Dependencies | ✅ | ✅ | ✅ |
| Local Dependencies | ✅ | ✅ | ✅ |
| Dev Dependencies | ✅ | ✅ | ✅ |
| Features | ✅ | ❌ | ✅ |
