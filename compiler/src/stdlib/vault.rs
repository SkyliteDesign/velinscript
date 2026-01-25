// HashiCorp Vault Integration
// Secret-Management und Certificate-Rotation

use crate::parser::ast::*;

pub struct VaultStdlib;

impl VaultStdlib {
    /// Generiert Vault Client Setup
    pub fn generate_vault_client(vault_address: &str, vault_token: Option<&str>) -> String {
        let token_line = if let Some(token) = vault_token {
            format!(r#"    let token = "{}";"#, token)
        } else {
            r#"    let token = std::env::var("VAULT_TOKEN").expect("VAULT_TOKEN must be set");"#
                .to_string()
        };

        format!(
            r#"use vaultrs::client::{{VaultClient, VaultClientSettingsBuilder}};

pub async fn create_vault_client() -> Result<VaultClient, vaultrs::error::ClientError> {{
    {}
    let settings = VaultClientSettingsBuilder::default()
        .address("{}")
        .token(token)
        .build()?;
    
    VaultClient::new(settings)
}}"#,
            token_line, vault_address
        )
    }

    /// Generiert Secret Retrieval
    pub fn generate_secret_retrieval(secret_path: &str, secret_key: &str) -> String {
        format!(
            r#"use vaultrs::api::kv2::requests::ReadSecretRequestBuilder;
use serde_json::Value;

pub async fn get_secret(
    client: &VaultClient,
    path: &str,
    key: &str,
) -> Result<String, vaultrs::error::ClientError> {{
    let response = client
        .kv2()
        .read(path)
        .await?;
    
    let secret_value = response
        .data
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| vaultrs::error::ClientError::Other("Secret key not found".to_string()))?;
    
    Ok(secret_value.to_string())
}}

// Usage:
// let secret = get_secret(&client, "{}", "{}").await?;"#,
            secret_path, secret_key
        )
    }

    /// Generiert API Key Retrieval
    pub fn generate_api_key_retrieval() -> String {
        r#"pub async fn get_api_key(
    client: &VaultClient,
    service: &str,
) -> Result<String, vaultrs::error::ClientError> {
    get_secret(client, &format!("secret/data/{}", service), "api_key").await
}

pub async fn get_database_credentials(
    client: &VaultClient,
    db_name: &str,
) -> Result<(String, String), vaultrs::error::ClientError> {
    let username = get_secret(client, &format!("secret/data/{}", db_name), "username").await?;
    let password = get_secret(client, &format!("secret/data/{}", db_name), "password").await?;
    Ok((username, password))
}"#
        .to_string()
    }

    /// Generiert Certificate Retrieval
    pub fn generate_certificate_retrieval() -> String {
        r#"use vaultrs::api::pki::requests::ReadCertificateRequestBuilder;

pub async fn get_certificate(
    client: &VaultClient,
    cert_path: &str,
) -> Result<(String, String), vaultrs::error::ClientError> {
    // Retrieve certificate and private key from Vault PKI
    let cert = client
        .pki()
        .read_certificate(cert_path)
        .await?;
    
    let key = client
        .pki()
        .read_private_key(cert_path)
        .await?;
    
    Ok((cert, key))
}

pub async fn rotate_certificate(
    client: &VaultClient,
    cert_path: &str,
) -> Result<(String, String), vaultrs::error::ClientError> {
    // Request new certificate from Vault
    let new_cert = client
        .pki()
        .issue_certificate(cert_path)
        .await?;
    
    Ok((new_cert.certificate, new_cert.private_key))
}"#
        .to_string()
    }

    /// Generiert Dynamic Secrets Support
    pub fn generate_dynamic_secrets() -> String {
        r#"pub async fn get_dynamic_secret(
    client: &VaultClient,
    secret_engine: &str,
    role: &str,
) -> Result<serde_json::Value, vaultrs::error::ClientError> {
    // Request dynamic secret (e.g., database credentials)
    let response = client
        .read(&format!("{}/creds/{}", secret_engine, role))
        .await?;
    
    Ok(response)
}

pub async fn renew_dynamic_secret(
    client: &VaultClient,
    lease_id: &str,
) -> Result<(), vaultrs::error::ClientError> {
    // Renew lease for dynamic secret
    client
        .write(&format!("sys/leases/renew/{}", lease_id), None)
        .await?;
    
    Ok(())
}"#
        .to_string()
    }

    /// Generiert Vault Health Check
    pub fn generate_health_check() -> String {
        r#"use vaultrs::api::sys::requests::HealthRequestBuilder;

pub async fn check_vault_health(
    client: &VaultClient,
) -> Result<bool, vaultrs::error::ClientError> {
    let health = client
        .sys()
        .health()
        .await?;
    
    Ok(health.initialized && !health.sealed)
}

pub async fn wait_for_vault(
    client: &VaultClient,
    max_retries: u32,
) -> Result<(), vaultrs::error::ClientError> {
    for _ in 0..max_retries {
        if check_vault_health(client).await? {
            return Ok(());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    Err(vaultrs::error::ClientError::Other(
        "Vault not available after retries".to_string()
    ))
}"#
        .to_string()
    }

    /// Generiert Vault Config Injection
    pub fn generate_config_injection() -> String {
        r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultConfig {
    pub address: String,
    pub token: Option<String>,
    pub mount_path: String,
}

pub async fn load_config_from_vault(
    client: &VaultClient,
    config_path: &str,
) -> Result<VaultConfig, vaultrs::error::ClientError> {
    let secret = client
        .kv2()
        .read(config_path)
        .await?;
    
    let config: VaultConfig = serde_json::from_value(secret.data)?;
    Ok(config)
}

/// Decorator-basierte Secret-Injection
/// @VaultSecret("secret/data/api", "api_key")
pub struct VaultSecret {
    pub path: String,
    pub key: String,
}

impl VaultSecret {
    pub async fn load(client: &VaultClient) -> Result<String, vaultrs::error::ClientError> {
        get_secret(client, &self.path, &self.key).await
    }
}"#
        .to_string()
    }

    /// Prüft ob ein Decorator ein Vault-Decorator ist
    pub fn is_vault_decorator(decorator: &Decorator) -> bool {
        matches!(
            decorator.name.as_str(),
            "VaultSecret" | "@VaultSecret" | "Vault" | "@Vault"
        )
    }
}
