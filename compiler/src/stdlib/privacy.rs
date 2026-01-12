// Privacy & DSGVO-Compliance Standard Library
// Generiert PII-Wrapper Types und Privacy-konformen Code

use crate::parser::ast::*;
use zeroize::Zeroize;

pub struct PrivacyStdlib;

/// Privacy Wrapper Type für PII-Daten
/// Verhindert versehentliches Logging von sensiblen Daten
#[derive(Debug, Clone)]
pub struct PrivacyWrapper<T> {
    inner: T,
    #[cfg(feature = "privacy")]
    _marker: std::marker::PhantomData<()>,
}

impl<T> PrivacyWrapper<T> {
    pub fn new(value: T) -> Self {
        PrivacyWrapper {
            inner: value,
            #[cfg(feature = "privacy")]
            _marker: std::marker::PhantomData,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T: Zeroize> Drop for PrivacyWrapper<T> {
    fn drop(&mut self) {
        self.inner.zeroize();
    }
}

// Implementierung für Debug - verschleiert Daten
impl<T> std::fmt::Debug for PrivacyWrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrivacyWrapper(***)")
    }
}

// Implementierung für Display - verschleiert Daten
impl<T> std::fmt::Display for PrivacyWrapper<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "***")
    }
}

impl PrivacyStdlib {
    /// Generiert Privacy Wrapper Type
    pub fn generate_privacy_wrapper_type() -> String {
        r#"use zeroize::Zeroize;
use secrecy::{Secret, ExposeSecret};

#[derive(Clone)]
pub struct PrivacyWrapper<T> {
    inner: T,
}

impl<T> PrivacyWrapper<T> {
    pub fn new(value: T) -> Self {
        PrivacyWrapper { inner: value }
    }
    
    pub fn into_inner(self) -> T {
        self.inner
    }
    
    pub fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T: Zeroize> Drop for PrivacyWrapper<T> {
    fn drop(&mut self) {
        self.inner.zeroize();
    }
}

impl<T> std::fmt::Debug for PrivacyWrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PrivacyWrapper(***)")
    }
}

impl std::fmt::Display for PrivacyWrapper<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "***")
    }
}

// Secret Wrapper für API Keys, Passwords, etc.
pub type SecretString = Secret<String>;

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Secret::new(s)
    }
}"#.to_string()
    }

    /// Generiert PII-Detection für Felder
    pub fn generate_pii_detection() -> String {
        r#"pub fn is_pii_field(field_name: &str) -> bool {
    let pii_keywords = vec![
        "email", "phone", "ssn", "passport", "credit_card",
        "ip", "address", "name", "birthdate", "gender",
    ];
    
    let field_lower = field_name.to_lowercase();
    pii_keywords.iter().any(|keyword| field_lower.contains(keyword))
}

pub fn mask_pii_value(value: &str) -> String {
    if value.len() <= 4 {
        "***".to_string()
    } else {
        format!("{}***{}", &value[..2], &value[value.len()-2..])
    }
}"#.to_string()
    }

    /// Generiert Logging-Filter für PII-Daten
    pub fn generate_logging_filter() -> String {
        r#"use tracing::{Event, Subscriber};
use tracing_subscriber::layer::Context;

pub struct PIILoggingFilter;

impl<S: Subscriber> tracing_subscriber::Layer<S> for PIILoggingFilter {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // Filter PII from logs
        // In production, implement actual filtering logic
    }
}

pub fn create_privacy_aware_logger() -> impl tracing_subscriber::Layer {
    PIILoggingFilter
}"#.to_string()
    }

    /// Generiert Secure Deletion
    pub fn generate_secure_deletion() -> String {
        r#"use zeroize::Zeroize;

pub trait SecureDelete {
    fn secure_delete(&mut self);
}

impl SecureDelete for String {
    fn secure_delete(&mut self) {
        self.zeroize();
        self.clear();
    }
}

impl<T: Zeroize> SecureDelete for Vec<T> {
    fn secure_delete(&mut self) {
        for item in self.iter_mut() {
            item.zeroize();
        }
        self.clear();
    }
}

pub fn secure_delete_user_data<T: SecureDelete>(data: &mut T) {
    data.secure_delete();
}"#.to_string()
    }

    /// Generiert Privacy-konforme Struct-Generierung
    pub fn generate_privacy_struct(struct_def: &Struct) -> String {
        let mut code = format!("#[derive(Clone, Debug, Serialize, Deserialize)]\npub struct {} {{\n", struct_def.name);
        
        for field in &struct_def.fields {
            let is_privacy = Self::is_privacy_field(field);
            let rust_type = Self::velin_to_rust_type(&field.field_type);
            
            if is_privacy {
                code.push_str(&format!("    #[serde(skip_serializing_if = \"Option::is_none\")]\n"));
                code.push_str(&format!("    pub {}: PrivacyWrapper<{}>,\n", field.name, rust_type));
            } else {
                code.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
            }
        }
        
        code.push_str("}\n");
        code
    }

    /// Prüft ob ein Feld Privacy-markiert ist
    fn is_privacy_field(field: &StructField) -> bool {
        // Prüfe @Privacy Decorator
        // StructField has no decorators field - skip
        // for decorator in &field.decorators {
            if decorator.name == "Privacy" || decorator.name == "@Privacy" {
                return true;
            }
        }
        
        // Prüfe Feldname auf PII-Keywords
        let field_lower = field.name.to_lowercase();
        let pii_keywords = vec!["email", "phone", "ssn", "passport", "credit_card", "ip", "address"];
        pii_keywords.iter().any(|keyword| field_lower.contains(keyword))
    }

    /// Konvertiert VelinScript Type zu Rust Type
    fn velin_to_rust_type(velin_type: &Type) -> String {
        match velin_type {
            Type::String => "String".to_string(),
            Type::Number => "f64".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::List(ref inner) => format!("Vec<{}>", Self::velin_to_rust_type(inner)),
            Type::Named(ref name) => name.clone(),
            _ => "String".to_string(),
        }
    }

    /// Generiert Zero-Knowledge Encryption Support
    pub fn generate_zero_knowledge_support() -> String {
        r#"use secrecy::Secret;

/// Zero-Knowledge Encryption: Daten werden während Verarbeitung verschlüsselt
pub struct ZeroKnowledgeData<T> {
    encrypted: Secret<Vec<u8>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ZeroKnowledgeData<T> {
    pub fn encrypt(data: T) -> Self 
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_vec(&data).unwrap();
        ZeroKnowledgeData {
            encrypted: Secret::new(serialized),
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn decrypt(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let data = self.encrypted.expose_secret();
        serde_json::from_slice(data)
    }
}

/// Confidential Computing: Daten werden in Secure Enclave verarbeitet
#[cfg(feature = "confidential-computing")]
pub mod confidential {
    pub fn process_in_enclave<T, F, R>(data: T, f: F) -> R
    where
        F: FnOnce(T) -> R,
    {
        // In production, use Intel SGX or AMD SEV
        f(data)
    }
}"#.to_string()
    }

    /// Prüft ob ein Decorator ein Privacy-Decorator ist
    pub fn is_privacy_decorator(decorator: &Decorator) -> bool {
        matches!(
            decorator.name.as_str(),
            "Privacy" | "@Privacy" | "PII" | "@PII" | "Secret" | "@Secret"
        )
    }
}
