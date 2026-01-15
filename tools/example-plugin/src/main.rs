// VelinScript Example Plugin
// Ein Beispiel-Plugin, das Code-Metriken analysiert

mod metrics;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use velin_compiler::parser::parser::Parser as VelinParser;
use walkdir::WalkDir;
use metrics::CodeMetrics;

#[derive(Parser)]
#[command(name = "velin-example-plugin")]
#[command(about = "VelinScript Example Plugin - Code Metrics Analyzer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analysiert Code-Metriken
    Metrics {
        /// Eingabe-Datei oder Verzeichnis
        #[arg(short, long, default_value = ".")]
        input: PathBuf,
        
        /// Output-Format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Verbose Output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Metrics { input, format, verbose } => {
            metrics_command(input, format, verbose)
        }
    }
}

fn metrics_command(input: PathBuf, format: String, verbose: bool) -> Result<()> {
    println!("📊 Analysiere Code-Metriken: {}\n", input.display());
    
    let files = collect_velin_files(&input)?;
    
    if files.is_empty() {
        eprintln!("⚠️  Keine VelinScript-Dateien gefunden");
        return Ok(());
    }
    
    if verbose {
        println!("📁 Gefundene Dateien: {}\n", files.len());
    }
    
    let mut total_metrics = CodeMetrics::new();
    
    for file in &files {
        if verbose {
            println!("📝 Analysiere: {}", file.display());
        }
        
        match std::fs::read_to_string(file) {
            Ok(content) => {
                match VelinParser::parse(&content) {
                    Ok(program) => {
                        let file_metrics = metrics::analyze_program(&program);
                        total_metrics.merge(&file_metrics);
                        
                        if verbose {
                            println!("  ✓ {} Funktionen, {} Structs, {} Enums",
                                file_metrics.function_count,
                                file_metrics.struct_count,
                                file_metrics.enum_count
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("  ⚠️  Parse-Fehler in {}: {}", file.display(), e.message);
                    }
                }
            }
            Err(e) => {
                eprintln!("  ⚠️  Fehler beim Lesen von {}: {}", file.display(), e);
            }
        }
    }
    
    // Output generieren
    match format.as_str() {
        "json" => {
            let json_output = serde_json::json!({
                "files_analyzed": files.len(),
                "metrics": {
                    "functions": total_metrics.function_count,
                    "structs": total_metrics.struct_count,
                    "enums": total_metrics.enum_count,
                    "total_lines": total_metrics.total_lines,
                    "avg_function_length": total_metrics.avg_function_length(),
                }
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
        _ => {
            println!("\n📊 Code-Metriken:");
            println!("  📁 Analysierte Dateien: {}", files.len());
            println!("  📝 Funktionen: {}", total_metrics.function_count);
            println!("  🏗️  Structs: {}", total_metrics.struct_count);
            println!("  📦 Enums: {}", total_metrics.enum_count);
            println!("  📏 Gesamt-Zeilen: {}", total_metrics.total_lines);
            
            if total_metrics.function_count > 0 {
                println!("  📈 Durchschnittliche Funktion-Länge: {:.1} Zeilen",
                    total_metrics.avg_function_length()
                );
            }
        }
    }
    
    Ok(())
}

fn collect_velin_files(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) == Some("velin") {
            files.push(path.clone());
        }
    } else if path.is_dir() {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("velin") {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    
    Ok(files)
}
