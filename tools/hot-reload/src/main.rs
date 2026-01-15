// VelinScript Hot Reload Tool
// Überwacht Dateiänderungen und lädt automatisch neu

mod watcher;
mod reloader;

use clap::{Parser, ArgGroup};
use anyhow::Result;
use std::path::PathBuf;
use watcher::FileWatcher;
use reloader::Reloader;

#[derive(Parser)]
#[command(name = "velin-hot-reload")]
#[command(about = "VelinScript Hot Reload - Automatisches Neuladen bei Änderungen", long_about = None)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["watch", "server"])
))]
struct Cli {
    /// Überwacht Dateien und kompiliert bei Änderungen
    #[arg(short, long)]
    watch: bool,
    
    /// Startet Development Server mit Hot Reload
    #[arg(short, long)]
    server: bool,
    
    /// Verzeichnis zu überwachen
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
    
    /// Port für Development Server
    #[arg(short, long, default_value = "3000")]
    port: u16,
    
    /// Kompilier-Befehl
    #[arg(long, default_value = "velin-compiler compile")]
    compile_command: String,
    
    /// Start-Befehl für Server
    #[arg(long, default_value = "cargo run")]
    run_command: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if cli.watch {
        // Watch Mode: Überwacht Dateien und kompiliert
        let mut watcher = FileWatcher::new(&cli.directory)?;
        let reloader = Reloader::new(&cli.compile_command);
        
        println!("🔍 Überwache Verzeichnis: {}", cli.directory.display());
        println!("📝 Kompiliere bei Änderungen...\n");
        
        loop {
            if let Some(changed_files) = watcher.wait_for_changes().await? {
                println!("📝 Änderungen erkannt in:");
                for file in &changed_files {
                    println!("  - {}", file.display());
                }
                
                println!("🔨 Kompiliere...");
                if let Err(e) = reloader.reload(&changed_files).await {
                    eprintln!("❌ Kompilierungsfehler: {}", e);
                } else {
                    println!("✓ Kompilierung erfolgreich\n");
                }
            }
        }
    } else if cli.server {
        // Server Mode: Startet Server mit Hot Reload
        let mut watcher = FileWatcher::new(&cli.directory)?;
        let reloader = Reloader::new(&cli.run_command);
        
        println!("🚀 Starte Development Server auf Port {}...", cli.port);
        println!("🔍 Überwache Verzeichnis: {}", cli.directory.display());
        println!("📝 Neustart bei Änderungen...\n");
        
        // Starte Server initial
        let mut server_handle = reloader.start_server().await?;
        
        loop {
            if let Some(changed_files) = watcher.wait_for_changes().await? {
                println!("📝 Änderungen erkannt in:");
                for file in &changed_files {
                    println!("  - {}", file.display());
                }
                
                println!("🔄 Neustarte Server...");
                server_handle = reloader.restart_server(server_handle).await?;
                println!("✓ Server neu gestartet\n");
            }
        }
    }
    
    Ok(())
}
