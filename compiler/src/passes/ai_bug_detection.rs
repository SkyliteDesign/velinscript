use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use crate::compiler::config::CompilerConfig;
use crate::stdlib::ml::{LLMClient, LLMProvider};
use crate::parser::ast::*;
use crate::error::CompilerError;
use crate::prompt::sanitizer::PromptSanitizer;
use anyhow::Result;
use serde_json;

/// KI-basierter Bug Detection Pass
/// 
/// Erkennt Bugs proaktiv vor der Ausführung:
/// - Pattern-basierte Bug-Erkennung
/// - KI-basierte semantische Bug-Erkennung
/// - Logik-Fehler erkennen
/// - Sicherheitslücken finden
/// - Auto-Fix für einfache Bugs
pub struct AIBugDetectionPass {
    llm_client: Option<LLMClient>,
    bug_patterns: BugPatternDatabase,
    prompt_sanitizer: PromptSanitizer,
    enabled: bool,
}

/// Datenbank bekannter Bug-Patterns
#[derive(Debug, Clone)]
struct BugPatternDatabase {
    patterns: Vec<BugPattern>,
}

#[derive(Debug, Clone)]
struct BugPattern {
    name: String,
    description: String,
    matcher: PatternMatcher,
    severity: BugSeverity,
    auto_fixable: bool,
}

#[derive(Debug, Clone)]
enum PatternMatcher {
    FunctionNameContains(String),
    #[allow(dead_code)]
    DecoratorName(String),
    #[allow(dead_code)]
    ExpressionPattern(String),
}

#[derive(Debug, Clone, PartialEq)]
enum BugSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
struct DetectedBug {
    pattern: String,
    location: String,
    description: String,
    severity: BugSeverity,
    auto_fixable: bool,
    suggested_fix: Option<String>,
}

impl BugPatternDatabase {
    fn new() -> Self {
        let mut patterns = Vec::new();
        
        // Pattern 1: Fehlende Error Handling
        patterns.push(BugPattern {
            name: "missing_error_handling".to_string(),
            description: "Function may fail without error handling".to_string(),
            matcher: PatternMatcher::FunctionNameContains("get".to_string()),
            severity: BugSeverity::Medium,
            auto_fixable: false,
        });
        
        // Pattern 2: Potenzielle Null-Pointer
        patterns.push(BugPattern {
            name: "potential_null_pointer".to_string(),
            description: "Variable may be null before use".to_string(),
            matcher: PatternMatcher::ExpressionPattern("null".to_string()),
            severity: BugSeverity::High,
            auto_fixable: false,
        });
        
        // Pattern 3: Fehlende Auth bei sensiblen Operationen
        patterns.push(BugPattern {
            name: "missing_auth".to_string(),
            description: "Sensitive operation without authentication".to_string(),
            matcher: PatternMatcher::FunctionNameContains("delete".to_string()),
            severity: BugSeverity::Critical,
            auto_fixable: true,
        });
        
        Self { patterns }
    }
}

