// Status command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;

use crate::vault::Vault;
use crate::git_ops::GitRepo;
use super::helpers::{get_vault_dir, get_active_vault_name};

pub fn status(vault: Option<String>) -> Result<()> {
    // Get vault path
    let vault_name = vault.unwrap_or_else(get_active_vault_name);
    let vault_dir = get_vault_dir(&vault_name)?;
    // Vault dir obtained above

    // Check if vault is initialized
    if !Vault::is_initialized(&vault_dir) {
        bail!("Vault not initialized. Run 'gfv init' first.");
    }

    // Load vault
    let vault = Vault::load(&vault_dir)
        .context("Failed to load vault")?;

    // Display vault info
    println!("{}", "Vault Status".bold());
    println!("  Path: {}", vault_dir.display());

    if let Some(ref remote_config) = vault.manifest.remote {
        println!("  Remote: {} ({})", remote_config.url, remote_config.branch);
    } else {
        println!("  Remote: {} (local-only mode)", "None".yellow());
    }

    println!("  Managed files: {}", vault.manifest.files.len());

    // Check Git status
    let git_repo = GitRepo::open(&vault.repo_path)
        .context("Failed to open git repository")?;

    let has_changes = git_repo.has_changes()?;

    if has_changes {
        println!("\n{} Uncommitted changes in vault", "●".yellow().bold());
        println!("  Run 'gfv backup' to commit changes");
    } else {
        println!("\n{} Vault is clean", "✓".green().bold());
    }

    // Check for differences between source and vault
    println!("\n{}", "File Status:".bold());

    let mut modified = Vec::new();
    let mut missing_source = Vec::new();
    let mut up_to_date = Vec::new();

    for (vault_relative_path, entry) in &vault.manifest.files {
        let source_path = std::path::PathBuf::from(&entry.source_path);
        let vault_file_path = vault.get_file_path(vault_relative_path);

        if !source_path.exists() {
            missing_source.push(vault_relative_path.clone());
            continue;
        }

        if !vault_file_path.exists() {
            modified.push(vault_relative_path.clone());
            continue;
        }

        // Simple check: compare file sizes
        if let (Ok(source_meta), Ok(vault_meta)) = (
            fs::metadata(&source_path),
            fs::metadata(&vault_file_path)
        ) {
            if source_meta.len() != vault_meta.len() {
                modified.push(vault_relative_path.clone());
            } else {
                up_to_date.push(vault_relative_path.clone());
            }
        }
    }

    if !modified.is_empty() {
        println!("\n{} Modified files:", "●".yellow().bold());
        for file in &modified {
            println!("  {} {}", "M".yellow(), file);
        }
        println!("\n  Run 'gfv backup' to save changes");
    }

    if !missing_source.is_empty() {
        println!("\n{} Missing source files:", "!".red().bold());
        for file in &missing_source {
            println!("  {} {}", "?".red(), file);
        }
    }

    if modified.is_empty() && missing_source.is_empty() {
        println!("\n{} All files are up to date", "✓".green().bold());
    }

    Ok(())
}
