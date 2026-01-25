use crate::codegen::boilerplate::BoilerplateGenerator;
use crate::compiler::config::CompilerConfig;
use crate::compiler::context::CompilationContext;
use crate::compiler::pass::Pass;
use crate::parser::ast::*;
use crate::passes::ai_code_review::AICodeReviewer;
use crate::passes::ai_sandbox::AICodeSandbox;
use crate::prompt::sanitizer::PromptSanitizer;
use crate::stdlib::ml::{LLMClient, LLMProvider};
use anyhow::Result;

/// KI-basierter Code Generation Pass
///
/// Generiert fehlende Code-Teile automatisch:
/// - Identifiziert fehlende Komponenten
/// - Generiert fehlende Funktionen
/// - Generiert fehlende Datenstrukturen
/// - Generiert fehlende Tests
/// - Validiert generierten Code
pub struct AICodeGenerationPass {
    llm_client: Option<LLMClient>,
    #[allow(dead_code)]
    boilerplate_generator: BoilerplateGenerator,
    prompt_sanitizer: PromptSanitizer,
    code_reviewer: AICodeReviewer,
    code_sandbox: AICodeSandbox,
    enabled: bool,
}

#[derive(Debug, Clone)]
struct MissingComponent {
    component_type: ComponentType,
    name: String,
    specification: String,
    requirements: Vec<String>,
}

#[derive(Debug, Clone)]
enum ComponentType {
    Function,
    Struct,
    #[allow(dead_code)]
    Test,
    #[allow(dead_code)]
    Validation,
    #[allow(dead_code)]
    Documentation,
}

