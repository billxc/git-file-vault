// Alias command implementation

use anyhow::{bail, Context, Result};
use colored::Colorize;
use crate::config::Config;

pub fn add(name: String, command: Vec<String>) -> Result<()> {
    // Validate alias name
    if name.is_empty() {
        bail!("Alias name cannot be empty");
    }

    // Check for reserved command names
    let reserved = ["init", "link", "unlink", "list", "status", "backup", "restore",
                    "config", "alias", "vault", "debug"];
    if reserved.contains(&name.as_str()) {
        bail!("Cannot create alias '{}': this is a reserved command name", name);
    }

    if command.is_empty() {
        bail!("Command cannot be empty");
    }

    // Load config
    let mut config = Config::load()
        .context("Failed to load config")?;

    // Join command parts into a single string
    let command_str = command.join(" ");

    // Check if alias already exists
    if let Some(existing) = config.aliases.get(&name) {
        println!("{} Alias '{}' already exists: {}",
            "Warning:".yellow().bold(),
            name,
            existing
        );
        println!("Do you want to overwrite it? (y/N)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Add alias
    config.aliases.insert(name.clone(), command_str.clone());

    // Save config
    config.save()
        .context("Failed to save config")?;

    println!("{} Alias '{}' → '{}' created successfully!",
        "✓".green().bold(),
        name.green(),
        command_str.cyan()
    );
    println!("\nYou can now use: gfv {}", name.green());

    Ok(())
}

pub fn remove(name: String) -> Result<()> {
    // Load config
    let mut config = Config::load()
        .context("Failed to load config")?;

    // Check if alias exists
    if !config.aliases.contains_key(&name) {
        bail!("Alias '{}' does not exist.\n\nList aliases with: gfv alias list", name);
    }

    // Remove alias
    let removed = config.aliases.remove(&name).unwrap();

    // Save config
    config.save()
        .context("Failed to save config")?;

    println!("{} Alias '{}' → '{}' removed",
        "✓".green().bold(),
        name.red(),
        removed.dimmed()
    );

    Ok(())
}

pub fn list() -> Result<()> {
    // Load config
    let config = Config::load()
        .context("Failed to load config")?;

    if config.aliases.is_empty() {
        println!("No aliases configured.");
        println!("\nCreate an alias with:");
        println!("  gfv alias add <name> <command>");
        println!("\nExample:");
        println!("  gfv alias add use vault switch");
        return Ok(());
    }

    println!("{}", "Command Aliases:".bold());
    println!();

    // Sort aliases by name for consistent output
    let mut aliases: Vec<_> = config.aliases.iter().collect();
    aliases.sort_by_key(|(name, _)| *name);

    for (name, command) in aliases {
        println!("  {} {} {}",
            name.green().bold(),
            "→".dimmed(),
            command.cyan()
        );
    }

    println!("\n{} aliases configured", config.aliases.len());

    Ok(())
}
