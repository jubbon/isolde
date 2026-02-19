# Claude Code Plugin Manager

A devcontainer feature that manages Claude Code's built-in plugins at project scope. Each devcontainer can have its own set of active plugins through `.claude/settings.json`.

## Overview

This feature allows you to:
- **Activate** specific plugins for a project
- **Deactivate** globally-installed plugins in a project
- Configure plugins via `devcontainer.json` or presets

## Usage

### In devcontainer.json

```json
{
  "features": {
    "./features/plugin-manager": {
      "activate_plugins": ["superpowers", "tdd", "frontend-design"],
      "deactivate_plugins": ["autopilot", "ralph"]
    }
  }
}
```

### In presets.yaml

```yaml
presets:
  python-ml:
    template: python
    claude_plugins:
      activate:
        - oh-my-claudecode
        - tdd
      deactivate:
        - autopilot
```

## Plugin Naming

Plugins can be specified by short name (e.g., `superpowers`). The feature automatically resolves them to their full identifiers (`name@marketplace`) from Claude Code's plugin registry.

Common marketplace identifiers:
- `superpowers-marketplace` - Superpowers workflow skills
- `omc` - oh-my-claudecode orchestration
- `claude-plugins-official` - Official Anthropic plugins

### Examples

| Short name | Resolves to |
|------------|-------------|
| `superpowers` | `superpowers@superpowers-marketplace` |
| `oh-my-claudecode` | `oh-my-claudecode@omc` |
| `frontend-design` | `frontend-design@claude-plugins-official` |
| `tdd` | `tdd@superpowers-marketplace` |

## How It Works

1. Feature reads `activate_plugins` and `deactivate_plugins` options
2. Discovers all installed plugins from `~/.claude/plugins/installed_plugins.json`
3. Resolves short names to full `name@marketplace` identifiers
4. Writes `.claude/settings.json` with `enabledPlugins` section

## Resulting Configuration

The feature creates/updates `.claude/settings.json`:

```json
{
  "enabledPlugins": {
    "superpowers@superpowers-marketplace": true,
    "tdd@superpowers-marketplace": true,
    "oh-my-claudecode@omc": true,
    "autopilot@omc": false
  }
}
```

## Requirements

- Claude Code CLI must be installed (use `claude-code` feature first)
- `jq` recommended for JSON parsing (fallback to sed/grep available)

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `activate_plugins` | array | `[]` | List of plugins to activate |
| `deactivate_plugins` | array | `[]` | List of plugins to deactivate |

## Priority

When a plugin is in both lists, `activate_plugins` takes priority.

## Error Handling

- Unknown plugins → Warning, continues execution
- Claude Code not installed → Error, exits with code 1
- Invalid JSON → Error, exits with code 1

## Examples

### Activate TDD workflow plugins

```json
{
  "features": {
    "./features/plugin-manager": {
      "activate_plugins": ["tdd", "test-driven-development"]
    }
  }
}
```

### Disable heavy orchestration modes

```json
{
  "features": {
    "./features/plugin-manager": {
      "deactivate_plugins": ["autopilot", "ralph", "ultrawork"]
    }
  }
}
```

### Minimal setup - only essential plugins

```json
{
  "features": {
    "./features/plugin-manager": {
      "activate_plugins": ["superpowers"],
      "deactivate_plugins": ["autopilot", "ralph-loop", "ultraqa"]
    }
  }
}
```
