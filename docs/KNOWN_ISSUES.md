# Known Issues

This document lists known limitations and issues with git-file-vault.

## SSH Authentication Issues

**Issue:** SSH authentication may hang or fail when using SSH URLs (e.g., `git@github.com:user/repo.git`)

**Symptoms:**
- Commands hang indefinitely when performing remote operations (clone, fetch, pull, push)
- Commands may display authentication or connection errors

**Affected Commands:**
- `gfv init --remote git@...`
- `gfv vault create --remote git@...`
- `gfv backup` (when pushing to SSH remote)
- `gfv restore` (when pulling from SSH remote)

**Possible Causes:**
- SSH agent not running or not configured correctly
- SSH keys not loaded in the agent
- SSH host key verification prompts waiting for user input
- Passphrase-protected SSH keys without proper agent setup
- Network or firewall issues blocking SSH connections

**Workaround:**
Use HTTPS URLs instead of SSH URLs for remote repositories:

```bash
# Instead of SSH:
gfv init --remote git@github.com:user/repo.git

# Use HTTPS:
gfv init --remote https://github.com/user/repo.git
```

**Notes:**
- HTTPS authentication is generally more reliable in automated tools
- You may need to configure Git credential helper for HTTPS authentication
- For private repositories, use personal access tokens with HTTPS

**Setup Git credential helper:**
```bash
# macOS (uses Keychain)
git config --global credential.helper osxkeychain

# Linux
git config --global credential.helper cache

# Windows
git config --global credential.helper wincred
```

**Alternative: Fix SSH Setup**
If you prefer to use SSH, ensure:
1. SSH agent is running: `eval "$(ssh-agent -s)"`
2. Your SSH key is added: `ssh-add ~/.ssh/id_rsa`
3. Test SSH connection: `ssh -T git@github.com`

**Status:**
This is a known limitation when using SSH-based Git authentication. HTTPS is the recommended approach for reliability.
