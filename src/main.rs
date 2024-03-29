mod openai;

use anyhow::{Context, Result};
use std::env;
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
struct API {
    model: String,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    prompts: Prompts,
    api: API,
}

impl Config {
    fn get_prompt(&self, name: &str) -> Option<&Prompt> {
        self.prompts.prompts.get(name)
    }
}

#[derive(Parser)]
struct Cli {
    mode: String,
    message: Option<String>,
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
    let config = read_config_file()?;

    // FIXME: This is a hacky way to check if the first argument is "list"
    // If using clap like `cargo run -- list`, it always complains about missing arguments `MODE` and `MESSAGE`
    // and I don't want to make them optional
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "list" {
        // The first argument is "list"
        // Show the name and descriptions of all the prompts
        println!("Available prompts:");
        for (name, prompt) in &config.prompts.prompts {
            println!("    {}: {}", name, prompt.description);
        }
        return Ok(());
    }

    let cli = Cli::parse();
    let prompt = config
        .get_prompt(&cli.mode)
        .context(format!("Failed to find prompt: {:?}", &cli.mode))?;

    if let Some(message) = &cli.message {
        process_message(prompt, message, &config.api).await?;
        return Ok(());
    }

    // interactive mode
    let mut message = String::new();
    loop {
        print!("\n> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut message)?;
        process_message(prompt, &message, &config.api).await?;
        message.clear();
    }
}

async fn process_message(prompt: &Prompt, message: &str, api: &API) -> Result<()> {
    let mut gpt_message = String::new();
    if let Some(prefix) = &prompt.prefix {
        gpt_message.push_str(prefix)
    }
    gpt_message.push_str(message);
    if let Some(postfix) = &prompt.suffix {
        gpt_message.push_str(postfix)
    }
    let stream =
        chat_completions(&api.model, vec![Message::new("system", &gpt_message)], api).await?;
    pin_mut!(stream);
    while let Some(resp) = stream.next().await {
        let resp = resp?;
        print!("{}", resp.content());
        std::io::stdout().flush()?;
    }
    Ok(())
}
