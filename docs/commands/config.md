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

### Set AI API key
```bash
gfv config ai.api_key sk-xxxxx
```

### Set AI model
```bash
gfv config ai.model gpt-4
```

### Get value
```bash
gfv config ai.api_key
```
Output:
```
sk-xxxxx
```

### List all settings
```bash
gfv config --list
```
Output:
```
ai.provider=openai
ai.api_key=sk-xxxxx
ai.model=gpt-3.5-turbo
sync.conflict_strategy=prompt
```

### Unset a value
```bash
gfv config --unset ai.model
```

## Configuration Keys

### AI Settings

| Key | Description | Default |
|-----|-------------|---------|
| `ai.provider` | AI provider (openai) | `openai` |
| `ai.api_key` | OpenAI API key | (none) |
| `ai.model` | Model name | `gpt-3.5-turbo` |

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
default = "/Users/username/.gfv"
work = "/Users/username/.work-vault"

[current]
active = "default"

[ai]
provider = "openai"
api_key = "sk-xxxxx"
model = "gpt-3.5-turbo"

[sync]
conflict_strategy = "prompt"
```

## Notes

- API keys stored with restricted permissions (0600)
- Environment variable `OPENAI_API_KEY` takes precedence over config file
- You can also manually edit `~/.config/gfv/config.toml`
- Vault-specific settings are in `<vault>/.vault-manifest.json`

## See Also

- [push.md](./push.md) - AI commit messages
- [sync.md](./sync.md) - Conflict resolution
- [OVERVIEW.md](../OVERVIEW.md) - Configuration details
