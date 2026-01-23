/// Error Suggestions - Bietet hilfreiche Vorschläge für Fehler
/// 
/// Dieses Modul implementiert "Did you mean?" Vorschläge und andere hilfreiche Fehler-Hinweise.

use crate::error::CompilerError;
use std::collections::HashSet;

/// Error Suggestion Engine
pub struct ErrorSuggestionEngine {
    common_functions: HashSet<String>,
    common_types: HashSet<String>,
    common_keywords: HashSet<String>,
}

impl ErrorSuggestionEngine {
    /// Erstellt einen neuen Error Suggestion Engine
    pub fn new() -> Self {
        let common_functions = HashSet::from([
            "add".to_string(),
            "subtract".to_string(),
            "multiply".to_string(),
            "divide".to_string(),
            "length".to_string(),
            "contains".to_string(),
            "trim".to_string(),
            "to_lowercase".to_string(),
            "to_uppercase".to_string(),
            "get".to_string(),
            "set".to_string(),
            "find".to_string(),
            "filter".to_string(),
            "map".to_string(),
            "reduce".to_string(),
        ]);

        let common_types = HashSet::from([
            "string".to_string(),
            "number".to_string(),
            "boolean".to_string(),
            "void".to_string(),
            "List".to_string(),
            "Map".to_string(),
            "Result".to_string(),
            "Option".to_string(),
        ]);

        let common_keywords = HashSet::from([
            "fn".to_string(),
            "struct".to_string(),
            "enum".to_string(),
            "let".to_string(),
            "mut".to_string(),
            "return".to_string(),
            "if".to_string(),
            "else".to_string(),
            "for".to_string(),
            "while".to_string(),
            "match".to_string(),
            "async".to_string(),
            "await".to_string(),
        ]);

        ErrorSuggestionEngine {
            common_functions,
            common_types,
            common_keywords,
        }
    }

    /// Verbessert eine Fehlermeldung mit Vorschlägen
    pub fn enhance_error(&self, error: &CompilerError) -> String {
        let base_message = error.to_string();
        
        match error {
            CompilerError::Parse { expected, found, message, location, .. } => {
                let mut enhanced = format!("❌ {}\n", base_message);
                
                // Zeige Datei und Position
                if let Some(file) = &location.file {
                    enhanced.push_str(&format!("📁 Datei: {}\n", file));
                }
                enhanced.push_str(&format!("📍 Position: Zeile {}, Spalte {}\n\n", location.line, location.column));
                
                if let (Some(exp), Some(fnd)) = (expected, found) {
                    // Prüfe auf Tippfehler
                    if let Some(suggestion) = self.suggest_correction(fnd, &self.common_keywords) {
                        enhanced.push_str(&format!("💡 Did you mean: '{}'?\n", suggestion));
                    }
                    
                    // Prüfe auf häufige Fehler
                    if exp.contains("function") && fnd.contains("fn") {
                        enhanced.push_str("💡 Tip: Function declarations use 'fn', not 'function'\n");
                        enhanced.push_str("   Beispiel: fn myFunction(): string { return \"hello\"; }\n");
                    }
                    if exp.contains("struct") && fnd.contains("class") {
                        enhanced.push_str("💡 Tip: VelinScript uses 'struct', not 'class'\n");
                        enhanced.push_str("   Beispiel: struct User { id: string, name: string }\n");
                    }
                    if exp.contains("string") && fnd.contains("str") {
                        enhanced.push_str("💡 Tip: Use 'string' type, not 'str'\n");
                        enhanced.push_str("   Beispiel: let name: string = \"John\";\n");
                    }
                }
                
                enhanced.push_str(&format!("\n📖 Erwartet: {}\n", expected.as_deref().unwrap_or("unbekannt")));
                enhanced.push_str(&format!("🔍 Gefunden: {}\n", found.as_deref().unwrap_or("unbekannt")));
                
                // Füge Lösungsvorschläge hinzu
                enhanced.push_str("\n🔧 Lösungsvorschläge:\n");
                if message.contains("unexpected token") {
                    enhanced.push_str("   - Prüfe auf fehlende oder überflüssige Klammern\n");
                    enhanced.push_str("   - Nutze 'velin check --autofix' für automatische Korrekturen\n");
                }
                if message.contains("expected") && message.contains("found") {
                    enhanced.push_str("   - Prüfe die Syntax in der Dokumentation\n");
                    enhanced.push_str("   - Siehe: docs/language/specification.md\n");
                }
                
                enhanced
            }
            CompilerError::Type { message, kind, location, .. } => {
                let mut enhanced = format!("❌ {}\n", base_message);
                
                // Zeige Datei und Position
                if let Some(file) = &location.file {
                    enhanced.push_str(&format!("📁 Datei: {}\n", file));
                }
                enhanced.push_str(&format!("📍 Position: Zeile {}, Spalte {}\n\n", location.line, location.column));
                
                // Prüfe auf häufige Type-Fehler
                if message.contains("undefined") {
                    if let Some(name) = self.extract_name_from_message(message) {
                        if let Some(suggestion) = self.suggest_correction(&name, &self.common_functions) {
                            enhanced.push_str(&format!("💡 Did you mean: '{}'?\n", suggestion));
                        }
                        if let Some(suggestion) = self.suggest_correction(&name, &self.common_types) {
                            enhanced.push_str(&format!("💡 Did you mean type: '{}'?\n", suggestion));
                        }
                    }
                }
                
                if let Some(k) = kind {
                    enhanced.push_str(&format!("📋 Fehler-Typ: {}\n", k));
                }
                
                // Füge Beispiele hinzu
                if message.contains("type mismatch") {
                    enhanced.push_str("\n💡 Beispiel für explizite Typ-Annotation:\n");
                    enhanced.push_str("   let x: number = 42;\n");
                    enhanced.push_str("   let name: string = \"John\";\n");
                }
                
                // Prüfe auf undefined variable
                if message.contains("undefined variable") {
                    if let Some(name) = self.extract_name_from_message(message) {
                        if let Some(suggestion) = self.suggest_correction(&name, &self.common_functions) {
                            enhanced.push_str(&format!("💡 Did you mean: '{}'?\n", suggestion));
                        }
                    }
                    enhanced.push_str("\n💡 Tipp: Stelle sicher, dass die Variable vor der Verwendung deklariert wurde\n");
                    enhanced.push_str("   Beispiel:\n");
                    enhanced.push_str("   let x = 10;  // Deklaration\n");
                    enhanced.push_str("   let y = x + 5;  // Verwendung\n");
                }
                
                // Prüfe auf type mismatch
                if message.contains("expected") && message.contains("found") {
                    enhanced.push_str("\n🔧 Lösungsvorschläge:\n");
                    enhanced.push_str("   - Prüfe die Typen deiner Variablen\n");
                    enhanced.push_str("   - Nutze explizite Typ-Annotationen bei Unsicherheit\n");
                    enhanced.push_str("   - Siehe: docs/guides/tutorial-1-basics.md\n");
                }
                
                enhanced
            }
            _ => base_message,
        }
    }

