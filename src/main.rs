mod cli;
mod vault;
mod config;
mod git_ops;
mod commands;
#[cfg(feature = "ai")]
mod ai;
mod error;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
