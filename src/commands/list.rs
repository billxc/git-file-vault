// List command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;

use crate::vault::Vault;

pub fn list(long: bool) -> Result<()> {
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

    // Check if empty
    if vault.manifest.files.is_empty() {
        println!("No files managed by gfv yet.");
        println!("\nAdd files with: gfv add <file>");
        return Ok(());
    }

    // Display files
    if long {
        // Long format with details
        println!("{} managed files:\n", vault.manifest.files.len());

        for (vault_path, entry) in &vault.manifest.files {
            println!("{}", vault_path.green().bold());
            println!("  Source: {}", entry.source_path);
            println!("  Type: {}", entry.file_type);
            if let Some(ref platform) = entry.platform {
                println!("  Platform: {}", platform);
            }
            println!("  Added: {}", entry.added_at.format("%Y-%m-%d %H:%M:%S"));
            if let Some(ref last_sync) = entry.last_sync {
                println!("  Last sync: {}", last_sync.format("%Y-%m-%d %H:%M:%S"));
            }
            println!();
        }
    } else {
        // Short format
        println!("{} managed files:\n", vault.manifest.files.len());

        let mut sorted_paths: Vec<_> = vault.manifest.files.keys().collect();
        sorted_paths.sort();

        for vault_path in sorted_paths {
            let entry = &vault.manifest.files[vault_path];
            let platform_tag = if let Some(ref p) = entry.platform {
                format!(" [{}]", p).yellow().to_string()
            } else {
                String::new()
            };

            let type_icon = if entry.file_type == "directory" {
                "📁"
            } else {
                "📄"
            };

            println!("  {} {}{}", type_icon, vault_path, platform_tag);
        }

        println!("\nUse 'gfv list --long' for detailed information.");
    }

    Ok(())
}
