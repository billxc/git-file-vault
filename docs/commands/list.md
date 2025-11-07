# `gfv list` - List Managed Files

List all files currently managed by gfv.

## Synopsis

```bash
gfv list [options]
```

## Description

Displays all files that are currently managed by the vault, along with their metadata.

## Options

- `--long` / `-l` - Show detailed information
- `--platform <os>` - Filter by platform (macos, linux, windows)
- `--modified` - Show only modified files
- `--vault <name>` - Specify which vault to use (default: active vault)

## Examples

### Basic list
```bash
gfv list
```

### Detailed listing
```bash
gfv list --long
```

### Filter by platform
```bash
gfv list --platform macos
```

### Show only modified
```bash
gfv list --modified
```

### List files in a specific vault
```bash
gfv list --vault work
gfv list --vault personal --long
```

## Output

### Basic Format
```
zsh/zshrc
gitconfig
vscode/settings.json
nvim/
ssh/config (macos)
```

### Long Format
```
VAULT PATH              SOURCE PATH                      STATUS     LAST SYNC
zsh/zshrc               ~/.zshrc                        synced     2025-11-06 14:30
gitconfig               ~/.gitconfig                    modified   2025-11-05 10:20
vscode/settings.json    ~/Library/.../settings.json     synced     2025-11-04 09:15
nvim/                   ~/.config/nvim/                 synced     2025-11-06 12:00
ssh/config              ~/.ssh/config                   synced     2025-11-03 08:45
  Platform: macos

5 files managed (1 modified)
```

### With platform filter
```bash
gfv list --platform macos

ssh/config              ~/.ssh/config                   synced

1 file (macos only)
```

### Modified only
```bash
gfv list --modified

gitconfig               ~/.gitconfig                    modified

1 file needs sync
```

### Empty vault
```
No files managed yet.

Add files with:
  gfv link <file>
```

## Information Displayed

### Basic Mode
- Vault path
- Platform tag (if any)

### Long Mode (`--long`)
- Vault path
- Source path (actual file location)
- Sync status (synced, modified, conflict)
- Last sync timestamp
- Platform (if restricted)
- File type (file or directory)

## Exit Codes

- `0` - Success
- `1` - Vault not initialized

## Notes

- **Read-only operation** - Safe to run anytime
- Sorted by vault path alphabetically
- Platform-restricted files show platform tag
- Use with `--modified` to see what needs syncing

## Common Uses

### See what's managed
```bash
gfv list
```

### Check for modifications
```bash
gfv list --modified
```

### See details of all files
```bash
gfv list --long
```

### List platform-specific files
```bash
gfv list --platform linux
```

## Machine-Readable Output

For scripting, combine with other tools:

```bash
# Get all vault paths
gfv list

# Get source paths (long format + awk)
gfv list --long | tail -n +2 | awk '{print $2}'

# Count managed files
gfv list | wc -l
```

## See Also

- [link.md](./link.md) - Add files to vault
- [unlink.md](./unlink.md) - Remove files from vault
- [status.md](./status.md) - Check sync status
- [OVERVIEW.md](../OVERVIEW.md) - Manifest format
