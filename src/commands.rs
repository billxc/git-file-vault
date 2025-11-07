// Commands module - implements all CLI commands

pub mod helpers;
pub mod init;
pub mod link;
pub mod list;
pub mod backup;
pub mod restore;
pub mod status;
pub mod unlink;
pub mod config;
pub mod vault;
pub mod debug;

// Re-export commonly used top-level commands
pub use init::init;
pub use link::link;
pub use list::list;
pub use backup::backup;
pub use restore::restore;
pub use status::status;
pub use unlink::unlink;
pub use config::config;

// Vault and debug subcommands use full module paths for clarity
// (e.g., commands::vault::create, commands::debug::show_paths)
