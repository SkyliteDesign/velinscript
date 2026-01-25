use crate::compiler::config::CompilerConfig;
use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::parser::ast::*;
/// AI Code Review - Validiert und reviewt AI-generierten Code
///
/// Dieses Modul implementiert einen Code-Review-Mechanismus für AI-generierten Code:
/// - Syntax-Validierung
/// - Type-Checking
/// - Security-Checks
/// - Sandbox-Execution (optional)
///
/// # Beispiel
///
/// ```rust
/// use velin_compiler::passes::ai_code_review::AICodeReviewer;
///
/// let reviewer = AICodeReviewer::new();
/// match reviewer.review_code(generated_code) {
///     Ok(approved) => println!("Code approved: {}", approved),
///     Err(e) => println!("Code rejected: {}", e),
/// }
/// ```
use crate::parser::parser::Parser;
use anyhow::Result;
use std::collections::HashSet;

/// AI Code Reviewer
pub struct AICodeReviewer {
    allowed_imports: HashSet<String>,
    forbidden_patterns: Vec<String>,
    max_complexity: usize,
}

/// Code Review Ergebnis
#[derive(Debug, Clone)]
pub struct CodeReviewResult {
    pub approved: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub security_issues: Vec<String>,
    pub suggestions: Vec<String>,
}

impl AICodeReviewer {
    /// Erstellt einen neuen AI Code Reviewer
    pub fn new() -> Self {
        // Erlaubte Imports (nur Standardbibliothek)
        let allowed_imports = HashSet::from([
            "string".to_string(),
            "math".to_string(),
            "collections".to_string(),
            "json".to_string(),
            "date".to_string(),
            "validation".to_string(),
        ]);

        // Verbotene Patterns (potenziell gefährlich)
        let forbidden_patterns = vec![
            "unsafe".to_string(),
            "system".to_string(),
            "exec".to_string(),
            "eval".to_string(),
            "shell".to_string(),
            "command".to_string(),
            "file_system".to_string(),
            "network".to_string(),
            "database".to_string(),
        ];

        AICodeReviewer {
            allowed_imports,
            forbidden_patterns,
            max_complexity: 50, // Maximale Zyklomatische Komplexität
        }
    }

    /// Reviewt AI-generierten Code
    pub fn review_code(&self, code: &str) -> Result<CodeReviewResult> {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        let mut security_issues = Vec::new();
        let mut suggestions = Vec::new();

        // 1. Syntax-Validierung
        match Parser::parse(code) {
            Ok(program) => {
                // 2. Type-Checking (vereinfacht)
                self.check_types(&program, &mut warnings, &mut errors)?;

                // 3. Security-Checks
                self.check_security(&program, &mut security_issues)?;

                // 4. Complexity-Check
                self.check_complexity(&program, &mut warnings, &mut suggestions)?;

                // 5. Import-Check
                self.check_imports(&program, &mut errors, &mut security_issues)?;

                // 6. Pattern-Check
                self.check_patterns(code, &mut security_issues)?;
            }
            Err(e) => {
                errors.push(format!("Syntax error: {}", e.message));
                return Ok(CodeReviewResult {
                    approved: false,
                    warnings,
                    errors,
                    security_issues,
                    suggestions,
                });
            }
        }

        let approved = errors.is_empty() && security_issues.is_empty();

        Ok(CodeReviewResult {
            approved,
            warnings,
            errors,
            security_issues,
            suggestions,
        })
    }

