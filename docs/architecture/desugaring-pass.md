# DesugaringPass - Syntaktischer Zucker Transformation

**Version:** 3.0.1 / 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Datei:** `compiler/src/passes/desugar.rs`

---

## Übersicht

Der `DesugaringPass` ist der **dritte Pass** im VelinScript Compiler (nach ParserPass). Er transformiert **syntaktischen Zucker** (sugar syntax) in Standard-Syntax, die vom Type Checker und Code Generator verarbeitet werden kann.

**Hauptfunktion:** Transformation von `try-catch` Statements zu `Result<T, E>` Pattern.

---

## Was ist "Syntaktischer Zucker"?

Syntaktischer Zucker ist eine vereinfachte Syntax, die für Entwickler bequemer ist, aber intern in Standard-Syntax transformiert wird.

**Beispiel:**
```velin
// Sugar-Syntax (bequem für Entwickler)
try {
    return riskyOperation();
} catch (err) {
    handleError(err);
}

// Nach Desugaring (Standard-Syntax)
let __try_result = (|| {
    return Result.ok(riskyOperation());
})();
if __try_result.isErr() {
    let err = __try_result.err().unwrap();
    handleError(err);
}
```

---

## Funktionsweise

### 1. Programm-Durchlauf

Der Pass durchläuft rekursiv alle Items im Programm:

```rust
fn desugar_program(&self, program: &mut Program) {
    for item in &mut program.items {
        self.desugar_item(item);
    }
}
```

### 2. Item-Desugaring

Verschiedene Item-Typen werden unterschiedlich behandelt:

```rust
fn desugar_item(&self, item: &mut Item) {
    match item {
        Item::Function(func) => {
            self.desugar_block(&mut func.body);
        }
        Item::Module(module) => {
            // Rekursiv für Sub-Items
            for sub_item in &mut module.items {
                self.desugar_item(sub_item);
            }
        }
        Item::Impl(impl_block) => {
            // Desugar alle Methoden
            for method in &mut impl_block.methods {
                self.desugar_block(&mut method.body);
            }
        }
        Item::TopLevelCode(expr_stmt) => {
            self.desugar_expression(&mut expr_stmt.expression);
        }
        _ => {}
    }
}
```

### 3. Block-Desugaring

Der Pass durchsucht alle Statements in einem Block:

```rust
fn desugar_block(&self, block: &mut Block) {
    for statement in block.statements.drain(..) {
        match statement {
            Statement::Try(try_stmt) => {
                // Transformiere try-catch zu Result-Pattern
                let desugared = self.desugar_try_statement(try_stmt);
                new_statements.extend(desugared);
            }
            Statement::If(mut if_stmt) => {
                // Rekursiv für verschachtelte Blöcke
                self.desugar_block(&mut if_stmt.then_block);
                if let Some(ref mut else_block) = if_stmt.else_block {
                    self.desugar_block(else_block);
                }
            }
            // ... weitere Statements
        }
    }
}
```

---

## Try-Catch Transformation

### Einfaches Try-Catch

**Vorher (Sugar):**
```velin
try {
    return someFunction();
} catch (err) {
    handleError(err);
}
```

**Nachher (Desugared):**
```velin
// 1. Try-Block wird zu Lambda
let __try_result = (|| {
    return Result.ok(someFunction());
})();

// 2. Catch-Block wird zu if-Statement
if __try_result.isErr() {
    let err = __try_result.err().unwrap();
    handleError(err);
}
```

### Transformation-Schritte

#### Schritt 1: Try-Block zu Lambda

```rust
// Try-Block wird in Lambda gewrappt
let try_lambda = Expression::Lambda {
    params: Vec::new(),
    return_type: None,
    body: Box::new(Expression::Block(try_block)),
};

// Lambda wird aufgerufen
let try_result_expr = Expression::Call {
    callee: Box::new(try_lambda),
    args: Vec::new(),
};
```

#### Schritt 2: Returns in Result.ok() wrappen

```rust
// Alle Returns im Try-Block werden zu Result.ok() gewrappt
fn wrap_returns_in_try_block(&self, block: &mut Block) {
    for statement in &mut block.statements {
        match statement {
            Statement::Return(return_stmt) => {
                if let Some(value) = return_stmt.value.take() {
                    // Wrap in Result.ok()
                    let wrapped = Expression::Call {
                        callee: Box::new(Expression::Member {
                            object: Box::new(Expression::Identifier("Result".to_string())),
                            member: "ok".to_string(),
                        }),
                        args: vec![value],
                    };
                    return_stmt.value = Some(wrapped);
                }
            }
            // ... rekursiv für verschachtelte Blöcke
        }
    }
}
```

**Beispiel:**
```velin
// Vorher:
return someValue;

// Nachher:
return Result.ok(someValue);
```

#### Schritt 3: Catch-Blocks zu if-Statements

