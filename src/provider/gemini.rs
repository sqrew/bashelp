use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use super::{Provider, ProviderError};

pub struct GeminiProvider {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<GeminiError>,
}

#[derive(Deserialize, Clone)]
struct Candidate {
    content: CandidateContent,
}

#[derive(Deserialize, Clone)]
struct CandidateContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize, Clone)]
struct ResponsePart {
    text: String,
}

#[derive(Deserialize)]
struct GeminiError {
    message: String,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    async fn complete(&self, prompt: &str) -> Result<String, ProviderError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ProviderError::ProviderError(format!(
                "Gemini returned {}: {}",
                status, body
            )));
        }

        let gemini_response: GeminiResponse = response.json().await?;

        if let Some(error) = gemini_response.error {
            return Err(ProviderError::ProviderError(error.message));
        }

        let text = gemini_response
            .candidates
            .and_then(|c| c.first().cloned())
            .and_then(|c| c.content.parts.first().cloned())
            .map(|p| p.text)
            .unwrap_or_default();

        Ok(text)
    }

    fn name(&self) -> &str {
        "gemini"
    }
}
