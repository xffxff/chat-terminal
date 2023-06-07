mod openai;

use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;
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

fn read_config_file() -> Result<Config> {
    let mut contents = String::new();
    let mut file_found = false;

    // Try to read .chatconfig.toml first
    if let Ok(mut file) = File::open(".chatconfig.toml") {
        file.read_to_string(&mut contents)
            .context("Failed to read config file")?;
        file_found = true;
    }

    // If .chatconfig.toml is not found, try to read ~/.config/chat/config.toml
    if !file_found {
        let mut path = PathBuf::new();
        path.push(std::env::var("HOME").context("Failed to get HOME directory")?);
        path.push(".config");
        path.push("chat");
        path.push("config.toml");

        if let Ok(mut file) = File::open(path) {
            file.read_to_string(&mut contents)
                .context("Failed to read config file")?;
            file_found = true;
        }
    }

    if !file_found {
        return Err(anyhow::anyhow!("Failed to find config file, make sure you have a .chatconfig.toml or ~/.config/chat/config.toml file"));
    }

    let config: Config = toml::from_str(&contents).context("Failed to parse TOML")?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = read_config_file()?;

    let prompt = config
        .get_prompt(&cli.mode)
        .context(format!("Failed to find prompt: {:?}", &cli.mode))?;

    let mut message = String::new();
    if let Some(prefix) = &prompt.prefix {
        message.push_str(prefix)
    }
    message.push_str(&cli.message);
    if let Some(postfix) = &prompt.suffix {
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
