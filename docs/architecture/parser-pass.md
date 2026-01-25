# ParserPass - Parsing & Modul-Auflösung

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Datei:** `compiler/src/passes/parser.rs`

---

## Übersicht

Der `ParserPass` ist der **zweite Pass** im VelinScript Compiler (nach AutoFixPass). Er ist verantwortlich für:

1. **Lexikalische Analyse** - Tokenisierung des Source-Codes
2. **Syntax-Analyse** - Erstellung des Abstract Syntax Tree (AST)
3. **Modul-Auflösung** - Rekursive Auflösung von `use` Statements
4. **AST-Merging** - Zusammenführung aller Module in einen globalen AST
5. **Parser-Kontext-Tracking** ✅ (Neu in 3.1.0) - Unterscheidung zwischen Struct-Definitionen und Struct-Literalen

---

## Funktionsweise

### 1. Parsing des Root-Moduls

Der Pass beginnt mit dem Parsing der Root-Datei:

```rust
let root_source = context.source_map.get(&context.root_file).unwrap().clone();
match Parser::parse(&root_source) {
    Ok(mut program) => {
        // Modul-Auflösung...
    }
    Err(e) => {
        context.errors.push(e.into());
    }
}
```

**Input:** Source-Code (String) aus `CompilationContext.source_map`  
**Output:** AST (`Program`) mit allen Items

### 2. Modul-Auflösung (Import Resolution)

Der Pass löst alle `use` Statements rekursiv auf:

#### Schritt 1: Use-Statements sammeln

```rust
for item in &program.items {
    if let Item::Use(use_stmt) = item {
        // Finde Modul-Datei: module_name.velin
        let module_path = base_path.join(format!("{}.velin", first_segment));
    }
}
```

#### Schritt 2: Module laden und parsen

```rust
// Lade Source-Code
let source = fs::read_to_string(&mod_path)?;

// Parse Modul
let mod_program = Parser::parse(&source)?;
```

#### Schritt 3: Rekursive Auflösung

```rust
// Rekursiv weitere Module auflösen
self.resolve_imports(&mod_program, mod_dir, context, visited_modules, global_modules)?;
```

#### Schritt 4: AST-Merging

```rust
// Wrappe importierte Items in Module-Item
let mod_item = Item::Module(Module {
    name: mod_name.clone(),
    items: mod_program.items,
    visibility: Visibility::Public,
    documentation: None,
});

// Füge zu globalem AST hinzu
global_modules.push(mod_item);
```

---

## Sicherheitsfeatures

### Path-Traversal-Schutz

Der ParserPass verhindert Path-Traversal-Angriffe:

```rust
// SECURITY: Path-Traversal-Prüfung
if first_segment.contains("..") || 
   first_segment.contains("\\") || 
   first_segment.starts_with("/") {
    context.errors.push(CompilerError::parse_error(
        format!("Invalid module path: '{}'. Path traversal (../) and absolute paths are not allowed.", first_segment),
        ErrorLocation::new(0, 0),
    ));
    continue;
}
```

**Verhindert:**
- `use ../parent` - Path-Traversal
- `use /absolute/path` - Absolute Pfade
- `use ..\windows\path` - Windows Path-Traversal

### Modulname-Validierung

Nur sichere Modulnamen sind erlaubt:

```rust
// SECURITY: Validierung von Modulnamen
if !first_segment.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
    context.errors.push(CompilerError::parse_error(
        format!("Invalid module name: '{}'. Only alphanumeric characters, underscore, and hyphen are allowed.", first_segment),
        ErrorLocation::new(0, 0),
    ));
    continue;
}
```

**Erlaubt:**
- `use models` ✅
- `use user_service` ✅
- `use api-v2` ✅

**Verhindert:**
- `use ../../etc/passwd` ❌
- `use module@name` ❌
- `use module.name` ❌

### Zirkuläre Abhängigkeiten

Der Pass verhindert unendliche Rekursion durch Zyklus-Erkennung:

```rust
// Check if already visited to prevent infinite recursion
if visited_modules.contains(&mod_path_str) {
    continue;  // Modul bereits geladen
}
visited_modules.insert(mod_path_str.clone());
```

**Beispiel:**
```velin
// main.velin
use models;

// models.velin
use services;

// services.velin
use models;  // Zirkuläre Abhängigkeit!
```

Der Pass erkennt dies und lädt `models` nur einmal.

---

## Datenstrukturen

### Program (AST)

```rust
pub struct Program {
    pub items: Vec<Item>,
}
```