impl AICodeGenerationPass {
    pub fn new(config: &CompilerConfig) -> Result<Self> {
        let llm_client = if config.enable_ai_codegen {
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
            boilerplate_generator: BoilerplateGenerator::new(),
            prompt_sanitizer: PromptSanitizer::new(),
            code_reviewer: AICodeReviewer::new(),
            code_sandbox: AICodeSandbox::new(),
            enabled: config.enable_ai_codegen,
        })
    }

    /// Identifiziert fehlende Komponenten
    fn identify_missing_components(
        &self,
        program: &Program,
        context: &CompilationContext,
    ) -> Vec<MissingComponent> {
        let mut missing = Vec::new();

        // Prüfe auf fehlende Komponenten basierend auf Semantic Metadata
        if !context.semantic_metadata.missing_components.is_empty() {
            for component_name in &context.semantic_metadata.missing_components {
                missing.push(MissingComponent {
                    component_type: ComponentType::Function,
                    name: component_name.clone(),
                    specification: format!(
                        "Function {} is referenced but not defined",
                        component_name
                    ),
                    requirements: Vec::new(),
                });
            }
        }

        // Prüfe auf fehlende Structs (wenn Typ verwendet aber nicht definiert)
        let defined_structs: std::collections::HashSet<String> = program
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
                            missing.push(MissingComponent {
                                component_type: ComponentType::Struct,
                                name: name.clone(),
                                specification: format!("Struct {} is used but not defined", name),
                                requirements: vec!["Used as return type".to_string()],
                            });
                        }
                    }
                }

                // Prüfe Parameter-Typen
                for param in &f.params {
                    if let Type::Named(name) = &param.param_type {
                        if !defined_structs.contains(name) && !self.is_builtin_type(name) {
                            missing.push(MissingComponent {
                                component_type: ComponentType::Struct,
                                name: name.clone(),
                                specification: format!("Struct {} is used but not defined", name),
                                requirements: vec![format!("Used as parameter type in {}", f.name)],
                            });
                        }
                    }
                }
            }
        }

        missing
    }

    /// Prüft ob Typ ein Built-in-Typ ist
    fn is_builtin_type(&self, name: &str) -> bool {
        matches!(
            name,
            "string" | "number" | "boolean" | "void" | "List" | "Map" | "Result" | "Option"
        )
    }

    /// Generiert Code für fehlende Komponente mit KI
    fn generate_code_with_ai(
        &self,
        component: &MissingComponent,
        program: &Program,
    ) -> Result<String> {
        if let Some(ref client) = self.llm_client {
            let code_context = self.extract_code_context(program);

            // Sanitize Code-Kontext
            let sanitized_context = self.prompt_sanitizer.sanitize_code_context(&code_context);

            // Sanitize Specification und Requirements
            let sanitized_spec = self
                .prompt_sanitizer
                .sanitize_user_input(&component.specification);
            let sanitized_reqs: Vec<String> = component
                .requirements
                .iter()
                .map(|r| self.prompt_sanitizer.sanitize_user_input(r))
                .collect();

            let prompt = format!(
                "Generate VelinScript code for the following missing component:\n\n\
                Type: {:?}\n\
                Name: {}\n\
                Specification: {}\n\
                Requirements: {:?}\n\n\
                Existing code context:\n{}\n\n\
                Generate only the VelinScript code for this component, no explanations.",
                component.component_type,
                component.name,
                sanitized_spec,
                sanitized_reqs,
                sanitized_context
            );

            // Sanitize Prompt vor dem Senden
            let sanitized_prompt = self.prompt_sanitizer.sanitize(&prompt);

            if !self.prompt_sanitizer.is_safe(&sanitized_prompt) {
                // Prompt enthält gefährliche Patterns, nutze Fallback
                return self.generate_code_fallback(component);
            }

            match client.generate(&sanitized_prompt) {
                Ok(generated_code) => {
                    // Bereinige generierten Code (entferne Markdown-Code-Blöcke falls vorhanden)
                    let cleaned = generated_code
                        .trim()
                        .trim_start_matches("```velin")
                        .trim_start_matches("```")
                        .trim_end_matches("```")
                        .trim()
                        .to_string();
                    Ok(cleaned)
                }
                Err(_e) => {
                    // Fallback zu BoilerplateGenerator
                    self.generate_code_fallback(component)
                }
            }
        } else {
            // Kein LLMClient, nutze Fallback
            self.generate_code_fallback(component)
        }
    }

    /// Fallback-Code-Generierung ohne KI
    fn generate_code_fallback(&self, component: &MissingComponent) -> Result<String> {
        match component.component_type {
            ComponentType::Function => {
                // Generiere Basis-Funktion basierend auf Requirements
                let params = if component.requirements.is_empty() {
                    "".to_string()
                } else {
                    format!("{}: string", component.requirements[0])
                };
                Ok(format!(
                    "fn {}({}): string {{\n    return \"{}\";\n}}",
                    component.name, params, component.specification
                ))
            }
            ComponentType::Struct => {
                // Generiere Struct mit Basis-Feldern
                let fields = if component.requirements.is_empty() {
                    "    id: string,\n    name: string".to_string()
                } else {
                    component.requirements.iter()
                        .enumerate()
                        .map(|(i, _req)| format!("    field{}: string", i))
                        .collect::<Vec<_>>()
                        .join(",\n")
                };
                Ok(format!(
                    "struct {} {{\n{}\n}}",
                    component.name, fields
                ))
            }
            ComponentType::Test => {
                Ok(format!(
                    "fn test_{}(): void {{\n    let result = {}();\n    assert(result != null, \"{} should return a value\");\n}}",
                    component.name, component.name, component.name
                ))
            }
            ComponentType::Validation => {
                Ok(format!(
                    "fn validate_{}(value: string): boolean {{\n    return value != null && value.length() > 0;\n}}",
                    component.name
                ))
            }
            ComponentType::Documentation => {
                Ok(format!(
                    "/// {}\n/// \n/// Generated automatically",
                    component.specification
                ))
            }
        }
    }

    /// Extrahiert Code-Kontext für KI
    fn extract_code_context(&self, program: &Program) -> String {
        let mut context = String::new();

        // Zeige relevante Funktionen und Structs
        for item in &program.items {
            match item {
                Item::Function(f) => {
                    context.push_str(&format!("fn {}(", f.name));
                    for (i, param) in f.params.iter().enumerate() {
                        if i > 0 {
                            context.push_str(", ");
                        }
                        context.push_str(&format!("{}: {:?}", param.name, param.param_type));
                    }
                    context.push_str(")");
                    if let Some(rt) = &f.return_type {
                        context.push_str(&format!(" -> {:?}", rt));
                    }
                    context.push_str("\n");
                }
                Item::Struct(s) => {
                    context.push_str(&format!("struct {} {{\n", s.name));
                    for field in &s.fields {
                        context.push_str(&format!("  {}: {:?}\n", field.name, field.field_type));
                    }
                    context.push_str("}\n");
                }
                _ => {}
            }
        }

        context
    }

    /// Validiert generierten Code
    fn validate_generated_code(&self, code: &str) -> Result<String> {
        // Einfache Validierung: Prüfe auf grundlegende Syntax
        if code.trim().is_empty() {
            return Err(anyhow::anyhow!("Generated code is empty"));
        }

        // Prüfe auf grundlegende VelinScript-Syntax
        if !code.contains("fn") && !code.contains("struct") {
            return Err(anyhow::anyhow!(
                "Generated code does not contain valid VelinScript syntax"
            ));
        }

        Ok(code.to_string())
    }

    /// Fügt generierte Komponente zum AST hinzu
    fn add_component_to_ast(&self, context: &mut CompilationContext, code: &str) -> Result<()> {
        use crate::parser::parser::Parser;

        // Parse generierten Code
        match Parser::parse(code) {
            Ok(mut generated_program) => {
                // Füge Items zum bestehenden Program hinzu
                if let Some(ref mut program) = context.program {
                    program.items.append(&mut generated_program.items);
                } else {
                    // Falls kein Program existiert, erstelle neues
                    context.program = Some(generated_program);
                }
                Ok(())
            }
            Err(e) => {
                // Parsing fehlgeschlagen, speichere als Metadaten für später
                context
                    .errors
                    .push(crate::error::CompilerError::Warning(format!(
                        "Failed to parse generated code: {}. Code will be added manually.",
                        e
                    )));
                Ok(())
            }
        }
    }
}

