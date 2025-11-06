# git-file-vault Design Document

## Project Name

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

### Key Design Principles

1. **Keep it simple first, iterate later**
2. **Requirements-driven development**
3. **Cross-platform support** (macOS, Linux, Windows)
4. **Git-native** - leverage existing Git infrastructure
5. **Abstraction over Git** - Users should NOT directly manage the Git repository; all Git operations are handled by git-vault

---

## Architecture

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

```json
{
  "version": "1.0",
  "vaultPath": "/Users/username/.gfv",
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

#### Config File Format

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

---

## User Mental Model

git-file-vault (gfv) completely manages the Git repository internally. Users never need to run Git commands directly.

### User Workflow

1. **Initialize** - Set up a vault (once per device)
2. **Add files** - Tell gfv which files to manage (once per file)
3. **Sync** - Bidirectionally sync between vault and source locations
4. **Push** - Upload changes to remote (auto-syncs, auto-commits, and pushes)
5. **Pull** - Download changes from remote (pulls and auto-syncs)

### Core Operations

- **add** - Start managing a file (gfv semantics, not git add)
- **sync** - Synchronize files between vault and actual locations
- **push** - Sync to vault → Auto-commit → Push to remote
- **pull** - Pull from remote → Sync to source locations

**No separate commit command** - Commits are handled automatically by push.

---

## Command Specifications

### MVP Commands (v0.1.0)

#### `gfv init [path]`

Initialize a new vault.

**Arguments:**
- `[path]` - Vault path (default: `~/.gfv`)

**Options:**
- `--remote <url>` - Set remote repository
- `--branch <name>` - Specify branch (default: `main`)
- `--name <name>` - Vault name (default: `default`)

**Examples:**
```bash
gfv init
gfv init ~/.config-vault --remote git@github.com:user/configs.git
gfv init --name work ~/.work-vault
```

**Behavior:**
1. Create directory if not exists
2. Initialize Git repository
3. Create `.vault-manifest.json`
4. Add to global config
5. Optionally set remote

---

#### `gfv add <source-path> [options]`

**Semantics:** Tell gfv to start managing this file. This is NOT git add - it's a gfv operation.

**Arguments:**
- `<source-path>` - File or directory to start managing

**Options:**
- `--name <name>` - Specify vault path (overrides auto-inference)
- `--platform <os>` - Mark as platform-specific (macos, linux, windows)
- `--vault <name>` - Use specific vault (default: current active)

**Examples:**
```bash
gfv add ~/.zshrc
gfv add ~/.config/nvim --name nvim
gfv add ~/.ssh/config --platform macos
```

**Behavior:**
1. Check if file is already managed (error if yes)
2. Resolve source path to absolute path
3. Infer vault path (or use `--name`)
4. Copy file/directory to vault
5. Update manifest
6. Internally: `git add` and `git commit` (transparent to user)

**Important:** This command is idempotent for the same file - if already managed, it will error. Users cannot "re-add" files; they use `sync` to update them.

**Path Inference Rules:**

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

---

#### `gfv remove <vault-path>`

**Semantics:** Stop managing a file.

**Arguments:**
- `<vault-path>` - File path in vault

**Options:**
- `--keep-source` - Keep source file (default)
- `--delete-source` - Also delete source file
- `--force` - No confirmation prompt

**Examples:**
```bash
gfv remove zsh/zshrc
gfv remove vscode/settings.json --delete-source
```

**Behavior:**
1. Remove from manifest
2. Remove from vault filesystem
3. Internally: `git rm` and `git commit` (transparent to user)
4. Optionally delete source file if `--delete-source`

---

#### `gfv list [options]`

List all managed files.

**Options:**
- `--long` / `-l` - Show detailed information
- `--platform <os>` - Filter by platform
- `--modified` - Show only modified files

**Output:**
```
VAULT PATH              SOURCE PATH                           STATUS
zsh/zshrc               ~/.zshrc                             synced
vscode/settings.json    ~/Library/.../settings.json          modified (vault)
nvim/                   ~/.config/nvim/                      modified (source)
```

---

#### `gfv sync [files...] [options]`

**Semantics:** Bidirectionally synchronize files between vault and source locations. Does NOT commit.

**Arguments:**
- `[files...]` - Specific files to sync (default: all)

**Options:**
- `--to-vault` / `-t` - Only sync source → vault
- `--from-vault` / `-f` - Only sync vault → source
- `--force-vault` - Use vault version on conflict
- `--force-source` - Use source version on conflict
- `--ignore-platform` - Ignore platform restrictions
- `--dry-run` - Show what would be done

**Examples:**
```bash
gfv sync                                  # Smart bidirectional sync
gfv sync --to-vault                       # Only copy source → vault
gfv sync zsh/zshrc vscode/settings.json   # Sync specific files
gfv sync --force-vault --dry-run          # Preview with conflict resolution
```

**Behavior:**
1. For each managed file, compare vault and source versions
2. Apply smart sync logic (see below)
3. Internally: `git add` changed files (does NOT commit)
4. Report what was synced

**Smart Sync Logic:**
- Only vault modified → Copy vault → source
- Only source modified → Copy source → vault
- Both modified → Conflict resolution (prompt user unless `--force-*`)
- Neither modified → Skip

**Conflict Resolution:**

Default strategy: **prompt user** (unless `--force-vault` or `--force-source` specified)

```
Conflict detected: zsh/zshrc
  Vault:  modified 2025-11-06 14:30:00
  Source: modified 2025-11-06 14:35:00

