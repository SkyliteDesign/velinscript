# VelinScript Language Specification

Version 0.1.0

## Übersicht

VelinScript ist eine moderne Programmiersprache für KI-APIs. Sie kombiniert die Einfachheit von modernen Sprachen mit leistungsstarken Features für API-Entwicklung, Security und KI/ML-Integration.

## Design-Prinzipien

1. **Einfachheit** - Klare, lesbare Syntax
2. **Type Safety** - Starke Typisierung mit Type Inference
3. **API-First** - Built-in Support für REST APIs
4. **Security** - Security-Features von Anfang an
5. **KI/ML Ready** - Native Unterstützung für KI/ML-Integration

## Lexikalische Struktur

### Kommentare

```velin
// Single-line Kommentar

/*
 * Multi-line Kommentar
 * Kann mehrere Zeilen umfassen
 */
```

### Keywords

- `fn` - Funktion definieren
- `let` - Variable definieren
- `return` - Wert zurückgeben
- `if`, `else` - Bedingte Ausführung
- `for`, `while` - Schleifen
- `match` - Pattern Matching
- `struct` - Struktur definieren
- `enum` - Enumeration definieren
- `type` - Type Alias
- `pub` - Public Visibility
- `use` - Import
- `mod` - Modul
- `const`, `static` - Konstanten
- `async`, `await` - Asynchrone Programmierung
- `in` - For-Loop Iterator

### Identifikatoren

- Beginnen mit Buchstabe oder `_`
- Können Buchstaben, Zahlen und `_` enthalten
- Case-sensitive
- Beispiele: `user`, `getUser`, `UserService`, `_private`

### Literale

#### String Literale

```velin
"Hello, World!"
'Hello, World!'
"String mit \"Escapes\""
```

#### Number Literale

```velin
42
3.14
0.5
```

#### Boolean Literale

```velin
true
false
```

#### Null Literal

```velin
null
```

### Operatoren

#### Arithmetische Operatoren

- `+` - Addition
- `-` - Subtraktion
- `*` - Multiplikation
- `/` - Division
- `%` - Modulo

#### Vergleichsoperatoren

- `==` - Gleich
- `!=` - Ungleich
- `<` - Kleiner
- `>` - Größer
- `<=` - Kleiner oder gleich
- `>=` - Größer oder gleich

#### Logische Operatoren

- `&&` - Und
- `||` - Oder
- `!` - Nicht

#### Zuweisungsoperatoren

- `=` - Zuweisung

## Typen

### Primitive Typen

- `string` - Zeichenkette
- `number` - Fließkommazahl (f64)
- `boolean` - Wahrheitswert
- `void` - Kein Rückgabewert
- `null` - Null-Typ

### Collection Typen

- `List<T>` - Liste von Elementen
- `Map<K, V>` - Schlüssel-Wert-Paarung
- `Optional<T>` - Optionaler Wert (kann null sein)

### Funktionstypen

```velin
fn(string, number) -> string
```

### Tuple Typen

```velin
(string, number, boolean)
```

### Named Typen

Benutzerdefinierte Typen (Structs, Enums, Type Aliases)

## Deklarationen

### Funktionen

```velin
fn functionName(param1: type1, param2: type2): returnType {
    // Body
}
```

**Beispiele:**

```velin
// Einfache Funktion
fn greet(name: string): string {
    return "Hello, " + name;
}

// Funktion ohne Parameter
fn getVersion(): string {
    return "0.1.0";
}

// Funktion ohne Rückgabewert
fn printMessage(msg: string): void {
    // ...
}

// Async Funktion
async fn fetchData(url: string): Data {
    // ...
}
```

### Variablen

```velin
let variableName: type = value;
let mutableVariable: type = value; // mit mut für mutable
```

**Type Inference:**

```velin
let name = "John"; // Typ: string (inferiert)
let age = 30;      // Typ: number (inferiert)
let active = true; // Typ: boolean (inferiert)
```

### Structs

```velin
struct StructName {
    field1: type1,
    field2: type2,
}
```

**Beispiel:**

```velin
struct User {
    id: string,
    name: string,
    email: string,
    age: number,
}
```

### Enums

```velin
enum EnumName {
    Variant1,
    Variant2(data: type),
    Variant3(type1, type2),
}
```

**Beispiel:**

```velin
enum Status {
    Pending,
    Active,
    Inactive,
}

enum Result {
    Ok(value: string),
    Error(message: string),
}
```

### Type Aliases

```velin
type AliasName = ExistingType;
```

**Beispiel:**

```velin
type UserId = string;
type UserList = List<User>;
```

## Decorators

Decorators sind Metadaten, die Funktionen, Structs oder andere Items annotieren.

### HTTP Decorators

