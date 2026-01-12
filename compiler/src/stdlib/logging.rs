// Standard Library für Logging-Funktionalität
// Logging-Funktionen für verschiedene Log-Level

use std::io::{self, Write};
use chrono::Utc;

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

/// Logger-Struktur
pub struct Logger {
    level: LogLevel,
    output: Box<dyn Write>,
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
