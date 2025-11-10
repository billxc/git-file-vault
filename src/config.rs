// Config module - manages global configuration

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub vaults: HashMap<String, String>,
    pub current: CurrentConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub sync: SyncConfig,
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentConfig {
    pub active: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncConfig {
    #[serde(default = "default_conflict_strategy")]
    pub conflict_strategy: String,
    #[serde(default = "default_branch")]
    pub default_branch: String,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            conflict_strategy: default_conflict_strategy(),
            default_branch: default_branch(),
        }
    }
}

fn default_conflict_strategy() -> String {
    "prompt".to_string()
}

fn default_branch() -> String {
    "main".to_string()
}

impl Config {
    /// Get the config file path
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Failed to get home directory")?;
        Ok(home.join(".gfv").join("config.toml"))
    }

    /// Load config from file, or return default if not exists
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            // Return default config
            return Ok(Self {
                vaults: HashMap::new(),
                current: CurrentConfig {
                    active: "default".to_string(),
                },
                ai: Default::default(),
                sync: Default::default(),
                aliases: HashMap::new(),
            });
        }

        let content = std::fs::read_to_string(&config_path)
            .context("Failed to read config file")?;

        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        std::fs::write(&config_path, content)
            .context("Failed to write config file")?;

        Ok(())
    }

    /// Get the current active vault directory
    pub fn get_active_vault_dir(&self) -> Result<PathBuf> {
        let vault_path = self.vaults.get(&self.current.active)
            .ok_or_else(|| anyhow::anyhow!(
                "Active vault '{}' not found in config",
                self.current.active
            ))?;

        Ok(PathBuf::from(vault_path))
    }

    /// Get a specific vault directory by name
    pub fn get_vault_dir(&self, name: &str) -> Option<PathBuf> {
        self.vaults.get(name).map(|path| PathBuf::from(path))
    }
}