```rust
// Prüfe ob Result ein Fehler ist
let is_err_check = Expression::Call {
    callee: Box::new(Expression::Member {
        object: Box::new(Expression::Identifier("__try_result".to_string())),
        member: "isErr".to_string(),
    }),
    args: Vec::new(),
};

// Hole Fehler-Wert
let get_error = Expression::Call {
    callee: Box::new(Expression::Member {
        object: Box::new(Expression::Identifier("__try_result".to_string())),
        member: "err".to_string(),
    }),
    args: Vec::new(),
};
```

**Beispiel:**
```velin
// Vorher:
catch (err) {
    handleError(err);
}

// Nachher:
if __try_result.isErr() {
    let err = __try_result.err().unwrap();
    handleError(err);
}
```

---

## Erweiterte Features

### Multiple Catch-Blocks

**Vorher (Sugar):**
```velin
try {
    return processData();
} catch (err: ValidationError) {
    handleValidationError(err);
} catch (err: NetworkError) {
    handleNetworkError(err);
} catch (err) {
    handleGenericError(err);
}
```

**Nachher (Desugared):**
```velin
let __try_result = (|| {
    return Result.ok(processData());
})();

if __try_result.isErr() {
    let __error = __try_result.err().unwrap();
    
    // Match auf Error-Typ
    match __error {
        ValidationError(err) => {
            handleValidationError(err);
        }
        NetworkError(err) => {
            handleNetworkError(err);
        }
        _ => {
            handleGenericError(__error);
        }
    }
}
```

**Implementierung:**
- Bei mehreren typisierten Catch-Blocks wird ein `match` Statement erstellt
- Bei einem einzelnen Catch-Block wird ein einfaches `if` Statement verwendet

### Finally-Blocks

**Vorher (Sugar):**
```velin
try {
    return openFile();
} catch (err) {
    log.error(err);
} finally {
    closeResources();
}
```

**Nachher (Desugared):**
```velin
let __try_result = (|| {
    return Result.ok(openFile());
})();

if __try_result.isErr() {
    let err = __try_result.err().unwrap();
    log.error(err);
}

// Finally-Block wird immer ausgeführt
(|| {
    closeResources();
})();
```

**Implementierung:**
- Finally-Block wird in Lambda gewrappt
- Lambda wird nach Try-Catch ausgeführt (immer)

### Verschachtelte Try-Catch

**Vorher (Sugar):**
```velin
try {
    try {
        return innerOperation();
    } catch (err) {
        handleInnerError(err);
    }
} catch (err) {
    handleOuterError(err);
}
```

**Nachher (Desugared):**
```velin
// Äußeres Try-Catch
let __try_result = (|| {
    // Inneres Try-Catch (bereits desugared)
    let __inner_try_result = (|| {
        return Result.ok(innerOperation());
    })();
    
    if __inner_try_result.isErr() {
        let err = __inner_try_result.err().unwrap();
        handleInnerError(err);
    }
    
    return Result.ok(());
})();

if __try_result.isErr() {
    let err = __try_result.err().unwrap();
    handleOuterError(err);
}
```

**Wichtig:** Verschachtelte Try-Catch-Blöcke werden rekursiv desugared.

---

## Expression-Desugaring

Der Pass desugared auch Expressions, die Blöcke enthalten können:

### Lambda-Expressions

```rust
Expression::Lambda { body, .. } => {
    if let Expression::Block(block) = body.as_mut() {
        self.desugar_block(block);
    } else {
        self.desugar_expression(body);
    }
}
```

### Verschachtelte Expressions

Alle verschachtelten Expressions werden rekursiv desugared:
- `Expression::Call` - Funktionsaufrufe
- `Expression::If` - If-Expressions
- `Expression::Lambda` - Lambda-Funktionen
- `Expression::Block` - Block-Expressions
- etc.

---

## Datenfluss

### Input

```rust
Program {
    items: [
        Function {
            body: Block {
                statements: [
                    Try {
                        try_block: Block { ... },
                        catch_blocks: [ ... ],
                        finally_block: Some(Block { ... })
                    }
                ]
            }
        }
    ]
}
```

### Output

```rust
Program {
    items: [
        Function {
            body: Block {
                statements: [
                    Let { name: "__try_result", ... },
                    If { condition: isErr, ... },
                    Expression { expression: finally_call }
                ]
            }
        }
    ]
}
```

---

## Integration in Compiler-Pipeline

### Pass-Reihenfolge

```
1. AutoFixPass      → Korrigiert Source-Code
2. ParserPass       → Erstellt AST
3. DesugaringPass   → Transformiert Sugar-Syntax (HIER!)
4. CodeOrderingPass → Sortiert Items
5. TypeCheckPass    → Type-Checking
...
```

### CompilationContext-Modifikationen

Der DesugaringPass modifiziert:

