# 05 - Der Ultimative Showcase

**Neu in Version 2.5** ✅

Dieses Beispielprojekt demonstriert die volle Leistungsfähigkeit von VelinScript 2.5. Es simuliert ein E-Commerce-Backend mit modernen Features wie KI-Integration, transaktionalen Flows und paralleler Pipeline-Verarbeitung.

Dieses Beispiel zeigt **alle neuen Features von Version 2.5**:
- VelinAutoDoc für automatische Dokumentationsgenerierung
- VelinPipeline für automatische Performance-Optimierung
- @Flow für transaktionales Flow-Management

## Projektstruktur

Das Projekt ist in mehrere Module unterteilt, um eine saubere Architektur zu gewährleisten:

*   **`main.velin`**: Der Einstiegspunkt der Anwendung. Hier werden die API-Endpunkte definiert und die Module verknüpft.
*   **`models.velin`**: Enthält die Datenstrukturen (Structs) wie `User`, `Product` und `Order`.
*   **`services.velin`**: Beinhaltet die Geschäftslogik. Hier kommen `@Flow` (für Transaktionen) und `@VelinPipeline` (für Parallelisierung) zum Einsatz.
*   **`security.velin`**: Implementiert Authentifizierungs- und Autorisierungslogik (Middleware).
*   **`intelligence.velin`**: Ein Modul für KI-Aufgaben, wie z.B. Produktempfehlungen.

## Features

### 1. Deklarative API-Definition
Endpunkte werden einfach mit Decorators wie `@GET` und `@POST` definiert.

### 2. Auto-Dokumentation
Mit `@VelinAutoDoc` werden automatisch Dokumentationen für Funktionen und Structs generiert.

### 3. Transaktionale Sicherheit
Die `@Flow` Engine in `services.velin` sorgt dafür, dass komplexe Operationen (wie ein Checkout) atomar sind. Schlägt ein Schritt fehl, werden alle vorherigen Datenbankänderungen automatisch zurückgerollt.

### 4. Performance durch Pipelines
Die `@VelinPipeline` Annotation in `loadUserDashboard` ermöglicht das parallele Laden von Daten (Benutzerprofil, Bestellungen, Empfehlungen), ohne komplexen Async-Code schreiben zu müssen.

### 5. Robuste Typisierung
Das System nutzt das starke Typensystem von VelinScript, um Fehler bereits zur Kompilierzeit zu erkennen.

### 6. KI-Integration
Das `intelligence` Modul zeigt, wie ML-Modelle nahtlos in den Code integriert werden können.

## Ausführung

Um das Projekt zu überprüfen (Parsing & Type Checking):

```bash
# In das Beispiel-Verzeichnis wechseln
cd examples/05-ultimate-showcase

# Projekt prüfen
velin-compiler check -i main.velin

# Projekt kompilieren
velin-compiler compile -i main.velin -o main.rs
```

## Weitere Informationen

- **VelinAutoDoc**: Siehe [Tutorial 8: Intelligence](../../docs/guides/tutorial-8-intelligence.md)
- **VelinPipeline**: Siehe [Tutorial 8: Intelligence](../../docs/guides/tutorial-8-intelligence.md)
- **@Flow**: Siehe [Tutorial 8: Intelligence](../../docs/guides/tutorial-8-intelligence.md)
- **API Decorators**: Siehe [Decorators Dokumentation](../../docs/api/decorators.md)

*Hinweis: Der aktuelle Compiler unterstützt noch keine automatische Auflösung von Modul-Importen über mehrere Dateien hinweg, daher können Typ-Fehler auftreten, wenn `models` oder `services` nicht gefunden werden.*
