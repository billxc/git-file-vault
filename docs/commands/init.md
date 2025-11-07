# `gfv init` - Initialize Vault

Initialize a vault - either brand new or from an existing remote.

**Note:** `gfv init` is equivalent to `gfv vault create` with the name "default". For creating additional vaults, use `gfv vault create`.

## Synopsis

```bash
gfv init [path] [options]
```

## Description

Creates a vault at the specified path (or default location). This is the single entry point for setting up gfv on any device.

**Smart remote handling:**
- If no `--remote` specified: Creates local-only vault
- If `--remote` specified and remote is empty: Creates new vault and sets remote
- If `--remote` specified and remote has content: Clones existing vault from remote

**Branch selection priority:**
1. `--branch` flag: Explicitly specified branch name
2. Remote default branch: When cloning from remote (uses whatever branch the remote has as default)
3. Config default: From `~/.gfv/config.toml` (default: `main`)

**Remote repository is optional** - You can use gfv purely for local version control without any remote.

This command should be run once per device before using gfv.

## Arguments

- `[path]` - Vault path (default: `~/.gfv`)

## Options

- `--remote <url>` - Remote repository URL (auto-detects if empty or has content)
- `--branch <name>` - Branch name (optional; defaults to remote default or config default)
- `--name <name>` - Vault name for multi-vault support (default: `default`)
- `--no-sync` - Don't sync files after cloning from remote (only if remote has content)

## Examples

### Local-only vault
```bash
gfv init
```
Creates vault at `~/.gfv` without remote. Perfect for single-device local version control.

### Connect to empty remote (first device)
```bash
gfv init --remote git@github.com:user/configs.git
```
Creates new vault and sets remote. Use `gfv backup` to upload files later.

### Connect to existing remote (additional device)
```bash
gfv init --remote git@github.com:user/configs.git
```
Same command! Automatically detects remote has content, clones it, and syncs files.

### Initialize at custom path
```bash
gfv init ~/.config-vault
```

### Named vault
```bash
gfv init --name work --remote git@github.com:user/work-configs.git
```

## Behavior

### Scenario 1: Local-only (no `--remote`)

1. **Check if vault exists**
   - If path exists and is already a vault → Error
   - If path exists and is not a vault → Warn and ask for confirmation

2. **Create vault directory**
   - Create directory if it doesn't exist
   - Set appropriate permissions

3. **Initialize Git repository**
   - Run `git init` internally
   - Set initial branch name (default: `main`)
   - Create initial `.gitignore` with common patterns

4. **Create manifest file**
   - Create `.vault-manifest.json` with empty files list
   - Record vault path and version

5. **Update global config**
   - Add vault entry to `~/.config/gfv/config.toml`
   - Set as active vault if it's the only one

6. **Create initial commit**
   - Commit manifest file: "chore: initialize vault"

### Scenario 2: Empty remote (first device)

Same as Scenario 1, plus:

5. **Configure remote**
   - Check if remote is reachable
   - Check if remote is empty (no commits)
   - Add Git remote with specified URL
   - Set up tracking branch
   - Record remote in manifest

### Scenario 3: Existing remote (additional device)

1. **Check remote**
   - Verify remote URL is reachable
   - Detect that remote has commits

2. **Clone repository** (internal `git clone`)
   - Clone to specified path
   - Read `.vault-manifest.json`

3. **Show sync preview**
   - Display files that will be synced
   - Show source locations
   - Warn about existing files

4. **Sync files** (unless `--no-sync`)
   - Run `gfv sync --from-vault`
   - Backup existing files if conflicts
   - Handle platform-specific files

5. **Update global config**
   - Add vault entry
   - Set as active vault

## Output

### Local-only initialization
```
Initializing vault at /Users/username/.gfv
✓ Created directory
✓ Initialized Git repository
✓ Created manifest file
✓ Vault ready!

Next steps:
  gfv link <file>    # Add files to vault
```

### Empty remote (first device)
```
Initializing vault at /Users/username/.gfv
✓ Created directory
✓ Initialized Git repository
✓ Created manifest file
✓ Configured remote: git@github.com:user/configs.git
✓ Vault ready!

Next steps:
  gfv link <file>    # Add files to vault
  gfv backup          # Push to remote
```

### Existing remote (additional device)
```
Connecting to git@github.com:user/configs.git...
✓ Remote has existing vault, cloning...
✓ Cloned to /Users/username/.gfv

Found 5 managed files:
  zsh/zshrc              → ~/.zshrc
  gitconfig              → ~/.gitconfig
  vscode/settings.json   → ~/Library/.../settings.json
  nvim/                  → ~/.config/nvim/
  ssh/config             → ~/.ssh/config

⚠️  Existing files will be backed up:
  ~/.zshrc
  ~/.gitconfig

Sync files now? [Y/n] y

Syncing from vault...
  ✓ zsh/zshrc        backed up and synced
  ✓ gitconfig        backed up and synced
  ✓ vscode/settings.json  synced (new)
  ✓ nvim/            synced (new)
  ✓ ssh/config       synced (new)

Backups saved to: ~/.gfv-backup-20251107-143000/

Your vault is ready!
```

### Already exists
```
Error: Vault already exists at /Users/username/.gfv
Use a different path or remove the existing vault first.
```

## Files Created

```
~/.gfv/
├── .git/                 # Git repository
├── .gitignore            # Git ignore patterns
└── .vault-manifest.json  # Vault manifest
```

### Default `.gitignore`
```
# System files
.DS_Store
Thumbs.db

# Temporary files
*.tmp
*.swp
*~

# Editor directories
.vscode/
.idea/
```

### Initial `.vault-manifest.json`
```json
{
  "version": "1.0",
  "vaultPath": "/Users/username/.gfv",
  "files": {},
  "remote": {
    "url": "git@github.com:user/configs.git",
    "branch": "main"
  }
}
```

## Exit Codes

- `0` - Success
- `1` - Vault already exists
- `2` - Invalid path or permissions error
- `3` - Git initialization failed
- `4` - Remote configuration/clone failed
- `5` - User cancelled sync

## Notes

- Same command works for all scenarios - `init` auto-detects
- The vault path is stored as an absolute path in the manifest
- Remote is completely optional
- Files are backed up before overwriting when cloning from remote
- Use `--no-sync` to review files before syncing

## See Also

- [link.md](./link.md) - Add files to vault after initialization
- [backup.md](./backup.md) - Push to remote
- [restore.md](./restore.md) - Pull updates from remote
- [OVERVIEW.md](../OVERVIEW.md) - General design overview
