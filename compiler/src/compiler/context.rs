use crate::compiler::language::VELISCH_LANGUAGE_NAME;
use crate::error::CompilerError;
use crate::parser::ast::Program;
use std::collections::HashMap;

/// Metadaten für KI-basierte Code-Analyse
#[derive(Debug, Clone, Default)]
pub struct SemanticMetadata {
    pub context_type: Option<String>, // "api", "service", "library", etc.
    pub dependencies: Vec<String>,
    pub security_requirements: Vec<String>,
    pub missing_components: Vec<String>,
}

/// Fehlerstatistiken für CompilationContext
#[derive(Debug, Default, Clone)]
pub struct ErrorStatistics {
    pub parse_errors: usize,
    pub type_errors: usize,
    pub codegen_errors: usize,
    pub io_errors: usize,
    pub validation_errors: usize,
    pub config_errors: usize,
    pub internal_errors: usize,
    pub warnings: usize,
    pub info: usize,
}

#[derive(Debug)]
pub struct CompilationContext {
    pub source_map: HashMap<String, String>, // filename -> source
    pub program: Option<Program>,
    pub errors: Vec<CompilerError>,
    pub warnings: Vec<CompilerError>, // Warnings werden separat gesammelt
    pub root_file: String,
    /// KI-basierte semantische Metadaten
    pub semantic_metadata: SemanticMetadata,
}

impl CompilationContext {
    pub fn new(root_file: String, source: String) -> Self {
        // Velisch Identity - Fingerabdruck im Context
        let _velisch_check = VELISCH_LANGUAGE_NAME;

        let mut source_map = HashMap::new();
        source_map.insert(root_file.clone(), source);

        Self {
            source_map,
            program: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            root_file,
            semantic_metadata: SemanticMetadata::default(),
        }
    }

    pub fn add_source(&mut self, filename: String, source: String) {
        self.source_map.insert(filename, source);
    }

    /// Fügt einen Fehler zum Context hinzu
    pub fn add_error(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    /// Fügt eine Warnung hinzu
    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(CompilerError::Warning(message));
    }

