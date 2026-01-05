# bashelp

Natural language to shell commands. Local-first, provider agnostic.

```
$ bashelp find all rust files modified this week

→ find . -name "*.rs" -mtime -7

[Enter to run, 'c' to copy, 'e' to edit, 'q' to quit]:
```

## Why bashelp?

Most AI shell assistants require cloud API keys. **bashelp is local-first** — it works with [ollama](https://ollama.ai) out of the box, keeping your data on your machine.

- **No API key required** to get started
- **Your shell context stays local** — nothing sent to the cloud (unless you choose a cloud provider)
- **Provider agnostic** — works with ollama, Claude, OpenAI, Gemini, Groq, Mistral, and more
- **Cross-platform** — Linux, macOS, and Windows
- **Fast** — single Rust binary, minimal dependencies

## Installation

```bash
cargo install bashelp
```

Or build from source:

```bash
git clone https://github.com/sqrew/bashelp
cd bashelp
cargo build --release
```

## Quick Start

1. **Install ollama** (if you haven't): https://ollama.ai

2. **Pull a model**:
   ```bash
   ollama pull llama3
   ```

3. **Set your default model**:
   ```bash
   bashelp use llama3
   ```

4. **Ask for help**:
   ```bash
   bashelp find files larger than 100mb
   ```

## Usage

```
bashelp <query>              Ask for a shell command (no quotes needed!)
bashelp use <model>          Set default model
bashelp config init          Create config file
bashelp config show          Show current config
bashelp --help               Show all options
```

### Flags

| Flag | Description |
|------|-------------|
| `-y, --yes` | Skip confirmation, run immediately |
| `-e, --explain` | Explain a command instead of generating one |
| `-m, --model` | Override model for this query |
| `-p, --provider` | Override provider for this query |
| `-v, --verbose` | Show debug info |
| `--dry-run` | Show command but don't execute |

### Examples

```bash
# Generate a command (no quotes needed!)
bashelp compress this folder

# Run without confirmation
bashelp -y update system packages

# Explain a command you don't understand
bashelp --explain "tar -xzvf"

# Use a specific model for one query
bashelp -m mistral disk usage by folder

# Use a different provider
bashelp -p groq -m llama-3.3-70b-versatile list docker containers
```

## Supported Providers

### Local (No API Key Required)

| Provider | Aliases | Default Endpoint |
|----------|---------|------------------|
| **ollama** | - | `http://localhost:11434` |

### Cloud Providers

| Provider | Aliases | Models |
|----------|---------|--------|
| **claude** | `anthropic` | `claude-3-5-haiku-20241022`, `claude-3-5-sonnet-20241022`, etc. |
| **openai** | `chatgpt`, `gpt` | `gpt-4o`, `gpt-4o-mini`, etc. |
| **gemini** | `google` | `gemini-1.5-flash`, `gemini-1.5-pro`, etc. |
| **grok** | `xai` | `grok-2`, etc. |
| **groq** | - | `llama-3.3-70b-versatile`, `mixtral-8x7b-32768`, etc. |
| **mistral** | - | `mistral-large-latest`, `mistral-small-latest`, etc. |
| **perplexity** | `pplx` | `llama-3.1-sonar-small-128k-online`, etc. |
| **together** | - | `meta-llama/Llama-3-70b-chat-hf`, etc. |
| **fireworks** | - | `accounts/fireworks/models/llama-v3-70b-instruct`, etc. |
| **deepseek** | - | `deepseek-chat`, `deepseek-coder`, etc. |
| **openrouter** | - | Any model available on OpenRouter |
| **openai-compatible** | `custom` | Any OpenAI-compatible API (bring your own endpoint) |

## Configuration

Config lives at `~/.config/bashelp/config.toml`:

```toml
[provider]
name = "ollama"
model = "llama3"
endpoint = "http://localhost:11434"
# api_key = "your-key-here"  # for cloud providers

[behavior]
confirm = true
dangerous_warn = true
```

### Setting Up Cloud Providers

```bash
# Claude
bashelp config set provider.name claude
bashelp config set provider.api_key sk-ant-...
bashelp use claude-3-5-haiku-20241022

# OpenAI
bashelp config set provider.name openai
bashelp config set provider.api_key sk-...
bashelp use gpt-4o-mini

# Groq (fast & free tier!)
bashelp config set provider.name groq
bashelp config set provider.api_key gsk_...
bashelp use llama-3.3-70b-versatile

# Gemini
bashelp config set provider.name gemini
bashelp config set provider.api_key ...
bashelp use gemini-1.5-flash

# Custom OpenAI-compatible endpoint
bashelp config set provider.name openai-compatible
bashelp config set provider.endpoint https://your-api.com/v1/chat/completions
bashelp config set provider.api_key your-key
bashelp use your-model
```

## License

MIT

## Contributing

PRs welcome! This project is built with love and Rust.

---

Made by [sqrew](https://github.com/sqrew) with help from Claude. 🦀