**Items können sein:**
- `Item::Use` - Import-Statements
- `Item::Function` - Funktions-Definitionen
- `Item::Struct` - Struct-Definitionen
- `Item::Enum` - Enum-Definitionen
- `Item::Module` - Modul-Wrapper (für importierte Module)
- `Item::TypeAlias` - Typ-Aliase
- `Item::Trait` - Trait-Definitionen
- `Item::Impl` - Trait-Implementierungen
- `Item::TopLevelCode` - Top-Level Ausdrücke

### Module-Item

Importierte Module werden in `Item::Module` gewrappt:

```rust
Item::Module(Module {
    name: "models",           // Modulname
    items: [...],             // Alle Items aus models.velin
    visibility: Public,
    documentation: None,
})
```

**Vorteil:** Ermöglicht Namespacing (z.B. `models.User`)

---

## Beispiel-Ablauf

### Projekt-Struktur

```
project/
├── main.velin
├── models.velin
└── services.velin
```

### main.velin

```velin
use models;
use services;

@GET("/api/users")
fn getUsers(): List<User> {
    return services.getUsers();
}
```

### models.velin

```velin
struct User {
    id: string,
    name: string,
    email: string,
}
```

### services.velin

```velin
use models;

fn getUsers(): List<User> {
    return db.findAll(User);
}
```

### Parsing-Ablauf

1. **Parse main.velin:**
   ```rust
   Program {
       items: [
           Use { path: ["models"] },
           Use { path: ["services"] },
           Function { name: "getUsers", ... }
       ]
   }
   ```

2. **Löse `use models` auf:**
   - Lade `models.velin`
   - Parse zu `Program { items: [Struct { name: "User", ... }] }`
   - Wrappe in `Item::Module { name: "models", items: [...] }`

3. **Löse `use services` auf:**
   - Lade `services.velin`
   - Parse zu `Program { items: [Use { path: ["models"] }, Function {...}] }`
   - Rekursiv: `use models` ist bereits geladen → Skip
   - Wrappe in `Item::Module { name: "services", items: [...] }`

4. **Finaler AST:**
   ```rust
   Program {
       items: [
           Use { path: ["models"] },
           Use { path: ["services"] },
           Module { name: "models", items: [Struct { name: "User", ... }] },
           Module { name: "services", items: [Function { name: "getUsers", ... }] },
           Function { name: "getUsers", ... }
       ]
   }
   ```

---

## Parser-Kontext-Tracking ✅ (Neu in 3.1.0)

Der Parser verwendet ein **Kontext-Tracking-System**, um zwischen verschiedenen Parsing-Kontexten zu unterscheiden. Dies ist besonders wichtig für die Unterscheidung zwischen **Struct-Definitionen** und **Struct-Literalen**.

### ParseContext Enum

```rust
enum ParseContext {
    TopLevel,          // Top-Level Code (Funktionen, Structs, etc.)
    Expression,        // Expression-Kontext (in return, let, etc.)
    StructDefinition,  // Struct-Definition (struct Name { ... })
    Pattern,           // Pattern-Matching (für zukünftige Features)
    Type,              // Typ-Annotationen
}
```

### Kontext-Stack

Der Parser verwaltet einen **Stack von Kontexten**, um verschachtelte Strukturen korrekt zu handhaben:

```rust
struct Parser {
    // ...
    context: Vec<ParseContext>,  // Kontext-Stack
}
```

### Kontext-Management

**Methoden:**
- `push_context(ctx: ParseContext)` - Fügt einen neuen Kontext zum Stack hinzu
- `pop_context()` - Entfernt den obersten Kontext vom Stack
- `current_context() -> ParseContext` - Gibt den aktuellen Kontext zurück

### Verwendung

**1. Expression-Kontext in `parse_return()`:**
```rust
fn parse_return(&mut self) -> Result<ReturnStatement, ParseError> {
    self.push_context(ParseContext::Expression);
    let expr = self.parse_expression();
    self.pop_context();
    // ...
}
```

**2. Struct-Definition-Kontext in `parse_struct()`:**
```rust
fn parse_struct(&mut self, ...) -> Result<Struct, ParseError> {
    self.push_context(ParseContext::StructDefinition);
    // Parse Struct-Felder (erwartet Typen nach ':')
    self.pop_context();
    // ...
}
```

**3. Struct-Literal-Erkennung in `parse_primary()`:**
```rust
// In parse_primary()
if self.check(&Token::LBrace) {
    // Wenn wir NICHT in einem Struct-Definition-Kontext sind,
    // ist es ein Struct-Literal
    if self.current_context() != ParseContext::StructDefinition {
        let fields = self.parse_struct_literal_fields()?;
        return Ok(Expression::StructLiteral { name, fields });
    }
}
```

