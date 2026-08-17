# VelinScript Bibliotheks-Generator

Ein vollständiges Tool zur automatischen Generierung neuer Standardbibliotheks-Module für VelinScript.

## 🎯 Übersicht

Der Bibliotheks-Generator erstellt automatisch:
- ✅ Modul-Dateien (`compiler/src/stdlib/{name}.rs`)
- ✅ Integration in `mod.rs`
- ✅ Type Checker Integration
- ✅ Code Generator Integration
- ✅ Unit Tests
- ✅ Vollständige Dokumentation

## 📦 Installation

```bash
cd tools/library-generator
cargo build --release
```

Das Binary wird in `target/release/velin-library-generator` erstellt.

## 🚀 Verwendung

### 1. Interaktiver Modus

```bash
cargo run -- generate --interactive
```

Der interaktive Modus führt Sie durch alle Schritte:
- Modul-Name
- Beschreibung
- Kategorie
- Funktionen (mit Parametern und Rückgabetypen)
- Typen (optional)

### 2. Mit Konfigurationsdatei (YAML)

Erstellen Sie eine YAML-Datei:

```yaml
# slug-library.yaml
name: slug
description: "URL-Slug-Generierung für SEO-freundliche URLs"
category: string_manipulation

functions:
  - name: generate
    description: "Generiert einen URL-Slug aus einem Text"
    params:
      - name: text
        type: string
        description: "Der zu konvertierende Text"
    return_type: string
    example: |
      let slug = slug.generate("Hello World");
      // Returns: "hello-world"
```

Dann ausführen:

```bash
cargo run -- generate --config slug-library.yaml
```

### 3. Direkt mit Parametern

```bash
cargo run -- generate \
  --name slug \
  --description "URL-Slug-Generierung"
```

### 4. Validierung

```bash
cargo run -- validate --config slug-library.yaml
```

## 📋 Konfigurationsformat

### Vollständiges Beispiel

```yaml
name: graphql
description: "GraphQL Client und Server Funktionen"
category: api

functions:
  - name: query
    description: "Führt eine GraphQL Query aus"
    params:
      - name: query_string
        type: string
        description: "Die GraphQL Query"
      - name: variables
        type: Map<string, any>
        optional: true
        description: "Variablen für die Query"
    return_type: GraphQLResponse
    example: |
      let result = graphql.query("{ users { id name } }", {});
      
  - name: mutation
    description: "Führt eine GraphQL Mutation aus"
    params:
      - name: mutation_string
        type: string
      - name: variables
        type: Map<string, any>
    return_type: GraphQLResponse

types:
  - name: GraphQLQuery
    description: "Eine GraphQL Query"
    fields:
      - name: query
        type: string
        description: "Die Query-Zeichenkette"
      - name: variables
        type: Map<string, any>
        optional: true
        description: "Variablen"

dependencies:
  - graphql_client
  - serde_json

features:
  - graphql
```

## 🔧 Unterstützte Typen

### VelinScript Typen → Rust Typen

- `string` → `String`
- `number` → `f64`
- `boolean` → `bool`
- `List<string>` → `Vec<String>`
- `Map<string, any>` → `HashMap<String, String>`
- `any` → `String` (vereinfacht)
- Benutzerdefinierte Typen → `Type::Named(...)`

## 📁 Generierte Dateien

Nach der Generierung finden Sie:

1. **Modul-Datei**: `compiler/src/stdlib/{name}.rs`
   - Vollständige Rust-Implementierung
   - Alle Funktionen mit Code-Generierung

2. **Integration in mod.rs**: Automatisch hinzugefügt
   - `pub mod {name};`

3. **Type Checker Integration**: `compiler/src/type_checker/checker.rs`
   - Typ-Definitionen
   - Variable-Definitionen
   - Funktions-Signaturen

4. **Code Generator Integration**: `compiler/src/codegen/rust.rs`
   - Dispatch-Logik
   - `generate_{name}_call` Funktion
   - Parameter-Handling

5. **Tests**: `compiler/tests/{name}_test.rs`
   - Unit Tests für alle Funktionen
   - Basis-Validierungen

