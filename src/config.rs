// Config module - manages global configuration

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub vaults: std::collections::HashMap<String, String>,
    pub current: CurrentConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub sync: SyncConfig,
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
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            conflict_strategy: default_conflict_strategy(),
        }
    }
}

fn default_conflict_strategy() -> String {
    "prompt".to_string()
}
