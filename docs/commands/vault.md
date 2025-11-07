# `gfv vault` - Manage Vaults

Manage multiple vaults.

## Synopsis

```bash
gfv vault <subcommand> [options]
```

## Subcommands

| Subcommand | Purpose |
|------------|---------|
| `list` | List all vaults |
| `create <name> [path]` | Create new vault |
| `remove <name>` | Remove vault |
| `switch <name>` | Switch to vault (becomes active and default) |
| `set-remote <url> [--branch <branch>]` | Set remote URL and branch |
| `set-branch <branch>` | Set remote branch |
| `remove-remote` | Remove remote |
| `info [name]` | Show vault info |

## Vault Create

The `create` subcommand supports the same smart remote handling and branch selection as `gfv init`.

**Branch selection priority:**
1. `--branch` flag: Explicitly specified branch name
2. Remote default branch: When cloning from remote (uses whatever branch the remote has as default)
3. Config default: From `~/.gfv/config.toml` (default: `main`)

**Smart remote handling:**
- If no `--remote` specified: Creates local-only vault
- If `--remote` specified and remote is empty: Creates new vault and sets remote
- If `--remote` specified and remote has content: Clones existing vault from remote

## Examples

### List vaults
```bash
gfv vault list
```
Output:
```
* default    /Users/username/.gfv              (active)
  work       /Users/username/.work-vault
```

### Create vault
```bash
# Local-only vault
gfv vault create personal

# With remote (will clone if remote has content)
gfv vault create work --remote git@github.com:company/configs.git

# With specific branch
gfv vault create work --remote git@github.com:company/configs.git --branch develop

# With custom path
gfv vault create work ~/work/vault --remote git@github.com:company/configs.git
```

### Switch vault
```bash
gfv vault switch work
```

Output:
```
Switched to vault 'work'
This vault is now active and set as default.
```

All subsequent `gfv` commands will use this vault.

### Remove vault
```bash
gfv vault remove old-vault
gfv vault remove old-vault --delete-files  # Also delete directory
```

**Note:** Cannot remove the currently active vault. Switch to another vault first.

### Manage remote

```bash
# Set remote for active vault
gfv vault set-remote https://github.com/user/configs.git

# Set remote with specific branch
gfv vault set-remote https://github.com/user/configs.git --branch develop

# Set remote for specific vault
gfv vault set-remote https://github.com/company/configs.git --name work

# Change branch only
gfv vault set-branch main

# Change branch for specific vault
gfv vault set-branch develop --name work

# Remove remote
gfv vault remove-remote

# Show remote info (use info command)
gfv vault info
```

### Show info
```bash
gfv vault info
gfv vault info work
```

Output:
```
Vault: work (active)
Path: /Users/username/.work-vault
Remote: git@github.com:company/configs.git
Files: 12 managed
```

## Configuration

Stored in `~/.config/gfv/config.toml`:
```toml
[vaults]
default = "/Users/username/.gfv"
work = "/Users/username/.work-vault"

[current]
active = "work"  # Current active vault
```

**Note:** There's only one "active" vault. When you `switch`, it becomes both the active vault and the default for future operations.

## Notes

- Most users only need one vault (default)
- Use multiple vaults to separate concerns (work/personal)
- `switch` makes a vault active for all subsequent commands
- Cannot remove the currently active vault

## See Also

- [init.md](./init.md) - Initialize vault
- [OVERVIEW.md](../OVERVIEW.md) - Multi-vault design
