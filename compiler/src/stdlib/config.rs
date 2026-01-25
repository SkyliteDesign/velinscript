pub struct ConfigStdlib;

impl ConfigStdlib {
    pub fn generate_get_env_code(key: &str) -> String {
        format!(
            "{{
                std::env::var({}).map_err(|_| format!(\"Environment variable '{{}}' not found\", {}))
            }}",
            key, key
        )
    }

    pub fn generate_get_or_default_code(key: &str, default: &str) -> String {
        format!(
            "std::env::var({}).unwrap_or_else(|_| {}.to_string())",
            key, default
        )
    }

    pub fn generate_load_dotenv_code() -> String {
        "dotenv::dotenv().ok()".to_string()
    }
}
