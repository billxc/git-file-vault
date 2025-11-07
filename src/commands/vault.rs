// Vault management commands

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::PathBuf;

use crate::config::Config;
use crate::vault::Vault;

pub fn list() -> Result<()> {
    let config = load_config()?;

    if config.vaults.is_empty() {
        println!("No vaults found.");
        println!("\nCreate a vault with:");
        println!("  gfv init");
        return Ok(());
    }

    for (name, path) in &config.vaults {
        let is_active = name == &config.current.active;
        let marker = if is_active { "*" } else { " " };
        let status = if is_active { "(active)".green() } else { "".normal() };

        println!("{} {:<15} {} {}",
            marker.green().bold(),
            name,
            path,
            status
        );
    }

    Ok(())
}

pub fn create(name: String, path: Option<String>, remote: Option<String>) -> Result<()> {
    let mut config = load_config()?;

    // Check if vault already exists
    if config.vaults.contains_key(&name) {
        bail!("Vault '{}' already exists at: {}", name, config.vaults[&name]);
    }

    // Determine vault path
    let vault_path = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        dirs::home_dir()
            .context("Failed to get home directory")?
            .join(".gfv")
            .join(&name)
    };

    let repo_path = vault_path.join("repo");

    // Check if vault already initialized at this path
    if Vault::is_initialized(&vault_path) {
        bail!("Vault already initialized at {}", vault_path.display());
    }

    // Create vault directory structure
    std::fs::create_dir_all(&vault_path)
        .context("Failed to create vault directory")?;
    std::fs::create_dir_all(&repo_path)
        .context("Failed to create repo directory")?;

    // Initialize Git repository
    let git_repo = crate::git_ops::GitRepo::init(&repo_path)
        .context("Failed to initialize Git repository")?;

    // Create manifest
    let remote_config = if let Some(url) = &remote {
        Some(crate::vault::manifest::RemoteConfig {
            url: url.clone(),
            branch: "main".to_string(),
        })
    } else {
        None
    };

    let manifest = crate::vault::manifest::Manifest::new(remote_config);
    manifest.save(&vault_path)
        .context("Failed to save manifest")?;

    // Create initial commit
    git_repo.add_all()
        .context("Failed to stage initial files")?;
    git_repo.commit("Initial commit")
        .context("Failed to create initial commit")?;

    // Set up remote if provided
    if let Some(url) = &remote {
        git_repo.add_remote("origin", url)
            .context("Failed to add remote")?;

        // Try to push
        match git_repo.push("origin", "main") {
            Ok(_) => {
                println!("{} Pushed to remote", "✓".green().bold());
            }
            Err(e) => {
                eprintln!("{} Failed to push to remote: {}", "⚠".yellow().bold(), e);
                eprintln!("You can push later with: gfv backup");
            }
        }
    }

    // Add to config
    config.vaults.insert(name.clone(), vault_path.display().to_string());

    // If this is the first vault, make it active
    if config.vaults.len() == 1 {
        config.current.active = name.clone();
    }

    save_config(&config)?;

    println!("{} Created vault '{}'", "✓".green().bold(), name);
    println!("  Path: {}", vault_path.display());
    if let Some(url) = remote {
        println!("  Remote: {}", url);
    }

    if config.current.active == name {
        println!("\n{} This is now the active vault", "→".blue());
    }

    Ok(())
}

pub fn switch(name: String) -> Result<()> {
    let mut config = load_config()?;

    // Check if vault exists
    if !config.vaults.contains_key(&name) {
        bail!("Vault '{}' not found.\n\nList available vaults with:\n  gfv vault list", name);
    }

    // Switch to vault
    config.current.active = name.clone();
    save_config(&config)?;

    println!("{} Switched to vault '{}'", "✓".green().bold(), name);
    println!("This vault is now active and set as default.");

    Ok(())
}

