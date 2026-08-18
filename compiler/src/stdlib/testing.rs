// Standard Library für Testing-Funktionalität
// Testing Decorators und Assert-Funktionen

use crate::parser::ast::Decorator;

/// Testing Standard Library
pub struct TestingStdlib;

impl TestingStdlib {
    pub fn is_test_decorator(decorator: &Decorator) -> bool {
        matches!(
            decorator.name.as_str(),
            "test" | "describe" | "fixture" | "mock"
        )
    }

    pub fn is_describe_decorator(decorator: &Decorator) -> bool {
        decorator.name == "describe"
    }

    pub fn is_fixture_decorator(decorator: &Decorator) -> bool {
        decorator.name == "fixture"
    }

    pub fn is_mock_decorator(decorator: &Decorator) -> bool {
        decorator.name == "mock"
    }

    pub fn generate_test_attribute() -> String {
        "#[test]".to_string()
    }

    pub fn generate_describe_attribute(suite_name: Option<&str>) -> String {
        if let Some(name) = suite_name {
            format!("// Test Suite: {}", name)
        } else {
            "// Test Suite".to_string()
        }
    }

    pub fn generate_fixture_function(name: &str) -> String {
        format!(
            "    fn setup_{name}() {{\n        // Fixture setup code\n    }}\n    fn teardown_{name}() {{\n        // Fixture teardown code\n    }}\n"
        )
    }

    pub fn generate_mock_trait_impl(trait_name: &str, struct_name: &str) -> String {
        format!(
            "\n    struct Mock{struct_name} {{\n        // Mock state\n    }}\n\n    impl {trait_name} for Mock{struct_name} {{\n        // Mock implementations\n    }}\n"
        )
    }

    pub fn generate_assert_code(condition: &str) -> String {
        format!("assert!({})", condition)
    }

    pub fn generate_assert_eq_code(left: &str, right: &str) -> String {
        format!("assert_eq!({}, {})", left, right)
    }

    pub fn generate_assert_ne_code(left: &str, right: &str) -> String {
        format!("assert_ne!({}, {})", left, right)
    }

    pub fn generate_test_module_setup() -> String {
        "#[cfg(test)]\nmod tests {\n    use super::*;\n".to_string()
    }

    pub fn generate_mock_framework_setup() -> String {
        "\n    // Mock framework setup\n    // Note: Requires mockall crate in Cargo.toml\n    // use mockall::*;\n"
            .to_string()
    }
}
