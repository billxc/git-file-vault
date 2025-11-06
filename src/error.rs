use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Vault not initialized at {0}")]
    NotInitialized(String),

    #[error("Vault already exists at {0}")]
    AlreadyExists(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("File already managed: {0}")]
    AlreadyManaged(String),

    #[error("File not in manifest: {0}")]
    NotInManifest(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("Remote not configured")]
    NoRemote,

    #[error("Git conflict detected")]
    GitConflict,

    #[error("Invalid manifest format")]
    InvalidManifest,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, VaultError>;
