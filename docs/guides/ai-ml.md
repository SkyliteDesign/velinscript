# KI & Machine Learning mit VelinScript

VelinScript ist eine der ersten Programmiersprachen, die "AI-Native" entwickelt wurde. Das bedeutet, dass der Zugriff auf große Sprachmodelle (LLMs), Vektor-Datenbanken und Machine-Learning-Workflows genauso einfach ist wie das Lesen einer Datei oder das Abfragen einer Datenbank.

Dieses Handbuch zeigt Ihnen, wie Sie intelligente Anwendungen bauen, die verstehen, generieren und lernen können.

---

## Inhaltsverzeichnis

1.  [Einführung in die AI-Module](#1-einführung-in-die-ai-module)
2.  [LLM Integration (`llm` Modul)](#2-llm-integration-llm-modul)
    *   [Textgenerierung und Prompting](#textgenerierung-und-prompting)
    *   [Chat-Schnittstellen](#chat-schnittstellen)
    *   [Strukturierte Datenextraktion](#strukturierte-datenextraktion)
    *   [Text-Klassifizierung und Sentiment-Analyse](#text-klassifizierung-und-sentiment-analyse)
3.  [Semantische Suche und RAG (`embedding` Modul)](#3-semantische-suche-und-rag-embedding-modul)
    *   [Embeddings verstehen](#embeddings-verstehen)
    *   [Vektorsuche (Nearest Neighbors)](#vektorsuche-nearest-neighbors)
    *   [Clustering](#clustering)
4.  [Machine Learning Training (`intelligence` Modul)](#4-machine-learning-training-intelligence-modul)
    *   [Modelle trainieren und feintunen](#modelle-trainieren-und-feintunen)
    *   [Lokale Inferenz (ONNX/GGUF)](#lokale-inferenz-onnxgguf)
5.  [Best Practices für AI-Apps](#5-best-practices-für-ai-apps)

---

## 1. Einführung in die AI-Module

VelinScript abstrahiert die Komplexität moderner AI-Backends. Sie müssen sich nicht mit HTTP-Requests an OpenAI oder HuggingFace herumschlagen oder PyTorch-Tensoren manuell verwalten.

Die wichtigsten Module sind:
*   `llm`: High-Level Schnittstelle zu Sprachmodellen (GPT-4, Claude, Llama).
*   `embedding`: Werkzeuge für Vektorraum-Operationen.
*   `intelligence`: Low-Level ML-Training und Modell-Management.

Konfiguration (in `velin.config.json`):
```json
{
  "ai": {
    "provider": "openai", // oder "anthropic", "local", "azure"
    "model": "gpt-4-turbo",
    "embeddingModel": "text-embedding-3-small"
  }
}
```

---

## 2. LLM Integration (`llm` Modul)

### Textgenerierung und Prompting

Die einfachste Form der Interaktion ist die Generierung von Text basierend auf einem Prompt.

```velin
use llm

fn generateProductDescription(productName: string, features: List<string>): string {
    let prompt = `
        Erstelle eine attraktive Produktbeschreibung für "${productName}".
        Features: ${features.join(", ")}.
        Tonfall: Professionell aber begeisternd.
    `;
    
    // Synchrone Ausführung (blockiert bis Antwort da ist)
    // Nutzen Sie 'async' für nicht-blockierende Aufrufe in echten Apps
    let description = llm.generate(prompt, { 
        temperature: 0.7, // Kreativität (0.0 - 1.0)
        maxTokens: 500 
    });
    
    return description;
}
```

### Chat-Schnittstellen

Für Chatbots oder Assistenten, die einen Kontext über mehrere Nachrichten behalten müssen.

```velin
use llm

struct ChatMessage { role: string, content: string }

fn chatSession() {
    let history: List<ChatMessage> = [
        { role: "system", content: "Du bist ein hilfreicher Support-Bot für VelinScript." }
    ];
    
    while (true) {
        let userInput = console.read(); // Pseudocode
        history.push({ role: "user", content: userInput });
        
        let response = llm.chat(history);
        
        log.info("Bot: " + response.content);
        history.push({ role: "assistant", content: response.content });
    }
}
```

### Strukturierte Datenextraktion

Eine der mächtigsten Funktionen: Verwandeln Sie unstrukturierten Text in typisierte Structs. Das ist oft zuverlässiger als Regex.

```velin
struct MeetingInfo {
    topic: string,
    participants: List<string>,
    date: string, // ISO Format
    priority: string // "High", "Medium", "Low"
}

fn parseEmail(emailBody: string): MeetingInfo {
    // VelinScript zwingt das LLM, valides JSON zu generieren, 
    // das genau diesem Struct entspricht.
    return llm.extract_entities(emailBody, MeetingInfo);
}

// Beispielaufruf
let email = "Hi Team, lasst uns morgen um 14 Uhr über das Q3 Budget sprechen. Alice und Bob müssen dabei sein. Es ist dringend!";
let info = parseEmail(email);

log.info(info.topic); // "Q3 Budget"
log.info(info.priority); // "High"
```

### Text-Klassifizierung und Sentiment-Analyse

Klassifizieren Sie Texte ohne eigene Trainingsdaten ("Zero-Shot Classification").

```velin
fn analyzeSupportTicket(ticketText: string) {
    // Sentiment (Stimmung): "positive", "negative", "neutral"
    let sentiment = llm.sentiment(ticketText);
    
    if (sentiment == "negative") {
        escalateToManager();
    }
    
    // Klassifizierung in Kategorien
    let category = llm.classify(ticketText, [
        "Rechnungsproblem", 
        "Technischer Bug", 
        "Feature Request", 
        "Sonstiges"
    ]);
    
    routeTicket(category);
}
```

---

## 3. Semantische Suche und RAG (`embedding` Modul)

Retrieval Augmented Generation (RAG) ist der Standard, um LLMs mit eigenem Wissen (Firmendokumente, Wikis) zu füttern. Das `embedding`-Modul ist der Schlüssel dazu.

### Embeddings verstehen

Ein Embedding ist eine Liste von Zahlen (Vektor), die die *Bedeutung* eines Textes repräsentiert. Ähnliche Texte haben mathematisch ähnliche Vektoren.

```velin
use embedding

let text1 = "Der Hund bellt.";
let text2 = "Ein Canis lupus familiaris macht Geräusche.";
let text3 = "Ich esse gerne Pizza.";

let vec1 = embedding.embed(text1);
let vec2 = embedding.embed(text2);
let vec3 = embedding.embed(text3);

// Distanz berechnen (Cosine Similarity)
// Hoher Wert (nahe 1.0) = sehr ähnlich
log.info(embedding.similarity(vec1, vec2)); // ~0.85 (Hohe Ähnlichkeit)
log.info(embedding.similarity(vec1, vec3)); // ~0.10 (Keine Ähnlichkeit)
```

### Vektorsuche (Nearest Neighbors)

Suchen Sie in Ihrer Datenbank nicht nach Schlüsselwörtern ("Hund"), sondern nach Bedeutung ("Haustier").

```velin
// 1. Dokumente indizieren (einmalig)
struct Doc { id: string, content: string, vector: List<number> }

fn indexDocuments(docs: List<string>) {
    for (content in docs) {
        let vector = embedding.embed(content);
        db.save(Doc { 
            id: utils.uuid(), 
            content: content, 
            vector: vector 
        });
    }
}

// 2. Suchen
fn search(query: string): List<string> {
    let queryVector = embedding.embed(query);
    
    // Findet die 3 semantisch ähnlichsten Dokumente
    // Dies nutzt intern einen optimierten Index (z.B. HNSW)
    let results = embedding.find_nearest(queryVector, "docs_collection", 3);
    
    return results.map(|doc| doc.content);
}
```

### Clustering

Gruppieren Sie Daten automatisch, um Themen zu erkennen.

```velin
fn analyzeFeedback(feedbacks: List<string>) {
    // Gruppiert Feedbacks in 5 Cluster
    let clusters = embedding.cluster(feedbacks, 5);
    
    for (cluster in clusters) {
        // Das LLM fasst das Thema des Clusters zusammen
        let topic = llm.summarize(cluster.samples.join("\n"));
        log.info("Thema: " + topic + " (" + cluster.count + " Einträge)");
    }
}
```

---

## 4. Machine Learning Training (`intelligence` Modul)

Für fortgeschrittene Nutzer bietet VelinScript Möglichkeiten, Modelle anzupassen.

### Modelle trainieren und feintunen

Starten Sie Fine-Tuning-Jobs direkt aus dem Code.

```velin
use intelligence

struct TrainingExample { input: string, output: string }

fn fineTuneModel(data: List<TrainingExample>) {
    let trainer = TrainingService.new({
        baseModel: "gpt-3.5-turbo",
        epochs: 3,
        learningRate: 0.001
    });
    
    log.info("Starte Training mit " + data.length() + " Beispielen...");
    
    // Async Job starten
    let jobId = trainer.train(data);
    
    // Auf Fertigstellung warten (oder Webhook nutzen)
    while (trainer.getStatus(jobId) != "succeeded") {
        utils.sleep("10s");
    }
    
    let newModelId = trainer.getModelId(jobId);
    log.info("Neues Modell bereit: " + newModelId);
    
    // Konfiguration aktualisieren, um neues Modell zu nutzen
    config.set("ai.model", newModelId);
}
```

### Lokale Inferenz (ONNX/GGUF)

VelinScript kann Modelle lokal auf der CPU/GPU ausführen, ohne Daten an die Cloud zu senden. Ideal für Datenschutz-kritische Anwendungen.

```velin
use intelligence

// Lädt ein quantisiertes Llama-3 Modell (GGUF Format)
let model = ModelLoader.load("./models/llama-3-8b-q4.gguf", {
    gpuLayers: 32, // Nutzt GPU wenn verfügbar
    contextSize: 4096
});

let response = model.predict("Warum ist der Himmel blau?");
```

---

## 5. Best Practices für AI-Apps

1.  **Immer Caching nutzen:** LLM-Aufrufe sind teuer und langsam. Nutzen Sie `@Cache` oder semantisches Caching, um Antworten auf ähnliche Fragen wiederzuverwenden.
2.  **Graceful Degradation:** Wenn die AI-API nicht erreichbar ist, sollte Ihre App nicht abstürzen. Bauen Sie Fallbacks ein (z.B. klassische Suche statt Vektorsuche).
3.  **User Feedback Loop:** Speichern Sie, ob Nutzer mit der AI-Antwort zufrieden waren, und nutzen Sie diese Daten für späteres Fine-Tuning.
4.  **Prompt Engineering im Code:** Trennen Sie Prompts vom Code (z.B. in Konfigurationsdateien oder Datenbank), um sie ohne Deployment iterieren zu können.
5.  **Sicherheit:** Übergeben Sie niemals ungefilterte Nutzereingaben direkt an `db.execute` oder Shell-Befehle, auch wenn sie vom LLM kommen ("Prompt Injection").

---

*Ende des AI-Guides. VelinScript macht Sie zum AI-Engineer – bauen Sie etwas Großartiges!*
