# chat-terminal

The `chat-terminal` is a CLI tool designed to simplify the process of using repetitive prompts with ChatGPT. It allows users to predefine their preferred prompt prefix or suffix, thus eliminating the need for repetitive, time-consuming input. This is particularly handy when you don't want to open new tabs or create a new session for each simple prompt when using ChatGPT on the web.

## Usage

First, clone the repository and install it on your local machine using the following commands:
```bash
git clone https://github.com/xffxff/chat-terminal.git
cargo install --path .
```

To configure `chat-terminal`, move the `.chatconfig.toml` file to `~/.config/chat/config.toml`. Alternatively, you can define your desired prompt prefix or suffix within the `~/.config/chat/config.toml` configuration file:
```toml
[prompts]
[prompts.to-ch]
description = "Translate the text into Chinese"
prefix = "Please translate the following text into Chinese: "

[prompts.to-en]
description = "Translate the text into English"
prefix = "Please translate the following text into English: "
```

Before you start using `chat-terminal`, ensure to set the `OPENAI_API_KEY` environment variable to your OpenAI API key:
```bash
export OPENAI_API_KEY="your-api-key" 
```

With `chat-terminal`, you can now access your predefined prompts quickly and easily using these simple commands:
```bash
chat to-ch "hello world"
```
```bash
chat to-en "你好世界"
```
### Interactive Mode
chat-terminal also includes an interactive mode. This mode allows you to enter your prompts directly into the terminal and receive responses in real time, providing a more engaging and dynamic user experience.

Here's an example of how to use the interactive mode:
```
chat to-ch

> hello world
你好，世界
> How are you?
你好吗？
```
> *NOTE:* Please be aware that the interactive mode is different from a typical chat session on the ChatGPT web interface. Unlike the web interface, the interactive mode does not retain the history of previous messages. It operates more like a loop of the `chat to-ch xxx` command. Each prompt is processed independently and does not have context from the previous prompts.

