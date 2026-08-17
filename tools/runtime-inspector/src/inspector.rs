// Runtime Inspector
// Haupt-Logik für Runtime-Inspection

use crate::variables::VariableInspector;
use crate::memory::MemoryInspector;
use velin_compiler::compiler::{VelinCompiler, config::CompilerConfig};
use velin_compiler::passes::{parser::ParserPass, type_check::TypeCheckPass};
use anyhow::Result;
use std::path::Path;
use std::fs;
use tokio::time::{sleep, Duration};

pub struct RuntimeInspector {
    variable_inspector: VariableInspector,
    memory_inspector: MemoryInspector,
}

impl RuntimeInspector {
    pub fn new() -> Self {
        Self {
            variable_inspector: VariableInspector::new(),
            memory_inspector: MemoryInspector::new(),
        }
    }
    
    pub async fn inspect(
        &self,
        file: &Path,
        show_variables: bool,
        show_memory: bool,
    ) -> Result<()> {
        let content = fs::read_to_string(file)?;
        
        // Kompiliere Code
        let mut config = CompilerConfig::default();
        config.enable_type_check = true;
        
        let mut compiler = VelinCompiler::new(config);
        compiler.add_pass(Box::new(ParserPass::new()));
        compiler.add_pass(Box::new(TypeCheckPass::new(true)));
        
        let context = compiler.compile(
            file.to_string_lossy().to_string(),
            content.clone(),
        )?;
        
        if context.has_errors() {
            eprintln!("⚠️  Kompilierungsfehler: {:?}", context.errors);
            return Ok(());
        }
        
        println!("✓ Code kompiliert erfolgreich\n");
        
        if show_variables {
            println!("📊 Variablen:");
            let variables = self.variable_inspector.extract_variables(&content)?;
            for var in &variables {
                println!("  {}: {}", var.name, var.value);
            }
            println!();
        }
        
        if show_memory {
            println!("💾 Memory-Usage:");
            let memory = self.memory_inspector.analyze(&content)?;
            println!("  Geschätzte Allokationen: {} bytes", memory.estimated_allocations);
            println!("  Variablen: {}", memory.variable_count);
            println!();
        }
        
        Ok(())
    }
    
    pub async fn watch(
        &self,
        file: &Path,
        show_variables: bool,
        show_memory: bool,
    ) -> Result<()> {
        loop {
            if let Err(e) = self.inspect(file, show_variables, show_memory).await {
                eprintln!("Fehler: {}", e);
            }
            
            sleep(Duration::from_secs(2)).await;
        }
    }
}
