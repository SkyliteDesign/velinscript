// Variable Inspector
// Extrahiert und inspiziert Variablen

use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub type_name: String,
    pub line: usize,
}

pub struct VariableInspector;

impl VariableInspector {
    pub fn new() -> Self {
        Self
    }
    
    pub fn extract_variables(&self, content: &str) -> Result<Vec<Variable>> {
        let mut variables = Vec::new();
        
        // Einfache String-basierte Extraktion
        // In Produktion sollte man den AST nutzen
        for (line_num, line) in content.lines().enumerate() {
            if line.trim().starts_with("let ") {
                if let Some(var) = self.parse_variable(line, line_num + 1) {
                    variables.push(var);
                }
            }
        }
        
        Ok(variables)
    }
    
    fn parse_variable(&self, line: &str, line_num: usize) -> Option<Variable> {
        // Vereinfachte Parsing-Logik
        // Extrahiere "let name = value"
        if let Some(start) = line.find("let ") {
            let rest = &line[start + 4..];
            if let Some(equals) = rest.find('=') {
                let name = rest[..equals].trim().to_string();
                let value = rest[equals + 1..].trim().to_string();
                
                return Some(Variable {
                    name,
                    value,
                    type_name: "unknown".to_string(),
                    line: line_num,
                });
            }
        }
        None
    }
}
