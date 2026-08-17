// VelinScript Dependency Graph
// Visualisiert Modul-Abhängigkeiten und erkennt zirkuläre Imports

mod analyzer;
mod graph;
mod visualizer;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use analyzer::DependencyAnalyzer;
use graph::DependencyGraph;
use visualizer::GraphVisualizer;

#[derive(Parser)]
#[command(name = "velin-deps")]
#[command(about = "VelinScript Dependency Graph - Visualisiert Modul-Abhängigkeiten", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generiert Dependency-Graph
    Graph {
        /// Eingabe-Datei oder Verzeichnis
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Output-Format (svg, dot, json)
        #[arg(short, long, default_value = "svg")]
        format: String,
        
        /// Zeigt nur zirkuläre Abhängigkeiten
        #[arg(short, long)]
        circular: bool,
        
        /// Output-Datei
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Graph { path, format, circular, output } => {
            graph_command(path, format, circular, output)
        }
    }
}

fn graph_command(path: PathBuf, format: String, circular_only: bool, output: Option<PathBuf>) -> Result<()> {
    println!("🔍 Analysiere Dependencies...\n");
    
    let analyzer = DependencyAnalyzer::new();
    let graph = analyzer.analyze(&path)?;
    
    // Prüfe auf zirkuläre Abhängigkeiten
    let circular_deps = graph.find_circular_dependencies();
    
    if !circular_deps.is_empty() {
        println!("⚠️  {} zirkuläre Abhängigkeit(en) gefunden:\n", circular_deps.len());
        for cycle in &circular_deps {
            println!("  {}", cycle.join(" → "));
        }
        println!();
    } else {
        println!("✓ Keine zirkulären Abhängigkeiten gefunden\n");
    }
    
    if circular_only && circular_deps.is_empty() {
        println!("Keine zirkulären Abhängigkeiten zum Anzeigen.");
        return Ok(());
    }
    
    let visualizer = GraphVisualizer::new();
    let output_content = match format.as_str() {
        "svg" => visualizer.to_svg(&graph, circular_only)?,
        "dot" => visualizer.to_dot(&graph, circular_only)?,
        "json" => visualizer.to_json(&graph, circular_only)?,
        _ => return Err(anyhow::anyhow!("Unbekanntes Format: {}. Unterstützt: svg, dot, json", format)),
    };
    
    if let Some(output_path) = output {
        std::fs::write(&output_path, output_content)?;
        println!("✓ Graph gespeichert: {}", output_path.display());
    } else {
        println!("{}", output_content);
    }
    
    if !circular_deps.is_empty() {
        std::process::exit(1);
    }
    
    Ok(())
}
