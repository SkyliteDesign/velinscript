// Interpreter
// Interpretiert einfache Ausdrücke direkt

use evalexpr::*;
use anyhow::Result;

pub struct Interpreter {
    context: HashMapContext,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut context = HashMapContext::new();
        
        // Registriere Standard-Funktionen
        context.set_function("sqrt".into(), Function::new(|argument| {
            let value = argument.as_float()?;
            Ok(Value::Float(value.sqrt()))
        })).unwrap();
        
        context.set_function("abs".into(), Function::new(|argument| {
            let value = argument.as_float()?;
            Ok(Value::Float(value.abs()))
        })).unwrap();
        
        Self { context }
    }
    
    pub fn evaluate_expression(&self, code: &str) -> Result<String> {
        // Versuche als einfachen Ausdruck zu evaluieren
        let trimmed = code.trim();
        
        // Prüfe ob es ein einfacher Ausdruck ist (keine Funktionen, keine Blöcke)
        if trimmed.contains('{') || trimmed.contains("fn") || trimmed.contains("let") {
            return Err(anyhow::anyhow!("Komplexer Code, nutze Compiler"));
        }
        
        let value = eval_with_context(trimmed, &self.context)?;
        Ok(format!("{}", value))
    }
}