**4. Expression-Kontext in `parse_struct_literal_fields()`:**
```rust
fn parse_struct_literal_fields(&mut self) -> Result<Vec<(String, Expression)>, ParseError> {
    // ...
    // Stelle sicher, dass wir im Expression-Kontext sind
    let was_in_expression = self.current_context() == ParseContext::Expression;
    if !was_in_expression {
        self.push_context(ParseContext::Expression);
    }
    let value = self.parse_expression();  // Parse Expression, nicht Typ!
    if !was_in_expression {
        self.pop_context();
    }
    // ...
}
```

### Vorteile

- **Eindeutige Unterscheidung:** Der Parser kann zuverlässig zwischen Struct-Definitionen und Struct-Literalen unterscheiden
- **Verschachtelte Strukturen:** Der Kontext-Stack ermöglicht korrektes Parsing von verschachtelten Strukturen
- **Bessere Fehlermeldungen:** Der Parser kann präzisere Fehlermeldungen geben, wenn er den Kontext kennt

### Bekannte Probleme

⚠️ **Aktuell:** Das Problem "Expected type (found: Number(0.0))" in `examples/system-diagnosis/security_checks.velin` besteht weiterhin, obwohl das Kontext-Tracking implementiert wurde. Weitere Debug-Arbeit ist erforderlich.

**Siehe:** `bauplan/Test/PARSER_STRUCT_LITERAL_BUG_ANALYSE.md` für Details.

---

## Fehlerbehandlung

### Parsing-Fehler

Wenn ein Modul nicht geparst werden kann:

```rust
Err(e) => {
    context.errors.push(CompilerError::parse_error(
        format!("Failed to parse module {}: {} (at line {}, column {})", 
                mod_name, e.message, e.line, e.column),
        ErrorLocation::new(e.line, e.column),
    ));
}
```

**Fehler werden gesammelt, aber der Compiler läuft weiter** (außer bei Root-Modul-Fehlern).

### Fehlende Module

Wenn ein Modul nicht gefunden wird:

```rust
if !module_path.exists() {
    context.errors.push(CompilerError::parse_error(
        format!("Module '{}' not found. Expected file: {}", 
                first_segment, module_path.display()),
        ErrorLocation::new(0, 0),
    ));
}
```

### Datei-Lese-Fehler

Wenn eine Modul-Datei nicht gelesen werden kann:

```rust
Err(e) => {
    eprintln!("Failed to read module {}: {}", mod_path.display(), e);
    continue;  // Skip dieses Modul
}
```

---

## Integration in Compiler-Pipeline

### Pass-Reihenfolge

```
1. AutoFixPass      → Korrigiert Source-Code
2. ParserPass       → Erstellt AST (HIER!)
3. DesugaringPass   → Normalisiert AST
4. CodeOrderingPass → Sortiert Items
5. TypeCheckPass    → Type-Checking
...
```

### CompilationContext-Modifikationen

Der ParserPass modifiziert:

- ✅ `context.program` - Setzt den geparsten AST
- ✅ `context.source_map` - Fügt geladene Module hinzu
- ✅ `context.errors` - Sammelt Parsing-Fehler

### Kritischer Pass

**Wichtig:** Wenn der ParserPass fehlschlägt (Root-Modul kann nicht geparst werden), stoppt der Compiler:

```rust
if context.has_errors() {
    if pass.name() == "Parser" {
        break;  // Stoppe Compiler
    }
}
```

---

## API-Referenz

### ParserPass::new()

Erstellt eine neue ParserPass-Instanz:

```rust
let pass = ParserPass::new();
```

### Pass::run()

Führt den Pass aus:

```rust
pass.run(&mut context)?;
```

**Parameter:**
- `context: &mut CompilationContext` - Compilation-Kontext

**Rückgabe:**
- `Result<()>` - Erfolg oder Fehler

### resolve_imports()

Interne Methode für rekursive Modul-Auflösung:

```rust
fn resolve_imports(
    &self,
    program: &Program,
    base_path: &Path,
    context: &mut CompilationContext,
    visited_modules: &mut HashSet<String>,
    global_modules: &mut Vec<Item>
) -> Result<()>
```

**Parameter:**
- `program` - Zu analysierendes Programm
- `base_path` - Basis-Pfad für Modul-Suche
- `context` - Compilation-Kontext
- `visited_modules` - Bereits besuchte Module (Zyklus-Erkennung)
- `global_modules` - Gesammelte Module-Items

---

## Best Practices

### Modul-Organisation

1. **Klare Modulnamen:** Verwende aussagekräftige Namen (`models`, `services`, `utils`)
2. **Flache Hierarchie:** Vermeide tiefe Verschachtelung
3. **Keine Zirkulären Abhängigkeiten:** Organisiere Code so, dass keine Zyklen entstehen

### Fehlerbehandlung

