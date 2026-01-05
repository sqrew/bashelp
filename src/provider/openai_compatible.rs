use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{Provider, ProviderError};

/// Generic provider for any OpenAI-compatible API.
/// Works with: Groq, Mistral, Perplexity, Together AI, Fireworks,
/// DeepSeek, OpenRouter, llama.cpp, vLLM, and more.
pub struct OpenAICompatibleProvider {
    api_key: String,
    model: String,
    endpoint: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

impl OpenAICompatibleProvider {
    pub fn new(api_key: String, model: String, endpoint: String) -> Self {
        Self {
            api_key,
            model,
            endpoint,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for OpenAICompatibleProvider {
    async fn complete(&self, prompt: &str) -> Result<String, ProviderError> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::ProviderError(format!(
                "API returned {}: {}",
                status, body
            )));
        }

        let api_response: OpenAIResponse = response.json().await?;
        let text = api_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(text)
    }

    fn name(&self) -> &str {
        "openai-compatible"
    }
}
