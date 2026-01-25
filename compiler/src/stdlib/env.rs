pub struct EnvStdlib;

impl EnvStdlib {
    pub fn generate_load_code(path: &str) -> String {
        format!(
            "{{
                use std::fs;
                use std::io::BufRead;
                let env_path = {};
                if let Ok(file) = fs::File::open(&env_path) {{
                    let reader = std::io::BufReader::new(file);
                    for line in reader.lines() {{
                        if let Ok(line_str) = line {{
                            if let Some((key, value)) = line_str.split_once('=') {{
                                let key = key.trim();
                                let value = value.trim().trim_matches('\"');
                                std::env::set_var(key, value);
                            }}
                        }}
                    }}
                }}
                Ok(())
            }}",
            path
        )
    }

    pub fn generate_get_code(key: &str, default: Option<&str>) -> String {
        if let Some(def) = default {
            format!(
                "std::env::var({}).unwrap_or_else(|_| {}.to_string())",
                key, def
            )
        } else {
            format!("std::env::var({}).map_err(|e| e.to_string())", key)
        }
    }

    pub fn generate_get_number_code(key: &str, default: &str) -> String {
        format!(
            "std::env::var({}).ok().and_then(|v| v.parse::<i64>().ok()).unwrap_or({})",
            key, default
        )
    }

    pub fn generate_get_bool_code(key: &str, default: &str) -> String {
        format!(
            "std::env::var({}).ok().and_then(|v| match v.to_lowercase().as_str() {{
                \"true\" | \"1\" | \"yes\" | \"on\" => Some(true),
                \"false\" | \"0\" | \"no\" | \"off\" => Some(false),
                _ => None
            }}).unwrap_or({})",
            key, default
        )
    }

    pub fn generate_set_code(key: &str, value: &str) -> String {
        format!("std::env::set_var({}, {})", key, value)
    }

    pub fn generate_validate_code(schema: &str) -> String {
        format!(
            "{{
                let schema: serde_json::Value = {};
                let mut errors = Vec::new();
                if let Some(obj) = schema.as_object() {{
                    for (key, rule) in obj {{
                        if let Some(required) = rule.get(\"required\").and_then(|v| v.as_bool()) {{
                            if required && std::env::var(key).is_err() {{
                                errors.push(format!(\"Missing required environment variable: {{}}\", key));
                            }}
                        }}
                        if let Some(env_type) = rule.get(\"type\").and_then(|v| v.as_str()) {{
                            if let Ok(value) = std::env::var(key) {{
                                match env_type {{
                                    \"number\" => {{
                                        if value.parse::<i64>().is_err() {{
                                            errors.push(format!(\"Environment variable {{}} must be a number\", key));
                                        }}
                                    }},
                                    \"boolean\" => {{
                                        if !matches!(value.to_lowercase().as_str(), \"true\" | \"false\" | \"1\" | \"0\" | \"yes\" | \"no\" | \"on\" | \"off\") {{
                                            errors.push(format!(\"Environment variable {{}} must be a boolean\", key));
                                        }}
                                    }},
                                    _ => {{}}
                                }}
                            }}
                        }}
                    }}
                }}
                if errors.is_empty() {{
                    Ok(())
                }} else {{
                    Err(errors.join(\", \"))
                }}
            }}",
            schema
        )
    }

    pub fn generate_get_secret_code(key: &str, vault: &str) -> String {
        format!(
            "{{
                use std::fs;
                use std::path::PathBuf;
                let vault_path = PathBuf::from({});
                let secret_path = vault_path.join(format!(\"{{}}.secret\", {}));
                fs::read_to_string(&secret_path).map_err(|e| e.to_string())
            }}",
            vault, key
        )
    }
}
