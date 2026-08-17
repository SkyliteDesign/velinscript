// Standard Library für Serialization
// Serde-Integration für JSON, YAML, Binary

use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;

/// Serialization Standard Library
pub struct SerializationStdlib;

impl SerializationStdlib {
    /// Transformiert VelinScript toJSON() zu Rust-Code
    pub fn generate_to_json_code(value: &str) -> String {
        format!("serde_json::to_string(&{}).unwrap()", value)
    }

    /// Transformiert VelinScript fromJSON() zu Rust-Code
    pub fn generate_from_json_code<T: for<'de> Deserialize<'de>>(
        json: &str,
        type_name: &str,
    ) -> String {
        format!("serde_json::from_str::<{}>({}).unwrap()", type_name, json)
    }

    /// Transformiert VelinScript toYAML() zu Rust-Code
    pub fn generate_to_yaml_code(value: &str) -> String {
        format!("serde_yaml::to_string(&{}).unwrap()", value)
    }

    /// Transformiert VelinScript fromYAML() zu Rust-Code
    pub fn generate_from_yaml_code<T: for<'de> Deserialize<'de>>(
        yaml: &str,
        type_name: &str,
    ) -> String {
        format!("serde_yaml::from_str::<{}>({}).unwrap()", type_name, yaml)
    }

    /// Transformiert VelinScript serialize() zu Rust-Code (Binary)
    pub fn generate_serialize_code(value: &str) -> String {
        format!("bincode::serialize(&{}).unwrap()", value)
    }

    /// Transformiert VelinScript deserialize() zu Rust-Code (Binary)
    pub fn generate_deserialize_code<T: for<'de> Deserialize<'de>>(
        data: &str,
        type_name: &str,
    ) -> String {
        format!("bincode::deserialize::<{}>({}).unwrap()", type_name, data)
    }

    /// Generiert Serialize/Deserialize Derive-Macros für Structs
    pub fn generate_derive_macros() -> String {
        "#[derive(Serialize, Deserialize)]".to_string()
    }
}

/// Helper-Funktionen für Serialization
pub fn serialize_to_json<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| format!("JSON serialization error: {}", e))
}

pub fn deserialize_from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T, String> {
    serde_json::from_str(json).map_err(|e| format!("JSON deserialization error: {}", e))
}

pub fn serialize_to_yaml<T: Serialize>(value: &T) -> Result<String, String> {
    serde_yaml::to_string(value).map_err(|e| format!("YAML serialization error: {}", e))
}

pub fn deserialize_from_yaml<T: for<'de> Deserialize<'de>>(yaml: &str) -> Result<T, String> {
    serde_yaml::from_str(yaml).map_err(|e| format!("YAML deserialization error: {}", e))
}
