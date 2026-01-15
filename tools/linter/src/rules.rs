// Linter Rules
// Definiert alle Linter-Regeln

use crate::analyzer::LintIssue;
use velin_compiler::parser::ast::*;

pub trait LintRule: Send + Sync {
    fn name(&self) -> &str;
    fn check(&self, program: &Program, file_path: &str) -> Vec<LintIssue>;
}

pub struct UnusedVariableRule;

impl LintRule for UnusedVariableRule {
    fn name(&self) -> &str {
        "unused-variable"
    }
    
    fn check(&self, program: &Program, file_path: &str) -> Vec<LintIssue> {
        use std::collections::{HashMap, HashSet};
        
        let mut issues = Vec::new();
        let mut defined_vars = HashMap::new();
        let mut used_vars = HashSet::new();
        
        // Sammle definierte und verwendete Variablen
        for item in &program.items {
            self.collect_variables(item, &mut defined_vars, &mut used_vars);
        }
        
        // Finde ungenutzte Variablen
        for (var_name, (line, col)) in &defined_vars {
            if !used_vars.contains(var_name) {
                issues.push(LintIssue {
                    file: file_path.to_string(),
                    line: *line,
                    column: *col,
                    severity: "warning".to_string(),
                    rule: self.name().to_string(),
                    message: format!("Ungenutzte Variable: '{}'", var_name),
                    suggestion: Some(format!("Entferne die Variable '{}' oder verwende sie", var_name)),
                });
            }
        }
        
        issues
    }
}

impl UnusedVariableRule {
    fn collect_variables(
        &self,
        item: &Item,
        defined: &mut HashMap<String, (usize, usize)>,
        used: &mut HashSet<String>,
    ) {
        match item {
            Item::Function(f) => {
                // Parameter sind immer verwendet
                for param in &f.params {
                    used.insert(param.name.clone());
                }
                self.collect_in_block(&f.body, defined, used);
            }
            _ => {}
        }
    }
    
    fn collect_in_block(
        &self,
        block: &Block,
        defined: &mut HashMap<String, (usize, usize)>,
        used: &mut HashSet<String>,
    ) {
        for stmt in &block.statements {
            match stmt {
                Statement::Let(let_stmt) => {
                    defined.insert(let_stmt.name.clone(), (0, 0)); // Line/Col tracking würde hier erweitert
                    self.collect_in_expression(&let_stmt.value, used);
                }
                Statement::Return(ret_stmt) => {
                    if let Some(ref expr) = ret_stmt.value {
                        self.collect_in_expression(expr, used);
                    }
                }
                Statement::Expression(expr_stmt) => {
                    self.collect_in_expression(&expr_stmt.expression, used);
                }
                Statement::If(if_stmt) => {
                    self.collect_in_expression(&if_stmt.condition, used);
                    self.collect_in_block(&if_stmt.then_block, defined, used);
                    if let Some(ref else_block) = if_stmt.else_block {
                        self.collect_in_block(else_block, defined, used);
                    }
                }
                Statement::For(for_stmt) => {
                    used.insert(for_stmt.variable.clone());
                    self.collect_in_expression(&for_stmt.iterable, used);
                    self.collect_in_block(&for_stmt.body, defined, used);
                }
                Statement::While(while_stmt) => {
                    self.collect_in_expression(&while_stmt.condition, used);
                    self.collect_in_block(&while_stmt.body, defined, used);
                }
                Statement::Match(match_stmt) => {
                    self.collect_in_expression(&match_stmt.expression, used);
                    for arm in &match_stmt.arms {
                        self.collect_in_block(&arm.body, defined, used);
                    }
                }
            }
        }
    }
    
    fn collect_in_expression(&self, expr: &Expression, used: &mut HashSet<String>) {
        match expr {
            Expression::Identifier(name) => {
                used.insert(name.clone());
            }
            Expression::BinaryOp { left, right, .. } => {
                self.collect_in_expression(left, used);
                self.collect_in_expression(right, used);
            }
            Expression::UnaryOp { expr, .. } => {
                self.collect_in_expression(expr, used);
            }
            Expression::Call { callee, args } => {
                self.collect_in_expression(callee, used);
                for arg in args {
                    self.collect_in_expression(arg, used);
                }
            }
            Expression::Member { object, .. } => {
                self.collect_in_expression(object, used);
            }
            Expression::Index { object, index } => {
                self.collect_in_expression(object, used);
                self.collect_in_expression(index, used);
            }
            Expression::If { condition, then_expr, else_expr } => {
                self.collect_in_expression(condition, used);
                self.collect_in_expression(then_expr, used);
                self.collect_in_expression(else_expr, used);
            }
            Expression::Block(block) => {
                // Block expressions haben ihren eigenen Scope
            }
            Expression::Await { expr } => {
                self.collect_in_expression(expr, used);
            }
            Expression::StructLiteral { fields, .. } => {
                for (_, field_expr) in fields {
                    self.collect_in_expression(field_expr, used);
                }
            }
            Expression::GenericConstructor { args, .. } => {
                for arg in args {
                    self.collect_in_expression(arg, used);
                }
            }
            Expression::Lambda { params, body, .. } => {
                // Parameter sind verwendet
                for param in params {
                    used.insert(param.name.clone());
                }
                self.collect_in_expression(body, used);
            }
            _ => {}
        }
    }
}

