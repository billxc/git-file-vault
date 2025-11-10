# `gfv config` - Manage Configuration

Manage global configuration settings.

## Synopsis

```bash
gfv config <key> [value]
gfv config --list
gfv config --unset <key>
```

## Description

Get or set configuration values in `~/.config/gfv/config.toml`.

## Arguments

- `<key>` - Configuration key (dot notation, e.g., `ai.api_key`)
- `[value]` - Value to set (omit to get current value)

## Options

- `--list` / `-l` - List all configuration values
- `--unset` - Remove a configuration key
- `--global` - Alias for default behavior (for consistency)

## Examples

### Configure AI for commit messages
```bash
# Set endpoint (required)
gfv config ai.endpoint https://api.openai.com/v1/chat/completions

# Set API key (required)
gfv config ai.api_key sk-xxxxx

# Set model (required)
gfv config ai.model gpt-4o-mini
```

### Get value
```bash
gfv config ai.endpoint
```
Output:
```
https://api.openai.com/v1/chat/completions
```

```bash
gfv config ai.api_key
```
Output:
```
****** (set)
```

### List all settings
```bash
gfv config --list
```
Output:
```
Vault Configuration:

General:
  vault.dir = /Users/username/.gfv/default
  vault.repo = /Users/username/.gfv/default/repo

Remote:
  remote.url = git@github.com:username/configs.git
  remote.branch = main

AI:
  ai.endpoint = https://api.openai.com/v1/chat/completions
  ai.api_key = ****** (set)
  ai.model = gpt-4o-mini
```

### Unset a value
```bash
gfv config --unset ai.endpoint
gfv config --unset ai.api_key
gfv config --unset ai.model
```

## Configuration Keys

### AI Settings

| Key | Description | Default |
|-----|-------------|---------|
| `ai.endpoint` | OpenAI-compatible API endpoint URL | (none) |
| `ai.api_key` | API key for authentication | (none) |
| `ai.model` | Model name (e.g., gpt-4o-mini, gpt-4, etc.) | (none) |

**Note:** All three AI settings must be configured for AI-generated commit messages to work.

### Sync Settings

| Key | Description | Default |
|-----|-------------|---------|
| `sync.conflict_strategy` | Conflict resolution strategy | `prompt` |

Valid values for `sync.conflict_strategy`:
- `prompt` - Ask user interactively (default)
- `use_vault` - Always prefer vault version
- `use_source` - Always prefer source version

## Configuration File

Location: `~/.config/gfv/config.toml`

Example:
```toml
[vaults]
default = "/Users/username/.gfv/default"
work = "/Users/username/.gfv/work"

[current]
active = "default"

[ai]
endpoint = "https://api.openai.com/v1/chat/completions"
api_key = "sk-xxxxx"
model = "gpt-4o-mini"

[sync]
conflict_strategy = "prompt"
default_branch = "main"

[aliases]
save = "backup"
```

## Notes

- Configuration file is stored at `~/.gfv/config.toml`
- You can also manually edit the config file directly
- AI settings are global and apply to all vaults
- Vault-specific settings (like remote URL) are in `<vault>/.vault-manifest.json`
- API keys are stored in plain text - ensure proper file permissions

## See Also

- [backup.md](./backup.md) - AI commit messages
- [sync.md](./sync.md) - Conflict resolution
- [OVERVIEW.md](../OVERVIEW.md) - Configuration details
