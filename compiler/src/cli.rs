use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "velin")]
#[command(about = "VelinScript Compiler - Eine moderne Programmiersprache für KI-APIs")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Kompiliert eine VelinScript Datei zu Rust
    Compile {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Ausgabe-Datei (.rs)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Überspringe Type Checking
        #[arg(long)]
        no_type_check: bool,
        
        /// Zeige generierten Code in der Konsole
        #[arg(long)]
        show_code: bool,
    },
    
    /// Prüft eine VelinScript Datei (nur Parsing & Type Checking)
    Check {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Formatiert eine VelinScript Datei
    Format {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Überschreibe die Datei
        #[arg(long)]
        in_place: bool,
    },
    
    /// Zeigt Informationen über eine VelinScript Datei
    Info {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Initialisiert ein neues VelinScript Projekt
    Init {
        /// Projekt-Name
        name: Option<String>,
        
        /// Erstelle im aktuellen Verzeichnis
        #[arg(long)]
        current_dir: bool,
    },
    
    /// Generiert OpenAPI Specification
    OpenAPI {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Ausgabe-Datei (.json oder .yaml)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Generiert Code (Boilerplate, CRUD, etc.)
    Generate {
        /// Art der Generierung (api, crud, test, client)
        #[arg(value_name = "TYPE")]
        gen_type: String,
        
        /// Name/Modell für die Generierung
        #[arg(short, long)]
        name: Option<String>,
        
        /// Felder (für CRUD)
        #[arg(short, long)]
        fields: Option<String>,
        
        /// Pfad (für API)
        #[arg(short, long)]
        path: Option<String>,
        
        /// OpenAPI Datei (für Client)
        #[arg(short, long)]
        openapi: Option<PathBuf>,
        
        /// Ausgabe-Sprache (für Client)
        #[arg(short, long)]
        language: Option<String>,
        
        /// Ausgabe-Datei
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Führt Tests aus (Unit + Integration)
    Test {
        /// Test-Verzeichnis
        #[arg(short, long)]
        directory: Option<PathBuf>,
        
        /// Nur Unit Tests
        #[arg(long)]
        unit: bool,
        
        /// Nur Integration Tests
        #[arg(long)]
        integration: bool,
        
        /// Verbose Output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Verwaltet velin.config.json
    Config {
        /// Subcommand
        #[command(subcommand)]
        subcommand: ConfigCommands,
    },
    
    /// Cache-Management
    Cache {
        /// Subcommand
        #[command(subcommand)]
        subcommand: CacheCommands,
    },
    
    /// Health Check
    Health {
        /// Endpoint-URL
        #[arg(short, long)]
        url: Option<String>,
        
        /// Zeige detaillierte Metriken
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Backup-Management
    Backup {
        /// Subcommand
        #[command(subcommand)]
        subcommand: BackupCommands,
    },
    
    /// Rollback-Management
    Rollback {
        /// Subcommand
        #[command(subcommand)]
        subcommand: RollbackCommands,
    },
    
    /// Serialization-Tools
    Serialize {
        /// Subcommand
        #[command(subcommand)]
        subcommand: SerializeCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialisiert velin.config.json
    Init {
        /// Verwende Beispiel-Config
        #[arg(long)]
        example: bool,
    },
    
    /// Validiert velin.config.json
    Validate {
        /// Config-Datei
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    
    /// Zeigt Config-Werte
    Show {
        /// Config-Datei
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Zeigt Cache-Statistiken
    Stats,
    
    /// Leert Cache
    Clear {
        /// Pattern für Keys
        pattern: Option<String>,
    },
    
    /// Wärmt Cache
    Warm,
}

#[derive(Subcommand)]
pub enum BackupCommands {
    /// Erstellt ein Backup
    Create {
        /// Backup-Strategie (full, incremental)
        #[arg(short, long)]
        strategy: Option<String>,
        
        /// Ziel-Verzeichnis
        #[arg(short, long)]
        destination: Option<String>,
        
        /// Komprimierung (gzip, zip, none)
        #[arg(short, long)]
        compression: Option<String>,
    },
    
    /// Stellt ein Backup wieder her
    Restore {
        /// Backup-ID
        backup_id: String,
        
        /// Ziel-Verzeichnis
        #[arg(short, long)]
        destination: Option<String>,
    },
    
    /// Listet alle Backups auf
    List {
        /// Verzeichnis mit Backups
        #[arg(short, long)]
        directory: Option<String>,
    },
    
    /// Löscht ein Backup
    Delete {
        /// Backup-ID
        backup_id: String,
        
        /// Verzeichnis mit Backups
        #[arg(short, long)]
        directory: Option<String>,
    },
    
    /// Verifiziert ein Backup
    Verify {
        /// Backup-ID
        backup_id: String,
        
        /// Verzeichnis mit Backups
        #[arg(short, long)]
        directory: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum RollbackCommands {
    /// Beginnt eine Transaktion
    Begin,
    
    /// Committet eine Transaktion
    Commit {
        /// Transaktions-ID
        transaction_id: String,
    },
    
    /// Rollback einer Transaktion
    Rollback {
        /// Transaktions-ID
        transaction_id: String,
    },
    
    /// Erstellt eine Version
    CreateVersion {
        /// Beschreibung
        description: String,
    },
    
    /// Rollback zu einer Version
    ToVersion {
        /// Version-ID
        version_id: String,
    },
    
    /// Listet alle Versionen auf
    ListVersions,
    
    /// Erstellt einen Snapshot
    CreateSnapshot {
        /// Beschreibung
        description: String,
    },
    
    /// Rollback zu einem Snapshot
    ToSnapshot {
        /// Snapshot-ID
        snapshot_id: String,
    },
    
    /// Listet alle Snapshots auf
    ListSnapshots,
}

#[derive(Subcommand)]
pub enum SerializeCommands {
    /// Konvertiert JSON zu YAML
    JsonToYaml {
        /// Eingabe-Datei
        #[arg(short, long)]
        input: PathBuf,
        
        /// Ausgabe-Datei
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Konvertiert YAML zu JSON
    YamlToJson {
        /// Eingabe-Datei
        #[arg(short, long)]
        input: PathBuf,
        
        /// Ausgabe-Datei
        #[arg(short, long)]
        output: PathBuf,
    },
    
    /// Validiert JSON
    ValidateJson {
        /// Datei
        #[arg(short, long)]
        file: PathBuf,
    },
    
    /// Validiert YAML
    ValidateYaml {
        /// Datei
        #[arg(short, long)]
        file: PathBuf,
    },
}
