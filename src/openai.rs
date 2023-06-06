use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Message {
    role: String,
    content: String,
}

impl Message {
    pub(crate) fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

impl ChatCompletionResponse {
    pub(crate) fn content(&self) -> &str {
        &self.choices[0].message.content
    }
}

pub(crate) async fn chat_completions(
    model: &str,
    messages: Vec<Message>,
) -> ChatCompletionResponse {
    let client = reqwest::Client::new();
    let request = ChatCompletionRequest {
        model: model.to_string(),
        messages,
    };
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(60))
        .json(&request)
        .send()
        .await
        .unwrap();

    let chat_completion_response: ChatCompletionResponse = response.json().await.unwrap();
    chat_completion_response
}
