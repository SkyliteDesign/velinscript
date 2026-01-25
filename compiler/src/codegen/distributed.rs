use crate::codegen::infrastructure::{DeploymentPlan, DeploymentType};
use crate::parser::ast::*;
use anyhow::Result;

/// Deployment Analyzer für verteilte Systeme
///
/// Analysiert Code und generiert Deployment-Pläne:
/// - Analysiert Ressourcen-Anforderungen
/// - Evaluiert Deployment-Optionen
/// - Wählt beste Option
/// - Generiert Deployment-Plan
pub struct DeploymentAnalyzer {
    resource_analyzer: ResourceAnalyzer,
    cost_calculator: CostCalculator,
}

struct ResourceAnalyzer {
    // Analysiert CPU, Memory, Storage Anforderungen
}

struct CostCalculator {
    // Berechnet Kosten für verschiedene Deployment-Optionen
}

impl DeploymentAnalyzer {
    pub fn new() -> Self {
        Self {
            resource_analyzer: ResourceAnalyzer {},
            cost_calculator: CostCalculator {},
        }
    }

    /// Analysiert AST für Deployment-Anforderungen
    pub fn analyze(&self, program: &Program) -> Result<DeploymentPlan> {
        // 1. Analysiere Ressourcen-Anforderungen
        let resources = self.resource_analyzer.analyze(program)?;

        // 2. Evaluiere Deployment-Optionen
        let options = self.evaluate_deployment_options(&resources)?;

        // 3. Wähle beste Option
        let best_option = self.choose_best_option(&options)?;

        // 4. Generiere Deployment-Plan
        let plan = self.generate_deployment_plan(&best_option, &resources)?;

        Ok(plan)
    }

    /// Generiert Infrastructure-Code
    pub fn generate_infrastructure(&self, plan: &DeploymentPlan) -> Result<InfrastructureCode> {
        use crate::codegen::infrastructure::InfrastructureGenerator;

        let generator = InfrastructureGenerator::new();
        generator.generate(plan)
    }
}

impl ResourceAnalyzer {
    fn analyze(&self, program: &Program) -> Result<ResourceRequirements> {
        let mut requirements = ResourceRequirements::default();

        // Analysiere Code für Ressourcen-Anforderungen
        for item in &program.items {
            if let Item::Function(f) = item {
                // Schätze CPU-Anforderungen basierend auf Komplexität
                let complexity = self.estimate_complexity(f);
                requirements.cpu_cores = requirements.cpu_cores.max(complexity / 10);

                // Schätze Memory-Anforderungen
                let memory = self.estimate_memory(f);
                requirements.memory_mb = requirements.memory_mb.max(memory);

                // Prüfe auf Database-Calls
                if self.has_database_calls(f) {
                    requirements.needs_database = true;
                }

                // Prüfe auf Caching
                if self.has_caching_needs(f) {
                    requirements.needs_caching = true;
                }
            }
        }

        Ok(requirements)
    }

    fn estimate_complexity(&self, func: &Function) -> u32 {
        // Verbesserte Komplexitäts-Analyse mit cyclomatic complexity
        let base_complexity = self.analyze_block_complexity(&func.body);
        let cyclomatic = self.calculate_cyclomatic_complexity(&func.body);

        // Kombiniere beide Metriken
        base_complexity + cyclomatic
    }

    fn analyze_block_complexity(&self, block: &Block) -> u32 {
        let mut complexity = 0;
        for stmt in &block.statements {
            complexity += match stmt {
                Statement::If(if_stmt) => {
                    1 + self.analyze_block_complexity(&if_stmt.then_block)
                        + if_stmt
                            .else_block
                            .as_ref()
                            .map(|b| self.analyze_block_complexity(b))
                            .unwrap_or(0)
                }
                Statement::While(while_stmt) => {
                    5 + self.analyze_block_complexity(&while_stmt.body) // Loops sind teurer
                }
                Statement::For(for_stmt) => 5 + self.analyze_block_complexity(&for_stmt.body),
                Statement::Match(match_stmt) => {
                    // Match-Statements sind komplexer
                    2 + match_stmt.arms.len() as u32
                }
                Statement::Return(_) => 1,
                Statement::Expression(expr_stmt) => {
                    // Prüfe auf komplexe Expressions
                    self.estimate_expression_complexity(&expr_stmt.expression)
                }
                Statement::Let(let_stmt) => {
                    // Prüfe auf komplexe Zuweisungen
                    self.estimate_expression_complexity(&let_stmt.value)
                }
                _ => 1,
            };
        }
        complexity
    }

