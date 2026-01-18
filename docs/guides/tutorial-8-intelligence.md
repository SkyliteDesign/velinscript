# Tutorial 8: Intelligence Features nutzen

VelinScript 2.5.0 führt eine Reihe von "Intelligence"-Features ein, die dich beim Schreiben, Dokumentieren, Testen und Optimieren von Code unterstützen. In diesem Tutorial lernst du, wie du diese mächtigen Werkzeuge einsetzt.

**Neu in Version 2.5**: Alle Intelligence-Features sind vollständig implementiert und produktionsreif ✅

## 1. Automatische Dokumentation mit `@VelinAutoDoc`

**Neu in Version 2.5** ✅

Schluss mit veralteter Doku. VelinScript generiert sie direkt aus deinem Code.

### Schritt 1: Code kommentieren

Nutze `///` Kommentare über deinen Funktionen. Diese werden als First-Class-Citizens im AST erfasst.

```velin
/// Berechnet den Gesamtpreis einer Bestellung inkl. Steuern.
/// 
/// # Arguments
/// * `items` - Die Liste der Artikel
/// * `taxRate` - Der Steuersatz (z.B. 0.19)
/// 
/// # Returns
/// Der Gesamtpreis als number
@VelinAutoDoc
fn calculateTotal(items: List<Item>, taxRate: number): number {
    // ...
}
```

### Schritt 2: Generierung

Der Compiler extrahiert diese Informationen in eine strukturierte JSON-Datei (`autodoc.json`), die nicht nur für Menschen lesbar ist, sondern auch KI-Modellen hilft, deinen Code zu verstehen.

**Features:**
- Erfasst `///` Doc-Comments im AST
- Extrahiert Typ-Signaturen, Parameter und Return-Types
- Erstellt `llm_prompt_context` für KI-gestützte Dokumentationsgenerierung
- Unterstützt Funktionen, Structs und Module

## 2. Automatische Tests mit `@VelinAutoTest`

**Neu in Version 2.5** ✅

Lass VelinScript die langweilige Arbeit des Test-Schreibens übernehmen.

```velin
@VelinAutoTest
fn validateUser(user: User): boolean {
    if (user.age < 18) return false;
    return true;
}
```

Wenn du `velin compile` ausführst, generiert der Compiler automatisch Test-Stubs mit Mock-Daten für alle Funktionen mit `@VelinAutoTest`.

**Features:**
- Automatische Test-Stub-Generierung
- Mock-Daten basierend auf Parametertypen
- Grundlegende Assertions
- Integration in Codegen-Pipeline
- Generiert Rust-Test-Code

**Generierter Test:**
```rust
#[tokio::test]
async fn test_auto_validateUser() {
    let user = User::default();
    let result = validateUser(user).await;
    assert!(result.is_ok(), "Function execution failed");
}
```

## 3. Selbstheilende Flows mit `@Flow`

**Neu in Version 2.5** ✅

Baue robuste Prozesse, die sich bei Fehlern selbst aufräumen.

```velin
@Flow
@POST("/checkout")
fn checkout(cart: Cart): Order {
    // Velin macht hier automatisch einen Snapshot des Inputs
    flow.snapshot_input(cart);
    
    let order = createOrder(cart);
    chargePayment(order); // Schlägt das fehl?
    
    // Velin führt automatisch einen Rollback durch, wenn eine Exception fliegt!
    return order;
}
```

Der `@Flow` Decorator injiziert eine Runtime, die den Ausführungsstatus überwacht.

**Features:**
- Automatisches State-Tracking (Pending, Running, Completed, Failed, Compensating, Compensated)
- Input-Snapshot-Management für Rollback
- Automatisches Commit bei Erfolg
- Automatisches Rollback mit Compensation-Logic bei Fehler
- Logging der Ausführungsdauer und Status
- Self-Healing durch Compensation-Hooks

## 4. Performance-Optimierung mit `@VelinPipeline`

**Neu in Version 2.5** ✅

Warum manuell parallelisieren, wenn der Compiler es besser kann?

```velin
@VelinPipeline
async fn loadUserProfile(id: string) {
    // Diese beiden Aufrufe hängen nicht voneinander ab.
    // VelinPipeline erkennt das und führt sie gleichzeitig aus (wie Promise.all).
    let profile = await db.find(Profile, id);
    let history = await db.find(History, id);
    let recommendations = await getRecommendations(id);
    
    return { profile, history, recommendations };
}
```

**Features:**
- Analysiert Datenabhängigkeiten zwischen Statements
- Erkennt automatisch unabhängige async Operationen
- Optimiert sequentielle Aufrufe zu parallelen Ausführungsgruppen
- Generiert automatisch `tokio::join!` für unabhängige Operationen
- Verbessert Performance durch Parallelisierung

**Beispiel-Transformation:**
```velin
// Vorher (sequentiell)
let a = await op1();
let b = await op2(); // Wartet auf op1
let c = await op3(); // Wartet auf op2

// Nachher (parallel mit @VelinPipeline)
let (a, b, c) = tokio::join!(op1(), op2(), op3());
```

## 5. Code-Analyse mit `@VelinInsight`

**Neu in Version 2.5** ✅

VelinInsight ist dein persönlicher Code-Reviewer.

Führe den Befehl "VelinScript: Run Insight Analysis" in VS Code aus. Velin analysiert dein Projekt auf:
*   **Unused Code**: Structs oder Funktionen, die nie aufgerufen werden.
*   **Komplexität**: Funktionen, die zu lang oder zu verschachtelt sind (Statement Count > 20).
*   **Ineffizienzen**: Unnötige Datenbank-Queries in Schleifen.

**Features:**
- Automatische Code-Analyse
- InsightReport mit detaillierten Empfehlungen
- Integration mit VS Code Extension
- Kann als Pass in der Compiler-Pipeline ausgeführt werden

**Beispiel:**
```velin
@VelinInsight
mod services {
    // Wird automatisch analysiert
    fn complexFunction() {
        // 25 Statements -> wird als komplex erkannt
    }
}
```

## Praktisches Beispiel

Sieh dir das **[Ultimate Showcase Beispiel](../../examples/05-ultimate-showcase/)** an, um alle Intelligence-Features in Aktion zu sehen:

- `@VelinAutoDoc` für automatische Dokumentation
- `@VelinPipeline` für Performance-Optimierung
- `@Flow` für transaktionales Flow-Management

## Zusammenfassung

Mit diesen Tools verschiebt sich deine Rolle vom "Code-Schreiber" zum "Architekten". Du definierst die Logik, VelinScript kümmert sich um Tests, Doku, Sicherheit und Performance.

**Neu in Version 2.5**: Alle Intelligence-Features sind vollständig implementiert und produktionsreif ✅
