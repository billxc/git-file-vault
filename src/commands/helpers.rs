// Helper functions for vault operations

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Get the current vault directory path
/// For now, always returns ~/.gfv/default/
/// TODO: Read from global config to support multiple vaults
pub fn get_current_vault_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Failed to get home directory")?;

    Ok(home.join(".gfv").join("default"))
}

/// Get the current vault name
/// TODO: Read from global config
pub fn get_current_vault_name() -> String {
    "default".to_string()
}
