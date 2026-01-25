# VelinScript Compiler - Fehlerbehandlung & Lösungsvorschläge

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Letzte Aktualisierung:** 2026-02-02

---

## Übersicht

Der VelinScript Compiler verfügt über ein umfassendes Fehlerbehandlungssystem mit intelligenten Lösungsvorschlägen für alle Fehlertypen. Dieses System hilft Entwicklern, Fehler schnell zu identifizieren und zu beheben.

---

## Fehlertypen

Der Compiler unterscheidet zwischen verschiedenen Fehlertypen, die jeweils spezifische Lösungsvorschläge erhalten:

### 1. Parse Errors (Syntax-Fehler)

**Beispiel:**
```
❌ Parse error: Unexpected token at line 5, column 10
📁 Datei: main.velin
📍 Position: Zeile 5, Spalte 10

📖 Erwartet: function
🔍 Gefunden: fn

💡 Did you mean: 'fn'?
💡 Tip: Function declarations use 'fn', not 'function'
   Beispiel: fn myFunction(): string { return "hello"; }

🔧 Lösungsvorschläge:
   - Prüfe auf fehlende oder überflüssige Klammern
   - Nutze 'velin check --autofix' für automatische Korrekturen
   - Prüfe die Syntax in der Dokumentation
   - Siehe: docs/language/specification.md
```

**Häufige Parse-Fehler:**
- Unerwartete Tokens
- Fehlende Klammern
- Falsche Syntax (z.B. `function` statt `fn`)
- Unvollständige Generic-Typen

---

### 2. Type Errors (Typ-Fehler)

**Beispiel:**
```
❌ Type error: Type mismatch at line 12, column 15
📁 Datei: main.velin
📍 Position: Zeile 12, Spalte 15

💡 Did you mean: 'length'?
💡 Beispiel für explizite Typ-Annotation:
   let x: number = 42;
   let name: string = "John";

🔧 Lösungsvorschläge:
   - Prüfe die Typen deiner Variablen
   - Nutze explizite Typ-Annotationen bei Unsicherheit
   - Siehe: docs/guides/tutorial-1-basics.md
```

**Häufige Type-Fehler:**
- Type Mismatch
- Undefined Variable
- Undefined Type
- Falsche Argument-Anzahl

---

### 3. CodeGen Errors (Code-Generierungsfehler)

**Beispiel:**
```
❌ Code Generation Error: IR code generation failed
📁 Datei: main.velin
📍 Position: Zeile 0, Spalte 0
📋 Context: Target: Rust, Module: main.velin

🔧 Lösungsvorschläge:
   - Prüfe die Syntax des generierten Codes
   - Stelle sicher, dass alle Dependencies verfügbar sind
   - Prüfe die IR-Repräsentation auf Korrektheit
   - Siehe: docs/architecture/code-generation.md
   - Prüfe die IR-Validierung
   - Nutze 'velin check' zur Diagnose
```

**Häufige CodeGen-Fehler:**
- IR-Generierungsfehler
- Target-spezifische Fehler
- Dependency-Probleme

---

### 4. IO Errors (Datei-/IO-Fehler)

**Beispiel:**
```
❌ IO Error: File not found: test.velin

🔧 Lösungsvorschläge:
   - Prüfe ob die Datei existiert
   - Prüfe den Dateipfad auf Tippfehler
   - Prüfe die Dateiberechtigungen
```

**Häufige IO-Fehler:**
- Datei nicht gefunden
- Berechtigungsfehler
- Datei zu groß (>5MB Limit)

---

### 5. Validation Errors (Validierungsfehler)

**Beispiel:**
```
❌ Validation Error: Field is required
📋 Feld: name

🔧 Lösungsvorschläge:
   - Prüfe die Validierungsregeln
   - Stelle sicher, dass alle erforderlichen Felder vorhanden sind
   - Siehe: docs/guides/validation.md
   - Stelle sicher, dass alle Pflichtfelder ausgefüllt sind
```

**Häufige Validation-Fehler:**
- Fehlende Pflichtfelder
- Format-Fehler
- Ungültige Werte

---

### 6. Config Errors (Konfigurationsfehler)

**Beispiel:**
```
❌ Configuration Error: Invalid JSON

🔧 Lösungsvorschläge:
   - Prüfe velin.config.json auf Syntax-Fehler
   - Nutze 'velin config validate' zur Validierung
   - Siehe: docs/guides/configuration.md
   - Prüfe die JSON-Syntax (Kommas, Anführungszeichen)
   - Nutze einen JSON-Validator
```

**Häufige Config-Fehler:**
- JSON-Syntax-Fehler
- Fehlende Konfigurationsfelder
- Ungültige Werte

---

### 7. Internal Errors (Interne Compiler-Fehler)

