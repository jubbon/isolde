# Claude Code Version Control

## Overview
The claude-code devcontainer feature supports version pinning to control which Claude Code CLI version is installed in your development environment.

## Options

| Version | Behavior | Auto-updates |
|---------|-----------|--------------|
| `latest` (default) | Most recent release | Enabled |
| `stable` | Latest stable release | Disabled |
| `1.2.41` | Specific version | Disabled |

## Configuration

In `.devcontainer/devcontainer.json`, specify the version for the claude-code feature:

```json
"./features/claude-code": {
  "version": "1.2.41"  // or "stable"
}
```

## Auto-Update Behavior

- **latest**: Claude Code can update itself (default behavior)
- **stable/X.Y.Z**: Auto-updates disabled via `DISABLE_AUTOUPDATER` environment variable
  - Written to project-local `.claude/settings.local.json`
  - Prevents unexpected version changes in devcontainer builds

## Implementation Details

### Version Passing

The version is passed to the upstream Claude install script:
```bash
curl -fsSL https://claude.ai/install.sh | bash -s -- "$VERSION_OPTION"
```

### Settings File Path

The `DISABLE_AUTOUPDATER` setting is written to:
```
/workspace/<project-name>/.claude/settings.local.json
```

The path is calculated dynamically by searching upward from the feature directory:
```bash
# Find .claude directory by searching upward from feature directory
FEATURE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_CLAUDE_SETTINGS="$(find "$FEATURE_DIR/../.." -maxdepth 2 -name ".claude" -type d | head -1)/settings.local.json"
```

This ensures the path works correctly whether:
- Installing from main project (`.devcontainer/features/claude-code/install.sh`)
- Installing from a worktree (`.worktrees/xxx/.devcontainer/features/claude-code/install.sh`)

### Merge Strategy

If `jq` is available, it's used for proper JSON merging:
```bash
jq --arg key "DISABLE_AUTOUPDATER" --arg value "1" '
    if .env == null then .env = {} end;
    .env[$key] = $value
' "$settings_local"
```

If `jq` is not available, a `sed` fallback is used with caution to avoid corrupting JSON.

## Usage Examples

### Pin to Stable Version

```json
{
  "features": {
    "claude-code": {
      "version": "stable"
    }
  }
}
```

Result: Settings file created at `.claude/settings.local.json` with `DISABLE_AUTOUPDATER="1"`

### Pin to Specific Version

```json
{
  "features": {
    "claude-code": {
      "version": "1.2.41"
    }
  }
}
```

Result: Settings file created/updated at `.claude/settings.local.json` with `DISABLE_AUTOUPDATER="1"`

### Use Latest (Default)

```json
{
  "features": {
    "claude-code": {
      // "version": "latest"  // or omit entirely
    }
  }
}
```

Result: No `DISABLE_AUTOUPDATER` setting added (auto-updates remain enabled)

## Troubleshooting

### Auto-updates Still Enabled

If you pinned a version but Claude Code still auto-updates, check:

1. **Verify settings.local.json exists** (in project root):
   ```bash
   cat .claude/settings.local.json | jq '.env.DISABLE_AUTOUPDATER'
   ```
   Expected output: `true`

2. **Rebuild devcontainer** to ensure changes take effect

3. **Check install logs** for version confirmation

### Version Not Working

If the specified version isn't installed:

1. **Check install script output** for version confirmation
2. **Verify upstream install script** accepts the version format
3. **Report an issue** at https://github.com/anthropics/claude-code

### See Also

- [CLAUDE.md](../CLAUDE.md) - Main project documentation
- [.devcontainer/features/claude-code/devcontainer-feature.json](.devcontainer/features/claude-code/devcontainer-feature.json) - Feature definition
- [.devcontainer/features/claude-code/install.sh](.devcontainer/features/claude-code/install.sh) - Installation script
