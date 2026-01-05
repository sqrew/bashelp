use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{Provider, ProviderError};

pub struct OllamaProvider {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

impl OllamaProvider {
    pub fn new(endpoint: String, model: String) -> Self {
        Self {
            endpoint,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    async fn complete(&self, prompt: &str) -> Result<String, ProviderError> {
        let url = format!("{}/api/generate", self.endpoint);

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::ProviderError(format!(
                "Ollama returned {}: {}",
                status, body
            )));
        }

        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.response)
    }

    fn name(&self) -> &str {
        "ollama"
    }
}
