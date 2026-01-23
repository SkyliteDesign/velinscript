# Borrow Checker - Ownership & Borrowing System

**Version:** 3.0.1 / 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Datum:** 2026-01-30

---

## Übersicht

Der Borrow Checker implementiert ein Ownership-System ähnlich Rust für Memory-Safety-Garantien in VelinScript.

## Features

- ✅ **Ownership-Tracking** - Verfolgt wer einen Wert besitzt
- ✅ **Borrow-Checks** - Prüft ob Referenzen gültig sind
- ✅ **Lifetime-Analyse** - Analysiert wie lange Werte leben
- ✅ **Memory-Safety** - Verhindert Use-After-Free, Double-Free

---

## Ownership-Typen

### Owned

Variable besitzt den Wert (move semantics):

```velin
fn take_ownership(data: string) {
    // data wird moved (owned)
    // Nach dem Aufruf ist data nicht mehr gültig
}
```

### Borrowed

Immutable Referenz (`&T`):

```velin
fn process(data: &string) {
    // data ist eine immutable Referenz
    // data kann nicht modifiziert werden
}
```

### BorrowedMut

Mutable Referenz (`&mut T`):

```velin
fn modify(data: &mut string) {
    // data ist eine mutable Referenz
    // data kann modifiziert werden
}
```

### Shared

Shared ownership (`Arc<T>` / `Rc<T>`):

```velin
fn shared_data(data: shared<string>) {
    // data wird geteilt (Arc/Rc)
    // Mehrere Referenzen möglich
}
```

### Copy

Copy-Semantik (primitive types):

```velin
fn copy_value(x: number) {
    // number ist Copy, wird kopiert
    // x bleibt nach dem Aufruf gültig
}
```

---

## Borrow-Fehler

Der Borrow Checker erkennt folgende Fehler:

### Use-After-Move

```velin
fn test() {
    let x = "hello";
    let y = x;  // Move
    let z = x;  // Error: use after move
}
```

### Multiple Mutable Borrows

```velin
fn test() {
    let mut x = "hello";
    let y = &mut x;  // Mutable borrow
    let z = &mut x;  // Error: multiple mutable borrows
}
```

### Immutable Borrow Mutation

```velin
fn test() {
    let x = "hello";
    let y = &x;  // Immutable borrow
    *y = "world";  // Error: cannot mutate immutable borrow
}
```

### Lifetime-Verletzungen

```velin
fn test() {
    let x = "hello";
    let y = &x;  // Borrow
    drop(x);     // Error: lifetime outlives scope
    use(y);
}
```

---

## Integration

Der Borrow Checker ist vollständig in den Type Checker integriert:

1. **Type Checking** - Standard Type Checking wird durchgeführt
2. **AST → IR** - IRBuilder konvertiert AST zu IR
3. **Borrow Checking** - BorrowChecker prüft IR auf Borrow-Verletzungen
4. **Fehler-Reporting** - Alle Fehler werden kombiniert und gemeldet

**Implementierung:** `compiler/src/passes/type_check.rs`

---

## Dateien

- `compiler/src/borrow/mod.rs` - Modul-Definition
- `compiler/src/borrow/ownership.rs` - Ownership-System
- `compiler/src/borrow/lifetime.rs` - Lifetime-System
- `compiler/src/borrow/checker.rs` - Borrow Checker

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.0.1
