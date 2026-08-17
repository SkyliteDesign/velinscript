# Automatic Code Ordering

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

Das Automatic Code Ordering System sortiert Funktionen, Typen und Blöcke automatisch basierend auf ihren Abhängigkeiten, sodass sie korrekt aufeinander aufbauen. Dies eliminiert die Notwendigkeit, Code manuell in der richtigen Reihenfolge zu schreiben.

---

## Features

### 1. Dependency-basierte Sortierung

**Status:** ✅ Implementiert in `compiler/src/passes/code_order.rs`

Der `CodeOrderingPass` analysiert alle Items im Programm und sortiert sie basierend auf ihren Abhängigkeiten.

#### Unterstützte Item-Typen

- **Functions** → Abhängig von: verwendeten Typen, aufgerufenen Funktionen, importierten Modulen
- **Structs** → Abhängig von: Feldtypen, generischen Parametern
- **Enums** → Abhängig von: Variant-Typen
- **TypeAliases** → Abhängig von: referenzierten Typen
- **Traits** → Abhängig von: Method-Signaturen
- **Impls** → Abhängig von: Trait-Namen, Method-Implementierungen
- **Modules** → Abhängig von: Sub-Items (rekursiv)
- **Use Statements** → Abhängig von: Modul-Pfaden

#### Sortierreihenfolge

1. **Use** Statements (immer zuerst)
2. **TypeAliases** (Typ-Definitionen)
3. **Enums** (Enum-Definitionen)
4. **Structs** (Struct-Definitionen)
5. **Traits** (Trait-Definitionen)
6. **Impls** (Trait-Implementierungen)
7. **Functions** (Funktions-Definitionen)
8. **TopLevelCode** (Top-Level Ausdrücke, immer zuletzt)

Innerhalb jeder Kategorie: **Abhängigkeitsbasierte Sortierung** (Topologische Sortierung)

#### Beispiel

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

---

### 2. Topologische Sortierung

**Status:** ✅ Implementiert mit `petgraph`

Das System verwendet topologische Sortierung, um Items in der korrekten Reihenfolge zu sortieren.

#### Dependency-Graph

- **Nodes**: Items (Functions, Structs, etc.)
- **Edges**: Abhängigkeiten (Item A hängt von Item B ab)

#### Zirkuläre Abhängigkeiten

Das System erkennt zirkuläre Abhängigkeiten und meldet sie als Fehler:

```
Error: Circular dependency detected involving: User, UserService
```

#### Beispiel mit komplexen Abhängigkeiten

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

---

### 3. Dependency-Extraktion

**Status:** ✅ Implementiert in `compiler/src/passes/code_order.rs`

Das System extrahiert Abhängigkeiten aus:

#### Funktionen
- Parameter-Typen
- Return-Typen
- Aufgerufene Funktionen (in Funktions-Body)
- Verwendete Typen (in Funktions-Body)

#### Structs
- Feldtypen
- Generische Parameter (Constraints)

#### Enums
- Variant-Typen

#### TypeAliases
- Aliased Typen

#### Expressions
- Funktionsaufrufe
- Member-Zugriffe
- Typ-Konstruktoren
- Lambda-Parameter und Return-Typen

---

## Build Orchestration (Multi-File)

**Status:** ✅ Implementiert in `compiler/src/compiler/orchestrator.rs`

Der `BuildOrchestrator` orchestriert den gesamten Build-Ablauf automatisch basierend auf Abhängigkeiten zwischen Dateien.

### Features

- **Multi-File Dependency-Graph**: Erstellt einen Dependency-Graph zwischen Dateien
- **Automatische Kompilierungsreihenfolge**: Bestimmt die korrekte Reihenfolge für Multi-File-Projekte
- **Zirkuläre Abhängigkeits-Erkennung**: Erkennt und meldet zirkuläre Abhängigkeiten zwischen Dateien

### Use-Statement Analyse

