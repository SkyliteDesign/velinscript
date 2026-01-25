# Pass-Verlauf & Funktionsweise

**Version:** 3.1.0  
**Status:** ✅ Vollständig dokumentiert

---

## Übersicht

Der VelinScript Compiler verwendet ein **Pass-basiertes System**, bei dem der Quellcode durch eine Reihe von Transformationen und Analysen läuft. Jeder Pass erfüllt eine spezifische Aufgabe und bereitet den Code für den nächsten Pass vor.

## Pass-Architektur

### Pass Trait

Alle Passes implementieren das `Pass` Trait:

```rust
pub trait Pass {
    fn name(&self) -> &str;
    fn run(&self, context: &mut CompilationContext) -> Result<()>;
}
```

**Eigenschaften:**
- Jeder Pass hat einen eindeutigen Namen
- Passes arbeiten auf dem `CompilationContext`
- Passes können den Context modifizieren (AST, Fehler, Metadaten)
- Passes können Fehler zurückgeben, die den Compiler stoppen

### CompilationContext

Der `CompilationContext` ist der zentrale Datencontainer, der durch alle Passes geteilt wird:

```rust
pub struct CompilationContext {
    pub root_file: String,
    pub source_map: HashMap<String, String>,
    pub program: Option<Program>,  // AST
    pub errors: Vec<CompilationError>,
    pub warnings: Vec<CompilationWarning>,
    pub semantic_metadata: SemanticMetadata,
    // ... weitere Metadaten
}
```

---

## Pass-Reihenfolge & Verlauf

### 1. AutoFixPass

**Position:** Pass 1 (immer zuerst)  
**Datei:** `compiler/src/passes/autofix.rs`  
**Status:** ✅ Immer aktiv (kann mit `--autofix` Flag aktiviert werden)

**Funktionsweise:**
1. Analysiert den Quellcode auf häufige Syntax-Fehler
2. Korrigiert automatisch:
   - Unbalancierte Klammern (`{`, `[`, `(`)
   - Fehlende Klammern in Funktionssignaturen
   - Unvollständige Generic-Typen (`List<T` → `List<T>`)
3. Modifiziert den Source-Code im `CompilationContext` **vor** dem Parsing

**Input:** Roher Source-Code (String)  
**Output:** Korrigierter Source-Code (String)  
**Modifiziert:** `context.source_map`

**Beispiel:**
```velin
// Vorher (fehlerhaft):
fn process(data: List<string {
    return data;
}

// Nachher (korrigiert):
fn process(data: List<string>) {
    return data;
}
```

---

### 2. ParserPass

**Position:** Pass 2  
**Datei:** `compiler/src/passes/parser.rs`  
**Status:** ✅ Immer aktiv

**Funktionsweise:**
1. **Lexikalische Analyse:** Tokenisiert den Source-Code
2. **Syntax-Analyse:** Erstellt den Abstract Syntax Tree (AST)
3. **Modul-Auflösung:** 
   - Erkennt `use` Statements
   - Lädt abhängige Module rekursiv
   - Merged alle Module in einen globalen AST
4. **Fehlerbehandlung:** Sammelt Parsing-Fehler

**Input:** Source-Code (String)  
**Output:** AST (`Program`)  
**Modifiziert:** `context.program`, `context.source_map`, `context.errors`

**Wichtig:** Wenn der Parser fehlschlägt, stoppt der Compiler (außer bei AutoFix).

**Beispiel:**
```velin
// Input (Source-Code):
use models;
struct User { name: string; }
fn getUser(): User { ... }

// Output (AST):
Program {
    items: [
        Use { module: "models" },
        Struct { name: "User", fields: [...] },
        Function { name: "getUser", return_type: "User", ... }
    ]
}
```

---

### 3. DesugaringPass

**Position:** Pass 3  
**Datei:** `compiler/src/passes/desugar.rs`  
**Status:** ✅ Immer aktiv (seit 3.0.1)

**Funktionsweise:**
1. **Syntaktischer Zucker Transformation:**
   - `try-catch` → `Result<T, E>` Pattern
   - Sugar-Syntax → Standard-Syntax
