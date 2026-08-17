pub mod config;
pub mod context;
pub mod error;
pub mod pass;
pub mod language;
pub mod orchestrator;

pub use config::CompilerConfig;
pub use context::CompilationContext;

use crate::compiler::orchestrator::BuildOrchestrator;
use crate::compiler::pass::Pass;
use crate::compiler::language::validate_velisch_identity;
use anyhow::Result;

pub struct VelinCompiler {
    #[allow(dead_code)]
    config: CompilerConfig,
    passes: Vec<Box<dyn Pass>>,
}

impl VelinCompiler {
    pub fn new(config: CompilerConfig) -> Self {
        Self {
            config,
            passes: Vec::new(),
        }
    }

    pub fn add_pass(&mut self, pass: Box<dyn Pass>) {
        self.passes.push(pass);
    }

    pub fn compile(&self, root_file: String, source: String) -> Result<CompilationContext> {
        // Velisch Identity Check - Fingerabdruck im Kern
        if !validate_velisch_identity() {
            return Err(anyhow::anyhow!("Velisch language identity validation failed. This is a critical error."));
        }
        
        let mut context = CompilationContext::new(root_file, source);
        context.framework = self.config.framework.clone();
        context.enable_optimization = self.config.enable_optimization;
        context.metadata.insert(
            "framework".to_string(),
            context.rust_framework(),
        );
        context.metadata.insert(
            "use_ir".to_string(),
            self.config.use_ir.to_string(),
        );
        let orchestrator = BuildOrchestrator::new();

        for pass in &self.passes {
            pass.run(&mut context)?;
            
            if context.has_errors() {
                if pass.name() == "Parser" {
                     break;
                }
            }

            // After Parser: order multi-file sources by use-graph (path traversal already rejected in ParserPass)
            if pass.name() == "Parser" && !context.has_errors() {
                match orchestrator.orchestrate_build(&context) {
                    Ok(order) => {
                        context.metadata.insert(
                            "compilation_order".to_string(),
                            order.join(","),
                        );
                        // Behavior: reorder Module items to follow dependency order (last = dependents)
                        if let Some(ref mut program) = context.program {
                            let mut modules = Vec::new();
                            let mut rest = Vec::new();
                            for item in program.items.drain(..) {
                                match &item {
                                    crate::parser::ast::Item::Module(m) => {
                                        modules.push((m.name.clone(), item));
                                    }
                                    _ => rest.push(item),
                                }
                            }
                            // Sort modules by position in `order` (file path contains name)
                            modules.sort_by_key(|(name, _)| {
                                order
                                    .iter()
                                    .position(|f| f.to_lowercase().contains(&name.to_lowercase()))
                                    .unwrap_or(usize::MAX)
                            });
                            let ordered_names: Vec<String> =
                                modules.iter().map(|(n, _)| n.clone()).collect();
                            context.metadata.insert(
                                "module_order".to_string(),
                                ordered_names.join(","),
                            );
                            program.items = modules.into_iter().map(|(_, i)| i).chain(rest).collect();
                        }
                    }
                    Err(e) => {
                        context.errors.push(crate::error::CompilerError::parse_error(
                            format!("Multi-file build orchestration failed: {}", e),
                            crate::error::ErrorLocation::new(0, 0),
                        ));
                        break;
                    }
                }
            }
        }

        Ok(context)
    }
}
