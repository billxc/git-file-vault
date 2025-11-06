# `gfv pull` - Pull from Remote

Download changes from remote repository and sync to source locations.

## Synopsis

```bash
gfv pull [options]
```

## Description

Pulls changes from the remote repository and automatically syncs them to your source file locations. This is the counterpart to `gfv push`.

## Options

- `--rebase` - Use rebase instead of merge
- `--no-sync` - Don't auto-sync after pull
- `--force-vault` - On conflict during sync, use vault version
- `--force-source` - On conflict during sync, use source version

## Examples

### Standard pull
```bash
gfv pull
```
Pulls from remote and syncs to source locations.

### Pull with rebase
```bash
gfv pull --rebase
```
Uses `git pull --rebase` internally for cleaner history.

### Pull without syncing (advanced)
```bash
gfv pull --no-sync
gfv sync --from-vault  # Sync manually later
```

## Behavior

1. **Pull from Git remote** (internal)
   - Run `git pull` (or `git pull --rebase`)
   - Handle merge conflicts if any

2. **Auto-sync from vault** (unless `--no-sync`)
   - Run `gfv sync --from-vault`
   - Update source files with pulled changes

3. **Handle sync conflicts**
   - If source files were modified locally → Conflict prompt
   - Use `--force-*` options to auto-resolve

## Output

### Success
```
Pulling from origin/main...
✓ Received 3 commits
  - Update zsh configuration
  - Add vim plugins
  - Fix gitconfig email

Syncing from vault to source...
  ✓ zsh/zshrc        vault → source
  ✓ vim/vimrc        vault → source
  ✓ gitconfig        vault → source

Your local files are up to date!
```

### No changes
```
Already up to date.
```

### With sync conflict
```
Pulling from origin/main...
✓ Received 1 commit

Syncing from vault to source...
  C zsh/zshrc        conflict (source modified locally)

[Interactive conflict resolution...]

Synced 1 file.
```

### Git merge conflict
```
Pulling from origin/main...
Error: Merge conflict in .vault-manifest.json

Please resolve Git conflicts first:
  cd ~/.gfv
  # Resolve conflicts
  git add .vault-manifest.json
  git commit

Then run:
  gfv pull --no-sync
```

## Exit Codes

- `0` - Success
- `1` - No remote configured
- `2` - Git merge conflict
- `3` - Sync conflict unresolved
- `4` - Network error

## Notes

- **Always syncs by default** - Your source files are updated automatically
- Use `--no-sync` if you want to review changes first
- Git merge conflicts must be resolved manually (rare)
- Safe to run multiple times

## Common Workflows

### Daily sync
```bash
gfv pull
```

### Review before syncing
```bash
gfv pull --no-sync
gfv status
gfv sync --from-vault
```

### Auto-resolve conflicts
```bash
gfv pull --force-vault    # Always use pulled version
gfv pull --force-source   # Always keep local version
```

## See Also

- [push.md](./push.md) - Push to remote
- [sync.md](./sync.md) - Manual sync
- [clone.md](./clone.md) - Initial clone
- [status.md](./status.md) - Check status