2. **Normalisierung:** Vereinfacht komplexe Ausdrücke
3. **AST-Transformation:** Modifiziert den AST direkt

**Input:** AST (mit Sugar-Syntax)  
**Output:** Normalisierter AST  
**Modifiziert:** `context.program`

**Beispiel:**
```velin
// Vorher (Sugar):
try {
    let result = riskyOperation();
} catch (e: Error) {
    handleError(e);
}

// Nachher (normalisiert):
match riskyOperation() {
    Ok(result) => { ... },
    Err(e) => { handleError(e); }
}
```

---

### 4. CodeOrderingPass

**Position:** Pass 4  
**Datei:** `compiler/src/passes/code_order.rs`  
**Status:** ✅ Immer aktiv (seit 3.1.0)

**Funktionsweise:**
1. **Dependency-Analyse:** 
   - Analysiert alle Items (Functions, Structs, Enums, etc.)
   - Erstellt einen Dependency-Graph
2. **Topologische Sortierung:**
   - Sortiert Items basierend auf Abhängigkeiten
   - Verwendet `petgraph` für Graph-Algorithmen
3. **Zirkuläre Abhängigkeiten:**
   - Erkennt und meldet zirkuläre Abhängigkeiten als Fehler

**Input:** AST (unsortiert)  
**Output:** AST (sortiert)  
**Modifiziert:** `context.program.items`

**Sortierreihenfolge:**
1. Use Statements
2. TypeAliases
3. Enums
4. Structs
5. Traits
6. Impls
7. Functions
8. TopLevelCode

**Beispiel:**
```velin
// Vorher (unsortiert):
fn processUser(user: User) { ... }
struct User { name: string; }

// Nachher (sortiert):
struct User { name: string; }
fn processUser(user: User) { ... }
```

---

### 5. AISemanticPass (Optional)

**Position:** Pass 5  
**Datei:** `compiler/src/passes/ai_semantic.rs`  
**Status:** ⚙️ Optional (via `--ai-semantic` Flag)

**Funktionsweise:**
1. **LLM-Analyse:** Sendet Code an LLM für semantische Analyse
2. **Kontext-Erkennung:** Erkennt Art des Codes (API, Service, Library, Application)
3. **Dependency-Erkennung:** Identifiziert benötigte Abhängigkeiten
4. **Security-Analyse:** Analysiert Sicherheitsanforderungen
5. **Metadaten-Speicherung:** Speichert Ergebnisse in `context.semantic_metadata`

**Input:** AST  
**Output:** Semantic Metadata  
**Modifiziert:** `context.semantic_metadata`

**Fallback:** Wenn kein LLM konfiguriert ist, verwendet heuristische Analysen.

---

### 6. AIBugDetectionPass (Optional)

**Position:** Pass 6  
**Datei:** `compiler/src/passes/ai_bug_detection.rs`  
**Status:** ⚙️ Optional (via `--ai-bug-detection` Flag)

**Funktionsweise:**
1. **Pattern-basierte Erkennung:** Erkennt bekannte Bug-Patterns
2. **KI-basierte Analyse:** Nutzt LLM für semantische Bug-Erkennung
3. **Auto-Fix:** Behebt einfache Bugs automatisch
4. **Fehler-Sammlung:** Sammelt alle gefundenen Bugs

**Input:** AST  
**Output:** Bug-Reports  
**Modifiziert:** `context.errors`, `context.warnings`

**Erkannte Patterns:**
- Fehlende Error Handling
- Potenzielle Null-Pointer
- Fehlende Auth bei sensiblen Operationen
- Sicherheitslücken

---

### 7. TypeCheckPass

**Position:** Pass 7  
**Datei:** `compiler/src/passes/type_check.rs`  
**Status:** ✅ Immer aktiv (kann mit `--no-type-check` deaktiviert werden)

**Funktionsweise:**
1. **Type-Inference:** 
   - Analysiert alle Ausdrücke
   - Leitet Typen automatisch ab
   - Verwendet `TypeChecker` aus `compiler/src/type_checker/`
