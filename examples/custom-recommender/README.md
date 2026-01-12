# Custom Recommender - Hybrid Recommendation System

Ein vollständiges Beispiel für ein hybrides Recommendation System in VelinScript, das Embedding-basierte und Collaborative Filtering Ansätze kombiniert.

## Übersicht

Dieses Beispiel zeigt, wie man ein intelligentes Recommendation System entwickelt, das:
- **Hybrid Recommendation** verwendet (Kombination aus Embeddings und Collaborative Filtering)
- **Vector Database** für Embedding-basierte Ähnlichkeitssuche nutzt
- **User Preference Tracking** implementiert
- **Feedback System** für kontinuierliche Verbesserung bietet

## Architektur

```
┌─────────────────────────────────────────────────────────┐
│                    API Endpoints                         │
│  (main.velin)                                           │
│  - GET /api/recommendations/:userId                     │
│  - POST /api/preferences                                │
│  - GET /api/users/:userId/history                       │
│  - POST /api/feedback                                   │
│  - GET /api/items/:itemId/similar                       │
└──────────────────┬──────────────────────────────────────┘
                   │
       ┌───────────┴───────────┐
       │                       │
┌──────▼──────────┐   ┌────────▼──────────┐
│ Recommendation  │   │  Vector Search     │
│ (recommendation.│   │  (vector_search.   │
│  velin)         │   │   velin)           │
│                 │   │                    │
│ - Hybrid        │   │ - Embeddings       │
│ - Collaborative │   │ - Vector DB       │
│ - Embedding     │   │ - Similarity      │
└─────────────────┘   └───────────────────┘
       │                       │
       └───────────┬───────────┘
                   │
          ┌────────▼────────┐
          │   Models        │
          │  (models.velin) │
          │                 │
          │ - User          │
          │ - Item          │
          │ - Preference    │
          │ - Recommendation│
          └─────────────────┘
```

## Dateistruktur

```
custom-recommender/
├── main.velin              # Hauptdatei mit API-Endpoints (optimiert)
├── models.velin            # Datenmodelle (User, Item, Preference, etc.)
├── recommendation.velin    # Recommendation Algorithmus
├── vector_search.velin     # Vector Database Integration
├── config.velin            # Konfigurations-Loading
├── security.velin           # Security-Features (erweitert)
├── wasm.velin              # WebAssembly Support
├── responses.velin         # Standardisierte Response-Formate
├── errors.velin            # Error-Definitionen und Handling
├── logging.velin           # Strukturiertes Logging-System
├── cache.velin             # Caching-Implementation
├── health.velin            # Health Check und Metrics
├── async.velin             # Asynchrone Operationen
├── velin.config.json       # Konfigurationsdatei (nicht in Git)
├── velin.config.example.json # Beispiel-Konfiguration
├── README.md               # Diese Datei
├── API_ENDPOINTS.md        # API-Dokumentation
├── SECURITY.md             # Security-Guide
├── WASM.md                 # WebAssembly-Dokumentation
└── tests/
    ├── unit/
    │   └── main_test.velin # Unit Tests
    └── integration/
        └── api_test.velin  # Integration Tests
```

## Features

### 1. Optimiertes API-Design

- **Standardisiertes Response-Format** - Einheitliche API-Responses mit Metadata
- **Umfassendes Error Handling** - Try-Catch + Result Types für alle Endpoints
- **Strukturiertes Logging** - Request/Response/Performance-Logging mit JSON-Format
- **Intelligentes Caching** - Automatisches Caching für Recommendations, Embeddings, History
- **Security-Middleware** - API Key, Rate Limiting, CORS, Input Validation in allen Endpoints
- **Input-Sanitization** - Automatische XSS- und SQL-Injection-Prävention
- **Asynchrone Operationen** - Background Jobs für teure Operationen
- **Health Monitoring** - Health Check, Metrics und Readiness Endpoints

### 2. Hybrid Recommendation Algorithmus

Das System kombiniert zwei Ansätze:

- **Embedding-basierte Empfehlungen (60% Gewichtung)**
  - Nutzt Vector-Embeddings für semantische Ähnlichkeit
  - Findet Items basierend auf Text-Ähnlichkeit (Titel, Beschreibung, Tags)

- **Collaborative Filtering (40% Gewichtung)**
  - Findet ähnliche Nutzer basierend auf gemeinsamen Präferenzen
  - Empfiehlt Items die ähnliche Nutzer hoch bewertet haben

### 2. Vector Database Integration

- **Embedding-Generierung**: Nutzt LLM (z.B. OpenAI) für Text-Embeddings
- **Vector Search**: Schnelle Ähnlichkeitssuche in Vector Database
- **Item Embeddings**: Automatische Embedding-Generierung für Items
- **User Embeddings**: Dynamische Embedding-Generierung basierend auf Nutzerverhalten

### 3. User Preference Tracking

- Speichert Nutzerbewertungen (1-5 Sterne)
- Trackt verschiedene Interaktionstypen (view, like, purchase)
- Ermöglicht personalisierte Empfehlungen

### 4. Feedback System

- Sammelt explizites Feedback zu Empfehlungen
- Unterstützt: positive, negative, not_interested
- Kann für Model-Training und Algorithmus-Verbesserung verwendet werden

### 5. Production-Ready Features

- **Error Handling** - Try-Catch mit strukturierten Fehlermeldungen
- **Logging** - Umfassendes Logging mit Performance-Metriken
- **Caching** - LRU-Cache mit konfigurierbaren TTLs
- **Security** - Vollständige Security-Middleware in allen Endpoints
- **Monitoring** - Health Checks und Metrics-Endpoints
- **Async Jobs** - Background-Jobs für Embedding-Generierung und Analytics
- **Tests** - Unit- und Integration-Tests für alle Features

