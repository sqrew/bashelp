mod claude;
mod gemini;
mod grok;
mod ollama;
mod openai;
mod openai_compatible;

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
        "openai" | "chatgpt" | "gpt" => {
            let api_key = config
                .provider
                .api_key
                .clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "OpenAI provider requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai::OpenAIProvider::new(api_key, model.to_string())))
        }
        "grok" | "xai" => {
            let api_key = config
                .provider
                .api_key
                .clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Grok provider requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(grok::GrokProvider::new(api_key, model.to_string())))
        }
        "gemini" | "google" => {
            let api_key = config
                .provider
                .api_key
                .clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Gemini provider requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(gemini::GeminiProvider::new(api_key, model.to_string())))
        }
        // Convenience aliases for popular OpenAI-compatible services
        "groq" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Groq requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(),
                "https://api.groq.com/openai/v1/chat/completions".to_string()
            )))
        }
        "mistral" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Mistral requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(),
                "https://api.mistral.ai/v1/chat/completions".to_string()
            )))
        }
        "perplexity" | "pplx" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Perplexity requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(),
                "https://api.perplexity.ai/chat/completions".to_string()
            )))
        }
        "together" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Together AI requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(),
                "https://api.together.xyz/v1/chat/completions".to_string()
            )))
        }
        "fireworks" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Fireworks AI requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(),
                "https://api.fireworks.ai/inference/v1/chat/completions".to_string()
            )))
        }
        "deepseek" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "DeepSeek requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(),
                "https://api.deepseek.com/chat/completions".to_string()
            )))
        }
        "openrouter" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "OpenRouter requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(),
                "https://openrouter.ai/api/v1/chat/completions".to_string()
            )))
        }
        // Generic OpenAI-compatible provider - bring your own endpoint
        "openai-compatible" | "custom" => {
            let api_key = config.provider.api_key.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Custom provider requires an API key. Set it with: bashelp config set provider.api_key YOUR_KEY".to_string()
                ))?;
            let endpoint = config.provider.endpoint.clone()
                .ok_or_else(|| ProviderError::ProviderError(
                    "Custom provider requires an endpoint. Set it with: bashelp config set provider.endpoint YOUR_URL".to_string()
                ))?;
            Ok(Box::new(openai_compatible::OpenAICompatibleProvider::new(
                api_key, model.to_string(), endpoint
            )))
        }
        _ => Err(ProviderError::UnknownProvider(name.to_string())),
    }
}
