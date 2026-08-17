use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use crate::config::LibraryConfig;

pub struct TypeCheckerIntegration;

impl TypeCheckerIntegration {
    pub fn new() -> Self {
        Self
    }
    
    pub fn integrate(&self, config: &LibraryConfig) -> Result<()> {
        let checker_path = PathBuf::from("compiler/src/type_checker/checker.rs");
        
        if !checker_path.exists() {
            anyhow::bail!(
                "❌ Fehler: compiler/src/type_checker/checker.rs nicht gefunden!\n\
                 Stellen Sie sicher, dass Sie das Tool vom Projekt-Root ausführen."
            );
        }
        
        // Lese bestehende checker.rs
        let content = fs::read_to_string(&checker_path)
            .with_context(|| format!("Konnte checker.rs nicht lesen: {:?}", checker_path))?;
        
        // Generiere Integration-Code
        let integration_code = self.generate_integration_code(config);
        
        // Finde Einfügepunkt (nach anderen Standard Library Definitionen)
        let insertion_marker = "// --- Extended Standard Library Variables ---";
        
        if !content.contains(insertion_marker) {
            anyhow::bail!("Konnte Einfügepunkt in checker.rs nicht finden");
        }
        
        // Prüfe ob bereits integriert
        if content.contains(&format!("{}Stdlib", config.pascal_case_name())) {
            println!("    ⚠️  Modul bereits im Type Checker vorhanden, überspringe...");
            return Ok(());
        }
        
        // Füge vor insertion_marker ein
        let new_content = content.replace(
            insertion_marker,
            &format!("{}\n{}", integration_code, insertion_marker)
        );
        
        fs::write(&checker_path, new_content)
            .with_context(|| "Konnte checker.rs nicht schreiben")?;
        
        Ok(())
    }
    
    fn generate_integration_code(&self, config: &LibraryConfig) -> String {
        let mut code = String::new();
        
        // Typ-Definition
        code.push_str(&format!(
            "        env.define_type(\"{}Stdlib\".to_string(), Type::Named(\"{}Stdlib\".to_string()));\n",
            config.pascal_case_name(),
            config.pascal_case_name()
        ));
        
        // Variable-Definition
        code.push_str(&format!(
            "        env.define_variable(\"{}\".to_string(), Type::Named(\"{}Stdlib\".to_string()));\n",
            config.snake_case_name(),
            config.pascal_case_name()
        ));
        
        // Funktionen
        for function in &config.functions {
            code.push_str(&format!(
                "        env.define_function(\"{}.{}\".to_string(), FunctionSignature {{\n",
                config.snake_case_name(),
                function.name
            ));
            code.push_str(&format!(
                "            name: \"{}.{}\".to_string(),\n",
                config.snake_case_name(),
                function.name
            ));
            code.push_str("            params: vec![\n");
            
            for param in &function.params {
                code.push_str(&format!(
                    "                ParameterInfo {{\n                    name: \"{}\".to_string(),\n                    param_type: {},\n                }},\n",
                    param.name,
                    self.velin_type_to_type(&param.param_type)
                ));
            }
            
            code.push_str("            ],\n");
            
            if let Some(return_type) = &function.return_type {
                code.push_str(&format!(
                    "            return_type: Some({}),\n",
                    self.velin_type_to_type(return_type)
                ));
            } else {
                code.push_str("            return_type: None,\n");
            }
            
            code.push_str("        });\n");
        }
        
        format!("        // --- {} Standard Library ---\n{}", config.pascal_case_name(), code)
    }
    
    fn velin_type_to_type(&self, velin_type: &str) -> String {
        match velin_type {
            "string" => "Type::String".to_string(),
            "number" => "Type::Number".to_string(),
            "boolean" => "Type::Boolean".to_string(),
            "List<string>" => "Type::List(Box::new(Type::String))".to_string(),
            "Map<string, any>" => "Type::Map(Box::new(Type::String), Box::new(Type::Any))".to_string(),
            "any" => "Type::Any".to_string(),
            "void" => "Type::Unit".to_string(),
            _ => format!("Type::Named(\"{}\".to_string())", velin_type),
        }
    }
}
