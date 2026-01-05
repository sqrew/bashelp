#+TITLE: shai - Shell AI
#+DATE: <2026-01-05 Sun>
#+DESCRIPTION: Natural language to bash. Any LLM. One binary. A shy little helper.

* Overview

shai (Shell AI) is a CLI tool that translates natural language into shell commands using any LLM provider.

** Elevator Pitch

"Tell your terminal what you want in plain English. Works with any LLM."

A shy little helper that whispers bash commands to you.

** Why This Could Pop

- Nothing definitive exists yet in this space
- Rust = single binary, fast, easy install
- API agnostic = works with whatever people already use
- Simple concept, immediately useful
- Screenshots/demos are compelling
- Daily use tool (the ripgrep pattern)
- Cute name with personality

* Core Flow

#+begin_src
User: shai "find large files in this directory"
        │
        ▼
┌───────────────────┐
│ shai CLI          │
│ - parse input     │
│ - load config     │
│ - build prompt    │
└────────┬──────────┘
         │
         ▼
┌───────────────────┐
│ LLM Provider      │
│ (ollama/openai/   │
│  claude/groq/etc) │
└────────┬──────────┘
         │
         ▼
┌───────────────────┐
│ Response Parser   │
│ - extract command │
│ - validate        │
└────────┬──────────┘
         │
         ▼
┌───────────────────┐
│ Confirmation      │
│ [Enter/e/q]       │
└────────┬──────────┘
         │ (if confirmed)
         ▼
┌───────────────────┐
│ Execute & Display │
└───────────────────┘
#+end_src

* Features

** MVP (v0.1)

- [ ] Natural language input → bash command output
- [ ] Single provider support (start with ollama)
- [ ] Confirmation before execution
- [ ] Basic config file (~/.config/shai/config.toml)
- [ ] Context: current working directory
- [ ] --yes flag to skip confirmation (dangerous but useful)

** v0.2

- [ ] Multiple provider support (ollama, openai, claude, groq)
- [ ] Provider switching via config or flag
- [ ] Shell history context (last N commands)
- [ ] Edit command before running (open in $EDITOR or inline)
- [ ] Explain mode: shai --explain "command" (explain what a command does)

** v0.3

- [ ] Conversation mode (follow-up questions)
- [ ] Command history/learning (remember what worked)
- [ ] Fish/zsh/bash completion
- [ ] Pipe support: echo "task" | shai
- [ ] OS-aware (different commands for linux/mac)

** Future / Maybe

- [ ] Plugin system for custom providers
- [ ] Local model support (llama.cpp direct, no ollama)
- [ ] TUI mode for browsing suggestions
- [ ] "Dangerous command" warnings (rm -rf, etc)
- [ ] Undo support (where possible)

* Architecture

** Crates / Modules

#+begin_src
shai/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, CLI parsing
│   ├── config.rs         # Config loading/saving
│   ├── provider/
│   │   ├── mod.rs        # Provider trait
│   │   ├── ollama.rs     # Ollama implementation
│   │   ├── openai.rs     # OpenAI implementation
│   │   ├── claude.rs     # Claude/Anthropic implementation
│   │   └── groq.rs       # Groq implementation
│   ├── prompt.rs         # Prompt building with context
│   ├── parser.rs         # Response parsing (extract command)
│   ├── executor.rs       # Command execution
│   └── ui.rs             # Confirmation UI, colors, output
└── README.md
#+end_src

** Provider Trait

#+begin_src rust
#[async_trait]
pub trait Provider {
    async fn complete(&self, prompt: &str) -> Result<String>;
    fn name(&self) -> &str;
}
#+end_src

** Config Structure

#+begin_src toml
# ~/.config/shai/config.toml

[provider]
name = "ollama"           # ollama | openai | claude | groq
model = "llama3"          # model name
endpoint = "http://localhost:11434"  # optional, for self-hosted

[provider.openai]
api_key = "sk-..."        # or use OPENAI_API_KEY env var

