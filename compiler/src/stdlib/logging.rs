// Standard Library für Logging-Funktionalität
// Logging-Funktionen für verschiedene Log-Level

use chrono::Utc;
use serde_json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Log-Level für verschiedene Detaillierungsgrade
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Gibt den String-Namen des Log-Levels zurück
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Velin Logger mit erweiterten Features
pub struct VelinLogger {
    level: LogLevel,
    output: Box<dyn Write + Send>,
    context: HashMap<String, String>,
    file_output: Option<File>,
    json_format: bool,
    rotation_enabled: bool,
    max_file_size: u64,
    current_file_size: u64,
}

/// Logger-Struktur (Legacy, für Rückwärtskompatibilität)
pub struct Logger {
    level: LogLevel,
    output: Box<dyn Write + Send>,
}

impl Logger {
    /// Erstellt einen neuen Logger mit Standard-Level (Info)
    pub fn new() -> Self {
        Logger {
            level: LogLevel::Info,
            output: Box::new(io::stdout()),
        }
    }

    /// Erstellt einen Logger mit spezifischem Level
    pub fn with_level(level: LogLevel) -> Self {
        Logger {
            level,
            output: Box::new(io::stdout()),
        }
    }

    /// Setzt das Log-Level
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    /// Loggt eine Nachricht auf Trace-Level
    pub fn trace(&mut self, message: &str) {
        self.log(LogLevel::Trace, message);
    }

