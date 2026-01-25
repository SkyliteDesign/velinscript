use super::{Template, TemplateConfig};
use anyhow::Result;

/// AI Client Template
///
/// Generiert LLM-Client Integration
/// Mit OpenAI, Anthropic, etc.
pub struct AIClientTemplate;

impl Template for AIClientTemplate {
    fn generate(&self, config: &TemplateConfig) -> Result<String, String> {
        let provider = config
            .options
            .get("provider")
            .and_then(|v| v.as_str())
            .unwrap_or("openai");

        Ok(self.generate_ai_client(provider))
    }
}

impl AIClientTemplate {
    fn generate_ai_client(&self, provider: &str) -> String {
        format!(
            r#"use crate::stdlib::ml::{{LLMClient, LLMProvider, ChatMessage}};

pub struct AIClient {{
    client: LLMClient,
}}

impl AIClient {{
    pub fn new(api_key: String) -> Self {{
        let provider = LLMProvider::{};
        Self {{
            client: LLMClient::new(provider, api_key),
        }}
    }}

    pub async fn generate(&self, prompt: &str) -> Result<String, String> {{
        self.client.generate(prompt)
    }}

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, String> {{
        self.client.chat(messages)
    }}
}}
"#,
            match provider {
                "openai" => "OpenAI",
                "anthropic" => "Anthropic",
                "gemini" | "google" => "GoogleGemini",
                _ => "Local",
            }
        )
    }
}
