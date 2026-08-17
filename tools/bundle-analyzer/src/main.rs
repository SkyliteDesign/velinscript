// VelinScript Bundle Analyzer
// Analysiert Bundle-Größe, Tree-Shaking-Potenzial und Code-Splitting-Möglichkeiten

mod analyzer;
mod tree_shaking;
mod report;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use analyzer::BundleAnalyzer;
use report::ReportGenerator;

#[derive(Parser)]
#[command(name = "velin-bundle")]
#[command(about = "VelinScript Bundle Analyzer - Analysiert Bundle-Größe und Tree-Shaking-Potenzial", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analysiert Bundle-Größe
    Analyze {
        /// Eingabe-Datei oder Verzeichnis
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Zeigt Tree-Shaking-Potenzial
        #[arg(long)]
        tree_shaking: bool,
        
        /// Zeigt Code-Splitting-Vorschläge
        #[arg(long)]
        code_splitting: bool,
        
        /// Output-Datei
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// JSON-Output
        #[arg(short, long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path, tree_shaking, code_splitting, output, json } => {
            analyze_command(path, tree_shaking, code_splitting, output, json)
        }
    }
}

fn analyze_command(
    path: PathBuf,
    tree_shaking: bool,
    code_splitting: bool,
    output: Option<PathBuf>,
    json: bool,
) -> Result<()> {
    println!("📦 Analysiere Bundle-Größe...\n");
    
    let analyzer = BundleAnalyzer::new();
    let analysis = analyzer.analyze(&path)?;
    
    let report_gen = ReportGenerator::new();
    let report = report_gen.generate(&analysis, tree_shaking, code_splitting)?;
    
    if json {
        let json_output = serde_json::to_string_pretty(&analysis)?;
        if let Some(output_path) = output {
            std::fs::write(&output_path, json_output)?;
            println!("✓ Report gespeichert: {}", output_path.display());
        } else {
            println!("{}", json_output);
        }
    } else {
        if let Some(output_path) = output {
            std::fs::write(&output_path, report)?;
            println!("✓ Report gespeichert: {}", output_path.display());
        } else {
            println!("{}", report);
        }
    }
    
    Ok(())
}
