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
| `remote set <url> [name]` | Set remote URL |
| `remote get [name]` | Show remote URL |
| `info [name]` | Show vault info |

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
gfv vault create work
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
gfv vault remote set git@github.com:user/configs.git

# Set remote for specific vault
gfv vault remote set git@github.com:company/configs.git work

# Show remote
gfv vault remote get
gfv vault remote get work

# Remove remote
gfv vault remote remove
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
