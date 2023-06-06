use futures::{stream::StreamExt, Stream};
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
    stream: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Delta {
    pub(crate) content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Choice {
    delta: Delta,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

impl ChatCompletionResponse {
    pub(crate) fn content(&self) -> &str {
        self.choices[0].delta.content.as_ref().map_or("", |s| s)
    }
}

pub(crate) async fn chat_completions(
    model: &str,
    messages: Vec<Message>,
) -> impl Stream<Item = ChatCompletionResponse> {
    let client = reqwest::Client::new();
    let request = ChatCompletionRequest {
        model: model.to_string(),
        messages,
        stream: true,
    };
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();

    let mut stream = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(60))
        .json(&request)
        .send()
        .await
        .unwrap()
        .bytes_stream();

    let stream = async_stream::stream! {
        while let Some(item) = stream.next().await {
            let item = item.unwrap();
            let item_str = std::str::from_utf8(&item).unwrap();
            for chunk in item_str.split('\n').filter(|chunk| chunk.starts_with("data:")) {
                let chunk = chunk.trim_start_matches("data: ");
                if chunk.starts_with("[DONE]") {
                    return;
                }
                let chat_completion_response: ChatCompletionResponse = serde_json::from_str(chunk).unwrap();
                yield chat_completion_response
            }
        }
    };
    stream
}
