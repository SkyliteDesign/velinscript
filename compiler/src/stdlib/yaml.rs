pub struct YamlStdlib;

impl YamlStdlib {
    pub fn generate_parse_code(yaml_string: &str) -> String {
        format!(
            "serde_yaml::from_str::<serde_json::Value>({}).map_err(|e| e.to_string())",
            yaml_string
        )
    }

    pub fn generate_parse_file_code(path: &str) -> String {
        format!(
            "std::fs::read_to_string({}).map_err(|e| e.to_string()).and_then(|s| serde_yaml::from_str::<serde_json::Value>(&s).map_err(|e| e.to_string()))",
            path
        )
    }

    pub fn generate_stringify_code(value: &str) -> String {
        format!(
            "serde_yaml::to_string(&{}).map_err(|e| e.to_string())",
            value
        )
    }

    pub fn generate_write_file_code(path: &str, value: &str) -> String {
        format!(
            "serde_yaml::to_string(&{}).map_err(|e| e.to_string()).and_then(|s| std::fs::write({}, s).map_err(|e| e.to_string()))",
            value, path
        )
    }

    pub fn generate_validate_code(path: &str, schema: &str) -> String {
        format!(
            "{{
                let yaml_data = std::fs::read_to_string({}).map_err(|e| e.to_string())?;
                let parsed: serde_json::Value = serde_yaml::from_str(&yaml_data).map_err(|e| e.to_string())?;
                let schema: serde_json::Value = {};
                // Simple validation - check if required fields exist
                if let Some(required) = schema.get(\"required\").and_then(|v| v.as_array()) {{
                    for field in required {{
                        if let Some(field_name) = field.as_str() {{
                            if !parsed.get(field_name).is_some() {{
                                return Err(format!(\"Missing required field: {{}}\", field_name));
                            }}
                        }}
                    }}
                }}
                Ok(true)
            }}",
            path, schema
        )
    }
}
