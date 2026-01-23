# KI-Compiler-Passes (Version 3.0.1 / 3.1.0)

**Version:** 3.0.1 / 3.1.0  
**Status:** ✅ Vollständig implementiert

---

## Übersicht

VelinScript 3.0 führt revolutionäre KI-basierte Compiler-Passes ein, die Code automatisch analysieren, optimieren und generieren. Diese Passes nutzen Large Language Models (LLMs) als Co-Prozessor für intelligente Code-Verarbeitung.

## KI-Compiler-Passes

### 1. AISemanticPass

**Datei:** `compiler/src/passes/ai_semantic.rs`

**Funktionalität:**
- Analysiert Code-Semantik mit Hilfe von LLM
- Erkennt Kontext (API, Service, Library, Application)
- Identifiziert Abhängigkeiten automatisch
- Analysiert Sicherheitsanforderungen
- Speichert Metadaten im CompilationContext

**Verwendung:**
```rust
let mut config = CompilerConfig::default();
config.enable_ai_semantic = true;
config.ai_provider = Some("openai".to_string());
config.ai_api_key = Some("your-api-key".to_string());

let pass = AISemanticPass::new(&config)?;
```

**Metadaten:**
Die Analyse-Ergebnisse werden in `CompilationContext.semantic_metadata` gespeichert:
- `context_type`: Art des Codes (api, service, library, application)
- `dependencies`: Liste benötigter Abhängigkeiten
- `security_requirements`: Sicherheitsanforderungen
- `missing_components`: Fehlende Komponenten

### 2. AIBugDetectionPass

**Datei:** `compiler/src/passes/ai_bug_detection.rs`

**Funktionalität:**
- Pattern-basierte Bug-Erkennung
- KI-basierte semantische Bug-Erkennung
- Logik-Fehler erkennen
- Sicherheitslücken finden
- Auto-Fix für einfache Bugs

**Bug-Patterns:**
- Fehlende Error Handling
- Potenzielle Null-Pointer
- Fehlende Auth bei sensiblen Operationen
- Und mehr...

**Auto-Fix:**
Einfache Bugs werden automatisch behoben, z.B.:
- Fehlende `@Auth` Decorators werden vorgeschlagen
- Sicherheitslücken werden identifiziert und gemeldet

### 3. AICodeGenerationPass

**Datei:** `compiler/src/passes/ai_codegen.rs`

**Funktionalität:**
- Identifiziert fehlende Komponenten
- Generiert fehlende Funktionen mit KI
- Generiert fehlende Datenstrukturen
- Generiert fehlende Tests
- Validiert generierten Code
- Fügt Code zum AST hinzu

**Code-Generierung:**
Der Pass nutzt LLMClient, um fehlende Komponenten zu generieren:
1. Identifiziert fehlende Komponenten aus Semantic Metadata
2. Generiert Code mit KI
3. Validiert generierten Code
4. Fügt Code zum AST hinzu

### 4. AIOptimizationPass

**Datei:** `compiler/src/passes/ai_optimization.rs`

**Funktionalität:**
- Analysiert Optimierungs-Potenzial mit KI
- Nutzt Profiling-Daten (falls vorhanden)
- Wendet Performance-Optimierungen an
- Optimiert Memory-Usage
- Verbessert Code-Readability

**Optimierungs-Typen:**
- **Performance**: Parallelisierung, Caching
- **Memory**: Reduzierung von Allokationen
- **Security**: Input-Validierung
- **Readability**: Refactoring-Vorschläge

### 5. AICodeReviewPass (Neu in 3.0.1)

**Datei:** `compiler/src/passes/ai_code_review.rs`

**Funktionalität:**
- Reviewt AI-generierten Code auf Sicherheit und Qualität
- Syntax-Validierung
- Type-Checking
- Security-Checks (fehlende Auth, Input-Validation)
- Complexity-Check (zyklomatische Komplexität)
- Import-Check (nur erlaubte Module)
- Pattern-Check (gefährliche Patterns erkennen)

**Sicherheitsprüfungen:**
- Verbotene Imports erkennen
- Unsichere Patterns erkennen
- Fehlende Authentifizierung erkennen
- Input-Validierung prüfen

