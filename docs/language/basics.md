# VelinScript Sprachgrundlagen

Willkommen zum umfassenden Handbuch für die VelinScript-Sprachgrundlagen. VelinScript ist eine moderne, statisch typisierte Programmiersprache, die entwickelt wurde, um die Lücke zwischen High-Level-Skriptsprachen (wie Python oder TypeScript) und performanten Systemprogrammiersprachen (wie Rust) zu schließen. Sie kombiniert eine ausdrucksstarke Syntax mit strikter Typsicherheit und einer mächtigen Standardbibliothek.

Dieses Dokument führt Sie detailliert durch die Kernkonzepte der Sprache, von einfachen Datentypen über komplexe Kollektionen bis hin zu fortgeschrittenen Mustern wie Pattern Matching und funktionaler Programmierung.

---

## Inhaltsverzeichnis

1.  [Einführung und Typsystem](#1-einführung-und-typsystem)
2.  [Variablen und Konstanten](#2-variablen-und-konstanten)
3.  [Kollektionen im Detail](#3-kollektionen-im-detail)
    *   [Listen (List<T>)](#listen-listt)
    *   [Maps (Map<K,V>)](#maps-mapkv)
4.  [Funktionen, Closures und Lambdas](#4-funktionen-closures-und-lambdas)
5.  [Kontrollfluss und Pattern Matching](#5-kontrollfluss-und-pattern-matching)
6.  [Fehlerbehandlung](#6-fehlerbehandlung)
7.  [Objektorientierte Konzepte (Structs & Enums)](#7-objektorientierte-konzepte)
8.  [Best Practices](#8-best-practices)

---

## 1. Einführung und Typsystem

VelinScript ist **statisch typisiert**, was bedeutet, dass Variablentypen zur Kompilierzeit bekannt sein müssen. Dies hilft, Fehler frühzeitig zu erkennen und ermöglicht dem Compiler, hochgradig optimierten Code zu generieren. Dennoch bietet die Sprache durch **Typinferenz** oft das Gefühl einer dynamischen Sprache – Sie müssen Typen nicht immer explizit angeben.

### Basistypen

Die Sprache stellt folgende primitive Typen zur Verfügung:

*   **`string`**: UTF-8 kodierte Zeichenketten.
    ```velin
    let message = "Hallo Welt";
    let multiline = "Zeile 1\nZeile 2";
    ```
*   **`number`**: 64-Bit Fließkommazahlen (f64). Es gibt keinen separaten Integer-Typ für Anwendungslogik, was die Mathematik vereinfacht.
    ```velin
    let count = 42;
    let pi = 3.14159;
    ```
*   **`boolean`**: Wahrheitswerte `true` oder `false`.
    ```velin
    let isActive = true;
    ```
*   **`void`**: Repräsentiert das Fehlen eines Wertes (z.B. bei Funktionen, die nichts zurückgeben).
*   **`null`**: Repräsentiert explizit "kein Wert". In VelinScript ist `null` jedoch strikt vom Typensystem getrennt (Null-Safety), außer bei `Optional`-Typen.
*   **`any`**: Ein "Escape-Hatch" aus dem Typsystem. Variablen vom Typ `any` können jeden Wert annehmen. **Warnung:** Die Nutzung von `any` deaktiviert die Typprüfung für diese Variable und sollte sparsam verwendet werden.

### Generics

VelinScript unterstützt Generics, um wiederverwendbaren Code für verschiedene Typen zu schreiben.

```velin
// Eine generische Identitätsfunktion
fn identity<T>(value: T): T {
    return value;
}

let s = identity<string>("Test");
let n = identity<number>(123);
```

---

## 2. Variablen und Konstanten

Variablen werden mit `let` deklariert. VelinScript unterscheidet (noch) nicht strikt zwischen `const` und `let` auf Syntaxebene wie JavaScript, aber der Compiler analysiert die Verwendung.

```velin
// Typinferenz: Compiler erkennt 'number'
let x = 10;

// Explizite Typangabe
let y: string = "Explizit";

// Spätere Zuweisung (Initialisierung ist Pflicht vor Nutzung)
let z: boolean;
z = true;
```

---

## 3. Kollektionen im Detail

VelinScript glänzt durch seine mächtigen eingebauten Kollektionen, die eine funktionale Arbeitsweise unterstützen.

### Listen (`List<T>`)

Listen sind geordnete, dynamische Arrays. Alle Elemente müssen denselben Typ haben (oder einen gemeinsamen Basistyp).

**Erstellung:**
```velin
let numbers: List<number> = [1, 2, 3, 4, 5];
let names = ["Alice", "Bob"]; // Typ List<string> inferiert
let empty: List<string> = [];
```

**Zugriff und Modifikation:**
```velin
// Index-Zugriff (0-basiert)
let first = names[0];

// Hinzufügen am Ende
names.push("Charlie");

// Entfernen
// (Hinweis: Aktuell über Filter oder spezifische Stdlib-Methoden)
```

**Funktionale Operationen:**
Das wahre Potenzial entfaltet sich durch Methoden wie `map`, `filter` und `reduce`.

```velin
let rawData = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 1. Filtern: Nur gerade Zahlen
let evens = rawData.filter(|n| n % 2 == 0);

// 2. Mappen: Quadrieren
let squares = evens.map(|n| n * n);

// 3. Reduzieren: Summe bilden
let sum = squares.reduce(|acc, val| acc + val, 0);

log.info("Summe der Quadrate gerader Zahlen: " + sum);
```

**Automatische Parallelisierung:**
Ein besonderes Feature von VelinScript ist die **Auto-Parallelisierung**. Wenn eine Liste eine bestimmte Größe überschreitet (konfigurierbar, default > 1000 Elemente), führen Methoden wie `map` und `filter` ihre Arbeit automatisch auf mehreren CPU-Kernen aus. Sie müssen Ihren Code dafür nicht ändern.

```velin
// Simulation einer großen Datenmenge
let bigData = range(0, 10000); 

// Dieser Filter läuft automatisch parallel im Hintergrund
let heavyResult = bigData.filter(|n| {
    // Teure Berechnung simulieren
    return math.isPrime(n); 
});
```

### Maps (`Map<K, V>`)

Maps sind Schlüssel-Wert-Speicher (HashMaps). Schlüssel müssen eindeutig sein.

**Erstellung:**
```velin
// Map-Literal
let config: Map<string, string> = {
    "host": "localhost",
    "port": "8080",
    "env": "dev"
};

// Leere Map
let cache: Map<string, any> = {};
```

**Methoden:**
*   `insert(key, value)`: Fügt ein Paar hinzu oder überschreibt es.
*   `get(key)`: Gibt einen `Optional<V>` zurück.
*   `contains(key)`: Prüft auf Existenz (`boolean`).
*   `remove(key)`: Löscht einen Eintrag.
*   `keys()`: Gibt eine Liste aller Schlüssel.
*   `values()`: Gibt eine Liste aller Werte.

**Beispiel:**
```velin
let scores: Map<string, number> = {};

scores.insert("Alice", 100);
scores.insert("Bob", 85);

// Sicherer Zugriff
let charlieScore = scores.get("Charlie"); // ist null/None

if (scores.contains("Alice")) {
    log.info("Alice hat Punkte: " + scores.get("Alice"));
}

// Iteration über Maps
for (entry in scores) {
    // entry hat .key und .value
    log.info("Spieler " + entry.key + ": " + entry.value);
}
```

---

## 4. Funktionen, Closures und Lambdas

Funktionen sind "First-Class Citizens" in VelinScript. Sie können Variablen zugewiesen und als Argumente übergeben werden.

### Normale Funktionen
```velin
fn add(a: number, b: number): number {
    return a + b;
}

// Kurzform für einzeilige Rückgaben (implizit return)
fn multiply(a: number, b: number) => a * b;
```

### Optionale Parameter und Default-Werte
```velin
fn greet(name: string, greeting: string = "Hallo") {
    log.info(greeting + ", " + name + "!");
}

greet("Max"); // "Hallo, Max!"
greet("Lisa", "Moin"); // "Moin, Lisa!"
```

### Lambdas (Anonyme Funktionen)
Lambdas sind besonders kompakt und nützlich für Callbacks.

**Syntax:** `|param1, param2| expression` oder `|...| { block }`

```velin
let numbers = [1, 2, 3];

// Lambda als Argument
numbers.map(|n| n * 2);

// Lambda in Variable speichern
let doubler = |n| n * 2;
log.info(doubler(5)); // 10
```

### Closures
Closures sind Lambdas, die Variablen aus ihrem definierenden Scope "fangen" (capture).

```velin
fn createAdder(base: number): fn(number) -> number {
    // Die zurückgegebene Funktion "merkt" sich 'base'
    return |n| n + base;
}

let addFive = createAdder(5);
let addTen = createAdder(10);

log.info(addFive(2)); // 7
log.info(addTen(2));  // 12
```
Dies ist extrem mächtig für Konfigurationen, Event-Handler oder funktionale Komposition.

---

## 5. Kontrollfluss und Pattern Matching
 
Neben Standard-Konstrukten wie `if/else`, `for` und `while` bietet VelinScript modernes **Pattern Matching**.
 
### Standard-Kontrollstrukturen (Optional Parentheses)
 
Seit Version 3.0 sind Klammern um Bedingungen in `if`, `for` und `while` optional, ähnlich wie in Rust oder Swift.
 
**If / Else:**
```velin
let x = 10;
 
// Mit Klammern (klassisch)
if (x > 5) {
    log.info("Groß");
}
 
// Ohne Klammern (modern)
if x > 5 {
    log.info("Groß");
} else {
    log.info("Klein");
}
```
 
**Schleifen:**
```velin
// For-Schleife über Range
for i in 0..10 {
    log.info(i);
}
 
// While-Schleife
while x > 0 {
    x = x - 1;
}
```
 
### Das `match`-Statement
`match` ist wie ein `switch` auf Steroiden. Es prüft einen Wert gegen eine Reihe von Mustern und führt den ersten passenden Zweig aus. Der Compiler prüft (bei Enums), ob alle Fälle abgedeckt sind ("Exhaustiveness Check").

**Grundlegendes Matching:**
```velin
let status = 404;

match status {
    200 => log.info("Alles OK"),
    404 => log.error("Nicht gefunden"),
    500 => log.error("Server Fehler"),
    _   => log.warn("Unbekannter Status") // Wildcard fängt alles andere
}
```

**Matching mit Bereichen und Logik:**
```velin
let age = 25;

match age {
    0..12  => log.info("Kind"),
    13..17 => log.info("Teenager"),
    18..65 => log.info("Erwachsener"),
    _      => log.info("Senior")
}
```

**Matching mit Guards:**
Sie können zusätzliche `if`-Bedingungen (Guards) an Muster anhängen.

```velin
let pair = [5, 0];

match pair {
    [x, 0] => log.info("Auf der X-Achse bei " + x),
    [0, y] => log.info("Auf der Y-Achse bei " + y),
    [x, y] if x == y => log.info("Auf der Diagonalen"),
    _ => log.info("Irgendwo anders")
}
```

**Destructuring:**
`match` kann komplexe Objekte zerlegen.

```velin
struct Response { status: number, body: string, error: string }

let res = Response { status: 400, body: "", error: "Invalid Input" };

match res {
    Response { status: 200, body } => {
        log.info("Erfolg: " + body);
    },
    Response { status, error } if status >= 400 => {
        log.error("Fehler " + status + ": " + error);
    },
    _ => log.warn("Unbekannte Antwort")
}
```

---

## 6. Fehlerbehandlung

VelinScript unterscheidet zwischen **wiederherstellbaren Fehlern** (z.B. Datei nicht gefunden) und **kritischen Fehlern** (z.B. Out of Memory, Logikfehler).

### Kritische Fehler (`throw`)
Mit `throw` wird die Ausführung sofort abgebrochen und der Fehlerstack hochgereicht. Dies sollte für unerwartete Zustände genutzt werden.

```velin
fn divide(a: number, b: number): number {
    if (b == 0) {
        throw Error("Division durch Null ist nicht erlaubt!");
    }
    return a / b;
}
```

### Wiederherstellbare Fehler (`Result` & `Optional`)
Für erwartbare Fehler (z.B. Datenbankeintrag fehlt) ist es besser, Typen zu verwenden, die das Fehlen oder den Fehler ausdrücken.

**Optional:** Ein Wert kann da sein oder `null`.
```velin
fn findUser(id: string): User? { // Kurzform für Optional<User>
    let u = db.find(User, id);
    return u; // Kann null sein
}

// Verwendung
let user = findUser("123");
if (user) {
    // Compiler weiß hier: user ist nicht null
    log.info(user.name);
} else {
    log.info("Kein User gefunden");
}
```

**Result:** Ein Wert oder ein Fehler (explizite Fehlerbehandlung).
```velin
fn parseNumber(input: string): Result<number, string> {
    // ... Parsing-Logik
    if (isValid) {
        return Result.ok(parsedValue);
    } else {
        return Result.err("Invalid number");
    }
}

// Verwendung mit Result
let result = parseNumber("42");
if (result.isOk()) {
    let value = result.unwrap();
} else {
    let error = result.unwrapErr();
}
```

**try-catch-finally (Syntaktischer Zucker)** ✅ (Version 3.0.1)
`try-catch-finally` ist syntaktischer Zucker, der automatisch in `Result`-basiertes Error-Handling desugared wird:

```velin
// Einfacher try-catch
try {
    let result = db.query("SELECT * FROM users");
    return result;
} catch (err) {
    log.error("Database error: " + err.message);
    return [];
}

// Mehrere catch-Blöcke mit Typ-Dispatch
try {
    return processData(data);
} catch (err: ValidationError) {
    handleValidationError(err);
} catch (err: NetworkError) {
    handleNetworkError(err);
} catch (err) {
    handleGenericError(err);
}

// Mit finally-Block
try {
    return openFile(path);
} catch (err) {
    log.error(err.message);
} finally {
    closeResources(); // Wird immer ausgeführt
}
```

**Wichtige Hinweise:**
- Jedes `return` im try-Block wird automatisch in `Result.ok(...)` gewrappt
- Mehrere catch-Blöcke mit spezifischen Fehlertypen werden zu `match`-Statements desugared
- Der finally-Block wird immer ausgeführt, unabhängig von Erfolg oder Fehler

---

## 7. Objektorientierte Konzepte

Obwohl VelinScript stark funktional geprägt ist, bietet es Structs für die Datenmodellierung.

### Structs
Structs sind benannte Datencontainer.

```velin
// Definition
struct Product {
    id: string,
    
    // Validierungs-Decorators (siehe Backend-Doku)
    @Validate(min: 0)
    price: number,
    
    name: string,
    tags: List<string>
}

// Instanziierung
let p = Product {
    id: "p1",
    price: 19.99,
    name: "T-Shirt",
    tags: ["kleidung", "sommer"]
};

// Zugriff
log.info(p.name);
```

### Enums
Enums können einfache Konstanten oder komplexe Datenträger sein (Algebraic Data Types).

```velin
enum PaymentMethod {
    Cash,
    CreditCard(string), // Speichert Kartennummer
    PayPal(string)      // Speichert Email
}

let pay = PaymentMethod.PayPal("user@example.com");

// Enums verarbeiten mit match
match pay {
    PaymentMethod.Cash => log.info("Zahlt bar"),
    PaymentMethod.CreditCard(num) => log.info("Karte: " + num),
    PaymentMethod.PayPal(email) => log.info("PayPal: " + email)
}
```

---

## 8. Best Practices

Um sauberen und performanten VelinScript-Code zu schreiben, beachten Sie folgende Richtlinien:

1.  **Immutability bevorzugen:** Versuchen Sie, Daten nicht zu verändern, sondern neue Daten zu erzeugen (z.B. `map` statt `for`-Loop mit Mutation). Das macht Code sicherer und leichter parallelisierbar.
2.  **Typen nutzen:** Vermeiden Sie `any`. Nutzen Sie Generics und Structs, um Ihre Domäne zu modellieren.
3.  **Kleine Funktionen:** Schreiben Sie kleine, fokussierte Funktionen, die genau eine Sache tun.
4.  **Validierung an der Quelle:** Nutzen Sie `@Validate` an Structs, um ungültige Daten gar nicht erst ins System zu lassen.
5.  **Verwenden Sie die Stdlib:** Bevor Sie eine Hilfsfunktion schreiben, prüfen Sie die Module `utils`, `string`, `math` oder `collections`. Oft gibt es schon eine optimierte Lösung.

---

*Ende der Sprachgrundlagen-Dokumentation. Für tiefergehende Informationen zu spezifischen APIs lesen Sie bitte die entsprechenden Backend- oder Security-Guides.*