pub fn remove(name: String, delete_files: bool) -> Result<()> {
    let mut config = load_config()?;

    // Check if vault exists
    if !config.vaults.contains_key(&name) {
        bail!("Vault '{}' not found", name);
    }

    // Cannot remove active vault
    if config.current.active == name {
        bail!(
            "Cannot remove the currently active vault '{}'.\n\nSwitch to another vault first:\n  gfv vault switch <name>",
            name
        );
    }

    let vault_path = config.vaults[&name].clone();

    // Confirm with user
    println!("{} Removing vault '{}'", "!".yellow().bold(), name);
    println!("  Path: {}", vault_path);

    if delete_files {
        println!("\n{} --delete-files specified", "⚠".yellow().bold());
        println!("This will PERMANENTLY DELETE the vault directory:");
        println!("  {}", vault_path);
        println!("\nAre you absolutely sure? (y/N)");
    } else {
        println!("\nThis will remove the vault from config.");
        println!("The vault directory will NOT be deleted.");
        println!("\nContinue? (y/N)");
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Cancelled.");
        return Ok(());
    }

    // Remove from config
    config.vaults.remove(&name);
    save_config(&config)?;

    println!("{} Removed vault '{}' from config", "✓".green().bold(), name);

    // Delete files if requested
    if delete_files {
        std::fs::remove_dir_all(&vault_path)
            .context("Failed to delete vault directory")?;
        println!("{} Deleted vault directory", "✓".green().bold());
    } else {
        println!("\nVault directory preserved at: {}", vault_path);
    }

    Ok(())
}

pub fn info(name: Option<String>) -> Result<()> {
    let config = load_config()?;

    let vault_name = name.unwrap_or_else(|| config.current.active.clone());

    // Check if vault exists
    if !config.vaults.contains_key(&vault_name) {
        bail!("Vault '{}' not found", vault_name);
    }

    let vault_path = PathBuf::from(&config.vaults[&vault_name]);
    let is_active = vault_name == config.current.active;

    println!("Vault: {} {}",
        vault_name.cyan().bold(),
        if is_active { "(active)".green() } else { "".normal() }
    );
    println!("Path: {}", vault_path.display());

    // Load vault to get more info
    if Vault::is_initialized(&vault_path) {
        let vault = Vault::load(&vault_path)?;

        if let Some(remote) = &vault.manifest.remote {
            println!("Remote: {}", remote.url);
            println!("Branch: {}", remote.branch);
        } else {
            println!("Remote: (none)");
        }

        let file_count = vault.manifest.files.len();
        println!("Files: {} managed", file_count);
    } else {
        println!("{} Vault not initialized", "⚠".yellow().bold());
    }

    Ok(())
}

pub fn remote_set(url: String, name: Option<String>) -> Result<()> {
    let config = load_config()?;
    let vault_name = name.unwrap_or_else(|| config.current.active.clone());

    if !config.vaults.contains_key(&vault_name) {
        bail!("Vault '{}' not found", vault_name);
    }

    let vault_path = PathBuf::from(&config.vaults[&vault_name]);
    let mut vault = Vault::load(&vault_path)?;

    // Update remote in manifest
    vault.manifest.remote = Some(crate::vault::manifest::RemoteConfig {
        url: url.clone(),
        branch: "main".to_string(),
    });

    vault.save_manifest()?;

    // Also set git remote
    let git_repo = crate::git_ops::GitRepo::open(&vault.repo_path)?;
    git_repo.set_remote("origin", &url)?;

    println!("{} Set remote for vault '{}'", "✓".green().bold(), vault_name);
    println!("  URL: {}", url);

    Ok(())
}

pub fn remote_get(name: Option<String>) -> Result<()> {
    let config = load_config()?;
    let vault_name = name.unwrap_or_else(|| config.current.active.clone());

    if !config.vaults.contains_key(&vault_name) {
        bail!("Vault '{}' not found", vault_name);
    }

    let vault_path = PathBuf::from(&config.vaults[&vault_name]);
    let vault = Vault::load(&vault_path)?;

    if let Some(remote) = &vault.manifest.remote {
        println!("Remote URL: {}", remote.url);
        println!("Branch: {}", remote.branch);
    } else {
        println!("No remote configured for vault '{}'", vault_name);
    }

    Ok(())
}

pub fn remote_remove(name: Option<String>) -> Result<()> {
    let config = load_config()?;
    let vault_name = name.unwrap_or_else(|| config.current.active.clone());

    if !config.vaults.contains_key(&vault_name) {
        bail!("Vault '{}' not found", vault_name);
    }

    let vault_path = PathBuf::from(&config.vaults[&vault_name]);
    let mut vault = Vault::load(&vault_path)?;

    if vault.manifest.remote.is_none() {
        println!("No remote configured for vault '{}'", vault_name);
        return Ok(());
    }

    vault.manifest.remote = None;
    vault.save_manifest()?;

    println!("{} Removed remote for vault '{}'", "✓".green().bold(), vault_name);

    Ok(())
}

fn load_config() -> Result<Config> {
    Config::load()
}

fn save_config(config: &Config) -> Result<()> {
    config.save()
}
