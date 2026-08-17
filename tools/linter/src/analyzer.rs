// Linter Analyzer
// Analysiert VelinScript Code mit verschiedenen Linter-Regeln

use crate::rules::{UnusedVariableRule, UnusedImportRule, ComplexityRule, NamingConventionRule, LintRule};
use velin_compiler::parser::ast::{Program, Statement, Expression};
use velin_compiler::parser::parser::Parser;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct LintIssue {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: String,
    pub rule: String,
    pub message: String,
    pub suggestion: Option<String>,
}

pub struct Linter {
    rules: Vec<Box<dyn LintRule>>,
    enabled_rules: HashMap<String, bool>,
}

impl Linter {
    pub fn new() -> Self {
        let mut linter = Linter {
            rules: Vec::new(),
            enabled_rules: HashMap::new(),
        };
        
        // Registriere Standard-Regeln
        linter.register_rule(Box::new(UnusedVariableRule));
        linter.register_rule(Box::new(UnusedImportRule));
        linter.register_rule(Box::new(ComplexityRule::new(10)));
        linter.register_rule(Box::new(NamingConventionRule));
        
        linter
    }
    
    pub fn register_rule(&mut self, rule: Box<dyn LintRule>) {
        let name = rule.name().to_string();
        self.rules.push(rule);
        self.enabled_rules.insert(name, true);
    }
    
    pub fn enable_all_rules(&mut self) {
        for (_, enabled) in &mut self.enabled_rules {
            *enabled = true;
        }
    }
    
    pub fn enable_rules(&mut self, rule_names: Vec<String>) {
        // Deaktiviere alle Regeln
        for (_, enabled) in &mut self.enabled_rules {
            *enabled = false;
        }
        
        // Aktiviere nur die angegebenen Regeln
        for rule_name in rule_names {
            if let Some(enabled) = self.enabled_rules.get_mut(&rule_name) {
                *enabled = true;
            }
        }
    }
    
    pub fn analyze(&self, code: &str, file_path: &std::path::Path) -> Result<Vec<LintIssue>> {
        // Parse Code
        let program = match Parser::parse(code) {
            Ok(p) => p,
            Err(_) => {
                // Wenn Parsing fehlschlägt, können wir nicht linten
                return Ok(vec![]);
            }
        };
        
        let file_str = file_path.to_string_lossy().to_string();
        let mut all_issues = Vec::new();
        
        // Führe alle aktivierten Regeln aus
        for rule in &self.rules {
            if self.enabled_rules.get(rule.name()).copied().unwrap_or(false) {
                let issues = rule.check(&program, &file_str);
                all_issues.extend(issues);
            }
        }
        
        Ok(all_issues)
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}
