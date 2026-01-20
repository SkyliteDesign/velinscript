// Standard Library für Caching-Funktionalität
// Cache-Funktionen für in-memory und persistent caching

use std::collections::HashMap;
use std::time::{SystemTime, Duration};

/// Cache Entry mit TTL (Time To Live)
pub struct CacheEntry<T> {
    value: T,
    expires_at: SystemTime,
}

/// In-Memory Cache mit TTL-Unterstützung
pub struct Cache<T> {
    data: HashMap<String, CacheEntry<T>>,
}

impl<T> Cache<T> {
    /// Erstellt einen neuen Cache
    pub fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }
    
    /// Setzt einen Wert mit optionaler TTL (in Sekunden)
    pub fn set(&mut self, key: String, value: T, ttl_seconds: Option<u64>) {
        let expires_at = if let Some(ttl) = ttl_seconds {
            SystemTime::now() + Duration::from_secs(ttl)
        } else {
            SystemTime::now() + Duration::from_secs(u64::MAX)
        };
        
        self.data.insert(key, CacheEntry {
            value,
            expires_at,
        });
    }
    
    /// Holt einen Wert aus dem Cache
    pub fn get(&self, key: &str) -> Option<&T> {
        if let Some(entry) = self.data.get(key) {
            if entry.expires_at > SystemTime::now() {
                Some(&entry.value)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Entfernt einen Wert aus dem Cache
    pub fn remove(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
    
    /// Löscht alle abgelaufenen Einträge
    pub fn cleanup(&mut self) {
        let now = SystemTime::now();
        self.data.retain(|_, entry| entry.expires_at > now);
    }
    
    /// Löscht den gesamten Cache
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Prüft ob ein Key existiert
    pub fn exists(&self, key: &str) -> bool {
        if let Some(entry) = self.data.get(key) {
            entry.expires_at > SystemTime::now()
        } else {
            false
        }
    }
    
    /// Gibt die Anzahl der Einträge zurück
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for Cache<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache Standard Library für Code-Generierung
pub struct CacheStdlib;

impl CacheStdlib {
    /// Generiert Rust-Code für cache.set()
    pub fn generate_set_code(key: &str, value: &str, ttl: Option<&str>) -> String {
        if let Some(ttl_val) = ttl {
            format!("cache.set(\"{}\".to_string(), {}, Some({} as u64))", key, value, ttl_val)
        } else {
            format!("cache.set(\"{}\".to_string(), {}, None)", key, value)
        }
    }
    
    /// Generiert Rust-Code für cache.get()
    pub fn generate_get_code(key: &str) -> String {
        format!("cache.get(\"{}\")", key)
    }
    
    /// Generiert Rust-Code für cache.remove()
    pub fn generate_remove_code(key: &str) -> String {
        format!("cache.remove(\"{}\")", key)
    }
    
    /// Generiert Rust-Code für cache.clear()
    pub fn generate_clear_code() -> String {
        "cache.clear()".to_string()
    }
    
    /// Generiert Rust-Code für cache.exists()
    pub fn generate_exists_code(key: &str) -> String {
        format!("cache.exists(\"{}\")", key)
    }

    /// Generiert Rust-Code für cache.size()
    pub fn generate_size_code() -> String {
        "cache.size()".to_string()
    }
    
    /// Liste der verfügbaren Cache-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "cache.set".to_string(),
                signature: "fn(string, T, Option<u64>) -> ()".to_string(),
            },
            FunctionInfo {
                name: "cache.get".to_string(),
                signature: "fn(string) -> Option<T>".to_string(),
            },
            FunctionInfo {
                name: "cache.remove".to_string(),
                signature: "fn(string) -> bool".to_string(),
            },
            FunctionInfo {
                name: "cache.clear".to_string(),
                signature: "fn() -> ()".to_string(),
            },
            FunctionInfo {
                name: "cache.exists".to_string(),
                signature: "fn(string) -> bool".to_string(),
            },
            FunctionInfo {
                name: "cache.size".to_string(),
                signature: "fn() -> usize".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
