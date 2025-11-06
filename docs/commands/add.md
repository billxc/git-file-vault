# `gfv add` - Add File to Vault

Start managing a file or directory with gfv.

## Synopsis

```bash
gfv add <source-path> [options]
```

## Description

Adds a file or directory to the vault for version control. The file is copied to the vault, metadata is recorded in the manifest, and changes are committed to Git.

**This is NOT `git add`** - it's a gfv-specific operation that means "start managing this file."

## Arguments

- `<source-path>` - File or directory to start managing (required)

## Options

- `--name <name>` - Specify vault path (overrides auto-inference)
- `--platform <os>` - Mark as platform-specific (`macos`, `linux`, `windows`)
- `--vault <name>` - Use specific vault (default: current active)

## Examples

### Add a single file
```bash
gfv add ~/.zshrc
```
Auto-inferred vault path: `zsh/zshrc`

### Add with custom name
```bash
gfv add ~/.config/nvim --name nvim
```
Stored in vault as: `nvim/`

### Add platform-specific file
```bash
gfv add ~/.ssh/config --platform macos
```
Only syncs on macOS systems.

### Add directory
```bash
gfv add ~/.config/alacritty
```
Recursively copies entire directory.

## Path Inference

gfv automatically infers a sensible vault path based on common patterns:

| Source Path | Inferred Vault Path | Pattern |
|-------------|---------------------|---------|
| `~/.zshrc` | `zsh/zshrc` | Dotfile pattern |
| `~/.bashrc` | `bash/bashrc` | Dotfile pattern |
| `~/.gitconfig` | `git/gitconfig` | Dotfile pattern |
| `~/.ssh/config` | `ssh/config` | SSH config |
| `~/.config/nvim/` | `nvim/` | Strip `.config/` |
| `~/.config/alacritty/` | `alacritty/` | Strip `.config/` |
| `~/Library/Application Support/Code/User/settings.json` | `vscode/settings.json` | Known app |
| `~/.myrc` | `myrc/myrc` | Generic dotfile |
| `~/Documents/script.sh` | `Documents/script.sh` | Other paths |

**Override with `--name` if needed.**

## Behavior

1. **Validate source path**
   - Check if file/directory exists
   - Resolve to absolute path
   - Check if already managed (error if yes)

2. **Check for sensitive files**
   - Detect patterns: `.env`, `*credentials*`, `*.key`, `*secret*`
   - Warn user with confirmation prompt
   - Require `--force` to proceed

3. **Infer or use vault path**
   - Apply path inference rules (unless `--name` provided)
   - Check if vault path already exists (error if yes)

4. **Copy file/directory to vault**
   - Preserve permissions and timestamps
   - For directories: recursive copy (respect `.gitignore` patterns)

5. **Update manifest**
   - Add entry with metadata (source path, type, platform, timestamp)
   - Write manifest to disk

6. **Commit to Git**
   - `git add` the new file(s) and manifest
   - `git commit` with message: `"Add <vault-path>"`
   - All transparent to user

## Output

### Success
```
Adding /Users/username/.zshrc
  Vault path: zsh/zshrc
  Platform: all
✓ Copied to vault
✓ Updated manifest
✓ Committed changes

File is now managed by gfv.
```

### Already managed
```
Error: File already managed
The file /Users/username/.zshrc is already in the vault as zsh/zshrc

To update it, use:
  gfv sync zsh/zshrc
```

### Sensitive file warning
```
Warning: Potentially sensitive file detected
  /Users/username/.env

This file may contain secrets or credentials.
Are you sure you want to add it to version control?

[y] Yes, add it
[n] No, cancel
[i] Ignore this warning (use --force)

Your choice:
```

### Directory
```
Adding /Users/username/.config/nvim (directory)
  Vault path: nvim/
  Files: 23 files, 1.2 MB
✓ Copied to vault
✓ Updated manifest
✓ Committed changes

Directory is now managed by gfv.
```

## Manifest Entry

After adding `~/.zshrc`, the manifest contains:

```json
{
  "files": {
    "zsh/zshrc": {
      "sourcePath": "/Users/username/.zshrc",
      "type": "file",
      "addedAt": "2025-11-06T14:30:00Z",
      "lastSync": "2025-11-06T14:30:00Z"
    }
  }
}
```

With platform tag:

```json
{
  "files": {
    "ssh/config": {
      "sourcePath": "/Users/username/.ssh/config",
      "type": "file",
      "platform": "macos",
      "addedAt": "2025-11-06T14:30:00Z",
      "lastSync": "2025-11-06T14:30:00Z"
    }
  }
}
```

## Exit Codes

- `0` - Success
- `1` - File already managed
- `2` - Source file not found
- `3` - Vault not initialized
- `4` - Sensitive file rejected by user
- `5` - Vault path conflict

## Notes

- **This command commits automatically** - No separate commit needed
- Once added, use `gfv sync` to update the file content
- Use `gfv remove` to stop managing a file
- The source file remains in its original location
- Files are copied, not moved or linked

## Common Patterns

### Add multiple dotfiles
```bash
gfv add ~/.zshrc
gfv add ~/.gitconfig
gfv add ~/.vimrc
```

### Add entire config directory
```bash
gfv add ~/.config/alacritty
gfv add ~/.config/nvim
```

### Add with explicit names for clarity
```bash
gfv add ~/.zshrc --name shell/zshrc
gfv add ~/.bashrc --name shell/bashrc
```

## See Also

- [remove.md](./remove.md) - Stop managing a file
- [sync.md](./sync.md) - Update file content
- [list.md](./list.md) - List managed files
- [OVERVIEW.md](../OVERVIEW.md#path-inference) - Path inference rules
