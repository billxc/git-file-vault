// Add command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::fs;
use chrono::Utc;

use crate::vault::{Vault, manifest::FileEntry};
use crate::git_ops::GitRepo;
use super::helpers::get_current_vault_dir;

pub fn add(
    source: String,
    name: Option<String>,
    platform: Option<String>,
) -> Result<()> {
    // Get vault directory
    let vault_dir = get_current_vault_dir()?;

    // Check if vault is initialized
    if !Vault::is_initialized(&vault_dir) {
        bail!("Vault not initialized. Run 'gfv init' first.");
    }

    // Load vault
    let mut vault = Vault::load(&vault_dir)
        .context("Failed to load vault")?;

    // Validate and resolve source path
    let source_path = PathBuf::from(&source);
    let source_path = if source_path.is_absolute() {
        source_path
    } else {
        std::env::current_dir()?.join(&source_path)
    };

    // Expand ~ if present
    let source_path = expand_tilde(&source_path);

    if !source_path.exists() {
        bail!("Source file not found: {}", source_path.display());
    }

    // Determine if it's a file or directory
    let file_type = if source_path.is_dir() {
        "directory"
    } else {
        "file"
    };

    // Infer or use provided vault path
    let vault_relative_path = if let Some(n) = name {
        n
    } else {
        infer_vault_path(&source_path)?
    };

    // Check if already managed
    if vault.manifest.get_file(&vault_relative_path).is_some() {
        bail!(
            "File already managed\nThe file {} is already in the vault as {}\n\nTo update it, use:\n  gfv backup",
            source_path.display(),
            vault_relative_path
        );
    }

    // Check for sensitive files
    if is_sensitive_file(&source_path) {
        println!("{} Potentially sensitive file detected", "Warning:".yellow().bold());
        println!("  {}", source_path.display());
        println!("\nThis file may contain secrets or credentials.");
        println!("Are you sure you want to add it to version control? (y/N)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("{} Adding {} {}",
        "==>".green().bold(),
        source_path.display(),
        if file_type == "directory" { "(directory)" } else { "" }
    );
    println!("  Vault path: {}", vault_relative_path);
    println!("  Platform: {}", platform.as_deref().unwrap_or("all"));

    // Copy file/directory to vault repo
    let vault_file_path = vault.get_file_path(&vault_relative_path);

    // Create parent directories if needed
    if let Some(parent) = vault_file_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create vault directories")?;
    }

    if source_path.is_dir() {
        copy_dir_recursive(&source_path, &vault_file_path)
            .context("Failed to copy directory to vault")?;
    } else {
        fs::copy(&source_path, &vault_file_path)
            .context("Failed to copy file to vault")?;
    }

    println!("{} Copied to vault", "✓".green().bold());

    // Create manifest entry
    let entry = FileEntry {
        source_path: source_path.display().to_string(),
        file_type: file_type.to_string(),
        platform,
        added_at: Utc::now(),
        last_sync: Some(Utc::now()),
    };

    // Update manifest
    vault.manifest.add_file(vault_relative_path.clone(), entry);
    vault.save_manifest()
        .context("Failed to save manifest")?;

    println!("{} Updated manifest", "✓".green().bold());

    // Commit changes to repo
    let git_repo = GitRepo::open(&vault.repo_path)
        .context("Failed to open git repository")?;

    git_repo.add_all()
        .context("Failed to stage changes")?;

    let commit_message = format!("Add {}", vault_relative_path);
    git_repo.commit(&commit_message)
        .context("Failed to commit changes")?;

    println!("{} Committed changes", "✓".green().bold());

    // Push to remote if configured
    if let Some(ref remote_config) = vault.manifest.remote {
        let current_branch = git_repo.current_branch()
            .unwrap_or_else(|_| remote_config.branch.clone());

        match git_repo.push("origin", &current_branch) {
            Ok(_) => {
                println!("{} Pushed to origin/{}", "✓".green().bold(), current_branch);
            }
            Err(e) => {
                eprintln!("{} Failed to push to remote: {}", "⚠".yellow().bold(), e);
                eprintln!("Your changes are committed locally but not pushed.");
                eprintln!("Run 'gfv backup' to retry pushing.");
            }
        }
    }

    println!("\n{} is now managed by gfv.",
        if file_type == "directory" { "Directory" } else { "File" }
    );

    Ok(())
}

/// Expand ~ to home directory
fn expand_tilde(path: &Path) -> PathBuf {
    if let Ok(stripped) = path.strip_prefix("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    }
    path.to_path_buf()
}

/// Infer vault path from source path based on common patterns
fn infer_vault_path(source_path: &Path) -> Result<String> {
    let home = dirs::home_dir()
        .context("Failed to get home directory")?;

    let file_name = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid file name")?;

    // If path is under ~/.config/, strip that prefix
    if let Ok(relative) = source_path.strip_prefix(home.join(".config")) {
        let parts: Vec<_> = relative.components().collect();
        if !parts.is_empty() {
            return Ok(relative.display().to_string());
        }
    }

    // Handle dotfiles in home directory
    if source_path.parent() == Some(&home) && file_name.starts_with('.') {
        // Extract base name (e.g., .zshrc -> zsh)
        let stripped = file_name.strip_prefix('.').unwrap_or(file_name);

        // Known patterns
        if let Some(base) = stripped.strip_suffix("rc") {
            // .zshrc -> zsh/zshrc
            return Ok(format!("{}/{}", base, stripped));
        } else if stripped == "gitconfig" {
            return Ok("git/gitconfig".to_string());
        } else if stripped.starts_with("ssh/") || file_name == ".ssh" {
            return Ok(format!("ssh/{}", stripped));
        } else {
            // Generic dotfile: .myrc -> myrc/myrc
            return Ok(format!("{0}/{0}", stripped));
        }
    }

    // Handle VSCode settings
    if source_path.to_string_lossy().contains("Code/User/settings.json") {
        return Ok("vscode/settings.json".to_string());
    }

    // For other paths, strip home and use relative path
    if let Ok(relative) = source_path.strip_prefix(&home) {
        return Ok(relative.display().to_string());
    }

    // Fallback: use file name
    Ok(file_name.to_string())
}

/// Check if file path contains sensitive patterns
fn is_sensitive_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();

    path_str.contains(".env") ||
    path_str.contains("credential") ||
    path_str.contains("secret") ||
    path_str.ends_with(".key") ||
    path_str.ends_with(".pem") ||
    path_str.contains("password")
}

/// Recursively copy directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
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
