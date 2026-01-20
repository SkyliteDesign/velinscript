use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use crate::config::LibraryConfig;

pub struct CodegenIntegration;

impl CodegenIntegration {
    pub fn new() -> Self {
        Self
    }
    
    pub fn integrate(&self, config: &LibraryConfig) -> Result<()> {
        // Suche nach codegen Dateien
        let codegen_path = PathBuf::from("compiler/src/codegen/rust.rs");
        
        if !codegen_path.exists() {
            anyhow::bail!(
                "❌ Fehler: compiler/src/codegen/rust.rs nicht gefunden!\n\
                 Stellen Sie sicher, dass Sie das Tool vom Projekt-Root ausführen."
            );
        }
        
        // Lese bestehende rust.rs
        let mut content = fs::read_to_string(&codegen_path)
            .with_context(|| format!("Konnte rust.rs nicht lesen: {:?}", codegen_path))?;
        
        // Prüfe ob bereits integriert
        if content.contains(&format!("generate_{}_call", config.snake_case_name())) {
            println!("    ⚠️  Modul bereits im Code Generator vorhanden, überspringe...");
            return Ok(());
        }
        
        // 1. Füge Dispatch-Logik hinzu
        let dispatch_marker = "} else if obj_name == \"env\" {";
        if content.contains(&dispatch_marker) {
            content = content.replace(
                &dispatch_marker,
                &format!("}} else if obj_name == \"{}\" {{\n                            self.generate_{}_call(member, args);\n                            return;\n                        {}",
                    config.snake_case_name(),
                    config.snake_case_name(),
                    dispatch_marker
                )
            );
        } else {
            println!("    ⚠️  Dispatch-Marker nicht gefunden, füge am Ende der Dispatch-Kette ein...");
        }
        
        // 2. Füge generate_*_call Funktion hinzu
        let function_code = self.generate_function_code(config);
        // Finde letzte generate_*_call Funktion und füge danach ein
        let function_marker = "    fn generate_env_call(&mut self, method: &str, args: &[Expression]) {";
        if content.contains(&function_marker) {
            // Finde das Ende dieser Funktion
            if let Some(pos) = content.find(&function_marker) {
                // Finde das nächste "    fn " nach dieser Funktion
                let after_function = &content[pos..];
                if let Some(next_fn_pos) = after_function.find("\n    fn ") {
                    let insert_pos = pos + next_fn_pos;
                    content.insert_str(insert_pos, &format!("\n{}", function_code));
                } else {
                    // Füge am Ende ein
                    content.push_str(&format!("\n{}", function_code));
                }
            }
        } else {
            // Füge am Ende ein
            content.push_str(&format!("\n{}", function_code));
        }
        
        fs::write(&codegen_path, content)
            .with_context(|| "Konnte rust.rs nicht schreiben")?;
        
        Ok(())
    }
    
    fn generate_function_code(&self, config: &LibraryConfig) -> String {
        let mut code = String::new();
        
        code.push_str(&format!(
            "    fn generate_{}_call(&mut self, method: &str, args: &[Expression]) {{\n",
            config.snake_case_name()
        ));
        code.push_str(&format!(
            "        use crate::stdlib::{}::{}Stdlib;\n",
            config.snake_case_name(),
            config.pascal_case_name()
        ));
        code.push_str("        \n");
        code.push_str("        match method {\n");
        
        for function in &config.functions {
            code.push_str(&format!(
                "            \"{}\" => {{\n",
                function.name
            ));
            
            // Generiere Parameter-Capture mit vollständiger Validierung
            let required_params = function.params.iter()
                .filter(|p| !p.optional.unwrap_or(false))
                .count();
            
            if required_params > 0 {
                code.push_str(&format!(
                    "                if args.len() >= {} {{\n",
                    required_params
                ));
            }
            
            if !function.params.is_empty() {
                for (idx, param) in function.params.iter().enumerate() {
                    if param.optional.unwrap_or(false) {
                        code.push_str(&format!(
                            "                    let {} = if args.len() > {} {{\n                        self.capture_expression(&args[{}])\n                    }} else {{\n                        String::new()\n                    }};\n",
                            param.name, idx, idx
                        ));
                    } else {
                        code.push_str(&format!(
                            "                    let {} = self.capture_expression(&args[{}]);\n",
                            param.name, idx
                        ));
                    }
                }
                
                // Generiere Funktionsaufruf
                let params: Vec<String> = function.params.iter()
                    .map(|p| format!("&{}", p.name))
                    .collect();
                code.push_str(&format!(
                    "                    self.write(&{}Stdlib::generate_{}_code({}));\n",
                    config.pascal_case_name(),
                    function.name.to_lowercase().replace("-", "_"),
                    params.join(", ")
                ));
            } else {
                code.push_str(&format!(
                    "                    self.write(&{}Stdlib::generate_{}_code());\n",
                    config.pascal_case_name(),
                    function.name.to_lowercase().replace("-", "_")
                ));
            }
            
            if required_params > 0 {
                code.push_str("                } else {\n");
                code.push_str(&format!(
                    "                    self.write(&format!(\"// Error: {} requires at least {} arguments\", args.len()));\n",
                    function.name, required_params
                ));
                code.push_str("                }\n");
            }
            
            code.push_str("            }\n");
        }
        
        code.push_str(&format!(
            "            _ => self.write(&format!(\"// Unknown {} method: {{}}\", method)),\n",
            config.snake_case_name()
        ));
        code.push_str("        }\n");
        code.push_str("    }\n");
        
        code
    }
}
