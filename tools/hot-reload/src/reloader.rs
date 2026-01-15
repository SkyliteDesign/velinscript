// Reloader
// Kompiliert Code neu und startet Server

use anyhow::Result;
use std::process::{Command, Child};
use std::path::PathBuf;
use tokio::process::Command as TokioCommand;

pub struct Reloader {
    compile_command: String,
}

impl Reloader {
    pub fn new(compile_command: &str) -> Self {
        Reloader {
            compile_command: compile_command.to_string(),
        }
    }
    
    pub async fn reload(&self, changed_files: &[PathBuf]) -> Result<()> {
        // Parse compile command
        let parts: Vec<&str> = self.compile_command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!("Ungültiger Kompilier-Befehl"));
        }
        
        let mut cmd = TokioCommand::new(parts[0]);
        
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }
        
        // Füge geänderte Dateien hinzu
        for file in changed_files {
            cmd.arg(file);
        }
        
        let output = cmd.output().await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Kompilierungsfehler: {}", stderr));
        }
        
        Ok(())
    }
    
    pub async fn start_server(&self) -> Result<Child> {
        // Parse run command
        let parts: Vec<&str> = self.compile_command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!("Ungültiger Server-Befehl"));
        }
        
        let mut cmd = Command::new(parts[0]);
        
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }
        
        let child = cmd.spawn()?;
        Ok(child)
    }
    
    pub async fn restart_server(&self, mut old_process: Child) -> Result<Child> {
        // Beende alten Prozess
        let _ = old_process.kill();
        let _ = old_process.wait();
        
        // Starte neuen Prozess
        self.start_server().await
    }
}
