// Standard Library für File I/O-Funktionalität
// Datei-Lese- und Schreib-Operationen

use std::fs;
use std::io::Write;
use std::path::Path;

/// File I/O Standard Library
pub struct FileIOStdlib;

impl FileIOStdlib {
    /// Liest eine Datei komplett ein
    pub fn read_file(path: &str) -> Result<String, String> {
        fs::read_to_string(path)
            .map_err(|e| format!("Fehler beim Lesen der Datei {}: {}", path, e))
    }
    
    /// Schreibt eine Datei komplett
    pub fn write_file(path: &str, content: &str) -> Result<(), String> {
        fs::write(path, content)
            .map_err(|e| format!("Fehler beim Schreiben der Datei {}: {}", path, e))
    }
    
    /// Hängt Text an eine Datei an
    pub fn append_file(path: &str, content: &str) -> Result<(), String> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("Fehler beim Öffnen der Datei {}: {}", path, e))?;
        
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Fehler beim Anhängen an Datei {}: {}", path, e))?;
        
        Ok(())
    }
    
    /// Prüft ob eine Datei existiert
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }
    
    /// Prüft ob ein Pfad ein Verzeichnis ist
    pub fn is_directory(path: &str) -> bool {
        Path::new(path).is_dir()
    }
    
    /// Prüft ob ein Pfad eine Datei ist
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }
    
    /// Löscht eine Datei
    pub fn delete_file(path: &str) -> Result<(), String> {
        fs::remove_file(path)
            .map_err(|e| format!("Fehler beim Löschen der Datei {}: {}", path, e))
    }
    
    /// Erstellt ein Verzeichnis
    pub fn create_directory(path: &str) -> Result<(), String> {
        fs::create_dir_all(path)
            .map_err(|e| format!("Fehler beim Erstellen des Verzeichnisses {}: {}", path, e))
    }
    
    /// Liest ein Verzeichnis
    pub fn read_directory(path: &str) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Fehler beim Lesen des Verzeichnisses {}: {}", path, e))?;
        
        let mut files = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| format!("Fehler beim Lesen des Eintrags: {}", e))?;
            if let Some(name) = entry.path().file_name() {
                files.push(name.to_string_lossy().to_string());
            }
        }
        
        Ok(files)
    }
    
    /// Generiert Rust-Code für file.read()
    pub fn generate_read_code(path: &str) -> String {
        format!("fileio::read_file(\"{}\")", path)
    }
    
    /// Generiert Rust-Code für file.write()
    pub fn generate_write_code(path: &str, content: &str) -> String {
        format!("fileio::write_file(\"{}\", {})", path, content)
    }
    
    /// Generiert Rust-Code für file.append()
    pub fn generate_append_code(path: &str, content: &str) -> String {
        format!("fileio::append_file(\"{}\", {})", path, content)
    }
    
    /// Generiert Rust-Code für file.exists()
    pub fn generate_exists_code(path: &str) -> String {
        format!("fileio::file_exists(\"{}\")", path)
    }
    
    /// Liste der verfügbaren File I/O-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "file.read".to_string(),
                signature: "fn(string) -> Result<string, string>".to_string(),
            },
            FunctionInfo {
                name: "file.write".to_string(),
                signature: "fn(string, string) -> Result<(), string>".to_string(),
            },
            FunctionInfo {
                name: "file.append".to_string(),
                signature: "fn(string, string) -> Result<(), string>".to_string(),
            },
            FunctionInfo {
                name: "file.exists".to_string(),
                signature: "fn(string) -> bool".to_string(),
            },
            FunctionInfo {
                name: "file.delete".to_string(),
                signature: "fn(string) -> Result<(), string>".to_string(),
            },
            FunctionInfo {
                name: "file.isDirectory".to_string(),
                signature: "fn(string) -> bool".to_string(),
            },
            FunctionInfo {
                name: "file.isFile".to_string(),
                signature: "fn(string) -> bool".to_string(),
            },
            FunctionInfo {
                name: "file.createDirectory".to_string(),
                signature: "fn(string) -> Result<(), string>".to_string(),
            },
            FunctionInfo {
                name: "file.readDirectory".to_string(),
                signature: "fn(string) -> Result<Vec<string>, string>".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
