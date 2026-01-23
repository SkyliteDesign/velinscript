# CodeOrderingPass - Automatische Code-Sortierung

**Version:** 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Datei:** `compiler/src/passes/code_order.rs`

---

## Übersicht

Der `CodeOrderingPass` ist der **vierte Pass** im VelinScript Compiler (nach DesugaringPass). Er sortiert automatisch alle Items (Funktionen, Typen, Blöcke) basierend auf ihren Abhängigkeiten, sodass sie in der korrekten Reihenfolge stehen.

**Hauptfunktion:** Eliminiert die Notwendigkeit, Code manuell in der richtigen Reihenfolge zu schreiben.

---

## Funktionsweise

### 1. Dependency-Graph Erstellung

Der Pass analysiert alle Items und erstellt einen Dependency-Graph:

```rust
fn build_dependency_graph(
    &self,
    items: &[Item],
) -> Result<(DiGraph<String, ()>, IndexMap<String, NodeIndex>, HashMap<NodeIndex, Item>)> {
    let mut graph = DiGraph::new();
    let mut node_map = IndexMap::new();
    let mut item_map = HashMap::new();
    
    // First pass: Add all items as nodes
    for item in items {
        if let Some(name) = self.get_item_name(item) {
            if !node_map.contains_key(&name) {
                let idx = graph.add_node(name.clone());
                node_map.insert(name.clone(), idx);
                item_map.insert(idx, item.clone());
            }
        }
    }
    
    // Second pass: Add edges based on dependencies
    for item in items {
        if let Some(item_name) = self.get_item_name(item) {
            let item_idx = *node_map.get(&item_name).unwrap();
            let dependencies = self.extract_dependencies(item);
            
            for dep in dependencies {
                if let Some(dep_idx) = node_map.get(&dep) {
                    // Add edge: dependency -> item (item depends on dependency)
                    graph.add_edge(*dep_idx, item_idx, ());
                }
            }
        }
    }
    
    Ok((graph, node_map, item_map))
}
```

### 2. Topologische Sortierung

Der Pass verwendet topologische Sortierung, um Items in der korrekten Reihenfolge zu sortieren:

```rust
fn order_program(&self, program: &mut Program) -> Result<()> {
    // Build dependency graph
    let (graph, _node_map, item_map) = self.build_dependency_graph(&program.items)?;
    
    // Perform topological sort
    let sorted_indices = match toposort(&graph, None) {
        Ok(indices) => indices,
        Err(cycle) => {
            // Circular dependency detected
            let cycle_node = graph[cycle.node_id()].clone();
            return Err(anyhow::anyhow!(
                "Circular dependency detected involving: {:?}",
                cycle_node
            ));
        }
    };
    
    // Reorder items based on topological sort
    let mut ordered_items = Vec::new();
    let mut item_map_mut = item_map;
    for idx in sorted_indices {
        if let Some(item) = item_map_mut.remove(&idx) {
            ordered_items.push(item);
        }
    }
    
    program.items = ordered_items;
    Ok(())
}
```

### 3. Dependency-Extraktion

Der Pass extrahiert Abhängigkeiten aus verschiedenen Item-Typen:

#### Funktionen

```rust
Item::Function(func) => {
    // Parameter-Typen
    for param in &func.params {
        dependencies.extend(self.extract_type_dependencies(&param.param_type));
    }
    
    // Return-Typ
    if let Some(ref return_type) = func.return_type {
        dependencies.extend(self.extract_type_dependencies(return_type));
    }
    
    // Aufgerufene Funktionen (im Body)
    self.extract_function_calls(&func.body, &mut dependencies);
}
```

#### Structs

```rust
Item::Struct(struct_def) => {
    // Feldtypen
    for field in &struct_def.fields {
        dependencies.extend(self.extract_type_dependencies(&field.field_type));
    }
    
    // Generische Parameter
    for generic in &struct_def.generics {
        dependencies.extend(self.extract_type_dependencies(&generic.bound));
    }
}
```

#### Enums

```rust
Item::Enum(enum_def) => {
    // Variant-Typen
    for variant in &enum_def.variants {
        if let Some(ref data) = variant.data {
            dependencies.extend(self.extract_type_dependencies(data));
        }
    }
}
```

---

## Sortierreihenfolge

### Kategorien-Reihenfolge

Items werden zuerst nach Kategorien sortiert:

