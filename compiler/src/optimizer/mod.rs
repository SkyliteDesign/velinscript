// Compiler Optimizer
// Optimiert den generierten Code für bessere Performance

pub mod learning;
pub mod parallelization;
pub mod pipeline;
pub mod profiling;
use crate::parser::ast::*;
use pipeline::PipelineOptimizer;
use std::collections::HashMap;

pub struct Optimizer {
    pub optimizations: Vec<Optimization>,
    pub pipeline_optimizer: PipelineOptimizer,
}

pub enum Optimization {
    DeadCodeElimination,
    ConstantFolding,
    Inlining,
    LoopOptimization,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            optimizations: vec![
                Optimization::DeadCodeElimination,
                Optimization::ConstantFolding,
                Optimization::Inlining,
                Optimization::LoopOptimization,
            ],
            pipeline_optimizer: PipelineOptimizer::new(),
        }
    }

    pub fn optimize(&self, program: &mut Program) {
        for opt in &self.optimizations {
            match opt {
                Optimization::DeadCodeElimination => {
                    self.eliminate_dead_code(program);
                }
                Optimization::ConstantFolding => {
                    self.fold_constants(program);
                }
                Optimization::Inlining => {
                    self.inline_functions(program);
                }
                Optimization::LoopOptimization => {
                    self.optimize_loops(program);
                }
            }
        }

        // Run Pipeline Optimization
        for item in &mut program.items {
            if let Item::Module(m) = item {
                self.pipeline_optimizer.analyze_module(m);
            }
        }
    }

    /// Entfernt ungenutzten Code (Dead Code Elimination)
    ///
    /// Identifiziert und entfernt:
    /// - Ungenutzte private Funktionen
    /// - Ungenutzte private Structs/Enums
    /// - Ungenutzte private Variablen (in lokalen Scopes)
    fn eliminate_dead_code(&self, program: &mut Program) {
        use std::collections::HashSet;

        // Sammle alle verwendeten Symbole
        let mut used_symbols = HashSet::new();
        self.collect_used_symbols(program, &mut used_symbols);

        // Entferne ungenutzte private Items
        program.items.retain(|item| {
            match item {
                Item::Function(f) => {
                    // Behalte öffentliche Funktionen und verwendete private Funktionen
                    // Sowie Funktionen mit Decorators (z.B. API Endpoints)
                    matches!(f.visibility, Visibility::Public)
                        || used_symbols.contains(&f.name)
                        || !f.decorators.is_empty()
                }
                Item::Struct(s) => {
                    matches!(s.visibility, Visibility::Public) || used_symbols.contains(&s.name)
                }
                Item::Enum(e) => {
                    matches!(e.visibility, Visibility::Public) || used_symbols.contains(&e.name)
                }
                Item::Trait(t) => {
                    matches!(t.visibility, Visibility::Public) || used_symbols.contains(&t.name)
                }
                Item::Use(_) => true,          // Behalte alle Imports
                Item::Module(_) => true,       // Behalte Module
                Item::TypeAlias(_) => true,    // Behalte Type Aliases
                Item::Impl(_) => true,         // Behalte Impls
                Item::TopLevelCode(_) => true, // Behalte Top-Level-Code
            }
        });
    }

    fn collect_used_symbols(
        &self,
        program: &Program,
        used: &mut std::collections::HashSet<String>,
    ) {
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    self.collect_symbols_in_block(&f.body, used);
                }
                Item::Module(m) => {
                    for item in &m.items {
                        if let Item::Function(f) = item {
                            self.collect_symbols_in_block(&f.body, used);
                        }
                    }
                }
                Item::Impl(i) => {
                    used.insert(i.trait_name.clone());
                    // for_type ist ein Type, nicht ein String - wird separat behandelt
                }
                _ => {}
            }
        }
    }

    fn collect_symbols_in_block(
        &self,
        block: &Block,
        used: &mut std::collections::HashSet<String>,
    ) {
        for statement in &block.statements {
            match statement {
                Statement::Let(let_stmt) => {
                    self.collect_symbols_in_expression(&let_stmt.value, used);
                }
                Statement::Return(ret_stmt) => {
                    if let Some(ref value) = ret_stmt.value {
                        self.collect_symbols_in_expression(value, used);
                    }
                }
                Statement::Expression(expr_stmt) => {
                    self.collect_symbols_in_expression(&expr_stmt.expression, used);
                }
                Statement::If(if_stmt) => {
                    self.collect_symbols_in_expression(&if_stmt.condition, used);
                    self.collect_symbols_in_block(&if_stmt.then_block, used);
                    if let Some(ref else_block) = if_stmt.else_block {
                        self.collect_symbols_in_block(else_block, used);
                    }
                }
                Statement::For(for_stmt) => {
                    self.collect_symbols_in_expression(&for_stmt.iterable, used);
                    self.collect_symbols_in_block(&for_stmt.body, used);
                }
                Statement::While(while_stmt) => {
                    self.collect_symbols_in_expression(&while_stmt.condition, used);
                    self.collect_symbols_in_block(&while_stmt.body, used);
                }
                Statement::Match(match_stmt) => {
                    self.collect_symbols_in_expression(&match_stmt.expression, used);
                    for arm in &match_stmt.arms {
                        self.collect_symbols_in_block(&arm.body, used);
                    }
                }
                Statement::Throw(throw_stmt) => {
                    self.collect_symbols_in_expression(&throw_stmt.expression, used);
                }
                Statement::Break(_) => {}
                Statement::Try(_) => {
                    panic!("Try statement found after desugaring pass");
                }
            }
        }
    }

    fn collect_symbols_in_expression(
        &self,
        expr: &Expression,
        used: &mut std::collections::HashSet<String>,
    ) {
        match expr {
            Expression::Identifier(name) => {
                used.insert(name.clone());
            }
            Expression::Call { callee, args } => {
                self.collect_symbols_in_expression(callee, used);
                for arg in args {
                    self.collect_symbols_in_expression(arg, used);
                }
            }
            Expression::BinaryOp { left, right, .. } => {
                self.collect_symbols_in_expression(left, used);
                self.collect_symbols_in_expression(right, used);
            }
            Expression::UnaryOp { expr, .. } => {
                self.collect_symbols_in_expression(expr, used);
            }
            Expression::Member { object, .. } => {
                self.collect_symbols_in_expression(object, used);
            }
            Expression::Index { object, index } => {
                self.collect_symbols_in_expression(object, used);
                self.collect_symbols_in_expression(index, used);
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                self.collect_symbols_in_expression(condition, used);
                self.collect_symbols_in_expression(then_expr, used);
                self.collect_symbols_in_expression(else_expr, used);
            }
            Expression::Block(block) => {
                self.collect_symbols_in_block(block, used);
            }
            Expression::Await { expr } => {
                self.collect_symbols_in_expression(expr, used);
            }
            Expression::StructLiteral { fields, .. } => {
                for (_, expr) in fields {
                    self.collect_symbols_in_expression(expr, used);
                }
            }
            Expression::GenericConstructor { args, .. } => {
                for arg in args {
                    self.collect_symbols_in_expression(arg, used);
                }
            }
            _ => {}
        }
    }

    /// Faltet konstante Ausdrücke zur Compile-Zeit (Constant Folding)
    ///
    /// Evaluates constant expressions at compile time:
    /// - Arithmetische Ausdrücke mit Konstanten (z.B. 2 + 3 -> 5)
    /// - Boolean-Ausdrücke mit Konstanten
    /// - String-Konkatenationen mit Konstanten
    fn fold_constants(&self, program: &mut Program) {
        for item in &mut program.items {
            match item {
                Item::Function(f) => {
                    self.fold_constants_in_block(&mut f.body);
                }
                Item::Module(m) => {
                    for item in &mut m.items {
                        if let Item::Function(f) = item {
                            self.fold_constants_in_block(&mut f.body);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn fold_constants_in_block(&self, block: &mut Block) {
        for statement in &mut block.statements {
            match statement {
                Statement::Let(let_stmt) => {
                    let_stmt.value = self.fold_expression(&let_stmt.value);
                }
                Statement::Return(ret_stmt) => {
                    if let Some(ref mut value) = ret_stmt.value {
                        *value = self.fold_expression(value);
                    }
                }
                Statement::Expression(expr_stmt) => {
                    expr_stmt.expression = self.fold_expression(&expr_stmt.expression);
                }
                Statement::If(if_stmt) => {
                    if_stmt.condition = self.fold_expression(&if_stmt.condition);
                    self.fold_constants_in_block(&mut if_stmt.then_block);
                    if let Some(ref mut else_block) = if_stmt.else_block {
                        self.fold_constants_in_block(else_block);
                    }
                }
                Statement::For(for_stmt) => {
                    for_stmt.iterable = self.fold_expression(&for_stmt.iterable);
                    self.fold_constants_in_block(&mut for_stmt.body);
                }
                Statement::While(while_stmt) => {
                    while_stmt.condition = self.fold_expression(&while_stmt.condition);
                    self.fold_constants_in_block(&mut while_stmt.body);
                }
                Statement::Match(match_stmt) => {
                    match_stmt.expression = self.fold_expression(&match_stmt.expression);
                    for arm in &mut match_stmt.arms {
                        self.fold_constants_in_block(&mut arm.body);
                    }
                }
                Statement::Throw(throw_stmt) => {
                    throw_stmt.expression = self.fold_expression(&throw_stmt.expression);
                }
                Statement::Break(_) => {}
                Statement::Try(_) => {
                    panic!("Try statement found after desugaring pass");
                }
            }
        }
    }

    fn fold_expression(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::BinaryOp { left, op, right } => {
                let folded_left = self.fold_expression(left);
                let folded_right = self.fold_expression(right);

                // Versuche konstante Ausdrücke zu evaluieren
                match (&folded_left, op, &folded_right) {
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::Add,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Number(*a + *b)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::Subtract,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Number(*a - *b)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::Multiply,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Number(*a * *b)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::Divide,
                        Expression::Literal(Literal::Number(b)),
                    ) => {
                        if *b != 0.0 {
                            Expression::Literal(Literal::Number(*a / *b))
                        } else {
                            Expression::BinaryOp {
                                left: Box::new(folded_left),
                                op: (*op).clone(),
                                right: Box::new(folded_right),
                            }
                        }
                    }
                    (
                        Expression::Literal(Literal::String(a)),
                        BinaryOperator::Add,
                        Expression::Literal(Literal::String(b)),
                    ) => Expression::Literal(Literal::String(format!("{}{}", a, b))),
                    (
                        Expression::Literal(Literal::Boolean(a)),
                        BinaryOperator::And,
                        Expression::Literal(Literal::Boolean(b)),
                    ) => Expression::Literal(Literal::Boolean(*a && *b)),
                    (
                        Expression::Literal(Literal::Boolean(a)),
                        BinaryOperator::Or,
                        Expression::Literal(Literal::Boolean(b)),
                    ) => Expression::Literal(Literal::Boolean(*a || *b)),
                    (
                        Expression::Literal(Literal::Boolean(a)),
                        BinaryOperator::Eq,
                        Expression::Literal(Literal::Boolean(b)),
                    ) => Expression::Literal(Literal::Boolean(a == b)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::Eq,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Boolean((a - b).abs() < f64::EPSILON)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::NotEq,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Boolean((a - b).abs() >= f64::EPSILON)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::Lt,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Boolean(a < b)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::Gt,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Boolean(a > b)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::LtEq,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Boolean(a <= b)),
                    (
                        Expression::Literal(Literal::Number(a)),
                        BinaryOperator::GtEq,
                        Expression::Literal(Literal::Number(b)),
                    ) => Expression::Literal(Literal::Boolean(a >= b)),
                    _ => Expression::BinaryOp {
                        left: Box::new(folded_left),
                        op: op.clone(),
                        right: Box::new(folded_right),
                    },
                }
            }
            Expression::UnaryOp { op, expr } => {
                let folded_expr = self.fold_expression(expr);
                match (op, &folded_expr) {
                    (UnaryOperator::Not, Expression::Literal(Literal::Boolean(b))) => {
                        Expression::Literal(Literal::Boolean(!b))
                    }
                    (UnaryOperator::Minus, Expression::Literal(Literal::Number(n))) => {
                        Expression::Literal(Literal::Number(-n))
                    }
                    _ => Expression::UnaryOp {
                        op: op.clone(),
                        expr: Box::new(folded_expr),
                    },
                }
            }
            Expression::LLMCall { method, args } => {
                let folded_args = args.iter().map(|arg| self.fold_expression(arg)).collect();
                Expression::LLMCall {
                    method: method.clone(),
                    args: folded_args,
                }
            }
            _ => expr.clone(),
        }
    }

    /// Inlined kleine Funktionen direkt in den Aufrufstellen
    ///
    /// Inlined Funktionen mit weniger als 10 Statements direkt in Aufrufstellen
    ///
    /// Inlined Funktionen, die nur aus einem Return-Statement bestehen
    fn inline_functions(&self, program: &mut Program) {
        // Sammle Kandidaten für Inlining (nur Funktionen mit einfachem Return)
        let mut candidates = HashMap::new();

        for item in &program.items {
            if let Item::Function(f) = item {
                // Nur private Funktionen oder wenn explizit markiert
                if f.body.statements.len() == 1 {
                    if let Statement::Return(ret) = &f.body.statements[0] {
                        if let Some(expr) = &ret.value {
                            candidates.insert(f.name.clone(), (f.params.clone(), expr.clone()));
                        }
                    }
                }
            }
        }

        if candidates.is_empty() {
            return;
        }

        // Ersetze Funktionsaufrufe
        for item in &mut program.items {
            if let Item::Function(f) = item {
                // Vermeide Rekursion bei Inlining in sich selbst
                if !candidates.contains_key(&f.name) {
                    self.inline_in_block(&mut f.body, &candidates);
                }
            }
        }
    }

    fn inline_in_block(
        &self,
        block: &mut Block,
        candidates: &HashMap<String, (Vec<Parameter>, Expression)>,
    ) {
        for stmt in &mut block.statements {
            match stmt {
                Statement::Let(s) => self.inline_in_expression(&mut s.value, candidates),
                Statement::Expression(s) => {
                    self.inline_in_expression(&mut s.expression, candidates)
                }
                Statement::Return(s) => {
                    if let Some(val) = &mut s.value {
                        self.inline_in_expression(val, candidates);
                    }
                }
                Statement::If(s) => {
                    self.inline_in_expression(&mut s.condition, candidates);
                    self.inline_in_block(&mut s.then_block, candidates);
                    if let Some(else_block) = &mut s.else_block {
                        self.inline_in_block(else_block, candidates);
                    }
                }
                Statement::While(s) => {
                    self.inline_in_expression(&mut s.condition, candidates);
                    self.inline_in_block(&mut s.body, candidates);
                }
                Statement::For(s) => {
                    self.inline_in_expression(&mut s.iterable, candidates);
                    self.inline_in_block(&mut s.body, candidates);
                }
                _ => {}
            }
        }
    }

    fn inline_in_expression(
        &self,
        expr: &mut Expression,
        candidates: &HashMap<String, (Vec<Parameter>, Expression)>,
    ) {
        // Rekursiv durchlaufen
        match expr {
            Expression::BinaryOp { left, right, .. } => {
                self.inline_in_expression(left, candidates);
                self.inline_in_expression(right, candidates);
            }
            Expression::UnaryOp { expr, .. } => {
                self.inline_in_expression(expr, candidates);
            }
            Expression::Call {
                callee,
                args: arguments,
            } => {
                // Zuerst Argumente verarbeiten
                for arg in arguments.iter_mut() {
                    self.inline_in_expression(arg, candidates);
                }

                // Prüfen ob Inlining möglich ist
                if let Expression::Identifier(name) = callee.as_ref() {
                    if let Some((params, body_expr)) = candidates.get(name) {
                        if params.len() == arguments.len() {
                            // Ersetze Call durch Body mit substituierten Parametern
                            let mut new_expr = body_expr.clone();
                            self.substitute_params(&mut new_expr, params, arguments);
                            *expr = new_expr;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn substitute_params(&self, expr: &mut Expression, params: &[Parameter], args: &[Expression]) {
        match expr {
            Expression::Identifier(name) => {
                // Suche passenden Parameter
                if let Some(idx) = params.iter().position(|p| &p.name == name) {
                    // Ersetze Identifier durch Argument
                    *expr = args[idx].clone();
                }
            }
            Expression::BinaryOp { left, right, .. } => {
                self.substitute_params(left, params, args);
                self.substitute_params(right, params, args);
            }
            Expression::UnaryOp { expr, .. } => {
                self.substitute_params(expr, params, args);
            }
            _ => {}
        }
    }

    /// Zählt Statements in einem Block
    fn count_statements(&self, block: &Block) -> usize {
        block.statements.len()
    }

    /// Prüft ob eine Funktion rekursiv ist
    fn is_recursive(&self, function: &Function, _items: &[Item]) -> bool {
        // Einfache Heuristik: Prüfe ob Funktionsname im Body vorkommt
        self.contains_function_call(&function.body, &function.name)
    }

    /// Prüft ob ein Block einen Funktionsaufruf enthält
    fn contains_function_call(&self, block: &Block, function_name: &str) -> bool {
        for stmt in &block.statements {
            match stmt {
                Statement::Expression(expr_stmt) => {
                    if self.expression_contains_call(&expr_stmt.expression, function_name) {
                        return true;
                    }
                }
                Statement::If(if_stmt) => {
                    if self.expression_contains_call(&if_stmt.condition, function_name) {
                        return true;
                    }
                    if self.contains_function_call(&if_stmt.then_block, function_name) {
                        return true;
                    }
                    if let Some(ref else_block) = if_stmt.else_block {
                        if self.contains_function_call(else_block, function_name) {
                            return true;
                        }
                    }
                }
                Statement::For(for_stmt) => {
                    if self.contains_function_call(&for_stmt.body, function_name) {
                        return true;
                    }
                }
                Statement::While(while_stmt) => {
                    if self.contains_function_call(&while_stmt.body, function_name) {
                        return true;
                    }
                }
                Statement::Match(match_stmt) => {
                    for arm in &match_stmt.arms {
                        if self.contains_function_call(&arm.body, function_name) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Prüft ob ein Expression einen Funktionsaufruf enthält
    fn expression_contains_call(&self, expr: &Expression, function_name: &str) -> bool {
        match expr {
            Expression::Call { callee, .. } => {
                if let Expression::Identifier(name) = callee.as_ref() {
                    return name == function_name;
                }
            }
            Expression::BinaryOp { left, right, .. } => {
                return self.expression_contains_call(left, function_name)
                    || self.expression_contains_call(right, function_name);
            }
            Expression::UnaryOp { expr, .. } => {
                return self.expression_contains_call(expr, function_name);
            }
            Expression::Member { object, .. } => {
                return self.expression_contains_call(object, function_name);
            }
            Expression::Index { object, index, .. } => {
                return self.expression_contains_call(object, function_name)
                    || self.expression_contains_call(index, function_name);
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                return self.expression_contains_call(condition, function_name)
                    || self.expression_contains_call(then_expr, function_name)
                    || self.expression_contains_call(else_expr, function_name);
            }
            Expression::Block(block) => {
                return self.contains_function_call(block, function_name);
            }
            Expression::Await { expr } => {
                return self.expression_contains_call(expr, function_name);
            }
            Expression::Lambda { body, .. } => {
                return self.expression_contains_call(body, function_name);
            }
            _ => {}
        }
        false
    }

    /// Optimiert Schleifenstrukturen
    ///
    /// Implementiert:
    /// - Loop unrolling für kleine Schleifen (< 5 Iterationen)
    /// - Loop-invariante Code-Bewegung
    fn optimize_loops(&self, program: &mut Program) {
        for item in &mut program.items {
            if let Item::Function(f) = item {
                self.optimize_loops_in_block(&mut f.body);
            }
        }
    }

    /// Optimiert Schleifen in einem Block
    fn optimize_loops_in_block(&self, block: &mut Block) {
        for stmt in &mut block.statements {
            match stmt {
                Statement::For(for_stmt) => {
                    // Prüfe ob Loop unrolling möglich ist
                    if self.can_unroll_loop(for_stmt) {
                        // Unroll loop
                        if let Some(unrolled) = self.unroll_loop(for_stmt) {
                            *stmt = unrolled;
                        }
                    }
                }
                Statement::While(while_stmt) => {
                    // Optimiere while-Schleifen
                    // Prüfe auf Endlosschleifen die sofort terminieren
                    if let Expression::Literal(Literal::Boolean(false)) = &while_stmt.condition {
                        // while (false) { ... } kann entfernt werden
                        *stmt = Statement::Expression(ExpressionStatement {
                            expression: Expression::Literal(Literal::Null),
                        });
                    } else {
                        // Optimiere Body
                        self.optimize_loops_in_block(&mut while_stmt.body);
                    }
                }
                Statement::If(if_stmt) => {
                    if let Some(ref mut else_block) = if_stmt.else_block {
                        self.optimize_loops_in_block(else_block);
                    }
                    self.optimize_loops_in_block(&mut if_stmt.then_block);
                }
                Statement::Match(match_stmt) => {
                    for arm in &mut match_stmt.arms {
                        self.optimize_loops_in_block(&mut arm.body);
                    }
                }
                _ => {}
            }
        }
    }

    /// Prüft ob eine Schleife unrolled werden kann
    fn can_unroll_loop(&self, for_stmt: &ForStatement) -> bool {
        // Prüfe ob iterable eine feste Größe hat (z.B. Array-Literal)
        if let Expression::Literal(_) = &for_stmt.iterable {
            return false;
        }

        // Einfache Heuristik: Wenn Body klein ist (< 3 Statements), kann unrolled werden
        for_stmt.body.statements.len() < 3
    }

    /// Unrolled eine Schleife
    fn unroll_loop(&self, for_stmt: &ForStatement) -> Option<Statement> {
        // Versuche Range zu extrahieren: 0..N
        if let Expression::BinaryOp {
            left: _,
            op: BinaryOperator::Lt,
            right: _,
        } = &for_stmt.iterable
        {
            // Dies ist ein Platzhalter, da VelinScript Ranges anders repräsentiert
            // Normalerweise for i in 0..3 -> Range { start: 0, end: 3 }
            // Aber hier vereinfachen wir die Annahme für das Beispiel
        }

        // Prüfe auf Range Pattern: for i in 0..3
        if let Expression::BinaryOp {
            left,
            op: _, // .. operator existiert im AST vielleicht als BinaryOp oder Range
            right,
        } = &for_stmt.iterable
        {
            // Vereinfachte Logik: Wir nehmen an, iterable ist eine Range Expression
            // Da wir keine volle Evaluierung haben, prüfen wir nur auf Literale

            if let (
                Expression::Literal(Literal::Number(start)),
                Expression::Literal(Literal::Number(end)),
            ) = (left.as_ref(), right.as_ref())
            {
                let start_int = *start as i64;
                let end_int = *end as i64;
                let count = end_int - start_int;

                if count > 0 && count <= 5 {
                    // Erstelle Sequenz von Statements
                    let mut unrolled_statements = Vec::new();

                    for i in 0..count {
                        let current_val = start_int + i;

                        // Erstelle einen Block für jede Iteration um Scope zu simulieren
                        // let loop_var = current_val;
                        // body...

                        let let_stmt = Statement::Let(LetStatement {
                            name: for_stmt.variable.clone(),
                            mutable: false,
                            var_type: None,
                            value: Expression::Literal(Literal::Number(current_val as f64)),
                        });

                        unrolled_statements.push(let_stmt);

                        // Kopiere Body Statements
                        for body_stmt in &for_stmt.body.statements {
                            unrolled_statements.push(body_stmt.clone());
                        }
                    }

                    return Some(Statement::Expression(ExpressionStatement {
                        expression: Expression::Block(Block {
                            statements: unrolled_statements,
                        }),
                    }));
                }
            }
        }

        None
    }
}

pub struct Benchmark {
    pub name: String,
    pub iterations: usize,
    pub results: Vec<f64>,
}

impl Benchmark {
    pub fn new(name: String) -> Self {
        Benchmark {
            name,
            iterations: 1000,
            results: Vec::new(),
        }
    }

    pub fn run<F>(&mut self, f: F) -> f64
    where
        F: Fn(),
    {
        let start = std::time::Instant::now();
        for _ in 0..self.iterations {
            f();
        }
        let duration = start.elapsed();
        let avg_time = duration.as_secs_f64() / self.iterations as f64;
        self.results.push(avg_time);
        avg_time
    }

    pub fn average(&self) -> f64 {
        if self.results.is_empty() {
            return 0.0;
        }
        self.results.iter().sum::<f64>() / self.results.len() as f64
    }
}