6. **Dokumentation**: `docs/api/{name}.md`
   - Vollständige API-Dokumentation
   - Beispiele für alle Funktionen
   - Typ-Dokumentation

## 🧪 Tests

```bash
# Alle Tests ausführen
cargo test

# Nur Library-Tests
cargo test --lib

# Mit Ausgabe
cargo test -- --nocapture
```

## 📚 Dokumentation

Die generierte Dokumentation wird automatisch in `docs/api/` erstellt und folgt dem Standard-Format der VelinScript-Dokumentation.

## ⚙️ Erweiterte Features

### Modul-Typen

Der Generator unterstützt drei Modul-Typen:

1. **Simple Functions**: Einfache Funktionen ohne Structs
   - Beispiel: `string`, `math`, `date`

2. **Struct Based**: Module mit benutzerdefinierten Typen
   - Beispiel: `http`, `database`

3. **Service Based**: Service-basierte Module mit State
   - Beispiel: `auth`, `llm`, `agent`

Der Typ wird automatisch erkannt basierend auf:
- Vorhandensein von Typen → Struct Based
- Service/Client in Funktionsnamen → Service Based
- Sonst → Simple Functions

## 🐛 Fehlerbehebung

### "Tool muss vom Projekt-Root ausgeführt werden"
- **Problem**: Das Tool wurde nicht vom Projekt-Root-Verzeichnis aus gestartet
- **Lösung**: Wechseln Sie ins Projekt-Root-Verzeichnis:
  ```bash
  cd /path/to/velinscript
  velin-library-generator generate --config my-library.yaml
  ```
- **Hinweis**: Das Tool prüft automatisch, ob `compiler/src/stdlib/mod.rs` existiert

### "Modul bereits vorhanden"
- **Problem**: Das Modul existiert bereits
- **Lösung**: 
  - Entfernen Sie das Modul manuell: `compiler/src/stdlib/{name}.rs`
  - Oder verwenden Sie einen anderen Modul-Namen
- **Hinweis**: Das Tool prüft automatisch, ob das Modul bereits existiert

### "Einfügepunkt nicht gefunden"
- **Problem**: Die Marker in den Dateien könnten sich geändert haben
- **Lösung**: Prüfen Sie die Dateien manuell:
  - `compiler/src/type_checker/checker.rs` sollte `// --- Extended Standard Library Variables ---` enthalten
  - `compiler/src/codegen/rust.rs` sollte `} else if obj_name == "env" {` enthalten

### "Modul hat keine Funktionen definiert"
- **Problem**: Die YAML-Konfiguration enthält keine Funktionen
- **Lösung**: Fügen Sie mindestens eine Funktion zur `functions` Liste hinzu

### "Funktions-Name enthält ungültige Zeichen"
- **Problem**: Funktionsnamen enthalten Sonderzeichen
- **Lösung**: Verwenden Sie nur alphanumerische Zeichen, Unterstriche und Bindestriche

## 📝 Best Practices

1. **Namen**: Verwenden Sie snake_case für Modul-Namen
2. **Beschreibungen**: Seien Sie präzise und hilfreich
3. **Beispiele**: Fügen Sie Beispiele hinzu für bessere Dokumentation
4. **Typen**: Definieren Sie Typen für komplexe Datenstrukturen
5. **Tests**: Erweitern Sie die generierten Tests mit spezifischen Validierungen

## 🔄 Workflow

1. **Planen**: Überlegen Sie sich die API
2. **Konfigurieren**: Erstellen Sie YAML-Datei oder nutzen Sie interaktiven Modus
3. **Generieren**: Führen Sie den Generator aus
4. **Implementieren**: Füllen Sie die TODO-Stellen in der generierten Datei
5. **Testen**: Führen Sie Tests aus und erweitern Sie sie
6. **Dokumentieren**: Ergänzen Sie die Dokumentation bei Bedarf

## 📄 Lizenz

Teil des VelinScript-Projekts.

## 🤝 Beitragen

Bei Fragen oder Problemen öffnen Sie ein Issue im VelinScript-Repository.
