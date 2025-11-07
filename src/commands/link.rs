// Link command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::{Path, PathBuf};
use chrono::Utc;

use crate::vault::{Vault, manifest::FileEntry};
use super::helpers::{get_vault_dir, get_active_vault_name};

pub fn link(
    source: String,
    name: Option<String>,
    platform: Option<String>,
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

    // Get vault file path
    let vault_file_path = vault.get_file_path(&vault_relative_path);

    // Check existence in both locations
    let exists_locally = source_path.exists();
    let exists_in_vault = vault_file_path.exists();

    // Must exist in at least one place
    if !exists_locally && !exists_in_vault {
        bail!(
            "File not found in either location:\n  Local: {}\n  Vault: {}",
            source_path.display(),
            vault_file_path.display()
        );
    }

    // Determine file type from whichever exists
    let file_type = if exists_locally {
        if source_path.is_dir() { "directory" } else { "file" }
    } else {
        if vault_file_path.is_dir() { "directory" } else { "file" }
    };

    // Check for sensitive files (only if exists locally)
    if exists_locally && is_sensitive_file(&source_path) {
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

    println!("{} Linking {} {}",
        "==>".green().bold(),
        source_path.display(),
        if file_type == "directory" { "(directory)" } else { "" }
    );
    println!("  Vault path: {}", vault_relative_path);
    println!("  Platform: {}", platform.as_deref().unwrap_or("all"));

    if exists_locally && !exists_in_vault {
        println!("{} File exists locally but not in vault", "→".blue());
        println!("   Use 'gfv backup' to upload it");
    } else if !exists_locally && exists_in_vault {
        println!("{} File exists in vault but not locally", "→".blue());
        println!("   Use 'gfv restore' to download it");
    } else if exists_locally && exists_in_vault {
        println!("{} File exists in both locations", "→".blue());
    }

    // Create manifest entry
    let entry = FileEntry {
        source_path: source_path.display().to_string(),
        file_type: file_type.to_string(),
        platform,
        added_at: Utc::now(),
        last_sync: None,  // No sync yet, just linking
    };

    // Update manifest
    vault.manifest.add_file(vault_relative_path.clone(), entry);
    vault.save_manifest()
        .context("Failed to save manifest")?;

    println!("{} Updated manifest", "✓".green().bold());

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
