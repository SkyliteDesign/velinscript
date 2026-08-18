# String Interpolation in VelinScript

VelinScript unterstützt String-Interpolation in **doppelten** Anführungszeichen. Der stabile Rust/Axum-Pfad erzeugt `format!` (nicht String-`+`).

## Syntax

Interpolation verwendet geschweifte Klammern `{}` in Strings mit doppelten Anführungszeichen. **Einfache** Anführungszeichen interpolieren nicht. JSON-artige Literale wie `"{"` oder `"{}"` sind normale Strings, keine Format-Strings.

```velin
let name = "John";
let age = 30;
let message = "Hello, {name}! You are {age} years old.";
```

## Einfache Variablen

Die einfachste Form der Interpolation ist das Einfügen von Variablen:

```velin
let userName = "Alice";
let greeting = "Welcome, {userName}!";
// Ergebnis: "Welcome, Alice!"
```

## Ausdrücke

Du kannst auch komplexere Ausdrücke innerhalb der geschweiften Klammern verwenden:

```velin
let x = 10;
let y = 20;
let result = "Sum: {x + y}";
// Ergebnis: "Sum: 30"

let price = 19.99;
let quantity = 3;
let total = "Total: {price * quantity}";
// Ergebnis: "Total: 59.97"
```

## Funktionsaufrufe

Funktionsaufrufe sind ebenfalls möglich:

```velin
fn getFullName(first: string, last: string): string {
    return first + " " + last;
}

let firstName = "John";
let lastName = "Doe";
let fullName = "Name: {getFullName(firstName, lastName)}";
// Ergebnis: "Name: John Doe"
```

## Methodenaufrufe

Methodenaufrufe auf Objekten:

```velin
let list = List<number>([1, 2, 3]);
let message = "List has {list.length()} items";
// Ergebnis: "List has 3 items"
```

## Mehrfache Interpolationen

Du kannst mehrere Interpolationen in einem String verwenden:

```velin
let product = "Laptop";
let price = 999.99;
let discount = 0.1;
let finalPrice = price * (1 - discount);
let message = "Product: {product}, Original: {price}, Discount: {discount * 100}%, Final: {finalPrice}";
// Ergebnis: "Product: Laptop, Original: 999.99, Discount: 10%, Final: 899.991"
```

## Escaping

Um geschweifte Klammern als Literale zu verwenden, musst du sie escapen:

```velin
// Escaping mit Backslash
let message = "This is a literal brace: \\{";
// Ergebnis: "This is a literal brace: {"

// Oder verwende einfache Anführungszeichen für Strings ohne Interpolation
let literal = 'This is {not interpolated}';
```

## Multi-line Strings

Format-Strings funktionieren auch mit Multi-line Strings:

```velin
let userId = "123";
let sql = "
    SELECT * FROM users
    WHERE id = {userId}
    AND active = true
";
```

## Best Practices

1. **Verwende Format-Strings für bessere Lesbarkeit:**
   ```velin
   // Gut
   let message = "Hello, {name}!";
   
   // Weniger gut
   let message = "Hello, " + name + "!";
   ```

2. **Komplexe Ausdrücke in Variablen auslagern:**
   ```velin
   // Gut
   let total = price * quantity;
   let message = "Total: {total}";
   
   // Auch OK, aber weniger lesbar
   let message = "Total: {price * quantity}";
   ```

3. **Verwende Format-Strings für SQL-Queries:**
   ```velin
   let query = "
       SELECT * FROM products
       WHERE category = {category}
       AND price <= {maxPrice}
   ";
   ```

## Compilation

Im IR-Pfad nach Rust werden Format-Strings zu `format!` kompiliert (3.5.1). Andere Targets erzeugen sprachübliche Konkatenation (`str()`, `String.valueOf`, `fmt.Sprintf`, …) — ohne gleichwertigen Runtime-Nachweis außer Rust/Axum.

```rust
// VelinScript
let message = "Hello, {name}!";

// Kompiliert zu:
let message = format!("Hello, {}!", name);
```

## Type Safety

Der Type Checker prüft, dass alle Ausdrücke innerhalb der geschweiften Klammern gültig sind und dass sie Typen haben, die als String formatiert werden können (implementieren `Display` Trait in Rust).

## Beispiele

### API Response

```velin
fn createResponse(userId: string, status: string): string {
    return "{ \"userId\": \"{userId}\", \"status\": \"{status}\" }";
}
```

### Logging

```velin
fn logInfo(component: string, message: string): void {
    let logMessage = "[{component}] {message}";
    // Log implementation
}
```

### Error Messages

```velin
fn createError(operation: string, error: string): string {
    return "Error in {operation}: {error}";
}
```
