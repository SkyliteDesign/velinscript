// Standard Library für Testing-Funktionalität
// Testing Decorators und Assert-Funktionen

use crate::parser::ast::Decorator;

/// Testing Standard Library
pub struct TestingStdlib;

impl TestingStdlib {
    /// Prüft ob ein Decorator ein Test-Decorator ist
    pub fn is_test_decorator(decorator: &Decorator) -> bool {
        matches!(
            decorator.name.as_str(),
            "test" | "describe" | "fixture" | "mock"
        )
    }

    /// Prüft ob ein Decorator ein @describe Decorator ist
    pub fn is_describe_decorator(decorator: &Decorator) -> bool {
        decorator.name == "describe"
    }

    /// Prüft ob ein Decorator ein @fixture Decorator ist
    pub fn is_fixture_decorator(decorator: &Decorator) -> bool {
        decorator.name == "fixture"
    }

    /// Prüft ob ein Decorator ein @mock Decorator ist
    pub fn is_mock_decorator(decorator: &Decorator) -> bool {
        decorator.name == "mock"
    }

    /// Generiert Rust-Code für @test Decorator
    pub fn generate_test_attribute() -> String {
        "#[test]".to_string()
    }

    /// Generiert Rust-Code für @describe Decorator (Test Suite)
    pub fn generate_describe_attribute(suite_name: Option<&str>) -> String {
        if let Some(name) = suite_name {
            format!("// Test Suite: {}", name)
        } else {
            "// Test Suite".to_string()
        }
    }

    /// Generiert Rust-Code für @fixture Decorator
    pub fn generate_fixture_function(name: &str) -> String {
        format!(
            r#"    fn setup_{}() {{
        // Fixture setup code
    }}
    
    fn teardown_{}() {{
        // Fixture teardown code
    }}
"#,
            name, name
        )
    }

    /// Generiert Rust-Code für @mock Decorator (Mock als Trait-Implementierung)
    pub fn generate_mock_trait_impl(trait_name: &str, struct_name: &str) -> String {
        format!(
            r#"
    struct Mock{} {{
        // Mock state
    }}
    
    impl {} for Mock{} {{
        // Mock implementations
    }}
"#,
            struct_name, trait_name, struct_name
        )
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
"#
        .to_string()
    }

    /// Generiert Mock-Framework Setup (mockall)
    pub fn generate_mock_framework_setup() -> String {
        r#"
    // Mock framework setup
    // Note: Requires mockall crate in Cargo.toml
    // use mockall::*;
"#
        .to_string()
    }
}
