# git-file-vault Design Overview

## Project Information

- **Full name:** `git-file-vault`
- **Short alias:** `gfv`
- **Commands:** Both `git-file-vault <command>` and `gfv <command>` are supported

## Overview

git-file-vault is a Git-based file version management tool for syncing configuration files across multiple devices.

### Problem Statement

Users have multiple devices (work computer, home computer, laptop, servers) and need to:
- Manage and sync important configuration files (vscode settings, zsh config, alacritty config, etc.)
- Version control these files with Git
- Handle files scattered across different directories
- Sync changes across devices using remote Git repositories

### Core Concept

- **Vault**: A Git repository that stores all managed files
- **Manifest**: Metadata file (`.vault-manifest.json`) that records file path mappings
- **Sync**: Bidirectional sync mechanism between vault and actual file locations
- **Remote (Optional)**: Git remote repository for cross-device synchronization

**Important:** Remote repository is completely optional. gfv works perfectly fine as a local-only version control system.

### Key Design Principles

1. **Keep it simple first, iterate later**
2. **Requirements-driven development**
3. **Cross-platform support** (macOS, Linux, Windows)
4. **Git-native** - leverage existing Git infrastructure
5. **Abstraction over Git** - Users should NOT directly manage the Git repository; all Git operations are handled by gfv

---

## User Mental Model

git-file-vault (gfv) completely manages the Git repository internally. Users never need to run Git commands directly.

### User Workflow

#### Single Device (Local Only)
1. **Initialize** - Set up a vault (once)
2. **Add files** - Tell gfv which files to manage
3. **Sync** - Keep vault and source files synchronized

#### Multi-Device (With Remote)
1. **Initialize** - Set up a vault (once per device)
2. **Add files** - Tell gfv which files to manage (once per file)
3. **Sync** - Bidirectionally sync and pull from remote
4. **Push** - Upload changes to remote (auto-syncs, auto-commits, and pushes)
5. **Pull** - Download changes from remote (pulls and auto-syncs)

### Core Operations

- **add** - Start managing a file (gfv semantics, not git add)
- **sync** - Full synchronization: commit vault → pull remote → bidirectional sync
- **push** - Upload to remote: sync to-vault → commit → push
- **pull** - Download from remote: pull → sync from-vault

**No separate commit command** - Commits are handled automatically by sync and push.

---

## Architecture

### Directory Structure

gfv uses a clean separation between Git repositories and local configuration:

```
~/.gfv/                          # gfv root directory
├── config.toml                  # Global configuration (AI keys, current vault, etc.)
│
├── default/                     # Default vault
│   ├── repo/                    # Git repository (only file contents)
│   │   ├── .git/
│   │   ├── nvim/init.vim
│   │   └── zsh/zshrc
│   └── manifest.json            # File mappings (NOT in Git)
│
├── work/                        # Work vault (example)
│   ├── repo/
│   └── manifest.json
│
└── personal/                    # Personal vault (example)
    ├── repo/
    └── manifest.json
```

**Key Design Points:**
- **`repo/`** - Git repository containing ONLY managed file contents
- **`manifest.json`** - Local configuration, NOT tracked by Git, stores file path mappings
- **`config.toml`** - Global settings shared across all vaults
- **Multiple vaults** - Each vault is independent with its own repo and manifest

**Why this structure?**
1. **Clean Git history** - Repository only contains actual files, no metadata
2. **Cross-device compatibility** - File mappings are local to each machine
3. **Privacy** - No local paths exposed in Git commits
4. **Flexibility** - Different devices can manage different file subsets

**Important for Cross-Device Usage:**
- Each device maintains its own `manifest.json` (NOT synced via Git)
- This allows Device A to map files differently than Device B
- Example: Device A has files at `/Users/alice/...`, Device B at `/Users/bob/...`
- The Git repo only knows about relative paths like `zsh/zshrc`, not absolute paths
- See [TODO.md](./TODO.md) for planned improvements to cross-device workflows

### Technology Stack

- **Language**: Rust
  - Single binary distribution
  - Cross-platform support
  - Excellent performance
  - Strong type system

- **Core Dependencies**:
  - `clap` - CLI framework
  - `git2` - Git operations
  - `serde` / `serde_json` - Serialization
  - `walkdir` - Directory traversal
  - `chrono` - Timestamp handling
  - `reqwest` - HTTP client (for AI API)
  - `tokio` - Async runtime

### Data Structures

#### Vault Manifest Format

