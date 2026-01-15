// VelinScript Dead Code Detector
// Findet ungenutzten Code automatisch

mod analyzer;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "velin-dead-code")]
#[command(about = "VelinScript Dead Code Detector", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scannt Code auf Dead Code
    Scan {
        /// Pfad zum zu scannenden Code
        #[arg(default_value = ".")]
        path: String,
        /// Entfernt Dead Code automatisch
        #[arg(short, long)]
        fix: bool,
        /// Generiert JSON Report
        #[arg(short, long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, fix, json } => {
            let findings = analyzer::scan_directory(&path)?;
            
            if findings.is_empty() {
                println!("✓ Kein Dead Code gefunden");
                return Ok(()); // Return instead of exit to avoid unreachable code
            }
            
            if json {
                let json_output = serde_json::to_string_pretty(&findings)?;
                println!("{}", json_output);
            } else {
                analyzer::print_report(&findings);
                
                if fix {
                    println!("\n⚠ Auto-Fix wird noch nicht unterstützt");
                    println!("Bitte entferne Dead Code manuell basierend auf dem Report");
                }
            }
            
            // Return error if dead code found
            anyhow::bail!("{} Dead Code Einträge gefunden", findings.len());
        }
    }
}
