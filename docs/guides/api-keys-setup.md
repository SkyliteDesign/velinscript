# 🔑 API-Keys Konfiguration

Diese Anleitung zeigt dir, wie du API-Keys für LLM-Features und andere externe Services konfigurierst.

---

## 🚀 Schnellstart

### 1. Umgebungsvariablen setzen

**Windows (PowerShell):**
```powershell
$env:OPENAI_API_KEY = "sk-..."
$env:ANTHROPIC_API_KEY = "sk-ant-..."
$env:GOOGLE_GEMINI_API_KEY = "AIza..."
```

**Windows (CMD):**
```cmd
set OPENAI_API_KEY=sk-...
set ANTHROPIC_API_KEY=sk-ant-...
set GOOGLE_GEMINI_API_KEY=AIza...
```

**Linux/Mac:**
```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GOOGLE_GEMINI_API_KEY="AIza..."
```

### 2. In Code verwenden

```velin
fn getApiKey(): string {
    // Nutze config.get_env() für Umgebungsvariablen
    return config.get_env("OPENAI_API_KEY", "");
}

@POST("/api/chat")
fn chat(message: string): string {
    let llm = LLMClient.new("openai", getApiKey());
    let result = await llm.generate(message);
    return result;
}
```

---

## 📋 Verfügbare API-Keys

### LLM-Provider

#### OpenAI
- **Umgebungsvariable:** `OPENAI_API_KEY`
- **Format:** `sk-...`
- **Erstellen:** [platform.openai.com/api-keys](https://platform.openai.com/api-keys)

#### Anthropic Claude
- **Umgebungsvariable:** `ANTHROPIC_API_KEY`
- **Format:** `sk-ant-...`
- **Erstellen:** [console.anthropic.com](https://console.anthropic.com/)

#### Google Gemini
- **Umgebungsvariable:** `GOOGLE_GEMINI_API_KEY`
- **Format:** `AIza...`
- **Erstellen:** [makersuite.google.com/app/apikey](https://makersuite.google.com/app/apikey)

### Vector Databases

#### Pinecone
- **Umgebungsvariable:** `PINECONE_API_KEY`
- **Erstellen:** [app.pinecone.io](https://app.pinecone.io)

#### Weaviate
- **Umgebungsvariable:** `WEAVIATE_API_KEY`
- **Erstellen:** [cloud.weaviate.io](https://cloud.weaviate.io)

#### Qdrant
- **Umgebungsvariable:** `QDRANT_API_KEY`
- **Erstellen:** [cloud.qdrant.io](https://cloud.qdrant.io)

---

## ⚙️ Konfiguration über velin.config.json

### 1. Config-Datei erstellen

```bash
velin config init
```

### 2. API-Keys in Config eintragen

```json
{
  "ml": {
    "llm": {
      "provider": "openai",
      "apiKey": "${OPENAI_API_KEY}",
      "anthropicApiKey": "${ANTHROPIC_API_KEY}",
      "geminiApiKey": "${GOOGLE_GEMINI_API_KEY}",
      "model": "gpt-4"
    }
  }
}
```

**Wichtig:** Nutze `${VARIABLE_NAME}` für Umgebungsvariablen, nicht die Keys direkt!

### 3. In Code verwenden

```velin
use config;

fn getConfig(): AppConfig {
    return config.loadConfig("velin.config.json");
}

@POST("/api/chat")
fn chat(message: string): string {
    let cfg = getConfig();
    let llm = LLMClient.new(cfg.ml.llm.provider, cfg.ml.llm.apiKey);
    return await llm.generate(message);
}
```

---

## 🔒 Sicherheit

### ✅ Best Practices

1. **Nie API-Keys im Code hardcoden**
   ```velin
   // ❌ SCHLECHT
   let apiKey = "sk-1234567890abcdef";
   
   // ✅ GUT
   let apiKey = config.get_env("OPENAI_API_KEY", "");
   ```

2. **Nutze Umgebungsvariablen**
   - Lokal: `.env` Datei (nicht committen!)
   - Production: Environment Variables im Deployment

3. **Nutze velin.config.json mit Variablen**
   ```json
   {
     "ml": {
       "llm": {
         "apiKey": "${OPENAI_API_KEY}"
       }
     }
   }
   ```

4. **Gitignore beachten**
   ```
   .env
   velin.config.json
   *.key
   ```

### ❌ Was du NICHT tun solltest

- ❌ API-Keys in Git committen
- ❌ API-Keys in Code hardcoden
- ❌ API-Keys in Logs ausgeben
- ❌ API-Keys in öffentlichen Repositories teilen

---

## 🧪 Testing ohne API-Keys

Für Tests kannst du den "local" Provider verwenden:

```velin
@POST("/api/chat")
fn chat(message: string): string {
    // "local" simuliert Antworten ohne API-Kosten
    let llm = LLMClient.new("local", "");
    return await llm.generate(message);
}
```

---

## 📚 Weitere Ressourcen

- **[Tutorial 7: ML/LLM](tutorial-7-ml.md)** - Vollständiges LLM-Tutorial
- **[Security Guide](security.md)** - Security-Best-Practices
- **[02-llm-chat Beispiel](../../examples/02-llm-chat/)** - Praktisches Beispiel

---

## 🆘 Probleme?

### "API Key not found"

1. Prüfe, ob die Umgebungsvariable gesetzt ist:
   ```bash
   # Windows
   echo $env:OPENAI_API_KEY
   
   # Linux/Mac
   echo $OPENAI_API_KEY
   ```

2. Prüfe, ob die Variable im aktuellen Terminal verfügbar ist
3. Starte den Terminal neu, wenn nötig

### "Invalid API Key"

1. Prüfe, ob der Key korrekt kopiert wurde
2. Prüfe, ob der Key noch gültig ist
3. Erstelle einen neuen Key, falls nötig

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.1.0
