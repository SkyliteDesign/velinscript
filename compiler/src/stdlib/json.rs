// Standard Library für JSON-Funktionalität
// JSON-Parsing und -Serialisierung

use serde_json::Value;

/// JSON Standard Library
pub struct JsonStdlib;

impl JsonStdlib {
    /// Parst einen JSON-String zu einem Value
    pub fn parse(json_string: &str) -> Result<Value, String> {
        serde_json::from_str(json_string)
            .map_err(|e| format!("Fehler beim Parsen von JSON: {}", e))
    }
    
    /// Serialisiert einen Value zu einem JSON-String
    pub fn stringify(value: &Value) -> String {
        serde_json::to_string(value)
            .unwrap_or_else(|_| "{}".to_string())
    }
    
    /// Serialisiert einen Value zu einem formatierten JSON-String
    pub fn stringify_pretty(value: &Value) -> String {
        serde_json::to_string_pretty(value)
            .unwrap_or_else(|_| "{}".to_string())
    }
    
    /// Holt einen Wert aus einem JSON-Objekt
    pub fn get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
        value.get(key)
    }
    
    /// Setzt einen Wert in einem JSON-Objekt
    pub fn set(value: &mut Value, key: &str, new_value: Value) {
        if let Some(obj) = value.as_object_mut() {
            obj.insert(key.to_string(), new_value);
        }
    }
    
    /// Prüft ob ein JSON-Objekt einen Key enthält
    pub fn has_key(value: &Value, key: &str) -> bool {
        value.get(key).is_some()
    }
    
    /// Gibt alle Keys eines JSON-Objekts zurück
    pub fn keys(value: &Value) -> Vec<String> {
        if let Some(obj) = value.as_object() {
            obj.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }
    
    /// Gibt die Anzahl der Elemente zurück (für Objekte und Arrays)
    pub fn length(value: &Value) -> usize {
        match value {
            Value::Object(obj) => obj.len(),
            Value::Array(arr) => arr.len(),
            _ => 0,
        }
    }
    
    /// Generiert Rust-Code für json.parse()
    pub fn generate_parse_code(json_string: &str) -> String {
        format!("json::parse(\"{}\")", json_string)
    }
    
    /// Generiert Rust-Code für json.stringify()
    pub fn generate_stringify_code(value: &str) -> String {
        format!("json::stringify({})", value)
    }
    
    /// Generiert Rust-Code für json.get()
    pub fn generate_get_code(value: &str, key: &str) -> String {
        format!("json::get(&{}, \"{}\")", value, key)
    }
    
    /// Liste der verfügbaren JSON-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "json.parse".to_string(),
                signature: "fn(string) -> Result<Value, string>".to_string(),
            },
            FunctionInfo {
                name: "json.stringify".to_string(),
                signature: "fn(Value) -> string".to_string(),
            },
            FunctionInfo {
                name: "json.stringifyPretty".to_string(),
                signature: "fn(Value) -> string".to_string(),
            },
            FunctionInfo {
                name: "json.get".to_string(),
                signature: "fn(Value, string) -> Option<Value>".to_string(),
            },
            FunctionInfo {
                name: "json.set".to_string(),
                signature: "fn(Value, string, Value) -> ()".to_string(),
            },
            FunctionInfo {
                name: "json.hasKey".to_string(),
                signature: "fn(Value, string) -> bool".to_string(),
            },
            FunctionInfo {
                name: "json.keys".to_string(),
                signature: "fn(Value) -> Vec<string>".to_string(),
            },
            FunctionInfo {
                name: "json.length".to_string(),
                signature: "fn(Value) -> usize".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
