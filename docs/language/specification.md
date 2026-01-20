# VelinScript Language Specification

Version 2.6.0

## Übersicht

VelinScript ist eine moderne Programmiersprache für KI-APIs. Sie kombiniert die Einfachheit von modernen Sprachen mit leistungsstarken Features für API-Entwicklung, Security und KI/ML-Integration.

**Neu in Version 2.5**: 
- 13 neue Standard Library Module mit 117+ Funktionen
- VelinAutoDoc für automatische Dokumentationsgenerierung
- VelinPipeline für automatische Performance-Optimierung
- VelinFlow Runtime für transaktionales Flow-Management

**Neu in Version 2.6**: 
- 5 neue kritische Standard Library Module mit 50+ Funktionen
- path, url, stream, redis, tracing Module hinzugefügt
- Alle Mock-Funktionen durch echte Implementierungen ersetzt
- Verbesserte Pipeline-Optimierung mit echter Dependency-Tracking

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

/// Doc-Comment (für @VelinAutoDoc)
/// Diese Kommentare werden vom Compiler erfasst
/// und für automatische Dokumentationsgenerierung verwendet
/// 
/// @param name - Der Name des Parameters
/// @returns Eine Beschreibung des Rückgabewerts
@VelinAutoDoc
fn example(name: string): string {
    // ...
}
```

**Doc-Comments (`///`):**
- Neu in Version 2.5 ✅
- Werden als First-Class-Citizens im AST erfasst
- Werden für automatische Dokumentationsgenerierung mit `@VelinAutoDoc` verwendet
- Unterstützen Markdown-Formatierung
- Können Parameter und Rückgabewerte dokumentieren

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
- `trait` - Trait definieren (Rust-ähnlich)
- `interface` - Interface definieren (TypeScript-ähnlich)
- `impl` - Trait/Interface Implementierung

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

#### Format-Strings (String Interpolation)

Format-Strings ermöglichen die Interpolation von Ausdrücken innerhalb von String-Literalen:

```velin
let name = "John";
let message = "Hello, {name}!";
// Ergebnis: "Hello, John!"

let x = 10;
let y = 20;
let result = "Sum: {x + y}";
// Ergebnis: "Sum: 30"
```

**Syntax:**
- Format-Strings verwenden geschweifte Klammern `{}` für Interpolation
- Beliebige Ausdrücke können innerhalb der Klammern verwendet werden
- Escaping: `\{` für literal `{`, `\}` für literal `}`

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
- `Result<T, E>` - Result Type für explizite Fehlerbehandlung

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

enum Status {
    Pending,
    Active,
    Inactive,
}
```

### Result Type

Der `Result<T, E>` Type ermöglicht explizite Fehlerbehandlung:

```velin
fn parseNumber(input: string): Result<number, string> {
    // ... Parsing-Logik
    if (isValid) {
        return Result::Ok(parsedValue);
    } else {
        return Result::Error("Invalid number");
    }
}

// Result verwenden
let result = parseNumber("42");
if (result.isOk()) {
    let value = result.unwrap();
} else {
    let error = result.unwrapErr();
}
```

**Result Methoden:**
- `isOk()` - Prüft ob Result Ok ist
- `isErr()` - Prüft ob Result Error ist
- `unwrap()` - Extrahiert Ok-Wert (panics bei Error)
- `unwrapOr(default)` - Extrahiert Ok-Wert oder gibt Default zurück
- `map(fn)` - Transformiert Ok-Wert
- `mapErr(fn)` - Transformiert Error-Wert

### Traits

Traits definieren Verträge für Typen (ähnlich wie Interfaces in anderen Sprachen):

```velin
trait Serialize {
    fn toJson(): string;
    fn fromJson(json: string): Self;
}

trait Clone {
    fn clone(): Self;
}

// Trait mit generischen Parametern
trait Comparable<T> {
    fn compare(other: T): number;
}
```

### Trait Implementierungen

```velin
impl Serialize for User {
    fn toJson(): string {
        // ... Implementation
    }
    
    fn fromJson(json: string): User {
        // ... Implementation
    }
}

// Trait für mehrere Typen implementieren
impl Clone for User {
    fn clone(): User {
        // ... Implementation
    }
}
```

### Interfaces

Interfaces sind syntaktisch identisch zu Traits, bieten aber TypeScript-ähnliche Semantik:

```velin
interface IUserService {
    fn getUser(id: string): User;
    fn createUser(user: User): User;
}

// Interface implementieren
impl IUserService for UserService {
    fn getUser(id: string): User {
        // ... Implementation
    }
    
    fn createUser(user: User): User {
        // ... Implementation
    }
}
```

### Generics mit Constraints

Generics ermöglichen wiederverwendbaren Code mit Type Safety:

```velin
// Einfaches Generic
fn identity<T>(value: T): T {
    return value;
}

