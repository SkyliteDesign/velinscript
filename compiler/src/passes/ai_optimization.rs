use crate::compiler::config::CompilerConfig;
use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::optimizer::pipeline::PipelineOptimizer;
use crate::parser::ast::*;
use crate::prompt::sanitizer::PromptSanitizer;
use crate::stdlib::ml::{LLMClient, LLMProvider};
use anyhow::Result;
use serde_json;

/// KI-basierter Optimization Pass
///
/// Optimiert Code automatisch mit KI:
/// - Analysiert Optimierungs-Potenzial
/// - Nutzt Profiling-Daten (falls vorhanden)
/// - Wendet Performance-Optimierungen an
/// - Optimiert Memory-Usage
/// - Verbessert Code-Readability
pub struct AIOptimizationPass {
    llm_client: Option<LLMClient>,
    pipeline_optimizer: PipelineOptimizer,
    prompt_sanitizer: PromptSanitizer,
    enabled: bool,
    profiling_data: Option<ProfilingData>,
}

#[derive(Debug, Clone)]
struct ProfilingData {
    hot_paths: Vec<String>,
    bottlenecks: Vec<String>,
    memory_usage: u64,
    cpu_usage: f64,
}

#[derive(Debug, Clone)]
struct OptimizationOpportunity {
    optimization_type: OptimizationType,
    location: String,
    #[allow(dead_code)]
    description: String,
    #[allow(dead_code)]
    estimated_improvement: String,
}

#[derive(Debug, Clone)]
enum OptimizationType {
    Performance,
    Memory,
    Security,
    Readability,
}

