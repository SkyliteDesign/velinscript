# Type-Inference System

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

Das VelinScript Type-Inference System bietet intelligente Typ-Inferenz für verschiedene Code-Konstrukte, einschließlich automatischer String-Konvertierung, Result-Type-Inference und verbesserter Type-Inference für desugared Code.

---

## Features

### 1. Type::Any Member-Access mit automatischer Type-Inference

**Status:** ✅ Implementiert in `compiler/src/type_checker/checker.rs`

Das System unterstützt automatische Type-Inference basierend auf Member-Namen für `Type::Any` Objekte. Dies ermöglicht flexible Member-Zugriffe mit intelligenter Typ-Erkennung.

#### Unterstützte Member-Patterns

**String-ähnliche Methoden:**
- `length` → `Number`
- `toUpperCase`, `toLowerCase`, `trim`, `substring`, `replace`, `split` → `String`
- `startsWith`, `endsWith`, `contains`, `isEmpty` → `Boolean`

**List-ähnliche Methoden:**
- `size`, `len` → `Number`
- `push`, `pop`, `clear`, `remove`, `add`, `insert` → `Void`
- `join` → `String`
- `map`, `filter`, `slice`, `chunk` → `List<Any>`
- `find`, `get` → `Optional<Any>`
- `reduce` → `Any`
- `sort`, `reverse` → `Void`

**Map-ähnliche Methoden:**
- `set`, `insert`, `put`, `delete` → `Void`
- `has`, `containsKey` → `Boolean`
- `keys`, `values` → `List<Any>`

**Fallback:**
- Unbekannte Member → `Type::Any` (kein Fehler für maximale Flexibilität)

#### Beispiel

```velin
fn processData(data: any) {
    // Automatische Type-Inference basierend auf Member-Namen
    if (data.startsWith("http://")) {
        // data.startsWith() → Boolean (automatisch inferiert)
        return data.toUpperCase(); // → String
    }
    
    if (data.length > 0) {
        // data.length → Number
        return data.trim(); // → String
    }
    
    // Unbekannte Member geben Any zurück (kein Fehler)
    return data.unknownMethod(); // → Any
}
```

#### Implementierung

Die Implementierung befindet sich in `check_expression` für `Expression::Member`:

```rust
Type::Any => {
    // Type::Any member access with type inference based on member name
    match member.as_str() {
        "length" => Ok(Type::Number),
        "toUpperCase" | "toLowerCase" | "trim" => Ok(Type::String),
        "startsWith" | "endsWith" | "contains" => Ok(Type::Boolean),
        // ... weitere Patterns
        _ => Ok(Type::Any), // Fallback
    }
}
```

---

### 2. Result-Type Inference Verbesserung

**Status:** ✅ Implementiert in `compiler/src/type_checker/checker.rs`

Das System verbessert die Type-Inference für Result-Types durch automatische Auflösung verschachtelter Result-Types und korrekte Type-Propagation.

#### Features

- **Verschachtelte Result-Types**: Automatische Auflösung von `Result<Result<T, E>, E>` → `Result<T, E>`
- **Type-Propagation**: Korrekte Propagation von Result-Types in Variablenzuweisungen
- **Call-Expression Support**: Verbesserte Inference in Funktionsaufrufen

#### Beispiel

```velin
fn fetchUser(): Result<Result<User, string>, string> {
    // Verschachtelte Result-Types werden automatisch aufgelöst
    return Result.ok(Result.ok(User { name: "John" }));
}

fn main() {
    let result = fetchUser(); 
    // Type wird automatisch zu Result<User, string> aufgelöst
    // (nicht Result<Result<User, string>, string>)
    
    if (result.isOk()) {
        let user = result.unwrap(); // → User (korrekt inferiert)
    }
}
```

#### Implementierung

Die `resolve_result_type` Funktion löst verschachtelte Result-Types auf:

```rust
fn resolve_result_type(&self, ty: &Type) -> Type {
    match ty {
        Type::Result { ok, err } => {
            // Check if ok is itself a Result type
            match ok.as_ref() {
                Type::Result { ok: inner_ok, err: inner_err } => {
                    // Nested Result: Result<Result<T, E>, E> -> Result<T, E>
                    Type::Result {
                        ok: inner_ok.clone(),
                        err: if matches!(inner_err.as_ref(), Type::String) {
                            err.clone()
                        } else {
                            inner_err.clone()
                        },
                    }
                }
                _ => ty.clone(),
            }
        }
        _ => ty.clone(),
    }
}
```

