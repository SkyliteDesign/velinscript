# Advanced VelinScript APIs - Dokumentation

Diese Dokumentation erklärt die Konzepte und Implementierungen in der Beispieldatei [`main.velin`](./main.velin).

Die Datei demonstriert fortgeschrittene Techniken für Backend-Entwicklung mit VelinScript, inklusive Echtzeit-Kommunikation, KI-Integration, Sicherheit und Datenverarbeitung.

## Inhaltsverzeichnis

1. [WebSocket Echo Client](#1-websocket-echo-client)
2. [Mini-Chatbot mit Embeddings](#2-mini-chatbot-mit-embeddings)
3. [Cronjob / Scheduler](#3-cronjob--scheduler)
4. [JWT Authentifizierung](#4-jwt-authentifizierung)
5. [File Upload API](#5-file-upload-api)
6. [In-Memory CRUD (Mini-ORM)](#6-in-memory-crud-mini-orm)

---

## 1. WebSocket Echo Client

**Konzept:** Echtzeit-Kommunikation über WebSockets.
Das Beispiel zeigt einen **Client**, der sich zu einem Server verbindet, eine Nachricht sendet und auf die Antwort wartet.

**Wichtige Funktionen:**
- `websocket.connect(url)`: Baut Verbindung auf.
- `websocket.send(ws, msg)`: Sendet Text.
- `websocket.receive(ws)`: Blockiert bis eine Nachricht eintrifft (synchron im Beispiel).
- `websocket.close(ws)`: Beendet Verbindung sauber.

**Code-Ausschnitt:**
```velin
fn websocketEcho(url: string, message: string): string {
    let wsResult = websocket.connect(url);
    // ... Error Handling ...
    let ws = wsResult.unwrap();
    websocket.send(ws, message);
    let response = websocket.receive(ws).unwrap();
    return response;
}
```

---

## 2. Mini-Chatbot mit Embeddings

**Konzept:** Semantische Suche (Vektor-Suche) für Chatbots.
Anstatt nur nach Stichworten zu suchen, wandeln wir Text in Zahlenreihen (Vektoren/Embeddings) um. Ähnliche Texte haben mathematisch ähnliche Vektoren.

**Ablauf:**
1. **Embedding erstellen:** `llm.embed(text)` wandelt User-Input in Vektor um.
2. **Vergleich:** `embedding.similarity(vecA, vecB)` berechnet Ähnlichkeit (Cosinus-Ähnlichkeit, 0.0 bis 1.0).
3. **Auswahl:** Die Antwort mit dem höchsten Score gewinnt.

**Code-Ausschnitt:**
```velin
fn chooseBestAnswer(userMessage: string, candidates: List<ChatMemoryItem>): string {
    let queryEmbedding = embedText(userMessage);
    // ... Loop über alle Kandidaten ...
    let score = embedding.similarity(queryEmbedding, item.embedding);
    // ... Besten Score finden ...
}
```

---

## 3. Cronjob / Scheduler

**Konzept:** Zeitgesteuerte Ausführung von Aufgaben.
Da VelinScript oft in einer Request/Response-Umgebung läuft, zeigt dieses Muster, wie man logische Zeitplanung implementiert (z.B. in einem Loop oder Worker-Prozess).

**Logik:**
- Jeder Job hat ein `intervalSeconds` und `lastRun`.
- Wir prüfen: `(jetzt - letzterLauf) >= intervall`.
- Wenn ja: Job ausführen und `lastRun` aktualisieren.

**Code-Ausschnitt:**
```velin
let shouldRun = (now - job.lastRun) >= job.intervalSeconds;
if (shouldRun) {
    // Job ausführen
    // Status aktualisieren
}
```

---

## 4. JWT Authentifizierung

**Konzept:** Sichere API-Endpunkte mit JSON Web Tokens (JWT).
Zeigt den kompletten Flow von Login bis zum geschützten Zugriff.

**Komponenten:**
1. **Login (`@POST`):** Prüft Credentials, erstellt `AuthService` und generiert Token mit `auth.generateToken(claims)`.
2. **Geschützter Endpunkt (`@GET`):** Nutzt `@Auth` Decorator (oder manuelle Prüfung).
3. **Verifizierung:** `auth.verifyToken(token)` prüft Signatur und Ablaufdatum.

**Code-Ausschnitt:**
```velin
@POST("/api/auth/login")
fn login(request: JwtLoginRequest): JWTToken {
    // ... Check Password ...
    let claims = UserClaims { ... };
    return auth.generateToken(claims);
}
```

---

## 5. File Upload API

**Konzept:** Empfangen und Speichern von Dateien.
Hier simuliert als API, die Dateiname und Inhalt (z.B. Base64) entgegennimmt und auf die Festplatte schreibt.

**Wichtige Module:**
- `file.write(path, content)`: Speichert die Daten.
- `fs.get_size(path)`: Prüft Erfolgreiches Schreiben.

**Code-Ausschnitt:**
```velin
@POST("/api/upload")
fn uploadFile(request: FileUploadRequest): FileUploadResponse {
    let path = "uploads/" + request.filename;
    file.write(path, request.content);
    // ...
}
```

---

## 6. In-Memory CRUD (Mini-ORM)

**Konzept:** Datenverwaltung ohne externe Datenbank.
Nützlich für Prototyping, Tests oder sehr kleine Apps. Die "Datenbank" ist einfach eine globale oder übergebene `List<User>`.

**Operationen:**
- **Create:** `list.push(newItem)`
- **Read:** `list.filter(...)` oder direkt zurückgeben.
- **Update:** Liste durchlaufen und Element ersetzen (oder mutable Referenz ändern).
- **Delete:** `list.filter(|item| item.id != id)` (erzeugt neue Liste ohne das Element).

**Code-Ausschnitt:**
```velin
fn deleteUser(users: List<User>, id: string): List<User> {
    // Funktionale Löschung durch Filter
    return users.filter(|u| u.id != id);
}
```