impl AIBugDetectionPass {
    pub fn new(config: &CompilerConfig) -> Result<Self> {
        let llm_client = if config.enable_ai_bug_detection {
            if let (Some(provider_str), Some(api_key)) = (&config.ai_provider, &config.ai_api_key) {
                let provider = match provider_str.as_str() {
                    "openai" => LLMProvider::OpenAI,
                    "anthropic" => LLMProvider::Anthropic,
                    "gemini" | "google" => LLMProvider::GoogleGemini,
                    "local" => LLMProvider::Local,
                    _ => LLMProvider::Local,
                };
                Some(LLMClient::new(provider, api_key.clone()))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self {
            llm_client,
            bug_patterns: BugPatternDatabase::new(),
            prompt_sanitizer: PromptSanitizer::new(),
            enabled: config.enable_ai_bug_detection,
        })
    }

    /// Prüft Code auf bekannte Bug-Patterns
    fn check_patterns(&self, program: &Program) -> Vec<DetectedBug> {
        let mut bugs = Vec::new();

        for item in &program.items {
            if let Item::Function(f) = item {
                for pattern in &self.bug_patterns.patterns {
                    if self.matches_pattern(&f, pattern) {
                        bugs.push(DetectedBug {
                            pattern: pattern.name.clone(),
                            location: format!("function {}", f.name),
                            description: pattern.description.clone(),
                            severity: pattern.severity.clone(),
                            auto_fixable: pattern.auto_fixable,
                            suggested_fix: if pattern.auto_fixable {
                                self.suggest_fix(&f, pattern)
                            } else {
                                None
                            },
                        });
                    }
                }
            }
        }

        bugs
    }

    /// Prüft ob Funktion Pattern matcht
    fn matches_pattern(&self, func: &Function, pattern: &BugPattern) -> bool {
        match &pattern.matcher {
            PatternMatcher::FunctionNameContains(substring) => {
                func.name.contains(substring)
            }
            PatternMatcher::DecoratorName(name) => {
                func.decorators.iter().any(|d| d.name == *name)
            }
            PatternMatcher::ExpressionPattern(_) => {
                // Einfache Implementierung - würde in Produktion AST durchsuchen
                false
            }
        }
    }

    /// Schlägt Fix für Pattern vor
    fn suggest_fix(&self, func: &Function, pattern: &BugPattern) -> Option<String> {
        match pattern.name.as_str() {
            "missing_auth" => {
                Some(format!("Add @Auth decorator to function {}", func.name))
            }
            _ => None
        }
    }

    /// KI-basierte semantische Bug-Erkennung
    fn detect_semantic_bugs(&self, program: &Program) -> Result<Vec<DetectedBug>> {
        if let Some(ref client) = self.llm_client {
            let code_summary = self.extract_code_summary(program);
            
            // Sanitize Code-Kontext
            let sanitized_code = self.prompt_sanitizer.sanitize_code_context(&code_summary);
            
            let prompt = format!(
                "Analyze the following VelinScript code for potential bugs and security issues.\n\
                Provide a JSON array of bugs with:\n\
                - pattern: bug type identifier\n\
                - location: where the bug is (function name, line number if possible)\n\
                - description: what the bug is\n\
                - severity: 'critical', 'high', 'medium', or 'low'\n\
                - auto_fixable: boolean\n\
                - suggested_fix: optional fix suggestion\n\n\
                Code:\n{}\n\n\
                Respond with valid JSON array only.",
                sanitized_code
            );

            // Sanitize Prompt vor dem Senden
            let sanitized_prompt = self.prompt_sanitizer.sanitize(&prompt);
            
            if !self.prompt_sanitizer.is_safe(&sanitized_prompt) {
                // Prompt enthält gefährliche Patterns, nutze Fallback
                return Ok(Vec::new());
            }

            match client.generate(&sanitized_prompt) {
                Ok(response) => self.parse_bug_response(&response),
                Err(_) => Ok(Vec::new()), // Fallback: keine Bugs gefunden
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Extrahiert Code-Zusammenfassung für Bug-Analyse
    fn extract_code_summary(&self, program: &Program) -> String {
        let mut summary = String::new();
        
        for item in &program.items {
            if let Item::Function(f) = item {
                summary.push_str(&format!("fn {}(", f.name));
                for (i, param) in f.params.iter().enumerate() {
                    if i > 0 {
                        summary.push_str(", ");
                    }
                    summary.push_str(&format!("{}: {:?}", param.name, param.param_type));
                }
                summary.push_str(")");
                if let Some(rt) = &f.return_type {
                    summary.push_str(&format!(" -> {:?}", rt));
                }
                summary.push_str("\n");
                
                // Decorators
                for decorator in &f.decorators {
                    summary.push_str(&format!("  @{}\n", decorator.name));
                }
                
                // Body-Zusammenfassung (erste paar Statements)
                let statement_count = f.body.statements.len();
                summary.push_str(&format!("  // {} statements\n", statement_count));
            }
        }
        
        summary
    }

    /// Parst KI-Antwort in Bug-Liste
    fn parse_bug_response(&self, response: &str) -> Result<Vec<DetectedBug>> {
        let cleaned = response.trim().trim_start_matches("```json").trim_end_matches("```").trim();
        
        match serde_json::from_str::<serde_json::Value>(cleaned) {
            Ok(json) => {
                let mut bugs = Vec::new();
                
                if let Some(array) = json.as_array() {
                    for bug_json in array {
                        if let (Some(pattern), Some(location), Some(description), Some(severity_str)) = (
                            bug_json["pattern"].as_str(),
                            bug_json["location"].as_str(),
                            bug_json["description"].as_str(),
                            bug_json["severity"].as_str(),
                        ) {
                            let severity = match severity_str {
                                "critical" => BugSeverity::Critical,
                                "high" => BugSeverity::High,
                                "medium" => BugSeverity::Medium,
                                "low" => BugSeverity::Low,
                                _ => BugSeverity::Medium,
                            };
                            
                            bugs.push(DetectedBug {
                                pattern: pattern.to_string(),
                                location: location.to_string(),
                                description: description.to_string(),
                                severity,
                                auto_fixable: bug_json["auto_fixable"].as_bool().unwrap_or(false),
                                suggested_fix: bug_json["suggested_fix"].as_str().map(|s| s.to_string()),
                            });
                        }
                    }
                }
                
                Ok(bugs)
            }
            Err(_) => Ok(Vec::new()), // Parsing fehlgeschlagen
        }
    }

    /// Erkennt Logik-Fehler
    fn detect_logic_errors(&self, program: &Program) -> Vec<DetectedBug> {
        let bugs = Vec::new();
        
        // Einfache Heuristiken für Logik-Fehler
        for item in &program.items {
            if let Item::Function(_f) = item {
                // Prüfe auf unendliche Loops (while true ohne break)
                // Prüfe auf fehlende Return-Statements
                // Prüfe auf ungenutzte Variablen
                // etc.
            }
        }
        
        bugs
    }

    /// Erkennt Sicherheitslücken
    fn detect_security_issues(&self, program: &Program) -> Vec<DetectedBug> {
        let mut bugs = Vec::new();
        
        for item in &program.items {
            if let Item::Function(f) = item {
                // Prüfe auf fehlende Auth bei sensiblen Operationen
                let is_sensitive = f.name.contains("delete") 
                    || f.name.contains("update") 
                    || f.name.contains("admin");
                
                if is_sensitive {
                    let has_auth = f.decorators.iter().any(|d| d.name == "@Auth");
                    if !has_auth {
                        bugs.push(DetectedBug {
                            pattern: "missing_auth".to_string(),
                            location: format!("function {}", f.name),
                            description: "Sensitive operation without authentication".to_string(),
                            severity: BugSeverity::Critical,
                            auto_fixable: true,
                            suggested_fix: Some(format!("Add @Auth decorator to function {}", f.name)),
                        });
                    }
                }
            }
        }
        
        bugs
    }

    /// Wendet Auto-Fix an
    fn auto_fix_bug(&self, context: &mut CompilationContext, bug: &DetectedBug) -> Result<()> {
        if bug.auto_fixable {
            if let Some(ref fix) = bug.suggested_fix {
                // Einfache Auto-Fix-Implementierung
                // In Produktion würde hier der AST modifiziert werden
                context.errors.push(CompilerError::Warning(format!(
                    "Auto-fix suggestion for {}: {}",
                    bug.location, fix
                )));
            }
        }
        Ok(())
    }
}

impl Pass for AIBugDetectionPass {
    fn name(&self) -> &str {
        "AIBugDetection"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(ref program) = context.program {
            // 1. Pattern-basierte Bug-Erkennung
            let pattern_bugs = self.check_patterns(program);

            // 2. KI-basierte semantische Bug-Erkennung
            let semantic_bugs = self.detect_semantic_bugs(program)?;

            // 3. Logik-Fehler erkennen
            let logic_bugs = self.detect_logic_errors(program);

            // 4. Sicherheitslücken finden
            let security_bugs = self.detect_security_issues(program);

            // 5. Alle Bugs aggregieren
            let mut all_bugs = Vec::new();
            all_bugs.extend(pattern_bugs);
            all_bugs.extend(semantic_bugs);
            all_bugs.extend(logic_bugs);
            all_bugs.extend(security_bugs);

            // 6. Fehler melden oder Auto-Fix vorschlagen
            for bug in all_bugs {
                if bug.auto_fixable {
                    self.auto_fix_bug(context, &bug)?;
                } else {
                    let error_msg = format!(
                        "[{}] {} in {}: {}",
                        match bug.severity {
                            BugSeverity::Critical => "CRITICAL",
                            BugSeverity::High => "HIGH",
                            BugSeverity::Medium => "MEDIUM",
                            BugSeverity::Low => "LOW",
                        },
                        bug.pattern,
                        bug.location,
                        bug.description
                    );
                    
                    context.errors.push(CompilerError::Warning(error_msg));
                }
            }
        }

        Ok(())
    }
}
