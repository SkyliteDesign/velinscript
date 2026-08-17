# Vektor-Datenbanken - Semantische Suche & RAG

**Version:** 3.1.0  
**Status:** ✅ Vollständig dokumentiert

---

## Übersicht

VelinScript bietet native Unterstützung für Vektor-Datenbanken (Vector Databases) für semantische Suche, Retrieval Augmented Generation (RAG) und Embedding-basierte Anwendungen.

**Unterstützte Provider:**
- ✅ **Pinecone** - Cloud-basierte Vektor-Datenbank
- ✅ **Weaviate** - Open-Source Vektor-Datenbank
- ✅ **Qdrant** - Production-Ready Vektor-Datenbank
- ✅ **Local** - In-Memory für Development/Testing

---

## Was sind Vektor-Datenbanken?

Vektor-Datenbanken speichern Daten als **hochdimensionale Vektoren** (Embeddings) und ermöglichen **semantische Suche** basierend auf Ähnlichkeit statt exakter Schlüsselwort-Matches.

### Vorteile

- **Semantische Suche:** Findet ähnliche Inhalte basierend auf Bedeutung
- **RAG (Retrieval Augmented Generation):** Füttert LLMs mit relevantem Kontext
- **Empfehlungssysteme:** Findet ähnliche Items basierend auf Embeddings
- **Klassifizierung:** Gruppiert ähnliche Dokumente automatisch

---

## Quick Start

### 1. VectorDB initialisieren

```velin
use ml;

// Initialisiere VectorDB
let vectorDB = VectorDB::new(
    VectorDBProvider::Pinecone,
    "your-connection-string"
);
```

### 2. Embedding generieren

```velin
use ml;

let llmClient = LLMClient::new(LLMProvider::OpenAI, apiKey);

// Generiere Embedding für Text
let embedding = llmClient.embed("Dies ist ein Beispieltext");
```

### 3. Vektor speichern

```velin
// Speichere Vektor in Collection
vectorDB.upsert("documents", documentId, embedding);
```

### 4. Semantische Suche

```velin
// Suche ähnliche Vektoren
let results = vectorDB.search("documents", queryEmbedding, 10);
```

---

## VectorDB-Provider

### Pinecone

**Cloud-basierte Vektor-Datenbank** - Ideal für Production.

```velin
let vectorDB = VectorDB::new(
    VectorDBProvider::Pinecone,
    "https://your-index.svc.environment.pinecone.io"
);
```

**Features:**
- ✅ Managed Service
- ✅ Hohe Performance
- ✅ Skalierbar
- ✅ Production-Ready

**Connection String Format:**
```
https://<index-name>.svc.<environment>.pinecone.io
```

### Weaviate

**Open-Source Vektor-Datenbank** - Ideal für Self-Hosting.

```velin
let vectorDB = VectorDB::new(
    VectorDBProvider::Weaviate,
    "http://localhost:8080"
);
```

**Features:**
- ✅ Open-Source
- ✅ GraphQL API
- ✅ Self-Hosted möglich
- ✅ Flexible Schema

### Qdrant

**Production-Ready Vektor-Datenbank** - Ideal für Performance.

```velin
let vectorDB = VectorDB::new(
    VectorDBProvider::Qdrant,
    "http://localhost:6333"
);
```

**Features:**
- ✅ High Performance
- ✅ Rust-basiert
- ✅ Self-Hosted möglich
- ✅ Production-Ready

### Local

**In-Memory Vektor-Datenbank** - Ideal für Development/Testing.

```velin
let vectorDB = VectorDB::new(
    VectorDBProvider::Local,
    ""
);
```

**Features:**
- ✅ Keine externe Abhängigkeit
- ✅ Schnell für Tests
- ✅ Keine Persistenz
- ⚠️ Nicht für Production

---

## Embedding-Modul

### Embedding generieren

Das `embedding` Modul bietet Funktionen für Embedding-Operationen:

```velin
use embedding;

// Embedding generieren (via LLM)
let embedding = llmClient.embed("Text zum Einbetten");

// Embedding normalisieren
let normalized = embedding.normalize(embedding);

// Ähnlichkeit berechnen
let similarity = embedding.similarity(embedding1, embedding2);

// Dimensionen
let dimensions = embedding.dimension(embedding);
```

### Embedding-Funktionen

#### `embedding.embed(text: string): List<number>`

Generiert Embedding für einen Text.

```velin
let embedding = embedding.embed("Dies ist ein Beispiel");
// Ergebnis: [0.123, -0.456, 0.789, ...] (1536 Dimensionen)
```

#### `embedding.similarity(a: List<number>, b: List<number>): number`

Berechnet Cosine Similarity zwischen zwei Embeddings.

