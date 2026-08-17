// VelinScript Package Manager
// Verwaltet Dependencies und Packages

mod registry;
mod resolver;
mod lockfile;
mod install;
mod updater;

use registry::Registry;
use updater::DependencyUpdater;

use clap::{Parser, Subcommand};
use anyhow::Result;
use toml;

#[derive(Parser)]
#[command(name = "velin-pkg")]
#[command(about = "VelinScript Package Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialisiert ein neues Projekt
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Fügt eine Dependency hinzu
    Add {
        /// Package Name (z.B. github.com/user/repo)
        package: String,
        /// Version (optional, z.B. ^1.0.0)
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Entfernt eine Dependency
    Remove {
        /// Package Name
        package: String,
    },
    /// Installiert alle Dependencies
    Install,
    /// Aktualisiert Dependencies
    Update {
        /// Package Name (optional, wenn nicht angegeben werden Updates geprüft)
        package: Option<String>,
        /// Aktualisiert alle Dependencies
        #[arg(long)]
        all: bool,
        /// Erlaubt Breaking Changes
        #[arg(long)]
        allow_breaking: bool,
    },
    /// Zeigt installierte Packages
    List,
    /// Veröffentlicht ein Package
    Publish {
        /// Version (z.B. 1.0.0)
        version: String,
    },
    /// Prüft Dependencies auf Vulnerabilities
    Audit,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            install::init_project(name.as_deref())?;
            println!("✓ Projekt initialisiert");
        }
        Commands::Add { package, version } => {
            resolver::add_dependency(&package, version.as_deref()).await?;
            println!("✓ Dependency hinzugefügt: {}", package);
        }
        Commands::Remove { package } => {
            resolver::remove_dependency(&package)?;
            println!("✓ Dependency entfernt: {}", package);
        }
        Commands::Install => {
            install::install_dependencies().await?;
            println!("✓ Dependencies installiert");
        }
        Commands::Update { package, all, allow_breaking } => {
            let toml_path = std::path::Path::new("velin.toml");
            if !toml_path.exists() {
                anyhow::bail!("velin.toml nicht gefunden");
            }
            
            let content = std::fs::read_to_string(toml_path)?;
            let mut config: resolver::VelinToml = toml::from_str(&content)?;
            
            if all {
                // Update alle Dependencies
                let updated = DependencyUpdater::update_all(&mut config, allow_breaking).await?;
                if updated.is_empty() {
                    println!("✓ Keine Updates verfügbar");
                }
            } else if let Some(pkg) = package {
                // Update spezifisches Package
                DependencyUpdater::update_dependency(&mut config, &pkg, None).await?;
            } else {
                // Check for updates
                let updates = DependencyUpdater::check_updates(&config).await?;
                if updates.is_empty() {
                    println!("✓ Alle Dependencies sind aktuell");
                } else {
                    println!("Verfügbare Updates:");
                    for update in &updates {
                        if update.breaking {
                            println!("  ⚠ {}: {} -> {} (Breaking Change)", update.package, update.current, update.latest);
                        } else {
                            println!("  ✓ {}: {} -> {}", update.package, update.current, update.latest);
                        }
                    }
                    println!("\nFühre 'velin-pkg update --all' aus um alle zu aktualisieren");
                }
            }
            
            // Speichere aktualisierte Config
            let toml_content = toml::to_string_pretty(&config)?;
            std::fs::write(toml_path, toml_content)?;
            
            println!("✓ Dependencies aktualisiert");
        }
        Commands::List => {
            let deps = resolver::list_dependencies()?;
            if deps.is_empty() {
                println!("Keine Dependencies gefunden");
            } else {
                println!("Installierte Dependencies:");
                for (name, version) in deps {
                    println!("  {} {}", name, version);
                }
            }
        }
        Commands::Publish { version } => {
            Registry::publish_package(&version).await?;
            println!("✓ Package veröffentlicht: v{}", version);
        }
        Commands::Audit => {
            let vulnerabilities = resolver::audit_dependencies().await?;
            if vulnerabilities.is_empty() {
                println!("✓ Keine Vulnerabilities gefunden");
            } else {
                println!("⚠ Gefundene Vulnerabilities:");
                for vuln in vulnerabilities {
                    println!("  - {}: {}", vuln.package, vuln.description);
                }
            }
        }
    }

    Ok(())
}