2. **Type-Checking:**
   - Prüft Typ-Kompatibilität
   - Validiert Funktionssignaturen
   - Prüft Generic-Constraints
3. **Borrow-Checking:**
   - Integriert `BorrowChecker` für Ownership-Analyse
   - Prüft auf Use-After-Move, Multiple Borrows, etc.
4. **Fehler-Sammlung:** Sammelt alle Type-Fehler

**Input:** AST  
**Output:** Type-annotierter AST  
**Modifiziert:** `context.program`, `context.errors`

**Wichtig:** Dieser Pass ist kritisch - viele nachfolgende Passes benötigen korrekte Typ-Informationen.

**Beispiel:**
```velin
// Input:
let x = "hello";
let y: number = x;  // Type-Fehler!

// Output:
Error: Cannot assign string to number
  at line 2: let y: number = x;
```

---

### 8. ParallelizationAnalyzer

**Position:** Pass 8  
**Datei:** `compiler/src/optimizer/parallelization.rs`  
**Status:** ✅ Immer aktiv (seit 3.1)

**Funktionsweise:**
1. **Datenabhängigkeits-Analyse:** 
   - Analysiert async Blöcke
   - Erkennt parallelisierbare Operationen
2. **Optimierung:**
   - Ersetzt sequentielle `await` durch `tokio::join!`
   - Optimiert Pipeline-Operationen
3. **Metadaten:** Speichert Optimierungs-Vorschläge

**Input:** Type-annotierter AST  
**Output:** Optimierter AST  
**Modifiziert:** `context.program`

**Beispiel:**
```velin
// Vorher:
let a = await operation1();
let b = await operation2();

// Nachher:
let (a, b) = tokio::join!(operation1(), operation2());
```

---

### 9. AICodeGenerationPass (Optional)

**Position:** Pass 9  
**Datei:** `compiler/src/passes/ai_codegen.rs`  
**Status:** ⚙️ Optional (via `--ai-codegen` Flag)

**Funktionsweise:**
1. **Fehlende Komponenten:** Identifiziert fehlende Funktionen/Strukturen
2. **KI-Generierung:** Nutzt LLM, um fehlenden Code zu generieren
3. **Validierung:** Validiert generierten Code
4. **AST-Erweiterung:** Fügt generierten Code zum AST hinzu

**Input:** AST (mit fehlenden Komponenten)  
**Output:** Erweiterter AST  
**Modifiziert:** `context.program`

**Fallback:** Wenn kein LLM konfiguriert ist, verwendet `BoilerplateGenerator`.

---

### 10. AICodeReviewPass (Optional)

**Position:** Pass 10  
**Datei:** `compiler/src/passes/ai_code_review.rs`  
**Status:** ⚙️ Optional (automatisch aktiviert wenn `--ai-codegen` aktiv ist)

**Funktionsweise:**
1. **Security-Checks:** Prüft auf Sicherheitslücken
2. **Syntax-Validierung:** Validiert generierten Code
3. **Type-Checking:** Prüft Typ-Korrektheit
4. **Complexity-Check:** Analysiert zyklomatische Komplexität
5. **Pattern-Check:** Erkennt gefährliche Patterns

**Input:** AST (mit AI-generiertem Code)  
**Output:** Review-Reports  
**Modifiziert:** `context.errors`, `context.warnings`

**Sicherheitsprüfungen:**
- Verbotene Imports
- Unsichere Patterns
- Fehlende Authentifizierung
- Input-Validierung

---

### 11. AISandboxPass (Optional)

**Position:** Pass 11  
**Datei:** `compiler/src/passes/ai_sandbox.rs`  
**Status:** ⚙️ Optional (automatisch aktiviert wenn `--ai-codegen` aktiv ist)

**Funktionsweise:**
1. **AST-Durchsuchung:** Durchsucht vollständig den AST
2. **Funktions-Validierung:** Prüft alle Funktionsaufrufe
3. **Sandbox-Checks:** 
   - Erlaubte Funktionen: sichere Operationen (add, subtract, etc.)
   - Verbotene Funktionen: read_file, write_file, execute, system, eval
