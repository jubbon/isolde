# Plugin Manager Feature - Design Document

**Date**: 2026-02-17
**Status**: Draft
**Author**: Claude Code + dmanakulikov

## Overview

A standalone devcontainer feature that manages Claude Code's built-in plugins at project scope. It allows each devcontainer to have its own active plugin set via `.claude/settings.json`, with both activation (allowlist) and deactivation (blocklist) capabilities.

## Architecture

```
┌─────────────────────┐
│  devcontainer.json  │
│  - activate_plugins │
│  - deactivate_plugins│
└──────────┬──────────┘
           │
           ▼
┌─────────────────────────────┐
│  plugin-manager/install.sh  │
│  1. Discover installed      │
│  2. Merge user config       │
│  3. Write .claude/settings  │
└──────────┬──────────────────┘
           │
           ▼
┌──────────────────────────────┐
│  <project>/.claude/settings  │
│  {                           │
│    "enabledPlugins": {       │
│      "plugin@market": true,  │
│      "other@market": false   │
│    }                         │
│  }                           │
└──────────────────────────────┘
```

## Configuration Schema

### In devcontainer.json:
```json
{
  "features": {
    "./features/plugin-manager": {
      "activate_plugins": ["superpowers", "oh-my-claudecode", "tdd"],
      "deactivate_plugins": ["autopilot", "ralph"]
    }
  }
}
```

### In presets.yaml:
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

### Generated .claude/settings.json:
```json
{
  "enabledPlugins": {
    "superpowers@superpowers-marketplace": true,
    "oh-my-claudecode@omc": true,
    "tdd@superpowers-marketplace": true,
    "autopilot@omc": false
  }
}
```

## Plugin Naming Scheme

From `installed_plugins.json` research:
- Format: `plugin-name@marketplace`
- Examples:
  - `superpowers@superpowers-marketplace`
  - `oh-my-claudecode@omc`
  - `frontend-design@claude-plugins-official`

## File Structure

```
core/features/plugin-manager/
├── install.sh                    # Main installer script
├── devcontainer-feature.json     # Feature metadata
└── README.md                     # Documentation

scripts/lib/
├── plugins.sh                    # New library for plugin management
│   ├── discover_installed_plugins()   # Get all installed plugins
│   ├── find_plugin_marketplace()      # Find marketplace for a plugin
│   ├── parse_plugin_config()          # Parse JSON arrays from options
│   ├── write_claude_settings()        # Write .claude/settings.json
│   └── merge_plugin_settings()        # Merge with existing settings
└── presets.sh
    └── [updates]                # Parse new claude_plugins format
```

## Installation Script Logic

### 1. Determine Target User
```bash
if [ "$(id -u)" -eq 0 ]; then
    TARGET_USER="${USERNAME:-${_REMOTE_USER:-user}}"
    TARGET_HOME=$(getent passwd "$TARGET_USER" | cut -d: -f6)
else
    TARGET_USER="$(whoami)"
    TARGET_HOME="$HOME"
fi
```

### 2. Parse Options
- `ACTIVATE_PLUGINS` - JSON array of plugins to activate
- `DEACTIVATE_PLUGINS` - JSON array of plugins to deactivate

### 3. Discover Installed Plugins
Read from `~/.claude/plugins/installed_plugins.json`:
```bash
jq -r '.plugins | to_entries[] | "\(.key) => \(.value[0].scope)"' \
    "$TARGET_HOME/.claude/plugins/installed_plugins.json"
```

### 4. Build enabledPlugins Object
- Start with existing project settings (if any)
- Add `activate_plugins` with `true`
- Add `deactivate_plugins` with `false`
- Resolve plugin names to full `name@marketplace` format

### 5. Write Project Settings
Write to `<project>/.claude/settings.json`:
```json
{
  "enabledPlugins": {
    "...": true/false
  }
}
```
Preserve other existing sections (permissions, etc.)

## Error Handling

1. **Claude Code not installed**: Error with clear message
2. **jq not available**: Fallback to sed/grep parsing
3. **Unknown plugins**: Warning, continue execution
4. **Conflict (activate + deactivate)**: activate has priority
5. **Empty arrays**: Valid - means "don't activate/deactivate anything"
6. **Existing .claude/settings.json**: Merge, preserve other settings

## Discovery Algorithm

```bash
# Find marketplace for a plugin name
find_plugin_marketplace() {
    local plugin_name="$1"
    local installed_json="$TARGET_HOME/.claude/plugins/installed_plugins.json"

    # Try exact match first
    jq -r ".plugins | to_entries[] | select(.key | startswith(\"${plugin_name}@\")) | .key" \
        "$installed_json" | head -1

    # If no match, try substring search
    jq -r ".plugins | to_entries[] | select(.key | contains(\"${plugin_name}\")) | .key" \
        "$installed_json" | head -1
}
```

## Preset Integration Changes

### scripts/lib/presets.sh
```bash
# New function to parse claude_plugins
parse_claude_plugins() {
    local preset_name="$1"
    local preset_data=$(get_preset "$preset_name")

    # Extract claude_plugins.activate and .deactivate
    echo "$preset_data" | jq -r '.claude_plugins.activate[]' 2>/dev/null
    echo "$preset_data" | jq -r '.claude_plugins.deactivate[]' 2>/dev/null
}
```

## Template Substitutions

### New placeholders:
- `{{CLAUDE_ACTIVATE_PLUGINS}}` → JSON array of plugins to activate
- `{{CLAUDE_DEACTIVATE_PLUGINS}}` → JSON array of plugins to deactivate

### scripts/lib/templates.sh
```bash
apply_claude_plugins_substitutions() {
    local preset="$1"
    local activate=$(parse_claude_plugins_activate "$preset")
    local deactivate=$(parse_claude_plugins_deactivate "$preset")

    sed -i "s|{{CLAUDE_ACTIVATE_PLUGINS}}|${activate}|g" "$devcontainer_json"
    sed -i "s|{{CLAUDE_DEACTIVATE_PLUGINS}}|${deactivate}|g" "$devcontainer_json"
}
```

## Implementation Tasks

1. **Research** ✅ - Understand Claude Code plugin schema
2. **Create plugins.sh library** - Plugin management functions
3. **Create plugin-manager feature** - install.sh with full logic
4. **Update presets.sh** - Parse new claude_plugins format
5. **Update templates** - Add plugin-manager feature to templates
6. **Update presets.yaml** - Migrate to new format
7. **Documentation** - README and usage docs
8. **Testing** - Verify plugin activation/deactivation

## Open Questions

1. Should the feature support version pinning (e.g., `superpowers@4.2.0`)?
2. Should we auto-resolve marketplace or require full `name@marketplace` format?
3. How to handle plugins that are installed but not in any marketplace?

## References

- Claude Code plugin schema: `~/.claude/settings.json` → `enabledPlugins`
- Plugin registry: `~/.claude/plugins/installed_plugins.json`
- Marketplace locations: `~/.claude/plugins/marketplaces/`
