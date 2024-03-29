use anyhow::{Context, Result};
use futures::{stream::StreamExt, Stream};
use serde::{Deserialize, Serialize};

use crate::API;

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
    api: &API,
) -> Result<impl Stream<Item = Result<ChatCompletionResponse>>> {
    let client = reqwest::Client::new();
    let request = ChatCompletionRequest {
        model: model.to_string(),
        messages,
        stream: true,
    };

    let mut stream = client
        .post(format!("{}/chat/completions", api.base_url))
        .header("Authorization", format!("Bearer {}", api.api_key))
        .timeout(std::time::Duration::from_secs(60))
        .json(&request)
        .send()
        .await?
        .bytes_stream();

    let stream = async_stream::stream! {
        while let Some(item) = stream.next().await {
            let item = item?;
            let item_str = std::str::from_utf8(&item).context(format!("Failed to convert bytes to string: {:?}", item))?;
            for chunk in item_str.split('\n').filter(|chunk| chunk.starts_with("data:")) {
                // To be able to deserialize the item_str, we need to split it into chunks
                // In general, the chunks look like this:
                // data: {
                //     "choices": [
                //         {
                //         "delta": {
                //             "content": "\n\n"
                //         },
                //         "finish_reason": null,
                //         "index": 0
                //         }
                //     ],
                //     "created": 1677825464,
                //     "id": "chatcmpl-6ptKyqKOGXZT6iQnqiXAH8adNLUzD",
                //     "model": "gpt-3.5-turbo-0301",
                //     "object": "chat.completion.chunk"
                // }
                // data: {
                //     "choices": [
                //         {
                //         "delta": {
                //             "content": "2"
                //         },
                //         "finish_reason": null,
                //         "index": 0
                //         }
                //     ],
                //     "created": 1677825464,
                //     "id": "chatcmpl-6ptKyqKOGXZT6iQnqiXAH8adNLUzD",
                //     "model": "gpt-3.5-turbo-0301",
                //     "object": "chat.completion.chunk"
                // }
                let chunk = chunk.trim_start_matches("data: ");
                // The last chunk is "data: [DONE]", which means the stream is done
                if chunk.starts_with("[DONE]") {
                    return;
                }
                let chat_completion_response: ChatCompletionResponse = serde_json::from_str(chunk)
                    .context(format!("Failed to deserialize chunk: {:?}", chunk))?;
                yield Ok(chat_completion_response)
            }
        }
    };
    Ok(stream)
}
