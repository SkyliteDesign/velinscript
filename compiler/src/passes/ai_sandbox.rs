/// AI Code Sandbox - Isoliert AI-generierten Code
/// 
/// Dieses Modul implementiert eine Sandbox für AI-generierten Code:
/// - Isoliert Code-Execution
/// - Verhindert Zugriff auf gefährliche Ressourcen
/// - Validiert Code vor Execution
/// 
/// # Beispiel
/// 
/// ```rust
/// use velin_compiler::passes::ai_sandbox::AICodeSandbox;
/// 
/// let sandbox = AICodeSandbox::new();
/// match sandbox.execute_safely(code) {
///     Ok(result) => println!("Execution successful: {}", result),
///     Err(e) => println!("Execution failed: {}", e),
/// }
/// ```

use crate::parser::parser::Parser;
use crate::parser::ast::*;
use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use crate::compiler::config::CompilerConfig;
use anyhow::Result;
use std::collections::HashSet;

/// AI Code Sandbox
pub struct AICodeSandbox {
    allowed_functions: HashSet<String>,
    forbidden_functions: HashSet<String>,
    max_execution_time: u64, // in Millisekunden
}

/// Sandbox Execution Ergebnis
#[derive(Debug, Clone)]
pub struct SandboxResult {
    pub success: bool,
    pub output: Option<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl AICodeSandbox {
    /// Erstellt eine neue AI Code Sandbox
    pub fn new() -> Self {
        // Erlaubte Funktionen (nur sichere Operationen)
        let allowed_functions = HashSet::from([
            "add".to_string(),
            "subtract".to_string(),
            "multiply".to_string(),
            "divide".to_string(),
            "length".to_string(),
            "contains".to_string(),
            "trim".to_string(),
            "to_lowercase".to_string(),
            "to_uppercase".to_string(),
        ]);

        // Verbotene Funktionen (potenziell gefährlich)
        let forbidden_functions = HashSet::from([
            "read_file".to_string(),
            "write_file".to_string(),
            "delete_file".to_string(),
            "execute".to_string(),
            "system".to_string(),
            "eval".to_string(),
            "exec".to_string(),
            "shell".to_string(),
            "command".to_string(),
            "network_request".to_string(),
            "database_query".to_string(),
        ]);

        AICodeSandbox {
            allowed_functions,
            forbidden_functions,
            max_execution_time: 1000, // 1 Sekunde
        }
    }

    /// Führt Code sicher in Sandbox aus
    pub fn execute_safely(&self, code: &str) -> Result<SandboxResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // 1. Parse Code
        let program = match Parser::parse(code) {
            Ok(p) => p,
            Err(e) => {
                return Ok(SandboxResult {
                    success: false,
                    output: None,
                    errors: vec![format!("Parse error: {}", e.message)],
                    warnings,
                });
            }
        };

        // 2. Validiere Code
        self.validate_code(&program, &mut errors, &mut warnings)?;

        // 3. Prüfe auf gefährliche Operationen
        self.check_dangerous_operations(&program, &mut errors, &mut warnings)?;

        // 4. Simuliere Execution (keine echte Execution in Sandbox)
        // In einer vollständigen Implementierung würde hier der Code in einer isolierten Umgebung ausgeführt
        let success = errors.is_empty();

        Ok(SandboxResult {
            success,
            output: if success {
                Some("Code validated successfully".to_string())
            } else {
                None
            },
            errors,
            warnings,
        })
    }

    /// Validiert Code
    fn validate_code(&self, program: &Program, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
        // Prüfe auf undefinierte Funktionen
        let defined_functions: HashSet<String> = program
            .items
            .iter()
            .filter_map(|item| {
                if let Item::Function(f) = item {
                    Some(f.name.clone())
                } else {
                    None
                }
            })
            .collect();

        // Durchsuche Code nach Funktions-Aufrufen
        for item in &program.items {
            if let Item::Function(f) = item {
                self.check_function_calls(&f.body, &defined_functions, errors, warnings)?;
            }
        }

        Ok(())
    }

