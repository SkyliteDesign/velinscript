// Test Parser
// Parst @test Annotationen aus VelinScript-Code

use velin_compiler::parser::parser::Parser;
use velin_compiler::parser::ast::{Item, Function};
use anyhow::{Result, anyhow};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Test {
    pub name: String,
    pub line: usize,
    pub function: Function,
    pub before: bool,
    pub after: bool,
}

pub struct TestParser;

impl TestParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse_tests(&self, content: &str, file: &Path) -> Result<Vec<Test>> {
        let program = Parser::parse(content).map_err(|e| anyhow!(e.message))?;
        let mut tests = Vec::new();
        
        for item in &program.items {
            if let Item::Function(func) = item {
                // Prüfe auf @test Decorator
                let has_test = func.decorators.iter().any(|d| d.name == "test");
                
                if has_test {
                    let has_before = func.decorators.iter().any(|d| d.name == "before");
                    
                    let has_after = func.decorators.iter().any(|d| d.name == "after");
                    
                    // Finde Zeilennummer (vereinfacht)
                    let line = self.find_line_number(content, &func.name);
                    
                    tests.push(Test {
                        name: func.name.clone(),
                        line,
                        function: func.clone(),
                        before: has_before,
                        after: has_after,
                    });
                }
            }
        }
        
        Ok(tests)
    }
    
    fn find_line_number(&self, content: &str, name: &str) -> usize {
        for (line_num, line) in content.lines().enumerate() {
            if line.contains(&format!("fn {}", name)) {
                return line_num + 1;
            }
        }
        0
    }
}
