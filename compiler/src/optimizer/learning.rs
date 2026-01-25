use anyhow::Result;
use std::collections::HashMap;

/// Learning System für Selbstoptimierung
///
/// Analysiert Optimierungs-Historie und lernt daraus:
/// - Analysiert Optimierungs-Historie
/// - Extrahiert Patterns
/// - Generiert neue Optimierungs-Regeln
/// - Validiert Regeln
pub struct LearningSystem {
    optimization_history: OptimizationHistory,
    success_metrics: SuccessMetrics,
}

#[derive(Debug, Clone)]
struct OptimizationHistory {
    optimizations: Vec<OptimizationRecord>,
}

#[derive(Debug, Clone)]
struct OptimizationRecord {
    optimization_type: String,
    location: String,
    before_performance: f64,
    after_performance: f64,
    success: bool,
}

#[derive(Debug, Clone)]
pub struct SuccessMetrics {
    total_optimizations: u64,
    successful_optimizations: u64,
    average_improvement: f64,
}

#[derive(Debug, Clone)]
pub struct OptimizationRules {
    rules: Vec<OptimizationRule>,
}

#[derive(Debug, Clone)]
struct OptimizationRule {
    pattern: String,
    condition: RuleCondition,
    action: RuleAction,
    confidence: f64,
}

