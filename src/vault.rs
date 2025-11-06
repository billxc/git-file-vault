// Vault module - manages the vault operations

pub mod manifest;

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use manifest::{Manifest, RemoteConfig};
use crate::error::VaultError;

pub struct Vault {
    pub path: PathBuf,
    pub manifest: Manifest,
}

impl Vault {
    /// Load an existing vault from a path
    pub fn load(path: &Path) -> Result<Self> {
        let manifest = Manifest::load(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            manifest,
        })
    }

    /// Check if a path contains an initialized vault
    pub fn is_initialized(path: &Path) -> bool {
        path.join(".vault-manifest.json").exists() && path.join(".git").exists()
    }

    /// Save the manifest to disk
    pub fn save_manifest(&self) -> Result<()> {
        self.manifest.save(&self.path)
    }
}
