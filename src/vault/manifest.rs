// Manifest module - manages manifest.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Utc};
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
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
    pub fn new(remote: Option<RemoteConfig>) -> Self {
        Self {
            version: "1.0".to_string(),
            files: HashMap::new(),
            remote,
        }
    }

    /// Load manifest from manifest.json in the vault directory
    pub fn load(vault_dir: &Path) -> Result<Self> {
        let manifest_path = vault_dir.join("manifest.json");

        if !manifest_path.exists() {
            // Return empty manifest if not exists
            return Ok(Self::new(None));
        }

        let content = std::fs::read_to_string(&manifest_path)
            .context("Failed to read manifest file")?;

        let manifest: Manifest = serde_json::from_str(&content)
            .context("Failed to parse manifest JSON")?;

        Ok(manifest)
    }

    /// Save manifest to manifest.json in the vault directory
    pub fn save(&self, vault_dir: &Path) -> Result<()> {
        let manifest_path = vault_dir.join("manifest.json");

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
