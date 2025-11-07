// Vault module - manages the vault operations

pub mod manifest;

use anyhow::Result;
use std::path::{Path, PathBuf};
use manifest::Manifest;

pub struct Vault {
    pub vault_dir: PathBuf,    // ~/.gfv/default/
    pub repo_path: PathBuf,     // ~/.gfv/default/repo/
    pub manifest: Manifest,
}

impl Vault {
    /// Load an existing vault from a vault directory
    pub fn load(vault_dir: &Path) -> Result<Self> {
        let repo_path = vault_dir.join("repo");
        let manifest = Manifest::load(vault_dir)?;

        Ok(Self {
            vault_dir: vault_dir.to_path_buf(),
            repo_path,
            manifest,
        })
    }

    /// Check if a vault directory is initialized
    pub fn is_initialized(vault_dir: &Path) -> bool {
        let repo_path = vault_dir.join("repo");
        repo_path.join(".git").exists()
    }

    /// Save the manifest to disk
    pub fn save_manifest(&self) -> Result<()> {
        self.manifest.save(&self.vault_dir)
    }

    /// Get the path helper for resolving vault paths
    pub fn get_file_path(&self, vault_relative_path: &str) -> PathBuf {
        self.repo_path.join(vault_relative_path)
    }
}