4. **Fehler-Meldung:** Meldet gefährliche Operationen

**Input:** AST  
**Output:** Sandbox-Reports  
**Modifiziert:** `context.errors`

**Wichtig:** Dieser Pass verhindert, dass AI-generierter Code gefährliche Operationen ausführt.

---

### 12. AIOptimizationPass (Optional)

**Position:** Pass 12  
**Datei:** `compiler/src/passes/ai_optimization.rs`  
**Status:** ⚙️ Optional (via `--ai-optimization` Flag)

**Funktionsweise:**
1. **Optimierungs-Analyse:** Nutzt LLM für Optimierungs-Vorschläge
2. **Performance-Optimierung:** Parallelisierung, Caching
3. **Memory-Optimierung:** Reduzierung von Allokationen
4. **Code-Refactoring:** Verbessert Code-Readability

**Input:** AST  
**Output:** Optimierter AST  
**Modifiziert:** `context.program`

**Fallback:** Wenn kein LLM konfiguriert ist, verwendet heuristische Optimierungen.

---

### 13. CodegenPass

**Position:** Pass 13 (letzter Pass)  
**Datei:** `compiler/src/passes/codegen.rs`  
**Status:** ✅ Immer aktiv

**Funktionsweise:**
1. **IR-Generierung (Standard):**
   - AST → IR (Intermediate Representation)
   - IR-Optimierung (Dead Code Elimination, Constant Folding, etc.)
   - IR → Target Code
2. **Legacy-Pipeline (optional):**
   - AST → Target Code (direkt)
3. **Multi-Target Support:**
   - Rust (Default)
   - PHP, Python, TypeScript, JavaScript, Go, Java, C#

**Input:** Optimierter AST  
**Output:** Generierter Code (String)  
**Modifiziert:** `context.generated_code`

**Pipeline:**
```
AST → IR Builder → IR → IR Optimizer → IR Validator → Code Generator → Target Code
```

**Beispiel:**
```velin
// Input (VelinScript):
@GET("/api/users")
fn getUsers(): List<User> {
    return db.findAll(User);
}

// Output (Rust):
#[get("/api/users")]
async fn get_users() -> Result<Json<Vec<User>>, AppError> {
    let users = db.find_all::<User>().await?;
    Ok(Json(users))
}
```

---

## Datenfluss durch die Passes

```
┌─────────────────────────────────────────────────────────────┐
│                    Source Code (String)                      │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 1: AutoFixPass                                         │
│  - Korrigiert Syntax-Fehler                                 │
│  - Modifiziert: source_map                                  │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 2: ParserPass                                         │
│  - Erstellt AST                                             │
│  - Modul-Auflösung                                          │
│  - Modifiziert: program, source_map, errors                │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 3: DesugaringPass                                    │
│  - Normalisiert Sugar-Syntax                                │
│  - Modifiziert: program                                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 4: CodeOrderingPass                                   │
│  - Sortiert Items basierend auf Abhängigkeiten              │
│  - Modifiziert: program.items                               │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 5: AISemanticPass (optional)                         │
│  - Semantische Analyse mit LLM                              │
│  - Modifiziert: semantic_metadata                           │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 6: AIBugDetectionPass (optional)                     │
│  - Bug-Erkennung                                            │
│  - Modifiziert: errors, warnings                            │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 7: TypeCheckPass                                      │
│  - Type-Inference & Type-Checking                           │
│  - Borrow-Checking                                          │
│  - Modifiziert: program, errors                            │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 8: ParallelizationAnalyzer                            │
│  - Parallelisierungs-Optimierung                             │
│  - Modifiziert: program                                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 9: AICodeGenerationPass (optional)                   │
│  - Generiert fehlenden Code                                 │
│  - Modifiziert: program                                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 10: AICodeReviewPass (optional)                      │
│  - Reviewt AI-generierten Code                              │
│  - Modifiziert: errors, warnings                            │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 11: AISandboxPass (optional)                          │
│  - Validiert Code in Sandbox                                 │
│  - Modifiziert: errors                                      │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 12: AIOptimizationPass (optional)                     │
│  - KI-basierte Optimierung                                  │
│  - Modifiziert: program                                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Pass 13: CodegenPass                                       │
│  - AST → IR → Optimized IR → Target Code                   │
│  - Multi-Target Code-Generierung                            │
│  - Modifiziert: generated_code                              │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                    Target Code (String)                      │
└─────────────────────────────────────────────────────────────┘
```

