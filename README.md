# bashelp

Natural language to shell commands. Local-first, provider agnostic.

```
$ bashelp "find all rust files modified this week"

→ find . -name "*.rs" -mtime -7

[Enter to run, 'c' to copy, 'e' to edit, 'q' to quit]:
```

## Why bashelp?

Most AI shell assistants require an OpenAI API key and send your commands to the cloud. **bashelp is local-first** — it works with [ollama](https://ollama.ai) out of the box, keeping your data on your machine.

- **No API key required** to get started
- **Your shell context stays local** — nothing sent to the cloud
- **Provider agnostic** — works with ollama, OpenAI, Claude, Groq, or any compatible API
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
   bashelp "list files larger than 100mb"
   ```

## Usage

```
bashelp <query>              Ask for a shell command
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
# Generate a command
bashelp "compress this folder"

# Run without confirmation
bashelp -y "update system packages"

# Explain a command you don't understand
bashelp --explain "tar -xzvf"

# Use a specific model for one query
bashelp -m mistral "disk usage by folder"
```

## Configuration

Config lives at `~/.config/bashelp/config.toml`:

```toml
[provider]
name = "ollama"
model = "llama3"
endpoint = "http://localhost:11434"

[behavior]
confirm = true
dangerous_warn = true
```

### Using Other Providers

**OpenAI:**
```bash
bashelp config set provider.name openai
bashelp config set provider.api_key sk-...
bashelp config set provider.model gpt-4
```

**Claude:**
```bash
bashelp config set provider.name claude
bashelp config set provider.api_key sk-ant-...
bashelp config set provider.model claude-3-sonnet
```

## Supported Providers

| Provider | Status | Local | API Key Required |
|----------|--------|-------|------------------|
| ollama | ✅ Works | Yes | No |
| OpenAI | 🚧 Planned | No | Yes |
| Claude | 🚧 Planned | No | Yes |
| Groq | 🚧 Planned | No | Yes |

## License

MIT

## Contributing

PRs welcome! This project is built with love and Rust.

---

Made by [sqrew](https://github.com/sqrew) with help from Claude.
