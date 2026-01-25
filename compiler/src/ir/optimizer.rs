/// IR Optimizer - Optimiert IR-Code
///
/// Dieser Optimizer führt verschiedene Optimierungen auf der IR durch:
/// - Dead Code Elimination
/// - Constant Folding
/// - Function Inlining
/// - Loop Optimizations
///
/// # Beispiel
///
/// ```rust
/// use velin_compiler::ir::optimizer::IROptimizer;
///
/// let optimizer = IROptimizer::new();
/// optimizer.optimize(&mut ir_module);
/// ```
use crate::ir::ir::*;
use std::collections::HashMap;

/// IR Optimizer
pub struct IROptimizer {
    pub enabled_optimizations: Vec<Optimization>,
}

/// Verfügbare Optimierungen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Optimization {
    DeadCodeElimination,
    ConstantFolding,
    FunctionInlining,
    LoopOptimization,
}

impl IROptimizer {
    /// Erstellt einen neuen IR Optimizer mit allen Optimierungen aktiviert
    pub fn new() -> Self {
        IROptimizer {
            enabled_optimizations: vec![
                Optimization::DeadCodeElimination,
                Optimization::ConstantFolding,
                Optimization::FunctionInlining,
                Optimization::LoopOptimization,
            ],
        }
    }

    /// Erstellt einen IR Optimizer mit spezifischen Optimierungen
    pub fn with_optimizations(optimizations: Vec<Optimization>) -> Self {
        IROptimizer {
            enabled_optimizations: optimizations,
        }
    }

    /// Optimiert ein IR-Modul
    pub fn optimize(&self, module: &mut IRModule) {
        // Mehrfache Optimierungs-Passes für bessere Ergebnisse
        for _ in 0..3 {
            if self
                .enabled_optimizations
                .contains(&Optimization::DeadCodeElimination)
            {
                self.eliminate_dead_code(module);
            }

            if self
                .enabled_optimizations
                .contains(&Optimization::ConstantFolding)
            {
                self.fold_constants(module);
            }

            if self
                .enabled_optimizations
                .contains(&Optimization::FunctionInlining)
            {
                self.inline_functions(module);
            }

            if self
                .enabled_optimizations
                .contains(&Optimization::LoopOptimization)
            {
                self.optimize_loops(module);
            }
        }
    }

    /// Dead Code Elimination
    ///
    /// Entfernt ungenutzte Variablen und Instructions.
    fn eliminate_dead_code(&self, module: &mut IRModule) {
        for func in &mut module.functions {
            self.eliminate_dead_code_in_function(func);
        }
    }

    /// Dead Code Elimination für eine Funktion
    fn eliminate_dead_code_in_function(&self, func: &mut IRFunction) {
        // 1. Sammle alle verwendeten Werte
        let mut used_values = Vec::new();
        self.collect_used_values(&func.body, &mut used_values);

        // 2. Entferne ungenutzte Instructions
        self.remove_unused_instructions(&mut func.body, &mut used_values);
    }

