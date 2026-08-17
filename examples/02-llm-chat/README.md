# 02-llm-chat

**Erstes KI-Beispiel.**

## Zweck

- LLM zeigen
- echte KI-API
- wenig Code

## Inhalt

- Chat-Endpoint
- LLMClient
- Streaming optional

## Voraussetzungen

Du brauchst einen API-Key für einen LLM-Provider:

- **OpenAI**: [API Key erstellen](https://platform.openai.com/api-keys)
- **Anthropic**: [API Key erstellen](https://console.anthropic.com/)
- **Google Gemini**: [API Key erstellen](https://makersuite.google.com/app/apikey)

## Setup

### 1. API-Key setzen

```bash
# Windows (PowerShell)
$env:OPENAI_API_KEY = "sk-..."

# Linux/Mac
export OPENAI_API_KEY="sk-..."
```

### 2. Provider in main.velin anpassen

Öffne `main.velin` und ändere:

```velin
// Für OpenAI
let llmClient = LLMClient::new(LLMProvider::OpenAI, getApiKey());

// Für Anthropic Claude
let llmClient = LLMClient::new(LLMProvider::Anthropic, getApiKey());

// Für Google Gemini
let llmClient = LLMClient::new(LLMProvider::GoogleGemini, getApiKey());
```

### 3. Kompilieren

```bash
cd examples/02-llm-chat
velin compile -i main.velin -o main.rs
```

### 4. Ausführen

```bash
cargo run --release
```

Die API läuft dann auf `http://localhost:8080`

## Testen

```bash
# Chat-Request senden
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hallo, wie geht es dir?"}'
```

**Response:**
```json
{
  "content": "Hallo! Mir geht es gut, danke der Nachfrage...",
  "model": "gpt-4"
}
```

## Was du lernst

- Wie man `LLMClient` verwendet
- Wie man verschiedene LLM-Provider nutzt
- Wie man async/await mit LLMs verwendet
- Wie einfach KI-Integration in VelinScript ist

## Nächste Schritte

- **Automatisierung?** → Siehe `03-automation-pipeline`
- **Volles System?** → Siehe `04-custom-recommender`