    fn calculate_cyclomatic_complexity(&self, block: &Block) -> u32 {
        let mut complexity = 1; // Basis-Komplexität
        for stmt in &block.statements {
            match stmt {
                Statement::If(_) => complexity += 1,
                Statement::While(_) => complexity += 1,
                Statement::For(_) => complexity += 1,
                Statement::Match(match_stmt) => complexity += match_stmt.arms.len() as u32,
                _ => {}
            }
        }
        complexity
    }

    fn estimate_expression_complexity(&self, expr: &crate::parser::ast::Expression) -> u32 {
        match expr {
            crate::parser::ast::Expression::Call { .. } => 2,
            crate::parser::ast::Expression::BinaryOp { left, right, .. } => {
                1 + self.estimate_expression_complexity(left)
                    + self.estimate_expression_complexity(right)
            }
            crate::parser::ast::Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                1 + self.estimate_expression_complexity(condition)
                    + self.estimate_expression_complexity(then_expr)
                    + self.estimate_expression_complexity(else_expr)
            }
            _ => 1,
        }
    }

    fn estimate_memory(&self, func: &Function) -> u64 {
        // Verbesserte Memory-Schätzung
        let vars = self.count_variables(&func.body);
        let complexity = self.estimate_complexity(func);

        // Basis: 10MB
        // + 1MB pro Variable
        // + 0.1MB pro Komplexitäts-Punkt (für Stack-Overhead)
        let base_mb = 10u64;
        let var_mb = vars as u64;
        let complexity_mb = (complexity as f64 * 0.1) as u64;

        base_mb + var_mb + complexity_mb
    }

    fn count_variables(&self, block: &Block) -> usize {
        let mut count = 0;
        for stmt in &block.statements {
            match stmt {
                Statement::Let(_) => count += 1,
                Statement::If(if_stmt) => {
                    count += self.count_variables(&if_stmt.then_block);
                    if let Some(else_block) = &if_stmt.else_block {
                        count += self.count_variables(else_block);
                    }
                }
                Statement::While(while_stmt) => count += self.count_variables(&while_stmt.body),
                Statement::For(for_stmt) => count += self.count_variables(&for_stmt.body),
                _ => {}
            }
        }
        count
    }

    fn has_database_calls(&self, func: &Function) -> bool {
        // Prüfe auf db.* Calls
        self.has_pattern(func, "db.")
    }

    fn has_caching_needs(&self, func: &Function) -> bool {
        // Prüfe auf häufige Reads
        self.has_pattern(func, "get") || self.has_pattern(func, "find")
    }

    fn has_pattern(&self, func: &Function, pattern: &str) -> bool {
        // Verbesserte Pattern-Erkennung: Prüfe Funktionsname und Body
        if func.name.contains(pattern) {
            return true;
        }

        // Prüfe auch im Body nach Pattern
        self.has_pattern_in_block(&func.body, pattern)
    }

    fn has_pattern_in_block(&self, block: &crate::parser::ast::Block, pattern: &str) -> bool {
        for stmt in &block.statements {
            match stmt {
                Statement::Expression(expr_stmt) => {
                    if self.has_pattern_in_expression(&expr_stmt.expression, pattern) {
                        return true;
                    }
                }
                Statement::Let(let_stmt) => {
                    if self.has_pattern_in_expression(&let_stmt.value, pattern) {
                        return true;
                    }
                }
                Statement::If(if_stmt) => {
                    if self.has_pattern_in_expression(&if_stmt.condition, pattern) {
                        return true;
                    }
                    if self.has_pattern_in_block(&if_stmt.then_block, pattern) {
                        return true;
                    }
                    if let Some(else_block) = &if_stmt.else_block {
                        if self.has_pattern_in_block(else_block, pattern) {
                            return true;
                        }
                    }
                }
                Statement::While(while_stmt) => {
                    if self.has_pattern_in_block(&while_stmt.body, pattern) {
                        return true;
                    }
                }
                Statement::For(for_stmt) => {
                    if self.has_pattern_in_block(&for_stmt.body, pattern) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn has_pattern_in_expression(
        &self,
        expr: &crate::parser::ast::Expression,
        pattern: &str,
    ) -> bool {
        match expr {
            crate::parser::ast::Expression::Identifier(name) => name.contains(pattern),
            crate::parser::ast::Expression::Member { object, member } => {
                member.contains(pattern) || self.has_pattern_in_expression(object, pattern)
            }
            crate::parser::ast::Expression::Call { callee, args } => {
                self.has_pattern_in_expression(callee, pattern)
                    || args
                        .iter()
                        .any(|arg| self.has_pattern_in_expression(arg, pattern))
            }
            crate::parser::ast::Expression::BinaryOp { left, right, .. } => {
                self.has_pattern_in_expression(left, pattern)
                    || self.has_pattern_in_expression(right, pattern)
            }
            crate::parser::ast::Expression::LLMCall { args, .. } => args
                .iter()
                .any(|arg| self.has_pattern_in_expression(arg, pattern)),
            _ => false,
        }
    }
}

impl DeploymentAnalyzer {
    fn evaluate_deployment_options(
        &self,
        resources: &ResourceRequirements,
    ) -> Result<Vec<DeploymentOption>> {
        let mut options = Vec::new();

        // Option 1: Local (für Entwicklung)
        options.push(DeploymentOption {
            deployment_type: DeploymentType::Local,
            cost_per_month: 0.0,
            scalability: Scalability::Low,
            latency: Latency::Low,
        });

        // Option 2: Cloud Single (für kleine Apps)
        if resources.memory_mb < 512 && resources.cpu_cores < 2 {
            options.push(DeploymentOption {
                deployment_type: DeploymentType::CloudSingle,
                cost_per_month: 10.0,
                scalability: Scalability::Medium,
                latency: Latency::Medium,
            });
        }

        // Option 3: Cloud Multi (für größere Apps)
        if resources.memory_mb >= 512 || resources.cpu_cores >= 2 {
            options.push(DeploymentOption {
                deployment_type: DeploymentType::CloudMulti,
                cost_per_month: 50.0,
                scalability: Scalability::High,
                latency: Latency::Low,
            });
        }

        // Option 4: Serverless (für sporadische Workloads)
        if resources.memory_mb < 256 {
            options.push(DeploymentOption {
                deployment_type: DeploymentType::Serverless,
                cost_per_month: 5.0,
                scalability: Scalability::VeryHigh,
                latency: Latency::Medium,
            });
        }

        Ok(options)
    }

    fn choose_best_option<'a>(
        &self,
        options: &'a [DeploymentOption],
    ) -> Result<&'a DeploymentOption> {
        // Einfache Heuristik: Wähle Option mit bestem Cost/Performance-Verhältnis
        options
            .iter()
            .min_by(|a, b| {
                let score_a = a.scalability.score() / a.cost_per_month.max(0.1);
                let score_b = b.scalability.score() / b.cost_per_month.max(0.1);
                score_b
                    .partial_cmp(&score_a)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| anyhow::anyhow!("No deployment options available"))
    }

    fn generate_deployment_plan(
        &self,
        option: &DeploymentOption,
        resources: &ResourceRequirements,
    ) -> Result<DeploymentPlan> {
        Ok(DeploymentPlan {
            deployment_type: option.deployment_type.clone(),
            needs_caching: resources.needs_caching,
            needs_database: resources.needs_database,
            replicas: Some(self.calculate_replicas(resources)),
            memory_request: Some(format!("{}Mi", resources.memory_mb)),
            cpu_request: Some(format!("{}m", resources.cpu_cores * 100)),
            memory_limit: Some(format!("{}Mi", resources.memory_mb * 2)),
            cpu_limit: Some(format!("{}m", resources.cpu_cores * 500)),
        })
    }

    fn calculate_replicas(&self, resources: &ResourceRequirements) -> u32 {
        // Verbesserte Skalierungs-Heuristik
        // Basis: Mindestens 1 Replica
        let base_replicas = 1u32;

        // CPU-basierte Skalierung: 1 Replica pro CPU Core (mit Minimum)
        let cpu_replicas = resources.cpu_cores.max(1);

        // Memory-basierte Skalierung: 1 Replica pro 512MB
        let memory_replicas = (resources.memory_mb / 512).max(1) as u32;

        // Nimm das Maximum für High Availability
        // Aber begrenze auf 10 Replicas (kann konfiguriert werden)
        let max_replicas = 10u32;
        (base_replicas.max(cpu_replicas).max(memory_replicas)).min(max_replicas)
    }
}

#[derive(Debug, Clone, Default)]
struct ResourceRequirements {
    cpu_cores: u32,
    memory_mb: u64,
    needs_database: bool,
    needs_caching: bool,
}

#[derive(Debug, Clone)]
struct DeploymentOption {
    deployment_type: DeploymentType,
    cost_per_month: f64,
    scalability: Scalability,
    latency: Latency,
}

#[derive(Debug, Clone)]
enum Scalability {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Scalability {
    fn score(&self) -> f64 {
        match self {
            Scalability::Low => 1.0,
            Scalability::Medium => 2.0,
            Scalability::High => 3.0,
            Scalability::VeryHigh => 4.0,
        }
    }
}

#[derive(Debug, Clone)]
enum Latency {
    Low,
    Medium,
    High,
}

use crate::codegen::infrastructure::InfrastructureCode;
