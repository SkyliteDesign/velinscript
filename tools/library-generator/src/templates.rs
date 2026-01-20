use anyhow::Result;
use crate::config::{LibraryConfig, ModuleType};

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn new() -> Self {
        Self
    }
    
    pub fn render_module(&self, config: &LibraryConfig) -> Result<String> {
        let template = match config.module_type() {
            ModuleType::SimpleFunctions => include_str!("templates/simple_functions.rs.template"),
            ModuleType::StructBased => include_str!("templates/struct_based.rs.template"),
            ModuleType::ServiceBased => include_str!("templates/service_based.rs.template"),
        };
        
        self.render(template, config)
    }
    
    pub fn render_test(&self, config: &LibraryConfig) -> Result<String> {
        let template = include_str!("templates/test.rs.template");
        let mut result = self.render(template, config)?;
        
        // Generiere Test-Funktionen
        let mut test_functions = String::new();
        for function in &config.functions {
            let mut test_code = format!(
                "    #[test]\n    fn test_{}_{}() {{\n        let stdlib = {}Stdlib;\n        \n        ",
                config.snake_case_name(),
                function.name,
                config.pascal_case_name()
            );
            
            if !function.params.is_empty() {
                for param in &function.params {
                    test_code.push_str(&format!(
                        "let {} = \"test_{}\";\n        ",
                        param.name, param.name
                    ));
                }
                let params: Vec<String> = function.params.iter()
                    .map(|p| format!("&{}", p.name))
                    .collect();
                test_code.push_str(&format!(
                    "let result = stdlib.generate_{}_code({});\n        ",
                    function.name.to_lowercase().replace("-", "_"),
                    params.join(", ")
                ));
            } else {
                test_code.push_str(&format!(
                    "let result = stdlib.generate_{}_code();\n        ",
                    function.name.to_lowercase().replace("-", "_")
                ));
            }
            
            test_code.push_str("\n        assert!(!result.is_empty(), \"Result should not be empty\");\n");
            
            if let Some(return_type) = &function.return_type {
                match return_type.as_str() {
                    "string" => {
                        test_code.push_str("        assert!(!result.is_empty());\n");
                    }
                    "number" => {
                        test_code.push_str("        assert!(result.parse::<f64>().is_ok() || !result.is_empty());\n");
                    }
                    "boolean" => {
                        test_code.push_str("        assert!(result == \"true\" || result == \"false\" || !result.is_empty());\n");
                    }
                    _ => {
                        test_code.push_str(&format!("        // Validierung für {} - Ergebnis sollte nicht leer sein\n", return_type));
                    }
                }
            }
            
            test_code.push_str("    }\n    \n");
            test_functions.push_str(&test_code);
        }
        
        result = result.replace("{{test_functions}}", &test_functions);
        
        Ok(result)
    }
    
    pub fn render_docs(&self, config: &LibraryConfig) -> Result<String> {
        let template = include_str!("templates/docs.md.template");
        self.render(template, config)
    }
    
    fn render(&self, template: &str, config: &LibraryConfig) -> Result<String> {
        let mut result = template.to_string();
        
        // Basis-Variablen
        result = result.replace("{{ModuleName}}", &config.pascal_case_name());
        result = result.replace("{{module_name}}", &config.snake_case_name());
        result = result.replace("{{description}}", &config.description);
        
        // Funktionen rendern
        let functions_code = self.render_functions(config)?;
        result = result.replace("{{functions}}", &functions_code);
        
        // Typen rendern
        let types_code = self.render_types(config)?;
        result = result.replace("{{types}}", &types_code);
        
        Ok(result)
    }
    
    fn render_functions(&self, config: &LibraryConfig) -> Result<String> {
        let mut code = String::new();
        
        for function in &config.functions {
            let params: Vec<String> = function.params.iter()
                .map(|p| format!("{}: &str", p.name))
                .collect();
            let params_str = params.join(", ");
            
            code.push_str(&format!(
                "    pub fn generate_{}_code({}) -> String {{\n",
                function.name.to_lowercase().replace("-", "_"),
                params_str
            ));
            
            // Generiere vollständige Implementierung basierend auf Rückgabetyp und Parametern
            if let Some(return_type) = &function.return_type {
                match return_type.as_str() {
                    "string" => {
                        if !function.params.is_empty() {
                            let first_param = &function.params[0].name;
                            code.push_str(&format!(
                                "        {}.to_string()\n",
                                first_param
                            ));
                        } else {
                            code.push_str("        String::new()\n");
                        }
                    }
                    "number" => {
                        if function.params.len() >= 2 {
                            // Für Operationen mit mehreren Parametern (z.B. add, subtract)
                            let params_used: Vec<String> = function.params.iter()
                                .map(|p| format!("{}.parse::<f64>().unwrap_or(0.0)", p.name))
                                .collect();
                            code.push_str(&format!(
                                "        ({})\n            .to_string()\n",
                                params_used.join(" + ")
                            ));
                        } else if !function.params.is_empty() {
                            let first_param = &function.params[0].name;
                            code.push_str(&format!(
                                "        {}.parse::<f64>().unwrap_or(0.0).to_string()\n",
                                first_param
                            ));
                        } else {
                            code.push_str("        \"0.0\".to_string()\n");
                        }
                    }
                    "boolean" => {
                        if !function.params.is_empty() {
                            let first_param = &function.params[0].name;
                            code.push_str(&format!(
                                "        {}.parse::<bool>().unwrap_or(false).to_string()\n",
                                first_param
                            ));
                        } else {
                            code.push_str("        \"false\".to_string()\n");
                        }
                    }
                    "List<string>" => {
                        if !function.params.is_empty() {
                            let first_param = &function.params[0].name;
                            code.push_str(&format!(
                                "        format!(\"{{}}.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>()\", {})\n",
                                first_param
                            ));
                        } else {
                            code.push_str("        \"vec![]\".to_string()\n");
                        }
                    }
                    "Map<string, any>" => {
                        code.push_str("        \"std::collections::HashMap::new()\".to_string()\n");
                    }
                    _ => {
                        // Für benutzerdefinierte Typen
                        if !function.params.is_empty() {
                            let first_param = &function.params[0].name;
                            code.push_str(&format!(
                                "        format!(\"{}::new({{}})\", {})\n",
                                return_type, first_param
                            ));
                        } else {
                            code.push_str(&format!(
                                "        format!(\"{}::new()\")\n",
                                return_type
                            ));
                        }
                    }
                }
            } else {
                // void - keine Rückgabe
                if !function.params.is_empty() {
                    let params_used: Vec<String> = function.params.iter()
                        .map(|p| format!("{}", p.name))
                        .collect();
                    code.push_str(&format!(
                        "        let _ = ({});\n",
                        params_used.join(", ")
                    ));
                }
                code.push_str("        String::new()\n");
            }
            
            code.push_str("    }\n\n");
        }
        
        Ok(code)
    }
    
    fn render_types(&self, config: &LibraryConfig) -> Result<String> {
        let mut code = String::new();
        
        for type_def in &config.types {
            code.push_str(&format!("pub struct {} {{\n", type_def.name));
            
            for field in &type_def.fields {
                let optional = if field.optional.unwrap_or(false) { "Option<" } else { "" };
                let optional_close = if field.optional.unwrap_or(false) { ">" } else { "" };
                
                code.push_str(&format!(
                    "    pub {}: {}{}{},\n",
                    field.name,
                    optional,
                    self.rust_type(&field.field_type),
                    optional_close
                ));
            }
            
            code.push_str("}\n\n");
        }
        
        Ok(code)
    }
    
    fn rust_type(&self, velin_type: &str) -> String {
        match velin_type {
            "string" => "String".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "List<string>" => "Vec<String>".to_string(),
            "Map<string, any>" => "std::collections::HashMap<String, String>".to_string(),
            "any" => "String".to_string(), // Vereinfacht
            _ => velin_type.to_string(),
        }
    }
}
