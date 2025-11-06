// Manifest module - manages .vault-manifest.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Utc};
use anyhow::{Context, Result};
use crate::error::VaultError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
    #[serde(rename = "vaultPath")]
    pub vault_path: String,
    pub files: HashMap<String, FileEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<RemoteConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    #[serde(rename = "sourcePath")]
    pub source_path: String,
    #[serde(rename = "type")]
    pub file_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "addedAt")]
    pub added_at: DateTime<Utc>,
    #[serde(rename = "lastSync", skip_serializing_if = "Option::is_none")]
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub url: String,
    pub branch: String,
}

impl Manifest {
    pub fn new(vault_path: String, remote: Option<RemoteConfig>) -> Self {
        Self {
            version: "1.0".to_string(),
            vault_path,
            files: HashMap::new(),
            remote,
        }
    }

    /// Load manifest from .vault-manifest.json in the vault directory
    pub fn load(vault_path: &Path) -> Result<Self> {
        let manifest_path = vault_path.join(".vault-manifest.json");

        if !manifest_path.exists() {
            return Err(VaultError::NotInitialized(vault_path.display().to_string()).into());
        }

        let content = std::fs::read_to_string(&manifest_path)
            .context("Failed to read manifest file")?;

        let manifest: Manifest = serde_json::from_str(&content)
            .context("Failed to parse manifest JSON")?;

        Ok(manifest)
    }

    /// Save manifest to .vault-manifest.json in the vault directory
    pub fn save(&self, vault_path: &Path) -> Result<()> {
        let manifest_path = vault_path.join(".vault-manifest.json");

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize manifest")?;

        std::fs::write(&manifest_path, content)
            .context("Failed to write manifest file")?;

        Ok(())
    }

    pub fn add_file(&mut self, vault_path: String, entry: FileEntry) {
        self.files.insert(vault_path, entry);
    }

    pub fn remove_file(&mut self, vault_path: &str) -> Option<FileEntry> {
        self.files.remove(vault_path)
    }

    pub fn get_file(&self, vault_path: &str) -> Option<&FileEntry> {
        self.files.get(vault_path)
    }
}
