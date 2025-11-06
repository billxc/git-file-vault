// Remove command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;

use crate::vault::Vault;
use crate::git_ops::GitRepo;

pub fn remove(
    file: String,
    delete_files: bool,
) -> Result<()> {
    // Get vault path
    let vault_dir = super::helpers::get_current_vault_dir()?;
    // Vault dir obtained above

    // Check if vault is initialized
    if !Vault::is_initialized(&vault_dir) {
        bail!("Vault not initialized. Run 'gfv init' first.");
    }

    // Load vault
    let mut vault = Vault::load(&vault_dir)
        .context("Failed to load vault")?;

    // Check if file is in manifest
    if vault.manifest.get_file(&file).is_none() {
        bail!("File '{}' is not managed by gfv.\n\nList managed files with: gfv list", file);
    }

    println!("{} Removing {} from vault...", "==>".green().bold(), file);

    // Remove from manifest
    let entry = vault.manifest.remove_file(&file)
        .expect("File should exist"); // We just checked above

    // Save updated manifest
    vault.save_manifest()
        .context("Failed to save manifest")?;

    println!("  {} Removed from manifest", "✓".green());

    // Delete from vault if requested
    if delete_files {
        let vault_file_path = vault.get_file_path(&file);

        if vault_file_path.exists() {
            if vault_file_path.is_dir() {
                fs::remove_dir_all(&vault_file_path)
                    .context("Failed to delete directory from vault")?;
            } else {
                fs::remove_file(&vault_file_path)
                    .context("Failed to delete file from vault")?;
            }

            println!("  {} Deleted from vault", "✓".green());
        }
    } else {
        println!("  {} Kept files in vault (use --delete-files to remove)",
            "→".blue()
        );
    }

    // Commit changes
    let git_repo = GitRepo::open(&vault.repo_path)
        .context("Failed to open git repository")?;

    git_repo.add_all()
        .context("Failed to stage changes")?;

    let commit_message = format!("Remove {}", file);
    git_repo.commit(&commit_message)
        .context("Failed to commit changes")?;

    println!("  {} Committed changes", "✓".green());

    println!("\n{} is no longer managed by gfv",
        if entry.file_type == "directory" { "Directory" } else { "File" }
    );
    println!("Source file location unchanged: {}", entry.source_path);

    Ok(())
}
