use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use crate::config::LibraryConfig;

pub struct ModRsIntegration;

impl ModRsIntegration {
    pub fn new() -> Self {
        Self
    }
    
    pub fn integrate(&self, config: &LibraryConfig) -> Result<()> {
        let mod_rs_path = PathBuf::from("compiler/src/stdlib/mod.rs");
        
        if !mod_rs_path.exists() {
            anyhow::bail!(
                "❌ Fehler: compiler/src/stdlib/mod.rs nicht gefunden!\n\
                 Stellen Sie sicher, dass Sie das Tool vom Projekt-Root ausführen.\n\
                 Aktuelles Verzeichnis: {:?}",
                std::env::current_dir().unwrap_or_default()
            );
        }
        
        // Lese bestehende mod.rs
        let content = fs::read_to_string(&mod_rs_path)
            .with_context(|| format!("Konnte mod.rs nicht lesen: {:?}", mod_rs_path))?;
        
        // Prüfe ob Modul bereits existiert
        let module_line = format!("pub mod {};", config.snake_case_name());
        if content.contains(&module_line) {
            println!("    ⚠️  Modul bereits in mod.rs vorhanden, überspringe...");
            return Ok(());
        }
        
        // Füge Modul hinzu (am Ende der pub mod Liste)
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        // Finde die letzte pub mod Zeile (vor cfg(feature) oder am Ende)
        let mut last_mod_idx = 0;
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("pub mod") && !trimmed.contains("#[cfg") {
                last_mod_idx = idx;
            }
        }
        
        // Prüfe ob alphabetisch sortiert werden soll
        let module_name = config.snake_case_name();
        let mut insert_pos = last_mod_idx + 1;
        
        // Versuche alphabetisch einzufügen
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("pub mod") {
                if let Some(mod_name) = trimmed.strip_prefix("pub mod").and_then(|s| s.strip_suffix(";")) {
                    let existing_mod = mod_name.trim();
                    if existing_mod > &module_name {
                        insert_pos = idx;
                        break;
                    }
                }
            }
        }
        
        // Füge neue Zeile ein
        lines.insert(insert_pos, module_line);
        
        // Schreibe zurück
        let new_content = lines.join("\n");
        fs::write(&mod_rs_path, new_content)
            .with_context(|| "Konnte mod.rs nicht schreiben")?;
        
        Ok(())
    }
}