```velin
let similarity = embedding.similarity(embedding1, embedding2);
// Ergebnis: 0.0 bis 1.0 (1.0 = identisch, 0.0 = komplett unterschiedlich)
```

#### `embedding.normalize(embedding: List<number>): List<number>`

Normalisiert einen Embedding-Vektor (Länge = 1.0).

```velin
let normalized = embedding.normalize(embedding);
```

#### `embedding.dimension(embedding: List<number>): number`

Gibt die Dimension eines Embeddings zurück.

```velin
let dims = embedding.dimension(embedding);
// Ergebnis: 1536 (für OpenAI text-embedding-ada-002)
```

---

## VectorDB-API

### `VectorDB::new(provider: VectorDBProvider, connection_string: string): VectorDB`

Erstellt eine neue VectorDB-Instanz.

```velin
let vectorDB = VectorDB::new(
    VectorDBProvider::Pinecone,
    "https://index.svc.env.pinecone.io"
);
```

### `vectorDB.upsert(collection: string, id: string, vector: List<number>): Result<void, string>`

Speichert oder aktualisiert einen Vektor in einer Collection.

```velin
// Speichere Vektor
let result = vectorDB.upsert("documents", "doc-123", embedding);

if (result.isErr()) {
    log.error("Fehler beim Speichern: " + result.err().unwrap());
}
```

**Parameter:**
- `collection` - Collection-Name (z.B. "documents", "items")
- `id` - Eindeutige ID für den Vektor
- `vector` - Embedding-Vektor (List<number>)

### `vectorDB.search(collection: string, query_vector: List<number>, top_k: number): Result<List<SearchResult>, string>`

Sucht ähnliche Vektoren in einer Collection.

```velin
// Suche ähnliche Vektoren
let results = vectorDB.search("documents", queryEmbedding, 10);

if (results.isOk()) {
    for (result in results.unwrap()) {
        log.info("ID: " + result.id + ", Score: " + result.score);
    }
}
```

**Parameter:**
- `collection` - Collection-Name
- `query_vector` - Query-Embedding
- `top_k` - Anzahl der Ergebnisse

**Rückgabe:**
```velin
struct SearchResult {
    id: string,
    score: number,  // Similarity-Score (0.0 bis 1.0)
}
```

---

## Vollständiges Beispiel

### RAG-System (Retrieval Augmented Generation)

```velin
use ml;
use embedding;

// Initialisiere Services
let llmClient = LLMClient::new(LLMProvider::OpenAI, apiKey);
let vectorDB = VectorDB::new(VectorDBProvider::Pinecone, connectionString);

struct Document {
    id: string,
    title: string,
    content: string,
    embedding: List<number>,
}

// 1. Dokumente indizieren
@POST("/api/documents")
fn createDocument(title: string, content: string): Document {
    // Embedding generieren
    let embedding = llmClient.embed(content);
    
    let document = Document {
        id: generateId(),
        title: title,
        content: content,
        embedding: embedding,
    };
    
    // In VectorDB speichern
    vectorDB.upsert("documents", document.id, document.embedding);
    
    // In normale DB speichern
    return db.save(document);
}

// 2. Semantische Suche
@POST("/api/documents/search")
fn searchDocuments(query: string, topK: number): List<Document> {
    // Query-Embedding generieren
    let queryEmbedding = llmClient.embed(query);
    
    // Suche in VectorDB
    let searchResults = vectorDB.search("documents", queryEmbedding, topK);
    
    if (searchResults.isErr()) {
        return [];
    }
    
    // Hole vollständige Dokumente aus DB
    let mut documents = List<Document>();
    for (result in searchResults.unwrap()) {
        let doc = db.find(Document, result.id);
        if (doc != null) {
            documents.push(doc);
        }
    }
    
    return documents;
}

// 3. RAG mit LLM
@POST("/api/rag/query")
fn ragQuery(question: string): string {
    // 1. Suche relevante Dokumente
    let relevantDocs = searchDocuments(question, 5);
    
    // 2. Erstelle Kontext
    let context = relevantDocs
        .map(|doc| doc.content)
        .join("\n\n");
    
    // 3. LLM mit Kontext
    let prompt = format("""
        Kontext:
        {}
        
        Frage: {}
        
        Antworte basierend auf dem Kontext.
    """, context, question);
    
    return await llmClient.generate(prompt);
}
```

---

## Empfehlungssysteme

### Item-basierte Empfehlungen

