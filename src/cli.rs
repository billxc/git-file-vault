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

        /// Branch name (defaults to remote default branch or config default)
        #[arg(short, long)]
        branch: Option<String>,

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

        /// Vault name to use
        #[arg(long)]
        vault: Option<String>,
    },

    /// Unlink a file from vault
    Unlink {
        /// Vault file path
        file: String,

        /// Also delete from vault
        #[arg(long)]
        delete_files: bool,

        /// Vault name to use
        #[arg(long)]
        vault: Option<String>,
    },

    /// List managed files
    List {
        /// Show detailed information
        #[arg(short, long)]
        long: bool,

        /// Vault name to use
        #[arg(long)]
        vault: Option<String>,
    },

    /// Show vault status
    Status {
        /// Vault name to use
        #[arg(long)]
        vault: Option<String>,
    },

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

        /// Vault name to use
        #[arg(long)]
        vault: Option<String>,
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

        /// Vault name to use
        #[arg(long)]
        vault: Option<String>,
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

        /// Branch name (defaults to remote default branch or config default)
        #[arg(short, long)]
        branch: Option<String>,
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

    /// Set remote URL
    SetRemote {
        /// Remote URL
        url: String,

        /// Branch name (optional, defaults to current branch or 'main')
        #[arg(short, long)]
        branch: Option<String>,

        /// Vault name (default: active vault)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Set remote branch
    SetBranch {
        /// Branch name
        branch: String,

        /// Vault name (default: active vault)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Remove remote
    RemoveRemote {
        /// Vault name (default: active vault)
        #[arg(short, long)]
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
            Commands::Link { source, name, platform, vault } => {
                commands::link(source, name, platform, vault)
            }
            Commands::Unlink { file, delete_files, vault } => {
                commands::unlink(file, delete_files, vault)
            }
            Commands::List { long, vault } => {
                commands::list(long, vault)
            }
            Commands::Status { vault } => {
                commands::status(vault)
            }
            Commands::Backup { message, force, set_upstream, vault } => {
                commands::backup(message, force, set_upstream, vault)
            }
            Commands::Restore { rebase, dry_run, force, vault } => {
                commands::restore(rebase, dry_run, force, vault)
            }
            Commands::Config { key, value, list, unset } => {
                commands::config(key, value, list, unset)
            }
            Commands::Vault { command } => {
                match command {
                    VaultCommands::List => commands::vault::list(),
                    VaultCommands::Create { name, path, remote, branch } => {
                        commands::vault::create(name, path, remote, branch)
                    }
                    VaultCommands::Switch { name } => commands::vault::switch(name),
                    VaultCommands::Remove { name, delete_files } => {
                        commands::vault::remove(name, delete_files)
                    }
                    VaultCommands::Info { name } => commands::vault::info(name),
                    VaultCommands::SetRemote { url, branch, name } => {
                        commands::vault::set_remote(url, branch, name)
                    }
                    VaultCommands::SetBranch { branch, name } => {
                        commands::vault::set_branch(branch, name)
                    }
                    VaultCommands::RemoveRemote { name } => {
                        commands::vault::remove_remote(name)
                    }
                }
            }
            Commands::Debug { command } => {
                match command {
                    DebugCommands::Paths => commands::debug::show_paths(),
                    DebugCommands::Clean { force } => commands::debug::clean(force),
                }
            }
        }
    }
}
