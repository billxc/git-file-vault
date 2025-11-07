use clap::{Parser, Subcommand};
use anyhow::Result;
use crate::commands;

#[derive(Parser)]
#[command(name = "gfv")]
#[command(version, about = "Git-based file version management tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new vault
    Init {
        /// Vault path (default: ~/.gfv)
        path: Option<String>,

        /// Remote repository URL
        #[arg(short, long)]
        remote: Option<String>,

        /// Branch name
        #[arg(short, long, default_value = "main")]
        branch: String,

        /// Vault name for multi-vault support
        #[arg(short, long, default_value = "default")]
        name: String,

        /// Don't sync files after cloning from remote
        #[arg(long)]
        no_sync: bool,
    },

    /// Link a file to vault
    Link {
        /// Source file path
        source: String,

        /// Custom name in vault
        #[arg(short, long)]
        name: Option<String>,

        /// Platform restriction (macos, linux, windows)
        #[arg(short, long)]
        platform: Option<String>,
    },

    /// Unlink a file from vault
    Unlink {
        /// Vault file path
        file: String,

        /// Also delete from vault
        #[arg(long)]
        delete_files: bool,
    },

    /// List managed files
    List {
        /// Show detailed information
        #[arg(short, long)]
        long: bool,
    },

    /// Show vault status
    Status,

    /// Backup changes to vault (and remote if configured)
    Backup {
        /// Commit message
        #[arg(short, long)]
        message: Option<String>,

        /// Force push
        #[arg(short, long)]
        force: bool,

        /// Set upstream branch
        #[arg(short = 'u', long)]
        set_upstream: bool,
    },

    /// Restore files from vault (pull from remote if configured)
    Restore {
        /// Use rebase instead of merge
        #[arg(long)]
        rebase: bool,

        /// Show what would be updated without doing it
        #[arg(long)]
        dry_run: bool,

        /// Skip warning and overwrite local changes
        #[arg(short, long)]
        force: bool,
    },

    /// Manage configuration
    Config {
        /// Configuration key (e.g., ai.api_key)
        key: Option<String>,

        /// Value to set
        value: Option<String>,

        /// List all configuration
        #[arg(short, long)]
        list: bool,

        /// Unset a configuration key
        #[arg(long)]
        unset: Option<String>,
    },

    /// Manage vaults
    Vault {
        #[command(subcommand)]
        command: VaultCommands,
    },

    /// Debug commands (development)
    Debug {
        #[command(subcommand)]
        command: DebugCommands,
    },
}

#[derive(Subcommand)]
enum DebugCommands {
    /// Show gfv paths and status
    Paths,

    /// Clean all gfv data
    Clean {
        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum VaultCommands {
    /// List all vaults
    List,

    /// Create a new vault
    Create {
        /// Vault name
        name: String,

        /// Vault path (default: ~/.gfv/<name>)
        path: Option<String>,

        /// Remote repository URL
        #[arg(short, long)]
        remote: Option<String>,
    },

    /// Switch to a vault
    Switch {
        /// Vault name
        name: String,
    },

    /// Remove a vault
    Remove {
        /// Vault name
        name: String,

        /// Also delete vault directory
        #[arg(long)]
        delete_files: bool,
    },

    /// Show vault information
    Info {
        /// Vault name (default: active vault)
        name: Option<String>,
    },

    /// Manage remote
    Remote {
        #[command(subcommand)]
        command: RemoteCommands,
    },
}

#[derive(Subcommand)]
enum RemoteCommands {
    /// Set remote URL
    Set {
        /// Remote URL
        url: String,

        /// Vault name (default: active vault)
        name: Option<String>,
    },

    /// Get remote URL
    Get {
        /// Vault name (default: active vault)
        name: Option<String>,
    },

    /// Remove remote
    Remove {
        /// Vault name (default: active vault)
        name: Option<String>,
    },
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Init { path, remote, branch, name, no_sync } => {
                commands::init(path, remote, branch, name, no_sync)
            }
            Commands::Link { source, name, platform } => {
                commands::link(source, name, platform)
            }
            Commands::Unlink { file, delete_files } => {
                commands::unlink(file, delete_files)
            }
            Commands::List { long } => {
                commands::list(long)
            }
            Commands::Status => {
                commands::status()
            }
            Commands::Backup { message, force, set_upstream } => {
                commands::backup(message, force, set_upstream)
            }
            Commands::Restore { rebase, dry_run, force } => {
                commands::restore(rebase, dry_run, force)
            }
            Commands::Config { key, value, list, unset } => {
                commands::config(key, value, list, unset)
            }
            Commands::Vault { command } => {
                match command {
                    VaultCommands::List => commands::vault_list(),
                    VaultCommands::Create { name, path, remote } => {
                        commands::vault_create(name, path, remote)
                    }
                    VaultCommands::Switch { name } => commands::vault_switch(name),
                    VaultCommands::Remove { name, delete_files } => {
                        commands::vault_remove(name, delete_files)
                    }
                    VaultCommands::Info { name } => commands::vault_info(name),
                    VaultCommands::Remote { command } => {
                        match command {
                            RemoteCommands::Set { url, name } => {
                                commands::vault_remote_set(url, name)
                            }
                            RemoteCommands::Get { name } => {
                                commands::vault_remote_get(name)
                            }
                            RemoteCommands::Remove { name } => {
                                commands::vault_remote_remove(name)
                            }
                        }
                    }
                }
            }
            Commands::Debug { command } => {
                match command {
                    DebugCommands::Paths => commands::debug_show_paths(),
                    DebugCommands::Clean { force } => commands::debug_clean(force),
                }
            }
        }
    }
}
