# Design: Fix ANTHROPIC_* Environment Variables in Devcontainer

**Date:** 2026-02-12
**Status:** Design Complete
**Author:** Claude & dmanakulikov

## Problem Statement

The `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_BASE_URL` environment variables are not being set when starting a new shell inside the devcontainer. This prevents Claude Code CLI from connecting to the LLM provider.

## Root Cause Analysis

1. **Dockerfile switches USER at line 41:** `USER ${USERNAME}`
2. **Dev Containers Features execute AFTER Dockerfile build** — in the context of the switched user
3. **install.sh check fails:** `if [ "$(id -u)" -eq 0 ]` returns FALSE when running as non-root
4. **Result:** `/etc/profile.d/claude-code-path.sh` is never created
5. **Additional issue:** Mounts (including `~/.claude`) are applied only at container STARTUP, not during BUILD

## Solution Architecture

### Key Insight

Store the provider name in a **container-local location** (not in mounted `~/.claude`) so:
- Each devcontainer has its own provider configuration
- No race conditions between multiple containers
- Works regardless of mount timing

### Design Flow

```
┌─────────────────────────────────────────────────────────────────┐
│ Phase 1: Build (install.sh)                            │
│ - Creates ~/.config/devcontainer/provider                      │
│ - Stores the provider value from feature option                │
└─────────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│ Phase 2: First Startup (postCreateCommand)               │
│ - Mounts are now applied                                   │
│ - Updates ~/.bashrc via sed                                 │
│ - Changes: local provider="" →                               │
│   local provider="$(cat ~/.config/devcontainer/provider)"        │
└─────────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│ Phase 3: Every New Shell                                 │
│ - ~/.bashrc sources                                         │
│ - configure_claude_provider() executes                       │
│ - Reads provider from ~/.config/devcontainer/provider            │
│ - Loads credentials from ~/.claude/providers/{provider}/       │
│ - Exports ANTHROPIC_AUTH_TOKEN and ANTHROPIC_BASE_URL     │
└─────────────────────────────────────────────────────────────────┘
```

## Implementation Details

### 1. Changes to `install.sh`

Add after line 97 (after `PROVIDER="${provider:-}"`):

```bash
# Save provider name for postCreateCommand (container-local, not in mounted .claude)
if [ -n "$PROVIDER" ]; then
    mkdir -p "$TARGET_HOME/.config/devcontainer"
    echo "$PROVIDER" > "$TARGET_HOME/.config/devcontainer/provider"
fi
```

### 2. Changes to `devcontainer.json`

Add to the root level:

```json
"postCreateCommand": "bash -c '[ -f ~/.config/devcontainer/provider ] && sed -i \"s/local provider=\\\"\\\"/local provider=\\\"\\\$(cat ~/.config/devcontainer/provider)\\\"/\" ~/.bashrc'"
```

### 3. Updated `configure_claude_provider()` Function

The function in `~/.bashrc` will be updated by `postCreateCommand` to:

```bash
configure_claude_provider() {
    local provider=""
    local provider_dir="$HOME/.claude/providers/$provider"
    local auth_file="$provider_dir/auth"
    local url_file="$provider_dir/base_url"

    # Read provider from container-local config
    if [ -f ~/.config/devcontainer/provider ]; then
        provider="$(cat ~/.config/devcontainer/provider)"
    fi

    # If no provider - exit without setting variables
    if [ -z "$provider" ]; then
        return 0
    fi

    # Check provider directory exists
    if [ ! -d "$provider_dir" ]; then
        echo "[WARN] Provider directory not found: $provider_dir" >&2
        return 1
    fi

    # Load credentials from provider directory
    if [ -f "$auth_file" ]; then
        export ANTHROPIC_AUTH_TOKEN="$(cat "$auth_file")"
    else
        echo "[WARN] Auth file not found: $auth_file" >&2
        return 1
    fi

    if [ -f "$url_file" ]; then
        export ANTHROPIC_BASE_URL="$(cat "$url_file" | tr -d '\n\r ')"
    fi

    echo "[INFO] Configured provider: $provider"
    return 0
}
```

## Error Handling

| Scenario | Behavior |
|-----------|------------|
| Provider found, files exist | ✅ Variables exported |
| Provider found, auth missing | ❌ Warning, return code 1 |
| Provider NOT specified | ⚪ No variables set |
| Provider directory missing | ❌ Warning, return code 1 |

## Files Modified

| File | Change |
|-------|--------|
| `.devcontainer/features/claude-code/install.sh` | Add 5 lines to create provider file |
| `.devcontainer/devcontainer.json` | Add postCreateCommand |
| `~/.bashrc` | Updated automatically by postCreateCommand |

## Testing Checklist

- [ ] install.sh creates `~/.config/devcontainer/provider`
- [ ] postCreateCommand updates `~/.bashrc` correctly
- [ ] `ANTHROPIC_AUTH_TOKEN` is set in new shell
- [ ] `ANTHROPIC_BASE_URL` is set in new shell
- [ ] Variables are NOT set when provider is empty
- [ ] Works with different providers (z.ai, anthropic, custom)