    /// Schlägt eine Korrektur vor (Levenshtein-Distanz)
    fn suggest_correction(&self, input: &str, candidates: &HashSet<String>) -> Option<String> {
        let mut best_match: Option<(String, usize)> = None;
        let max_distance = 3; // Maximale Edit-Distanz

        for candidate in candidates {
            let distance = self.levenshtein_distance(input, candidate);
            if distance <= max_distance {
                if let Some((_, best_dist)) = &best_match {
                    if distance < *best_dist {
                        best_match = Some((candidate.clone(), distance));
                    }
                } else {
                    best_match = Some((candidate.clone(), distance));
                }
            }
        }

        best_match.map(|(suggestion, _)| suggestion)
    }

    /// Berechnet Levenshtein-Distanz
    fn levenshtein_distance(&self, a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();

        if a_len == 0 {
            return b_len;
        }
        if b_len == 0 {
            return a_len;
        }

        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

        for i in 0..=a_len {
            matrix[i][0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }

        for i in 1..=a_len {
            for j in 1..=b_len {
                let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[a_len][b_len]
    }

    /// Extrahiert Namen aus Fehlermeldung
    fn extract_name_from_message(&self, message: &str) -> Option<String> {
        // Vereinfachte Extraktion: Suche nach "undefined X" oder "X is undefined"
        if let Some(start) = message.find("undefined ") {
            let rest = &message[start + 10..];
            if let Some(end) = rest.find(' ') {
                return Some(rest[..end].to_string());
            }
            return Some(rest.to_string());
        }
        if let Some(start) = message.find(" is undefined") {
            if let Some(name_start) = message[..start].rfind(' ') {
                return Some(message[name_start + 1..start].to_string());
            }
        }
        None
    }
}

impl Default for ErrorSuggestionEngine {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorLocation;

    #[test]
    fn test_suggest_correction() {
        let engine = ErrorSuggestionEngine::new();
        let suggestion = engine.suggest_correction("lenght", &engine.common_functions);
        assert_eq!(suggestion, Some("length".to_string()));
    }

    #[test]
    fn test_enhance_parse_error() {
        let engine = ErrorSuggestionEngine::new();
        let error = CompilerError::Parse {
            message: "Unexpected token".to_string(),
            location: ErrorLocation::new(1, 1),
            expected: Some("function".to_string()),
            found: Some("fn".to_string()),
            line: 1,
            column: 1,
            source_context: None,
        };
        let enhanced = engine.enhance_error(&error);
        assert!(enhanced.contains("Did you mean"));
    }
}
