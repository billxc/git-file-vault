// Init command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::PathBuf;
use std::fs;

use crate::vault::{Vault, manifest::{Manifest, RemoteConfig}};
use crate::git_ops::GitRepo;

pub fn init(
    _path: Option<String>,
    remote: Option<String>,
    branch: String,
    name: String,
    _no_sync: bool,
) -> Result<()> {
    // Determine vault directory: ~/.gfv/{name}/
    let home = dirs::home_dir()
        .context("Failed to get home directory")?;
    let vault_dir = home.join(".gfv").join(&name);
    let repo_path = vault_dir.join("repo");

    // Check if vault already exists
    if Vault::is_initialized(&vault_dir) {
        bail!("Vault '{}' already initialized at {}", name, vault_dir.display());
    }

    // Create vault directory structure
    fs::create_dir_all(&vault_dir)
        .context("Failed to create vault directory")?;
    fs::create_dir_all(&repo_path)
        .context("Failed to create repo directory")?;

    println!("{} Initializing vault '{}'...", "==>".green().bold(), name);
    println!("  Vault dir: {}", vault_dir.display());
    println!("  Repo dir: {}", repo_path.display());

    // Handle three scenarios based on documentation
    if let Some(remote_url) = remote {
        // Check if remote is empty or has content by attempting to clone
        println!("{} Checking remote repository...", "==>".green().bold());

        // Try to clone into repo directory
        match GitRepo::clone(&remote_url, &repo_path) {
            Ok(_git_repo) => {
                // Remote has content - use cloned repository
                println!("{} Cloned existing vault from remote", "✓".green().bold());

                // Load or create manifest
                let manifest = Manifest::load(&vault_dir)?;

                println!("{} Vault '{}' initialized successfully!", "✓".green().bold(), name);
                println!("  Remote: {}", remote_url);
                println!("  Files: {}", manifest.files.len());

                // Add vault to global config
                add_vault_to_config(&name, &vault_dir)?;
            }
            Err(_) => {
                // Remote is empty or doesn't exist - create new vault and push
                println!("{} Remote is empty, creating new vault...", "==>".green().bold());

                // Remove the failed clone directory and recreate
                fs::remove_dir_all(&repo_path).ok();
                fs::create_dir_all(&repo_path)?;

                // Initialize Git repository
                let git_repo = GitRepo::init(&repo_path)
                    .context("Failed to initialize Git repository")?;

                // Create manifest with remote config
                let remote_config = RemoteConfig {
                    url: remote_url.clone(),
                    branch: branch.clone(),
                };
                let manifest = Manifest::new(Some(remote_config));

                // Save manifest
                manifest.save(&vault_dir)
                    .context("Failed to save manifest")?;

                // Create a .gitignore in repo to avoid accidentally committing local files
                let gitignore_path = repo_path.join(".gitignore");
                fs::write(&gitignore_path, "# Git-file-vault managed repository\n")?;

                // Add and commit
                git_repo.add_all()
                    .context("Failed to add files")?;
                git_repo.commit("Initialize vault")
                    .context("Failed to commit")?;

                // Set the branch name to match what user requested
                git_repo.set_branch(&branch)
                    .context("Failed to set branch name")?;

                // Add remote and push
                git_repo.add_remote("origin", &remote_url)
                    .context("Failed to add remote")?;
                git_repo.push("origin", &branch)
                    .context("Failed to push to remote")?;

                println!("{} Vault '{}' initialized and pushed to remote!", "✓".green().bold(), name);
                println!("  Remote: {}", remote_url);

                // Add vault to global config
                add_vault_to_config(&name, &vault_dir)?;
            }
        }
    } else {
        // No remote - local-only vault
        println!("{} Creating local-only vault...", "==>".green().bold());

        // Initialize Git repository in repo directory
        let git_repo = GitRepo::init(&repo_path)
            .context("Failed to initialize Git repository")?;

        // Create manifest
        let manifest = Manifest::new(None);

        // Save manifest
        manifest.save(&vault_dir)
            .context("Failed to save manifest")?;

        // Create a .gitignore in repo
        let gitignore_path = repo_path.join(".gitignore");
        fs::write(&gitignore_path, "# Git-file-vault managed repository\n")?;

        // Add and commit
        git_repo.add_all()
            .context("Failed to add .gitignore")?;
        git_repo.commit("Initialize vault")
            .context("Failed to commit")?;

        println!("{} Vault '{}' initialized successfully!", "✓".green().bold(), name);
        println!("  Mode: Local-only (no remote)");

        // Add vault to global config
        add_vault_to_config(&name, &vault_dir)?;
    }

    Ok(())
}

fn add_vault_to_config(name: &str, vault_dir: &PathBuf) -> Result<()> {
    let mut config = crate::config::Config::load()?;

    // Add vault
    config.vaults.insert(name.to_string(), vault_dir.display().to_string());

    // If this is the first vault or no active vault, make it active
    if config.vaults.len() == 1 || !config.vaults.contains_key(&config.current.active) {
        config.current.active = name.to_string();
    }

    // Save config
    config.save()?;

    Ok(())
}