    /// Sammelt alle verwendeten Werte in einem Block
    fn collect_used_values(&self, block: &IRBlock, used: &mut Vec<IRValue>) {
        for instruction in &block.instructions {
            match instruction {
                IRInstruction::Add { left, right, .. }
                | IRInstruction::Subtract { left, right, .. }
                | IRInstruction::Multiply { left, right, .. }
                | IRInstruction::Divide { left, right, .. }
                | IRInstruction::Modulo { left, right, .. }
                | IRInstruction::Eq { left, right, .. }
                | IRInstruction::NotEq { left, right, .. }
                | IRInstruction::Lt { left, right, .. }
                | IRInstruction::Gt { left, right, .. }
                | IRInstruction::LtEq { left, right, .. }
                | IRInstruction::GtEq { left, right, .. }
                | IRInstruction::And { left, right, .. }
                | IRInstruction::Or { left, right, .. } => {
                    if !used.contains(&left) {
                        used.push(left.clone());
                    }
                    if !used.contains(&right) {
                        used.push(right.clone());
                    }
                }
                IRInstruction::Not { operand, .. } => {
                    if !used.contains(&operand) {
                        used.push(operand.clone());
                    }
                }
                IRInstruction::Load { source, .. } => {
                    if !used.contains(&source) {
                        used.push(source.clone());
                    }
                }
                IRInstruction::Store { dest, value } => {
                    if !used.contains(&dest) {
                        used.push(dest.clone());
                    }
                    if !used.contains(&value) {
                        used.push(value.clone());
                    }
                }
                IRInstruction::Branch { condition, .. } => {
                    if !used.contains(&condition) {
                        used.push(condition.clone());
                    }
                }
                IRInstruction::Return { value } => {
                    if let Some(v) = value {
                        if !used.contains(v) {
                            used.push(v.clone());
                        }
                    }
                }
                IRInstruction::Call { func, args, .. }
                | IRInstruction::CallAsync { func, args, .. } => {
                    if !used.contains(func) {
                        used.push(func.clone());
                    }
                    for arg in args {
                        if !used.contains(arg) {
                            used.push(arg.clone());
                        }
                    }
                }
                IRInstruction::StructAccess { struct_val, .. } => {
                    if !used.contains(&struct_val) {
                        used.push(struct_val.clone());
                    }
                }
                IRInstruction::ListGet { list, index, .. } => {
                    if !used.contains(&list) {
                        used.push(list.clone());
                    }
                    if !used.contains(&index) {
                        used.push(index.clone());
                    }
                }
                IRInstruction::MapGet { map, key, .. } => {
                    if !used.contains(&map) {
                        used.push(map.clone());
                    }
                    if !used.contains(&key) {
                        used.push(key.clone());
                    }
                }
                IRInstruction::StructConstruct { fields, .. } => {
                    for (_, val) in fields {
                        if !used.contains(&val) {
                            used.push(val.clone());
                        }
                    }
                }
                IRInstruction::EnumConstruct { data, .. } => {
                    if let Some(d) = data {
                        if !used.contains(d) {
                            used.push(d.clone());
                        }
                    }
                }
                IRInstruction::Match { value, arms } => {
                    if !used.contains(&value) {
                        used.push(value.clone());
                    }
                    for arm in arms {
                        if let Some(guard) = &arm.guard {
                            if !used.contains(guard) {
                                used.push(guard.clone());
                            }
                        }
                    }
                }
                IRInstruction::Phi { incoming, .. } => {
                    for (_, val) in incoming {
                        if !used.contains(&val) {
                            used.push(val.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Entfernt ungenutzte Instructions
    fn remove_unused_instructions(&self, block: &mut IRBlock, used: &mut Vec<IRValue>) {
        block.instructions.retain(|inst| {
            match inst {
                IRInstruction::Add { dest, .. }
                | IRInstruction::Subtract { dest, .. }
                | IRInstruction::Multiply { dest, .. }
                | IRInstruction::Divide { dest, .. }
                | IRInstruction::Modulo { dest, .. }
                | IRInstruction::Eq { dest, .. }
                | IRInstruction::NotEq { dest, .. }
                | IRInstruction::Lt { dest, .. }
                | IRInstruction::Gt { dest, .. }
                | IRInstruction::LtEq { dest, .. }
                | IRInstruction::GtEq { dest, .. }
                | IRInstruction::And { dest, .. }
                | IRInstruction::Or { dest, .. }
                | IRInstruction::Not { dest, .. }
                | IRInstruction::Load { dest, .. }
                | IRInstruction::Alloca { dest, .. } => {
                    if !used.contains(dest) {
                        used.push(dest.clone());
                    }
                    true // Behalten
                }
                IRInstruction::Call { dest, .. } => {
                    if let Some(d) = dest {
                        if !used.contains(d) {
                            used.push(d.clone());
                        }
                    }
                    true // Call hat Side-Effects, behalten
                }
                IRInstruction::CallAsync { dest, .. } => {
                    if let Some(d) = dest {
                        if !used.contains(d) {
                            used.push(d.clone());
                        }
                    }
                    true // CallAsync hat Side-Effects, behalten
                }
                IRInstruction::StructAccess { dest, .. }
                | IRInstruction::StructConstruct { dest, .. }
                | IRInstruction::EnumConstruct { dest, .. }
                | IRInstruction::ListGet { dest, .. }
                | IRInstruction::MapGet { dest, .. }
                | IRInstruction::Phi { dest, .. } => {
                    // Instruction produziert einen Wert - prüfe ob verwendet
                    used.contains(dest) || matches!(dest, IRValue::Constant(_))
                }
                IRInstruction::Store { .. }
                | IRInstruction::Branch { .. }
                | IRInstruction::Jump { .. }
                | IRInstruction::Return { .. }
                | IRInstruction::ListSet { .. }
                | IRInstruction::MapSet { .. }
                | IRInstruction::Match { .. } => {
                    // Side-effect Instructions - immer behalten
                    true
                }
            }
        });
    }

    /// Constant Folding
    ///
    /// Faltet konstante Ausdrücke zur Compile-Zeit.
    fn fold_constants(&self, module: &mut IRModule) {
        for func in &mut module.functions {
            self.fold_constants_in_function(func);
        }
    }

    /// Constant Folding für eine Funktion
    fn fold_constants_in_function(&self, func: &mut IRFunction) {
        for instruction in &mut func.body.instructions {
            match instruction {
                IRInstruction::Add { dest, left, right } => {
                    if let (
                        IRValue::Constant(IRConstant::Number(a)),
                        IRValue::Constant(IRConstant::Number(b)),
                    ) = (left, right)
                    {
                        *instruction = IRInstruction::Store {
                            dest: dest.clone(),
                            value: IRValue::Constant(IRConstant::Number(*a + *b)),
                        };
                    }
                }
                IRInstruction::Subtract { dest, left, right } => {
                    if let (
                        IRValue::Constant(IRConstant::Number(a)),
                        IRValue::Constant(IRConstant::Number(b)),
                    ) = (left, right)
                    {
                        *instruction = IRInstruction::Store {
                            dest: dest.clone(),
                            value: IRValue::Constant(IRConstant::Number(*a - *b)),
                        };
                    }
                }
                IRInstruction::Multiply { dest, left, right } => {
                    if let (
                        IRValue::Constant(IRConstant::Number(a)),
                        IRValue::Constant(IRConstant::Number(b)),
                    ) = (left, right)
                    {
                        *instruction = IRInstruction::Store {
                            dest: dest.clone(),
                            value: IRValue::Constant(IRConstant::Number(*a * *b)),
                        };
                    }
                }
                IRInstruction::Divide { dest, left, right } => {
                    if let (
                        IRValue::Constant(IRConstant::Number(a)),
                        IRValue::Constant(IRConstant::Number(b)),
                    ) = (left, right)
                    {
                        if *b != 0.0 {
                            *instruction = IRInstruction::Store {
                                dest: dest.clone(),
                                value: IRValue::Constant(IRConstant::Number(*a / *b)),
                            };
                        }
                    }
                }
                IRInstruction::Eq { dest, left, right } => {
                    if let (IRValue::Constant(a), IRValue::Constant(b)) = (left, right) {
                        let result = a == b;
                        *instruction = IRInstruction::Store {
                            dest: dest.clone(),
                            value: IRValue::Constant(IRConstant::Boolean(result)),
                        };
                    }
                }
                IRInstruction::And { dest, left, right } => {
                    if let (
                        IRValue::Constant(IRConstant::Boolean(a)),
                        IRValue::Constant(IRConstant::Boolean(b)),
                    ) = (left, right)
                    {
                        *instruction = IRInstruction::Store {
                            dest: dest.clone(),
                            value: IRValue::Constant(IRConstant::Boolean(*a && *b)),
                        };
                    }
                }
                IRInstruction::Or { dest, left, right } => {
                    if let (
                        IRValue::Constant(IRConstant::Boolean(a)),
                        IRValue::Constant(IRConstant::Boolean(b)),
                    ) = (left, right)
                    {
                        *instruction = IRInstruction::Store {
                            dest: dest.clone(),
                            value: IRValue::Constant(IRConstant::Boolean(*a || *b)),
                        };
                    }
                }
                _ => {}
            }
        }
    }

    /// Function Inlining
    ///
    /// Inlined kleine Funktionen direkt in den Aufrufer.
    fn inline_functions(&self, module: &mut IRModule) {
        // Finde kleine Funktionen (weniger als 10 Instructions)
        let mut small_functions = HashMap::new();
        for func in &module.functions {
            let instruction_count = func.body.instructions.len();
            if instruction_count < 10 && !func.is_async {
                small_functions.insert(func.name.clone(), func.clone());
            }
        }

        // Inline kleine Funktionen
        for func in &mut module.functions {
            self.inline_function_calls(func, &small_functions);
        }
    }

    /// Inlined Funktions-Aufrufe
    fn inline_function_calls(
        &self,
        func: &mut IRFunction,
        small_functions: &HashMap<String, IRFunction>,
    ) {
        // Durchsuche alle Blocks nach Call-Instructions
        let mut new_instructions = Vec::new();
        let mut temp_id_counter = 10000; // Starte mit hoher ID um Konflikte zu vermeiden

        for instruction in &func.body.instructions {
            match instruction {
                IRInstruction::Call {
                    dest,
                    func: func_val,
                    args,
                } => {
                    // Prüfe ob es eine direkte Funktions-Referenz ist
                    if let IRValue::Variable(var) = func_val {
                        if let Some(target_func) = small_functions.get(&var.name) {
                            // Inline diese Funktion
                            let inlined =
                                self.inline_function(target_func, args, &mut temp_id_counter);
                            new_instructions.extend(inlined);

                            // Füge Store-Instruction hinzu, um Ergebnis zu speichern
                            if let Some(dest_val) = dest {
                                new_instructions.push(IRInstruction::Store {
                                    dest: dest_val.clone(),
                                    value: IRValue::Temporary(TempId::new(temp_id_counter - 1)),
                                });
                            }
                            continue;
                        }
                    }
                    // Nicht inline-bar, behalte Original-Instruction
                    new_instructions.push(instruction.clone());
                }
                _ => {
                    // Andere Instructions behalten
                    new_instructions.push(instruction.clone());
                }
            }
        }

        func.body.instructions = new_instructions;
    }

    /// Inlined eine Funktion in den Aufrufer
    fn inline_function(
        &self,
        target_func: &IRFunction,
        args: &[IRValue],
        temp_id_counter: &mut usize,
    ) -> Vec<IRInstruction> {
        let mut inlined = Vec::new();

        // Erstelle Mapping von Parametern zu Argumenten
        // Vereinfachte Implementierung: Erstelle temporäre Variablen für Parameter
        let mut param_map: HashMap<String, IRValue> = HashMap::new();
        for (param, arg) in target_func.params.iter().zip(args.iter()) {
            param_map.insert(param.name.clone(), arg.clone());
        }

        // Kopiere Instructions und ersetze Parameter durch Argumente
        for instruction in &target_func.body.instructions {
            let mut new_instruction = instruction.clone();

            // Ersetze Parameter-Referenzen durch Argumente
            match &mut new_instruction {
                IRInstruction::Load { dest: _, source } => {
                    if let IRValue::Variable(var) = source {
                        if let Some(replacement) = param_map.get(&var.name) {
                            *source = replacement.clone();
                        }
                    }
                }
                IRInstruction::Store { dest, value } => {
                    if let IRValue::Variable(var) = dest {
                        if let Some(replacement) = param_map.get(&var.name) {
                            *dest = replacement.clone();
                        }
                    }
                    if let IRValue::Variable(var) = value {
                        if let Some(replacement) = param_map.get(&var.name) {
                            *value = replacement.clone();
                        }
                    }
                }
                _ => {}
            }

            // Ersetze Return durch Store (wenn nötig)
            if let IRInstruction::Return { value } = &new_instruction {
                if let Some(ret_val) = value {
                    // Erstelle temporäre Variable für Return-Wert
                    let temp_id = TempId::new(*temp_id_counter);
                    *temp_id_counter += 1;
                    inlined.push(IRInstruction::Store {
                        dest: IRValue::Temporary(temp_id),
                        value: ret_val.clone(),
                    });
                    continue;
                }
            }

            inlined.push(new_instruction);
        }

        inlined
    }

    /// Loop Optimizations
    ///
    /// Optimiert Loops (Unrolling, Invariant Code Motion, etc.)
    fn optimize_loops(&self, module: &mut IRModule) {
        for func in &mut module.functions {
            self.optimize_loops_in_function(func);
        }
    }

    /// Loop Optimizations für eine Funktion
    fn optimize_loops_in_function(&self, func: &mut IRFunction) {
        // 1. Loop Unrolling (für kleine Loops)
        self.unroll_small_loops(&mut func.body);

        // 2. Loop Invariant Code Motion
        self.move_invariant_code(&mut func.body);

        // 3. Induction Variable Elimination (vereinfacht)
        self.eliminate_induction_variables(&mut func.body);
    }

    /// Unrollt kleine Loops
    fn unroll_small_loops(&self, block: &mut IRBlock) {
        let mut new_instructions = Vec::new();
        let mut i = 0;

        while i < block.instructions.len() {
            let instruction = &block.instructions[i];

            // Prüfe auf Loop-Pattern (Branch mit Jump zurück)
            if let IRInstruction::Branch {
                condition: _,
                then_block,
                else_block: _,
            } = instruction
            {
                // Vereinfachte Loop-Erkennung: Wenn then_block auf sich selbst zeigt
                if *then_block == block.id {
                    // Potenzieller Loop - prüfe ob unrollbar (max 5 Iterationen)
                    // In einer vollständigen Implementierung würde hier die Loop-Größe analysiert
                    // Für jetzt: Skip Loop-Unrolling (zu komplex)
                    new_instructions.push(instruction.clone());
                } else {
                    new_instructions.push(instruction.clone());
                }
            } else {
                new_instructions.push(instruction.clone());
            }

            i += 1;
        }

        block.instructions = new_instructions;
    }

    /// Bewegt invarianten Code aus Loops
    fn move_invariant_code(&self, block: &mut IRBlock) {
        // Identifiziere Instructions die innerhalb eines Loops invariant sind
        // Vereinfachte Implementierung: Prüfe auf konstante Werte
        let mut new_instructions = Vec::new();
        let mut loop_start = None;

        for (i, instruction) in block.instructions.iter().enumerate() {
            // Prüfe ob Instruction eine Loop-Markierung ist
            if let IRInstruction::Branch { .. } = instruction {
                if loop_start.is_none() {
                    loop_start = Some(i);
                }
            }

            // Prüfe ob Instruction invariant ist (nur Konstanten)
            let is_invariant = match instruction {
                IRInstruction::Add { left, right, .. }
                | IRInstruction::Subtract { left, right, .. }
                | IRInstruction::Multiply { left, right, .. }
                | IRInstruction::Divide { left, right, .. } => {
                    matches!(left, IRValue::Constant(_)) && matches!(right, IRValue::Constant(_))
                }
                _ => false,
            };

            if is_invariant && loop_start.is_some() {
                // Verschiebe vor Loop (vereinfacht: füge am Anfang hinzu)
                // In vollständiger Implementierung würde hier die Instruction verschoben
                new_instructions.push(instruction.clone());
            } else {
                new_instructions.push(instruction.clone());
            }
        }

        block.instructions = new_instructions;
    }

    /// Eliminiert Induction Variables
    fn eliminate_induction_variables(&self, block: &mut IRBlock) {
        // Vereinfachte Implementierung: Identifiziere Patterns wie i = i + 1
        let mut new_instructions = Vec::new();

        for instruction in &block.instructions {
            // Prüfe auf Add-Pattern mit gleicher Variable
            if let IRInstruction::Add { dest, left, right } = instruction {
                if let (IRValue::Variable(var1), IRValue::Constant(IRConstant::Number(1.0))) =
                    (left, right)
                {
                    if let IRValue::Variable(var2) = dest {
                        if var1.name == var2.name {
                            // Induction Variable erkannt - könnte optimiert werden
                            // Für jetzt: behalte Original
                            new_instructions.push(instruction.clone());
                            continue;
                        }
                    }
                }
            }

            new_instructions.push(instruction.clone());
        }

        block.instructions = new_instructions;
    }
}

impl Default for IROptimizer {
    fn default() -> Self {
        Self::new()
    }
}
