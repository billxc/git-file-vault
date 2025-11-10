// Backup command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::fs;

use crate::vault::Vault;
use crate::git_ops::GitRepo;
use crate::config::Config;
#[cfg(feature = "ai")]
use crate::ai::AiClient;
use super::helpers::{get_vault_dir, get_active_vault_name};

pub async fn backup(
    message: Option<String>,
    _force: bool,
    _set_upstream: bool,
    vault: Option<String>,
) -> Result<()> {
    // Get vault directory
    let vault_name = vault.unwrap_or_else(get_active_vault_name);
    let vault_dir = get_vault_dir(&vault_name)?;

    // Check if vault is initialized
    if !Vault::is_initialized(&vault_dir) {
        bail!("Vault not initialized. Run 'gfv init' first.");
    }

    // Load vault
    let vault = Vault::load(&vault_dir)
        .context("Failed to load vault")?;

    if vault.manifest.files.is_empty() {
        println!("No files to backup. Add files with 'gfv link <file>'.");
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

    let has_changes = git_repo.has_changes()?;

    // Step 3: Commit changes if there are any
    if has_changes {
        // Generate commit message
        let commit_msg = if let Some(msg) = message {
            msg
        } else {
            // Try AI generation if configured
            generate_commit_message_auto(&git_repo).await?
        };

        git_repo.add_all()
            .context("Failed to stage changes")?;

        git_repo.commit(&commit_msg)
            .context("Failed to commit changes")?;

        println!("  {} Committed locally: \"{}\"", "✓".green(), commit_msg);
    }

    // Step 4: Sync with remote (if configured)
    if let Some(ref remote_config) = vault.manifest.remote {
        println!("  {} Syncing with remote...", "==>".green());

        // Get the actual current branch (it might differ from manifest if repo was created with different default)
        let current_branch = git_repo.current_branch()
            .unwrap_or_else(|_| remote_config.branch.clone());

        // Try to fetch first to get remote refs (ignore errors if remote is empty/new)
        let _ = git_repo.fetch("origin", &current_branch);

        // Only pull if remote branch exists (skip on first push)
        if git_repo.remote_branch_exists("origin", &current_branch) {
            match git_repo.pull("origin", &current_branch, true) {
                Ok(_) => {
                    println!("    {} Pulled from origin/{}", "✓".green(), current_branch);
                }
                Err(e) => {
                    eprintln!("{} Failed to pull from remote: {}", "✗".red().bold(), e);
                    eprintln!("\nYour changes are committed locally but not pushed.");
                    eprintln!("Resolve conflicts manually in: {}", vault.repo_path.display());
                    return Err(e);
                }
            }
        } else {
            println!("    {} First push to remote (skipping pull)", "→".blue());
        }

        // Push
        match git_repo.push("origin", &current_branch) {
            Ok(_) => {
                println!("    {} Pushed to origin/{}", "✓".green(), current_branch);
                println!("\n{} Your files are backed up to remote!", "✓".green().bold());
            }
            Err(e) => {
                eprintln!("{} Failed to push to remote: {}", "✗".red().bold(), e);
                eprintln!("\nYour changes are committed locally but not pushed.");
                return Err(e);
            }
        }
    } else {
        // No remote configured
        if has_changes {
            println!("\n{} Your files are backed up locally!", "✓".green().bold());
            println!("(No remote configured - local-only mode)");
        } else {
            println!("\n{} Everything up to date", "✓".green().bold());
        }
    }

    Ok(())
}

/// Generate commit message automatically (using AI if configured, or fallback)
async fn generate_commit_message_auto(git_repo: &GitRepo) -> Result<String> {
    // Load global config
    let config = Config::load().unwrap_or_else(|_| Config {
        vaults: std::collections::HashMap::new(),
        current: crate::config::CurrentConfig {
            active: "default".to_string(),
        },
        ai: Default::default(),
        sync: Default::default(),
        aliases: std::collections::HashMap::new(),
    });

    // Check if AI is configured
    #[cfg(feature = "ai")]
    {
        if let (Some(endpoint), Some(api_key), Some(model)) = (
            &config.ai.endpoint,
            &config.ai.api_key,
            &config.ai.model,
        ) {
            println!("  {} Generating commit message with AI...", "→".blue());

            // Get the diff
            let diff = git_repo.get_diff()
                .context("Failed to get git diff")?;

            if !diff.trim().is_empty() {
                // Try to generate with AI
                let ai_client = AiClient::new(
                    endpoint.clone(),
                    api_key.clone(),
                    model.clone(),
                );

                match ai_client.generate_commit_message(&diff).await {
                    Ok(message) => {
                        return Ok(message);
                    }
                    Err(e) => {
                        eprintln!("  {} AI generation failed: {}", "⚠".yellow(), e);
                        eprintln!("  {} Falling back to default message", "→".yellow());
                    }
                }
            }
        }
    }

    // Fallback to default message
    Ok("Update vault".to_string())
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
