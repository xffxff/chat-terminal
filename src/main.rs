mod openai;

use anyhow::{Context, Result};
use std::io::Write;
use std::{collections::HashMap, fs::File, io::Read};

use clap::Parser;
use futures::pin_mut;
use futures::StreamExt;
use openai::{chat_completions, Message};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Prompt {
    description: String,
    prefix: Option<String>,
    suffix: Option<String>,
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
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut file = File::open(".chatconfig.toml").context("Failed to open config file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read config file")?;

    let config: Config = toml::from_str(&contents).context("Failed to parse TOML")?;

    let prompt = config
        .get_prompt(&cli.mode)
        .context(format!("Failed to find prompt: {:?}", &cli.mode))?;

    let mut message = String::new();
    if let Some(prefix) = &prompt.prefix {
        message.push_str(prefix)
    }
    message.push_str(&cli.message);
    if let Some(postfix) = &prompt.postfix {
        message.push_str(postfix)
    }
    let stream = chat_completions("gpt-3.5-turbo", vec![Message::new("system", &message)]).await?;
    pin_mut!(stream);
    while let Some(resp) = stream.next().await {
        let resp = resp?;
        print!("{}", resp.content());
        std::io::stdout().flush()?;
    }
    Ok(())
}
