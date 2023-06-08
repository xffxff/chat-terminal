# dry-prompt (do not repeat your prompt)

Finding myself constantly repeating the same prompt when using ChatGPT, such as "Please translate the following text into Chinese: " or "Please edit the following text to make it more natural" was becoming frustrating. To simplify the process without opening new tabs or creating a new session for each simple prompt when using ChatGPT on the web, I decided to create a CLI tool that can be used within the terminal called "dry-prompt."

dry-prompt allows users to predefine their preferred prompt prefix or suffix, avoiding repetitive, time-consuming input.

## Usage

Clone the repository and install to your local machine:
```bash
git clone https://github.com/xffxff/dry-prompt.git
cargo install --path .
```

To get started, move the `.chatconfig.toml` file to `~/.config/chat/config.toml` or define your desired prompt prefix or suffix within the `~/.config/chat/config.toml` configuration file:
```toml
[prompts]
[prompts.to-ch]
description = "Translate the text into Chinese"
prefix = "Please translate the following text into Chinese: "

[prompts.to-en]
description = "Translate the text into English"
prefix = "Please translate the following text into English: "
```

Before using the tool, make sure to set the `OPENAI_API_KEY` environment variable to your OpenAI API key:
```bash
export OPENAI_API_KEY="your-api-key" 
```

Now, quickly and easily access your predefined prompts with these simple commands:
```bash
chat to-ch "hello world"
```
```bash
chat to-en "你好世界"
```