impl Pass for AICodeGenerationPass {
    fn name(&self) -> &str {
        "AICodeGeneration"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // 1. Fehlende Komponenten identifizieren
        // Wir müssen den Borrow von program begrenzen, damit wir context später mutieren können
        let missing = if let Some(ref program) = context.program {
            self.identify_missing_components(program, context)
        } else {
            Vec::new()
        };

        // 2. Für jede fehlende Komponente:
        for component in missing {
            // 3. KI generiert Code
            // Auch hier Borrow begrenzen
            let generated_code_result = if let Some(ref program) = context.program {
                self.generate_code_with_ai(&component, program)
            } else {
                // Sollte nicht passieren, da missing nur existiert wenn program existiert
                continue;
            };

            let generated_code = match generated_code_result {
                Ok(code) => code,
                Err(e) => {
                    // Log Fehler aber fahre fort
                    context
                        .errors
                        .push(crate::error::CompilerError::warning(format!(
                            "Failed to generate code for {}: {}",
                            component.name, e
                        )));
                    continue;
                }
            };

            // 4. Code validieren
            let validated = match self.validate_generated_code(&generated_code) {
                Ok(code) => code,
                Err(e) => {
                    context
                        .errors
                        .push(crate::error::CompilerError::warning(format!(
                            "Generated code for {} is invalid: {}",
                            component.name, e
                        )));
                    continue;
                }
            };

            // 5. Code-Review durchführen
            match self.code_reviewer.review_code(&validated) {
                Ok(review_result) => {
                    if !review_result.approved {
                        // Code wurde nicht approved
                        context
                            .errors
                            .push(crate::error::CompilerError::warning(format!(
                                "Generated code for {} was rejected by code review: {:?}",
                                component.name, review_result.errors
                            )));
                        continue;
                    }

                    // Warnungen aus Review hinzufügen
                    for warning in review_result.warnings {
                        context
                            .errors
                            .push(crate::error::CompilerError::warning(format!(
                                "Code review warning for {}: {}",
                                component.name, warning
                            )));
                    }
                }
                Err(e) => {
                    context
                        .errors
                        .push(crate::error::CompilerError::warning(format!(
                            "Code review failed for {}: {}",
                            component.name, e
                        )));
                    continue;
                }
            }

            // 6. Sandbox-Check (optional, aber empfohlen)
            match self.code_sandbox.execute_safely(&validated) {
                Ok(sandbox_result) => {
                    if !sandbox_result.success {
                        context
                            .errors
                            .push(crate::error::CompilerError::warning(format!(
                                "Generated code for {} failed sandbox check: {:?}",
                                component.name, sandbox_result.errors
                            )));
                        continue;
                    }
                }
                Err(e) => {
                    context
                        .errors
                        .push(crate::error::CompilerError::warning(format!(
                            "Sandbox check failed for {}: {}",
                            component.name, e
                        )));
                    continue;
                }
            }

            // 7. Zum AST hinzufügen (nur wenn alle Checks bestanden)
            if let Err(e) = self.add_component_to_ast(context, &validated) {
                context
                    .errors
                    .push(crate::error::CompilerError::warning(format!(
                        "Failed to add generated code for {}: {}",
                        component.name, e
                    )));
            }
        }

        Ok(())
    }
}
