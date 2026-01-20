# VelinScript Bibliotheks-Generator

**Version:** 1.0.0  
**Status:** ✅ Vollständig implementiert  
**Stand:** 2026-01-30

## Übersicht

Der Bibliotheks-Generator ist ein CLI-Tool zur automatischen Generierung neuer Standardbibliotheks-Module für VelinScript. Es erstellt vollständige Module mit Integration in alle System-Komponenten.

## Wofür ist der Bibliotheks-Generator gedacht?

Der Bibliotheks-Generator ist ideal für:
- ✅ **Neue Standardbibliotheks-Module erstellen** - Automatische Generierung von vollständigen Modulen
- ✅ **Konsistente Code-Struktur** - Einheitliche Module-Struktur für alle Standardbibliotheks-Module
- ✅ **Vollständige Integration** - Automatische Integration in Type Checker, Code Generator und Tests
- ✅ **Zeitersparnis** - Reduziert manuellen Boilerplate-Code erheblich
- ✅ **Dokumentations-Generierung** - Automatische Erstellung von API-Dokumentation
- ✅ **Test-Generierung** - Automatische Erstellung von Unit-Tests
- ✅ **Template-basiert** - Unterstützt verschiedene Modul-Typen (Utility, Service, Data Structure)

## Wofür ist der Bibliotheks-Generator NICHT gedacht?

Der Bibliotheks-Generator ist NICHT gedacht für:
- ❌ **Anwendungs-Code** - Für Anwendungs-Code nutzen Sie normale VelinScript-Dateien
- ❌ **Externe Bibliotheken** - Für externe Bibliotheken nutzen Sie den Package Manager
- ❌ **Einmalige Skripte** - Für einmalige Skripte ist der Generator zu komplex
- ❌ **Sehr spezielle Module** - Bei sehr speziellen Anforderungen ist manuelle Implementierung besser

## Installation

```bash
cd tools/library-generator
cargo build --release
```

Das Binary befindet sich in `target/release/velin-library-generator`.

## Verwendung

### Interaktiver Modus

```bash
cargo run -- generate --interactive
```

### Mit YAML-Konfiguration

```bash
cargo run -- generate --config my-library.yaml
```

### Validierung

```bash
cargo run -- validate --config my-library.yaml
```

## Generierte Komponenten

1. **Modul-Datei** (`compiler/src/stdlib/{name}.rs`)
2. **mod.rs Integration** (automatisch)
3. **Type Checker Integration** (automatisch)
4. **Code Generator Integration** (automatisch)
5. **Unit Tests** (`compiler/tests/{name}_test.rs`)
6. **Dokumentation** (`docs/api/{name}.md`)

## Vollständige Dokumentation

Siehe [tools/library-generator/README.md](../../tools/library-generator/README.md) für vollständige Dokumentation.

## Status

✅ **Vollständig implementiert:**
- CLI-Tool mit allen Features
- Template-System mit 3 Modul-Typen
- Vollständige Integration in alle Komponenten
- Test-Generierung
- Dokumentations-Generierung
- Validierung
- Interaktiver Modus
- YAML-Konfiguration

## Siehe auch

- [Bibliotheks-Generator Plan](../../bauplan/BIBLIOTHEKS_GENERATOR_PLAN.md)
- [Standardbibliothek Übersicht](../../bauplan/STANDARDBIBLIOTHEK_ÜBERSICHT.md)
