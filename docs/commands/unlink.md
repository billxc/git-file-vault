# `gfv unlink` - Unlink File from Vault

Stop managing a file or directory.

## Synopsis

```bash
gfv unlink <vault-path> [options]
```

## Description

Unlinks a file from vault management. The file is removed from the manifest, optionally deleted from the vault, and changes are committed to Git.

**Important:** This does NOT delete your actual source file - only breaks the management link.

## Arguments

- `<vault-path>` - File path in vault (required)

## Options

- `--delete-files` - Also delete from vault
- `--vault <name>` - Specify which vault to use (default: active vault)

## Examples

### Remove from vault only
```bash
gfv unlink zsh/zshrc
```
Removes file from manifest and vault. Keeps `~/.zshrc` intact.

### Remove and delete from vault
```bash
gfv unlink vscode/settings.json --delete-files
```
Removes from manifest and deletes the vault copy.

### Unlink from a specific vault
```bash
gfv unlink zsh/zshrc --vault work
gfv unlink old-config --vault personal
```

## Behavior

1. **Check if file exists** in manifest
   - Error if not found

2. **Remove from manifest**
   - Delete entry from `.vault-manifest.json`

3. **Optionally delete from vault**
   - If `--delete-files`: Delete file/directory from vault

4. **Commit changes** (internal)
   - `git commit` with message: `"Unlink <vault-path>"`

**Note:** Source file is never deleted. Only vault copy and manifest entry are affected.

## Output

### Standard removal (without --delete-files)
```
==> Unlinking zsh/zshrc from vault...
  ✓ Removed from manifest
  → Kept files in vault (use --delete-files to remove)
  ✓ Committed changes

File is no longer managed by gfv
Source file location unchanged: /Users/username/.zshrc
```

### With --delete-files
```
==> Unlinking old-config from vault...
  ✓ Removed from manifest
  ✓ Deleted from vault
  ✓ Committed changes

File is no longer managed by gfv
Source file location unchanged: /Users/username/.old-config
```

### File not found
```
Error: File 'unknown-file' is not managed by gfv.

List managed files with: gfv list
```

## Exit Codes

- `0` - Success
- `1` - File not found in manifest
- `4` - Vault not initialized

## Notes

- **Source file is always safe** - Never deleted, only vault copy is affected
- Use `--delete-files` to also remove from vault (optional)
- Changes are committed automatically
- You can re-link the file later if needed

## Comparison with Other Operations

| Command | Effect on Vault | Effect on Source |
|---------|-----------------|------------------|
| `gfv unlink` | Removes from manifest | Keeps |
| `gfv unlink --delete-files` | Removes from manifest and vault | Keeps |
| Manual delete of source | No change | Deleted |

## Recovery

If you accidentally remove a file:

### Restore from vault (if source still exists)
```bash
# If you kept the source file:
gfv link ~/.zshrc
```

### Restore from Git history
```bash
cd ~/.gfv
git log --all --oneline | grep "Remove"
git checkout <commit-hash>~1 -- <vault-path>
# Manually restore manifest entry
```

## Common Uses

### Stop managing a file
```bash
gfv unlink old-config
# Removed from manifest, source file kept
```

### Clean up vault completely
```bash
gfv unlink old-config --delete-files
# Removed from manifest and vault
```

### Clean up after moving file
```bash
# You moved ~/.zshrc to ~/.zsh/zshrc
gfv unlink zsh/zshrc
gfv link ~/.zsh/zshrc
```

### Remove platform-specific file
```bash
# Removing a macOS-only file
gfv unlink ssh/config-macos
```

## See Also

- [link.md](./link.md) - Add files to vault
- [list.md](./list.md) - List managed files
- [OVERVIEW.md](../OVERVIEW.md) - Architecture
