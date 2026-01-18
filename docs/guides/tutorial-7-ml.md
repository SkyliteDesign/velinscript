# Tutorial 7: ML/LLM Integration

Lerne, wie du KI/ML-Features in VelinScript verwendest.

> **Hinweis**: Für detaillierte Informationen zum Model Training siehe [ML Training Tutorial](tutorial-ml-training.md).

## ML Model Loading

### Model laden

```velin
let loader: ModelLoader = ModelLoader.new();
loader.loadModel("sentiment", "sentiment", "models/sentiment.onnx");
loader.loadModel("classifier", "classification", "models/classifier.onnx");
```

### Prediction

```velin
@AI(model: "sentiment")
@POST("/api/analyze/sentiment")
fn analyzeSentiment(text: string): SentimentResult {
    let loader: ModelLoader = ModelLoader.new();
    let prediction = loader.predict("sentiment", text);
    
    return SentimentResult {
        text: text,
        sentiment: prediction,
    };
}
```

## LLM Integration

### OpenAI, Anthropic, Gemini, Local

Die LLM-Integration ist nun vollständig implementiert und unterstützt echte API-Calls.

```velin
@POST("/api/chat")
fn chat(message: string): string {
    // Unterstützte Provider: "openai", "anthropic", "gemini", "local"
    // "local" Modus simuliert Antworten für Tests ohne API-Kosten
    let llm: LLMClient = LLMClient.new("openai", "api-key");
    
    // Asynchroner Aufruf via HTTP Client
    let result = await llm.generate(message);
    return result;
}
```

### Embeddings

```velin
@POST("/api/embed")
fn embed(text: string): List<number> {
    let llm: LLMClient = LLMClient.new("openai", "api-key");
    // Generiert echte Vektor-Embeddings (z.B. 1536 Dimensionen für OpenAI)
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

### Basis-Training

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

### ONNX Training

```velin
let config = ONNXTrainingConfig {
    epochs: 100,
    batch_size: 32,
    learning_rate: 0.001,
    optimizer: "Adam",
    loss_function: "CrossEntropy"
};

let result = training.train_with_onnx("my_model", config);
```

### TensorFlow Training

```velin
let config = TensorFlowTrainingConfig {
    epochs: 100,
    batch_size: 32,
    learning_rate: 0.001,
    optimizer: "Adam",
    loss_function: "SparseCategoricalCrossentropy",
    validation_split: 0.2
};

let result = training.train_with_tensorflow("tf_model", config);
```

### Model Evaluation

```velin
let testData = [
    TrainingExample { input: "test1", output: "expected1" }
];

let evalResult = training.evaluate_model("my_model", testData);
// evalResult.accuracy, evalResult.precision, evalResult.recall, evalResult.f1_score
```

> **Siehe auch**: [ML Training Tutorial](tutorial-ml-training.md) für detaillierte Informationen.

## Best Practices

1. **Model Caching** für Performance
2. **Error Handling** für API Calls
3. **Rate Limiting** für LLM APIs
4. **Cost Management** für externe APIs

## Nächste Schritte

- [API Documentation](../api/) - Vollständige API-Referenz
- [Language Specification](../language/specification.md) - Sprach-Spezifikation