---

## Fehlerbehandlung ✅ (Verbessert in 3.1.0)

### Pass-Fehler

Wenn ein Pass fehlschlägt:

1. **AutoFixPass:** Fehler werden ignoriert (AutoFix ist optional)
2. **ParserPass:** Compiler stoppt (kritischer Fehler) ✅
3. **TypeCheckPass:** Compiler stoppt (kritischer Fehler) ✅ (Neu in 3.1.0)
4. **CodegenPass:** Compiler stoppt (kritischer Fehler) ✅ (Neu in 3.1.0)
5. **Andere Passes:** Fehler werden gesammelt, Compiler läuft weiter (außer bei kritischen Fehlern)

### Fehler-Aggregation

Alle Fehler werden im `CompilationContext` gesammelt:

```rust
context.errors      // Alle Fehler
context.warnings    // Alle Warnungen (separat) ✅ (Neu in 3.1.0)
```

**Zentrale API:**
```rust
context.add_error(CompilerError::parse_error(...));  // ✅ Neu
context.add_warning("Deprecated function".to_string());  // ✅ Neu
context.add_info("Optimization applied".to_string());  // ✅ Neu
```

### Fehler-Reporting mit Lösungsvorschlägen ✅ (Neu in 3.1.0)

Am Ende der Kompilierung werden alle Fehler mit intelligenten Lösungsvorschlägen ausgegeben:

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

⚠️  Warning: Deprecated function
  at main.velin:15: let y = 42;
```

**Siehe auch:** [Fehlerbehandlung & Lösungsvorschläge](error-handling.md) ✅ (Neu in 3.1.0)

---

## Pass-Konfiguration

### CLI-Flags

```bash
velin compile -i main.velin \
  --autofix \                    # AutoFix aktivieren
  --ai-semantic \                # AISemanticPass aktivieren
  --ai-bug-detection \           # AIBugDetectionPass aktivieren
  --ai-codegen \                 # AICodeGenerationPass aktivieren
  --ai-optimization \            # AIOptimizationPass aktivieren
  --no-type-check \              # TypeCheckPass deaktivieren
  --target rust                  # CodegenPass Target
```

### Config-Datei

```toml
[compiler]
enable_autofix = true
enable_ai_semantic = false
enable_ai_bug_detection = false
enable_ai_codegen = false
enable_ai_optimization = false
enable_type_check = true
target = "rust"
```

---

## Best Practices

### Pass-Entwicklung

1. **Ein Pass, eine Aufgabe:** Jeder Pass sollte eine klar definierte Aufgabe haben
2. **Immutability wo möglich:** Modifiziere nur, was nötig ist
3. **Fehler-Sammlung:** Sammle Fehler, stoppe nicht sofort (außer bei kritischen Fehlern)
4. **Metadaten:** Nutze `CompilationContext` für Metadaten zwischen Passes

### Pass-Reihenfolge

Die Pass-Reihenfolge ist kritisch:
- **AutoFix** muss vor **Parser** sein
- **Parser** muss vor allen anderen sein
- **TypeCheck** sollte vor **Codegen** sein
- **Codegen** ist immer der letzte Pass

---

## Siehe auch

- [Compiler Architecture](./compiler-architecture.md) - Übersicht der Compiler-Architektur
- [Type Inference](./type-inference.md) - Type-Inference System
- [Code Ordering](./code-ordering.md) - Code-Ordering System
- [AI Compiler Passes](./ai-compiler-passes.md) - KI-basierte Passes
- [IR Representation](./ir-representation.md) - Intermediate Representation

---

**Letzte Aktualisierung:** 2026-02-02  
**Version:** 3.1.0
