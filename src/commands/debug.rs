// Debug command - for development and troubleshooting

use anyhow::{Context, Result};
use colored::Colorize;

pub fn show_paths() -> Result<()> {
    let home = dirs::home_dir()
        .context("Failed to get home directory")?;

    println!("{}", "GFV Paths:".cyan().bold());
    println!();

    // Config file
    let config_path = crate::config::Config::config_path()?;
    println!("Config file:");
    println!("  {}", config_path.display());
    if config_path.exists() {
        println!("  Status: {}", "EXISTS".green());
    } else {
        println!("  Status: {}", "NOT FOUND".yellow());
    }
    println!();

    // Vault directory
    let vault_dir = home.join(".gfv");
    println!("Vault directory:");
    println!("  {}", vault_dir.display());
    if vault_dir.exists() {
        println!("  Status: {}", "EXISTS".green());

        // List vaults
        if let Ok(entries) = std::fs::read_dir(&vault_dir) {
            let mut vaults = vec![];
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    if let Some(name) = entry.file_name().to_str() {
                        if name != "." && name != ".." {
                            vaults.push(name.to_string());
                        }
                    }
                }
            }
            if !vaults.is_empty() {
                println!("  Vaults found: {}", vaults.join(", "));
            }
        }
    } else {
        println!("  Status: {}", "NOT FOUND".yellow());
    }
    println!();

    Ok(())
}

pub fn clean(force: bool) -> Result<()> {
    let home = dirs::home_dir()
        .context("Failed to get home directory")?;

    let gfv_dir = home.join(".gfv");

    if !gfv_dir.exists() {
        println!("Nothing to clean - ~/.gfv does not exist");
        return Ok(());
    }

    println!("{} This will delete:", "⚠".yellow().bold());
    println!("  {}", gfv_dir.display());
    println!();
    println!("{}", "This includes:".yellow());
    println!("  - All vaults and their Git repositories");
    println!("  - All configuration");
    println!("  - All manifests");
    println!();
    println!("{} Source files (dotfiles, etc.) will NOT be deleted", "→".blue());
    println!();

    if !force {
        println!("Are you sure? Type 'yes' to confirm:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() != "yes" {
            println!("Cancelled.");
            return Ok(());
        }
    }

    std::fs::remove_dir_all(&gfv_dir)
        .context("Failed to delete ~/.gfv")?;

    println!("{} Deleted {}", "✓".green().bold(), gfv_dir.display());

    Ok(())
}
