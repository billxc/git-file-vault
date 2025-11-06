// Config command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;

use crate::vault::Vault;

pub fn config(
    key: Option<String>,
    value: Option<String>,
    list: bool,
    unset: Option<String>,
) -> Result<()> {
    // Get vault path
    let home = dirs::home_dir()
        .context("Failed to get home directory")?;
    let vault_path = home.join(".gfv");

    // Check if vault is initialized
    if !Vault::is_initialized(&vault_path) {
        bail!("Vault not initialized. Run 'gfv init' first.");
    }

    // Load vault
    let vault = Vault::load(&vault_path)
        .context("Failed to load vault")?;

    if list {
        // List all configuration
        println!("{}", "Vault Configuration:".bold());
        println!("\n{}", "General:".bold());
        println!("  vault.path = {}", vault.path.display());

        if let Some(ref remote_config) = vault.manifest.remote {
            println!("\n{}", "Remote:".bold());
            println!("  remote.url = {}", remote_config.url);
            println!("  remote.branch = {}", remote_config.branch);
        } else {
            println!("\n{}", "Remote:".bold());
            println!("  {} (local-only mode)", "Not configured".yellow());
        }

        // TODO: AI configuration (read from global config)
        println!("\n{}", "AI:".bold());
        println!("  {} (not implemented yet)", "Not configured".yellow());

        return Ok(());
    }

    if let Some(unset_key) = unset {
        // Unset a key
        println!("Unsetting configuration is not yet implemented: {}", unset_key);
        bail!("Feature not implemented");
    }

    if let (Some(ref k), Some(ref v)) = (&key, &value) {
        // Set a key
        println!("Setting configuration is not yet implemented: {} = {}", k, v);
        bail!("Feature not implemented");
    }

    if let Some(k) = key {
        // Get a single key
        match k.as_str() {
            "vault.path" => {
                println!("{}", vault.path.display());
            }
            "remote.url" => {
                if let Some(ref remote_config) = vault.manifest.remote {
                    println!("{}", remote_config.url);
                } else {
                    println!("{}", "Not configured".yellow());
                }
            }
            "remote.branch" => {
                if let Some(ref remote_config) = vault.manifest.remote {
                    println!("{}", remote_config.branch);
                } else {
                    println!("{}", "Not configured".yellow());
                }
            }
            _ => {
                bail!("Unknown configuration key: {}", k);
            }
        }

        return Ok(());
    }

    // No arguments - show help
    println!("{}", "Usage:".bold());
    println!("  gfv config --list              # List all configuration");
    println!("  gfv config <key>               # Get a configuration value");
    println!("  gfv config <key> <value>       # Set a configuration value");
    println!("  gfv config --unset <key>       # Unset a configuration value");

    Ok(())
}
