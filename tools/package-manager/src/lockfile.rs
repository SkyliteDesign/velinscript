// Lock File Management - Speichert exakte Versionen für reproduzierbare Builds

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// All imports are used (Deserialize for structs, HashMap for packages map)
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct LockFile {
    pub version: String,
    pub packages: HashMap<String, LockedPackage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockedPackage {
    pub version: String,
    pub source: String,
    pub checksum: Option<String>,
}

impl LockFile {
    /// Lädt Lock File
    pub fn load() -> Result<Self> {
        let lock_path = Path::new("velin.lock");
        
        if !lock_path.exists() {
            return Ok(LockFile {
                version: "1".to_string(),
                packages: HashMap::new(),
            });
        }

        let content = fs::read_to_string(lock_path)?;
        let lock: LockFile = toml::from_str(&content)?;
        Ok(lock)
    }

    /// Speichert Lock File
    pub fn save(&self) -> Result<()> {
        let lock_path = Path::new("velin.lock");
        let content = toml::to_string_pretty(self)?;
        fs::write(lock_path, content)?;
        Ok(())
    }

    /// Aktualisiert Lock File mit neuen Dependencies
    pub fn update(&mut self, packages: HashMap<String, LockedPackage>) {
        self.packages = packages;
    }
}