```velin
@GET("/api/users")
@POST("/api/users")
@PUT("/api/users/:id")
@DELETE("/api/users/:id")
@PATCH("/api/users/:id")
```

### Security Decorators

```velin
@Auth
@Role("admin")
```

### Performance Decorators

```velin
@Cache(ttl: "5m", key: "user:{id}")
```

### SEO Decorators

```velin
@SEO(title: "Product: {name}", description: "{description}")
```

### AI/ML Decorators

```velin
@AI(model: "sentiment")
```

### Testing Decorators

```velin
@test
```

**Beispiel:**

```velin
@GET("/api/users/:id")
@Auth
@Role("user")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

## Statements

### Let Statement

```velin
let name: string = "John";
let age = 30; // Type Inference
let mut counter = 0; // Mutable
```

### Return Statement

```velin
return value;
return; // void
```

### If Statement

```velin
if (condition) {
    // then block
} else {
    // else block
}
```

### For Statement

```velin
for (item in collection) {
    // body
}
```

### While Statement

```velin
while (condition) {
    // body
}
```

### Match Statement

```velin
match (value) {
    pattern1 => {
        // body
    },
    pattern2 => {
        // body
    },
}
```

## Expressions

### Literal Expressions

```velin
"string"
42
3.14
true
false
null
```

### Identifier Expressions

```velin
variableName
functionName
```

### Binary Operations

```velin
a + b
a - b
a * b
a / b
a % b
a == b
a != b
a < b
a > b
a <= b
a >= b
a && b
a || b
```

### Unary Operations

```velin
!condition
-number
```

### Function Calls

```velin
functionName(arg1, arg2)
db.find(User, id)
```

### Member Access

```velin
object.member
user.name
```

### Index Access

```velin
array[index]
list[0]
```

### Conditional Expressions

```velin
condition ? thenExpr : elseExpr
```

### Block Expressions

```velin
{
    let x = 1;
    x + 1
}
```

## Standard Library

### Database Funktionen

```velin
db.find(Entity, id)        // Findet ein Entity
db.findAll(Entity)         // Findet alle Entities
db.save(entity)            // Speichert ein Entity
db.delete(Entity, id)      // Löscht ein Entity
```

### Assert Funktionen

```velin
assert(condition)
assert(left == right)      // Wird zu assert_eq!()
assert(left != right)      // Wird zu assert_ne!()
```

## Module System

### Module Definition

```velin
mod moduleName {
    // Items
}
```

### Use Statements

```velin
use module::item;
use module::item as alias;
```

## Visibility

- `pub` - Public (exportiert)
- (kein Modifier) - Private (standard)

## Type System

### Type Inference

VelinScript unterstützt Type Inference für Variablen:

```velin
let name = "John"; // Typ: string
let age = 30;      // Typ: number
```

### Type Checking

Alle Typen werden zur Compile-Zeit geprüft:

```velin
fn add(a: number, b: number): number {
    return a + b;
}

let result = add(2, 3); // OK
let error = add("2", 3); // Type Error!
```

### Generic Types

```velin
List<User>
Map<string, number>
Optional<User>
```

## Beispiele

### Hello World

```velin
@GET("/api/hello")
fn hello(): string {
    return "Hello, VelinScript! 🚀";
}
```

### API Endpoint

```velin
struct User {
    id: string,
    name: string,
    email: string,
}

@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

### Security

```velin
@Auth
@Role("admin")
@GET("/api/admin/users")
fn getAdminUsers(): List<User> {
    return db.findAll(User);
}
```

### Testing

```velin
@test
fn testGetUser() {
    let user = getUser("123");
    assert(user.name == "John");
}
```

## Compilation

VelinScript wird zu Rust Code kompiliert:

**VelinScript:**
```velin
@GET("/api/users/:id")
fn getUser(id: string): User {
    return db.find(User, id);
}
```

**Rust Output:**
```rust
#[get("/api/users/:id")]
fn get_user(id: String) -> User {
    db.find::<User>(id).await
}
```

## Syntax-Zusammenfassung

### Funktion

```
[Decorators]
[pub] [async] fn name(params): returnType {
    statements
}
```

### Variable

```
let [mut] name[: type] = value;
```

### Struct

```
[pub] struct Name {
    [pub] field: type,
}
```

### Enum

```
[pub] enum Name {
    Variant1,
    Variant2(type),
}
```

## Bekannte Einschränkungen (v0.1.0)

- Module System: Basis-Implementierung
- Generic Constraints: Noch nicht vollständig
- Pattern Matching: Vereinfacht
- Error Handling: Basis-Implementierung
- Async/Await: Basis-Implementierung

## Zukünftige Features

- Traits/Interfaces
- Generics mit Constraints
- Erweiterte Pattern Matching
- Error Handling mit Result<T, E>
- Macros
- Package Manager
- LSP Server
