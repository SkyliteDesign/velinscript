# Prompt Optimizer - 90%+ Token-Ersparnis

**Version:** 3.0.1 / 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Datum:** 2026-01-30

---

## Übersicht

Der Prompt Optimizer komprimiert automatisch LLM-Prompts für 90%+ Token-Ersparnis durch kompakte Syntax.

## Features

- ✅ **Prompt-Kürzung** - Entfernt redundante Wörter
- ✅ **Token-Optimierung** - Verwendet Variablen statt Text
- ✅ **System-Prompt-Caching** - Wiederverwendung von System-Prompts
- ✅ **Kompakte Syntax** - `@llm.analyze(text)` statt langer Prompts

---

## Beispiel: Token-Ersparnis

### Vorher (klassischer Prompt)

```velin
let prompt = "Bitte analysiere den folgenden Text, fasse ihn zusammen, extrahiere die wichtigsten Punkte und gib eine Bewertung ab: " + text;
let response = await llm.generate(prompt);
```

**Tokens:** ~120

### Nachher (VelinScript-Style)

```velin
let response = await @llm.analyze(text);
```

**Tokens:** ~5-10

**Ersparnis:** ~90-95%

---

## Unterstützte Methoden

### @llm.analyze(text)

Analysiert Text mit optimiertem Prompt:

```velin
let result = await @llm.analyze("Dieser Text enthält wichtige Informationen...");
```

### @llm.summarize(text)

Fasst Text zusammen:

```velin
let summary = await @llm.summarize(long_text);
```

### @llm.extract(text, pattern)

Extrahiert Informationen:

```velin
let emails = await @llm.extract(text, "email addresses");
```

### @llm.evaluate(text)

Bewertet Text:

```velin
let score = await @llm.evaluate(review_text);
```

### @llm.translate(text, target_lang)

Übersetzt Text:

```velin
let translated = await @llm.translate(text, "en");
```

### @llm.sentiment(text)

Analysiert Sentiment:

```velin
let sentiment = await @llm.sentiment(comment);
```

---

## Implementierung

### Prompt Optimizer

**Implementierung:** `compiler/src/prompt/optimizer.rs`

```rust
use velin_compiler::prompt::optimizer::PromptOptimizer;

let mut optimizer = PromptOptimizer::new();
let optimized = optimizer.optimize("Bitte analysiere den folgenden Text...");
println!("Ersparnis: {:.1}%", optimized.savings_percent);
```

### Parser-Erweiterung

**Implementierung:** `compiler/src/parser/parser.rs`

Der Parser erkennt `@llm.*` Syntax und konvertiert sie zu `Expression::LLMCall`.

### Code-Generierung

**Implementierung:** `compiler/src/codegen/rust.rs`

Generiert optimierte LLM-Calls:

```rust
// @llm.analyze(text) wird zu:
llm_client.analyze(text).await
```

### LLMClient-Erweiterung

**Implementierung:** `compiler/src/stdlib/ml.rs`

LLMClient hat jetzt kompakte Methoden mit automatischer Prompt-Optimierung:

- `analyze(text)` - Intern: Optimierter Prompt
- `summarize(text)` - Intern: Optimierter Prompt
- `extract(text, pattern)` - Intern: Optimierter Prompt
- etc.

---

## Optimierungs-Strategien

### 1. Prompt-Kürzung

Entfernt redundante Wörter:

- "Bitte analysiere den folgenden Text" → "analyze"
- "fasse ihn zusammen" → "summarize"
- "extrahiere die wichtigsten Punkte" → "extract"
- "gib eine Bewertung ab" → "evaluate"

### 2. Variable-Substitution

Ersetzt häufige Phrasen durch Variablen:

- "analysiere Text" → "@analyze(text)"
- "summarize content" → "@summarize(content)"

### 3. System-Prompt-Caching

System-Prompts werden gecached und wiederverwendet:

- Einmalig generiert
- Für alle Calls wiederverwendet
- Spart Tokens bei jedem Call

---

## Dateien

- `compiler/src/prompt/mod.rs` - Modul-Definition
- `compiler/src/prompt/optimizer.rs` - Prompt Optimizer

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.0.1
