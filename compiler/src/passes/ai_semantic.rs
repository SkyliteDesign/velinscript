use crate::analysis::insight::InsightAnalyzer;
use crate::compiler::config::CompilerConfig;
use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::parser::ast::*;
use crate::prompt::sanitizer::PromptSanitizer;
use crate::stdlib::ml::{LLMClient, LLMProvider};
use anyhow::Result;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

/// KI-basierter Semantic Analysis Pass
///
/// Analysiert Code-Semantik mit Hilfe von LLM und VelinInsight:
/// - Erkennt Kontext (API, Service, Library)
/// - Identifiziert Abhängigkeiten
/// - Analysiert Sicherheitsanforderungen
/// - Speichert Metadaten im CompilationContext
pub struct AISemanticPass {
    llm_client: Option<LLMClient>,
    insight_analyzer: InsightAnalyzer,
    prompt_sanitizer: PromptSanitizer,
    enabled: bool,
    response_cache: Arc<Mutex<HashMap<u64, SemanticAnalysis>>>, // Cache für deterministische Ergebnisse
}

impl AISemanticPass {
    pub fn new(config: &CompilerConfig) -> Result<Self> {
        let llm_client = if config.enable_ai_semantic {
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
            insight_analyzer: InsightAnalyzer::new(),
            prompt_sanitizer: PromptSanitizer::new(),
            enabled: config.enable_ai_semantic,
            response_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Analysiert Code mit KI (mit Caching für deterministische Ergebnisse)
    fn analyze_code_with_ai(&self, program: &Program) -> Result<SemanticAnalysis> {
        // Erstelle Hash für deterministisches Caching
        let code_summary = self.extract_code_summary(program);
        let mut hasher = DefaultHasher::new();
        code_summary.hash(&mut hasher);
        let cache_key = hasher.finish();

        // Prüfe Cache
        {
            let cache = self.response_cache.lock().unwrap();
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        if let Some(ref client) = self.llm_client {
            // Sanitize Code-Kontext
            let sanitized_code = self.prompt_sanitizer.sanitize_code_context(&code_summary);

            let prompt = format!(
                "Analyze the following VelinScript code and provide a JSON response with:\n\
                1. context_type: one of 'api', 'service', 'library', 'application'\n\
                2. dependencies: list of required dependencies (e.g., 'database', 'auth', 'llm')\n\
                3. security_requirements: list of security concerns (e.g., 'authentication', 'rate_limiting', 'input_validation')\n\
                4. missing_components: list of potentially missing components\n\n\
                Code:\n{}\n\n\
                Respond with valid JSON only.",
                sanitized_code
            );

            // Sanitize Prompt vor dem Senden
            let sanitized_prompt = self.prompt_sanitizer.sanitize(&prompt);

            if !self.prompt_sanitizer.is_safe(&sanitized_prompt) {
                // Prompt enthält gefährliche Patterns, nutze Fallback
                let result = self.heuristic_analysis(program);
                {
                    let mut cache = self.response_cache.lock().unwrap();
                    cache.insert(cache_key, result.clone());
                }
                return Ok(result);
            }

            match client.generate(&sanitized_prompt) {
                Ok(response) => {
                    let result = self.parse_ai_response(&response)?;
                    {
                        let mut cache = self.response_cache.lock().unwrap();
                        cache.insert(cache_key, result.clone());
                    }
                    Ok(result)
                }
                Err(_e) => {
                    // Fallback zu heuristischer Analyse wenn KI fehlschlägt
                    let result = self.heuristic_analysis(program);
                    {
                        let mut cache = self.response_cache.lock().unwrap();
                        cache.insert(cache_key, result.clone());
                    }
                    Ok(result)
                }
            }
        } else {
            // Kein LLMClient verfügbar, nutze heuristische Analyse
            let result = self.heuristic_analysis(program);
            {
                let mut cache = self.response_cache.lock().unwrap();
                cache.insert(cache_key, result.clone());
            }
            Ok(result)
        }
    }

    /// Extrahiert Code-Zusammenfassung für KI-Analyse
    fn extract_code_summary(&self, program: &Program) -> String {
        let mut summary = String::new();

        for item in &program.items {
            match item {
                Item::Function(f) => {
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

                    // Decorators analysieren
                    for decorator in &f.decorators {
                        summary.push_str(&format!("  @{}\n", decorator.name));
                    }
                }
                Item::Struct(s) => {
                    summary.push_str(&format!("struct {} {{\n", s.name));
                    for field in &s.fields {
                        summary.push_str(&format!("  {}: {:?}\n", field.name, field.field_type));
                    }
                    summary.push_str("}\n");
                }
                _ => {}
            }
        }

        summary
    }

    /// Parst KI-Antwort in SemanticAnalysis
    fn parse_ai_response(&self, response: &str) -> Result<SemanticAnalysis> {
        // Versuche JSON zu parsen
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str::<serde_json::Value>(cleaned) {
            Ok(json) => Ok(SemanticAnalysis {
                context_type: json["context_type"].as_str().map(|s| s.to_string()),
                dependencies: json["dependencies"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                security_requirements: json["security_requirements"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                missing_components: json["missing_components"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
            }),
            Err(_) => {
                // JSON-Parsing fehlgeschlagen, nutze heuristische Analyse
                Err(anyhow::anyhow!("Failed to parse AI response as JSON"))
            }
        }
    }

    /// Heuristische Code-Analyse ohne KI
    fn heuristic_analysis(&self, program: &Program) -> SemanticAnalysis {
        let mut context_type = None;
        let mut dependencies = Vec::new();
        let mut security_requirements = Vec::new();
        let missing_components = Vec::new();

        // Analysiere Decorators und Funktionsnamen
        for item in &program.items {
            if let Item::Function(f) = item {
                // Prüfe auf HTTP-Decorators
                for decorator in &f.decorators {
                    match decorator.name.as_str() {
                        "@GET" | "@POST" | "@PUT" | "@DELETE" | "@PATCH" => {
                            context_type = Some("api".to_string());
                            dependencies.push("http".to_string());
                        }
                        "@Auth" => {
                            security_requirements.push("authentication".to_string());
                            dependencies.push("auth".to_string());
                        }
                        "@RateLimit" => {
                            security_requirements.push("rate_limiting".to_string());
                            dependencies.push("rate_limit".to_string());
                        }
                        _ => {}
                    }
                }

                // Prüfe Funktionsnamen auf Patterns
                if f.name.contains("api") || f.name.contains("endpoint") {
                    context_type = Some("api".to_string());
                }
                if f.name.contains("service") {
                    context_type = Some("service".to_string());
                }
                if f.name.contains("db") || f.name.contains("database") {
                    dependencies.push("database".to_string());
                }
                if f.name.contains("llm") || f.name.contains("chat") || f.name.contains("ai") {
                    dependencies.push("llm".to_string());
                }
            }
        }

        // Entferne Duplikate
        dependencies.sort();
        dependencies.dedup();
        security_requirements.sort();
        security_requirements.dedup();

        SemanticAnalysis {
            context_type,
            dependencies,
            security_requirements,
            missing_components,
        }
    }

    /// Erkennt Kontext aus AST und Insight
    fn extract_context(
        &self,
        program: &Program,
        insight: &crate::analysis::insight::InsightReport,
    ) -> String {
        // Nutze Insight-Daten und AST-Analyse
        if !insight.complex_functions.is_empty() {
            "application".to_string()
        } else if program.items.iter().any(|item| {
            if let Item::Function(f) = item {
                f.decorators
                    .iter()
                    .any(|d| matches!(d.name.as_str(), "@GET" | "@POST" | "@PUT" | "@DELETE"))
            } else {
                false
            }
        }) {
            "api".to_string()
        } else {
            "library".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SemanticAnalysis {
    context_type: Option<String>,
    dependencies: Vec<String>,
    security_requirements: Vec<String>,
    missing_components: Vec<String>,
}

impl Pass for AISemanticPass {
    fn name(&self) -> &str {
        "AISemantic"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(ref program) = context.program {
            // 1. Bestehende Insight-Analyse nutzen
            let insight = self.insight_analyzer.analyze(program);

            // 2. KI-Analyse mit LLMClient
            let semantic_analysis = self.analyze_code_with_ai(program)?;

            // 3. Kontext-Erkennung (erweitert bestehende Analyse)
            let detected_context = self.extract_context(program, &insight);
            let context_type = semantic_analysis.context_type.unwrap_or(detected_context);

            // 4. Metadaten zum Context hinzufügen
            context.semantic_metadata.context_type = Some(context_type);
            context.semantic_metadata.dependencies = semantic_analysis.dependencies;
            context.semantic_metadata.security_requirements =
                semantic_analysis.security_requirements;
            context.semantic_metadata.missing_components = semantic_analysis.missing_components;
        }

        Ok(())
    }
}