Das System analysiert `use` Statements, um Datei-Abhängigkeiten zu bestimmen:

```velin
// main.velin
use models;
use services;

// models.velin wird vor main.velin kompiliert
// services.velin wird vor main.velin kompiliert
```

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

---

## Integration in den Compiler

### Pass-Reihenfolge

1. **ParserPass** - Parsing und AST-Erstellung
2. **DesugaringPass** - Transformation
3. **CodeOrderingPass** ✅ (Neu in 3.1.0) - Automatische Code-Sortierung
4. **TypeCheckPass** - Type Checking
5. **CodegenPass** - Code-Generierung

### Verwendung

Der `CodeOrderingPass` wird automatisch nach dem Desugaring-Pass ausgeführt:

```rust
compiler.add_pass(Box::new(ParserPass::new()));
compiler.add_pass(Box::new(DesugaringPass::new()));
compiler.add_pass(Box::new(CodeOrderingPass::new())); // Automatische Sortierung
compiler.add_pass(Box::new(TypeCheckPass::new(true)));
```

---

## Best Practices

### Code-Organisation

- **Schreibe Code in natürlicher Reihenfolge**: Das System sortiert automatisch, du musst nicht über die Reihenfolge nachdenken
- **Nutze explizite Typen**: Explizite Typen helfen dem System, Abhängigkeiten besser zu erkennen
- **Vermeide zirkuläre Abhängigkeiten**: Zirkuläre Abhängigkeiten werden erkannt, aber sollten vermieden werden

### Multi-File-Projekte

- **Nutze `use` Statements**: Das System verwendet `use` Statements, um Datei-Abhängigkeiten zu bestimmen
- **Organisiere Module logisch**: Auch wenn die Sortierung automatisch ist, hilft logische Organisation

---

## Technische Details

### Implementierung

**Dateien:**
- `compiler/src/passes/code_order.rs` - CodeOrderingPass Implementierung
- `compiler/src/compiler/orchestrator.rs` - BuildOrchestrator für Multi-File-Projekte

**Abhängigkeiten:**
- `petgraph` - Graph-Datenstrukturen und topologische Sortierung
- `indexmap` - IndexMap für stabile Sortierung

### Algorithmus

1. **Dependency-Graph Erstellung**: Analysiert alle Items und erstellt einen Dependency-Graph
2. **Topologische Sortierung**: Führt topologische Sortierung durch
3. **Zirkuläre Abhängigkeits-Erkennung**: Erkennt Zyklen und meldet Fehler
4. **Item-Reihenfolge**: Sortiert Items basierend auf topologischer Sortierung

### Performance

- **O(V + E)**: Linear in Anzahl der Items und Abhängigkeiten
- **Effizient**: Nutzt effiziente Graph-Algorithmen von `petgraph`

---

## Beispiele

### Einfaches Beispiel

```velin
// Input
fn main() {
    let user = createUser("John");
    print(user.name);
}

struct User {
    name: string;
}

fn createUser(name: string): User {
    return User { name };
}

// Output (automatisch sortiert)
struct User {
    name: string;
}

fn createUser(name: string): User {
    return User { name };
}

fn main() {
    let user = createUser("John");
    print(user.name);
}
```

### Komplexes Beispiel mit Generics

```velin
// Input
fn processList<T>(items: List<T>): List<T> {
    return items.map(transform);
}

fn transform<T>(item: T): T {
    return item;
}

struct Container<T> {
    value: T;
}

// Output (automatisch sortiert)
struct Container<T> {
    value: T;
}

fn transform<T>(item: T): T {
    return item;
}

fn processList<T>(items: List<T>): List<T> {
    return items.map(transform);
}
```

---

## Siehe auch

- [Compiler Architecture](./compiler-architecture.md) - Compiler-Pass-System
- [Type Inference](./type-inference.md) - Type-Inference System
- [Module Resolution](./module-resolution.md) - Modul-Auflösung

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
