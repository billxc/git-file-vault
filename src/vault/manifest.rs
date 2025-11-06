// Manifest module - manages .vault-manifest.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

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
