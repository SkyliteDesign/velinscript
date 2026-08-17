# Intermediate Representation (IR) - VelinScript 3.0.1

**Version:** 3.0.1 / 3.1.0  
**Status:** ✅ Vollständig implementiert  
**Datum:** 2026-01-30

---

## Übersicht

Die Intermediate Representation (IR) ist eine echte Intermediate Representation zwischen AST und Code-Generierung. Sie verwendet SSA (Single Static Assignment) Format für optimierte Code-Generierung.

## Architektur

```
Source Code → AST → IR → Optimized IR → Target Code
```

## Vorteile

- **Mehrfache Optimierungen** - Optimierungen können mehrfach auf IR-Ebene durchgeführt werden
- **Unabhängigkeit von Ziel-Sprache** - IR ist sprachunabhängig
- **Bessere Code-Qualität** - Optimierungen führen zu besserem generiertem Code
- **Einfacheres Debugging** - IR kann visualisiert und analysiert werden

---

## IR-Struktur

### IRModule

Haupt-IR-Struktur die ein vollständiges VelinScript-Modul repräsentiert:

```rust
pub struct IRModule {
    pub name: String,
    pub functions: Vec<IRFunction>,
    pub structs: Vec<IRStruct>,
    pub enums: Vec<IREnum>,
    pub constants: Vec<IRConstant>,
}
```

### IRFunction

Repräsentiert eine Funktion in IR-Format:

```rust
pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRParameter>,
    pub return_type: IRType,
    pub body: IRBlock,
    pub attributes: Vec<IRAttribute>,
    pub is_async: bool,
    pub visibility: Visibility,
}
```

### IRBlock (SSA)

Ein Block enthält Instructions und Control Flow Information:

```rust
pub struct IRBlock {
    pub id: BlockId,
    pub instructions: Vec<IRInstruction>,
    pub predecessors: Vec<BlockId>,
    pub successors: Vec<BlockId>,
}
```

### IRInstruction

SSA-Instructions (jede Instruction produziert höchstens einen Wert):

- **Arithmetik**: `Add`, `Subtract`, `Multiply`, `Divide`, `Modulo`
- **Vergleich**: `Eq`, `NotEq`, `Lt`, `Gt`, `LtEq`, `GtEq`
- **Logik**: `And`, `Or`, `Not`
- **Speicher**: `Load`, `Store`, `Alloca`
- **Kontrollfluss**: `Branch`, `Jump`, `Return`
- **Funktions-Aufrufe**: `Call`, `CallAsync`
- **Struct/Enum**: `StructAccess`, `StructConstruct`, `EnumConstruct`
- **Pattern Matching**: `Match`
- **Collections**: `ListGet`, `ListSet`, `MapGet`, `MapSet`
- **SSA**: `Phi` (für Control Flow)

---

## IR Builder

**Implementierung:** `compiler/src/ir/builder.rs`

Konvertiert vollständig AST zu IR:

- Alle Statements werden konvertiert
- Alle Expressions werden konvertiert
- SSA-Format wird eingehalten
- Control Flow Graph (CFG) wird erstellt

### Beispiel

```rust
use velin_compiler::ir::builder::IRBuilder;

let mut builder = IRBuilder::new();
let ir_module = builder.build_module(&ast_program);
```

---

## IR Optimizer

**Implementierung:** `compiler/src/ir/optimizer.rs`

Führt verschiedene Optimierungen durch:

### Dead Code Elimination

Entfernt ungenutzte Variablen und Instructions.

### Constant Folding

Faltet konstante Ausdrücke zur Compile-Zeit:

```velin
// Vorher:
let x = 2 + 3;

// Nachher (IR):
let x = 5;
```

### Function Inlining

Inlined kleine Funktionen direkt in den Aufrufer.

### Loop Optimizations

- Loop Unrolling
- Loop Invariant Code Motion
- Induction Variable Elimination

### Beispiel

```rust
use velin_compiler::ir::optimizer::IROptimizer;

let optimizer = IROptimizer::new();
optimizer.optimize(&mut ir_module);
```

---

## IR Validator

**Implementierung:** `compiler/src/ir/validator.rs`

Prüft IR auf Korrektheit:

- SSA-Format wird eingehalten
- Alle Block-Referenzen sind gültig
- Typen sind konsistent

### Beispiel

```rust
use velin_compiler::ir::validator::IRValidator;

let mut validator = IRValidator::new();
match validator.validate(&ir_module) {
    Ok(_) => println!("IR ist gültig"),
    Err(errors) => println!("Fehler: {:?}", errors),
}
```

---

## IR Code Generator

**Implementierung:** `compiler/src/codegen/ir_codegen.rs`

Generiert Code aus IR:

- **Rust** - ✅ Vollständig implementiert
- **PHP** - ✅ Vollständig implementiert
- **Python** - ✅ Vollständig implementiert
- **Andere Targets** - Nutzen Rust als Fallback (direkte Codegen-Pipeline)

### Beispiel

```rust
use velin_compiler::codegen::ir_codegen::IRCodeGenerator;
use velin_compiler::codegen::traits::TargetLanguage;

let generator = IRCodeGenerator::new(TargetLanguage::Rust);
let code = generator.generate(&ir_module)?;
```

---

## Integration

Die IR ist vollständig in die Compiler-Pipeline integriert:

1. **AST → IR** - IRBuilder konvertiert AST zu IR
2. **IR-Optimierungen** - IROptimizer optimiert IR
3. **IR-Validierung** - IRValidator prüft IR auf Korrektheit
4. **IR → Target Code** - IRCodeGenerator generiert Target-Code

**Aktivierung:**
- Standardmäßig aktiviert in `CodegenPass`
- Kann mit `CodegenPass::with_ir(false)` deaktiviert werden (Legacy-Modus)

---

## Dateien

- `compiler/src/ir/mod.rs` - Modul-Definition
- `compiler/src/ir/ir.rs` - IR-Strukturen
- `compiler/src/ir/builder.rs` - AST → IR Konvertierung
- `compiler/src/ir/optimizer.rs` - IR-Optimierungen
- `compiler/src/ir/validator.rs` - IR-Validierung
- `compiler/src/codegen/ir_codegen.rs` - IR → Target Code

---

**Letzte Aktualisierung:** 2026-01-30  
**Version:** 3.0.1
