use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PromptError {
    #[error("Failed to get current directory: {0}")]
    CwdError(#[from] std::io::Error),
    #[error("LLM returned an error: {0}")]
    LlmError(String),
}

const SYSTEM_PROMPT: &str = r#"You are a shell command generator. The user will describe what they want to do, and you output ONLY the shell command to accomplish it. No explanation, no markdown, no code blocks - just the raw command.

Context:
- Operating system: {os}
- Current directory: {cwd}
- Shell: {shell}

Rules:
- Output ONLY the command, nothing else
- Use common, portable commands when possible
- If the task is ambiguous, make reasonable assumptions
- If the task is impossible or dangerous, output: ERROR: <reason>
"#;

const EXPLAIN_PROMPT: &str = r#"You are a shell command explainer. The user will provide a shell command, and you will explain what it does in simple terms. Be concise but thorough.

Command to explain: {command}

Explain what this command does, including any flags or arguments.
"#;

pub fn build_prompt(query: &str, explain_mode: bool) -> Result<String, PromptError> {
    if explain_mode {
        return Ok(EXPLAIN_PROMPT.replace("{command}", query));
    }

    let os = env::consts::OS;
    let cwd = env::current_dir()?.display().to_string();
    let shell = env::var("SHELL").unwrap_or_else(|_| "bash".to_string());

    let system = SYSTEM_PROMPT
        .replace("{os}", os)
        .replace("{cwd}", &cwd)
        .replace("{shell}", &shell);

    Ok(format!("{}\n\nUser request: {}", system, query))
}

pub fn parse_response(response: &str) -> Result<String, PromptError> {
    let trimmed = response.trim();

    // Check if LLM returned an error
    if trimmed.starts_with("ERROR:") {
        return Err(PromptError::LlmError(
            trimmed.strip_prefix("ERROR:").unwrap_or(trimmed).trim().to_string()
        ));
    }

    // Strip markdown code blocks if present (LLMs sometimes add them despite instructions)
    let command = trimmed
        .strip_prefix("```bash")
        .or_else(|| trimmed.strip_prefix("```sh"))
        .or_else(|| trimmed.strip_prefix("```"))
        .unwrap_or(trimmed);

    let command = command.strip_suffix("```").unwrap_or(command);

    Ok(command.trim().to_string())
}
