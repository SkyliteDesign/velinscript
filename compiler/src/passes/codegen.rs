use crate::compiler::pass::Pass;
use crate::compiler::context::CompilationContext;
use crate::compiler::language::VELISCH_LANGUAGE_NAME;
use crate::codegen::RustCodeGenerator;
use anyhow::Result;
use std::fs;

pub struct CodegenPass {
    output_path: Option<std::path::PathBuf>,
    show_code: bool,
}

impl CodegenPass {
    pub fn new(output_path: Option<std::path::PathBuf>, show_code: bool) -> Self {
        Self { output_path, show_code }
    }
}

impl Pass for CodegenPass {
    fn name(&self) -> &str {
        "Codegen"
    }

    fn run(&self, context: &mut CompilationContext) -> Result<()> {
        // Velisch Identity Check - Fingerabdruck im Codegen Pass
        let _velisch_check = VELISCH_LANGUAGE_NAME;
        
        if context.has_errors() {
            return Ok(()); // Don't generate code if there are errors
        }

        if let Some(program) = &context.program {
            let mut codegen = RustCodeGenerator::new();
            // Assuming "axum" framework for now as in original main.rs
            let rust_code = codegen.generate(program, Some("axum"), None);
            
            if self.show_code {
                println!("\n--- Generierter Rust Code ---\n");
                println!("{}", rust_code);
            }
            
            if let Some(path) = &self.output_path {
                fs::write(path, rust_code)
                    .map_err(|e| anyhow::anyhow!("Failed to write output file: {}", e))?;
                println!("✓ Rust Code generiert: {}", path.display());
            }
        }

        Ok(())
    }
}