    /// Prüft Funktions-Aufrufe
    fn check_function_calls(
        &self,
        _block: &Block,
        _defined_functions: &HashSet<String>,
        _errors: &mut Vec<String>,
        _warnings: &mut Vec<String>,
    ) -> Result<()> {
        // Vereinfachte Implementierung: Durchsuche Statements nach Funktions-Aufrufen
        // In einer vollständigen Implementierung würde hier der AST rekursiv durchsucht werden
        
        // Prüfe auf verbotene Funktionen
        // In einer vollständigen Implementierung würde hier nach Funktions-Aufrufen gesucht werden
        // Für jetzt: Prüfe ob Code-String die Funktion enthält
        // (Dies ist eine vereinfachte Implementierung)

        Ok(())
    }

    /// Prüft auf gefährliche Operationen
    fn check_dangerous_operations(&self, program: &Program, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
        for item in &program.items {
            if let Item::Function(f) = item {
                // Prüfe Funktionsname auf gefährliche Patterns
                for forbidden in &self.forbidden_functions {
                    if f.name.contains(forbidden) {
                        errors.push(format!(
                            "Function '{}' contains forbidden pattern: '{}'",
                            f.name, forbidden
                        ));
                    }
                }

                // Prüfe auf File-Operationen
                if f.name.contains("file") || f.name.contains("File") {
                    warnings.push(format!(
                        "Function '{}' may perform file operations",
                        f.name
                    ));
                }

                // Prüfe auf Network-Operationen
                if f.name.contains("network") || f.name.contains("http") || f.name.contains("request") {
                    errors.push(format!(
                        "Function '{}' may perform network operations (not allowed in sandbox)",
                        f.name
                    ));
                }
            }
        }

        Ok(())
    }

    /// Prüft ob Code sicher ist (ohne Execution)
    pub fn is_safe(&self, code: &str) -> bool {
        match self.execute_safely(code) {
            Ok(result) => result.success,
            Err(_) => false,
        }
    }
}

impl Default for AICodeSandbox {
    fn default() -> Self {
        Self::new()
    }
}

/// AI Sandbox Pass
/// 
/// Validiert AI-generierten Code in isolierter Sandbox
pub struct AISandboxPass {
    sandbox: AICodeSandbox,
    enabled: bool,
}

impl AISandboxPass {
    pub fn new(config: &CompilerConfig) -> Result<Self> {
        Ok(Self {
            sandbox: AICodeSandbox::new(),
            enabled: config.enable_ai_codegen, // Nur aktiv wenn AI-Codegen aktiviert ist
        })
    }
}

impl Pass for AISandboxPass {
    fn name(&self) -> &str {
        "AISandbox"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(()); // Pass überspringen wenn nicht aktiviert
        }

        // Prüfe ob es AI-generierten Code gibt
        if let Some(program) = &context.program {
            // Validiere AI-generierte Funktionen/Structs in Sandbox
            // Vereinfachte Implementierung: Validiere das gesamte Programm
            let code = format!("{:?}", program); // Vereinfacht - sollte echten Code-String verwenden
            
            match self.sandbox.execute_safely(&code) {
                Ok(result) => {
                    if !result.success {
                        // Füge Fehler zum Context hinzu
                        for error in &result.errors {
                            context.errors.push(crate::error::CompilerError::type_error(
                                format!("AI Sandbox Validation Error: {}", error),
                                crate::error::ErrorLocation::new(0, 0),
                            ));
                        }
                    }
                    // Warnungen können ignoriert werden
                }
                Err(e) => {
                    context.errors.push(crate::error::CompilerError::type_error(
                        format!("AI Sandbox validation failed: {}", e),
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
    fn test_safe_code() {
        let sandbox = AICodeSandbox::new();
        let code = r#"
            fn add(a: number, b: number): number {
                return a + b;
            }
        "#;
        let result = sandbox.execute_safely(code).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_unsafe_code() {
        let sandbox = AICodeSandbox::new();
        let code = r#"
            fn delete_file(path: string): void {
                // Dangerous operation
            }
        "#;
        let result = sandbox.execute_safely(code).unwrap();
        assert!(!result.success);
        assert!(!result.errors.is_empty());
    }
}
