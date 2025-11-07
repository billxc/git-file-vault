# `gfv link` - Link File to Vault

Create a management relationship between a local path and vault path.

## Synopsis

```bash
gfv link <source-path> [options]
```

## Description

Establishes a link in the manifest between a local file path and a vault path. This tells gfv to start managing the file.

**Key behaviors:**
- **Does NOT copy files** - Only updates the manifest
- File can exist locally, in vault, or both
- Use `gfv backup` to upload local → vault
- Use `gfv restore` to download vault → local

**This is NOT `git add`** - it's a gfv-specific operation for declaring "I want to manage this file."

## Arguments

- `<source-path>` - Local path to manage (can be non-existent)

## Options

- `--name <name>` - Specify vault path (overrides auto-inference)
- `--platform <os>` - Mark as platform-specific (`macos`, `linux`, `windows`)
- `--vault <name>` - Specify which vault to use (default: active vault)

## Examples

### Link a file
```bash
gfv link ~/.zshrc
```
Inferred vault path: `zsh/zshrc`

### Link with custom name
```bash
gfv link ~/.config/nvim --name nvim
```
Vault path: `nvim/`

### Link platform-specific file
```bash
gfv link ~/.ssh/config --platform macos
```

### Link to a specific vault
```bash
gfv link ~/.zshrc --vault work
gfv link ~/.gitconfig --vault personal
```

### Link on new device (file in vault only)
```bash
# After: gfv init --remote <url>
gfv link ~/.zshrc        # Creates manifest link
gfv restore              # Downloads from vault
```

## Path Inference

gfv automatically infers vault paths from source paths:

| Source | Vault Path | Rule |
|--------|-----------|------|
| `~/.zshrc` | `zsh/zshrc` | Dotfile: `.{name}rc` → `{name}/{name}rc` |
| `~/.gitconfig` | `git/gitconfig` | Known pattern |
| `~/.config/nvim/` | `nvim/` | Strip `~/.config/` |
| `~/.ssh/config` | `ssh/config` | SSH files |
| `~/Documents/file.txt` | `Documents/file.txt` | Relative to home |

**Use `--name` to override inference.**

## Behavior

1. Resolve source path to absolute
2. Infer or use provided vault path
3. Check if already managed (error if yes)
4. Check file existence (must exist locally OR in vault)
5. Detect file type (file/directory)
6. Warn if sensitive file pattern detected
7. Create manifest entry (local only)
8. Display status and next steps

**No files are copied or committed.**

## Output

### File exists locally only
```
==> Linking /Users/user/.zshrc
  Vault path: zsh/zshrc
  Platform: all
→ File exists locally but not in vault
   Use 'gfv backup' to upload it
✓ Updated manifest

File is now managed by gfv.
```

### File exists in vault only
```
==> Linking /Users/user/.zshrc
  Vault path: zsh/zshrc
  Platform: all
→ File exists in vault but not locally
   Use 'gfv restore' to download it
✓ Updated manifest

File is now managed by gfv.
```

### File exists in both
```
==> Linking /Users/user/.zshrc
  Vault path: zsh/zshrc
  Platform: all
→ File exists in both locations
✓ Updated manifest

File is now managed by gfv.
```

### File not found
```
Error: File not found in either location:
  Local: /Users/user/.zshrc
  Vault: /Users/user/.gfv/default/repo/zsh/zshrc
```

### Already managed
```
Error: File already managed
The file /Users/user/.zshrc is already in the vault as zsh/zshrc

To update it, use:
  gfv backup
```

### Sensitive file warning
```
Warning: Potentially sensitive file detected
  /Users/user/.env

This file may contain secrets or credentials.
Are you sure you want to add it to version control? (y/N)
```

## Manifest Entry

After linking `~/.zshrc`:

```json
{
  "files": {
    "zsh/zshrc": {
      "sourcePath": "/Users/user/.zshrc",
      "type": "file",
      "addedAt": "2025-11-06T14:30:00Z",
      "lastSync": null
    }
  }
}
```

With platform tag:

```json
{
  "files": {
    "ssh/config": {
      "sourcePath": "/Users/user/.ssh/config",
      "type": "file",
      "platform": "macos",
      "addedAt": "2025-11-06T14:30:00Z",
      "lastSync": null
    }
  }
}
```

## Notes

- **Manifest only** - Link updates local manifest, not vault repo
- **No file operations** - Doesn't copy, move, or modify files
- File can be non-existent locally (useful for multi-device setup)
- Use `gfv backup` to sync local → vault
- Use `gfv restore` to sync vault → local
- Use `gfv unlink` to stop managing
- Sensitive patterns: `.env`, `*credential*`, `*.key`, `*.pem`, `*secret*`, `*password*`

## Common Workflows

### First device
```bash
gfv link ~/.zshrc
gfv backup              # Upload to vault
```

### New device
```bash
gfv init --remote <url>
gfv link ~/.zshrc       # Create link
gfv restore             # Download files
```

### Explicit naming
```bash
gfv link ~/.zshrc --name shell/zshrc
gfv link ~/.bashrc --name shell/bashrc
```

## See Also

- [backup.md](./backup.md) - Upload local → vault
- [restore.md](./restore.md) - Download vault → local
- [unlink.md](./unlink.md) - Stop managing
- [list.md](./list.md) - List managed files