impl AIOptimizationPass {
    pub fn new(config: &CompilerConfig) -> Result<Self> {
        let llm_client = if config.enable_ai_optimization {
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
            pipeline_optimizer: PipelineOptimizer::new(),
            prompt_sanitizer: PromptSanitizer::new(),
            enabled: config.enable_ai_optimization,
            profiling_data: None,
        })
    }

    /// Analysiert Code für Optimierungs-Potenzial
    fn analyze_optimization(&self, program: &Program) -> Result<Vec<OptimizationOpportunity>> {
        if let Some(ref client) = self.llm_client {
            let code_summary = self.extract_code_summary(program);
            let profiling_context = if let Some(ref profiling) = self.profiling_data {
                format!(
                    "\nProfiling Data:\n- Hot Paths: {:?}\n- Bottlenecks: {:?}\n- Memory Usage: {} bytes\n- CPU Usage: {:.2}%",
                    profiling.hot_paths, profiling.bottlenecks, profiling.memory_usage, profiling.cpu_usage
                )
            } else {
                String::new()
            };

            // Sanitize Code-Kontext
            let sanitized_code = self.prompt_sanitizer.sanitize_code_context(&code_summary);

            let prompt = format!(
                "Analyze the following VelinScript code for optimization opportunities.\n\
                Provide a JSON array of optimizations with:\n\
                - optimization_type: 'performance', 'memory', 'security', or 'readability'\n\
                - location: where to optimize (function name)\n\
                - description: what to optimize\n\
                - estimated_improvement: expected improvement\n\n\
                Code:\n{}{}\n\n\
                Respond with valid JSON array only.",
                sanitized_code, profiling_context
            );

            // Sanitize Prompt vor dem Senden
            let sanitized_prompt = self.prompt_sanitizer.sanitize(&prompt);

            if !self.prompt_sanitizer.is_safe(&sanitized_prompt) {
                // Prompt enthält gefährliche Patterns, nutze Fallback
                return Ok(self.heuristic_optimizations(program));
            }

            match client.generate(&sanitized_prompt) {
                Ok(response) => self.parse_optimization_response(&response),
                Err(_) => Ok(self.heuristic_optimizations(program)),
            }
        } else {
            Ok(self.heuristic_optimizations(program))
        }
    }

    /// Extrahiert Code-Zusammenfassung für Optimierungs-Analyse
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

                // Body-Komplexität
                let statement_count = f.body.statements.len();
                summary.push_str(&format!("  // {} statements\n", statement_count));

                // Decorators
                for decorator in &f.decorators {
                    summary.push_str(&format!("  @{}\n", decorator.name));
                }
            }
        }

        summary
    }

    /// Parst KI-Antwort in Optimierungs-Liste
    fn parse_optimization_response(&self, response: &str) -> Result<Vec<OptimizationOpportunity>> {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim();

        match serde_json::from_str::<serde_json::Value>(cleaned) {
            Ok(json) => {
                let mut opportunities = Vec::new();

                if let Some(array) = json.as_array() {
                    for opt_json in array {
                        if let (Some(opt_type_str), Some(location), Some(description)) = (
                            opt_json["optimization_type"].as_str(),
                            opt_json["location"].as_str(),
                            opt_json["description"].as_str(),
                        ) {
                            let opt_type = match opt_type_str {
                                "performance" => OptimizationType::Performance,
                                "memory" => OptimizationType::Memory,
                                "security" => OptimizationType::Security,
                                "readability" => OptimizationType::Readability,
                                _ => OptimizationType::Performance,
                            };

                            opportunities.push(OptimizationOpportunity {
                                optimization_type: opt_type,
                                location: location.to_string(),
                                description: description.to_string(),
                                estimated_improvement: opt_json["estimated_improvement"]
                                    .as_str()
                                    .unwrap_or("Unknown")
                                    .to_string(),
                            });
                        }
                    }
                }

                Ok(opportunities)
            }
            Err(_) => Ok(Vec::new()),
        }
    }

    /// Heuristische Optimierungen ohne KI
    fn heuristic_optimizations(&self, program: &Program) -> Vec<OptimizationOpportunity> {
        let mut opportunities = Vec::new();

        for item in &program.items {
            if let Item::Function(f) = item {
                // Prüfe auf komplexe Funktionen
                if f.body.statements.len() > 20 {
                    opportunities.push(OptimizationOpportunity {
                        optimization_type: OptimizationType::Readability,
                        location: format!("function {}", f.name),
                        description: "Function is too complex, consider refactoring".to_string(),
                        estimated_improvement: "Better maintainability".to_string(),
                    });
                }

                // Prüfe auf fehlende Parallelisierung
                if f.is_async && f.body.statements.len() > 5 {
                    opportunities.push(OptimizationOpportunity {
                        optimization_type: OptimizationType::Performance,
                        location: format!("function {}", f.name),
                        description: "Async function could benefit from parallelization"
                            .to_string(),
                        estimated_improvement: "Potential 2-4x speedup".to_string(),
                    });
                }
            }
        }

        opportunities
    }

    /// Identifiziert Hot Paths aus Profiling-Daten
    fn identify_hot_paths(
        &self,
        profiling: &ProfilingData,
    ) -> Result<Vec<OptimizationOpportunity>> {
        let mut opportunities = Vec::new();

        for hot_path in &profiling.hot_paths {
            opportunities.push(OptimizationOpportunity {
                optimization_type: OptimizationType::Performance,
                location: hot_path.clone(),
                description: "Hot path identified from profiling".to_string(),
                estimated_improvement: "Significant performance improvement expected".to_string(),
            });
        }

        Ok(opportunities)
    }

    /// Wendet Optimierung an
    fn apply_optimization(
        &self,
        context: &mut CompilationContext,
        opportunity: &OptimizationOpportunity,
    ) -> Result<()> {
        match opportunity.optimization_type {
            OptimizationType::Performance => {
                self.optimize_performance(context, opportunity)?;
            }
            OptimizationType::Memory => {
                self.optimize_memory(context, opportunity)?;
            }
            OptimizationType::Security => {
                self.optimize_security(context, opportunity)?;
            }
            OptimizationType::Readability => {
                self.optimize_readability(context, opportunity)?;
            }
        }
        Ok(())
    }

    /// Optimiert Performance
    fn optimize_performance(
        &self,
        context: &mut CompilationContext,
        opportunity: &OptimizationOpportunity,
    ) -> Result<()> {
        if let Some(ref mut program) = context.program {
            // Nutze PipelineOptimizer für Parallelisierung
            if opportunity.location.starts_with("function ") {
                let func_name = opportunity.location.strip_prefix("function ").unwrap_or("");

                // Finde Funktion im Program
                for item in &mut program.items {
                    if let crate::parser::ast::Item::Function(f) = item {
                        if f.name == func_name {
                            // Analysiere Block für Parallelisierungs-Möglichkeiten
                            let parallel_groups =
                                self.pipeline_optimizer.identify_parallel_groups(&f.body);

                            if !parallel_groups.is_empty() {
                                // Markiere Funktion für Parallelisierung
                                // In Produktion würde hier der Code transformiert werden
                                context.errors.push(crate::error::CompilerError::Info(
                                    format!("Performance optimization: Function {} can be parallelized ({} groups)", 
                                        func_name, parallel_groups.len())
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Optimiert Memory
    fn optimize_memory(
        &self,
        context: &mut CompilationContext,
        opportunity: &OptimizationOpportunity,
    ) -> Result<()> {
        if let Some(ref mut program) = context.program {
            // Memory-Optimierungen: Identifiziere große Datenstrukturen
            if opportunity.location.starts_with("function ") {
                let func_name = opportunity.location.strip_prefix("function ").unwrap_or("");

                for item in &mut program.items {
                    if let crate::parser::ast::Item::Function(f) = item {
                        if f.name == func_name {
                            // Prüfe auf große Listen/Map-Allokationen
                            let large_allocs = self.find_large_allocations(&f.body);

                            if !large_allocs.is_empty() {
                                context.errors.push(crate::error::CompilerError::Info(
                                    format!("Memory optimization: Function {} has {} large allocations that could be optimized", 
                                        func_name, large_allocs.len())
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Findet große Allokationen
    fn find_large_allocations(&self, block: &crate::parser::ast::Block) -> Vec<String> {
        let mut allocations = Vec::new();

        for stmt in &block.statements {
            if let crate::parser::ast::Statement::Let(let_stmt) = stmt {
                // Prüfe auf List/Map-Literale mit vielen Elementen
                if let crate::parser::ast::Expression::ListLiteral(items) = &let_stmt.value {
                    if items.len() > 100 {
                        allocations.push(format!("Large list allocation: {} items", items.len()));
                    }
                }
                if let crate::parser::ast::Expression::MapLiteral(fields) = &let_stmt.value {
                    if fields.len() > 100 {
                        allocations.push(format!("Large map allocation: {} fields", fields.len()));
                    }
                }
            }
        }

        allocations
    }

    /// Optimiert Security
    fn optimize_security(
        &self,
        context: &mut CompilationContext,
        opportunity: &OptimizationOpportunity,
    ) -> Result<()> {
        if let Some(ref mut program) = context.program {
            // Security-Optimierungen: Prüfe auf fehlende Validierung
            if opportunity.location.starts_with("function ") {
                let func_name = opportunity.location.strip_prefix("function ").unwrap_or("");

                for item in &mut program.items {
                    if let crate::parser::ast::Item::Function(f) = item {
                        if f.name == func_name {
                            // Prüfe auf fehlende Input-Validierung
                            let needs_validation = self.needs_input_validation(f);

                            if needs_validation {
                                context.errors.push(crate::error::CompilerError::Warning(
                                    format!("Security optimization: Function {} should validate input parameters", func_name)
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Prüft ob Funktion Input-Validierung braucht
    fn needs_input_validation(&self, func: &crate::parser::ast::Function) -> bool {
        // Prüfe ob Funktion Parameter hat aber keine Validierung
        if !func.params.is_empty() {
            // Prüfe ob Body Validierung enthält
            !self.has_validation_in_body(&func.body)
        } else {
            false
        }
    }

    /// Prüft ob Body Validierung enthält
    fn has_validation_in_body(&self, block: &crate::parser::ast::Block) -> bool {
        for stmt in &block.statements {
            if let crate::parser::ast::Statement::If(if_stmt) = stmt {
                // Prüfe ob Condition eine Validierung ist (z.B. != null, > 0, etc.)
                if self.is_validation_condition(&if_stmt.condition) {
                    return true;
                }
            }
        }
        false
    }

    /// Prüft ob Expression eine Validierung ist
    fn is_validation_condition(&self, expr: &crate::parser::ast::Expression) -> bool {
        match expr {
            crate::parser::ast::Expression::BinaryOp { op, .. } => {
                matches!(
                    op,
                    crate::parser::ast::BinaryOperator::NotEq
                        | crate::parser::ast::BinaryOperator::Eq
                        | crate::parser::ast::BinaryOperator::Gt
                        | crate::parser::ast::BinaryOperator::Lt
                )
            }
            _ => false,
        }
    }

    /// Optimiert Readability
    fn optimize_readability(
        &self,
        context: &mut CompilationContext,
        opportunity: &OptimizationOpportunity,
    ) -> Result<()> {
        if let Some(ref mut program) = context.program {
            // Readability-Optimierungen: Refactoring-Vorschläge
            if opportunity.location.starts_with("function ") {
                let func_name = opportunity.location.strip_prefix("function ").unwrap_or("");

                for item in &mut program.items {
                    if let crate::parser::ast::Item::Function(f) = item {
                        if f.name == func_name {
                            // Prüfe Komplexität
                            let complexity = self.calculate_complexity(&f.body);

                            if complexity > 20 {
                                context.errors.push(crate::error::CompilerError::Info(
                                    format!("Readability optimization: Function {} has high complexity ({}), consider refactoring", 
                                        func_name, complexity)
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Berechnet Code-Komplexität
    fn calculate_complexity(&self, block: &crate::parser::ast::Block) -> usize {
        let mut complexity = block.statements.len();

        for stmt in &block.statements {
            match stmt {
                crate::parser::ast::Statement::If(if_stmt) => {
                    complexity += 1; // +1 für if
                    complexity += self.calculate_complexity(&if_stmt.then_block);
                    if let Some(ref else_block) = if_stmt.else_block {
                        complexity += self.calculate_complexity(else_block);
                    }
                }
                crate::parser::ast::Statement::For(for_stmt) => {
                    complexity += 1; // +1 für loop
                    complexity += self.calculate_complexity(&for_stmt.body);
                }
                crate::parser::ast::Statement::While(while_stmt) => {
                    complexity += 1; // +1 für loop
                    complexity += self.calculate_complexity(&while_stmt.body);
                }
                crate::parser::ast::Statement::Match(match_stmt) => {
                    complexity += match_stmt.arms.len(); // +1 pro arm
                    for arm in &match_stmt.arms {
                        complexity += self.calculate_complexity(&arm.body);
                    }
                }
                _ => {}
            }
        }

        complexity
    }
}

impl Pass for AIOptimizationPass {
    fn name(&self) -> &str {
        "AIOptimization"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(ref program) = context.program {
            // 1. Code-Analyse für Optimierungs-Potenzial
            let mut optimization_opportunities = self.analyze_optimization(program)?;

            // 2. Profiling-Daten nutzen (falls vorhanden)
            if let Some(ref profiling) = self.profiling_data {
                let hot_path_opportunities = self.identify_hot_paths(profiling)?;
                optimization_opportunities.extend(hot_path_opportunities);
            }

            // 3. Optimierungen anwenden
            for opportunity in optimization_opportunities {
                if let Err(e) = self.apply_optimization(context, &opportunity) {
                    context
                        .errors
                        .push(crate::error::CompilerError::Warning(format!(
                            "Failed to apply optimization for {}: {}",
                            opportunity.location, e
                        )));
                }
            }
        }

        Ok(())
    }
}
