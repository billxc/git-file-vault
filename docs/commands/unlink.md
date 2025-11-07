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

- `--keep-source` - Keep source file (default)
- `--delete-source` - Also delete the actual source file
- `--force` - No confirmation prompt
- `--vault <name>` - Specify which vault to use (default: active vault)

## Examples

### Remove from vault only
```bash
gfv unlink zsh/zshrc
```
Stops managing the file but keeps `~/.zshrc` intact.

### Remove and delete source
```bash
gfv unlink vscode/settings.json --delete-source
```
⚠️ Also deletes the actual source file.

### Unlink from a specific vault
```bash
gfv unlink zsh/zshrc --vault work
gfv unlink old-config --vault personal --force
```

### Force removal (no prompt)
```bash
gfv unlink old-config --force
```

## Behavior

1. **Check if file exists** in manifest
   - Error if not found

2. **Confirm with user** (unless `--force`)
   - Show vault path and source path
   - Ask if also delete source file (if `--delete-source`)

3. **Remove from manifest**
   - Delete entry from `.vault-manifest.json`

4. **Remove from vault**
   - Delete file/directory from vault
   - Internally: `git rm`

5. **Optionally delete source**
   - If `--delete-source`: Delete actual file

6. **Commit changes** (internal)
   - `git commit` with message: `"Remove <vault-path>"`

## Output

### Standard removal
```
Removing: zsh/zshrc
  Source: /Users/username/.zshrc

This will stop managing this file.
The source file will NOT be deleted.

Continue? [Y/n] y

✓ Removed from vault
✓ Updated manifest
✓ Committed changes

File is no longer managed by gfv.
Your source file at /Users/username/.zshrc is still there.
```

### With source deletion
```
Removing: old-config
  Source: /Users/username/.old-config

⚠️  WARNING: --delete-source specified
This will PERMANENTLY DELETE:
  - File in vault
  - Source file: /Users/username/.old-config

Are you absolutely sure? [y/N] y

✓ Removed from vault
✓ Deleted source file
✓ Updated manifest
✓ Committed changes

File removed from vault and source deleted.
```

### File not found
```
Error: File not managed
'unknown-file' is not in the vault.

List managed files with:
  gfv list
```

### With --force
```
Removing: zsh/zshrc (forced)
✓ Removed from vault
✓ Updated manifest
✓ Committed changes
```

## Exit Codes

- `0` - Success
- `1` - File not found in manifest
- `2` - User cancelled
- `3` - Source file deletion failed
- `4` - Vault not initialized

## Notes

- **Source file is safe by default** - Only vault copy is removed
- Use `--delete-source` with caution - it's permanent
- Changes are committed automatically
- You can re-add the file later if needed

## Comparison with Other Operations

| Command | Effect on Vault | Effect on Source |
|---------|-----------------|------------------|
| `gfv unlink` | Deletes | Keeps (default) |
| `gfv unlink --delete-source` | Deletes | Deletes ⚠️ |
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
# Source file kept, vault cleaned up
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
