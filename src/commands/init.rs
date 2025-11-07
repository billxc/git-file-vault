// Init command implementation
// This is now just a wrapper around vault create with the name "default"

use anyhow::Result;

pub fn init(
    path: Option<String>,
    remote: Option<String>,
    branch: Option<String>,
    name: String,
    _no_sync: bool,
) -> Result<()> {
    // Simply delegate to vault create
    crate::commands::vault::create(name, path, remote, branch)
}
