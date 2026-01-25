# Pass-Dokumentation Mapping

**Version:** 3.1.0  
**Status:** ✅ Vollständige Übersicht

---

## Übersicht

Diese Übersicht zeigt, welche Dokumentation für welchen Pass zuständig ist.

---

## Core-Passes (Immer aktiv)

### 1. AutoFixPass

**Datei:** `compiler/src/passes/autofix.rs`  
**Hauptdokumentation:**
- **[Auto Repair Tool](../tools/auto-repair.md)** - Vollständige Tool-Dokumentation ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#1-autofixpass) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Vollständig dokumentiert

---

### 2. ParserPass

**Datei:** `compiler/src/passes/parser.rs`  
**Hauptdokumentation:**
- **[ParserPass](./parser-pass.md)** - Vollständige Pass-Dokumentation ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#2-parserpass) - Kurze Beschreibung im Pass-Verlauf
- [Module Resolution](./module-resolution.md) - Detaillierte Modul-Auflösung

**Status:** ✅ Vollständig dokumentiert

---

### 3. DesugaringPass

**Datei:** `compiler/src/passes/desugar.rs`  
**Hauptdokumentation:**
- **[DesugaringPass](./desugaring-pass.md)** - Vollständige Pass-Dokumentation ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#3-desugaringpass) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Vollständig dokumentiert

---

### 4. CodeOrderingPass

**Datei:** `compiler/src/passes/code_order.rs`  
**Hauptdokumentation:**
- **[CodeOrderingPass](./code-ordering-pass.md)** - Vollständige Pass-Dokumentation ✅