Location: `~/.gfv/{vault-name}/manifest.json` (NOT in Git)

```json
{
  "version": "1.0",
  "files": {
    "zsh/zshrc": {
      "sourcePath": "/Users/username/.zshrc",
      "type": "file",
      "platform": "macos",
      "addedAt": "2025-11-06T14:30:00Z",
      "lastSync": "2025-11-06T15:00:00Z"
    },
    "vscode/settings.json": {
      "sourcePath": "/Users/username/Library/Application Support/Code/User/settings.json",
      "type": "file",
      "addedAt": "2025-11-06T14:31:00Z"
    }
  },
  "remote": {
    "url": "git@github.com:user/my-configs.git",
    "branch": "main"
  }
}
```

**Fields:**
- `version` - Manifest format version
- `vaultPath` - Absolute path to vault directory
- `files` - Map of vault paths to file metadata
  - `sourcePath` - Absolute path to actual file location
  - `type` - "file" or "directory"
  - `platform` - Optional platform restriction (macos, linux, windows)
  - `addedAt` - ISO 8601 timestamp when file was added
  - `lastSync` - ISO 8601 timestamp of last successful sync
- `remote` - Optional remote repository information

#### Global Config Format

Location: `~/.config/gfv/config.toml`

```toml
[vaults]
default = "/Users/username/.gfv"
work = "/Users/username/.work-vault"

[current]
active = "default"

[ai]
provider = "openai"
api_key = "sk-xxxxx"
model = "gpt-3.5-turbo"

[sync]
conflict_strategy = "prompt"  # prompt | use_vault | use_source
```

**Sections:**
- `[vaults]` - Named vault paths
- `[current]` - Currently active vault
- `[ai]` - AI commit message generation settings
- `[sync]` - Sync behavior configuration

---

## Commands Overview

### MVP Commands (v0.1.0)

| Command | Purpose | Documentation |
|---------|---------|---------------|
| `init` | Initialize vault | [init.md](./commands/init.md) |
| `add` | Start managing file | [add.md](./commands/add.md) |
| `remove` | Stop managing file | [remove.md](./commands/remove.md) |
| `list` | List managed files | [list.md](./commands/list.md) |
| `status` | Show status | [status.md](./commands/status.md) |
| `backup` | Copy source → vault, commit, pull, and push | [backup.md](./commands/backup.md) |
| `restore` | Pull and copy vault → source | [restore.md](./commands/restore.md) |
| `config` | Manage configuration | [config.md](./commands/config.md) |

**8 commands** - Simple one-way sync, no conflict detection in MVP.

### Future Commands (v0.2.0+)

- `sync` - Bidirectional sync with conflict detection ([future/sync.md](./commands/future/sync.md))
- `vault` - Multi-vault management ([future/vault.md](./commands/future/vault.md))
- `diff` - Show differences between vault and source
- `restore` - Restore to specific version

**Important:** There is NO `commit` command. Commits are handled automatically.

---

## Core Features

### Path Inference

When adding files, gfv automatically infers a sensible vault path:

| Source Path | Inferred Vault Path |
|-------------|---------------------|
| `~/.zshrc` | `zsh/zshrc` |
| `~/.bashrc` | `bash/bashrc` |
| `~/.gitconfig` | `git/gitconfig` |
| `~/.ssh/config` | `ssh/config` |
| `~/.config/nvim/` | `nvim/` |
| `~/.config/alacritty/` | `alacritty/` |
| `~/Library/Application Support/Code/User/settings.json` | `vscode/settings.json` |
| Other dotfiles `~/.xxxrc` | `xxx/xxxrc` |
| Other paths | Relative to `~` |

Users can override with `--name` option.

### Conflict Resolution

When both vault and source are modified:

```
Conflict detected: zsh/zshrc
  Vault:  modified 2025-11-06 14:30:00
  Source: modified 2025-11-06 14:35:00

Options:
  [V] Keep vault version
  [S] Keep source version
  [D] Show diff and decide
  [C] Cancel (skip this file)

Your choice:
```

**Smart Sync Logic:**
- Only vault modified → Copy vault → source
- Only source modified → Copy source → vault
- Both modified → Conflict resolution (prompt user)
- Neither modified → Skip

### Platform-Specific Files

Files can be optionally marked as platform-specific:

```bash
gfv link ~/.zshrc --platform macos
```

**Behavior:**
- Files without platform tag → Sync on all platforms
- Files with platform tag → Only sync on matching platform
- Use `--ignore-platform` to override