---

### 3. Desugared Code Type Inference

**Status:** ✅ Implementiert in `compiler/src/type_checker/checker.rs`

Das System verfeinert die Typen von desugared Variablen nach dem initialen Type-Check-Pass, um bessere Type-Inference für transformierten Code zu ermöglichen.

#### Unterstützte Desugared Variablen

- **`__try_result`**: Typ wird aus dem try-Block's Return-Type abgeleitet
- **`__await_result_*`**: Typ wird aus dem await-Ausdruck abgeleitet
- **Andere `__*` Variablen**: Typ wird aus dem Wert-Ausdruck abgeleitet, wenn aktuell `Any`

#### Beispiel

```velin
fn fetchData(): Result<string, string> {
    try {
        return "success";
    } catch (e: string) {
        return Result.err("error");
    }
}

fn main() {
    // __try_result wird initial als Any registriert
    // Nach refine_desugared_types wird es zu Result<string, string> verfeinert
    let result = __try_result; // → Result<string, string> (verfeinert)
    
    if (result.isOk()) {
        let data = result.unwrap(); // → string (korrekt inferiert)
    }
}
```

#### Implementierung

Die `refine_desugared_types` Funktion läuft als dritter Pass nach dem Type-Check:

```rust
pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
    // First pass: Register definitions
    Self::register_module_definitions(&mut self.environment, &items);
    
    // Second pass: Type check
    self.check_module_content(&program.items)?;
    
    // Third pass: Refine desugared types
    self.refine_desugared_types(program)?;
    
    // ...
}
```

Die Funktion analysiert die Verwendung von desugared Variablen und verfeinert deren Typen basierend auf dem tatsächlichen Code-Kontext.

---

## Integration in den Compiler

### Pass-Reihenfolge

1. **ParserPass** - Parsing und AST-Erstellung
2. **DesugaringPass** - Transformation (erstellt desugared Variablen)
3. **TypeCheckPass** - Initial Type-Check (registriert desugared Variablen als `Any`)
4. **refine_desugared_types** - Verfeinert Typen von desugared Variablen (innerhalb TypeCheckPass)

### Type-Check-Flow

```
Expression/Statement
    ↓
check_expression / check_statement
    ↓
Type::Any Member-Access?
    ↓
Member-Name-basierte Inference
    ↓
Result-Type?
    ↓
resolve_result_type (verschachtelte Auflösung)
    ↓
Return Type
```

---

## Best Practices

### Type::Any Member-Access

- **Verwende spezifische Member-Namen**: Nutze bekannte Patterns wie `length`, `startsWith`, etc. für bessere Inference
- **Fallback zu expliziten Typen**: Wenn möglich, verwende explizite Typen statt `any` für bessere Type-Safety

### Result-Types

- **Vermeide unnötige Verschachtelung**: Das System löst verschachtelte Result-Types auf, aber explizite Typen sind klarer
- **Nutze Type-Propagation**: Das System propagiert Result-Types automatisch in Variablenzuweisungen

### Desugared Code

- **Vertraue auf automatische Verfeinerung**: Das System verfeinert Typen automatisch, manuelle Typ-Annotationen sind selten nötig
- **Verwende try-catch für bessere Inference**: try-catch Blöcke werden automatisch zu Result-Types transformiert

---

## Technische Details

### Dateien

- `compiler/src/type_checker/checker.rs` - Hauptimplementierung
  - `check_expression` - Expression Type-Checking mit Type::Any Support
  - `resolve_result_type` - Result-Type Auflösung
  - `refine_desugared_types` - Desugared Type-Verfeinerung

### Abhängigkeiten

- Keine externen Abhängigkeiten (nur Standard Rust)

---

## Siehe auch

- [Compiler Architecture](./compiler-architecture.md) - Compiler-Pass-System
- [Type System](../language/specification.md#type-system) - Vollständige Type-System Dokumentation
- [Desugaring](./compiler-architecture.md#desugaring-pass) - Desugaring Pass Details

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