## API-Dokumentation

### GET /api/recommendations/:userId

Gibt personalisierte Empfehlungen für einen Nutzer zurück.

**Request:**
```json
{
  "limit": 10,
  "filters": {
    "category": "electronics",
    "tags": "popular"
  }
}
```

**Response:**
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
        "description": "...",
        "tags": ["tag1", "tag2"]
      }
    }
  ],
  "totalCount": 10,
  "generatedAt": "2024-01-11T10:00:00Z"
}
```

### POST /api/preferences

Speichert Nutzerpräferenzen.

**Request:**
```json
{
  "userId": "user123",
  "itemId": "item456",
  "rating": 5,
  "interactionType": "purchase"
}
```

### GET /api/users/:userId/history

Ruft den Nutzerverlauf ab.

**Query Parameters:**
- `limit` (optional): Maximale Anzahl von Einträgen

### POST /api/feedback

Sammelt Feedback zu Empfehlungen.

**Request:**
```json
{
  "userId": "user123",
  "itemId": "item456",
  "feedbackType": "positive",
  "comment": "Great recommendation!"
}
```

### GET /api/items/:itemId/similar

Findet ähnliche Items.

**Query Parameters:**
- `limit` (optional): Maximale Anzahl von ähnlichen Items

## Setup-Anleitung

### 1. Voraussetzungen

- VelinScript Compiler installiert
- Vector Database (optional, für Production)
- LLM API Key (z.B. OpenAI) für Embedding-Generierung

### 2. Konfiguration

#### Umgebungsvariablen setzen

```bash
# Windows (PowerShell)
$env:OPENAI_API_KEY = "sk-..."
$env:ANTHROPIC_API_KEY = "sk-ant-..."
$env:GOOGLE_GEMINI_API_KEY = "AIza..."
$env:JWT_SECRET = "your-secret-key-here"
$env:VECTOR_DB_CONNECTION = "your-connection-string"

# Linux/Mac
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GOOGLE_GEMINI_API_KEY="AIza..."
export JWT_SECRET="your-secret-key-here"
export VECTOR_DB_CONNECTION="your-connection-string"
```

#### Konfigurationsdatei erstellen

```bash
# Kopiere Beispiel-Konfiguration
cp velin.config.example.json velin.config.json

# Bearbeite velin.config.json und setze deine Werte
# API Keys werden automatisch aus Umgebungsvariablen geladen
```

#### Konfigurationsdatei anpassen

Bearbeite `velin.config.json`:
- Setze `ml.llm.apiKey` auf `${OPENAI_API_KEY}` (wird automatisch geladen)
- Setze `ml.llm.anthropicApiKey` auf `${ANTHROPIC_API_KEY}` für Claude
- Setze `ml.llm.geminiApiKey` auf `${GOOGLE_GEMINI_API_KEY}` für Gemini
- Setze `security.jwt.secret` auf `${JWT_SECRET}` für JWT-Authentifizierung
- Passe `ml.recommendation.embeddingWeight` und `collaborativeWeight` an
- Konfiguriere `database.connectionString` für Production
- Aktiviere Security-Features für Production (`security.apiKeyRequired: true`)

### 3. Kompilieren

```bash
cd examples/custom-recommender
velin compile -i main.velin -o main.rs
```

### 4. Tests ausführen

```bash
# Unit Tests
velin test tests/unit/main_test.velin

# Integration Tests
velin test tests/integration/api_test.velin

# Alle Tests
velin test tests/
```

### 4. Ausführen

```bash
# Kompiliertes Rust-Programm ausführen
cargo run --release
```

## Verwendungsbeispiele

### Beispiel 1: Empfehlungen abrufen

```bash
curl -X POST http://localhost:8080/api/recommendations/user123 \
  -H "Content-Type: application/json" \
  -d '{
    "limit": 10,
    "filters": {
      "category": "electronics"
    }
  }'
```

### Beispiel 2: Präferenz speichern

```bash
curl -X POST http://localhost:8080/api/preferences \
  -H "Content-Type: application/json" \
  -d '{
    "userId": "user123",
    "itemId": "item456",
    "rating": 5,
    "interactionType": "purchase"
  }'
```

### Beispiel 3: Ähnliche Items finden

```bash
curl http://localhost:8080/api/items/item456/similar?limit=5
```

## Best Practices

1. **Model Loading**: Lade Embedding-Models beim Start, nicht bei jedem Request
2. **Caching**: Cache User-Embeddings und häufige Empfehlungen
3. **Error Handling**: Behandle Vector DB und LLM API Fehler gracefully
4. **Validation**: Validiere alle Inputs vor Verarbeitung
5. **Monitoring**: Track Recommendation Quality und User Feedback
6. **A/B Testing**: Teste verschiedene Gewichtungen für Hybrid-Algorithmus

## Erweiterungsmöglichkeiten

- **Real-time Updates**: Aktualisiere Empfehlungen basierend auf Echtzeit-Interaktionen
- **Cold Start Problem**: Implementiere Content-based Fallbacks für neue Nutzer
- **Diversity**: Füge Diversität in Empfehlungen ein (nicht nur ähnliche Items)
- **Explainability**: Erweitere "reason" Feld mit detaillierten Erklärungen
- **Model Training**: Nutze Feedback für kontinuierliches Model-Training

## Weitere Ressourcen

- [VelinScript ML/LLM Tutorial](../docs/guides/tutorial-7-ml.md)
- [VelinScript Standard Library - ML Module](../../compiler/src/stdlib/ml.rs)
- [VelinScript Sentiment Analysis Beispiel](../ml-sentiment-analysis.velin)

## License

MIT