    /// Loggt eine Nachricht auf Debug-Level
    pub fn debug(&mut self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Loggt eine Nachricht auf Info-Level
    pub fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Loggt eine Nachricht auf Warn-Level
    pub fn warn(&mut self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Loggt eine Nachricht auf Error-Level
    pub fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Interne Log-Funktion
    fn log(&mut self, level: LogLevel, message: &str) {
        if level >= self.level {
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let log_line = format!("[{}] [{}] {}\n", timestamp, level.as_str(), message);
            let _ = self.output.write_all(log_line.as_bytes());
            let _ = self.output.flush();
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl VelinLogger {
    /// Erstellt einen neuen Velin Logger
    pub fn new() -> Self {
        VelinLogger {
            level: LogLevel::Info,
            output: Box::new(io::stdout()),
            context: HashMap::new(),
            file_output: None,
            json_format: false,
            rotation_enabled: false,
            max_file_size: 10 * 1024 * 1024, // 10MB default
            current_file_size: 0,
        }
    }

    /// Erstellt einen Velin Logger mit File-Logging
    pub fn with_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("Failed to open log file: {}", e))?;

        Ok(VelinLogger {
            level: LogLevel::Info,
            output: Box::new(io::stdout()),
            context: HashMap::new(),
            file_output: Some(file),
            json_format: false,
            rotation_enabled: false,
            max_file_size: 10 * 1024 * 1024,
            current_file_size: 0,
        })
    }

    /// Setzt das Log-Level
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    /// Fügt Context-Informationen hinzu
    pub fn add_context(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }

    /// Entfernt Context-Informationen
    pub fn remove_context(&mut self, key: &str) {
        self.context.remove(key);
    }

    /// Aktiviert JSON-Format
    pub fn enable_json_format(&mut self) {
        self.json_format = true;
    }

    /// Aktiviert Log-Rotation
    pub fn enable_rotation(&mut self, max_size: u64) {
        self.rotation_enabled = true;
        self.max_file_size = max_size;
    }

    /// Loggt eine Nachricht mit Context
    pub fn log_with_context(
        &mut self,
        level: LogLevel,
        message: &str,
        additional_context: Option<HashMap<String, String>>,
    ) {
        if level >= self.level {
            let timestamp = Utc::now().to_rfc3339();

            if self.json_format {
                // JSON-Format
                let mut log_entry = serde_json::Map::new();
                log_entry.insert(
                    "timestamp".to_string(),
                    serde_json::Value::String(timestamp),
                );
                log_entry.insert(
                    "level".to_string(),
                    serde_json::Value::String(level.as_str().to_string()),
                );
                log_entry.insert(
                    "message".to_string(),
                    serde_json::Value::String(message.to_string()),
                );

                // Add context
                for (key, value) in &self.context {
                    log_entry.insert(key.clone(), serde_json::Value::String(value.clone()));
                }

                // Add additional context
                if let Some(extra) = additional_context {
                    for (key, value) in extra {
                        log_entry.insert(key, serde_json::Value::String(value));
                    }
                }

                let json_str = serde_json::to_string(&log_entry).unwrap_or_default();
                let log_line = format!("{}\n", json_str);
                self.write_log(&log_line);
            } else {
                // Text-Format mit Context
                let mut context_str = String::new();
                if !self.context.is_empty() {
                    let context_parts: Vec<String> = self
                        .context
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    context_str = format!(" [{}]", context_parts.join(", "));
                }

                if let Some(extra) = additional_context {
                    let extra_parts: Vec<String> =
                        extra.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
                    if !context_str.is_empty() {
                        context_str.push_str(&format!(", {}", extra_parts.join(", ")));
                    } else {
                        context_str = format!(" [{}]", extra_parts.join(", "));
                    }
                }

                let log_line = format!(
                    "[{}] [{}]{}{}\n",
                    timestamp,
                    level.as_str(),
                    context_str,
                    message
                );
                self.write_log(&log_line);
            }
        }
    }

    /// Standard-Log-Methoden
    pub fn trace(&mut self, message: &str) {
        self.log_with_context(LogLevel::Trace, message, None);
    }

    pub fn debug(&mut self, message: &str) {
        self.log_with_context(LogLevel::Debug, message, None);
    }

    pub fn info(&mut self, message: &str) {
        self.log_with_context(LogLevel::Info, message, None);
    }

    pub fn warn(&mut self, message: &str) {
        self.log_with_context(LogLevel::Warn, message, None);
    }

    pub fn error(&mut self, message: &str) {
        self.log_with_context(LogLevel::Error, message, None);
    }

    /// Interne Write-Funktion
    fn write_log(&mut self, log_line: &str) {
        let _ = self.output.write_all(log_line.as_bytes());
        let _ = self.output.flush();

        if let Some(ref mut file) = self.file_output {
            let _ = file.write_all(log_line.as_bytes());
            let _ = file.flush();
            self.current_file_size += log_line.len() as u64;

            // Check rotation
            if self.rotation_enabled && self.current_file_size >= self.max_file_size {
                // Rotation would be implemented here
                // For now, just reset the counter
                self.current_file_size = 0;
            }
        }
    }
}

impl Default for VelinLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Logging Standard Library für Code-Generierung
pub struct LoggingStdlib;

impl LoggingStdlib {
    /// Generiert Rust-Code für log.trace()
    pub fn generate_trace_code(message: &str) -> String {
        format!("logger.trace(\"{}\")", message)
    }

    /// Generiert Rust-Code für log.debug()
    pub fn generate_debug_code(message: &str) -> String {
        format!("logger.debug(\"{}\")", message)
    }

    /// Generiert Rust-Code für log.info()
    pub fn generate_info_code(message: &str) -> String {
        format!("logger.info(\"{}\")", message)
    }

    /// Generiert Rust-Code für log.warn()
    pub fn generate_warn_code(message: &str) -> String {
        format!("logger.warn(\"{}\")", message)
    }

    /// Generiert Rust-Code für log.error()
    pub fn generate_error_code(message: &str) -> String {
        format!("logger.error(\"{}\")", message)
    }

    /// Liste der verfügbaren Logging-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "log.trace".to_string(),
                signature: "fn(string) -> ()".to_string(),
            },
            FunctionInfo {
                name: "log.debug".to_string(),
                signature: "fn(string) -> ()".to_string(),
            },
            FunctionInfo {
                name: "log.info".to_string(),
                signature: "fn(string) -> ()".to_string(),
            },
            FunctionInfo {
                name: "log.warn".to_string(),
                signature: "fn(string) -> ()".to_string(),
            },
            FunctionInfo {
                name: "log.error".to_string(),
                signature: "fn(string) -> ()".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