    /// Prüft Typen (vereinfacht)
    fn check_types(
        &self,
        program: &Program,
        _warnings: &mut Vec<String>,
        errors: &mut Vec<String>,
    ) -> Result<()> {
        // Prüfe auf undefinierte Typen
        let defined_structs: HashSet<String> = program
            .items
            .iter()
            .filter_map(|item| {
                if let Item::Struct(s) = item {
                    Some(s.name.clone())
                } else {
                    None
                }
            })
            .collect();

        for item in &program.items {
            if let Item::Function(f) = item {
                // Prüfe Return-Type
                if let Some(return_type) = &f.return_type {
                    if let Type::Named(name) = return_type {
                        if !defined_structs.contains(name) && !self.is_builtin_type(name) {
                            errors.push(format!(
                                "Function {} returns undefined type: {}",
                                f.name, name
                            ));
                        }
                    }
                }

                // Prüfe Parameter-Typen
                for param in &f.params {
                    if let Type::Named(name) = &param.param_type {
                        if !defined_structs.contains(name) && !self.is_builtin_type(name) {
                            errors.push(format!(
                                "Function {} has parameter {} with undefined type: {}",
                                f.name, param.name, name
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Prüft ob Typ ein Built-in-Typ ist
    fn is_builtin_type(&self, name: &str) -> bool {
        matches!(
            name,
            "string" | "number" | "boolean" | "void" | "List" | "Map" | "Result" | "Option" | "any"
        )
    }

    /// Prüft Security
    fn check_security(&self, program: &Program, security_issues: &mut Vec<String>) -> Result<()> {
        for item in &program.items {
            if let Item::Function(f) = item {
                // Prüfe auf fehlende Input-Validierung bei sensiblen Operationen
                let is_sensitive = f.name.contains("delete")
                    || f.name.contains("update")
                    || f.name.contains("admin")
                    || f.name.contains("modify");

                if is_sensitive {
                    let has_auth = f.decorators.iter().any(|d| d.name == "@Auth");
                    if !has_auth {
                        security_issues.push(format!(
                            "Function {} performs sensitive operation without @Auth decorator",
                            f.name
                        ));
                    }
                }

                // Prüfe auf direkte String-Konkatenation (SQL-Injection-Risiko)
                // Dies würde in einer vollständigen Implementierung den AST durchsuchen
            }
        }

        Ok(())
    }

    /// Prüft Komplexität
    fn check_complexity(
        &self,
        program: &Program,
        warnings: &mut Vec<String>,
        suggestions: &mut Vec<String>,
    ) -> Result<()> {
        for item in &program.items {
            if let Item::Function(f) = item {
                let complexity = self.calculate_complexity(&f.body);
                if complexity > self.max_complexity {
                    warnings.push(format!(
                        "Function {} has high complexity: {} (max: {})",
                        f.name, complexity, self.max_complexity
                    ));
                    suggestions.push(format!(
                        "Consider refactoring function {} into smaller functions",
                        f.name
                    ));
                }
            }
        }

        Ok(())
    }

    /// Berechnet Code-Komplexität
    fn calculate_complexity(&self, block: &Block) -> usize {
        let mut complexity = 1; // Basis-Komplexität

        for stmt in &block.statements {
            match stmt {
                Statement::If(if_stmt) => {
                    complexity += 1;
                    complexity += self.calculate_complexity(&if_stmt.then_block);
                    if let Some(ref else_block) = if_stmt.else_block {
                        complexity += self.calculate_complexity(else_block);
                    }
                }
                Statement::For(for_stmt) => {
                    complexity += 1;
                    complexity += self.calculate_complexity(&for_stmt.body);
                }
                Statement::While(while_stmt) => {
                    complexity += 1;
                    complexity += self.calculate_complexity(&while_stmt.body);
                }
                Statement::Match(match_stmt) => {
                    complexity += match_stmt.arms.len();
                    for arm in &match_stmt.arms {
                        complexity += self.calculate_complexity(&arm.body);
                    }
                }
                _ => {}
            }
        }

        complexity
    }

    /// Prüft Imports
    fn check_imports(
        &self,
        program: &Program,
        errors: &mut Vec<String>,
        security_issues: &mut Vec<String>,
    ) -> Result<()> {
        for item in &program.items {
            if let Item::Use(use_stmt) = item {
                // Use hat path: Vec<String>, nimm ersten Teil als Modul-Name
                let module_name = use_stmt.path.first().map(|s| s.as_str()).unwrap_or("");

                // Prüfe ob Import erlaubt ist
                if !self.allowed_imports.contains(module_name) {
                    security_issues.push(format!(
                        "Import of '{}' is not allowed in AI-generated code",
                        module_name
                    ));
                }

                // Prüfe auf verbotene Module
                for forbidden in &self.forbidden_patterns {
                    if module_name.contains(forbidden) {
                        errors.push(format!(
                            "Import of '{}' is forbidden (contains '{}')",
                            module_name, forbidden
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Prüft Code auf gefährliche Patterns
    fn check_patterns(&self, code: &str, security_issues: &mut Vec<String>) -> Result<()> {
        for pattern in &self.forbidden_patterns {
            if code.contains(pattern) {
                security_issues.push(format!("Code contains forbidden pattern: '{}'", pattern));
            }
        }

        // Prüfe auf verdächtige String-Literale
        if code.contains("api_key") || code.contains("password") || code.contains("secret") {
            security_issues.push("Code contains potentially sensitive string literals".to_string());
        }

        Ok(())
    }
}

impl Default for AICodeReviewer {
    fn default() -> Self {
        Self::new()
    }
}

/// AI Code Review Pass
///
/// Reviewt AI-generierten Code auf Sicherheit und Qualität
pub struct AICodeReviewPass {
    reviewer: AICodeReviewer,
    enabled: bool,
}

impl AICodeReviewPass {
    pub fn new(config: &CompilerConfig) -> Result<Self> {
        Ok(Self {
            reviewer: AICodeReviewer::new(),
            enabled: config.enable_ai_codegen, // Nur aktiv wenn AI-Codegen aktiviert ist
        })
    }
}

impl Pass for AICodeReviewPass {
    fn name(&self) -> &str {
        "AICodeReview"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(()); // Pass überspringen wenn nicht aktiviert
        }

        // Prüfe ob es AI-generierten Code gibt (z.B. aus AICodeGenerationPass)
        if let Some(program) = &context.program {
            // Review nur AI-generierte Funktionen/Structs
            // (könnte durch Metadaten markiert sein)

            // Für jetzt: Review alle Funktionen die nach AICodeGenerationPass hinzugefügt wurden
            // In einer vollständigen Implementierung würde man Metadaten verwenden

            // Vereinfachte Implementierung: Review das gesamte Programm
            let code = format!("{:?}", program); // Vereinfacht - sollte echten Code-String verwenden

            match self.reviewer.review_code(&code) {
                Ok(result) => {
                    if !result.approved {
                        // Füge Warnungen/Fehler zum Context hinzu
                        for error in &result.errors {
                            context.errors.push(crate::error::CompilerError::type_error(
                                format!("AI Code Review Error: {}", error),
                                crate::error::ErrorLocation::new(0, 0),
                            ));
                        }
                        for security_issue in &result.security_issues {
                            context.errors.push(crate::error::CompilerError::type_error(
                                format!("AI Code Review Security Issue: {}", security_issue),
                                crate::error::ErrorLocation::new(0, 0),
                            ));
                        }
                    }
                    // Warnungen können ignoriert werden oder als Info gespeichert werden
                }
                Err(e) => {
                    context.errors.push(crate::error::CompilerError::type_error(
                        format!("AI Code Review failed: {}", e),
                        crate::error::ErrorLocation::new(0, 0),
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_review_valid_code() {
        let reviewer = AICodeReviewer::new();
        let code = r#"
            fn add(a: number, b: number): number {
                return a + b;
            }
        "#;
        let result = reviewer.review_code(code).unwrap();
        assert!(result.approved);
    }

    #[test]
    fn test_review_invalid_syntax() {
        let reviewer = AICodeReviewer::new();
        let code = "fn invalid {";
        let result = reviewer.review_code(code).unwrap();
        assert!(!result.approved);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_review_forbidden_import() {
        let reviewer = AICodeReviewer::new();
        let code = r#"
            use file_system;
            fn test(): void {}
        "#;
        let result = reviewer.review_code(code).unwrap();
        assert!(!result.approved);
        assert!(!result.security_issues.is_empty());
    }
}
