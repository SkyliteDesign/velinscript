# Einfacher Blog

## 📝 Beschreibung

Ein vollständiges Blog-System mit Posts, Kommentaren, Tags und Such-/Filterfunktionen. Dieses Beispiel demonstriert:

- CRUD-Operationen für Blog-Posts
- Kommentarsystem
- Tag-basierte Organisation
- Such- und Filterfunktionen
- Pagination
- View-Tracking
- Authentifizierung für geschützte Aktionen

## 🎯 Lernziele

- Komplexe Datenmodelle mit Beziehungen
- Authentifizierung mit `@Auth` Decorator
- Pagination implementieren
- Filter- und Suchfunktionen
- Aggregation (Zählen von Kommentaren)
- Kaskadierende Löschoperationen
- Query Builder für komplexe Abfragen

## 🚀 Verwendung

### Blog-Posts verwalten

#### Alle Posts auflisten (mit Pagination)
```bash
GET /api/blog/posts?page=0&pageSize=10
```

#### Einzelnen Post anzeigen
```bash
GET /api/blog/posts/:id
```
*Erhöht automatisch den View-Counter*

#### Neuen Post erstellen (Auth erforderlich)
```bash
POST /api/blog/posts
{
    "title": "Mein erster VelinScript Post",
    "content": "Das ist der Inhalt meines ersten Blog-Posts...",
    "author": "Max Mustermann",
    "tags": ["velinscript", "tutorial", "anfänger"]
}
```

#### Post aktualisieren (Auth erforderlich)
```bash
PUT /api/blog/posts/:id
{
    "title": "Aktualisierter Titel",
    "content": "Neuer Inhalt...",
    "tags": ["velinscript", "fortgeschritten"]
}
```

#### Post veröffentlichen (Auth erforderlich)
```bash
PATCH /api/blog/posts/:id/publish
```

#### Post löschen (Auth erforderlich)
```bash
DELETE /api/blog/posts/:id
```
*Löscht auch alle zugehörigen Kommentare*

### Kommentare verwalten

#### Kommentare eines Posts anzeigen
```bash
GET /api/blog/posts/:postId/comments
```

#### Kommentar hinzufügen
```bash
POST /api/blog/posts/:postId/comments
{
    "author": "Anna Schmidt",
    "content": "Toller Artikel!"
}
```

#### Kommentar löschen (Auth erforderlich)
```bash
DELETE /api/blog/comments/:id
```

### Suchen und Filtern

#### Posts nach Tag
```bash
GET /api/blog/posts/by-tag/velinscript
```

#### Posts nach Autor
```bash
GET /api/blog/posts/by-author/Max%20Mustermann
```

#### Volltextsuche
```bash
GET /api/blog/search?query=tutorial
```
*Durchsucht Titel und Inhalt*

#### Beliebteste Posts
```bash
GET /api/blog/posts/popular?limit=5
```
*Sortiert nach View-Count*

## 💡 Wichtige Konzepte

1. **Datenbeziehungen**: Posts haben viele Kommentare (1:n)

2. **Authentifizierung**: `@Auth` schützt sensible Endpoints
   ```velin
   @POST("/api/blog/posts")
   @Auth
   fn createPost(...) { ... }
   ```

3. **Pagination**: Große Datensätze aufteilen
   ```velin
   let offset = page * pageSize;
   db.query(...).limit(pageSize).offset(offset)
   ```

4. **View-Tracking**: Automatisches Zählen von Aufrufen
   ```velin
   post.viewCount = post.viewCount + 1;
   db.update(post);
   ```

5. **Kaskadierende Löschung**: Beim Löschen eines Posts werden auch Kommentare gelöscht

6. **Post-Entwürfe**: Neue Posts sind standardmäßig unpublished

7. **Excerpt-Generierung**: Automatische Textauszüge für Übersichten

## 📊 Datenstruktur

```
BlogPost
├── id: string
├── title: string
├── content: string
├── author: string
├── tags: List<string>
├── createdAt: string
├── updatedAt: string
├── published: boolean
└── viewCount: number

Comment
├── id: string
├── postId: string (→ BlogPost.id)
├── author: string
├── content: string
└── createdAt: string
```

## 🔧 Erweiterungsmöglichkeiten

- Benutzerverwaltung mit Registrierung/Login
- Markdown-Unterstützung für Post-Inhalt
- Bilder/Medien-Upload
- Like/Dislike-System
- Kategorien zusätzlich zu Tags
- RSS-Feed generieren
- SEO-Optimierung (Meta-Tags, Slugs)
- Kommentar-Threading (Antworten auf Kommentare)
- Spam-Filter für Kommentare
- Lesezeit-Berechnung
- Related Posts Vorschläge
- Archivierung alter Posts

## ⚠️ Sicherheitshinweise

- Alle Post-Änderungen erfordern Authentifizierung
- Content sollte vor Ausgabe sanitized werden (XSS-Schutz)
- Rate-Limiting für Kommentare empfohlen
- Input-Validierung für alle User-Eingaben
