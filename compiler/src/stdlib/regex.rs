// Standard Library für Regex-Funktionalität
// Reguläre Ausdrücke für Pattern-Matching

use regex::Regex;

/// Regex Standard Library
pub struct RegexStdlib;

impl RegexStdlib {
    /// Kompiliert einen Regex-Pattern
    pub fn compile(pattern: &str) -> Result<Regex, String> {
        Regex::new(pattern)
            .map_err(|e| format!("Fehler beim Kompilieren des Regex-Patterns: {}", e))
    }

    /// Prüft ob ein String einem Pattern entspricht
    pub fn is_match(pattern: &str, text: &str) -> Result<bool, String> {
        let regex = Self::compile(pattern)?;
        Ok(regex.is_match(text))
    }

    /// Findet die erste Übereinstimmung
    pub fn find(pattern: &str, text: &str) -> Result<Option<String>, String> {
        let regex = Self::compile(pattern)?;
        Ok(regex.find(text).map(|m| m.as_str().to_string()))
    }

    /// Findet alle Übereinstimmungen
    pub fn find_all(pattern: &str, text: &str) -> Result<Vec<String>, String> {
        let regex = Self::compile(pattern)?;
        Ok(regex
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect())
    }

    /// Ersetzt alle Übereinstimmungen
    pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> Result<String, String> {
        let regex = Self::compile(pattern)?;
        Ok(regex.replace_all(text, replacement).to_string())
    }

    /// Ersetzt die erste Übereinstimmung
    pub fn replace(pattern: &str, text: &str, replacement: &str) -> Result<String, String> {
        let regex = Self::compile(pattern)?;
        Ok(regex.replace(text, replacement).to_string())
    }

    /// Splittet einen String anhand eines Patterns
    pub fn split(pattern: &str, text: &str) -> Result<Vec<String>, String> {
        let regex = Self::compile(pattern)?;
        Ok(regex.split(text).map(|s| s.to_string()).collect())
    }

    /// Extrahiert Capture-Gruppen
    pub fn captures(pattern: &str, text: &str) -> Result<Option<Vec<String>>, String> {
        let regex = Self::compile(pattern)?;
        if let Some(captures) = regex.captures(text) {
            let groups: Vec<String> = captures
                .iter()
                .skip(1) // Skip the full match
                .filter_map(|m| m.map(|m| m.as_str().to_string()))
                .collect();
            Ok(Some(groups))
        } else {
            Ok(None)
        }
    }

    /// Generiert Rust-Code für regex.match()
    pub fn generate_match_code(pattern: &str, text: &str) -> String {
        format!("regex::is_match(\"{}\", {})", pattern, text)
    }

    /// Generiert Rust-Code für regex.find()
    pub fn generate_find_code(pattern: &str, text: &str) -> String {
        format!("regex::find(\"{}\", {})", pattern, text)
    }

    /// Generiert Rust-Code für regex.replace()
    pub fn generate_replace_code(pattern: &str, text: &str, replacement: &str) -> String {
        format!(
            "regex::replace_all(\"{}\", {}, \"{}\")",
            pattern, text, replacement
        )
    }

    /// Liste der verfügbaren Regex-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "regex.compile".to_string(),
                signature: "fn(string) -> Result<Regex, string>".to_string(),
            },
            FunctionInfo {
                name: "regex.match".to_string(),
                signature: "fn(string, string) -> Result<bool, string>".to_string(),
            },
            FunctionInfo {
                name: "regex.find".to_string(),
                signature: "fn(string, string) -> Result<Option<string>, string>".to_string(),
            },
            FunctionInfo {
                name: "regex.findAll".to_string(),
                signature: "fn(string, string) -> Result<Vec<string>, string>".to_string(),
            },
            FunctionInfo {
                name: "regex.replace".to_string(),
                signature: "fn(string, string, string) -> Result<string, string>".to_string(),
            },
            FunctionInfo {
                name: "regex.replaceAll".to_string(),
                signature: "fn(string, string, string) -> Result<string, string>".to_string(),
            },
            FunctionInfo {
                name: "regex.split".to_string(),
                signature: "fn(string, string) -> Result<Vec<string>, string>".to_string(),
            },
            FunctionInfo {
                name: "regex.captures".to_string(),
                signature: "fn(string, string) -> Result<Option<Vec<string>>, string>".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
