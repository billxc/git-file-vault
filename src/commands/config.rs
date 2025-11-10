// Config command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;

use crate::vault::Vault;
use crate::config::Config;
use super::helpers::{get_vault_dir, get_active_vault_name};

pub fn config(
    key: Option<String>,
    value: Option<String>,
    list: bool,
    unset: Option<String>,
) -> Result<()> {
    // Get vault path
    let vault_name = get_active_vault_name();
    let vault_dir = get_vault_dir(&vault_name)?;
    // Vault dir obtained above

    // Check if vault is initialized
    if !Vault::is_initialized(&vault_dir) {
        bail!("Vault not initialized. Run 'gfv init' first.");
    }

    // Load vault
    let vault = Vault::load(&vault_dir)
        .context("Failed to load vault")?;

    if list {
        // List all configuration
        println!("{}", "Vault Configuration:".bold());
        println!("\n{}", "General:".bold());
        println!("  vault.dir = {}", vault.vault_dir.display());
        println!("  vault.repo = {}", vault.repo_path.display());

        if let Some(ref remote_config) = vault.manifest.remote {
            println!("\n{}", "Remote:".bold());
            println!("  remote.url = {}", remote_config.url);
            println!("  remote.branch = {}", remote_config.branch);
        } else {
            println!("\n{}", "Remote:".bold());
            println!("  {} (local-only mode)", "Not configured".yellow());
        }

        // AI configuration (read from global config)
        let global_config = Config::load().unwrap_or_default();
        println!("\n{}", "AI:".bold());
        if let Some(ref endpoint) = global_config.ai.endpoint {
            println!("  ai.endpoint = {}", endpoint);
        } else {
            println!("  ai.endpoint = {}", "Not configured".yellow());
        }
        if global_config.ai.api_key.is_some() {
            println!("  ai.api_key = {}", "****** (set)".green());
        } else {
            println!("  ai.api_key = {}", "Not configured".yellow());
        }
        if let Some(ref model) = global_config.ai.model {
            println!("  ai.model = {}", model);
        } else {
            println!("  ai.model = {}", "Not configured".yellow());
        }

        return Ok(());
    }

    if let Some(unset_key) = unset {
        // Unset a key - support AI config keys
        if unset_key.starts_with("ai.") {
            let mut global_config = Config::load().unwrap_or_default();

            match unset_key.as_str() {
                "ai.endpoint" => {
                    global_config.ai.endpoint = None;
                    global_config.save()?;
                    println!("{} Unset ai.endpoint", "✓".green());
                }
                "ai.api_key" => {
                    global_config.ai.api_key = None;
                    global_config.save()?;
                    println!("{} Unset ai.api_key", "✓".green());
                }
                "ai.model" => {
                    global_config.ai.model = None;
                    global_config.save()?;
                    println!("{} Unset ai.model", "✓".green());
                }
                _ => {
                    bail!("Unknown configuration key: {}", unset_key);
                }
            }
            return Ok(());
        }

        println!("Unsetting configuration is not yet implemented: {}", unset_key);
        bail!("Feature not implemented");
    }

    if let (Some(ref k), Some(ref v)) = (&key, &value) {
        // Set a key - support AI config keys
        if k.starts_with("ai.") {
            let mut global_config = Config::load().unwrap_or_default();

            match k.as_str() {
                "ai.endpoint" => {
                    global_config.ai.endpoint = Some(v.clone());
                    global_config.save()?;
                    println!("{} Set ai.endpoint = {}", "✓".green(), v);
                }
                "ai.api_key" => {
                    global_config.ai.api_key = Some(v.clone());
                    global_config.save()?;
                    println!("{} Set ai.api_key = ******", "✓".green());
                }
                "ai.model" => {
                    global_config.ai.model = Some(v.clone());
                    global_config.save()?;
                    println!("{} Set ai.model = {}", "✓".green(), v);
                }
                _ => {
                    bail!("Unknown configuration key: {}", k);
                }
            }
            return Ok(());
        }

        println!("Setting configuration is not yet implemented: {} = {}", k, v);
        bail!("Feature not implemented");
    }

    if let Some(k) = key {
        // Get a single key
        match k.as_str() {
            "vault.dir" => {
                println!("{}", vault.vault_dir.display());
            }
            "vault.repo" => {
                println!("{}", vault.repo_path.display());
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
            "ai.endpoint" => {
                let global_config = Config::load().unwrap_or_default();
                if let Some(ref endpoint) = global_config.ai.endpoint {
                    println!("{}", endpoint);
                } else {
                    println!("{}", "Not configured".yellow());
                }
            }
            "ai.api_key" => {
                let global_config = Config::load().unwrap_or_default();
                if global_config.ai.api_key.is_some() {
                    println!("{}", "****** (set)".green());
                } else {
                    println!("{}", "Not configured".yellow());
                }
            }
            "ai.model" => {
                let global_config = Config::load().unwrap_or_default();
                if let Some(ref model) = global_config.ai.model {
                    println!("{}", model);
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
