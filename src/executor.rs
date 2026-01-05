use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Failed to execute command: {0}")]
    ExecutionError(#[from] std::io::Error),
    #[error("Command failed with exit code: {0}")]
    NonZeroExit(i32),
}

#[cfg(windows)]
fn get_shell() -> (&'static str, &'static str) {
    ("cmd", "/c")
}

#[cfg(not(windows))]
fn get_shell() -> (String, &'static str) {
    (std::env::var("SHELL").unwrap_or_else(|_| "sh".to_string()), "-c")
}

pub fn run_command(command: &str) -> Result<(), ExecutorError> {
    let (shell, flag) = get_shell();

    let status = Command::new(&shell)
        .arg(flag)
        .arg(command)
        .status()?;

    if !status.success() {
        if let Some(code) = status.code() {
            return Err(ExecutorError::NonZeroExit(code));
        }
    }

    Ok(())
}
