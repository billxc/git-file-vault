// AI module - generates commit messages using AI APIs

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

pub struct AiClient {
    endpoint: String,
    api_key: String,
    model: String,
}

impl AiClient {
    /// Create a new AI client
    pub fn new(endpoint: String, api_key: String, model: String) -> Self {
        Self {
            endpoint,
            api_key,
            model,
        }
    }

    /// Generate a commit message based on git diff
    pub async fn generate_commit_message(&self, diff: &str) -> Result<String> {
        if diff.trim().is_empty() {
            bail!("No changes to generate commit message for");
        }

        // Prepare the prompt
        let prompt = format!(
            "You are a helpful assistant that generates concise git commit messages based on code changes.\n\
            \n\
            Generate a commit message for the following git diff. The message should:\n\
            - Be concise (1-2 lines maximum)\n\
            - Start with a verb in present tense (e.g., 'add', 'fix', 'update', 'remove')\n\
            - Describe WHAT changed, not HOW it changed\n\
            - Not include any prefixes like 'feat:', 'fix:', etc.\n\
            - Not include markdown formatting\n\
            \n\
            Git diff:\n\
            ```\n\
            {}\n\
            ```\n\
            \n\
            Respond with ONLY the commit message, nothing else.",
            // Truncate diff if too long (to avoid token limits)
            if diff.len() > 4000 {
                &diff[..4000]
            } else {
                diff
            }
        );

        // Create request
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.7,
            max_tokens: 100,
        };

        // Send request
        let client = reqwest::Client::new();
        let response = client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to AI endpoint")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            bail!("AI request failed with status {}: {}", status, error_text);
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse AI response")?;

        let message = chat_response
            .choices
            .first()
            .context("No response from AI")?
            .message
            .content
            .trim()
            .to_string();

        Ok(message)
    }
}
