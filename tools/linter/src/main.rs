// VelinScript Linter
// Analysiert VelinScript Code auf Code-Qualität, Best Practices und potenzielle Probleme

mod analyzer;
mod rules;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use analyzer::Linter;

#[derive(Parser)]
#[command(name = "velin-lint")]
#[command(about = "VelinScript Linter - Code Quality Analysis", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analysiert Code auf Linter-Probleme
    Check {
        /// Pfad zu analysieren (Datei oder Verzeichnis)
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Auto-fix für einfache Probleme
        #[arg(short, long)]
        fix: bool,
        
        /// JSON-Output
        #[arg(short, long)]
        json: bool,
        
        /// Nur bestimmte Regeln ausführen
        #[arg(short, long)]
        rules: Option<Vec<String>>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { path, fix, json, rules } => {
            let mut linter = Linter::new();
            
            // Lade Regeln
            if let Some(rule_names) = rules {
                linter.enable_rules(rule_names);
            } else {
                linter.enable_all_rules();
            }
            
            // Sammle VelinScript-Dateien
            let mut files = Vec::new();
            
            if path.is_file() {
                if path.extension().and_then(|s| s.to_str()) == Some("velin") {
                    files.push(path);
                }
            } else if path.is_dir() {
                for entry in WalkDir::new(&path) {
                    let entry = entry?;
                    if entry.file_type().is_file() {
                        if entry.path().extension().and_then(|s| s.to_str()) == Some("velin") {
                            files.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
            
            if files.is_empty() {
                eprintln!("Keine VelinScript-Dateien gefunden");
                return Ok(());
            }
            
            println!("🔍 Analysiere {} Datei(en)...\n", files.len());
            
            let mut all_issues = Vec::new();
            
            for file in &files {
                match fs::read_to_string(file) {
                    Ok(content) => {
                        let issues = linter.analyze(&content, file)?;
                        all_issues.extend(issues);
                    }
                    Err(e) => {
                        eprintln!("⚠️  Fehler beim Lesen von {}: {}", file.display(), e);
                    }
                }
            }
            
            if json {
                let json_output = serde_json::json!({
                    "issues": all_issues,
                    "total": all_issues.len()
                });
                println!("{}", serde_json::to_string_pretty(&json_output)?);
            } else {
                // Zeige Issues
                if all_issues.is_empty() {
                    println!("✓ Keine Probleme gefunden!");
                } else {
                    println!("📊 Gefundene Probleme: {}\n", all_issues.len());
                    
                    for issue in &all_issues {
                        println!("{}:{}:{} [{}] {}", 
                            issue.file,
                            issue.line,
                            issue.column,
                            issue.severity,
                            issue.message
                        );
                        if let Some(ref suggestion) = issue.suggestion {
                            println!("  💡 Vorschlag: {}", suggestion);
                        }
                    }
                    
                    if fix {
                        println!("\n🔧 Auto-fix wird in zukünftigen Versionen unterstützt");
                    }
                }
            }
            
            if !all_issues.is_empty() {
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
