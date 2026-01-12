// Standard Library für Crypto-Funktionalität
// Kryptografische Funktionen für Hashing und Verschlüsselung

use sha2::{Sha256, Digest};
use std::fmt::Write;

/// Crypto Standard Library
pub struct CryptoStdlib;

impl CryptoStdlib {
    /// Erstellt einen SHA-256 Hash
    pub fn sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        
        let mut hex_string = String::with_capacity(64);
        for byte in result {
            write!(&mut hex_string, "{:02x}", byte).unwrap();
        }
        hex_string
    }
    
    /// Erstellt einen MD5 Hash (für Legacy-Kompatibilität)
    pub fn md5(input: &str) -> String {
        let digest = md5::compute(input.as_bytes());
        format!("{:x}", digest)
    }
    
    /// Erstellt einen zufälligen String
    pub fn random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
    
    /// Erstellt eine zufällige UUID v4
    pub fn uuid() -> String {
        use uuid::Uuid;
        Uuid::new_v4().to_string()
    }
    
    /// Erstellt einen Salt für Passwort-Hashing
    pub fn generate_salt() -> String {
        Self::random_string(32)
    }
    
    /// Hasht ein Passwort mit einem Salt (einfache Implementierung)
    pub fn hash_password(password: &str, salt: &str) -> String {
        Self::sha256(&format!("{}{}", password, salt))
    }
    
    /// Verifiziert ein Passwort gegen einen Hash
    pub fn verify_password(password: &str, salt: &str, hash: &str) -> bool {
        let computed_hash = Self::hash_password(password, salt);
        computed_hash == hash
    }
    
    /// Base64-Enkodierung
    pub fn base64_encode(input: &str) -> String {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.encode(input.as_bytes())
    }
    
    /// Base64-Dekodierung
    pub fn base64_decode(input: &str) -> Result<String, String> {
        use base64::{Engine as _, engine::general_purpose};
        let decoded = general_purpose::STANDARD
            .decode(input)
            .map_err(|e| format!("Fehler beim Base64-Dekodieren: {}", e))?;
        String::from_utf8(decoded)
            .map_err(|e| format!("Fehler beim Konvertieren zu UTF-8: {}", e))
    }
    
    /// Generiert Rust-Code für crypto.sha256()
    pub fn generate_sha256_code(input: &str) -> String {
        format!("crypto::sha256(\"{}\")", input)
    }
    
    /// Generiert Rust-Code für crypto.md5()
    pub fn generate_md5_code(input: &str) -> String {
        format!("crypto::md5(\"{}\")", input)
    }
    
    /// Generiert Rust-Code für crypto.uuid()
    pub fn generate_uuid_code() -> String {
        "crypto::uuid()".to_string()
    }
    
    /// Generiert Rust-Code für crypto.hashPassword()
    pub fn generate_hash_password_code(password: &str, salt: &str) -> String {
        format!("crypto::hash_password(\"{}\", \"{}\")", password, salt)
    }
    
    /// Liste der verfügbaren Crypto-Funktionen
    pub fn get_functions() -> Vec<FunctionInfo> {
        vec![
            FunctionInfo {
                name: "crypto.sha256".to_string(),
                signature: "fn(string) -> string".to_string(),
            },
            FunctionInfo {
                name: "crypto.md5".to_string(),
                signature: "fn(string) -> string".to_string(),
            },
            FunctionInfo {
                name: "crypto.randomString".to_string(),
                signature: "fn(usize) -> string".to_string(),
            },
            FunctionInfo {
                name: "crypto.uuid".to_string(),
                signature: "fn() -> string".to_string(),
            },
            FunctionInfo {
                name: "crypto.generateSalt".to_string(),
                signature: "fn() -> string".to_string(),
            },
            FunctionInfo {
                name: "crypto.hashPassword".to_string(),
                signature: "fn(string, string) -> string".to_string(),
            },
            FunctionInfo {
                name: "crypto.verifyPassword".to_string(),
                signature: "fn(string, string, string) -> bool".to_string(),
            },
            FunctionInfo {
                name: "crypto.base64Encode".to_string(),
                signature: "fn(string) -> string".to_string(),
            },
            FunctionInfo {
                name: "crypto.base64Decode".to_string(),
                signature: "fn(string) -> Result<string, string>".to_string(),
            },
        ]
    }
}

pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
}
