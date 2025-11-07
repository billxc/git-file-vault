// Restore command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;

use crate::vault::Vault;
use crate::git_ops::GitRepo;
use super::helpers::{get_vault_dir, get_active_vault_name};

pub fn restore(
    _rebase: bool,
    dry_run: bool,
    force: bool,
    vault: Option<String>,
) -> Result<()> {
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

    if vault.manifest.files.is_empty() {
        println!("No files to restore. Vault is empty.");
        return Ok(());
    }

    println!("{} Restoring from vault...", "==>".green().bold());

    // Step 1: Pull from remote if configured
    if let Some(ref remote_config) = vault.manifest.remote {
        println!("  {} Pulling from remote...", "==>".green());

        let git_repo = GitRepo::open(&vault.repo_path)
            .context("Failed to open git repository")?;

        // Get the actual current branch (it might differ from manifest)
        let current_branch = git_repo.current_branch()
            .unwrap_or_else(|_| remote_config.branch.clone());

        match git_repo.pull("origin", &current_branch) {
            Ok(_) => {
                println!("    {} Pulled from origin/{}", "✓".green(), current_branch);
            }
            Err(e) => {
                eprintln!("{} Failed to pull from remote: {}", "✗".red().bold(), e);
                eprintln!("\nResolve conflicts manually in: {}", vault.repo_path.display());
                return Err(e);
            }
        }
    }

    // Step 2: Check for uncommitted source changes (simplified for MVP)
    if !force && !dry_run {
        let mut has_local_changes = false;
        let mut changed_files = Vec::new();

        for (vault_relative_path, entry) in &vault.manifest.files {
            let source_path = std::path::PathBuf::from(&entry.source_path);
            let vault_file_path = vault.get_file_path(vault_relative_path);

            if source_path.exists() && vault_file_path.exists() {
                // Simple check: compare file sizes (for MVP)
                if let (Ok(source_meta), Ok(vault_meta)) = (
                    fs::metadata(&source_path),
                    fs::metadata(&vault_file_path)
                ) {
                    if source_meta.len() != vault_meta.len() {
                        has_local_changes = true;
                        changed_files.push(source_path.display().to_string());
                    }
                }
            }
        }

        if has_local_changes {
            println!("\n{} You have local changes that will be overwritten:",
                "Warning:".yellow().bold()
            );
            for file in &changed_files {
                println!("  {} (modified)", file);
            }
            println!("\nContinue? [y/N] ");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Cancelled.");
                return Ok(());
            }
        }
    }

    // Step 3: Copy files from vault to source locations
    let mut files_restored = 0;
    let mut files_skipped = 0;

    for (vault_relative_path, entry) in &vault.manifest.files {
        let source_path = std::path::PathBuf::from(&entry.source_path);
        let vault_file_path = vault.get_file_path(vault_relative_path);

        // Skip if vault file doesn't exist
        if !vault_file_path.exists() {
            println!("  {} Skipping {} (not in vault)",
                "⚠".yellow(),
                vault_relative_path
            );
            files_skipped += 1;
            continue;
        }

        // Check platform restriction
        if let Some(ref platform) = entry.platform {
            let current_os = std::env::consts::OS;
            let platform_match = match platform.as_str() {
                "macos" => current_os == "macos",
                "linux" => current_os == "linux",
                "windows" => current_os == "windows",
                _ => true,
            };

            if !platform_match {
                println!("  {} Skipping {} (platform: {})",
                    "⚠".yellow(),
                    vault_relative_path,
                    platform
                );
                files_skipped += 1;
                continue;
            }
        }

        if dry_run {
            println!("  Would restore: {} -> {}",
                vault_relative_path,
                source_path.display()
            );
            files_restored += 1;
            continue;
        }

        // Ensure parent directory exists
        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Copy file or directory
        if vault_file_path.is_dir() {
            // Remove existing directory and copy fresh
            if source_path.exists() {
                fs::remove_dir_all(&source_path)?;
            }
            copy_dir_recursive(&vault_file_path, &source_path)?;
        } else {
            fs::copy(&vault_file_path, &source_path)?;
        }

        println!("  {} Restored: {}",
            "✓".green(),
            source_path.display()
        );
        files_restored += 1;
    }

    // Step 4: Report results
    println!();
    if dry_run {
        println!("{} Would restore {} files (skipped {})",
            "✓".green().bold(),
            files_restored,
            files_skipped
        );
        println!("Run without --dry-run to apply changes.");
    } else {
        println!("{} Restored {} files (skipped {})",
            "✓".green().bold(),
            files_restored,
            files_skipped
        );
        println!("Your configs are up to date!");
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
