// Ein Test-Modul für den Bibliotheks-Generator

pub struct TestExampleStdlib;

impl TestExampleStdlib {
    pub fn generate_hello_code(, name: &str) -> String {
        name.to_string()
    }

    pub fn generate_add_code(, a: &str, b: &str) -> String {
        a.parse::<f64>().unwrap_or(0.0).to_string()
    }


}
