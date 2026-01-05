use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Failed to execute command: {0}")]
    ExecutionError(#[from] std::io::Error),
    #[error("Command failed with exit code: {0}")]
    NonZeroExit(i32),
}

pub fn run_command(command: &str) -> Result<(), ExecutorError> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string());

    let status = Command::new(&shell)
        .arg("-c")
        .arg(command)
        .status()?;

    if !status.success() {
        if let Some(code) = status.code() {
            return Err(ExecutorError::NonZeroExit(code));
        }
    }

    Ok(())
}
