// Security Rules - Definiert Patterns für Security-Vulnerabilities

use velin_compiler::parser::ast::*;

#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub name: String,
    pub severity: Severity,
    pub pattern: RulePattern,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub enum RulePattern {
    SqlInjection,
    Xss,
    Csrf,
    InsecureRandom,
    HardcodedSecret,
    UnsafeDeserialization, // Used in all_rules() method
    PathTraversal,
    CommandInjection,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SecurityFinding {
    pub rule: String,
    pub severity: Severity,
    pub location: String,
    pub message: String,
    pub recommendation: String,
}

pub struct SecurityRules;

impl SecurityRules {
    /// Gibt alle Security Rules zurück
    pub fn all_rules() -> Vec<SecurityRule> {
        vec![
            SecurityRule {
                name: "SQL Injection".to_string(),
                severity: Severity::Critical,
                pattern: RulePattern::SqlInjection,
                description: "Potenzielle SQL Injection durch String-Konkatenation".to_string(),
                recommendation: "Verwende Parameterized Queries oder ORM".to_string(),
            },
            SecurityRule {
                name: "XSS (Cross-Site Scripting)".to_string(),
                severity: Severity::High,
                pattern: RulePattern::Xss,
                description: "Ungeprüfte User-Input wird in HTML ausgegeben".to_string(),
                recommendation: "Sanitize alle User-Inputs".to_string(),
            },
            SecurityRule {
                name: "CSRF (Cross-Site Request Forgery)".to_string(),
                severity: Severity::High,
                pattern: RulePattern::Csrf,
                description: "Fehlende CSRF-Protection bei State-changing Operations".to_string(),
                recommendation: "Implementiere CSRF-Tokens".to_string(),
            },
            SecurityRule {
                name: "Hardcoded Secrets".to_string(),
                severity: Severity::Critical,
                pattern: RulePattern::HardcodedSecret,
                description: "Hardcoded Passwords, API Keys oder Secrets im Code".to_string(),
                recommendation: "Verwende Environment Variables oder Secret Management".to_string(),
            },
            SecurityRule {
                name: "Insecure Random".to_string(),
                severity: Severity::Medium,
                pattern: RulePattern::InsecureRandom,
                description: "Verwendung von unsicheren Random-Funktionen".to_string(),
                recommendation: "Verwende kryptographisch sichere Random-Funktionen".to_string(),
            },
            SecurityRule {
                name: "Path Traversal".to_string(),
                severity: Severity::High,
                pattern: RulePattern::PathTraversal,
                description: "Ungeprüfte File-Paths ermöglichen Path Traversal".to_string(),
                recommendation: "Validiere und sanitize File-Paths".to_string(),
            },
            SecurityRule {
                name: "Command Injection".to_string(),
                severity: Severity::Critical,
                pattern: RulePattern::CommandInjection,
                description: "User-Input wird direkt in System-Commands verwendet".to_string(),
                recommendation: "Verwende Parameterized Commands oder Whitelisting".to_string(),
            },
            SecurityRule {
                name: "Unsafe Deserialization".to_string(),
                severity: Severity::High,
                pattern: RulePattern::UnsafeDeserialization, // Use UnsafeDeserialization
                description: "Unsichere Deserialisierung von User-Input".to_string(),
                recommendation: "Validiere und sanitize alle deserialisierten Daten".to_string(),
            },
        ]
    }

    /// Prüft ob ein Expression ein Security-Pattern matcht
    pub fn check_expression(expr: &Expression, rules: &[SecurityRule]) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();

        for rule in rules {
            if let Some(finding) = match_rule_pattern(expr, rule) {
                findings.push(finding);
            }
        }

        findings
    }
}

fn match_rule_pattern(expr: &Expression, rule: &SecurityRule) -> Option<SecurityFinding> {
    match &rule.pattern {
        RulePattern::SqlInjection => {
            // Prüfe auf String-Konkatenation in db.* Calls
            if let Expression::Call { callee, args } = expr {
                if let Expression::Member { object, member } = callee.as_ref() {
                    if let Expression::Identifier(obj_name) = object.as_ref() {
                        if obj_name == "db" && (member == "find" || member == "query") {
                            // Prüfe ob Arguments String-Konkatenation enthalten
                            for arg in args {
                                if contains_string_concat(arg) {
                                    return Some(SecurityFinding {
                                        rule: rule.name.clone(),
                                        severity: rule.severity.clone(),
                                        location: "unknown".to_string(),
                                        message: rule.description.clone(),
                                        recommendation: rule.recommendation.clone(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        RulePattern::Xss => {
            // Prüfe auf ungeprüfte User-Input in Response
            if let Expression::Call { callee, args } = expr {
                if let Expression::Identifier(name) = callee.as_ref() {
                    if name == "successResponse" || name == "errorResponse" {
                        // Prüfe ob User-Input direkt verwendet wird
                        for arg in args {
                            if is_user_input(arg) {
                                return Some(SecurityFinding {
                                    rule: rule.name.clone(),
                                    severity: rule.severity.clone(),
                                    location: "unknown".to_string(),
                                    message: rule.description.clone(),
                                    recommendation: rule.recommendation.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
        RulePattern::UnsafeDeserialization => {
            // Prüfe auf unsichere Deserialisierung
            if let Expression::Call { callee, .. } = expr {
                if let Expression::Identifier(name) = callee.as_ref() {
                    if name.contains("deserialize") || name.contains("from_str") {
                        return Some(SecurityFinding {
                            rule: rule.name.clone(),
                            severity: rule.severity.clone(),
                            location: "unknown".to_string(),
                            message: rule.description.clone(),
                            recommendation: rule.recommendation.clone(),
                        });
                    }
                }
            }
        }
        _ => {}
    }

    None
}

fn contains_string_concat(expr: &Expression) -> bool {
    match expr {
        Expression::BinaryOp { op, .. } => {
            matches!(op, BinaryOperator::Add)
        }
        Expression::Call { args, .. } => {
            args.iter().any(|arg| contains_string_concat(arg))
        }
        _ => false,
    }
}

fn is_user_input(expr: &Expression) -> bool {
    match expr {
        Expression::Member { object, member } => {
            if let Expression::Identifier(obj_name) = object.as_ref() {
                obj_name == "request" && (member == "body" || member == "query" || member == "params")
            } else {
                false
            }
        }
        _ => false,
    }
}
