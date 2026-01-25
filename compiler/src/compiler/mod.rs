pub mod config;
pub mod context;
pub mod error;
pub mod language;
pub mod orchestrator;
pub mod pass;

use crate::compiler::config::CompilerConfig;
use crate::compiler::context::CompilationContext;
use crate::compiler::language::validate_velisch_identity;
use crate::compiler::pass::Pass;
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
            return Err(anyhow::anyhow!(
                "Velisch language identity validation failed. This is a critical error."
            ));
        }

        let mut context = CompilationContext::new(root_file, source);

        // Definiere kritische Passes, die bei Fehlern stoppen sollten
        let critical_passes = ["Parser", "TypeCheck", "Codegen"];

        for pass in &self.passes {
            // tracing::info!("Running pass: {}", pass.name());
            pass.run(&mut context)?;

            if context.has_errors() {
                // Stoppe bei kritischen Passes
                if critical_passes.contains(&pass.name()) {
                    break;
                }
            }
        }

        Ok(context)
    }
}
