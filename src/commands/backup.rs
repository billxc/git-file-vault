// Backup command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;

use crate::vault::Vault;
use crate::git_ops::GitRepo;
use super::helpers::get_current_vault_dir;

pub fn backup(
    message: Option<String>,
    _force: bool,
    _set_upstream: bool,
) -> Result<()> {
    // Get vault directory
    let vault_dir = get_current_vault_dir()?;

    // Check if vault is initialized
    if !Vault::is_initialized(&vault_dir) {
        bail!("Vault not initialized. Run 'gfv init' first.");
    }

    // Load vault
    let vault = Vault::load(&vault_dir)
        .context("Failed to load vault")?;

    if vault.manifest.files.is_empty() {
        println!("No files to backup. Add files with 'gfv add <file>'.");
        return Ok(());
    }

    println!("{} Backing up changes...", "==>".green().bold());

    // Step 1: Copy all source files to vault
    let mut files_copied = 0;
    for (vault_relative_path, entry) in &vault.manifest.files {
        let source_path = std::path::PathBuf::from(&entry.source_path);
        let vault_file_path = vault.get_file_path(vault_relative_path);

        // Skip if source doesn't exist
        if !source_path.exists() {
            println!("  {} Skipping {} (source not found)",
                "⚠".yellow(),
                vault_relative_path
            );
            continue;
        }

        // Copy file or directory
        if source_path.is_dir() {
            // Remove existing directory in vault and copy fresh
            if vault_file_path.exists() {
                fs::remove_dir_all(&vault_file_path)?;
            }
            copy_dir_recursive(&source_path, &vault_file_path)?;
        } else {
            // Ensure parent directory exists
            if let Some(parent) = vault_file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&source_path, &vault_file_path)?;
        }

        files_copied += 1;
    }

    println!("  {} Copied {} files/directories", "✓".green(), files_copied);

    // Step 2: Check if there are any Git changes
    let git_repo = GitRepo::open(&vault.repo_path)
        .context("Failed to open git repository")?;

    if !git_repo.has_changes()? {
        println!("\n{} Everything up to date", "✓".green().bold());
        return Ok(());
    }

    // Step 3: Generate commit message
    let commit_msg = if let Some(msg) = message {
        msg
    } else {
        // TODO: Try AI generation if configured
        // For now, auto-generate based on changed files
        "Update vault".to_string()
    };

    // Step 4: Commit changes
    git_repo.add_all()
        .context("Failed to stage changes")?;

    git_repo.commit(&commit_msg)
        .context("Failed to commit changes")?;

    println!("  {} Committed locally", "✓".green());

    // Step 5: Sync with remote (if configured)
    if let Some(ref remote_config) = vault.manifest.remote {
        println!("  {} Syncing with remote...", "==>".green());

        // Pull first
        match git_repo.pull("origin", &remote_config.branch) {
            Ok(_) => {
                println!("    {} Pulled from origin/{}", "✓".green(), remote_config.branch);
            }
            Err(e) => {
                eprintln!("{} Failed to pull from remote: {}", "✗".red().bold(), e);
                eprintln!("\nYour changes are committed locally but not pushed.");
                eprintln!("Resolve conflicts manually in: {}", vault.repo_path.display());
                return Err(e);
            }
        }

        // Push
        match git_repo.push("origin", &remote_config.branch) {
            Ok(_) => {
                println!("    {} Pushed to origin/{}", "✓".green(), remote_config.branch);
                println!("\n{} Your configs are backed up to remote!", "✓".green().bold());
            }
            Err(e) => {
                eprintln!("{} Failed to push to remote: {}", "✗".red().bold(), e);
                eprintln!("\nYour changes are committed locally but not pushed.");
                return Err(e);
            }
        }
    } else {
        println!("\n{} Your configs are backed up locally!", "✓".green().bold());
        println!("(No remote configured - local-only mode)");
    }

    Ok(())
}

/// Recursively copy directory
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
