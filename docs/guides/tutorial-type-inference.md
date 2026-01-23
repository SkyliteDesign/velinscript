# Type-Inference und Code-Ordering Tutorial

**Version:** 3.1.0  
**Für:** Entwickler, die die neuen Type-Inference und Code-Ordering Features nutzen möchten

---

## Übersicht

Dieses Tutorial führt dich durch die neuen Type-Inference und Code-Ordering Features in VelinScript 3.1.0:

1. **Type::Any Member-Access** - Automatische Type-Inference basierend auf Member-Namen
2. **Result-Type Inference** - Verbesserte Inference für Result-Types
3. **Desugared Code Type Inference** - Automatische Type-Verfeinerung für transformierten Code
4. **Automatic Code Ordering** - Automatische Sortierung von Code basierend auf Abhängigkeiten

---

## 1. Type::Any Member-Access

### Was ist das?

Das System unterstützt automatische Type-Inference für `any` Typen basierend auf Member-Namen. Du kannst auf `any` Objekte zugreifen, und das System erkennt automatisch den Typ basierend auf dem Member-Namen.

### Beispiel

```velin
fn processData(data: any) {
    // Automatische Type-Inference
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

### Unterstützte Member-Patterns

**String-Methoden:**
- `length` → `Number`
- `toUpperCase()`, `toLowerCase()`, `trim()` → `String`
- `startsWith()`, `endsWith()`, `contains()` → `Boolean`

**List-Methoden:**
- `length`, `size`, `len` → `Number`
- `push()`, `pop()`, `clear()` → `Void`
- `map()`, `filter()` → `List<Any>`
- `find()`, `get()` → `Optional<Any>`

**Map-Methoden:**
- `get()` → `Optional<Any>`
- `set()`, `insert()`, `put()` → `Void`
- `has()`, `containsKey()` → `Boolean`
- `keys()`, `values()` → `List<Any>`

### Best Practices

- **Nutze bekannte Member-Namen**: Das System erkennt gängige Patterns automatisch
- **Fallback zu expliziten Typen**: Wenn möglich, verwende explizite Typen für bessere Type-Safety

---

## 2. Result-Type Inference

### Was ist das?

Das System verbessert die Type-Inference für Result-Types durch automatische Auflösung verschachtelter Result-Types und korrekte Type-Propagation.

### Beispiel

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

### Features

- **Verschachtelte Result-Types**: Automatische Auflösung von `Result<Result<T, E>, E>` → `Result<T, E>`
- **Type-Propagation**: Korrekte Propagation in Variablenzuweisungen
- **Call-Expression Support**: Verbesserte Inference in Funktionsaufrufen

### Best Practices

- **Vermeide unnötige Verschachtelung**: Explizite Typen sind klarer
- **Nutze Type-Propagation**: Das System propagiert Result-Types automatisch

---

## 3. Desugared Code Type Inference

### Was ist das?

Das System verfeinert automatisch die Typen von desugared Variablen (z.B. `__try_result`, `__await_result_*`) nach dem initialen Type-Check-Pass.

### Beispiel

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

### Unterstützte Desugared Variablen

- **`__try_result`**: Typ wird aus dem try-Block's Return-Type abgeleitet
- **`__await_result_*`**: Typ wird aus dem await-Ausdruck abgeleitet
- **Andere `__*` Variablen**: Typ wird aus dem Wert-Ausdruck abgeleitet

### Best Practices

- **Vertraue auf automatische Verfeinerung**: Das System verfeinert Typen automatisch
- **Verwende try-catch für bessere Inference**: try-catch Blöcke werden automatisch zu Result-Types transformiert

---

## 4. Automatic Code Ordering

### Was ist das?

Das System sortiert automatisch Funktionen, Typen und Blöcke basierend auf ihren Abhängigkeiten, sodass sie korrekt aufeinander aufbauen.

### Beispiel

**Vorher (manuelle Reihenfolge):**
```velin
fn processUser(user: User) {
    return user.name.toUpperCase();
}

struct User {
    name: string;
    email: string;
}
```

**Nachher (automatisch sortiert):**
```velin
struct User {
    name: string;
    email: string;
}

