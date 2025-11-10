# git-file-vault (gfv)

Git-based file version management tool for syncing configuration files across multiple devices.

**Command:** `git-file-vault` or `gfv` (short alias)

## Overview

git-file-vault helps you manage and sync important configuration files (dotfiles, settings, etc.) across multiple devices using Git as the backend. It handles files scattered across different directories, provides version control, and makes it easy to keep your configs synchronized.

## Key Features

- 📦 **Git-based version control** - Leverage Git's power for your config files
- 🔄 **Smart sync** - Bidirectional sync with intelligent conflict resolution
- 🌍 **Cross-platform** - Works on macOS, Linux, and Windows
- 🤖 **AI-powered messages** - Automatically uses AI if configured, seamless fallback
- 🎯 **Simple CLI** - Easy-to-use command-line interface
- 🏷️ **Platform-specific files** - Tag files for specific operating systems
- 🚫 **No Git knowledge required** - All Git operations handled internally

## Installation

### macOS / Linux (Homebrew)

```bash
# Add the tap
brew tap billxc/tap

# Install gfv
brew install gfv

# Or install in one command
brew install billxc/tap/gfv
```

### Windows (Scoop)

```powershell
# Add the bucket
scoop bucket add xc-scoop https://github.com/billxc/xc-scoop

# Install gfv
scoop install gfv
```

### From source (Rust required)

```bash
git clone https://github.com/billxc/git-file-vault.git
cd git-file-vault
cargo install --path .
```

## Quick Start

### Local-Only Usage (Single Device)

Perfect for local version control without remote sync:

```bash
# 1. Initialize vault
gfv init

# 2. Add config files
gfv link ~/.zshrc
gfv link ~/.gitconfig
gfv link ~/.config/nvim

# 3. Keep files synced
gfv backup
```

Your files are now version controlled locally!

### Multi-Device Usage (With Remote)

For syncing configs across multiple devices:

#### On first device
```bash
# 1. Initialize with remote
gfv init --remote git@github.com:username/my-configs.git

# 2. Add your config files
gfv link ~/.zshrc
gfv link ~/.config/nvim
gfv link ~/.gitconfig

# 3. Push to remote
gfv backup
```

#### On another device
```bash
gfv init --remote git@github.com:username/my-configs.git
```

That's it! Your config files are now synced across devices.

## Usage

### Basic Commands

```bash
# Initialize a new vault
gfv init

# Initialize with remote repository
gfv init --remote git@github.com:username/my-configs.git

# Link files to vault (creates management relationship)
gfv link ~/.zshrc
gfv link ~/.config/nvim --name nvim

# Link platform-specific files
gfv link ~/.ssh/config --platform macos

# List managed files
gfv list

# List with detailed information
gfv list --long

# Check status of managed files
gfv status

# Backup changes to vault (copies files, commits, and pushes to remote if configured)
gfv backup

# Backup with custom commit message
gfv backup -m "Update zsh config"

# Restore from vault (pulls from remote if configured, then copies vault → source)
gfv restore

# Restore with force (overwrites local changes without prompt)
gfv restore --force

# Unlink file from vault (stops managing, keeps source file)
gfv unlink zsh/zshrc

# Unlink and delete from vault
gfv unlink old-config --delete-files
```

### Platform-Specific Files

Mark files as platform-specific:

```bash
gfv link ~/.zshrc --platform macos
gfv link ~/.bashrc --platform linux
```

Files with platform tags will only sync on matching platforms.

### Command Aliases

Create custom shortcuts for frequently used commands:

```bash
# Create an alias
gfv alias add use vault switch

# Use the alias
gfv use work      # Same as: gfv vault switch work

# List aliases
gfv alias list

# Remove alias
gfv alias remove use
```

Common aliases:
```bash
gfv alias add sw vault switch    # Quick vault switching
gfv alias add ls list --long     # Detailed file listing
gfv alias add save backup         # Shorter backup command
```

