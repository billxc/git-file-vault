# `gfv restore` - Restore from Vault

Restore files from vault to local source locations. If remote is configured, pulls latest changes first.

## Synopsis

```bash
gfv restore [options]
```

## Description

Restores files from the vault to your source locations.

- **With remote**: Pulls from remote first, then copies vault → source
- **Without remote**: Just copies vault → source (useful after manual Git operations)

**Important**: This command will overwrite source files with vault versions. If you have uncommitted local changes, you'll be warned before proceeding.

## Options

- `--rebase` - Use rebase instead of merge when pulling
- `--dry-run` - Show what would be updated without doing it
- `--force` / `-f` - Skip warning and overwrite local changes

## Examples

### Standard restore
```bash
gfv restore
```
Pulls from remote and applies changes to source files.

### Restore with rebase
```bash
gfv restore --rebase
```
Uses `git pull --rebase` internally for cleaner history.

### Preview changes
```bash
gfv restore --dry-run
```
Shows what files would be updated without changing anything.

### Force restore (skip warnings)
```bash
gfv restore --force
```
Overwrites local changes without prompting.

## Behavior

1. **Check for uncommitted source changes**
   - Compare source files with vault versions
   - If changes detected and not `--force`:
     ```
     Warning: You have local changes that will be overwritten:
       ~/.zshrc (modified)
       ~/.gitconfig (modified)
     Continue? [y/N]
     ```
   - If user cancels → Exit
   - If `--force` or user confirms → Continue

2. **Pull from Git remote** (internal)
   - Run `git pull` (or `git pull --rebase`)
   - If Git conflict → Stop and report error
   - If merge successful → Continue

3. **Apply changes to source files**
   - Copy updated files from vault → source locations
   - Respect platform-specific file tags
   - Skip files that don't exist in manifest

4. **Report results**
   - List files that were updated
   - Show any errors or skipped files

## Output

### Success
```
Restoring from origin/main...
✓ Pulled 2 commits

Applying changes to source files...
  ✓ ~/.zshrc          updated
  ✓ ~/.gitconfig      updated
  - ~/.vimrc          no changes

Restored 2 files.
```

### With local changes (warning)
```
Warning: You have local changes that will be overwritten:
  ~/.zshrc (modified 2 hours ago)
  ~/.gitconfig (modified 1 hour ago)

These changes have NOT been backed up.
Continue? [y/N] y

Restoring from origin/main...
✓ Pulled 1 commit

Applying changes...
  ✓ ~/.zshrc          overwritten
  ✓ ~/.gitconfig      overwritten

Restored 2 files (local changes lost).
```

### Already up to date
```
Already up to date
No changes to restore.
```

### Git conflict
```
Error: Git conflict detected during pull

Please resolve manually:
  cd ~/.gfv
  git status
  # Resolve conflicts
  git add .
  git commit

Then run 'gfv restore' again.
```

### Dry run
```
Would restore from remote:
  ~/.zshrc          (2 lines changed)
  ~/.gitconfig      (5 lines changed)

No changes made (dry run).
```

## Exit Codes


## Notes

- **Works with or without remote** - Can restore from local vault too
- **Warns before overwriting** local changes (unless `--force`)
- Automatically handles simple Git merges (when remote configured)
- If Git conflict occurs, manual resolution needed
- Use `--dry-run` to preview changes first
- Platform-specific files are respected

## Common Workflows

### Get latest changes (with remote)
```bash
gfv restore
```

### Restore from local vault (no remote)
```bash
# After manual Git operations in vault
gfv restore
# Just copies vault → source
```

### After making local changes accidentally
```bash
# Oops, edited files without backing up
gfv restore --force
# Reverts to vault version
```

### Check what would change
```bash
gfv restore --dry-run
# See what files would be updated
```

### After Git conflict (remote only)
```bash
gfv restore
# → Error: Git conflict

# Resolve manually
cd ~/.gfv
git status
vim conflicted-file
git add .
git commit

# Try again
gfv restore
```

## See Also

- [backup.md](./backup.md) - Backup to remote
- [status.md](./status.md) - Check what changed
- [init.md](./init.md) - Initialize or clone vault
