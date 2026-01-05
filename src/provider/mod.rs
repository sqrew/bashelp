mod claude;
mod ollama;

use async_trait::async_trait;
use thiserror::Error;
use crate::config::Config;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Unknown provider: {0}")]
    UnknownProvider(String),
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<String, ProviderError>;
    fn name(&self) -> &str;
}

pub fn create_provider(
    name: &str,
    model: &str,
    config: &Config,
) -> Result<Box<dyn Provider>, ProviderError> {
    match name {
        "ollama" => {
            let endpoint = config
                .provider
                .endpoint
                .clone()
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            Ok(Box::new(ollama::OllamaProvider::new(endpoint, model.to_string())))
        }
        "claude" | "anthropic" => {
            let api_key = config
                .provider
                .api_key
                .clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Claude provider requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(claude::ClaudeProvider::new(api_key, model.to_string())))
        }
        // Future providers:
        // "openai" => { ... }
        // "groq" => { ... }
        _ => Err(ProviderError::UnknownProvider(name.to_string())),
    }
}