### AI Commit Message Generation

**Smart auto-detection** - No manual flag needed.

**Configuration:**
```bash
gfv config ai.api_key sk-xxxxx
```

**Priority:**
1. User-specified message (`-m "message"`)
2. AI-generated (if configured)
3. Auto-generated (simple fallback)

**Behavior:**
- If AI configured: Try AI generation → Show preview → Let user accept/edit
- If AI fails: Fallback to simple message
- Always works, even without configuration

---

## Multi-Vault Support

### MVP: Single Default Vault

- Default vault at `~/.gfv`
- Global config tracks active vault
- Extension points for future multi-vault support

### Future: Multiple Vaults

```bash
gfv init --name work ~/.work-vault
gfv vault switch work
gfv link ~/.ssh/work_config --vault work
```

---

## Security Considerations

### Sensitive Files

- Detect potentially sensitive files (`.env`, `credentials.json`, `*.key`)
- Warn user before adding
- Require `--force` flag to proceed
- Recommend `.gitignore` for sensitive patterns

### API Keys

- Never log or display API keys
- Store in config file with restricted permissions (0600)
- Support environment variables

---

## Error Handling

### Principles

1. **Clear error messages** - Tell user what went wrong and how to fix it
2. **Graceful degradation** - Continue operation when possible
3. **No data loss** - Always backup before destructive operations

### Example Messages

```
Error: Source file not found: /Users/username/.zshrc
Hint: The file may have been moved or deleted. Remove it from vault with:
  gfv unlink zsh/zshrc
```

```
Error: Vault not initialized
Run 'gfv init' to create a new vault.
```

---

## Command Interaction with Git

### What Users See vs What Happens

| User Command | User-Facing Behavior | Internal Git Operations |
|--------------|---------------------|-------------------------|
| `gfv link` | Start managing a file | git add + git commit |
| `gfv unlink` | Stop managing a file | git rm + git commit |
| `gfv sync` | Bidirectional sync | sync to-vault + git commit + git pull + sync from-vault |
| `gfv backup` | Upload changes | sync to-vault + git commit + git push |
| `gfv restore` | Download changes | git pull + sync from-vault |
| `gfv status` | Show sync status | git status + custom logic |

**Users NEVER run Git commands directly.**

### Sync Command Implementation

`gfv sync` performs a full synchronization:

1. **Sync source → vault** - Copy modified source files to vault
2. **Commit changes** - Auto-commit with message like "sync: update from sources"
3. **Pull from remote** - Get latest changes (git pull)
4. **Sync vault → source** - Copy pulled changes back to source locations

This ensures vault, source files, and remote stay synchronized.

---

## Testing Strategy

### Unit Tests
- Path inference logic
- Manifest serialization/deserialization
- Conflict detection
- Platform detection

### Integration Tests
- End-to-end command flows
- Git operations
- File sync operations
- Multi-device scenarios (simulated)

### Manual Testing
- Cross-platform testing (macOS, Linux, Windows)
- Real-world dotfiles
- Large file/directory handling

---

## Roadmap

### v0.1.0 - MVP (Simplified)
- ✅ Single vault support (default at `~/.gfv`)
- ✅ Core commands: init, add, remove, list, status, backup, restore, config (8 commands)
- ✅ Simple one-way operations: backup (source → vault → remote), restore (remote → vault → source)
- ✅ NO conflict detection in MVP (except Git-level conflicts which stop execution)
- ✅ NO separate commit command (handled by backup)
- ✅ NO separate clone command (handled by init)
- ✅ Platform tags (optional)
- ✅ AI message generation (smart auto-detection)
- ✅ Path inference
- ✅ Complete Git abstraction

### v0.2.0 - Enhanced Sync
- Bidirectional sync with conflict detection (`sync` command)
- Multi-vault support (`vault` command)
- Conflict resolution strategies
- External merge tools
- `diff` command
- `restore` command with version history
- Config management commands

### v0.3.0 - Advanced Features
- Multi-platform file variants
- Encrypted vault support
- Hooks system (pre-sync, post-sync, etc.)
- Template system for initial setups
- Interactive TUI mode

### v1.0.0 - Production Ready
- Comprehensive documentation
- Extensive test coverage
- Performance optimization
- Published to package managers (cargo, homebrew, apt, etc.)

---

## Development Notes

- Design follows "keep it simple first, iterate later" principle
- Extension points built in for future features
- Focus on user experience and clear error messages
- Cross-platform support is first-class concern
