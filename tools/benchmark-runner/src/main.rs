// VelinScript Benchmark Runner
// Führt Performance-Benchmarks aus mit statistischer Auswertung

mod runner;
mod parser;
mod stats;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use runner::BenchmarkRunner;

#[derive(Parser)]
#[command(name = "velin-bench")]
#[command(about = "VelinScript Benchmark Runner - Performance-Benchmarks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Führt Benchmarks aus
    Run {
        /// Benchmark-Datei oder Verzeichnis
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Anzahl Iterationen
        #[arg(short, long, default_value = "100")]
        iterations: usize,
        
        /// Vergleicht mit vorherigen Runs
        #[arg(short, long)]
        compare: bool,
        
        /// Output-Datei
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Verbose Output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path, iterations, compare, output, verbose } => {
            run_benchmarks(path, iterations, compare, output, verbose)
        }
    }
}

fn run_benchmarks(
    path: PathBuf,
    iterations: usize,
    compare: bool,
    output: Option<PathBuf>,
    verbose: bool,
) -> Result<()> {
    println!("⚡ Führe Benchmarks aus...\n");
    
    let runner = BenchmarkRunner::new(iterations, compare);
    let results = runner.run(&path, verbose)?;
    
    // Zeige Ergebnisse
    println!("\n📊 Benchmark-Ergebnisse:");
    for result in &results {
        println!("  {}: {:.2}ms ({} Iterationen)", 
            result.name, result.mean_time, result.iterations);
        if verbose {
            println!("    Min: {:.2}ms, Max: {:.2}ms, StdDev: {:.2}ms",
                result.min_time, result.max_time, result.std_dev);
        }
    }
    
    if let Some(output_path) = output {
        let json = serde_json::to_string_pretty(&results)?;
        std::fs::write(&output_path, json)?;
        println!("\n✓ Ergebnisse gespeichert: {}", output_path.display());
    }
    
    Ok(())
}
