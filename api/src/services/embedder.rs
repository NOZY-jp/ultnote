use crate::error::{AppError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct EmbedderClient {
    client: Client,
    base_url: String,
}

#[derive(Serialize)]
struct EmbedRequest {
    text: String,
}

#[derive(Deserialize)]
struct EmbedResponse {
    vector: Vec<f32>,
}

impl EmbedderClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn embed_for_storage(&self, text: &str) -> Result<Vec<f32>> {
        self.embed(&format!("passage: {}", text)).await
    }

    pub async fn embed_for_search(&self, query: &str) -> Result<Vec<f32>> {
        self.embed(&format!("query: {}", query)).await
    }

    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let url = format!("{}/embed", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&EmbedRequest {
                text: text.to_string(),
            })
            .send()
            .await
            .map_err(|e| AppError::Embedder(format!("Failed to connect: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Embedder(format!(
                "Embedder returned {}: {}",
                status, error_text
            )));
        }

        let data: EmbedResponse = response
            .json()
            .await
            .map_err(|e| AppError::Embedder(format!("Failed to parse response: {}", e)))?;

        Ok(data.vector)
    }

    pub async fn health_check(&self) -> Result<()> {
        let url = format!("{}/health", self.base_url);
        self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Embedder(format!("Health check failed: {}", e)))?;
        Ok(())
    }
}