```velin
struct Item {
    id: string,
    name: string,
    description: string,
    embedding: List<number>,
}

// Item-Embedding generieren
fn generateItemEmbedding(item: Item): List<number> {
    let combinedText = item.name + " " + item.description;
    return llmClient.embed(combinedText);
}

// Item speichern
fn storeItem(item: Item): void {
    // Embedding generieren falls nicht vorhanden
    if (item.embedding.length == 0) {
        item.embedding = generateItemEmbedding(item);
    }
    
    // In VectorDB speichern
    vectorDB.upsert("items", item.id, item.embedding);
    
    // In normale DB speichern
    db.save(item);
}

// Ähnliche Items finden
@GET("/api/items/:id/similar")
fn getSimilarItems(id: string, limit: number): List<Item> {
    // Hole Item
    let item = db.find(Item, id);
    if (item == null) {
        return [];
    }
    
    // Suche ähnliche Items
    let searchResults = vectorDB.search("items", item.embedding, limit);
    
    if (searchResults.isErr()) {
        return [];
    }
    
    // Konvertiere zu Items
    let mut similarItems = List<Item>();
    for (result in searchResults.unwrap()) {
        // Überspringe das ursprüngliche Item
        if (result.id == id) {
            continue;
        }
        
        let similarItem = db.find(Item, result.id);
        if (similarItem != null) {
            similarItems.push(similarItem);
        }
    }
    
    return similarItems;
}
```

---

## Clustering

### Dokumente automatisch gruppieren

```velin
use embedding;

// Clustering von Embeddings
fn clusterDocuments(documents: List<Document>, k: number): List<List<Document>> {
    // Extrahiere Embeddings
    let embeddings = documents.map(|doc| doc.embedding);
    
    // Clustering
    let clusters = embedding.cluster(embeddings, k);
    
    // Gruppiere Dokumente nach Clustern
    let mut documentClusters = List<List<Document>>();
    for (cluster in clusters) {
        let mut clusterDocs = List<Document>();
        for (embedding in cluster) {
            // Finde Dokument mit diesem Embedding
            let doc = documents.find(|d| d.embedding == embedding);
            if (doc != null) {
                clusterDocs.push(doc);
            }
        }
        documentClusters.push(clusterDocs);
    }
    
    return documentClusters;
}
```

---

## Best Practices

### Embedding-Dimensionen

- **OpenAI text-embedding-ada-002:** 1536 Dimensionen
- **Google Gemini embedding-001:** 768 Dimensionen
- **Anthropic:** Noch keine öffentliche Embedding-API

### Collection-Organisation

1. **Eine Collection pro Datentyp:**
   - `documents` - Dokumente
   - `items` - Items
   - `users` - User-Profile

2. **Konsistente IDs:**
   - Verwende UUIDs oder eindeutige IDs
   - IDs sollten mit DB-IDs übereinstimmen

### Performance

1. **Batch-Upserts:** Speichere mehrere Vektoren gleichzeitig
2. **Index-Optimierung:** Nutze Provider-spezifische Index-Optionen
3. **Caching:** Cache häufig verwendete Embeddings

### Sicherheit

1. **API-Keys sicher speichern:** Nutze Umgebungsvariablen
2. **Connection Strings:** Nicht in Code hardcoden
3. **Access Control:** Nutze Provider-spezifische Access Controls

---

## Fehlerbehandlung

### VectorDB-Fehler

```velin
let result = vectorDB.upsert("documents", id, embedding);

match result {
    Ok(_) => {
        log.info("Vektor erfolgreich gespeichert");
    }
    Err(error) => {
        log.error("Fehler: " + error);
        // Fehlerbehandlung
    }
}
```

### Häufige Fehler

1. **Connection Error:** Prüfe Connection String
2. **Dimension Mismatch:** Stelle sicher, dass alle Embeddings die gleiche Dimension haben
3. **Collection nicht gefunden:** Erstelle Collection zuerst im Provider

---

## Provider-spezifische Konfiguration

### Pinecone

```velin
// Connection String Format
let connectionString = "https://<index-name>.svc.<environment>.pinecone.io";

// API Key in Umgebungsvariable
let apiKey = env.get("PINECONE_API_KEY");
```

### Weaviate

```velin
// Connection String
let connectionString = "http://localhost:8080";

// Optional: API Key
let apiKey = env.get("WEAVIATE_API_KEY");
```

### Qdrant

```velin
// Connection String
let connectionString = "http://localhost:6333";

// Optional: API Key
let apiKey = env.get("QDRANT_API_KEY");
```

---

## Siehe auch

- [LLM Integration](tutorial-7-ml.md) - Machine Learning & LLM-Integration
- [AI/ML Guide](ai-ml.md) - KI & Machine Learning
- [Standard Library](../api/standard-library.md) - Embedding & ML Module
- [Custom Recommender Example](../../examples/custom-recommender/) - Vollständiges Beispiel

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
