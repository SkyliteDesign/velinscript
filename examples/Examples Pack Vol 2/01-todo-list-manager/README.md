# Todo-Listen Manager

## 📋 Beschreibung

Ein einfacher Todo-Listen Manager mit vollständigen CRUD-Operationen (Create, Read, Update, Delete). Dieses Beispiel demonstriert:

- REST API Endpoints mit verschiedenen HTTP-Methoden
- Datenbank-Operationen
- Strukturierte Datenmodelle
- Fehlerbehandlung
- String-Interpolation

## 🎯 Lernziele

- Verstehen von REST API-Design
- Arbeiten mit Strukturen (structs)
- CRUD-Operationen implementieren
- HTTP-Methoden korrekt einsetzen (@GET, @POST, @PUT, @PATCH, @DELETE)
- Datenbankabfragen durchführen

## 🚀 Verwendung

### Alle Todos abrufen
```bash
GET /api/todos
```

### Ein Todo erstellen
```bash
POST /api/todos
{
    "title": "VelinScript lernen",
    "description": "Die Grundlagen von VelinScript durchgehen"
}
```

### Ein Todo aktualisieren
```bash
PUT /api/todos/:id
{
    "title": "VelinScript beherrschen",
    "description": "Fortgeschrittene Features erkunden",
    "completed": true
}
```

### Ein Todo als erledigt markieren
```bash
PATCH /api/todos/:id/complete
```

### Ein Todo löschen
```bash
DELETE /api/todos/:id
```

### Gefilterte Listen
```bash
GET /api/todos/completed  # Nur erledigte Todos
GET /api/todos/pending    # Nur offene Todos
```

## 💡 Wichtige Konzepte

1. **Strukturen**: Definieren das Datenmodell (`Todo`, `CreateTodoRequest`)
2. **API Decorators**: `@GET`, `@POST`, etc. definieren die Endpoints
3. **Datenbankoperationen**: `db.find()`, `db.save()`, `db.update()`, `db.delete()`
4. **Fehlerbehandlung**: `throw NotFoundError()` für nicht gefundene Ressourcen
5. **Query Builder**: Filterung mit `.where()` für komplexere Abfragen

## 🔧 Erweiterungsmöglichkeiten

- Authentifizierung hinzufügen
- Todos nach Priorität sortieren
- Fälligkeitsdatum implementieren
- Tags/Kategorien hinzufügen
- Suchfunktion einbauen
