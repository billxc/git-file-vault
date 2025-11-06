# `gfv clone` - Clone Remote Vault

Clone an existing vault from a remote repository.

## Synopsis

```bash
gfv clone <remote-url> [path] [options]
```

## Description

Clones a vault from a remote Git repository and optionally syncs the files to their source locations on your system.

This is typically used when setting up gfv on a new device.

## Arguments

- `<remote-url>` - Remote repository URL (required)
- `[path]` - Local vault path (default: `~/.gfv`)

## Options

- `--no-sync` - Don't auto-sync files after clone
- `--name <name>` - Vault name (default: `default`)
- `--select` - Interactively select which files to sync

## Examples

### Basic clone
```bash
gfv clone git@github.com:user/configs.git
```
Clones to `~/.gfv` and syncs all files.

### Clone to custom location
```bash
gfv clone git@github.com:user/configs.git ~/.my-vault
```

### Clone without syncing
```bash
gfv clone git@github.com:user/configs.git --no-sync
```
Review manifest first, sync manually later.

### Clone with file selection
```bash
gfv clone git@github.com:user/configs.git --select
```
Interactively choose which files to sync.

### Named vault
```bash
gfv clone git@github.com:user/work-configs.git --name work
```

## Behavior

1. **Validate URL**
   - Check if URL is reachable
   - Check if already cloned

2. **Clone repository** (internal)
   - Run `git clone <url> <path>`
   - Read `.vault-manifest.json`

3. **Show sync preview**
   - Display files that will be synced
   - Show source locations
   - Warn about existing files

4. **Confirm with user** (unless `--no-sync`)
   - Ask to proceed with sync
   - Handle existing files (backup/overwrite/skip)

5. **Sync files** (if confirmed)
   - Run `gfv sync --from-vault`
   - Handle conflicts interactively

6. **Update global config**
   - Add vault to config
   - Set as active vault

## Output

### Successful clone with sync
```
Cloning from git@github.com:user/configs.git...
✓ Cloned to /Users/username/.gfv

Found 5 managed files:
  zsh/zshrc              → ~/.zshrc
  gitconfig              → ~/.gitconfig
  vscode/settings.json   → ~/Library/.../settings.json
  nvim/                  → ~/.config/nvim/
  ssh/config             → ~/.ssh/config

⚠️  Existing files detected:
  ~/.zshrc (will be backed up)
  ~/.gitconfig (will be backed up)

Sync these files? [Y/n] y

Syncing from vault...
  ✓ zsh/zshrc        backed up and synced
  ✓ gitconfig        backed up and synced
  ✓ vscode/settings.json  synced (new)
  ✓ nvim/            synced (new)
  ✓ ssh/config       synced (new)

Backups saved to: ~/.gfv-backup-20251106-143000/

Your vault is ready!
```

### Clone without sync
```
Cloning from git@github.com:user/configs.git...
✓ Cloned to /Users/username/.gfv

Vault cloned successfully (--no-sync specified).

To sync files later:
  gfv sync --from-vault
```

### Clone with selection
```
Cloning from git@github.com:user/configs.git...
✓ Cloned to /Users/username/.gfv

Select files to sync:
  [x] zsh/zshrc → ~/.zshrc
  [x] gitconfig → ~/.gitconfig
  [ ] vscode/settings.json → ~/Library/.../settings.json
  [x] nvim/ → ~/.config/nvim/
  [ ] ssh/config → ~/.ssh/config

Continue with 3 selected? [Y/n] y

Syncing 3 files...
...
```

### Already exists
```
Error: Vault already exists at /Users/username/.gfv

Use a different path:
  gfv clone <url> ~/.other-vault

Or remove existing vault first.
```

## Handling Existing Files

When a source file already exists:

### Default: Backup
```
File exists: ~/.zshrc

[B] Backup and overwrite (creates ~/.gfv-backup-*/zshrc)
[O] Overwrite (no backup) ⚠️
[S] Skip this file
[Q] Quit

Your choice: b

✓ Backed up to ~/.gfv-backup-20251106-143000/.zshrc
✓ Synced from vault
```

### Batch mode
```
Multiple files exist. Choose strategy for all:
  [B] Backup all
  [O] Overwrite all ⚠️
  [S] Skip all
  [I] Decide individually

Your choice:
```

## Exit Codes

- `0` - Success
- `1` - Clone failed (network, authentication)
- `2` - Vault already exists at path
- `3` - Invalid manifest in cloned repo
- `4` - Sync conflicts unresolved
- `5` - User cancelled

## Notes

- **Safe by default** - Backs up existing files
- Always creates backup directory with timestamp
- Use `--no-sync` to review before syncing
- Use `--select` for fine-grained control
- Platform-specific files automatically handled

## Common Workflows

### New device setup
```bash
gfv clone git@github.com:user/configs.git
# Review what will be synced, confirm
# Done! Files synced to proper locations
```

### Review before syncing
```bash
gfv clone git@github.com:user/configs.git --no-sync
gfv list --long
gfv status
gfv sync --from-vault
```

### Selective sync
```bash
gfv clone git@github.com:user/configs.git --select
# Uncheck files you don't want on this device
```

### Secondary vault
```bash
gfv clone git@github.com:user/work-configs.git --name work
gfv vault switch work
```

## Backup Location

Backups are saved to: `~/.gfv-backup-YYYYMMDD-HHMMSS/`

Example:
```
~/.gfv-backup-20251106-143000/
├── .zshrc
├── .gitconfig
└── .vimrc
```

You can safely delete these after verifying everything works.

## See Also

- [init.md](./init.md) - Initialize a new vault
- [pull.md](./pull.md) - Pull updates after clone
- [sync.md](./sync.md) - Manual sync
- [OVERVIEW.md](../OVERVIEW.md) - Architecture
