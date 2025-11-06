# `gfv init` - Initialize Vault

Initialize a new vault repository.

## Synopsis

```bash
gfv init [path] [options]
```

## Description

Creates a new vault at the specified path (or default location). This initializes a Git repository and creates the manifest file.

**Remote repository is optional** - You can use gfv purely for local version control without any remote.

This command should be run once per device before using gfv.

## Arguments

- `[path]` - Vault path (default: `~/.gfv`)

## Options

- `--remote <url>` - Set remote repository URL
- `--branch <name>` - Specify branch name (default: `main`)
- `--name <name>` - Vault name for multi-vault support (default: `default`)

## Examples

### Basic initialization (local only)
```bash
gfv init
```
Creates vault at `~/.gfv` without remote. Perfect for single-device local version control.

### Initialize with remote (multi-device)
```bash
gfv init --remote git@github.com:user/configs.git
```
Creates vault and configures Git remote for syncing across devices.

### Initialize at custom path
```bash
gfv init ~/.config-vault
```
Creates vault at specified location.

### Initialize named vault (future)
```bash
gfv init --name work ~/.work-vault
```
Creates a named vault for multi-vault workflows.

## Behavior

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

5. **Configure remote (if provided)**
   - Add Git remote with specified URL
   - Set up tracking branch

6. **Update global config**
   - Add vault entry to `~/.config/gfv/config.toml`
   - Set as active vault if it's the only one

7. **Create initial commit**
   - Commit manifest file: "chore: initialize vault"

## Output

### Success
```
Initializing vault at /Users/username/.gfv
✓ Created directory
✓ Initialized Git repository
✓ Created manifest file
✓ Configured remote: git@github.com:user/configs.git
✓ Vault ready!

Next steps:
  gfv add <file>    # Add files to vault
  gfv push          # Push to remote (if configured)
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
- `4` - Remote configuration failed

## Notes

- The vault path is stored as an absolute path in the manifest
- If no remote is specified, you can add it later with `git remote add` (or future `gfv remote add`)
- The default vault name is `default` unless specified otherwise

## See Also

- [clone.md](./clone.md) - Clone an existing vault from remote
- [add.md](./add.md) - Add files to vault after initialization
- [OVERVIEW.md](../OVERVIEW.md) - General design overview