See [alias.md](./docs/commands/alias.md) for more details.

### Conflict Resolution

When both vault and source files are modified, gfv will prompt you:

```
Conflict detected: zsh/zshrc
  Vault:  modified 2025-11-06 14:30:00
  Source: modified 2025-11-06 14:35:00

Choose:
  [V] Keep vault version
  [S] Keep source version
  [D] Show diff and decide
  [C] Cancel
```

### AI-Generated Commit Messages

**Automatic and seamless** - Just configure once, and `backup` will use AI automatically.

Configure AI endpoint (OpenAI-compatible API):

```bash
gfv config ai.endpoint https://api.openai.com/v1/chat/completions
gfv config ai.api_key sk-xxxxx
gfv config ai.model gpt-4o-mini
```

Then every time you backup without a message:

```bash
gfv backup
# → Generating commit message with AI...
# ✓ Committed locally: "update zsh configuration and git aliases"
```

**Smart fallback:**
- AI configured → Uses AI to generate message automatically
- AI fails or not configured → Falls back to "Update vault"
- Always can override with `-m "message"`

**Supports any OpenAI-compatible endpoint:**
- OpenAI (GPT-3.5, GPT-4, GPT-4o, etc.)
- Azure OpenAI
- Local models (Ollama, LM Studio, etc.)
- Any API following the OpenAI chat completions format

## How It Works

gfv creates a Git repository (vault) that stores all your config files. It maintains a manifest file (`.vault-manifest.json`) that maps each file in the vault to its actual location on your system.

**You never need to use Git commands directly** - gfv handles all Git operations internally:

- `add` - Copies file to vault and commits
- `sync` - Synchronizes files bidirectionally
- `push` - Syncs to vault, commits changes, and pushes to remote
- `pull` - Pulls from remote and syncs to source locations

This allows you to:
- Keep config files in their original locations
- Version control them with Git (transparent to you)
- Sync across devices using Git remotes
- Avoid symlink/hardlink complications
- Never worry about Git commands

## Project Status

🚧 **Early Development (v0.1.0)** - This project is in active development. APIs may change.

### Current Limitations

**Cross-Device Workflow:**
- `gfv link` automatically pushes to remote (cannot defer push)
- `gfv unlink` doesn't push to remote (requires manual `gfv backup`)
- When initializing from existing remote on a new device, local files may differ from repo
- No conflict detection when adding files that exist in remote with different content

**Planned for v0.2.0** (see [TODO.md](./docs/TODO.md)):
- Add `--no-push` flag to `add` and `remove` commands for manual control
- Conflict detection when adding files
- Better support for multi-device workflows

Current workaround for cross-device setup:
```bash
# On new device after init:
gfv restore ~/.zshrc    # Get remote version first
# Review differences before adding
gfv link ~/.zshrc        # This will push immediately
```

## Documentation

- **[Design Overview](./docs/OVERVIEW.md)** - Architecture and design principles
- **Command Reference:**
  - [init](./docs/commands/init.md) - Initialize a vault
  - [link](./docs/commands/link.md) - Link files to vault
  - [unlink](./docs/commands/unlink.md) - Unlink files from vault
  - [list](./docs/commands/list.md) - List managed files
  - [status](./docs/commands/status.md) - Show status
  - [backup](./docs/commands/backup.md) - Backup to remote
  - [restore](./docs/commands/restore.md) - Restore from remote
  - [config](./docs/commands/config.md) - Manage configuration
  - [alias](./docs/commands/alias.md) - Manage command aliases
  - [vault](./docs/commands/vault.md) - Manage vaults

## Roadmap

- [x] Design and architecture
- [ ] Core vault and manifest implementation
- [ ] File sync engine
- [ ] CLI commands (no separate commit command)
- [ ] AI message generation (integrated in push)
- [ ] Cross-platform testing
- [ ] Package distribution

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT License - see LICENSE file for details

## Author

billxc