Choose:
  [V] Keep vault version
  [S] Keep source version
  [D] Show diff and decide
  [C] Cancel (skip this file)

Your choice:
```

**Important:** `sync` does NOT create Git commits. Use `push` to commit and upload changes.

---

#### `gfv status`

**Semantics:** Show current state of managed files and remote status.

**Options:**
- `--short` / `-s` - Short format

**Output:**
```
Vault: /Users/username/.gfv (default)
Remote: git@github.com:user/configs.git
Branch: main

Sync Status:
  M  zsh/zshrc          source modified (2025-11-06 15:30:00)
  M  gitconfig          vault modified (2025-11-06 14:20:00)
  C  nvim/init.lua      conflict
  ✓  vscode/settings.json  synced

Remote Status:
  ↓ 2 commits behind
  ↑ 1 commit ahead (uncommitted changes in vault)

Summary:
  3 files need sync (1 conflict)
  Run 'gfv sync' to synchronize
  Run 'gfv pull' to get remote changes
  Run 'gfv push' to upload local changes
```

**Note:** This command shows both sync status AND Git repository status, but users don't need to understand Git internals.

---

#### `gfv push [options]`

**Semantics:** Upload local changes to remote. Automatically syncs, commits, and pushes.

**Options:**
- `--message <msg>` / `-m <msg>` - Specify commit message
- `--no-sync` - Skip auto-sync (advanced users only)
- `--force` / `-f` - Force push (dangerous!)
- `--set-upstream` / `-u` - Set upstream branch

**Examples:**
```bash
# Auto mode: sync + auto-commit + push
# If AI configured, uses AI; otherwise uses simple auto-generated message
gfv push
# → Syncing to vault...
# → Changes detected: zsh/zshrc
# → [AI configured] Generating commit message...
# → Suggested: "feat: add git aliases to zsh config"
# → Pushing to remote...

# Specify commit message (bypasses AI)
gfv push -m "Add new git aliases"

# Nothing to push
gfv push
# → Everything up to date
```

**Behavior:**
1. Auto `sync --to-vault` (unless `--no-sync`)
2. Check for changes in vault
3. If changes exist:
   - If `-m` provided: Use user message
   - Else if AI configured: Try AI generation (fallback to auto if fails)
   - Else: Auto-generate simple message
4. Internally: `git commit` (transparent to user)
5. `git push` to remote

**Commit Message Generation Priority:**
1. **User-specified** (`-m "message"`) - Highest priority
2. **AI-generated** (if API key configured) - Try to generate, show preview
3. **Auto-generated** (fallback) - Simple message like "Update zsh/zshrc"

**AI Generation Flow (if configured):**
```
Analyzing changes...
Suggested message: "feat: add git aliases and improve prompt"

[A]ccept / [E]dit / [R]egenerate / [C]ancel?
> a

Pushing to remote...
```

**Auto-Generated Message Examples:**
- Single file: `"Update zsh/zshrc"`
- Multiple files: `"Update zsh/zshrc, gitconfig, and 2 other files"`
- New file added: `"Add nvim/init.lua"`
- File removed: `"Remove old-config"`

**Important:** Users never run `git commit` manually. `push` handles all committing.

---

#### `gfv pull [options]`

**Semantics:** Download changes from remote and sync to source locations.

**Options:**
- `--rebase` - Use rebase instead of merge
- `--no-sync` - Don't auto-sync after pull
- `--force-vault` - On conflict, use vault version
- `--force-source` - On conflict, use source version

**Examples:**
```bash
gfv pull
# → Pulling from remote...
# → Syncing from vault to source locations...
# → Updated: zsh/zshrc, gitconfig

