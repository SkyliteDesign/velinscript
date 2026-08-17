// VelinScript Test Runner
// Führt Unit- und Integrationstests aus mit Assertions, Mocking und Coverage-Reports

mod runner;
mod parser;
mod coverage;
mod mocking;
mod assertions;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use runner::TestRunner;

#[derive(Parser)]
#[command(name = "velin-test")]
#[command(about = "VelinScript Test Runner - Unit- und Integrationstests", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Führt alle Tests aus
    Run {
        /// Test-Datei oder Verzeichnis
        #[arg(default_value = ".")]
        path: PathBuf,
        
        /// Führt nur Unit-Tests aus
        #[arg(short, long)]
        unit: bool,
        
        /// Führt nur Integration-Tests aus
        #[arg(short, long)]
        integration: bool,
        
        /// Generiert Coverage-Report
        #[arg(short, long)]
        coverage: bool,
        
        /// Aktiviert Mocking
        #[arg(short, long)]
        mock: bool,
        
        /// Verbose Output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { path, unit, integration, coverage, mock, verbose } => {
            run_tests(path, unit, integration, coverage, mock, verbose).await
        }
    }
}

async fn run_tests(
    path: PathBuf,
    unit: bool,
    integration: bool,
    coverage: bool,
    mock: bool,
    verbose: bool,
) -> Result<()> {
    println!("🧪 Führe Tests aus...\n");
    
    let runner = TestRunner::new(coverage, mock);
    let results = runner.run(&path, unit, integration, verbose).await?;
    
    // Zeige Ergebnisse
    println!("\n📊 Test-Ergebnisse:");
    println!("  ✓ Bestanden: {}", results.passed);
    println!("  ✗ Fehlgeschlagen: {}", results.failed);
    println!("  ⏭️  Übersprungen: {}", results.skipped);
    
    if !results.failures.is_empty() {
        println!("\n❌ Fehlgeschlagene Tests:");
        for failure in &results.failures {
            println!("  - {}: {}", failure.test_name, failure.message);
        }
    }
    
    if coverage {
        println!("\n📈 Coverage-Report:");
        if let Some(ref coverage_data) = results.coverage {
            println!("  Zeilen-Coverage: {:.2}%", coverage_data.line_coverage);
            println!("  Funktionen-Coverage: {:.2}%", coverage_data.function_coverage);
        }
    }
    
    if results.failed > 0 {
        std::process::exit(1);
    }
    
    Ok(())
}
