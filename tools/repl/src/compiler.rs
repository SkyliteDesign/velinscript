// REPL Compiler
// Kompiliert komplexe Code-Blöcke

use velin_compiler::compiler::{VelinCompiler, config::CompilerConfig};
use velin_compiler::passes::{parser::ParserPass, type_check::TypeCheckPass};
use anyhow::Result;
use std::process::Command;
use std::fs;
use tempfile::NamedTempFile;

pub struct ReplCompiler;

impl ReplCompiler {
    pub fn new() -> Self {
        Self
    }
    
    pub fn compile_and_run(&self, code: &str) -> Result<String> {
        // Erstelle temporäre Datei
        let mut temp_file = NamedTempFile::new()?.path().to_path_buf();
        temp_file.set_extension("velin");
        
        // Schreibe Code in temporäre Datei
        fs::write(&temp_file, code)?;
        
        // Kompiliere
        let mut config = CompilerConfig::default();
        config.enable_type_check = true;
        
        let mut compiler = VelinCompiler::new(config);
        compiler.add_pass(Box::new(ParserPass::new()));
        compiler.add_pass(Box::new(TypeCheckPass::new(true)));
        
        let context = compiler.compile(
            temp_file.to_string_lossy().to_string(),
            code.to_string(),
        )?;
        
        if context.has_errors() {
            return Err(anyhow::anyhow!("Kompilierungsfehler: {:?}", context.errors));
        }
        
        // Für jetzt geben wir nur eine Bestätigung zurück
        // In einer vollständigen Implementierung würde man den Code ausführen
        Ok("Code kompiliert erfolgreich".to_string())
    }
}
