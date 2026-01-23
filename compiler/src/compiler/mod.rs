pub mod config;
pub mod context;
pub mod error;
pub mod pass;
pub mod language;
pub mod orchestrator;

use crate::compiler::config::CompilerConfig;
use crate::compiler::context::CompilationContext;
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

        for pass in &self.passes {
            // tracing::info!("Running pass: {}", pass.name());
            pass.run(&mut context)?;
            
            if context.has_errors() {
                // For now, stop on first pass failure unless we have a "tolerant" mode
                // But let's allow Autofix pass to proceed even if previous had issues? 
                // Actually, Autofix is first.
                // If Parser fails, we stop.
                if pass.name() == "Parser" {
                     break;
                }
            }
        }

        Ok(context)
    }
}
