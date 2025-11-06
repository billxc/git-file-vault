use clap::{Parser, Subcommand};
use anyhow::Result;

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

    /// Add a file to vault
    Add {
        /// Source file path
        source: String,

        /// Custom name in vault
        #[arg(short, long)]
        name: Option<String>,

        /// Platform restriction (macos, linux, windows)
        #[arg(short, long)]
        platform: Option<String>,
    },

    /// Remove a file from vault
    Remove {
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
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Init { path, remote, branch, name, no_sync } => {
                todo!("Implement init command")
            }
            Commands::Add { source, name, platform } => {
                todo!("Implement add command")
            }
            Commands::Remove { file, delete_files } => {
                todo!("Implement remove command")
            }
            Commands::List { long } => {
                todo!("Implement list command")
            }
            Commands::Status => {
                todo!("Implement status command")
            }
            Commands::Backup { message, force, set_upstream } => {
                todo!("Implement backup command")
            }
            Commands::Restore { rebase, dry_run, force } => {
                todo!("Implement restore command")
            }
            Commands::Config { key, value, list, unset } => {
                todo!("Implement config command")
            }
        }
    }
}
