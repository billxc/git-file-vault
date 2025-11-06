# `gfv push` - Push Changes to Remote

Upload local changes to remote repository. Automatically syncs, commits, and pushes.

## Synopsis

```bash
gfv push [options]
```

## Description

Uploads local changes to the remote repository. This command automatically:
1. Syncs source files to vault (`sync --to-vault`)
2. Commits changes with appropriate message
3. Pushes to Git remote

**Requires remote configuration** - Run `gfv vault remote set <url>` first if not configured during init.

**No separate commit needed** - `push` handles everything.

## Options

- `--message <msg>` / `-m <msg>` - Specify commit message (bypasses AI)
- `--no-sync` - Skip auto-sync (advanced users only)
- `--force` / `-f` - Force push (dangerous!)
- `--set-upstream` / `-u` - Set upstream branch

## Examples

### Basic push (with AI if configured)
```bash
gfv push
```
Auto-syncs, generates commit message, and pushes.

### Push with custom message
```bash
gfv push -m "Update zsh configuration"
```
Uses user-provided message instead of AI/auto-generation.

### Push without syncing (advanced)
```bash
gfv push --no-sync
```
Only commits and pushes what's already in vault.

### Force push
```bash
gfv push --force
```
⚠️ Use with caution - can overwrite remote history.

## Commit Message Generation

**Three-tier strategy with smart fallback:**

### 1. User-specified (highest priority)
```bash
gfv push -m "Add git aliases to zsh"
```
Uses provided message directly.

### 2. AI-generated (if configured)
```bash
# ~/.config/gfv/config.toml contains:
# [ai]
# api_key = "sk-xxxxx"

gfv push
# → Analyzing changes...
# → Suggested: "feat: add git aliases and improve zsh prompt"
# → [A]ccept / [E]dit / [R]egenerate / [C]ancel?
```

User can:
- **A** - Accept and push
- **E** - Edit message inline
- **R** - Regenerate with AI
- **C** - Cancel push

### 3. Auto-generated (fallback)
No AI configured or AI fails:
- Single file: `"Update zsh/zshrc"`
- Multiple files: `"Update zsh/zshrc, gitconfig, and 2 other files"`
- Mixed: `"Update 3 files, add 1 file"`

## Behavior

1. **Check vault status**
   - Ensure vault is initialized
   - Check if remote is configured

2. **Auto-sync to vault** (unless `--no-sync`)
   - Run `gfv sync --to-vault`
   - Handle any conflicts interactively

3. **Check for changes**
   - If no changes → "Everything up to date"
   - If changes → Proceed to commit

4. **Generate commit message**
   - If `-m` provided → Use it
   - Else if AI configured → Try AI generation
   - Else → Auto-generate simple message

5. **Commit changes** (internal)
   - `git add` all changed files
   - `git commit` with generated message

6. **Push to remote** (internal)
   - `git push` (or `git push --force` if `--force`)

## Output

### With AI (successful)
```
Syncing to vault...
✓ Synced 2 files

Analyzing changes...
Suggested commit message:
  "feat: add git aliases and improve zsh prompt"

[A]ccept / [E]dit / [R]egenerate / [C]ancel? a

Pushing to remote...
✓ Pushed 1 commit to origin/main

Your configs are now backed up!
```

### With AI (edit)
```
Analyzing changes...
Suggested commit message:
  "feat: add git aliases and improve zsh prompt"

[A]ccept / [E]dit / [R]egenerate / [C]ancel? e

Edit message: feat: add useful git aliases to zsh config

Pushing to remote...
✓ Pushed 1 commit to origin/main
```

### Auto-generated (no AI)
```
Syncing to vault...
✓ Synced 1 file

Committing changes...
  Message: "Update zsh/zshrc"

Pushing to remote...
✓ Pushed 1 commit to origin/main
```

### Nothing to push
```
Everything up to date
No changes to push.
```

### No remote configured
```
Error: No remote repository configured
Configure a remote first:
  cd ~/.gfv && git remote add origin <url>

Or re-initialize with:
  gfv init --remote <url>
```

## AI Configuration

### Setup
```bash
# Option 1: Config file
gfv config ai.api_key sk-xxxxx
gfv config ai.model gpt-3.5-turbo  # optional

# Option 2: Environment variable
export OPENAI_API_KEY=sk-xxxxx
```

### Priority
1. Environment variable `OPENAI_API_KEY`
2. Config file `~/.config/gfv/config.toml`
3. Not configured → Auto-generate

### Error Handling
- API call fails → Fallback to auto-generation
- Invalid API key → Warn once, fallback
- Network timeout → Fallback
- Rate limit → Warn user, fallback

## Exit Codes

- `0` - Success
- `1` - No changes to push
- `2` - Sync conflict unresolved
- `3` - No remote configured
- `4` - Push rejected by remote
- `5` - User cancelled

## Notes

- **Always safe to run** - Will not push if there are conflicts
- AI generation is **completely optional** - works without configuration
- User can **always override** AI with `-m`
- `--no-sync` is for advanced users who know vault is up to date
- Use `--force` only when you're certain (e.g., after rebase)

## Common Workflows

### Daily push
```bash
# Edit files normally
vim ~/.zshrc
vim ~/.gitconfig

# Push everything at once
gfv push
```

### Quick push with message
```bash
vim ~/.vimrc
gfv push -m "Update vim colorscheme"
```

### Push after manual sync
```bash
gfv sync --to-vault
# Review changes...
gfv push --no-sync
```

## See Also

- [sync.md](./sync.md) - Manual sync operation
- [pull.md](./pull.md) - Download changes from remote
- [status.md](./status.md) - Check what would be pushed
- [OVERVIEW.md](../OVERVIEW.md#ai-commit-message-generation) - AI details
