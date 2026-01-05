mod config;
mod provider;
mod prompt;
mod executor;
mod ui;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "bashelp")]
#[command(about = "Natural language to shell commands. Local-first, provider agnostic.")]
#[command(version)]
#[command(trailing_var_arg = true)]
struct Cli {
    /// The natural language query (no quotes needed)
    #[arg(trailing_var_arg = true)]
    query: Vec<String>,

    /// Skip confirmation, run immediately
    #[arg(short = 'y', long)]
    yes: bool,

    /// Explain what a command does instead of generating one
    #[arg(short = 'e', long)]
    explain: bool,

    /// Override provider for this query
    #[arg(short = 'p', long)]
    provider: Option<String>,

    /// Override model for this query
    #[arg(short = 'm', long)]
    model: Option<String>,

    /// Show debug info
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Show command but don't offer to execute
    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Set the default model (e.g., shai use mistral)
    Use {
        /// Model name to use as default
        model: String,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show all configuration
    Show,
    /// Set a config value
    Set { key: String, value: String },
    /// Get a config value
    Get { key: String },
    /// Initialize default config
    Init,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        return handle_command(command).await;
    }

    // Need a query for main operation
    let query = if cli.query.is_empty() {
        eprintln!("{}", "Usage: bashelp how do I list files".yellow());
        eprintln!("       bashelp --help for more options");
        std::process::exit(1);
    } else {
        cli.query.join(" ")
    };

    // Load config
    let cfg = config::Config::load()?;

    // Override provider/model if specified
    let provider_name = cli.provider.as_deref().unwrap_or(&cfg.provider.name);
    let model_name = cli.model.as_deref().unwrap_or(&cfg.provider.model);

    if cli.verbose {
        eprintln!("{}: {} / {}", "Provider".dimmed(), provider_name, model_name);
    }

    // Build the provider
    let provider = provider::create_provider(provider_name, model_name, &cfg)?;

    // Build prompt with context
    let full_prompt = prompt::build_prompt(&query, cli.explain)?;

    if cli.verbose {
        eprintln!("{}\n{}", "Prompt:".dimmed(), full_prompt.dimmed());
    }

    // Get completion from LLM
    eprintln!("{}", "thinking...".dimmed());
    let response = provider.complete(&full_prompt).await?;

    // Parse the response
    let command = prompt::parse_response(&response)?;

    if cli.verbose {
        eprintln!("{}: {}", "Raw response".dimmed(), response.dimmed());
    }

    // Display the suggestion
    ui::display_suggestion(&command);

    // Handle execution
    if cli.dry_run {
        return Ok(());
    }

    if cli.yes {
        executor::run_command(&command)?;
    } else {
        match ui::confirm_execution(&command)? {
            ui::UserAction::Run => executor::run_command(&command)?,
            ui::UserAction::Copy => {} // Already handled in confirm_execution
            ui::UserAction::Edit => {} // TODO
            ui::UserAction::Quit => {}
        }
    }

    Ok(())
}

async fn handle_command(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Config { action } => match action {
            ConfigAction::Show => {
                let cfg = config::Config::load()?;
                println!("{}", toml::to_string_pretty(&cfg)?);
            }
            ConfigAction::Init => {
                config::Config::init_default()?;
                println!("{}", "Config initialized at ~/.config/bashelp/config.toml".green());
            }
            ConfigAction::Set { key, value } => {
                config::set_value(&key, &value)?;
                println!("{} {} = {}", "Set".green(), key, value);
            }
            ConfigAction::Get { key } => {
                let value = config::get_value(&key)?;
                println!("{}", value);
            }
        },
        Commands::Use { model } => {
            config::set_value("provider.model", &model)?;
            println!("{} {}", "Now using:".green(), model.bright_white().bold());
        }
    }
    Ok(())
}
