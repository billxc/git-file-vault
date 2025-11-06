# `gfv status` - Show Status

Show current state of managed files and remote status.

## Synopsis

```bash
gfv status [options]
```

## Description

Displays the current sync status, Git repository status, and remote status. Helps you understand what needs to be synced or pushed.

## Options

- `--short` / `-s` - Compact output format

## Examples

### Full status
```bash
gfv status
```

### Short format
```bash
gfv status --short
```

## Output

### Full Format
```
Vault: /Users/username/.gfv (default)
Remote: git@github.com:user/configs.git
Branch: main

Sync Status:
  M  zsh/zshrc          source modified (2025-11-06 15:30:00)
  M  gitconfig          vault modified (2025-11-06 14:20:00)
  C  nvim/init.lua      conflict
  ✓  vscode/settings.json  synced
  -  ssh/config        skipped (platform: linux)

Remote Status:
  ↓ 2 commits behind
  ↑ 1 commit ahead (uncommitted changes in vault)

Summary:
  3 files need sync (1 conflict)
  Run 'gfv sync' to synchronize
  Run 'gfv restore' to get remote changes
  Run 'gfv backup' to upload local changes
```

### Short Format
```
M  zsh/zshrc
M  gitconfig
C  nvim/init.lua
✓  vscode/settings.json

↓2 ↑1
```

### Everything up to date
```
Vault: /Users/username/.gfv (default)
Remote: git@github.com:user/configs.git
Branch: main

All files synced.
Up to date with remote.

Nothing to do!
```

## Status Symbols

### Sync Status
- `✓` - Synced (vault and source match)
- `M` - Modified (needs sync)
- `C` - Conflict (both modified)
- `?` - Missing (source file not found)
- `-` - Skipped (platform mismatch)

### Remote Status
- `↑ N` - N commits ahead (need to push)
- `↓ N` - N commits behind (need to pull)
- `=` - Up to date

## What It Shows

### 1. Vault Information
- Vault path
- Current vault name (if multi-vault)
- Remote repository URL
- Current branch

### 2. Sync Status
For each managed file:
- Sync state (synced, modified, conflict)
- Which side is modified (vault or source)
- Last modification time
- Platform restrictions

### 3. Remote Status
- Commits ahead (need push)
- Commits behind (need pull)
- Uncommitted changes in vault

### 4. Actionable Summary
- Count of files needing attention
- Suggested next commands

## Exit Codes

- `0` - Success
- `1` - Vault not initialized

## Notes

- **Fast operation** - Only checks timestamps, no content comparison
- **Safe to run anytime** - Read-only operation
- Use before `sync`, `push`, or `pull` to know what will happen
- Platform-specific files show as skipped on other platforms

## Common Uses

### Before syncing
```bash
gfv status
gfv sync
```

### Check before push
```bash
gfv status
# Review what would be pushed
gfv backup
```

### After editing files
```bash
vim ~/.zshrc
gfv status
# Shows: M  zsh/zshrc  source modified
```

## See Also

- [sync.md](./sync.md) - Synchronize files
- [backup.md](./backup.md) - Push changes
- [restore.md](./restore.md) - Pull changes
- [list.md](./list.md) - List all managed files
