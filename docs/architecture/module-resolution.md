# Modul-Auflösung in VelinScript

**Version:** 2.5.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

VelinScript unterstützt modulare Projekte mit automatischer Modul-Auflösung. Module können andere Module importieren, und der Compiler löst alle Abhängigkeiten automatisch auf.

## Syntax

### Module importieren

```velin
// Importiert ein Modul aus einer .velin Datei
use models;
use services;
use security;
```

### Module definieren

```velin
// models.velin
struct User {
    id: string,
    name: string,
    email: string,
}

struct Product {
    id: string,
    name: string,
    price: number,
}
```

## Funktionsweise

### 1. Modul-Erkennung

Der `ParserPass` scannt den AST nach `use` Statements:

```velin
use models;  // Sucht nach models.velin
```

### 2. Datei-Suche

Der Compiler sucht automatisch nach `.velin` Dateien:
- Im gleichen Verzeichnis wie die aktuelle Datei
- Rekursiv für verschachtelte Module

### 3. Parsing & Merging

- Die gefundenen Module werden geparst
- Der AST wird in den globalen `Program` AST eingefügt
- Rekursive Auflösung für verschachtelte Imports

### 4. Type Checking

Der Type Checker hat Zugriff auf alle Definitionen aus allen Modulen.

## Beispiel-Projekt

### Projekt-Struktur

```
my-project/
├── main.velin
├── models.velin
├── services.velin
└── security.velin
```

### main.velin

```velin
use models;
use services;
use security;

@GET("/api/users")
@Auth
fn getUsers(): List<User> {
    return services.getAllUsers();
}
```

### models.velin

```velin
struct User {
    id: string,
    name: string,
    email: string,
}
```

### services.velin

```velin
use models;

fn getAllUsers(): List<User> {
    return db.findAll(User);
}
```

### security.velin

```velin
// Security-Middleware und Auth-Logik
```

## Rekursive Auflösung

Module können andere Module importieren:

```velin
// main.velin
use services;  // Lädt services.velin

// services.velin
use models;    // Lädt models.velin (rekursiv)
use security;  // Lädt security.velin (rekursiv)
```

Der Compiler löst alle Abhängigkeiten automatisch auf.

## Implementierung

**Datei:** `compiler/src/passes/parser.rs`

**Methode:** `resolve_imports()`

**Features:**
- Sammelt alle `use` Statements
- Sucht nach entsprechenden `.velin` Dateien
- Parst Module rekursiv
- Fügt geparste Module in den AST ein
- Verhindert zirkuläre Abhängigkeiten

## Best Practices

1. **Klare Modul-Struktur**: Ein Modul pro Datei
2. **Vermeide zirkuläre Abhängigkeiten**: Module sollten nicht sich gegenseitig importieren
3. **Konsistente Namensgebung**: Modul-Namen sollten Dateinamen entsprechen
4. **Separation of Concerns**: Models, Services, Security in separate Module

## Fehlerbehandlung

Wenn ein Modul nicht gefunden wird:
- Der Compiler gibt eine Warnung aus
- Die Kompilierung wird fortgesetzt
- Type-Checking-Fehler werden gemeldet

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 2.5.0