// Generic mit Constraint (T muss Serialize implementieren)
fn serialize<T: Serialize>(item: T): string {
    return item.toJson();
}

// Multiple Constraints
fn process<T: Serialize & Clone>(item: T): string {
    let cloned = item.clone();
    return cloned.toJson();
}

// Generic Struct
struct Container<T> {
    value: T,
}

// Generic mit Constraints in Struct
struct Cache<T: Clone> {
    data: T,
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
@describe("Test Suite Name")
@fixture("fixture-name")
@mock("TraitName")
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
    pattern2 if condition => {
        // body with guard
    },
    pattern3 | pattern4 => {
        // or pattern
    },
    _ => {
        // wildcard
    }
}
```

#### Pattern Types

- **Literal Patterns**: `"hello"`, `42`, `true`
- **Identifier Patterns**: `value` (bindet Wert an Variable)
- **Wildcard Pattern**: `_` (matched alles)
- **Range Patterns**: `0..10` (exclusive), `0..=10` (inclusive)
- **Tuple Patterns**: `(a, b, c)`
- **Struct Patterns**: `User { name, email }`
- **Enum Variant Patterns**: `Result::Ok(value)`, `Result::Error(err)`
- **Or Patterns**: `pattern1 | pattern2`

#### Pattern Guards

Pattern Guards erlauben zusätzliche Bedingungen:

```velin
match (value) {
    Ok(x) if x > 0 => "positive",
    Ok(x) if x < 0 => "negative",
    Ok(0) => "zero",
    _ => "unknown"
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

### Lambda Expressions

```velin
// Lambda mit Expression Body
let add = (a: number, b: number) => a + b;

// Lambda mit Block Body
let multiply = (a: number, b: number) => {
    let result = a * b;
    return result;
};

// Lambda mit Type Inference
let square = (x) => x * x;
```

## Standard Library

### Database Funktionen

```velin
db.find(Entity, id)        // Findet ein Entity
db.findAll(Entity)         // Findet alle Entities
db.save(entity)            // Speichert ein Entity
db.delete(Entity, id)      // Löscht ein Entity
```

### Collections Library

#### List<T>

```velin
let list = List<number>([1, 2, 3, 4, 5]);

// Transformation
let doubled = list.map((x: number) => x * 2);
let evens = list.filter((x: number) => x % 2 == 0);
let sum = list.reduce((acc: number, x: number) => acc + x, 0);

// Suche
let found = list.find((x: number) => x > 3);
let hasFive = list.contains(5);
let index = list.indexOf(3);

// Sortierung
let sorted = list.sort();
let reversed = list.reverse();

// Chunking & Slicing
let chunks = list.chunk(2);
let slice = list.slice(1, 3);
```

#### Map<K, V>

```velin
let map = Map<string, number>();

map.set("one", 1);
let value = map.get("one");
let has = map.has("one");
let keys = map.keys();
let values = map.values();
let entries = map.entries();
map.delete("one");
let size = map.size();
```

#### Set<T>

```velin
let set = Set<number>();

set.add(1);
set.remove(1);
let has = set.has(1);
let size = set.size();
let union = set1.union(set2);
let intersection = set1.intersection(set2);
let difference = set1.difference(set2);
```

### HTTP Client Library

```velin
let client = HttpClient.new();

// GET Request
let response = await client.get("https://api.example.com/users");
let data = response.json();

// POST Request
let response = await client.post(
    "https://api.example.com/users",
    { name: "John", email: "john@example.com" }
);

// PUT, DELETE, PATCH
let response = await client.put(url, body);
let response = await client.delete(url);
let response = await client.patch(url, body);

// Response Handling
let json = response.json();
let text = response.text();
let status = response.status();
```

### Rate Limiting Library

```velin
@RateLimit(
    requests: 100,
    window: "1m",
    strategy: "sliding-window",
    key: "user:{userId}"
)
@GET("/api/users")
fn getUsers(): List<User> {
    // ...
}
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
- Pattern Matching: Vereinfacht
- Async/Await: Basis-Implementierung
- Macros: Noch nicht implementiert

## Implementierte Features (v0.1.0)

✅ **Result<T, E> Type** - Explizite Fehlerbehandlung mit Result Type
✅ **Traits/Interfaces** - Polymorphismus durch Traits und Interfaces
✅ **Generics mit Constraints** - Type-safe generische Programmierung mit Trait Constraints
✅ **Erweiterte Test-Features** - @describe, @fixture, @mock Decorators
✅ **Package Manager (velin-pkg)** - Dependency Management mit velin.toml
✅ **Security Scanner (velin-security)** - Automatische Security-Vulnerability-Erkennung
✅ **LSP Server** - Language Server Protocol für IDE-Integration
✅ **VS Code Extension** - Vollständige IDE-Unterstützung

## Zukünftige Features

- Erweiterte Pattern Matching
- Macros
- Compile-time Evaluation
- Advanced Type System Features