    /// Fügt eine Info-Meldung hinzu
    pub fn add_info(&mut self, message: String) {
        self.warnings.push(CompilerError::Info(message));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// Gibt Fehlerstatistiken zurück
    pub fn get_error_statistics(&self) -> ErrorStatistics {
        let mut stats = ErrorStatistics::default();

        for error in &self.errors {
            match error {
                CompilerError::Parse { .. } => stats.parse_errors += 1,
                CompilerError::Type { .. } => stats.type_errors += 1,
                CompilerError::CodeGen { .. } => stats.codegen_errors += 1,
                CompilerError::Io { .. } => stats.io_errors += 1,
                CompilerError::Validation { .. } => stats.validation_errors += 1,
                CompilerError::Config { .. } => stats.config_errors += 1,
                CompilerError::Internal { .. } => stats.internal_errors += 1,
                CompilerError::Warning(_) => stats.warnings += 1,
                CompilerError::Info(_) => stats.info += 1,
            }
        }

        // Zähle auch Warnings aus warnings-Vector
        for warning in &self.warnings {
            match warning {
                CompilerError::Warning(_) => stats.warnings += 1,
                CompilerError::Info(_) => stats.info += 1,
                _ => {}
            }
        }

        stats
    }

    /// Gibt alle Fehler mit Vorschlägen zurück
    pub fn get_errors_with_suggestions(&self) -> Vec<String> {
        self.errors.iter().map(|e| e.with_suggestions()).collect()
    }

    /// Exportiert Fehler als JSON
    pub fn export_errors_json(&self) -> Result<String, serde_json::Error> {
        use serde_json::json;

        let errors_json = json!({
            "errors": self.errors.iter().map(|e| {
                json!({
                    "type": format!("{:?}", e),
                    "message": e.to_string(),
                    "suggestion": e.with_suggestions()
                })
            }).collect::<Vec<_>>(),
            "warnings": self.warnings.iter().map(|e| {
                json!({
                    "type": format!("{:?}", e),
                    "message": e.to_string()
                })
            }).collect::<Vec<_>>(),
            "statistics": {
                "parse_errors": self.get_error_statistics().parse_errors,
                "type_errors": self.get_error_statistics().type_errors,
                "codegen_errors": self.get_error_statistics().codegen_errors,
                "io_errors": self.get_error_statistics().io_errors,
                "validation_errors": self.get_error_statistics().validation_errors,
                "config_errors": self.get_error_statistics().config_errors,
                "internal_errors": self.get_error_statistics().internal_errors,
                "warnings": self.get_error_statistics().warnings,
                "info": self.get_error_statistics().info,
            }
        });

        serde_json::to_string_pretty(&errors_json)
    }

    /// Exportiert Fehler als HTML-Report
    pub fn export_errors_html(&self) -> String {
        let stats = self.get_error_statistics();

        let mut html = String::from(
            r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VelinScript Compiler - Fehlerreport</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }
        h2 { color: #555; margin-top: 30px; }
        .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin: 20px 0; }
        .stat-card { background: #f9f9f9; padding: 15px; border-radius: 5px; border-left: 4px solid #4CAF50; }
        .stat-card.error { border-left-color: #f44336; }
        .stat-card.warning { border-left-color: #ff9800; }
        .stat-label { font-size: 12px; color: #666; text-transform: uppercase; }
        .stat-value { font-size: 24px; font-weight: bold; color: #333; }
        .error-item { background: #fff3f3; border-left: 4px solid #f44336; padding: 15px; margin: 10px 0; border-radius: 4px; }
        .warning-item { background: #fff8e1; border-left: 4px solid #ff9800; padding: 15px; margin: 10px 0; border-radius: 4px; }
        .error-type { font-weight: bold; color: #d32f2f; }
        .error-message { margin: 10px 0; color: #333; }
        .error-suggestion { background: #e8f5e9; padding: 10px; border-radius: 4px; margin-top: 10px; font-family: monospace; white-space: pre-wrap; }
        pre { background: #f5f5f5; padding: 10px; border-radius: 4px; overflow-x: auto; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔍 VelinScript Compiler - Fehlerreport</h1>
"#,
        );

        // Statistiken
        html.push_str("<h2>📊 Statistiken</h2>");
        html.push_str("<div class=\"stats\">");

        html.push_str(&format!(r#"<div class="stat-card error"><div class="stat-label">Parse Fehler</div><div class="stat-value">{}</div></div>"#, stats.parse_errors));
        html.push_str(&format!(r#"<div class="stat-card error"><div class="stat-label">Type Fehler</div><div class="stat-value">{}</div></div>"#, stats.type_errors));
        html.push_str(&format!(r#"<div class="stat-card error"><div class="stat-label">CodeGen Fehler</div><div class="stat-value">{}</div></div>"#, stats.codegen_errors));
        html.push_str(&format!(r#"<div class="stat-card error"><div class="stat-label">IO Fehler</div><div class="stat-value">{}</div></div>"#, stats.io_errors));
        html.push_str(&format!(r#"<div class="stat-card error"><div class="stat-label">Validation Fehler</div><div class="stat-value">{}</div></div>"#, stats.validation_errors));
        html.push_str(&format!(r#"<div class="stat-card warning"><div class="stat-label">Warnungen</div><div class="stat-value">{}</div></div>"#, stats.warnings));

        html.push_str("</div>");

        // Fehler
        if !self.errors.is_empty() {
            html.push_str("<h2>❌ Fehler</h2>");
            for error in &self.errors {
                html.push_str("<div class=\"error-item\">");
                html.push_str(&format!("<div class=\"error-type\">{:?}</div>", error));
                html.push_str(&format!(
                    "<div class=\"error-message\">{}</div>",
                    error.to_string()
                ));
                html.push_str(&format!(
                    "<div class=\"error-suggestion\">{}</div>",
                    error.with_suggestions()
                ));
                html.push_str("</div>");
            }
        }

        // Warnungen
        if !self.warnings.is_empty() {
            html.push_str("<h2>⚠️ Warnungen</h2>");
            for warning in &self.warnings {
                html.push_str("<div class=\"warning-item\">");
                html.push_str(&format!(
                    "<div class=\"error-message\">{}</div>",
                    warning.to_string()
                ));
                html.push_str("</div>");
            }
        }

        html.push_str(
            r#"
    </div>
</body>
</html>"#,
        );

        html
    }

    /// Filtert Fehler nach Typ
    pub fn filter_errors(&self, filter: ErrorFilter) -> Vec<&CompilerError> {
        self.errors
            .iter()
            .filter(|e| match filter {
                ErrorFilter::All => true,
                ErrorFilter::Parse => matches!(e, CompilerError::Parse { .. }),
                ErrorFilter::Type => matches!(e, CompilerError::Type { .. }),
                ErrorFilter::CodeGen => matches!(e, CompilerError::CodeGen { .. }),
                ErrorFilter::Io => matches!(e, CompilerError::Io { .. }),
                ErrorFilter::Validation => matches!(e, CompilerError::Validation { .. }),
                ErrorFilter::Config => matches!(e, CompilerError::Config { .. }),
                ErrorFilter::Internal => matches!(e, CompilerError::Internal { .. }),
                ErrorFilter::Warnings => matches!(e, CompilerError::Warning(_)),
                ErrorFilter::Critical => matches!(
                    e,
                    CompilerError::Parse { .. }
                        | CompilerError::Type { .. }
                        | CompilerError::CodeGen { .. }
                ),
            })
            .collect()
    }
}

/// Fehlerfilter für CompilationContext
#[derive(Debug, Clone, Copy)]
pub enum ErrorFilter {
    All,
    Parse,
    Type,
    CodeGen,
    Io,
    Validation,
    Config,
    Internal,
    Warnings,
    Critical,
}
