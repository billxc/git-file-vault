# TODO List

## High Priority - Cross-Device Scenarios

### 1. Add `--no-push` flag to `gfv add` command
**Problem**: When initializing from an existing remote on a new machine, the local files and repo files may differ. Currently `gfv add` automatically pushes, which would overwrite the remote state.

**Scenario**:
```bash
# Machine A: Create vault and add files
gfv init --remote https://github.com/user/vault
gfv add ~/.zshrc  # Pushes to remote

# Machine B: Clone vault
gfv init --remote https://github.com/user/vault  # Clones existing repo
gfv add ~/.zshrc  # Currently would push immediately, overwriting Machine A's version
```

**Solution**: Add `--no-push` flag to allow adding files without immediate push:
```bash
gfv add ~/.zshrc --no-push  # Just commit locally
gfv backup  # User decides when to push
```

**Implementation**:
- Add `no_push: bool` parameter to `add()` function
- Add `--no-push` CLI flag
- Skip push step if flag is set
- Update documentation

### 2. Add `--keep-files` option to `gfv remove` command
**Problem**: Currently `gfv remove` requires `--delete-files` flag. The default behavior should be configurable.

**Scenario**:
```bash
# User wants to stop managing a file but keep it in vault
gfv remove config/app.conf --keep-files  # Remove from manifest, keep in repo
gfv remove config/app.conf --delete-files  # Remove from manifest and repo
```

**Current behavior**:
- Without `--delete-files`: Keeps files in vault (good default)
- No push after removal (should add `--no-push` flag too)

**Solution**: Add `--no-push` flag to `remove` command:
```bash
gfv remove test/file.txt --delete-files --no-push  # Remove but don't push yet
```

## Medium Priority - Multi-Machine Workflow

### 3. Detect and warn about file conflicts
**Problem**: When adding a file that exists in remote but differs from local version, should warn user.

**Solution**:
```bash
gfv add ~/.zshrc
# Warning: File exists in remote vault with different content
# Remote version: 2.3 KB, modified 2 days ago
# Local version: 1.8 KB, modified today
#
# Options:
# [1] Use local version (overwrites remote)
# [2] Use remote version (restore from vault)
# [3] Cancel
```

### 4. Add `gfv diff` command
**Purpose**: Show differences between local files and vault versions

```bash
gfv diff ~/.zshrc           # Diff specific file
gfv diff                    # Diff all managed files
gfv diff --remote           # Compare vault with remote
```

### 5. Improve `gfv restore` for selective restore
**Current**: Restores all files
**Needed**: Allow selective restore on new machine

```bash
gfv restore ~/.zshrc        # Restore specific file
gfv restore --select        # Interactive selection
gfv restore --platform      # Only restore platform-matched files
```

## Low Priority - Quality of Life

### 6. Configuration for default behavior
Allow users to configure default push behavior:

```bash
gfv config auto_push true   # Always push after add/remove
gfv config auto_push false  # Never push automatically
gfv config auto_push prompt # Prompt user each time
```

### 7. Batch operations
```bash
gfv add ~/.zshrc ~/.gitconfig ~/.vimrc --no-push  # Add multiple files
gfv backup --batch  # Push all pending changes at once
```

### 8. Show unpushed commits
```bash
gfv status
# Output should show:
# ✓ 3 managed files
# ● 2 unpushed commits (run 'gfv backup' to push)
```

## Implementation Notes

### For v0.2.0:
- [ ] Add `--no-push` flag to `add` command
- [ ] Add `--no-push` flag to `remove` command
- [ ] Update documentation for cross-device workflows
- [ ] Add conflict detection/warning to `add` command

### For v0.3.0:
- [ ] Implement `gfv diff` command
- [ ] Improve `restore` with selective restore
- [ ] Add `auto_push` configuration option
- [ ] Show unpushed commits in `status`

### For v1.0.0:
- [ ] Batch operations support
- [ ] Interactive conflict resolution
- [ ] Multi-vault support

## Cross-Device Workflow Documentation

Document the recommended workflow for using gfv across multiple machines:

**Initial setup on Machine A**:
```bash
gfv init --remote https://github.com/user/vault
gfv add ~/.zshrc
gfv add ~/.gitconfig
# Files automatically pushed
```

**Setup on Machine B** (new workflow with --no-push):
```bash
gfv init --remote https://github.com/user/vault  # Clones repo
gfv restore ~/.zshrc    # Get remote version first
gfv restore ~/.gitconfig

# If local files differ from remote:
gfv diff ~/.zshrc       # Compare differences
gfv add ~/.zshrc --no-push  # Add local version without pushing
# ... review all differences
gfv backup              # Push all changes when ready
```

**Ongoing sync**:
```bash
# Machine A: Make changes
vim ~/.zshrc
gfv backup              # Push changes

# Machine B: Get changes
gfv restore ~/.zshrc    # Pull and restore latest version
```