1. **Use Statements** (immer zuerst)
2. **TypeAliases** (Typ-Definitionen)
3. **Enums** (Enum-Definitionen)
4. **Structs** (Struct-Definitionen)
5. **Traits** (Trait-Definitionen)
6. **Impls** (Trait-Implementierungen)
7. **Functions** (Funktions-Definitionen)
8. **TopLevelCode** (Top-Level Ausdrücke, immer zuletzt)

### Innerhalb jeder Kategorie

Innerhalb jeder Kategorie werden Items basierend auf Abhängigkeiten sortiert (topologische Sortierung).

---

## Dependency-Extraktion Details

### Type-Dependencies

```rust
fn extract_type_dependencies(&self, ty: &Type) -> Vec<String> {
    match ty {
        Type::Named(name) => vec![name.clone()],
        Type::Generic { name, params } => {
            let mut deps = vec![name.clone()];
            for param in params {
                deps.extend(self.extract_type_dependencies(param));
            }
            deps
        }
        Type::List(item_type) => self.extract_type_dependencies(item_type),
        Type::Map { key_type, value_type } => {
            let mut deps = self.extract_type_dependencies(key_type);
            deps.extend(self.extract_type_dependencies(value_type));
            deps
        }
        _ => Vec::new(),
    }
}
```

### Function-Call-Dependencies

```rust
fn extract_function_calls(&self, block: &Block, dependencies: &mut Vec<String>) {
    for stmt in &block.statements {
        match stmt {
            Statement::Expression(expr_stmt) => {
                self.extract_function_calls_from_expr(&expr_stmt.expression, dependencies);
            }
            Statement::Return(return_stmt) => {
                if let Some(ref value) = return_stmt.value {
                    self.extract_function_calls_from_expr(value, dependencies);
                }
            }
            // ... weitere Statements
        }
    }
}
```

---

## Beispiele

### Beispiel 1: Einfache Abhängigkeit

**Vorher (unsortiert):**
```velin
fn processUser(user: User) {
    return user.name.toUpperCase();
}

struct User {
    name: string;
    email: string;
}
```

**Nachher (sortiert):**
```velin
struct User {
    name: string;
    email: string;
}

fn processUser(user: User) {
    return user.name.toUpperCase();
}
```

**Erklärung:** `processUser` hängt von `User` ab, daher wird `User` zuerst platziert.

### Beispiel 2: Komplexe Abhängigkeiten

**Vorher (unsortiert):**
```velin
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
```

**Nachher (sortiert):**
```velin
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

**Erklärung:**
1. `User` Struct wird zuerst platziert (keine Abhängigkeiten)
2. `generateEmail` wird vor `createUser` platziert (wird von `createUser` aufgerufen)
3. `createUser` wird zuletzt platziert (hängt von `User` und `generateEmail` ab)

### Beispiel 3: Generics

**Vorher (unsortiert):**
```velin
fn processList<T>(items: List<T>): List<T> {
    return items.map(transform);
}

fn transform<T>(item: T): T {
    return item;
}

struct Container<T> {
    value: T;
}
```

**Nachher (sortiert):**
```velin
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

## Zirkuläre Abhängigkeiten

### Erkennung

Der Pass erkennt zirkuläre Abhängigkeiten und meldet sie als Fehler:

```rust
let sorted_indices = match toposort(&graph, None) {
    Ok(indices) => indices,
    Err(cycle) => {
        // Circular dependency detected
        let cycle_node = graph[cycle.node_id()].clone();
        return Err(anyhow::anyhow!(
            "Circular dependency detected involving: {:?}",
            cycle_node
        ));
    }
};
```

### Beispiel

**Zirkuläre Abhängigkeit:**
```velin
// User hängt von UserService ab
struct User {
    service: UserService;
}

// UserService hängt von User ab
struct UserService {
    user: User;
}
```

**Fehler:**
```
Error: Circular dependency detected involving: User, UserService
```

### Lösung

Zirkuläre Abhängigkeiten müssen manuell aufgelöst werden:

```velin
// Option 1: Referenz statt direkte Abhängigkeit
struct User {
    service_id: string;  // Statt UserService
}

struct UserService {
    user_id: string;  // Statt User
}

// Option 2: Optionale Abhängigkeit
struct User {
    service: Option<UserService>;
}
```

---

## Integration in Compiler-Pipeline

### Pass-Reihenfolge

```
1. AutoFixPass      → Korrigiert Source-Code
2. ParserPass       → Erstellt AST
3. DesugaringPass   → Transformiert Sugar-Syntax
4. CodeOrderingPass → Sortiert Items (HIER!)
5. TypeCheckPass    → Type-Checking
...
```

