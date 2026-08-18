use clap::{Parser, Subcommand};
use std::path::PathBuf;

// Velisch Identity - Fingerabdruck in CLI
// Diese Imports dienen als Fingerabdruck und werden absichtlich nicht direkt verwendet

#[derive(Parser)]
#[command(name = "velin")]
#[command(about = "Velisch Compiler - Eine moderne Programmiersprache für KI-APIs")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Kompiliert eine Velisch Datei zu Rust
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
        
        /// Automatische Fehlerkorrektur aktivieren
        #[arg(long)]
        autofix: bool,
        
        /// KI-Semantik-Analyse aktivieren
        #[arg(long)]
        ai_semantic: bool,
        
        /// KI-Bug-Erkennung aktivieren
        #[arg(long)]
        ai_bug_detection: bool,
        
        /// KI-Code-Generierung aktivieren
        #[arg(long)]
        ai_codegen: bool,
        
        /// KI-Optimierung aktivieren
        #[arg(long)]
        ai_optimization: bool,
        
        /// AI Provider (openai, anthropic, gemini, local)
        #[arg(long)]
        ai_provider: Option<String>,
        
        /// AI API Key
        #[arg(long)]
        ai_api_key: Option<String>,

        /// Ziel-Sprache (rust, php, python, etc.)
        #[arg(long, default_value = "rust")]
        target: String,

        /// Web Framework (laravel, symfony, fastapi, flask, axum, actix)
        #[arg(long)]
        framework: Option<String>,

        /// Codegen path: `ir` (default, productive) or `ast-legacy` (debug/compare only)
        #[arg(long, default_value = "ir")]
        codegen: String,
    },
    
    /// Prüft eine Velisch Datei (nur Parsing & Type Checking)
    Check {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,
        
        /// Automatische Fehlerkorrektur aktivieren
        #[arg(long)]
        autofix: bool,
    },
    
    /// Formatiert eine Velisch Datei
    Format {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,

        /// Überschreibe die Datei
        #[arg(long)]
        in_place: bool,
    },
    
    /// Zeigt Informationen über eine Velisch Datei
    Info {
        /// Eingabe-Datei (.velin)
        #[arg(short, long)]
        input: PathBuf,
    },
    
    /// Initialisiert ein neues Velisch Projekt
    Init {
        /// Projekt-Name
        name: Option<String>,

        /// Erstelle im aktuellen Verzeichnis
        #[arg(long)]
        current_dir: bool,
    },
    
    /// Alias für `init` - Erstellt ein neues Velisch Projekt
    New {
        /// Projekt-Name
        name: Option<String>,
        
        /// Erstelle im aktuellen Verzeichnis
        #[arg(long)]
        current_dir: bool,
    },
    
    /// Schreibt ein Axum-Scaffold unter .velin/serve-scaffold (startet den Server nicht)
    Serve {
        /// Eingabe-Datei (.velin; Lese-Alias .vel)
        #[arg(short, long)]
        input: Option<PathBuf>,
        
        /// Port (Standard: 8080)
        #[arg(short, long, default_value = "8080")]
        port: u16,
        
        /// Host (Standard: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Watch-Mode (nicht implementiert; ehrliche Meldung, kein Reload)
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Kompiliert die Quelle und startet den Axum-Server (Default-Port 8080)
    Run {
        /// Eingabe-Datei (.velin; Lese-Alias .vel)
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        /// Eingabe-Datei (alternative zu FILE)
        #[arg(short, long)]
        input: Option<PathBuf>,
        
        /// Port (Standard: 8080)
        #[arg(short, long, default_value = "8080")]
        port: u16,
        
        /// Host, an den gebunden wird (Standard: 127.0.0.1)
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        
        /// Watch-Mode (nicht implementiert; ehrliche Meldung, kein Reload)
        #[arg(short, long)]
        watch: bool,
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
        #[arg(long)]
        openapi: Option<PathBuf>,

        /// Ausgabe-Sprache (für Client)
        #[arg(short, long)]
        language: Option<String>,

        /// Ausgabe-Datei
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Parse Velin test files under tests/ (full runner: use velin-test-runner)
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
    
    /// Cache management (requires generated runtime; hidden)
    #[command(hide = true)]
    Cache {
        /// Subcommand
        #[command(subcommand)]
        subcommand: CacheCommands,
    },
    
    /// Health check against a running service (hidden)
    #[command(hide = true)]
    Health {
        /// Endpoint-URL
        #[arg(short, long)]
        url: Option<String>,

        /// Zeige detaillierte Metriken
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Projekt-Dateien unter .velin/backup sichern
    Backup {
        /// Subcommand
        #[command(subcommand)]
        subcommand: BackupCommands,
    },
    
    /// Rollback helpers (hidden until filesystem ops are complete)
    #[command(hide = true)]
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
