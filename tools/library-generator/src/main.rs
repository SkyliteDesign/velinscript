use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod config;
mod generator;
mod integration;
mod templates;

use config::LibraryConfig;
use generator::LibraryGenerator;

#[derive(Parser)]
#[command(name = "velin-library-generator")]
#[command(about = "Generiert neue Standardbibliotheks-Module für VelinScript")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generiere eine neue Standardbibliothek
    Generate {
        /// Name des Moduls (z.B. "graphql")
        #[arg(short, long)]
        name: Option<String>,
        
        /// Beschreibung des Moduls
        #[arg(short, long)]
        description: Option<String>,
        
        /// Konfigurationsdatei (YAML)
        #[arg(short, long)]
        config: Option<PathBuf>,
        
        /// Interaktiver Modus
        #[arg(short, long)]
        interactive: bool,
    },
    
    /// Validiere eine Konfigurationsdatei
    Validate {
        /// Konfigurationsdatei (YAML)
        #[arg(short, long)]
        config: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate { name, description, config, interactive } => {
            let library_config = if let Some(config_path) = config {
                // Lade aus Konfigurationsdatei
                LibraryConfig::from_file(&config_path)?
            } else if interactive {
                // Interaktiver Modus
                LibraryConfig::interactive()?
            } else if let Some(name) = name {
                // Basis-Konfiguration aus Parametern
                LibraryConfig::from_params(name, description)?
            } else {
                eprintln!("Fehler: Bitte geben Sie --name, --config oder --interactive an");
                std::process::exit(1);
            };
            
            // Validiere Konfiguration
            library_config.validate()?;
            
            // Generiere Bibliothek
            let generator = LibraryGenerator::new();
            generator.generate(&library_config)?;
        }
        
        Commands::Validate { config } => {
            let library_config = LibraryConfig::from_file(&config)?;
            library_config.validate()?;
            println!("✅ Konfiguration ist gültig!");
        }
    }
    
    Ok(())
}