### CompilationContext-Modifikationen

Der CodeOrderingPass modifiziert:

- ✅ `context.program.items` - Sortiert Items direkt
- ❌ `context.errors` - Nur bei zirkulären Abhängigkeiten
- ❌ `context.source_map` - Keine Änderungen

### Wichtig

**Der CodeOrderingPass ist immer aktiv** und läuft nach dem DesugaringPass, aber vor dem TypeCheckPass.

---

## Datenfluss

### Input

```rust
Program {
    items: [
        Function { name: "processUser", params: [Param { type: "User" }] },
        Struct { name: "User", fields: [...] }
    ]
}
```

### Output

```rust
Program {
    items: [
        Struct { name: "User", fields: [...] },
        Function { name: "processUser", params: [Param { type: "User" }] }
    ]
}
```

---

## Algorithmus-Details

### Topologische Sortierung

Der Pass verwendet den **Kahn-Algorithmus** (implementiert in `petgraph::algo::toposort`):

1. **Graph-Erstellung:** Alle Items werden als Nodes hinzugefügt
2. **Edge-Erstellung:** Abhängigkeiten werden als Edges hinzugefügt
3. **Topologische Sortierung:** Items werden in topologischer Reihenfolge sortiert
4. **Zyklen-Erkennung:** Zirkuläre Abhängigkeiten werden erkannt

### Performance

- **Zeitkomplexität:** O(V + E) - Linear in Anzahl der Items und Abhängigkeiten
- **Speicherkomplexität:** O(V + E) - Graph-Speicher
- **Effizienz:** Nutzt effiziente Graph-Algorithmen von `petgraph`

---

## API-Referenz

### CodeOrderingPass::new()

Erstellt eine neue CodeOrderingPass-Instanz:

```rust
let pass = CodeOrderingPass::new();
```

### Pass::run()

Führt den Pass aus:

```rust
pass.run(&mut context)?;
```

**Parameter:**
- `context: &mut CompilationContext` - Compilation-Kontext

**Rückgabe:**
- `Result<()>` - Erfolg oder Fehler (bei zirkulären Abhängigkeiten)

### Interne Methoden

- `order_program()` - Sortiert gesamtes Programm
- `build_dependency_graph()` - Erstellt Dependency-Graph
- `extract_dependencies()` - Extrahiert Abhängigkeiten aus Item
- `extract_type_dependencies()` - Extrahiert Typ-Abhängigkeiten
- `extract_function_calls()` - Extrahiert Funktionsaufrufe
- `get_item_name()` - Holt Item-Namen

---

## Best Practices

### Code-Organisation

1. **Schreibe Code in natürlicher Reihenfolge:** Das System sortiert automatisch
2. **Nutze explizite Typen:** Explizite Typen helfen dem System, Abhängigkeiten besser zu erkennen
3. **Vermeide zirkuläre Abhängigkeiten:** Zirkuläre Abhängigkeiten werden erkannt, aber sollten vermieden werden

### Dependency-Management

1. **Klare Abhängigkeiten:** Stelle sicher, dass Abhängigkeiten klar definiert sind
2. **Minimale Abhängigkeiten:** Vermeide unnötige Abhängigkeiten
3. **Modulare Struktur:** Organisiere Code in logische Module

---

## Debugging

### Dependency-Graph anzeigen

Um den Dependency-Graph zu sehen:

```rust
let (graph, node_map, item_map) = self.build_dependency_graph(&program.items)?;

for node in graph.node_indices() {
    println!("Node: {:?}", graph[node]);
    for edge in graph.edges(node) {
        println!("  -> {:?}", graph[edge.target()]);
    }
}
```

### Sortierreihenfolge prüfen

Um die Sortierreihenfolge zu prüfen:

```rust
if let Some(ref program) = context.program {
    for (i, item) in program.items.iter().enumerate() {
        println!("{}: {:?}", i, self.get_item_name(item));
    }
}
```

---

## Siehe auch

- [Pass-Verlauf](./pass-verlauf.md) - Übersicht aller Passes
- [Code Ordering](./code-ordering.md) - Feature-Dokumentation
- [ParserPass](./parser-pass.md) - Parsing & Modul-Auflösung
- [DesugaringPass](./desugaring-pass.md) - Syntaktischer Zucker Transformation
- [Type Inference](./type-inference.md) - Type-Inference System

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
