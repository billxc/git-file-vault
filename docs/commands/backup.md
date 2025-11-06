# `gfv backup` - Backup Changes

Backup local changes to vault. Commits changes locally, and pushes to remote if configured.

## Synopsis

```bash
gfv backup [options]
```

## Description

Backs up local changes. This command automatically:
1. Copies all managed source files to vault (overwrites vault versions)
2. Commits changes with appropriate message
3. If remote is configured: Pulls from remote (handles Git merge if needed) and pushes

**Remote is optional** - Works perfectly fine without remote for local-only version control.

## Options

- `--message <msg>` / `-m <msg>` - Specify commit message (bypasses AI)
- `--force` / `-f` - Force push (dangerous, remote only!)
- `--set-upstream` / `-u` - Set upstream branch (remote only)

## Examples

### Basic backup
```bash
# Edit files normally
vim ~/.zshrc
vim ~/.gitconfig

# Backup
gfv backup
```
Automatically copies changed files, generates commit message, commits locally.
If remote configured: also pulls and pushes.

### Backup with custom message
```bash
gfv backup -m "Update zsh configuration"
```

### Force push (remote only)
```bash
gfv backup --force
```
⚠️ Use with caution - can overwrite remote history.

## Behavior

1. **Copy source files to vault**
   - For each file in manifest, copy source → vault
   - Overwrites vault versions (no conflict detection in MVP)

2. **Check for changes**
   - If changes exist → Commit them
   - If no changes → Skip commit (but still push if needed)

3. **Commit changes (if any)**
   - Stage all changes with `git add`
   - Create commit with message (auto-generated or user-provided)

4. **Sync with remote (if configured)**
   - Pull from remote (only if remote branch exists)
   - Push to remote (always, even if no new changes to commit)
   - This ensures unpushed commits from `gfv add` get pushed

## Output Examples

### Local-only (with changes)
```
==> Backing up changes...
  ✓ Copied 2 files/directories
  ✓ Committed locally

✓ Your configs are backed up locally!
(No remote configured - local-only mode)
```

### Local-only (no changes)
```
==> Backing up changes...
  ✓ Copied 2 files/directories

✓ Everything up to date
```

### With remote
```
==> Backing up changes...
  ✓ Copied 2 files/directories
  ✓ Committed locally
  ==> Syncing with remote...
    ✓ Pulled from origin/main
    ✓ Pushed to origin/main

✓ Your configs are backed up to remote!
```

### First push to new remote
```
==> Backing up changes...
  ✓ Copied 2 files/directories
  ✓ Committed locally
  ==> Syncing with remote...
    → First push to remote (skipping pull)
    ✓ Pushed to origin/main

✓ Your configs are backed up to remote!
```

### Git conflict (remote only)
```
✗ Failed to pull from remote: Git conflict detected

Your changes are committed locally but not pushed.
Resolve conflicts manually in: /Users/username/.gfv/default/repo
```

## AI Configuration

Configure AI for commit messages:
```bash
gfv config ai.api_key sk-xxxxx
gfv config ai.model gpt-3.5-turbo  # optional
```

Or use environment variable:
```bash
export OPENAI_API_KEY=sk-xxxxx
```

**Priority**: Environment variable > Config file > Auto-generate

**Fallback**: If AI fails, automatically falls back to simple auto-generated messages.

## Exit Codes

- `0` - Success
- `1` - No changes to backup
- `2` - Git conflict (remote only)
- `3` - User cancelled

## Notes

- Works with or without remote
- No need to manually run `gfv add` before backup
- Automatically handles simple Git merges (when remote configured)
- AI generation is completely optional
- Use `--force` only when certain (e.g., after rebase)

## See Also

- [add.md](./add.md) - Add new files to vault
- [restore.md](./restore.md) - Restore from vault
- [status.md](./status.md) - Check what changed
- [config.md](./config.md) - Configure AI
