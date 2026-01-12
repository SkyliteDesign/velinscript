// Standard Library für Testing-Funktionalität
// Testing Decorators und Assert-Funktionen

use crate::parser::ast::Decorator;

/// Testing Standard Library
pub struct TestingStdlib;

impl TestingStdlib {
    /// Prüft ob ein Decorator ein Test-Decorator ist
    pub fn is_test_decorator(decorator: &Decorator) -> bool {
        decorator.name == "test"
    }
    
    /// Generiert Rust-Code für @test Decorator
    pub fn generate_test_attribute() -> String {
        "#[test]".to_string()
    }
    
    /// Generiert Rust-Code für assert! Makro
    pub fn generate_assert_code(condition: &str) -> String {
        format!("assert!({})", condition)
    }
    
    /// Generiert Rust-Code für assert_eq! Makro
    pub fn generate_assert_eq_code(left: &str, right: &str) -> String {
        format!("assert_eq!({}, {})", left, right)
    }
    
    /// Generiert Rust-Code für assert_ne! Makro
    pub fn generate_assert_ne_code(left: &str, right: &str) -> String {
        format!("assert_ne!({}, {})", left, right)
    }
    
    /// Generiert Test-Module Setup
    pub fn generate_test_module_setup() -> String {
        r#"#[cfg(test)]
mod tests {
    use super::*;
"#.to_string()
    }
}
