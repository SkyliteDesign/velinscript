# Tutorial 1: VelinScript Basics

Dieses Tutorial führt dich durch die Grundlagen von VelinScript.

## Variablen und Typen

### Variablen deklarieren

```velin
// Type Inference
let name = "John";
let age = 30;
let active = true;

// Explizite Typen
let email: string = "john@example.com";
let score: number = 95.5;
let isAdmin: boolean = false;

// Mutable Variablen
let mut counter = 0;
counter = counter + 1;
```

### Typen

VelinScript hat folgende primitive Typen:

- `string` - Zeichenketten
- `number` - Fließkommazahlen
- `boolean` - Wahrheitswerte
- `void` - Kein Wert
- `null` - Null-Wert

## Funktionen

### Einfache Funktionen

```velin
fn greet(name: string): string {
    return "Hello, " + name + "!";
}

fn add(a: number, b: number): number {
    return a + b;
}
```

### Funktionen ohne Parameter

```velin
fn getVersion(): string {
    return "0.1.0";
}
```

### Funktionen ohne Rückgabewert

```velin
fn printMessage(msg: string): void {
    // Ausgabe (später implementiert)
}
```

> **Tipp:** Nutze `@VelinAutoDoc` über deinen Funktionen, um automatisch Dokumentation zu generieren. Mehr dazu in [Tutorial 8](tutorial-8-intelligence.md).

```velin
/// Gibt eine Begrüßung zurück
@VelinAutoDoc
fn greet(name: string): string {
    return "Hello, " + name + "!";
}
```

## Structs

### Struct definieren

```velin
struct User {
    id: string,
    name: string,
    email: string,
    age: number,
}
```

### Struct verwenden

```velin
let user = User {
    id: "123",
    name: "John Doe",
    email: "john@example.com",
    age: 30,
};

// Zugriff auf Felder
let userName = user.name;
```

## Kontrollstrukturen

### If-Else

```velin
if (age >= 18) {
    return "Adult";
} else {
    return "Minor";
}
```

### For-Loops

```velin
let numbers = [1, 2, 3, 4, 5];
for (num in numbers) {
    print(num);
}
```

### While-Loops

```velin
let mut counter = 0;
while (counter < 10) {
    counter = counter + 1;
}
```

### Match

```velin
match (status) {
    "pending" => {
        return "Waiting";
    },
    "active" => {
        return "Running";
    },
    _ => {
        return "Unknown";
    },
}
```

## Collections

### Lists

```velin
let numbers: List<number> = [1, 2, 3, 4, 5];
let first = numbers[0];
let users: List<User> = db.findAll(User);
```

### Maps

```velin
let config: Map<string, string> = Map();
config["host"] = "localhost";
config["port"] = "8080";
let host = config["host"];
```

## Nächste Schritte

- [Tutorial 2: APIs](tutorial-2-apis.md) - REST API Entwicklung
- [Tutorial 3: Security](tutorial-3-security.md) - Security Features
- [Tutorial 4: Database](tutorial-4-database.md) - Database Integration
- [Tutorial 8: Intelligence](tutorial-8-intelligence.md) - AutoDoc, AutoTest & mehr