gfv pull --rebase
gfv pull --no-sync  # Advanced: pull but don't sync to source
```

**Behavior:**
1. Internally: `git pull` (or `git pull --rebase`)
2. Auto `sync --from-vault` (unless `--no-sync`)
3. Handle conflicts during sync if any

---

#### `gfv clone <remote-url> [path]`

Clone a vault from remote repository.

**Arguments:**
- `<remote-url>` - Remote repository URL
- `[path]` - Local vault path (default: `~/.gfv`)

**Options:**
- `--no-sync` - Don't auto-sync after clone
- `--name <name>` - Vault name (default: `default`)

**Examples:**
```bash
gfv clone git@github.com:user/configs.git
gfv clone https://github.com/user/configs.git ~/.my-vault --name personal
```

**Behavior:**
1. Git clone repository
2. Read manifest
3. Show files that will be synced
4. Prompt for confirmation
5. Sync files from vault to source locations
   - If source file exists → Prompt (backup/overwrite/skip)

---

### Future Commands (v0.2.0+)

- `gfv diff [vault-path]` - Show differences between vault and source
- `gfv restore <vault-path> --version <ref>` - Restore to specific version
- `gfv remote <add|remove|list>` - Manage remotes (wrapper for git remote)
- `gfv config <key> [value]` - Manage configuration
- `gfv vault <list|switch|create>` - Multi-vault management

**Note:** There is NO `gfv commit` command. Commits are handled automatically by `push` and internal operations like `add` and `remove`.

---

## Conflict Resolution Strategy

### MVP: Three-way choice with diff display

When both vault and source are modified:

```
Conflict detected: zsh/zshrc
  Vault:  modified 2025-11-06 14:30:00 (5 lines changed)
  Source: modified 2025-11-06 14:35:00 (3 lines changed)

Options:
  [V] Keep vault version
  [S] Keep source version
  [D] Show diff and decide
  [C] Cancel (skip this file)

Your choice: D

--- Vault version (2025-11-06 14:30:00) ---
+++ Source version (2025-11-06 14:35:00) +++
@@ -10,3 +10,4 @@
 alias ll='ls -la'
+alias gs='git status'
 export PATH="$HOME/.local/bin:$PATH"
+export EDITOR="nvim"

After viewing diff, choose: [V/S/C]:
```

### Future: External merge tools

```toml
[merge]
tool = "vimdiff"  # or "code --diff", "meld", etc.
auto_merge = false
```

---

## Platform-Specific Files

### MVP: Optional platform tags

Files can be optionally marked as platform-specific:

```bash
gfv add ~/.zshrc --platform macos
```

**Sync behavior:**
- Files without platform tag → Sync on all platforms
- Files with platform tag → Only sync on matching platform
- Use `--ignore-platform` to override

### Future: Multi-platform variants

Support multiple versions of the same file for different platforms:

```bash
gfv add ~/.zshrc --platform macos --name zsh/zshrc-macos
gfv add ~/.zshrc --platform linux --name zsh/zshrc-linux
```

Auto-select appropriate version during sync based on current platform.

---

## AI Commit Message Generation

### Smart Auto-Detection

**No manual flag needed** - gfv automatically uses AI if configured.

**Configuration:**
```bash
gfv config ai.api_key sk-xxxxx
gfv config ai.model gpt-3.5-turbo  # optional
```

**Priority:**
1. Environment variable `OPENAI_API_KEY`
2. Config file `~/.config/gfv/config.toml`
3. If not configured: Skip AI, use simple auto-generation

**Behavior:**
- If AI key configured: Try AI generation → Show preview → Let user accept/edit/regenerate
- If AI fails or not configured: Fallback to simple auto-generation
- User can always override with `-m "message"`

**API Call:**
```
POST https://api.openai.com/v1/chat/completions
{
  "model": "gpt-3.5-turbo",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant that generates concise git commit messages. Follow conventional commit format when appropriate (feat:, fix:, chore:, docs:, etc.)."
    },
    {
      "role": "user",
      "content": "Generate a commit message for these changes:\n\n<git diff output>"
    }
  ],
  "temperature": 0.7,
  "max_tokens": 100
}
```

**Error Handling:**
- API call fails → Fallback to simple auto-generation
- Invalid API key → Warn user once, fallback to simple auto-generation
- Network timeout → Fallback to simple auto-generation

### Future: Multiple providers

```toml
[ai]
provider = "openai"  # or "anthropic", "local", "ollama"
api_key = "sk-xxxxx"
model = "gpt-4"
api_base = "https://api.openai.com/v1"  # custom endpoint support
```

---

## Multi-Vault Support

### MVP: Single default vault with extension points

Most users only need one vault. Multi-vault support is designed but simplified in MVP:

**Current implementation:**
- Default vault at `~/.gfv`
- Global config tracks current active vault
- All commands operate on active vault

**Extension points in code:**
- Vault resolution logic abstracted
- Config supports multiple vault entries
- Commands accept `--vault <name>` option (parsed but may not be fully implemented)

### Future: Full multi-vault support

```bash
# Create named vaults
gfv init --name work ~/.work-vault
gfv init --name personal ~/.personal-vault

