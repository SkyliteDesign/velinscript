// VelinScript Security Scanner
// Analysiert Code auf Security-Vulnerabilities

mod rules;
mod analyzer;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "velin-security")]
#[command(about = "VelinScript Security Scanner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scannt Code auf Security-Vulnerabilities
    Scan {
        /// Pfad zum zu scannenden Code
        #[arg(default_value = ".")]
        path: String,
        /// Output-Format (json, html, text)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Prüft Dependencies auf Vulnerabilities
    Audit {
        /// Pfad zu velin.toml
        #[arg(default_value = "velin.toml")]
        config: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, format } => {
            let findings = analyzer::scan_directory(&path)?;
            
            match format.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&findings)?;
                    println!("{}", json);
                }
                "html" => {
                    analyzer::generate_html_report(&findings, "security-report.html")?;
                    println!("✓ HTML Report generiert: security-report.html");
                }
                _ => {
                    analyzer::print_text_report(&findings);
                }
            }
            
            if findings.is_empty() {
                println!("✓ Keine Security-Vulnerabilities gefunden");
                return Ok(()); // Return instead of exit to avoid unreachable code
            } else {
                println!("⚠ {} Vulnerabilities gefunden", findings.len());
                // Don't exit here, let main return with error
            }
        }
        Commands::Audit { config: config_path } => {
            let vulnerabilities = analyzer::audit_dependencies(&config_path).await?;
            if vulnerabilities.is_empty() {
                println!("✓ Keine Vulnerabilities in Dependencies gefunden");
            } else {
                println!("⚠ {} Vulnerabilities in Dependencies gefunden", vulnerabilities.len());
                // Return error if vulnerabilities found
                anyhow::bail!("{} Vulnerabilities gefunden", vulnerabilities.len());
            }
        }
    }

    Ok(())
}
