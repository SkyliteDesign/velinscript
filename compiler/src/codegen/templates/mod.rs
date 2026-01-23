/// Component Templates für System-Generierung
/// 
/// Wiederverwendbare Templates für System-Komponenten:
/// - APIServerTemplate
/// - AuthTemplate
/// - RateLimitTemplate
/// - AIClientTemplate
/// - DeploymentTemplate

pub mod api_server;
pub mod auth;
pub mod rate_limit;
pub mod ai_client;
pub mod deployment;

pub use api_server::APIServerTemplate;
pub use auth::AuthTemplate;
pub use rate_limit::RateLimitTemplate;
pub use ai_client::AIClientTemplate;
pub use deployment::DeploymentTemplate;

use serde_json::Value;

/// Trait für alle Templates
pub trait Template {
    fn generate(&self, config: &TemplateConfig) -> Result<String, String>;
}

#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub name: String,
    pub options: Value,
}
