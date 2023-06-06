mod openai;

use std::{collections::HashMap, fs::File, io::Read};
use std::io::Write;

use futures::StreamExt;
use openai::{chat_completions, Message};
use clap::Parser;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Prompt {
    description: String,
    prefix: Option<String>,
    postfix: Option<String>
}

#[derive(Debug, Deserialize)]
struct Prompts {
    #[serde(flatten)]
    prompts: HashMap<String, Prompt>,
}

#[derive(Debug, Deserialize)]
struct Config {
    prompts: Prompts,
}

impl Config {
    fn get_prompt(&self, name: &str) -> Option<&Prompt> {
        self.prompts.prompts.get(name)
    }
}


#[derive(Parser)]
struct Cli {
    mode: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut file = File::open("config.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let config: Config = toml::from_str(&contents).expect("Failed to parse TOML");

    let prompt = config.get_prompt(&cli.mode).expect("Failed to find prompt");

    let mut message = String::new();
    if let Some(prefix) = &prompt.prefix {
        message.push_str(prefix)
    }
    message.push_str(&cli.message);
    if let Some(postfix) = &prompt.postfix {
        message.push_str(postfix)
    }
    let stream = chat_completions("gpt-3.5-turbo", vec![Message::new("system", &message)]).await;
    stream
        .for_each(|resp| async move {
            print!("{}", resp.content());
            std::io::stdout().flush().unwrap();
        })
        .await;
}
