use arboard::Clipboard;
use colored::Colorize;
use std::io::{self, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Clipboard error: {0}")]
    ClipboardError(#[from] arboard::Error),
}

/// What the user chose to do
pub enum UserAction {
    Run,
    Copy,
    Edit,
    Quit,
}

pub fn display_suggestion(command: &str) {
    println!();
    println!("{} {}", "→".cyan(), command.bright_white().bold());
    println!();
}

pub fn confirm_execution(command: &str) -> Result<UserAction, UiError> {
    print!("{}", "[Enter to run, 'c' to copy, 'e' to edit, 'q' to quit]: ".dimmed());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "" | "y" | "yes" | "r" | "run" => Ok(UserAction::Run),
        "c" | "copy" => {
            copy_to_clipboard(command)?;
            println!("{}", "copied to clipboard".green());
            Ok(UserAction::Copy)
        }
        "e" | "edit" => {
            // TODO: implement edit mode
            println!("{}", "edit mode not yet implemented".yellow());
            Ok(UserAction::Edit)
        }
        "q" | "n" | "no" | "quit" => {
            println!("{}", "cancelled".yellow());
            Ok(UserAction::Quit)
        }
        _ => {
            println!("{}", "cancelled".yellow());
            Ok(UserAction::Quit)
        }
    }
}

fn copy_to_clipboard(text: &str) -> Result<(), UiError> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}