**Zusätzliche Dokumentation:**
- [Code Ordering](./code-ordering.md) - Feature-Dokumentation
- [Pass-Verlauf](./pass-verlauf.md#4-codeorderingpass) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Vollständig dokumentiert

---

### 5. TypeCheckPass

**Datei:** `compiler/src/passes/type_check.rs`  
**Hauptdokumentation:**
- **[Type Inference](./type-inference.md)** - Type-Inference System ✅
- **[Borrow Checker](./borrow-checker.md)** - Ownership & Borrowing System ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#7-typecheckpass) - Kurze Beschreibung im Pass-Verlauf

**Status:** ⚠️ Feature-Dokumentation vorhanden, spezifische Pass-Doku fehlt

**Hinweis:** TypeCheckPass nutzt Type-Inference und Borrow-Checker, die separat dokumentiert sind.

---

## Optimizer-Passes

### 6. ParallelizationAnalyzer

**Datei:** `compiler/src/optimizer/parallelization.rs`  
**Hauptdokumentation:**
- **[Parallelization](./parallelization.md)** - Vollständige Dokumentation ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#8-parallelizationanalyzer) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Vollständig dokumentiert

---

## KI-Passes (Optional)

### 7. AISemanticPass

**Datei:** `compiler/src/passes/ai_semantic.rs`  
**Hauptdokumentation:**
- **[AI Compiler Passes](./ai-compiler-passes.md#1-aisemanticpass)** - Vollständige Beschreibung ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#5-aisemanticpass-optional) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Dokumentiert (in AI-Passes-Doku enthalten)

---

### 8. AIBugDetectionPass

**Datei:** `compiler/src/passes/ai_bug_detection.rs`  
**Hauptdokumentation:**
- **[AI Compiler Passes](./ai-compiler-passes.md#2-aibugdetectionpass)** - Vollständige Beschreibung ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#6-aibugdetectionpass-optional) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Dokumentiert (in AI-Passes-Doku enthalten)

---

### 9. AICodeGenerationPass

**Datei:** `compiler/src/passes/ai_codegen.rs`  
**Hauptdokumentation:**
- **[AI Compiler Passes](./ai-compiler-passes.md#3-aicodegenerationpass)** - Vollständige Beschreibung ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#9-aicodegenerationpass-optional) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Dokumentiert (in AI-Passes-Doku enthalten)

---

### 10. AICodeReviewPass

**Datei:** `compiler/src/passes/ai_code_review.rs`  
**Hauptdokumentation:**
- **[AI Compiler Passes](./ai-compiler-passes.md#5-aicodereviewpass-neu-in-301)** - Vollständige Beschreibung ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#10-aicodereviewpass-optional) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Dokumentiert (in AI-Passes-Doku enthalten)

---

### 11. AISandboxPass

**Datei:** `compiler/src/passes/ai_sandbox.rs`  
**Hauptdokumentation:**
- **[AI Compiler Passes](./ai-compiler-passes.md#6-aisandboxpass-neu-in-301)** - Vollständige Beschreibung ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#11-aisandboxpass-optional) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Dokumentiert (in AI-Passes-Doku enthalten)

---

### 12. AIOptimizationPass

**Datei:** `compiler/src/passes/ai_optimization.rs`  
**Hauptdokumentation:**
- **[AI Compiler Passes](./ai-compiler-passes.md#4-aioptimizationpass)** - Vollständige Beschreibung ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#12-aioptimizationpass-optional) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Dokumentiert (in AI-Passes-Doku enthalten)

---

## Code-Generierung

### 13. CodegenPass

**Datei:** `compiler/src/passes/codegen.rs`  
**Hauptdokumentation:**
- **[Code Generation](./code-generation.md)** - Vollständige Dokumentation ✅
- **[Multi-Target Compilation](./multi-target-compilation.md)** - Multi-Target Support ✅
- **[IR Representation](./ir-representation.md)** - Intermediate Representation ✅

**Zusätzliche Dokumentation:**
- [Pass-Verlauf](./pass-verlauf.md#13-codegenpass) - Kurze Beschreibung im Pass-Verlauf

**Status:** ✅ Vollständig dokumentiert

---

## Zusammenfassung

### Dokumentationsstruktur

| Pass | Hauptdokumentation | Zusätzliche Dokumentation |
|------|-------------------|-------------------------|
| **1. AutoFixPass** | `tools/auto-repair.md` | `pass-verlauf.md` |
| **2. ParserPass** | `parser-pass.md` | `pass-verlauf.md`, `module-resolution.md` |
| **3. DesugaringPass** | `desugaring-pass.md` | `pass-verlauf.md` |
| **4. CodeOrderingPass** | `code-ordering-pass.md` | `code-ordering.md`, `pass-verlauf.md` |
| **5. TypeCheckPass** | `type-inference.md`, `borrow-checker.md` | `pass-verlauf.md` |
| **6. ParallelizationAnalyzer** | `parallelization.md` | `pass-verlauf.md` |
| **7. AISemanticPass** | `ai-compiler-passes.md` | `pass-verlauf.md` |
| **8. AIBugDetectionPass** | `ai-compiler-passes.md` | `pass-verlauf.md` |
| **9. AICodeGenerationPass** | `ai-compiler-passes.md` | `pass-verlauf.md` |
| **10. AICodeReviewPass** | `ai-compiler-passes.md` | `pass-verlauf.md` |
| **11. AISandboxPass** | `ai-compiler-passes.md` | `pass-verlauf.md` |
| **12. AIOptimizationPass** | `ai-compiler-passes.md` | `pass-verlauf.md` |
| **13. CodegenPass** | `code-generation.md` | `multi-target-compilation.md`, `ir-representation.md`, `pass-verlauf.md` |

---

## Dokumentationsdateien

### Spezifische Pass-Dokumentationen

1. **`parser-pass.md`** → ParserPass
2. **`desugaring-pass.md`** → DesugaringPass
3. **`code-ordering-pass.md`** → CodeOrderingPass

### Feature-Dokumentationen

1. **`type-inference.md`** → TypeCheckPass (Type-Inference)
2. **`borrow-checker.md`** → TypeCheckPass (Borrow-Checker)
3. **`parallelization.md`** → ParallelizationAnalyzer
4. **`code-generation.md`** → CodegenPass
5. **`multi-target-compilation.md`** → CodegenPass (Multi-Target)
6. **`ir-representation.md`** → CodegenPass (IR)
7. **`code-ordering.md`** → CodeOrderingPass (Feature)

### Gruppierte Dokumentationen

1. **`ai-compiler-passes.md`** → Alle KI-Passes (7-12)
   - AISemanticPass
   - AIBugDetectionPass
   - AICodeGenerationPass
   - AICodeReviewPass
   - AISandboxPass
   - AIOptimizationPass

### Tool-Dokumentationen

1. **`tools/auto-repair.md`** → AutoFixPass

### Übersichts-Dokumentationen

1. **`pass-verlauf.md`** → Alle Passes (Übersicht)
2. **`passes-uebersicht.md`** → Alle Passes (Status-Übersicht)
3. **`compiler-architecture.md`** → Compiler-Architektur (inkl. Passes)
4. **`error-handling.md`** ✅ (Neu in 3.1.0) → Fehlerbehandlung & Lösungsvorschläge für alle Passes

---

## Empfehlungen

### Für Entwickler

- **Neue Passes verstehen:** Starte mit `pass-verlauf.md` für Übersicht
- **Spezifische Pass-Details:** Nutze die spezifischen Pass-Dokumentationen
- **Feature-Details:** Nutze Feature-Dokumentationen für tiefere Einblicke

### Für Dokumentations-Erweiterungen

- **Neue Core-Passes:** Erstelle spezifische `*-pass.md` Datei
- **Neue KI-Passes:** Füge zu `ai-compiler-passes.md` hinzu
- **Neue Features:** Erstelle separate Feature-Dokumentation

---

## Fehlerbehandlung ✅ (Neu in 3.1.0)

Alle Passes verwenden jetzt ein konsistentes Fehlerbehandlungssystem:

- **Zentrale API:** `context.add_error()`, `context.add_warning()`, `context.add_info()`
- **Intelligente Lösungsvorschläge:** Alle Fehlertypen erhalten kontextbezogene Vorschläge
- **Separate Warnings:** Warnings werden nicht mehr als Errors behandelt
- **Fehlerstatistiken:** Detaillierte Statistiken über alle Fehlertypen
- **Export-Funktionen:** JSON- und HTML-Export verfügbar

**Siehe:** [Fehlerbehandlung & Lösungsvorschläge](./error-handling.md) ✅ (Neu in 3.1.0)

---

## Siehe auch

- [Pass-Verlauf](./pass-verlauf.md) - Detaillierte Erklärung aller Passes
- [Passes-Übersicht](./passes-uebersicht.md) - Vollständige Übersicht aller Passes
- [Compiler Architecture](./compiler-architecture.md) - Compiler-Architektur
- [Fehlerbehandlung & Lösungsvorschläge](./error-handling.md) ✅ (Neu in 3.1.0) - Umfassendes Fehlerbehandlungssystem

---

**Letzte Aktualisierung:** 2026-02-02  
**Version:** 3.1.0
