// Helper functions for vault operations

use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::config::Config;

/// Get the current vault directory path
pub fn get_current_vault_dir() -> Result<PathBuf> {
    let config = Config::load()?;

    // If no vaults configured, use default
    if config.vaults.is_empty() {
        let home = dirs::home_dir()
            .context("Failed to get home directory")?;
        return Ok(home.join(".gfv").join("default"));
    }

    config.get_current_vault_dir()
}