**Beispiel:**
```
❌ Internal Compiler Error: Unexpected error

⚠️  Dies ist ein interner Fehler. Bitte melde diesen Fehler:
   - GitHub Issues: https://github.com/velinscript/velinscript/issues
   - Include: Compiler-Version, Fehlermeldung, Code-Beispiel
   - Minimales Reproduktionsbeispiel wäre hilfreich

💡 Mögliche Workarounds:
   - Versuche den Code zu vereinfachen
   - Prüfe ob der Fehler mit anderen Dateien reproduzierbar ist
   - Nutze 'velin check' zur Diagnose
```

**Hinweis:** Internal Errors sind selten und sollten als Bug gemeldet werden.

---

### 8. Warnings (Warnungen)

**Beispiel:**
```
⚠️  Warning: Deprecated function
```

Warnings werden separat von Fehlern behandelt und stoppen die Kompilierung nicht.

---

### 9. Info (Informationsmeldungen)

**Beispiel:**
```
ℹ️  Info: Optimization applied
```

Info-Meldungen sind rein informativ und zeigen Optimierungen oder Hinweise an.

---

## Fehlerbehandlung im Compiler

### CompilationContext API

Der `CompilationContext` bietet zentrale Methoden zur Fehlerbehandlung:

```rust
// Fehler hinzufügen
context.add_error(CompilerError::parse_error(...));
context.add_error(CompilerError::type_error(...));
context.add_error(CompilerError::codegen_error(...));

// Warnings hinzufügen
context.add_warning("Deprecated function".to_string());
context.add_info("Optimization applied".to_string());

// Fehler prüfen
if context.has_errors() {
    // Fehlerbehandlung
}

if context.has_warnings() {
    // Warnings anzeigen
}

// Fehlerstatistiken
let stats = context.get_error_statistics();
println!("Parse Errors: {}", stats.parse_errors);
println!("Type Errors: {}", stats.type_errors);
println!("Warnings: {}", stats.warnings);
```

---

## Pass-Fehlerbehandlung

### Kritische Passes

Bestimmte Passes stoppen die Kompilierung bei Fehlern:

1. **ParserPass** - Stoppt bei Syntax-Fehlern
2. **TypeCheckPass** - Stoppt bei Type-Fehlern
3. **CodegenPass** - Stoppt bei Code-Generierungsfehlern

**Implementierung:**
```rust
let critical_passes = ["Parser", "TypeCheck", "Codegen"];

for pass in &self.passes {
    pass.run(&mut context)?;
    
    if context.has_errors() {
        if critical_passes.contains(&pass.name()) {
            break; // Stoppe bei kritischen Passes
        }
    }
}
```

### Fehler-Sammlung

Alle Passes sammeln Fehler im `CompilationContext`:

```rust
// In einem Pass
context.add_error(CompilerError::parse_error(
    "Unexpected token".to_string(),
    ErrorLocation::new(line, column)
));
```

---

## Lösungsvorschläge

### Error-Suggestion-Engine

Die `ErrorSuggestionEngine` bietet intelligente Lösungsvorschläge:

**Features:**
- **Did you mean?** - Levenshtein-Distanz-basierte Korrekturvorschläge
- **Kontextbezogene Hinweise** - Spezifische Tipps je Fehlertyp
- **Dokumentations-Links** - Direkte Links zu relevanten Dokumentationen
- **Code-Beispiele** - Praktische Beispiele zur Fehlerbehebung

**Verwendung:**
```rust
let error = CompilerError::parse_error(...);
let enhanced_message = error.with_suggestions();
println!("{}", enhanced_message);
```

---

## Fehlerstatistiken

### ErrorStatistics

Der Compiler sammelt detaillierte Statistiken über alle Fehlertypen:

```rust
pub struct ErrorStatistics {
    pub parse_errors: usize,
    pub type_errors: usize,
    pub codegen_errors: usize,
    pub io_errors: usize,
    pub validation_errors: usize,
    pub config_errors: usize,
    pub internal_errors: usize,
    pub warnings: usize,
    pub info: usize,
}
```

**Verwendung:**
```rust
let stats = context.get_error_statistics();
println!("Total Errors: {}", 
    stats.parse_errors + 
    stats.type_errors + 
    stats.codegen_errors
);
```

---

## Fehler-Export

### JSON-Export

Fehler können als JSON exportiert werden:

```rust
let json = context.export_errors_json()?;
println!("{}", json);
```

**JSON-Struktur:**
```json
{
  "errors": [
    {
      "type": "Parse",
      "message": "Unexpected token",
      "suggestion": "❌ Parse error: ..."
    }
  ],
  "warnings": [...],
  "statistics": {
    "parse_errors": 1,
    "type_errors": 0,
    ...
  }
}
```

### HTML-Export

Fehler können als HTML-Report exportiert werden:

```rust
let html = context.export_errors_html();
fs::write("error-report.html", html)?;
```