pub struct UnusedImportRule;

impl LintRule for UnusedImportRule {
    fn name(&self) -> &str {
        "unused-import"
    }
    
    fn check(&self, program: &Program, file_path: &str) -> Vec<LintIssue> {
        use std::collections::HashSet;
        
        let mut issues = Vec::new();
        let mut imports = HashSet::new();
        let mut used_items = HashSet::new();
        
        // Sammle Imports
        for item in &program.items {
            if let Item::Use(use_stmt) = item {
                let path = use_stmt.path.join("::");
                imports.insert(path.clone());
                if let Some(ref alias) = use_stmt.alias {
                    imports.insert(alias.clone());
                }
            }
        }
        
        // Sammle verwendete Items (vereinfacht)
        // In einer vollständigen Implementierung würde man alle Identifier durchsuchen
        
        // Finde ungenutzte Imports
        for import in &imports {
            if !used_items.contains(import) {
                issues.push(LintIssue {
                    file: file_path.to_string(),
                    line: 0,
                    column: 0,
                    severity: "warning".to_string(),
                    rule: self.name().to_string(),
                    message: format!("Ungenutzter Import: '{}'", import),
                    suggestion: Some(format!("Entferne den Import '{}'", import)),
                });
            }
        }
        
        issues
    }
}

pub struct ComplexityRule {
    max_complexity: usize,
}

impl ComplexityRule {
    pub fn new(max_complexity: usize) -> Self {
        ComplexityRule { max_complexity }
    }
}

impl LintRule for ComplexityRule {
    fn name(&self) -> &str {
        "complexity"
    }
    
    fn check(&self, program: &Program, file_path: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        for item in &program.items {
            if let Item::Function(f) = item {
                let complexity = self.calculate_complexity(&f.body);
                if complexity > self.max_complexity {
                    issues.push(LintIssue {
                        file: file_path.to_string(),
                        line: 0,
                        column: 0,
                        severity: "warning".to_string(),
                        rule: self.name().to_string(),
                        message: format!(
                            "Funktion '{}' hat eine Zyklomatische Komplexität von {} (max: {})",
                            f.name, complexity, self.max_complexity
                        ),
                        suggestion: Some("Teile die Funktion in kleinere Funktionen auf".to_string()),
                    });
                }
            }
        }
        
        issues
    }
}

impl ComplexityRule {
    fn calculate_complexity(&self, block: &Block) -> usize {
        let mut complexity = 1; // Basis-Komplexität
        
        for stmt in &block.statements {
            match stmt {
                Statement::If(_) => complexity += 1,
                Statement::While(_) => complexity += 1,
                Statement::For(_) => complexity += 1,
                Statement::Match(match_stmt) => {
                    complexity += match_stmt.arms.len();
                }
                Statement::Expression(expr_stmt) => {
                    if let Expression::If { .. } = expr_stmt.expression {
                        complexity += 1;
                    }
                }
                _ => {}
            }
        }
        
        complexity
    }
}

pub struct NamingConventionRule;

impl LintRule for NamingConventionRule {
    fn name(&self) -> &str {
        "naming"
    }
    
    fn check(&self, program: &Program, file_path: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();
        
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    if !self.is_snake_case(&f.name) && !self.is_camel_case(&f.name) {
                        issues.push(LintIssue {
                            file: file_path.to_string(),
                            line: 0,
                            column: 0,
                            severity: "warning".to_string(),
                            rule: self.name().to_string(),
                            message: format!("Funktionsname '{}' folgt nicht der Konvention (snake_case oder camelCase)", f.name),
                            suggestion: Some(format!("Benenne um zu: {}", self.to_snake_case(&f.name))),
                        });
                    }
                }
                Item::Struct(s) => {
                    if !self.is_pascal_case(&s.name) {
                        issues.push(LintIssue {
                            file: file_path.to_string(),
                            line: 0,
                            column: 0,
                            severity: "warning".to_string(),
                            rule: self.name().to_string(),
                            message: format!("Struct-Name '{}' sollte PascalCase sein", s.name),
                            suggestion: Some(format!("Benenne um zu: {}", self.to_pascal_case(&s.name))),
                        });
                    }
                }
                _ => {}
            }
        }
        
        issues
    }
}

impl NamingConventionRule {
    fn is_snake_case(&self, s: &str) -> bool {
        s.chars().all(|c| c.is_lowercase() || c == '_' || c.is_numeric())
    }
    
    fn is_camel_case(&self, s: &str) -> bool {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            first.is_lowercase() && chars.all(|c| c.is_alphanumeric())
        } else {
            false
        }
    }
    
    fn is_pascal_case(&self, s: &str) -> bool {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            first.is_uppercase() && chars.all(|c| c.is_alphanumeric())
        } else {
            false
        }
    }
    
    fn to_snake_case(&self, s: &str) -> String {
        s.chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    vec!['_', c.to_lowercase().next().unwrap()]
                } else {
                    vec![c.to_lowercase().next().unwrap()]
                }
            })
            .collect()
    }
    
    fn to_pascal_case(&self, s: &str) -> String {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            format!("{}{}", first.to_uppercase(), chars.as_str())
        } else {
            s.to_string()
        }
    }
}
