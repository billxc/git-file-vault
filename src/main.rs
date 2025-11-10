mod cli;
mod vault;
mod config;
mod git_ops;
mod commands;
#[cfg(feature = "ai")]
mod ai;
mod error;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    // Load config to check for aliases
    let config = Config::load().unwrap_or_else(|_| Config {
        vaults: std::collections::HashMap::new(),
        current: config::CurrentConfig {
            active: "default".to_string(),
        },
        ai: Default::default(),
        sync: Default::default(),
        aliases: std::collections::HashMap::new(),
    });

    // Resolve aliases in command line arguments
    let args = resolve_aliases(&config);

    let cli = Cli::parse_from(args);
    cli.execute()
}

/// Resolve aliases in command line arguments
fn resolve_aliases(config: &Config) -> Vec<String> {
    let mut args: Vec<String> = std::env::args().collect();

    // Only process if we have at least 2 arguments (program name + command)
    if args.len() >= 2 {
        let command = &args[1];

        // Check if this is an alias
        if let Some(expansion) = config.aliases.get(command) {
            // Replace the command with its expansion
            // Split expansion in case it contains multiple words
            let expansion_parts: Vec<String> = expansion.split_whitespace()
                .map(|s| s.to_string())
                .collect();

            // Remove the original command and insert expansion
            args.remove(1);
            for (i, part) in expansion_parts.into_iter().enumerate() {
                args.insert(1 + i, part);
            }
        }
    }

    args
}