**Features:**
- Professionelles Styling
- Fehler- und Warning-Kategorisierung
- Statistik-Dashboard
- Responsive Design

---

## Fehler-Filterung

### ErrorFilter

Fehler können nach Typ gefiltert werden:

```rust
// Nur Parse-Fehler
let parse_errors = context.filter_errors(ErrorFilter::Parse);

// Nur kritische Fehler
let critical_errors = context.filter_errors(ErrorFilter::Critical);

// Alle Fehler
let all_errors = context.filter_errors(ErrorFilter::All);
```

**Verfügbare Filter:**
- `All` - Alle Fehler
- `Parse` - Nur Parse-Fehler
- `Type` - Nur Type-Fehler
- `CodeGen` - Nur CodeGen-Fehler
- `Io` - Nur IO-Fehler
- `Validation` - Nur Validierungsfehler
- `Config` - Nur Config-Fehler
- `Internal` - Nur interne Fehler
- `Warnings` - Nur Warnungen
- `Critical` - Nur kritische Fehler (Parse, Type, CodeGen)

---

## Best Practices

### Für Entwickler

1. **Immer `with_suggestions()` verwenden:**
   ```rust
   for error in &context.errors {
       eprintln!("{}", error.with_suggestions());
   }
   ```

2. **Warnings separat behandeln:**
   ```rust
   if context.has_warnings() {
       for warning in &context.warnings {
           eprintln!("{}", warning.with_suggestions());
       }
   }
   ```

3. **Fehlerstatistiken nutzen:**
   ```rust
   let stats = context.get_error_statistics();
   if stats.parse_errors > 0 {
       // Spezifische Behandlung für Parse-Fehler
   }
   ```

### Für Pass-Entwickler

1. **Zentrale API verwenden:**
   ```rust
   // ✅ Gut
   context.add_error(CompilerError::parse_error(...));
   
   // ❌ Schlecht
   context.errors.push(...);
   ```

2. **Error-Location immer angeben:**
   ```rust
   context.add_error(CompilerError::parse_error(
       message,
       ErrorLocation::with_file(line, column, file)
   ));
   ```

3. **Warnings für nicht-kritische Probleme:**
   ```rust
   context.add_warning("Deprecated function used".to_string());
   ```

---

## CLI-Integration

### Fehleranzeige

Der Compiler zeigt Fehler automatisch mit Vorschlägen an:

```bash
$ velin compile -i main.velin

✗ Kompilierung fehlgeschlagen mit 2 Fehlern:

❌ Parse error: Unexpected token at line 5, column 10
📁 Datei: main.velin
📍 Position: Zeile 5, Spalte 10
...
────────────────────────────────────────────────────────────

⚠️  1 Warnung(en):

⚠️  Warning: Deprecated function
────────────────────────────────────────────────────────────

💡 Tipp: Nutze 'velin check --autofix' für automatische Korrekturen
📖 Hilfe: Siehe docs/guides/getting-started.md für weitere Informationen
```

### Warnings bei erfolgreicher Kompilierung

Auch bei erfolgreicher Kompilierung werden Warnings angezeigt:

```bash
$ velin compile -i main.velin

⚠️  2 Warnung(en):

⚠️  Warning: Deprecated function
⚠️  Warning: Unused variable

✓ Kompilierung erfolgreich
```

---

## Implementierungsdetails

### Error-Suggestion-Engine

**Datei:** `compiler/src/error/suggestions.rs`

**Features:**
- Levenshtein-Distanz für "Did you mean?" Vorschläge
- Kontextbezogene Lösungsvorschläge
- Automatische Dokumentations-Links
- Code-Beispiele für häufige Fehler

### CompilationContext

**Datei:** `compiler/src/compiler/context.rs`

**Erweiterungen:**
- `warnings: Vec<CompilerError>` - Separate Warnings-Sammlung
- `add_error()`, `add_warning()`, `add_info()` - Zentrale API
- `get_error_statistics()` - Fehlerstatistiken
- `export_errors_json()` - JSON-Export
- `export_errors_html()` - HTML-Export
- `filter_errors()` - Fehler-Filterung

### Pass-Fehlerbehandlung

**Datei:** `compiler/src/compiler/mod.rs`

**Verbesserungen:**
- Kritische Passes stoppen bei Fehlern
- Konsistente Fehlerbehandlung
- Verbesserte Fehlermeldungen

---

## Siehe auch

- **[Pass-Verlauf](pass-verlauf.md)** - Wie Passes Fehler behandeln
- **[Compiler Architecture](compiler-architecture.md)** - Compiler-Architektur
- **[Getting Started](../guides/getting-started.md)** - Erste Schritte
- **[Language Specification](../language/specification.md)** - Sprachspezifikation

---

**Letzte Aktualisierung:** 2026-02-02  
**Version:** 3.1.0