[provider.claude]
api_key = "sk-ant-..."    # or use ANTHROPIC_API_KEY env var

[behavior]
confirm = true            # require confirmation before running
context_lines = 10        # lines of shell history to include
dangerous_warn = true     # warn on rm -rf, etc
#+end_src

* Prompt Engineering

** System Prompt (v1)

#+begin_example
You are a shell command generator. The user will describe what they want to do, and you output ONLY the shell command to accomplish it. No explanation, no markdown, no code blocks - just the raw command.

Context:
- Operating system: {os}
- Current directory: {cwd}
- Shell: {shell}
- Recent commands:
{history}

Rules:
- Output ONLY the command, nothing else
- Use common, portable commands when possible
- If the task is ambiguous, make reasonable assumptions
- If the task is impossible or dangerous, output: ERROR: <reason>
#+end_example

** Example Exchanges

| Input | Output |
|-------|--------|
| "find rust files modified today" | find . -name "*.rs" -mtime 0 |
| "disk usage sorted by size" | du -sh * \vert sort -h |
| "kill process on port 3000" | kill $(lsof -t -i:3000) |
| "compress this folder" | tar -czvf archive.tar.gz . |
| "search for TODO in all files" | grep -r "TODO" . |

* CLI Interface

#+begin_src
shai - Natural language to shell commands

USAGE:
    shai <query>              Execute natural language query
    shai --explain <cmd>      Explain what a command does
    shai config set <k> <v>   Set config value
    shai config get <k>       Get config value
    shai config show          Show all config
    shai --help               Show help

FLAGS:
    -y, --yes              Skip confirmation, run immediately
    -e, --explain          Explain mode (describe a command)
    -p, --provider <name>  Override provider for this query
    -m, --model <name>     Override model for this query
    -v, --verbose          Show debug info (prompt, response, etc)
    --dry-run              Show command but don't offer to execute

EXAMPLES:
    shai "list files larger than 100mb"
    shai "git commits from last week"
    shai --yes "update all packages"
    shai --explain "tar -xzvf"
#+end_src

* Dependencies

#+begin_src toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
clap = { version = "4", features = ["derive"] }
dirs = "5"                    # for config paths
colored = "2"                 # terminal colors
dialoguer = "0.11"            # confirmations/prompts
async-trait = "0.1"
thiserror = "1"
#+end_src

* Competition / Existing Tools

| Tool | Language | Providers | Notes |
|------|----------|-----------|-------|
| shell-gpt | Python | OpenAI mainly | Popular but Python, slow startup |
| github copilot cli | ??? | GitHub/OpenAI | Locked to GitHub ecosystem |
| aichat | Rust | Multiple | More of a chat tool, not shell-focused |
| mods | Go | Multiple | By Charm, more general purpose |

** Our Angle

- Rust (fast, single binary)
- Shell-focused (not general chat)
- Truly provider agnostic
- Simple, does one thing well
- Pretty output, good UX
- Personality (the shy helper)

* Marketing / Launch

** README Must-Haves

- Sick GIF demo at the top
- One-liner install
- 3-step quickstart
- Provider setup guides
- "Why shai?" section

** Places to Post

- r/rust
- r/commandline
- r/linux
- Hacker News
- Lobste.rs
- Twitter/X
- Mastodon

** Taglines

- "Talk to your terminal"
- "Natural language to bash. Any LLM. One binary."
- "Stop googling shell commands"
- "A shy little AI that whispers bash commands"
- "Your shy shell helper"

* Open Questions

- [X] Name: shai (Shell AI, sounds like "shy")
- [ ] License: MIT? Apache?
- [ ] Should explain mode be default or require flag?
- [ ] How to handle multi-line commands / scripts?
- [ ] Windows support? (PowerShell translation?)

* References

- [[https://github.com/TheR1D/shell_gpt][shell-gpt]]
- [[https://github.com/charmbracelet/mods][mods by Charm]]
- [[https://github.com/sigoden/aichat][aichat]]

* Notes

Space for random thoughts as we build...

