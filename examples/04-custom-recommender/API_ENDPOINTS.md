# API Endpoints Dokumentation

Vollständige Dokumentation aller API-Endpunkte des Custom Recommender Systems.

## Base URL

```
http://localhost:8080
```

## Endpoints

### 1. GET /api/recommendations/:userId

Gibt personalisierte Empfehlungen für einen Nutzer zurück.

**Method:** `POST`  
**Path:** `/api/recommendations/:userId`  
**Path Parameters:**
- `userId` (string, required) - ID des Nutzers

**Request Body:**
```json
{
  "limit": 10,
  "filters": {
    "category": "electronics",
    "tags": "popular"
  }
}
```

**Request Body Schema:**
- `limit` (number, optional) - Anzahl der gewünschten Empfehlungen (1-50, default: 10)
- `filters` (object, optional) - Filter für Empfehlungen
  - `category` (string, optional) - Filter nach Kategorie
  - `tags` (string, optional) - Filter nach Tag

**Response 200:**
```json
{
  "userId": "user123",
  "recommendations": [
    {
      "itemId": "item456",
      "score": 0.95,
      "reason": "Kombiniert aus Embedding-Ähnlichkeit und ähnlichen Nutzern",
      "method": "hybrid",
      "item": {
        "id": "item456",
        "title": "Product Name",
        "description": "Product description",
        "tags": ["tag1", "tag2"],
        "category": "electronics",
        "embedding": [0.1, 0.2, 0.3],
        "metadata": {},
        "createdAt": "2024-01-11T10:00:00Z"
      }
    }
  ],
  "totalCount": 10,
  "generatedAt": "2024-01-11T10:00:00Z"
}
```

**Response 400:**
```json
{
  "error": "Invalid request parameters"
}
```

**Response 404:**
```json
{
  "error": "User not found"
}
```

---

### 2. POST /api/preferences

Speichert Nutzerpräferenzen (Bewertungen, Interaktionen).

**Method:** `POST`  
**Path:** `/api/preferences`

**Request Body:**
```json
{
  "userId": "user123",
  "itemId": "item456",
  "rating": 5,
  "interactionType": "purchase"
}
```

**Request Body Schema:**
- `userId` (string, required) - ID des Nutzers
- `itemId` (string, required) - ID des Items
- `rating` (number, required) - Bewertung von 1-5
- `interactionType` (string, required) - Typ der Interaktion ("view", "like", "purchase", etc.)

**Response 200:**
```json
{
  "id": "pref123",
  "userId": "user123",
  "itemId": "item456",
  "rating": 5,
  "timestamp": "2024-01-11T10:00:00Z",
  "interactionType": "purchase"
}
```

**Response 400:**
```json
{
  "error": "Invalid preference data"
}
```

**Response 404:**
```json
{
  "error": "User or Item not found"
}
```

---

### 3. GET /api/users/:userId/history

Ruft den Nutzerverlauf ab (alle Interaktionen).

**Method:** `GET`  
**Path:** `/api/users/:userId/history`

**Path Parameters:**
- `userId` (string, required) - ID des Nutzers

**Query Parameters:**
- `limit` (number, optional) - Maximale Anzahl von Einträgen

**Response 200:**
```json
[
  {
    "id": "pref123",
    "userId": "user123",
    "itemId": "item456",
    "rating": 5,
    "timestamp": "2024-01-11T10:00:00Z",
    "interactionType": "purchase"
  },
  {
    "id": "pref124",
    "userId": "user123",
    "itemId": "item789",
    "rating": 4,
    "timestamp": "2024-01-10T15:30:00Z",
    "interactionType": "view"
  }
]
```

**Response 400:**
```json
{
  "error": "Invalid userId"
}
```

---

### 4. POST /api/feedback

Sammelt Feedback zu Empfehlungen.

**Method:** `POST`  
**Path:** `/api/feedback`

**Request Body:**
```json
{
  "userId": "user123",
  "itemId": "item456",
  "feedbackType": "positive",
  "comment": "Great recommendation!"
}
```

**Request Body Schema:**
- `userId` (string, required) - ID des Nutzers
- `itemId` (string, required) - ID des Items
- `feedbackType` (string, required) - Typ des Feedbacks ("positive", "negative", "not_interested")
- `comment` (string, optional) - Optionaler Kommentar

**Response 200:**
```json
{
  "id": "feedback123",
  "userId": "user123",
  "itemId": "item456",
  "feedbackType": "positive",
  "timestamp": "2024-01-11T10:00:00Z",
  "comment": "Great recommendation!"
}
```

**Response 400:**
```json
{
  "error": "Invalid feedback type"
}
```

---

### 5. GET /api/items/:itemId/similar

Findet ähnliche Items basierend auf Vector-Ähnlichkeit.

**Method:** `GET`  
**Path:** `/api/items/:itemId/similar`

**Path Parameters:**
- `itemId` (string, required) - ID des Items

**Query Parameters:**
- `limit` (number, optional) - Maximale Anzahl von ähnlichen Items (default: 10)

**Response 200:**
```json
{
  "itemId": "item456",
  "similarItems": [
    {
      "id": "item789",
      "title": "Similar Product",
      "description": "Similar product description",
      "tags": ["tag1", "tag3"],
      "category": "electronics",
      "embedding": [0.15, 0.25, 0.35],
      "metadata": {},
      "createdAt": "2024-01-11T09:00:00Z"
    }
  ],
  "totalCount": 5
}
```

**Response 400:**
```json
{
  "error": "Invalid itemId"
}
```

**Response 404:**
```json
{
  "error": "Item not found"
}
```

---

## Fehlerbehandlung

Alle Endpoints verwenden standardisierte HTTP Status Codes:

- `200 OK` - Erfolgreiche Anfrage
- `400 Bad Request` - Ungültige Anfrage-Parameter
- `404 Not Found` - Ressource nicht gefunden
- `500 Internal Server Error` - Server-Fehler

## Rate Limiting

Standardmäßig: 100 Requests pro Minute pro IP-Adresse.

Konfigurierbar in `velin.config.json`:
```json
{
  "security": {
    "rateLimit": {
      "enabled": true,
      "requestsPerMinute": 100
    }
  }
}
```

## Authentifizierung

Aktuell optional. Konfigurierbar in `velin.config.json`:
```json
{
  "security": {
    "apiKeyRequired": false
  }
}
```

## CORS

Standardmäßig aktiviert für:
- `http://localhost:3000`
- `http://localhost:5173`

Konfigurierbar in `velin.config.json`:
```json
{
  "api": {
    "cors": {
      "enabled": true,
      "origins": ["http://localhost:3000"]
    }
  }
}
```
