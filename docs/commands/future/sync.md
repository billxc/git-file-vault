# `gfv sync` - Synchronize Files

Bidirectionally synchronize files between vault and source locations.

## Synopsis

```bash
gfv sync [files...] [options]
```

## Description

Synchronizes files between the vault and their actual locations. This is a smart bidirectional sync that compares timestamps and handles conflicts.

**Behavior depends on remote configuration:**
- **Without remote**: Only syncs files between vault and source locations
- **With remote**: Also commits changes and pulls from remote to stay synchronized

Ensures your vault and source files stay in sync.

## Arguments

- `[files...]` - Specific files to sync (default: all managed files)

## Options

- `--to-vault` / `-t` - Only sync source → vault (one-way)
- `--from-vault` / `-f` - Only sync vault → source (one-way)
- `--force-vault` - Use vault version on conflict (no prompt)
- `--force-source` - Use source version on conflict (no prompt)
- `--ignore-platform` - Ignore platform restrictions
- `--dry-run` - Show what would be done without doing it

## Examples

### Smart bidirectional sync (default)
```bash
gfv sync
```
Syncs everything with remote and local files.

### Sync specific files
```bash
gfv sync zsh/zshrc gitconfig
```

### One-way sync to vault
```bash
gfv sync --to-vault
```
Only copies source → vault.

### One-way sync from vault
```bash
gfv sync --from-vault
```
Only copies vault → source.

### Preview changes
```bash
gfv sync --dry-run
```

### Auto-resolve conflicts
```bash
gfv sync --force-vault    # Always use vault version
gfv sync --force-source   # Always use source version
```

## Smart Sync Logic

gfv compares modification times and determines the sync direction:

| Vault Status | Source Status | Action |
|--------------|---------------|--------|
| Modified | Unchanged | Copy vault → source |
| Unchanged | Modified | Copy source → vault |
| Modified | Modified | **Conflict** - Prompt user |
| Unchanged | Unchanged | Skip |

## Conflict Resolution

When both vault and source are modified:

```
Conflict detected: zsh/zshrc
  Vault:  modified 2025-11-06 14:30:00
  Source: modified 2025-11-06 14:35:00

Options:
  [V] Keep vault version
  [S] Keep source version
  [D] Show diff and decide
  [C] Cancel (skip this file)

Your choice:
```

## Output

### No changes
```
Everything in sync.
```

### Changes synced
```
Syncing changes:
  ✓ zsh/zshrc        source → vault
  ✓ gitconfig        vault → source

Synced 2 files.
```

### With conflicts
```
  C zsh/zshrc        conflict
  ✓ gitconfig        source → vault

[Interactive conflict resolution...]

Synced 2 files (1 conflict resolved).
```

### Dry run
```
Would sync:
  → zsh/zshrc        source → vault
  ← gitconfig        vault → source

No changes made (dry run).
```

## Exit Codes

- `0` - Success
- `1` - Vault not initialized
- `2` - Conflict unresolved (user cancelled)
- `3` - File not found
- `4` - Remote error

## Notes

- **Safe to run anytime** - Will not overwrite without confirmation
- Use `--dry-run` to preview before syncing
- Conflicts always require user input (unless `--force-*`)
- Platform restrictions respected by default

## Common Workflows

### Daily sync
```bash
gfv sync
```

### Check for changes
```bash
gfv sync --dry-run
```

### Bulk conflict resolution
```bash
# Always prefer local changes
gfv sync --force-source

# Always prefer vault
gfv sync --force-vault
```

## See Also

- [backup.md](./backup.md) - Push to remote
- [restore.md](./restore.md) - Pull from remote
- [status.md](./status.md) - Check sync status
- [OVERVIEW.md](../OVERVIEW.md#conflict-resolution) - Conflict details