fn processUser(user: User) {
    return user.name.toUpperCase();
}
```

Die `User` Struct wird automatisch vor `processUser` platziert, da die Funktion von `User` abhängt.

### Sortierreihenfolge

1. **Use** Statements (immer zuerst)
2. **TypeAliases** (Typ-Definitionen)
3. **Enums** (Enum-Definitionen)
4. **Structs** (Struct-Definitionen)
5. **Traits** (Trait-Definitionen)
6. **Impls** (Trait-Implementierungen)
7. **Functions** (Funktions-Definitionen)
8. **TopLevelCode** (Top-Level Ausdrücke, immer zuletzt)

Innerhalb jeder Kategorie: **Abhängigkeitsbasierte Sortierung**

### Komplexes Beispiel

```velin
// Input (unsortiert)
fn createUser(name: string): User {
    return User { name, email: generateEmail(name) };
}

fn generateEmail(name: string): string {
    return format("{}@example.com", name);
}

struct User {
    name: string;
    email: string;
}

// Output (automatisch sortiert)
struct User {
    name: string;
    email: string;
}

fn generateEmail(name: string): string {
    return format("{}@example.com", name);
}

fn createUser(name: string): User {
    return User { name, email: generateEmail(name) };
}
```

### Zirkuläre Abhängigkeiten

Das System erkennt zirkuläre Abhängigkeiten und meldet sie als Fehler:

```
Error: Circular dependency detected involving: User, UserService
```

### Best Practices

- **Schreibe Code in natürlicher Reihenfolge**: Das System sortiert automatisch
- **Nutze explizite Typen**: Explizite Typen helfen dem System, Abhängigkeiten besser zu erkennen
- **Vermeide zirkuläre Abhängigkeiten**: Zirkuläre Abhängigkeiten werden erkannt, aber sollten vermieden werden

---

## 5. Build Orchestration (Multi-File)

### Was ist das?

Der `BuildOrchestrator` orchestriert den gesamten Build-Ablauf automatisch basierend auf Abhängigkeiten zwischen Dateien.

### Beispiel

**Projekt-Struktur:**
```
project/
  ├── main.velin
  ├── models.velin
  └── services.velin
```

**Dependencies:**
- `main.velin` → `models.velin`, `services.velin`
- `services.velin` → `models.velin`

**Kompilierungsreihenfolge (automatisch):**
1. `models.velin`
2. `services.velin`
3. `main.velin`

### Use-Statement Analyse

Das System analysiert `use` Statements, um Datei-Abhängigkeiten zu bestimmen:

```velin
// main.velin
use models;
use services;

// models.velin wird vor main.velin kompiliert
// services.velin wird vor main.velin kompiliert
```

### Best Practices

- **Nutze `use` Statements**: Das System verwendet `use` Statements, um Datei-Abhängigkeiten zu bestimmen
- **Organisiere Module logisch**: Auch wenn die Sortierung automatisch ist, hilft logische Organisation

---

## Zusammenfassung

### Type-Inference Features

✅ **Type::Any Member-Access** - Automatische Inference basierend auf Member-Namen  
✅ **Result-Type Inference** - Verbesserte Inference für Result-Types  
✅ **Desugared Code Type Inference** - Automatische Type-Verfeinerung

### Code-Ordering Features

✅ **Automatic Code Ordering** - Automatische Sortierung basierend auf Abhängigkeiten  
✅ **Build Orchestration** - Multi-File Dependency-Management

### Vorteile

- **Weniger manuelle Arbeit**: Keine manuelle Code-Sortierung nötig
- **Bessere Type-Safety**: Automatische Type-Inference für `any` Typen
- **Klarerer Code**: Automatische Sortierung macht Abhängigkeiten sichtbar
- **Multi-File Support**: Automatische Kompilierungsreihenfolge für große Projekte

---

## Nächste Schritte

- **[Type Inference Dokumentation](../architecture/type-inference.md)** - Detaillierte technische Dokumentation
- **[Code Ordering Dokumentation](../architecture/code-ordering.md)** - Detaillierte technische Dokumentation
- **[Compiler Architecture](../architecture/compiler-architecture.md)** - Compiler-Pass-System

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
