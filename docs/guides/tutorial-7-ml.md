# Tutorial 7: ML/LLM Integration

Lerne, wie du KI/ML-Features in VelinScript verwendest.

## ML Model Loading

### Model laden

```velin
let mut loader = ModelLoader::new();
loader.load_model("sentiment", ModelType::Sentiment, "models/sentiment.onnx");
loader.load_model("classifier", ModelType::Classification, "models/classifier.onnx");
```

### Prediction

```velin
@AI(model: "sentiment")
@POST("/api/analyze/sentiment")
fn analyzeSentiment(text: string): SentimentResult {
    let loader = ModelLoader::new();
    let prediction = loader.predict("sentiment", text);
    
    return SentimentResult {
        text: text,
        sentiment: prediction,
    };
}
```

## LLM Integration

### OpenAI

```velin
@POST("/api/chat")
fn chat(message: string): string {
    let llm = LLMClient::new(LLMProvider::OpenAI, getApiKey());
    return llm.generate(message);
}
```

### Embeddings

```velin
@POST("/api/embed")
fn embed(text: string): List<number> {
    let llm = LLMClient::new(LLMProvider::OpenAI, getApiKey());
    return llm.embed(text);
}
```

## Vector Database

### Vektoren speichern

```velin
@POST("/api/documents")
fn createDocument(text: string): Document {
    let llm = LLMClient::new(LLMProvider::OpenAI, getApiKey());
    let embedding = llm.embed(text);
    
    let db = VectorDB::new(VectorDBProvider::Pinecone, getConnectionString());
    let doc = Document {
        id: generateId(),
        text: text,
        embedding: embedding,
    };
    
    db.upsert("documents", doc.id, doc.embedding);
    return db.save(doc);
}
```

### Ähnlichkeitssuche

```velin
@POST("/api/documents/search")
fn searchDocuments(query: string): List<Document> {
    let llm = LLMClient::new(LLMProvider::OpenAI, getApiKey());
    let query_embedding = llm.embed(query);
    
    let db = VectorDB::new(VectorDBProvider::Pinecone, getConnectionString());
    let results = db.search("documents", query_embedding, 10);
    
    return results.map(|r| db.find(Document, r.id));
}
```

## Model Training

```velin
@POST("/api/train")
fn trainModel(modelName: string): void {
    let mut training = TrainingService::new();
    
    // Training Data hinzufügen
    training.add_example("input1", "output1");
    training.add_example("input2", "output2");
    
    // Training starten
    training.train(modelName);
}
```

## Best Practices

1. **Model Caching** für Performance
2. **Error Handling** für API Calls
3. **Rate Limiting** für LLM APIs
4. **Cost Management** für externe APIs

## Nächste Schritte

- [API Documentation](../api/) - Vollständige API-Referenz
- [Language Specification](../language/specification.md) - Sprach-Spezifikation
