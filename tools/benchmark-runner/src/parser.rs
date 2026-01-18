// Benchmark Parser
// Parst @benchmark Annotationen

use velin_compiler::parser::parser::Parser;
use velin_compiler::parser::ast::{Item, Function};
use anyhow::{Result, anyhow};
use std::path::Path;

use crate::runner::Benchmark;

pub struct BenchmarkParser;

impl BenchmarkParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse_benchmarks(&self, content: &str, file: &Path) -> Result<Vec<Benchmark>> {
        let program = Parser::parse(content).map_err(|e| anyhow!(e.message))?;
        let mut benchmarks = Vec::new();
        
        for item in &program.items {
            if let Item::Function(func) = item {
                // Prüfe auf @benchmark Decorator
                let has_benchmark = func.decorators.iter().any(|d| d.name == "benchmark");
                
                if has_benchmark {
                    let line = self.find_line_number(content, &func.name);
                    
                    // Generiere Code für Benchmark
                    let code = format!("fn {}() {{\n{}\n}}", func.name, 
                        self.extract_function_body(content, &func.name));
                    
                    benchmarks.push(Benchmark {
                        name: func.name.clone(),
                        code,
                        file: file.to_string_lossy().to_string(),
                        line,
                    });
                }
            }
        }
        
        Ok(benchmarks)
    }
    
    fn find_line_number(&self, content: &str, name: &str) -> usize {
        for (line_num, line) in content.lines().enumerate() {
            if line.contains(&format!("fn {}", name)) {
                return line_num + 1;
            }
        }
        0
    }
    
    fn extract_function_body(&self, content: &str, name: &str) -> String {
        // Vereinfachte Extraktion
        // In Produktion sollte man den AST nutzen
        let mut in_function = false;
        let mut body = String::new();
        let mut brace_count = 0;
        
        for line in content.lines() {
            if line.contains(&format!("fn {}", name)) {
                in_function = true;
            }
            
            if in_function {
                body.push_str(line);
                body.push('\n');
                
                brace_count += line.matches('{').count() as i32;
                brace_count -= line.matches('}').count() as i32;
                
                if brace_count == 0 && in_function {
                    break;
                }
            }
        }
        
        body
    }
}
