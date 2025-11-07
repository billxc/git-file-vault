// Helper functions for vault operations

use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::config::Config;

/// Get the vault directory path by name
/// If name is provided, use that specific vault
/// Otherwise, use the active vault from config
pub fn get_vault_dir(vault_name: &str) -> Result<PathBuf> {
    let config = Config::load()?;

    // Try to get the specified vault
    if let Some(path) = config.get_vault_dir(vault_name) {
        return Ok(path);
    }

    // If vault doesn't exist in config, check if it's "default" and use default path
    if vault_name == "default" && config.vaults.is_empty() {
        let home = dirs::home_dir()
            .context("Failed to get home directory")?;
        return Ok(home.join(".gfv").join("default"));
    }

    Err(anyhow::anyhow!("Vault '{}' not found", vault_name))
}

/// Get the active vault name from config
/// Always returns a value, defaults to "default" if no config
pub fn get_active_vault_name() -> String {
    let config = Config::load().ok();
    config
        .and_then(|c| {
            if c.vaults.is_empty() {
                None
            } else {
                Some(c.current.active.clone())
            }
        })
        .unwrap_or_else(|| "default".to_string())
}
