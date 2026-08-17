
use crate::parser::ast::*;

pub struct AutoTestGenerator;

impl AutoTestGenerator {
    pub fn new() -> Self {
        AutoTestGenerator
    }

    pub fn generate(&self, program: &Program) -> String {
        let mut tests = String::new();
        
        for item in &program.items {
            if let Item::Function(f) = item {
                if f.decorators.iter().any(|d| d.name == "VelinAutoTest") {
                    tests.push_str(&self.generate_test_for_function(f));
                    tests.push_str("\n\n");
                }
            }
        }
        
        if !tests.is_empty() {
            format!(
                "#[cfg(test)]\nmod autotests {{\n    use super::*;\n    use crate::*;\n\n{}\n}}",
                tests
            )
        } else {
            String::new()
        }
    }

    fn generate_test_for_function(&self, func: &Function) -> String {
        let mut test_code = String::new();
        let test_name = format!("test_auto_{}", func.name);
        
        test_code.push_str(&format!("    #[tokio::test]\n    async fn {}() {{\n", test_name));
        
        // Generate Mock Data for arguments
        let mut args_list = Vec::new();
        for param in &func.params {
            let mock_val = self.generate_mock_value(&param.param_type, &param.name);
            test_code.push_str(&format!("        let {} = {};\n", param.name, mock_val));
            args_list.push(param.name.clone());
        }
        
        // Call function
        let args_str = args_list.join(", ");
        let call = if func.is_async {
            format!("{}({}).await", func.name, args_str)
        } else {
            format!("{}({})", func.name, args_str)
        };
        
        test_code.push_str(&format!("        let result = {};\n", call));
        
        // Basic Assertions
        test_code.push_str("        assert!(result.is_ok(), \"Function execution failed\");\n");
        test_code.push_str("    }");
        
        test_code
    }

    fn generate_mock_value(&self, ty: &Type, name: &str) -> String {
        match ty {
            Type::String => format!("\"mock_{}\".to_string()", name),
            Type::Number => "42.0".to_string(),
            Type::Boolean => "true".to_string(),
            Type::List(_) => "vec![]".to_string(),
            Type::Named(n) => format!("{}::default()", n), // Assumes Default trait or custom default
            _ => "Default::default()".to_string(),
        }
    }
}
