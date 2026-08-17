// VelinScript Runtime Inspector
// Live-Inspection von Variablen, State und Memory während der Ausführung

mod inspector;
mod variables;
mod memory;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use inspector::RuntimeInspector;

#[derive(Parser)]
#[command(name = "velin-inspect")]
#[command(about = "VelinScript Runtime Inspector - Live-Inspection von Variablen und Memory", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Startet Inspector
    Inspect {
        /// Zu inspizierende Datei
        file: PathBuf,
        
        /// Live-Monitoring
        #[arg(short, long)]
        watch: bool,
        
        /// Zeigt alle Variablen
        #[arg(short, long)]
        variables: bool,
        
        /// Zeigt Memory-Usage
        #[arg(short, long)]
        memory: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { file, watch, variables, memory } => {
            inspect_command(file, watch, variables, memory).await
        }
    }
}

async fn inspect_command(
    file: PathBuf,
    watch: bool,
    show_variables: bool,
    show_memory: bool,
) -> Result<()> {
    println!("🔍 Runtime Inspector für: {}\n", file.display());
    
    let inspector = RuntimeInspector::new();
    
    if watch {
        println!("👀 Watch-Mode aktiviert (CTRL-C zum Beenden)\n");
        inspector.watch(&file, show_variables, show_memory).await?;
    } else {
        inspector.inspect(&file, show_variables, show_memory).await?;
    }
    
    Ok(())
}