- ✅ `context.program` - Transformiert AST direkt
- ❌ `context.errors` - Keine Fehler (nur Transformation)
- ❌ `context.source_map` - Keine Änderungen

### Wichtig

**Der DesugaringPass ist immer aktiv** und läuft nach dem ParserPass, aber vor dem TypeCheckPass.

---

## Beispiele

### Beispiel 1: Einfaches Try-Catch

**Input:**
```velin
fn processData(): string {
    try {
        return db.query("SELECT * FROM users");
    } catch (err) {
        return "Error: " + err.message;
    }
}
```

**Output (nach Desugaring):**
```velin
fn processData(): string {
    let __try_result = (|| {
        return Result.ok(db.query("SELECT * FROM users"));
    })();
    
    if __try_result.isErr() {
        let err = __try_result.err().unwrap();
        return Result.ok("Error: " + err.message);
    }
    
    // Unwrap Result wenn kein Fehler
    return __try_result.unwrap();
}
```

### Beispiel 2: Typisierte Catch-Blocks

**Input:**
```velin
fn fetchUser(id: string): User {
    try {
        return db.find(User, id);
    } catch (err: NotFoundError) {
        throw NotFoundError("User not found");
    } catch (err: DatabaseError) {
        throw DatabaseError("Database connection failed");
    }
}
```

**Output (nach Desugaring):**
```velin
fn fetchUser(id: string): User {
    let __try_result = (|| {
        return Result.ok(db.find(User, id));
    })();
    
    if __try_result.isErr() {
        let __error = __try_result.err().unwrap();
        
        match __error {
            NotFoundError(err) => {
                throw NotFoundError("User not found");
            }
            DatabaseError(err) => {
                throw DatabaseError("Database connection failed");
            }
            _ => {
                throw __error;
            }
        }
    }
    
    return __try_result.unwrap();
}
```

### Beispiel 3: Mit Finally

**Input:**
```velin
fn processFile(path: string): string {
    let file = openFile(path);
    try {
        return readFile(file);
    } catch (err) {
        log.error(err);
        return "";
    } finally {
        closeFile(file);
    }
}
```

**Output (nach Desugaring):**
```velin
fn processFile(path: string): string {
    let file = openFile(path);
    
    let __try_result = (|| {
        return Result.ok(readFile(file));
    })();
    
    if __try_result.isErr() {
        let err = __try_result.err().unwrap();
        log.error(err);
        return "";
    }
    
    // Finally-Block wird immer ausgeführt
    (|| {
        closeFile(file);
    })();
    
    return __try_result.unwrap();
}
```

---

## API-Referenz

### DesugaringPass::new()

Erstellt eine neue DesugaringPass-Instanz:

```rust
let pass = DesugaringPass::new();
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

### Interne Methoden

- `desugar_program()` - Desugared gesamtes Programm
- `desugar_item()` - Desugared einzelnes Item
- `desugar_block()` - Desugared Block
- `desugar_try_statement()` - Transformiert Try-Statement
- `desugar_expression()` - Desugared Expression
- `wrap_returns_in_try_block()` - Wrappt Returns in Result.ok()
- `create_catch_dispatch()` - Erstellt Catch-Handling
- `create_finally_block()` - Erstellt Finally-Handling

---

## Best Practices

### Try-Catch Verwendung

1. **Verwende typisierte Catch-Blocks:** Für bessere Fehlerbehandlung
2. **Finally für Cleanup:** Nutze Finally für Ressourcen-Cleanup
3. **Vermeide verschachtelte Try-Catch:** Kann komplex werden

### Code-Organisation

1. **Klare Fehlerbehandlung:** Jeder Catch-Block sollte einen spezifischen Zweck haben
2. **Konsistente Fehler-Typen:** Verwende konsistente Error-Typen
3. **Dokumentation:** Dokumentiere erwartete Fehler

---

## Debugging

### AST nach Desugaring anzeigen

Um den transformierten AST zu sehen:

```rust
if let Some(ref program) = context.program {
    for item in &program.items {
        if let Item::Function(func) = item {
            println!("Function: {}", func.name);
            for stmt in &func.body.statements {
                println!("  Statement: {:?}", stmt);
            }
        }
    }
}
```

### Try-Statement prüfen

Nach dem Desugaring sollten keine `Statement::Try` mehr vorhanden sein:

```rust
// Nach Desugaring sollte dies false sein
assert!(!matches!(stmt, Statement::Try(_)));
```

---

## Siehe auch

- [Pass-Verlauf](./pass-verlauf.md) - Übersicht aller Passes
- [ParserPass](./parser-pass.md) - Parsing & Modul-Auflösung
- [Type Inference](./type-inference.md) - Type-Inference System
- [Compiler Architecture](./compiler-architecture.md) - Compiler-Architektur

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.0.1 / 3.1.0
