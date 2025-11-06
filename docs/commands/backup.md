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
   - If no changes → "Everything up to date"
   - If changes → Proceed to commit

3. **Generate commit message**
   - If `-m` provided → Use it
   - Else if AI configured → Try AI generation
   - Else → Auto-generate (e.g., "Update zsh/zshrc")

4. **Commit changes**
   - `git add` and `git commit` internally

5. **Sync with remote** (only if remote configured)
   - `git pull` to merge remote changes
   - If Git conflict → Stop and report error
   - If successful → `git push`

## Output Examples

### Local-only
```
Committing changes...
  Message: "Update zsh/zshrc"

✓ Committed to vault
Your configs are backed up locally!
```

### With remote
```
Backing up to remote...
✓ Committed locally
✓ Pulled from origin/main
✓ Pushed to origin/main

Your configs are backed up!
```

### With AI (if configured)
```
Analyzing changes...
Suggested: "feat: add git aliases and improve zsh prompt"

[A]ccept / [E]dit / [R]egenerate / [C]ancel? a

✓ Committed and pushed
```

### Git conflict (remote only)
```
Error: Git conflict detected during pull

Please resolve manually:
  cd ~/.gfv
  git status
  # Resolve conflicts
  git add .
  git commit

Then run 'gfv backup' again.
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
