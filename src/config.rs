use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found. Run `bashelp config init` to create one.")]
    NotFound,
    #[error("Failed to read config: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),
    #[error("Unknown config key: {0}")]
    UnknownKey(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub provider: ProviderConfig,
    pub behavior: BehaviorConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub name: String,
    pub model: String,
    pub endpoint: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BehaviorConfig {
    pub confirm: bool,
    pub context_lines: usize,
    pub dangerous_warn: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: ProviderConfig {
                name: "ollama".to_string(),
                model: "llama3".to_string(),
                endpoint: Some("http://localhost:11434".to_string()),
                api_key: None,
            },
            behavior: BehaviorConfig {
                confirm: true,
                context_lines: 10,
                dangerous_warn: true,
            },
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("bashelp")
            .join("config.toml")
    }

    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path();

        if !path.exists() {
            // Return default config if no config file exists
            return Ok(Config::default());
        }

        let content = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::config_path();

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    pub fn init_default() -> Result<(), ConfigError> {
        let config = Config::default();
        config.save()?;
        Ok(())
    }
}

pub fn set_value(key: &str, value: &str) -> Result<(), ConfigError> {
    let mut config = Config::load()?;

    match key {
        "provider.name" => config.provider.name = value.to_string(),
        "provider.model" => config.provider.model = value.to_string(),
        "provider.endpoint" => config.provider.endpoint = Some(value.to_string()),
        "provider.api_key" => config.provider.api_key = Some(value.to_string()),
        "behavior.confirm" => config.behavior.confirm = value.parse().unwrap_or(true),
        "behavior.context_lines" => config.behavior.context_lines = value.parse().unwrap_or(10),
        "behavior.dangerous_warn" => config.behavior.dangerous_warn = value.parse().unwrap_or(true),
        _ => return Err(ConfigError::UnknownKey(key.to_string())),
    }

    config.save()?;
    Ok(())
}

pub fn get_value(key: &str) -> Result<String, ConfigError> {
    let config = Config::load()?;

    match key {
        "provider.name" => Ok(config.provider.name),
        "provider.model" => Ok(config.provider.model),
        "provider.endpoint" => Ok(config.provider.endpoint.unwrap_or_default()),
        "provider.api_key" => Ok(config.provider.api_key.unwrap_or_else(|| "[not set]".to_string())),
        "behavior.confirm" => Ok(config.behavior.confirm.to_string()),
        "behavior.context_lines" => Ok(config.behavior.context_lines.to_string()),
        "behavior.dangerous_warn" => Ok(config.behavior.dangerous_warn.to_string()),
        _ => Err(ConfigError::UnknownKey(key.to_string())),
    }
}
