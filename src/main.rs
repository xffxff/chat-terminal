mod openai;

use openai::{chat_completions, Message};

#[tokio::main]
async fn main() {
    let response = chat_completions("gpt-3.5-turbo", vec![Message::new("system", "Hello")]).await;
    dbg!(&response.content());
}