#[derive(Debug, Clone)]
enum RuleCondition {
    FunctionComplexity(#[allow(dead_code)] usize),
    CallFrequency(#[allow(dead_code)] u64),
    ExecutionTime(#[allow(dead_code)] f64),
    MemoryUsage(#[allow(dead_code)] u64),
}

#[derive(Debug, Clone)]
enum RuleAction {
    Parallelize,
    Inline,
    Cache,
    Optimize,
}

impl LearningSystem {
    pub fn new() -> Self {
        Self {
            optimization_history: OptimizationHistory {
                optimizations: Vec::new(),
            },
            success_metrics: SuccessMetrics {
                total_optimizations: 0,
                successful_optimizations: 0,
                average_improvement: 0.0,
            },
        }
    }

    /// Lernt aus Optimierungs-Historie
    pub fn learn_from_history(&mut self) -> Result<OptimizationRules> {
        // 1. Analysiere erfolgreiche Optimierungen
        let successful_patterns = self.extract_successful_patterns()?;

        // 2. Generiere neue Regeln
        let rules = self.generate_rules(&successful_patterns)?;

        // 3. Validiere Regeln
        let validated_rules = self.validate_rules(rules)?;

        Ok(validated_rules)
    }

    /// Extrahiert erfolgreiche Patterns mit verbesserter statistischer Analyse
    fn extract_successful_patterns(&self) -> Result<Vec<OptimizationPattern>> {
        let mut patterns = Vec::new();

        // Gruppiere Optimierungen nach Typ
        let mut by_type: HashMap<String, Vec<&OptimizationRecord>> = HashMap::new();
        for record in &self.optimization_history.optimizations {
            if record.success {
                by_type
                    .entry(record.optimization_type.clone())
                    .or_insert_with(Vec::new)
                    .push(record);
            }
        }

        // Extrahiere Patterns für jeden Typ mit statistischer Analyse
        for (opt_type, records) in by_type {
            if records.len() >= 3 {
                // Mindestens 3 erfolgreiche Optimierungen für Pattern
                let improvements: Vec<f64> = records
                    .iter()
                    .map(|r| r.after_performance - r.before_performance)
                    .collect();

                let avg_improvement = improvements.iter().sum::<f64>() / improvements.len() as f64;

                // Berechne Standardabweichung für Confidence
                let variance = improvements
                    .iter()
                    .map(|x| (x - avg_improvement).powi(2))
                    .sum::<f64>()
                    / improvements.len() as f64;
                let std_dev = variance.sqrt();

                // Confidence basierend auf Konsistenz (niedrige Standardabweichung = hohe Confidence)
                let consistency = if std_dev > 0.0 {
                    (avg_improvement / std_dev.max(0.1)).min(1.0)
                } else {
                    1.0
                };

                let success_rate = records.len() as f64
                    / self
                        .optimization_history
                        .optimizations
                        .iter()
                        .filter(|r| r.optimization_type == opt_type)
                        .count() as f64;

                // Kombiniere Success Rate und Consistency für finale Confidence
                let confidence = (success_rate * 0.6 + consistency * 0.4).min(1.0);

                if confidence > 0.5 {
                    let pattern = OptimizationPattern {
                        optimization_type: opt_type.clone(),
                        success_rate: confidence,
                        average_improvement: avg_improvement,
                        common_location: self.find_common_location(&records),
                    };
                    patterns.push(pattern);
                }
            }
        }

        Ok(patterns)
    }

    /// Findet gemeinsame Location in Records
    fn find_common_location(&self, records: &[&OptimizationRecord]) -> String {
        // Einfache Heuristik: Nimm häufigste Location
        let mut location_counts: HashMap<String, u64> = HashMap::new();
        for record in records {
            *location_counts.entry(record.location.clone()).or_insert(0) += 1;
        }

        location_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(location, _)| location.clone())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Generiert neue Regeln aus Patterns
    fn generate_rules(&self, patterns: &[OptimizationPattern]) -> Result<OptimizationRules> {
        let mut rules = Vec::new();

        for pattern in patterns {
            // Generiere Regel basierend auf Pattern
            let rule = OptimizationRule {
                pattern: format!(
                    "Apply {} to {}",
                    pattern.optimization_type, pattern.common_location
                ),
                condition: self.infer_condition(pattern),
                action: self.infer_action(&pattern.optimization_type),
                confidence: pattern.success_rate,
            };
            rules.push(rule);
        }

        Ok(OptimizationRules { rules })
    }

    /// Inferiert Condition aus Pattern
    fn infer_condition(&self, pattern: &OptimizationPattern) -> RuleCondition {
        // Einfache Heuristik basierend auf Pattern-Typ
        match pattern.optimization_type.as_str() {
            "parallelize" => RuleCondition::ExecutionTime(0.1), // > 100ms
            "inline" => RuleCondition::FunctionComplexity(10),  // < 10 statements
            "cache" => RuleCondition::CallFrequency(100),       // > 100 calls
            _ => RuleCondition::ExecutionTime(0.5),             // Default
        }
    }

    /// Inferiert Action aus Optimization Type
    fn infer_action(&self, opt_type: &str) -> RuleAction {
        match opt_type {
            "parallelize" => RuleAction::Parallelize,
            "inline" => RuleAction::Inline,
            "cache" => RuleAction::Cache,
            _ => RuleAction::Optimize,
        }
    }

    /// Validiert Regeln mit Test-Validierung
    fn validate_rules(&self, rules: OptimizationRules) -> Result<OptimizationRules> {
        let mut validated_rules = Vec::new();

        for rule in rules.rules {
            // Basis-Validierung: Confidence > 0.5
            if rule.confidence <= 0.5 {
                continue;
            }

            // Zusätzliche Validierung: Prüfe ob Regel in Historie erfolgreich war
            let matching_records: Vec<&OptimizationRecord> = self
                .optimization_history
                .optimizations
                .iter()
                .filter(|r| {
                    r.optimization_type == self.rule_action_to_type(&rule.action)
                        && r.location.contains(&rule.pattern)
                })
                .collect();

            if !matching_records.is_empty() {
                let success_count = matching_records.iter().filter(|r| r.success).count();
                let success_rate = success_count as f64 / matching_records.len() as f64;

                // Nur akzeptieren wenn Success Rate > 60%
                if success_rate > 0.6 {
                    validated_rules.push(rule);
                }
            } else {
                // Neue Regel ohne Historie - akzeptiere mit niedrigerer Confidence
                if rule.confidence > 0.7 {
                    validated_rules.push(rule);
                }
            }
        }

        Ok(OptimizationRules {
            rules: validated_rules,
        })
    }

    /// Konvertiert RuleAction zu Optimization Type String
    fn rule_action_to_type(&self, action: &RuleAction) -> String {
        match action {
            RuleAction::Parallelize => "parallelize".to_string(),
            RuleAction::Inline => "inline".to_string(),
            RuleAction::Cache => "cache".to_string(),
            RuleAction::Optimize => "optimize".to_string(),
        }
    }

    /// Rollback-Mechanismus für fehlgeschlagene Optimierungen
    pub fn should_rollback(&self, opt_type: &str, location: &str) -> bool {
        // Prüfe Historie für diese Optimierung
        let recent_records: Vec<&OptimizationRecord> = self
            .optimization_history
            .optimizations
            .iter()
            .filter(|r| r.optimization_type == opt_type && r.location == location)
            .rev()
            .take(5) // Letzte 5 Versuche
            .collect();

        if recent_records.is_empty() {
            return false;
        }

        // Rollback wenn mehr als 60% der letzten Versuche fehlgeschlagen sind
        let failure_count = recent_records.iter().filter(|r| !r.success).count();
        let failure_rate = failure_count as f64 / recent_records.len() as f64;

        failure_rate > 0.6
    }

    /// Registriert Optimierung
    pub fn record_optimization(
        &mut self,
        opt_type: String,
        location: String,
        before: f64,
        after: f64,
    ) {
        let success = after > before; // Bessere Performance = Erfolg
        let improvement = after - before;

        self.optimization_history
            .optimizations
            .push(OptimizationRecord {
                optimization_type: opt_type,
                location,
                before_performance: before,
                after_performance: after,
                success,
            });

        self.success_metrics.total_optimizations += 1;
        if success {
            self.success_metrics.successful_optimizations += 1;
        }

        // Update average improvement
        let total_improvement = self.success_metrics.average_improvement
            * (self.success_metrics.total_optimizations - 1) as f64
            + improvement;
        self.success_metrics.average_improvement =
            total_improvement / self.success_metrics.total_optimizations as f64;
    }

    /// Gibt Success Metrics zurück
    pub fn get_success_metrics(&self) -> &SuccessMetrics {
        &self.success_metrics
    }
}

#[derive(Debug, Clone)]
struct OptimizationPattern {
    optimization_type: String,
    #[allow(dead_code)]
    success_rate: f64,
    #[allow(dead_code)]
    average_improvement: f64,
    common_location: String,
}