1. **Prüfe Fehler:** Nach dem Parsing sollten `context.errors` geprüft werden
2. **Validiere Module:** Stelle sicher, dass alle Module existieren
3. **Sichere Pfade:** Verwende nur relative Pfade ohne `..`

---

## Debugging

### Parsing-Fehler debuggen

Wenn ein Modul nicht geparst werden kann:

1. **Prüfe Source-Code:** Stelle sicher, dass die Syntax korrekt ist
2. **Prüfe Fehler-Meldung:** Die Fehler-Meldung enthält Zeile und Spalte
3. **Prüfe Context:** Der Fehler enthält Kontext um die fehlerhafte Stelle

### Modul-Auflösung debuggen

Um zu sehen, welche Module geladen werden:

```rust
// In resolve_imports():
eprintln!("Loading module: {}", mod_path.display());
```

### AST-Inhalt anzeigen

Um den generierten AST zu sehen:

```rust
if let Some(ref program) = context.program {
    println!("AST Items: {}", program.items.len());
    for item in &program.items {
        println!("  - {:?}", item);
    }
}
```

---

## Siehe auch

- [Pass-Verlauf](./pass-verlauf.md) - Übersicht aller Passes
- [Module Resolution](./module-resolution.md) - Detaillierte Modul-Auflösung
- [Compiler Architecture](./compiler-architecture.md) - Compiler-Architektur
- [Parser Implementation](../../compiler/src/parser/parser.rs) - Parser-Implementierung

---

**Letzte Aktualisierung:** 2026-02-02  
**Version:** 3.1.0

### 7. Parser-Debug-System ✅ (Neu in 3.1.0)

Um Parsing-Probleme zu diagnostizieren, wurde ein umfassendes Debug-System implementiert.

**Debug-Ausgaben:**
- **`parse_struct()`**: Zeigt Start, Struct-Name, Feld-Parsing und Kontext-Änderungen
- **`parse_struct_literal_fields()`**: Zeigt Start, Feld-Parsing und Token nach `:`
- **`parse_struct_literal_value()`**: Zeigt Token, Kontext und Fallback-Warnungen
- **`parse_type()`**: Zeigt Aufruf, Token, Kontext und Stack

**Verwendung:**
Debug-Ausgaben werden nur in Debug-Builds (`#[cfg(debug_assertions)]`) ausgegeben und helfen bei der Diagnose von Parsing-Problemen.

**Beispiel:**
```
🔍 DEBUG parse_struct() START - Context: [TopLevel, StructDefinition]
🔍 DEBUG parse_struct() - Struct-Name: LogAnalysis
🔍 DEBUG parse_struct() - Parsing field 'warningCount' at line ~73
   Context: StructDefinition, Stack: [TopLevel, StructDefinition]
   Next token after ':': Some(Identifier("number"))
🔍 DEBUG parse_type() aufgerufen - Token: Some(Identifier("number")), Line: ~73, Context: StructDefinition, Stack: [TopLevel, StructDefinition]
   Successfully parsed field type: Number
```

**Siehe auch:**
- `bauplan/Test/PARSER_DEBUG_ANALYSE.md` - Detaillierte Debug-Analyse

---

## Lambda-Erkennung & Leere Parameterlisten ✅ (Neu in 3.1.1)

### Problem

Wenn `()` (leere Parameterliste) geparst wird, versucht der Parser fälschlicherweise, eine Lambda-Funktion zu erkennen und ruft `consume_identifier()` auf, obwohl das nächste Token `RParen` ist. Dies führt zu dem Fehler "Expected identifier (found: LParen)".

### Lösung

In `parse_expression()` wird zuerst geprüft, ob nach `LParen` direkt `RParen` kommt. Wenn ja, wird die leere Parameterliste als Gruppierung behandelt, nicht als Lambda-Funktion.

**Code-Änderung:** `compiler/src/parser/parser.rs`, Zeile 2397-2406

```rust
Some(Token::LParen) => {
    self.advance();
    
    // FIX: Prüfe zuerst, ob es eine leere Parameterliste ist: ()
    if self.check(&Token::RParen) {
        // Leere Parameterliste: () - kein Lambda, sondern Gruppierung
        self.advance(); // consume ')'
        expr = Expr::Grouping(Box::new(expr));
    } else {
        // Lambda-Erkennung nur wenn Parameter vorhanden sind
        // ...
    }
}
```

**Ergebnis:** Der Fehler "Expected identifier (found: LParen)" tritt nicht mehr auf, wenn leere Parameterlisten geparst werden.

**Siehe auch:**
- `bauplan/Test/PARSER_PROBLEM_ANALYSE.md` - Detaillierte Problem-Analyse