# Switch between vaults
gfv vault switch work
gfv vault list

# Operate on specific vault
gfv add ~/.ssh/work_config --vault work
gfv status --vault personal
```

---

## Security Considerations

### Sensitive Files

**Warning system:**
- Detect potentially sensitive files (`.env`, `credentials.json`, `*.key`, etc.)
- Warn user before adding
- Require `--force` flag to proceed

**Git security:**
- Always use `.gitignore` for sensitive patterns
- Recommend encrypted Git repos for sensitive configs

### API Keys

- Never log or display API keys
- Store in config file with restricted permissions (0600)
- Support environment variables for CI/CD

---

## Error Handling

### Principles

1. **Clear error messages** - Tell user what went wrong and how to fix it
2. **Graceful degradation** - Continue operation when possible
3. **No data loss** - Always backup before destructive operations

### Examples

```
Error: Source file not found: /Users/username/.zshrc
Hint: The file may have been moved or deleted. Remove it from vault with:
  gfv remove zsh/zshrc
```

```
Error: Vault not initialized
Run 'gfv init' to create a new vault.
```

```
Error: Uncommitted changes in vault
You have uncommitted changes. Please push to upload them:
  gfv push
Or specify a commit message:
  gfv push -m "your message"
```

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

### v0.1.0 - MVP
- ✅ Single vault support
- ✅ Core commands: init, add, remove, list, sync, status, push, pull, clone
- ✅ NO separate commit command (handled by push)
- ✅ Basic conflict resolution (three-way choice)
- ✅ Platform tags (optional)
- ✅ OpenAI commit message generation (integrated in push)
- ✅ Path inference
- ✅ Complete Git abstraction

### v0.2.0 - Enhanced Features
- Multi-vault support (full implementation)
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

## Open Questions

Items to discuss with user:

1. ~~**add/commit command behavior**~~ - ✅ RESOLVED: No separate commit command; commits handled by push
2. ~~**Auto-sync timing**~~ - ✅ RESOLVED: push auto-syncs to vault, pull auto-syncs from vault
3. **Backup strategy** - Should we auto-backup files before overwriting?
4. **Large file handling** - Size limits? Git LFS integration?
5. **Directory sync** - Recursive by default? Exclude patterns?
6. **Initial commit message** - What message to use when `add` creates its automatic commit?

---

## Command Summary

### Commands Available to Users

| Command | Purpose | Git Interaction |
|---------|---------|-----------------|
| `init` | Initialize vault | Creates Git repo |
| `add` | Start managing file | Internal: git add + commit |
| `remove` | Stop managing file | Internal: git rm + commit |
| `list` | List managed files | Reads manifest |
| `sync` | Bidirectional sync | Internal: git add (no commit) |
| `status` | Show status | Reads Git status |
| `push` | Upload changes | Internal: sync + commit + push |
| `pull` | Download changes | Internal: pull + sync |
| `clone` | Clone remote vault | Internal: git clone + sync |

### Commands Users NEVER Use

- `git add` - Handled internally by sync, add, remove
- `git commit` - Handled internally by push, add, remove
- `git push` - Wrapped by `gfv push`
- `git pull` - Wrapped by `gfv pull`

Users operate at the file-sync abstraction level, not the Git level.

---

## Notes

- Design follows "keep it simple first, iterate later" principle
- Extension points built in for future features
- Focus on user experience and clear error messages
- Cross-platform support is first-class concern
