use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use crate::config::LibraryConfig;
use crate::templates::TemplateEngine;
use crate::integration::{ModRsIntegration, TypeCheckerIntegration, CodegenIntegration};

pub struct LibraryGenerator {
    template_engine: TemplateEngine,
}

impl LibraryGenerator {
    pub fn new() -> Self {
        Self {
            template_engine: TemplateEngine::new(),
        }
    }
    
    /// Prüft ob das Tool vom Projekt-Root ausgeführt wird
    fn check_working_directory(&self) -> Result<()> {
        let mod_rs_path = PathBuf::from("compiler/src/stdlib/mod.rs");
        if !mod_rs_path.exists() {
            anyhow::bail!(
                "❌ Fehler: Tool muss vom Projekt-Root ausgeführt werden!\n\
                 Aktuelles Verzeichnis: {:?}\n\
                 Erwarteter Pfad: compiler/src/stdlib/mod.rs\n\
                 \n\
                 💡 Tipp: Wechseln Sie ins Projekt-Root-Verzeichnis oder verwenden Sie:\n\
                 cd /path/to/velinscript && velin-library-generator ...",
                std::env::current_dir().unwrap_or_default()
            );
        }
        Ok(())
    }
    
    /// Prüft ob das Modul bereits existiert
    fn check_module_exists(&self, config: &LibraryConfig) -> Result<bool> {
        let module_path = PathBuf::from(format!("compiler/src/stdlib/{}.rs", config.snake_case_name()));
        Ok(module_path.exists())
    }
    
    pub fn generate(&self, config: &LibraryConfig) -> Result<()> {
        // Prüfe Arbeitsverzeichnis
        self.check_working_directory()?;
        
        // Prüfe ob Modul bereits existiert
        if self.check_module_exists(config)? {
            anyhow::bail!(
                "❌ Fehler: Modul '{}' existiert bereits!\n\
                 Datei: compiler/src/stdlib/{}.rs\n\
                 \n\
                 💡 Tipp: Entfernen Sie das Modul manuell oder verwenden Sie einen anderen Namen.",
                config.name,
                config.snake_case_name()
            );
        }
        
        println!("📦 Generiere Modul '{}'...", config.name);
        
        // 1. Generiere Modul-Datei
        println!("  → Erstelle compiler/src/stdlib/{}.rs", config.snake_case_name());
        let module_code = self.template_engine.render_module(config)?;
        self.write_file(
            &format!("compiler/src/stdlib/{}.rs", config.snake_case_name()),
            &module_code
        )?;
        
        // 2. Integriere in mod.rs
        println!("  → Integriere in compiler/src/stdlib/mod.rs");
        let mod_rs_integration = ModRsIntegration::new();
        mod_rs_integration.integrate(config)?;
        
        // 3. Integriere in Type Checker
        println!("  → Integriere in Type Checker");
        let type_checker_integration = TypeCheckerIntegration::new();
        type_checker_integration.integrate(config)?;
        
        // 4. Integriere in Code Generator
        println!("  → Integriere in Code Generator");
        let codegen_integration = CodegenIntegration::new();
        codegen_integration.integrate(config)?;
        
        // 5. Generiere Tests
        println!("  → Erstelle compiler/tests/{}_test.rs", config.snake_case_name());
        let test_code = self.template_engine.render_test(config)?;
        self.write_file(
            &format!("compiler/tests/{}_test.rs", config.snake_case_name()),
            &test_code
        )?;
        
        // 6. Generiere Dokumentation
        println!("  → Erstelle docs/api/{}.md", config.snake_case_name());
        let docs = self.template_engine.render_docs(config)?;
        self.write_file(
            &format!("docs/api/{}.md", config.snake_case_name()),
            &docs
        )?;
        
        // Zusammenfassung
        println!("\n✅ Modul '{}' erfolgreich generiert!", config.name);
        println!("\n📋 Generierte Dateien:");
        println!("  • compiler/src/stdlib/{}.rs", config.snake_case_name());
        println!("  • compiler/tests/{}_test.rs", config.snake_case_name());
        println!("  • docs/api/{}.md", config.snake_case_name());
        println!("\n💡 Nächste Schritte:");
        println!("  1. Implementieren Sie die Funktions-Logik in compiler/src/stdlib/{}.rs", config.snake_case_name());
        println!("  2. Erweitern Sie die Tests in compiler/tests/{}_test.rs", config.snake_case_name());
        println!("  3. Führen Sie 'cargo test' aus, um die Tests zu überprüfen");
        println!("  4. Kompilieren Sie den Compiler mit 'cargo build'");
        
        Ok(())
    }
    
    fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let path = PathBuf::from(path);
        
        // Erstelle Verzeichnis falls nötig
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Konnte Verzeichnis nicht erstellen: {:?}", parent))?;
        }
        
        fs::write(&path, content)
            .with_context(|| format!("Konnte Datei nicht schreiben: {:?}", path))?;
        
        Ok(())
    }
}
