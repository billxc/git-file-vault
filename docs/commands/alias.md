# `gfv alias` - Manage Command Aliases

Create custom command aliases to simplify frequently used commands.

## Synopsis

```bash
gfv alias add <name> <command...>
gfv alias remove <name>
gfv alias list
```

## Description

The alias system allows you to create custom shortcuts for gfv commands. This is useful for:
- Simplifying long or frequently used commands
- Creating memorable shortcuts (e.g., `use` instead of `vault switch`)
- Customizing your workflow

Aliases are stored in your global config file (`~/.gfv/config.toml`) and work across all vaults.

## Subcommands

### `add` - Create an alias

```bash
gfv alias add <name> <command...>
```

Creates a new alias. The command can be any valid gfv command with arguments.

**Arguments:**
- `<name>` - Alias name (cannot conflict with existing commands)
- `<command...>` - Command to alias (one or more words)

**Examples:**
```bash
# Create 'use' as alias for 'vault switch'
gfv alias add use vault switch

# Create 'ls' as alias for 'list --long'
gfv alias add ls list --long

# Create 'save' as alias for 'backup'
gfv alias add save backup
```

### `remove` - Delete an alias

```bash
gfv alias remove <name>
```

Removes an existing alias.

**Arguments:**
- `<name>` - Alias name to remove

**Examples:**
```bash
gfv alias remove use
gfv alias remove ls
```

### `list` - Show all aliases

```bash
gfv alias list
```

Lists all configured aliases.

## Examples

### Basic Usage

```bash
# Create an alias
gfv alias add use vault switch

# Use the alias
gfv use work      # Equivalent to: gfv vault switch work

# List aliases
gfv alias list

# Remove alias
gfv alias remove use
```

### Common Aliases

```bash
# Shorter command names
gfv alias add sw vault switch
gfv alias add ls list --long
gfv alias add st status

# Convenience aliases
gfv alias add save backup
gfv alias add sync backup
gfv alias add pull restore
gfv alias add push backup
```

### Multi-word Commands

```bash
# Alias for list with options
gfv alias add ll list --long

# Alias for vault operations
gfv alias add new vault create
gfv alias add rm vault remove
```

## Output

### Adding an alias

```
✓ Alias 'use' → 'vault switch' created successfully!

You can now use: gfv use
```

### Overwriting an existing alias

```
Warning: Alias 'use' already exists: vault switch
Do you want to overwrite it? (y/N) y

✓ Alias 'use' → 'vault switch' created successfully!

You can now use: gfv use
```

### Listing aliases

```
Command Aliases:

  ls → list --long
  save → backup
  use → vault switch

3 aliases configured
```

### No aliases configured

```
No aliases configured.

Create an alias with:
  gfv alias add <name> <command>

Example:
  gfv alias add use vault switch
```

### Removing an alias

```
✓ Alias 'use' → 'vault switch' removed
```

## Restrictions

### Reserved Names

You cannot create aliases with these reserved command names:
- `init`
- `link`
- `unlink`
- `list`
- `status`
- `backup`
- `restore`
- `config`
- `alias`
- `vault`
- `debug`

Attempting to use a reserved name will result in:
```
Error: Cannot create alias 'list': this is a reserved command name
```

### Alias Not Found

```
Error: Alias 'unknown' does not exist.

List aliases with: gfv alias list
```

## How Aliases Work

1. **Resolution**: When you run a gfv command, the alias system checks if the first argument matches any configured alias
2. **Expansion**: If a match is found, the alias is expanded to its full command
3. **Execution**: The expanded command is executed as normal

**Example:**
```bash
# You run:
gfv use work

# Alias system expands to:
gfv vault switch work

# Which is then executed
```

## Configuration File

Aliases are stored in `~/.gfv/config.toml`:

```toml
[aliases]
use = "vault switch"
ls = "list --long"
save = "backup"
```

You can also edit this file directly, but using the `gfv alias` commands is recommended.

## Notes

- Aliases are global and work across all vaults
- Aliases can include flags and options
- Aliases cannot chain (an alias cannot reference another alias)
- Alias resolution happens before command parsing
- Use `gfv alias list` to see all configured aliases

## See Also

- [config.md](./config.md) - Global configuration
- [vault.md](./vault.md) - Vault management commands