**Integration:**
Wird automatisch vor `AIOptimizationPass` ausgeführt, wenn `enable_ai_code_review` aktiviert ist.

### 6. AISandboxPass (Neu in 3.0.1)

**Datei:** `compiler/src/passes/ai_sandbox.rs`

**Funktionalität:**
- Führt AI-generierten Code in isolierter Sandbox aus
- Validiert Code vor Execution
- Prüft auf verbotene Funktionen (File-Operationen, System-Operationen, Network-Operationen)
- Prüft auf gefährliche Operationen
- **Vollständige AST-Durchsuchung** für Funktions-Aufrufe

**Sicherheitsfeatures:**
- Erlaubte Funktionen: nur sichere Operationen (add, subtract, multiply, etc.)
- Verbotene Funktionen: read_file, write_file, execute, system, eval, etc.
- Rekursive AST-Durchsuchung aller Statements und Expressions

**Integration:**
Wird automatisch vor `AIOptimizationPass` ausgeführt, wenn `enable_ai_sandbox` aktiviert ist.

### 7. PromptSanitizer (Neu in 3.0.1)

**Datei:** `compiler/src/prompt/sanitizer.rs`

**Funktionalität:**
- Sanitized alle LLM-Prompts vor dem Senden
- Entfernt gefährliche Patterns (Ignore instructions, Role manipulation, Command injection)
- Escaped spezielle Zeichen
- Validiert Prompts auf Sicherheit

**Geschützte Patterns:**
- "Ignore previous instructions"
- "You are now" / "Act as" (Role manipulation)
- Command injection (`execute`, `system`, `eval`, etc.)
- Code block injection (```python, ```bash, etc.)

**Integration:**
Wird automatisch in allen AI-Passes verwendet:
- `AISemanticPass`
- `AIBugDetectionPass`
- `AICodeGenerationPass`
- `AIOptimizationPass`

## Integration

Die KI-Passes werden in die Compiler-Pipeline integriert:

```rust
// Reihenfolge der Passes:
1. AutoFixPass
2. ParserPass
3. AISemanticPass (optional) - nutzt PromptSanitizer
4. AIBugDetectionPass (optional) - nutzt PromptSanitizer
5. TypeCheckPass
6. ParallelizationAnalyzer
7. AICodeGenerationPass (optional) - nutzt PromptSanitizer
8. AICodeReviewPass (optional) - Reviewt AI-generierten Code auf Sicherheit und Qualität
9. AISandboxPass (optional) - Validiert AI-generierten Code in isolierter Sandbox
10. AIOptimizationPass (optional) - nutzt PromptSanitizer
11. CodegenPass
```

## Konfiguration

**CLI:**
```bash
velin compile --input app.velin \
  --ai-semantic \
  --ai-bug-detection \
  --ai-codegen \
  --ai-optimization \
  --ai-provider openai \
  --ai-api-key $OPENAI_API_KEY
```

**Config:**
```rust
let mut config = CompilerConfig::default();
config.enable_ai_semantic = true;
config.enable_ai_bug_detection = true;
config.enable_ai_codegen = true;
// AICodeReviewer und AISandbox werden intern von AICodeGenerationPass verwendet
config.enable_ai_optimization = true;
config.ai_provider = Some("openai".to_string());
config.ai_api_key = Some("your-api-key".to_string());
```

## Fallback-Verhalten

Wenn kein LLMClient konfiguriert ist, nutzen die Passes heuristische Analysen:
- **AISemanticPass**: Heuristische Kontext-Erkennung
- **AIBugDetectionPass**: Pattern-basierte Erkennung
- **AICodeGenerationPass**: BoilerplateGenerator
- **AIOptimizationPass**: Heuristische Optimierungen

## Best Practices

1. **API Keys sicher speichern**: Nutze Umgebungsvariablen
2. **KI-Passes optional aktivieren**: Nur wenn nötig
3. **Ergebnisse prüfen**: KI-generierter Code sollte immer reviewt werden
4. **Profiling nutzen**: Für bessere Optimierungen

---

## Siehe auch

- [Compiler Architecture](./compiler-architecture.md)
- [System Generator](./system-generation.md)
- [Parallelization](./parallelization.md)
