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

pub fn create(name: String, path: Option<String>, remote: Option<String>, branch: Option<String>) -> Result<()> {
    let mut config = load_config()?;

    // Check if vault already exists
    if config.vaults.contains_key(&name) {
        bail!("Vault '{}' already exists at: {}", name, config.vaults[&name]);
    }

    // Determine vault path
    let vault_dir = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        dirs::home_dir()
            .context("Failed to get home directory")?
            .join(".gfv")
            .join(&name)
    };

    let repo_path = vault_dir.join("repo");

    // Check if vault already initialized at this path
    if Vault::is_initialized(&vault_dir) {
        bail!("Vault already initialized at {}", vault_dir.display());
    }

    // Create vault directory structure
    std::fs::create_dir_all(&vault_dir)
        .context("Failed to create vault directory")?;
    std::fs::create_dir_all(&repo_path)
        .context("Failed to create repo directory")?;

    println!("{} Initializing vault '{}'...", "==>".green().bold(), name);
    println!("  Vault dir: {}", vault_dir.display());
    println!("  Repo dir: {}", repo_path.display());

    // Handle three scenarios based on documentation
    if let Some(remote_url) = remote {
        // Check if remote is empty or has content by attempting to clone
        println!("{} Checking remote repository...", "==>".green().bold());

        // Try to clone into repo directory
        match crate::git_ops::GitRepo::clone(&remote_url, &repo_path) {
            Ok(git_repo) => {
                // Remote has content - use cloned repository
                println!("{} Cloned existing vault from remote", "✓".green().bold());

                // Branch selection priority:
                // 1. User specified --branch
                // 2. Remote default branch (what we just cloned)
                let selected_branch = if let Some(user_branch) = branch {
                    // User specified a branch
                    git_repo.set_branch(&user_branch)
                        .context("Failed to set branch name")?;
                    user_branch
                } else {
                    // Use the remote default branch (current branch after clone)
                    git_repo.current_branch()
                        .context("Failed to get current branch")?
                };

                // Load or create manifest
                let mut manifest = crate::vault::manifest::Manifest::load(&vault_dir)?;

                // Ensure manifest has remote config
                if manifest.remote.is_none() {
                    manifest.remote = Some(crate::vault::manifest::RemoteConfig {
                        url: remote_url.clone(),
                        branch: selected_branch.clone(),
                    });
                    manifest.save(&vault_dir)
                        .context("Failed to save manifest")?;
                }

                println!("{} Vault '{}' initialized successfully!", "✓".green().bold(), name);
                println!("  Remote: {}", remote_url);
                println!("  Branch: {}", selected_branch);
                println!("  Files: {}", manifest.files.len());

                // Add to config
                config.vaults.insert(name.clone(), vault_dir.display().to_string());

                // If this is the first vault, make it active
                if config.vaults.len() == 1 {
                    config.current.active = name.clone();
                }

                save_config(&config)?;

                if config.current.active == name {
                    println!("\n{} This is now the active vault", "→".blue());
                }
            }
            Err(_) => {
                // Remote is empty or doesn't exist - create new vault and push
                println!("{} Remote is empty, creating new vault...", "==>".green().bold());

                // Remove the failed clone directory and recreate
                std::fs::remove_dir_all(&repo_path).ok();
                std::fs::create_dir_all(&repo_path)?;

                // Branch selection priority:
                // 1. User specified --branch
                // 2. Config default branch
                let selected_branch = branch.unwrap_or_else(|| config.sync.default_branch.clone());

                // Initialize Git repository
                let git_repo = crate::git_ops::GitRepo::init(&repo_path)
                    .context("Failed to initialize Git repository")?;

                // Create manifest with remote config
                let remote_config = crate::vault::manifest::RemoteConfig {
                    url: remote_url.clone(),
                    branch: selected_branch.clone(),
                };
                let manifest = crate::vault::manifest::Manifest::new(Some(remote_config));

                // Save manifest
                manifest.save(&vault_dir)
                    .context("Failed to save manifest")?;

                // Create a .gitignore in repo to avoid accidentally committing local files
                let gitignore_path = repo_path.join(".gitignore");
                std::fs::write(&gitignore_path, "# Git-file-vault managed repository\n")?;

                // Add and commit
                git_repo.add_all()
                    .context("Failed to add files")?;
                git_repo.commit("Initialize vault")
                    .context("Failed to commit")?;

                // Set the branch name
                git_repo.set_branch(&selected_branch)
                    .context("Failed to set branch name")?;

                // Add remote and push
                git_repo.add_remote("origin", &remote_url)
                    .context("Failed to add remote")?;
                git_repo.push("origin", &selected_branch)
                    .context("Failed to push to remote")?;

                println!("{} Vault '{}' initialized and pushed to remote!", "✓".green().bold(), name);
                println!("  Remote: {}", remote_url);
                println!("  Branch: {}", selected_branch);

                // Add to config
                config.vaults.insert(name.clone(), vault_dir.display().to_string());

                // If this is the first vault, make it active
                if config.vaults.len() == 1 {
                    config.current.active = name.clone();
                }

                save_config(&config)?;

                if config.current.active == name {
                    println!("\n{} This is now the active vault", "→".blue());
                }
            }
        }
    } else {
        // No remote - local-only vault
        println!("{} Creating local-only vault...", "==>".green().bold());

        // Branch selection priority:
        // 1. User specified --branch
        // 2. Config default branch
        let selected_branch = branch.unwrap_or_else(|| config.sync.default_branch.clone());

        // Initialize Git repository
        let git_repo = crate::git_ops::GitRepo::init(&repo_path)
            .context("Failed to initialize Git repository")?;

        // Create manifest
        let manifest = crate::vault::manifest::Manifest::new(None);

        // Save manifest
        manifest.save(&vault_dir)
            .context("Failed to save manifest")?;

        // Create a .gitignore in repo
        let gitignore_path = repo_path.join(".gitignore");
        std::fs::write(&gitignore_path, "# Git-file-vault managed repository\n")?;

        // Add and commit
        git_repo.add_all()
            .context("Failed to add .gitignore")?;
        git_repo.commit("Initialize vault")
            .context("Failed to commit")?;

        // Set branch
        git_repo.set_branch(&selected_branch)
            .context("Failed to set branch name")?;

        println!("{} Vault '{}' initialized successfully!", "✓".green().bold(), name);
        println!("  Mode: Local-only (no remote)");
        println!("  Branch: {}", selected_branch);

        // Add to config
        config.vaults.insert(name.clone(), vault_dir.display().to_string());

        // If this is the first vault, make it active
        if config.vaults.len() == 1 {
            config.current.active = name.clone();
        }

        save_config(&config)?;

        if config.current.active == name {
            println!("\n{} This is now the active vault", "→".blue());
        }
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
