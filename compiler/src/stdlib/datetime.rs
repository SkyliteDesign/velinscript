// Standard Library für Date/Time-Funktionalität
// Datum- und Zeit-Operationen

use std::time::{SystemTime, UNIX_EPOCH};
use chrono;

/// Date/Time Standard Library
pub struct DateTimeStdlib;

impl DateTimeStdlib {
    /// Gibt die aktuelle Zeit als Unix-Timestamp zurück
    pub fn now_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    /// Gibt die aktuelle Zeit als Unix-Timestamp in Millisekunden zurück
    pub fn now_timestamp_millis() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
    
    /// Formatiert einen Unix-Timestamp als ISO 8601 String
    pub fn format_iso8601(timestamp: u64) -> String {
        let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
            .unwrap_or_default();
        datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
    
    /// Formatiert einen Unix-Timestamp als benutzerdefinierten String
    pub fn format_custom(timestamp: u64, format: &str) -> String {
        let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp as i64, 0)
            .unwrap_or_default();
        datetime.format(format).to_string()
    }
    
    /// Parst einen ISO 8601 String zu einem Unix-Timestamp
    pub fn parse_iso8601(iso_string: &str) -> Result<u64, String> {
        match chrono::DateTime::parse_from_rfc3339(iso_string) {
            Ok(dt) => Ok(dt.timestamp() as u64),
            Err(_) => {
                chrono::NaiveDateTime::parse_from_str(iso_string, "%Y-%m-%dT%H:%M:%S")
                    .map(|dt| dt.and_utc().timestamp() as u64)
                    .map_err(|e| format!("Fehler beim Parsen des Datums: {}", e))
            }
        }
    }
    
    /// Addiert Sekunden zu einem Timestamp
    pub fn add_seconds(timestamp: u64, seconds: i64) -> u64 {
        (timestamp as i64 + seconds) as u64
    }
    
    /// Subtrahiert Sekunden von einem Timestamp
    pub fn subtract_seconds(timestamp: u64, seconds: i64) -> u64 {
        (timestamp as i64 - seconds) as u64
    }
    
    /// Berechnet die Differenz zwischen zwei Timestamps in Sekunden
    pub fn difference_seconds(timestamp1: u64, timestamp2: u64) -> i64 {
        timestamp1 as i64 - timestamp2 as i64
    }
    
    /// Generiert Rust-Code für datetime.now()
    pub fn generate_now_code() -> String {
        "datetime::now_timestamp()".to_string()
    }
    
    /// Generiert Rust-Code für datetime.format()
    pub fn generate_format_code(timestamp: u64, format: &str) -> String {
        format!("datetime::format_custom({}, \"{}\")", timestamp, format)
    }
    
    /// Generiert Rust-Code für datetime.parse()
    pub fn generate_parse_code(iso_string: &str) -> String {
        format!("datetime::parse_iso8601(\"{}\")", iso_string)
    }
    
    /// Liste der verfügbaren Date/Time-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "datetime.now".to_string(),
                signature: "fn() -> u64".to_string(),
            },
            FunctionInfo {
                name: "datetime.nowMillis".to_string(),
                signature: "fn() -> u64".to_string(),
            },
            FunctionInfo {
                name: "datetime.format".to_string(),
                signature: "fn(u64, string) -> string".to_string(),
            },
            FunctionInfo {
                name: "datetime.formatISO8601".to_string(),
                signature: "fn(u64) -> string".to_string(),
            },
            FunctionInfo {
                name: "datetime.parse".to_string(),
                signature: "fn(string) -> Result<u64, string>".to_string(),
            },
            FunctionInfo {
                name: "datetime.addSeconds".to_string(),
                signature: "fn(u64, i64) -> u64".to_string(),
            },
            FunctionInfo {
                name: "datetime.subtractSeconds".to_string(),
                signature: "fn(u64, i64) -> u64".to_string(),
            },
            FunctionInfo {
                name: "datetime.difference".to_string(),
                signature: "fn(u64, u64) -> i64".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
