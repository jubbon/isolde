# Claude Code Version Control Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Allow specifying Claude Code version in devcontainer feature with auto-update disabled for pinned versions.

**Architecture:** The feature already has a `version` option (unused). We'll: (1) pass version to Claude install script, (2) disable auto-updates for non-latest versions by merging into project-local `.claude/settings.local.json`.

**Tech Stack:** Bash shell scripting, JSON manipulation (jq or fallback), devcontainer feature API

---

### Task 1: Add version option capture to install.sh

**Files:**
- Modify: `.devcontainer/features/claude-code/install.sh`

**Step 1: Capture VERSION option**

Devcontainers converts option names to uppercase. Add after line 36 (after proxy export):

```bash
# Version option from feature (default: latest)
VERSION_OPTION="${VERSION:-latest}"
log_info "Claude Code version: $VERSION_OPTION"
```

**Step 2: Verify variable is set**

Run: `cd .devcontainer/features/claude-code && bash -c 'source install.sh; echo "VERSION_OPTION=$VERSION_OPTION"'`
Expected: `VERSION_OPTION=latest`

**Step 3: Commit**

```bash
git add .devcontainer/features/claude-code/install.sh
git commit -m "feat: capture version option in install.sh"
```

---

### Task 2: Pass version to Claude install script

**Files:**
- Modify: `.devcontainer/features/claude-code/install.sh:74-85`

**Step 1: Modify install command to pass version**

Find the two places where `curl ... | bash` is called. Pass version as argument.

Root install (line ~76-82), change:
```bash
# FROM:
su - "$TARGET_USER" -c 'curl -vL --http1.1 --tlsv1.2 https://claude.ai/install.sh | bash'

# TO:
su - "$TARGET_USER" -c "curl -vL --http1.1 --tlsv1.2 https://claude.ai/install.sh | bash -s -- $VERSION_OPTION"
```

Non-root install (line ~91), change:
```bash
# FROM:
curl -fsSL https://claude.ai/install.sh | bash

# TO:
curl -fsSL https://claude.ai/install.sh | bash -s -- "$VERSION_OPTION"
```

**Step 2: Verify syntax**

Run: `bash -n .devcontainer/features/claude-code/install.sh`
Expected: No syntax errors

**Step 3: Commit**

```bash
git add .devcontainer/features/claude-code/install.sh
git commit -m "feat: pass version to Claude install script"
```

---

### Task 3: Add auto-update disable logic

**Files:**
- Modify: `.devcontainer/features/claude-code/install.sh`

**Step 1: Add function to merge settings.local.json**

Add after line 25 (after log_error function):

```bash
# Merge DISABLE_AUTOUPDATER into settings.local.json
configure_auto_update() {
    local settings_local="$1"
    local disabled="$2"

    if [ "$disabled" != "true" ]; then
        return 0
    fi

    mkdir -p "$(dirname "$settings_local")"

    if [ ! -f "$settings_local" ]; then
        # Create new file with env section
        echo '{"env": {"DISABLE_AUTOUPDATER": "1"}}' > "$settings_local"
        log_info "Created settings.local.json with auto-updates disabled"
    else
        # Merge with existing file
        if command -v jq >/dev/null 2>&1; then
            # Use jq for proper merge
            local tmp_file=$(mktemp)
            jq --arg key "DISABLE_AUTOUPDATER" --arg value "1" '
                if .env == null then .env = {} end;
                .env[$key] = $value
            ' "$settings_local" > "$tmp_file" && mv "$tmp_file" "$settings_local"
            log_info "Merged DISABLE_AUTOUPDATER into existing settings.local.json"
        else
            # Fallback: append using sed (may not be perfect JSON)
            if ! grep -q '"DISABLE_AUTOUPDATER"' "$settings_local"; then
                if grep -q '"env"' "$settings_local"; then
                    # env section exists, add to it
                    sed -i 's/"env": { *"env": { "DISABLE_AUTOUPDATER": "1",/' "$settings_local"
                else
                    # no env section, prepend before closing brace
                    sed -i 's/}/\n  "env": {\n    "DISABLE_AUTOUPDATER": "1"\n  }/' "$settings_local"
                fi
                log_info "Added DISABLE_AUTOUPDATER to settings.local.json (fallback merge)"
            fi
        fi
    fi
}
```

**Step 2: Verify function syntax**

Run: `bash -c 'source .devcontainer/features/claude-code/install.sh; type configure_auto_update'`
Expected: Function is defined

**Step 3: Commit**

```bash
git add .devcontainer/features/claude-code/install.sh
git commit -m "feat: add auto-update configuration function"
```

---

### Task 4: Call auto-update disable after installation

**Files:**
- Modify: `.devcontainer/features/claude-code/install.sh`

**Step 1: Add call after Claude installation**

After line 93 (after "installation completed" message), add:

```bash
# Disable auto-updates for pinned versions (not latest)
PROJECT_CLAUDE_SETTINGS="$(dirname "${BASH_SOURCE[0]}")/../../../.claude/settings.local.json"

if [ "$VERSION_OPTION" != "latest" ]; then
    configure_auto_update "$PROJECT_CLAUDE_SETTINGS" "true"
    log_info "Auto-updates disabled for pinned version: $VERSION_OPTION"
else
    log_info "Auto-updates enabled for latest version"
fi
```

**Step 2: Verify relative path**

Run: `cd .devcontainer/features/claude-code && ls -la ../../../.claude/settings.local.json`
Expected: File exists

**Step 3: Commit**

```bash
git add .devcontainer/features/claude-code/install.sh
git commit -m "feat: disable auto-updates for pinned versions"
```

---

### Task 5: Update feature documentation

**Files:**
- Modify: `.devcontainer/features/claude-code/devcontainer-feature.json`

**Step 1: Update version option description**

Change the version option description (line 8-12):

```json
"version": {
  "type": "string",
  "default": "latest",
  "description": "Claude Code version: 'latest' (default, auto-updates enabled), 'stable', or specific version like '1.2.41' (auto-updates disabled)"
}
```

**Step 2: Bump feature version**

Change line 3:

```json
"version": "1.2.0",
```

**Step 3: Verify JSON syntax**

Run: `jq . .devcontainer/features/claude-code/devcontainer-feature.json`
Expected: Valid JSON output

**Step 4: Commit**

```bash
git add .devcontainer/features/claude-code/devcontainer-feature.json
git commit -m "docs: update version option description and bump to 1.2.0"
```

---

### Task 6: Test the implementation

**Files:**
- Test: Manual verification in devcontainer

**Step 1: Build devcontainer with pinned version**

Run:
```bash
cd .devcontainer && docker build -t claude-code-dev .
```

**Step 2: Verify settings.local.json was created**

In the built image, check:
```bash
docker run --rm claude-code-dev cat /workspace/.claude/settings.local.json
```

Expected: Contains `"DISABLE_AUTOUPDATER": "1"`

**Step 3: Test with latest version**

Update `devcontainer.json` to use `"version": "latest"` and rebuild.

Verify `settings.local.json` does NOT contain DISABLE_AUTOUPDATER.

**Step 4: Commit test results**

```bash
# If all tests pass
git commit --allow-empty -m "test: verify version control and auto-update behavior"
```

---

### Task 7: Update project documentation

**Files:**
- Modify: `CLAUDE.md`
- Create: `docs/claude-version-control.md`

**Step 1: Update CLAUDE.md**

Add to "Common Commands" section:

```markdown
### Version Control
# Use specific Claude version (disables auto-updates)
# In devcontainer.json: "version": "stable" or "version": "1.2.41"

# Use latest (default, auto-updates enabled)
# In devcontainer.json: omit version or use "version": "latest"
```

**Step 2: Create feature documentation**

Create `docs/claude-version-control.md`:

```markdown
# Claude Code Version Control

## Overview
The claude-code feature supports version pinning to control which Claude Code CLI version is installed.

## Options

| Version | Behavior | Auto-updates |
|---------|-----------|--------------|
| `latest` (default) | Most recent release | Enabled |
| `stable` | Latest stable release | Disabled |
| `1.2.41` | Specific version | Disabled |

## Configuration

In `.devcontainer/devcontainer.json`:

```json
"./features/claude-code": {
  "version": "1.2.41"  // or "stable"
}
```

## Auto-Update Behavior

- **latest**: Claude can update itself (default behavior)
- **stable/X.Y.Z**: Auto-updates disabled via `DISABLE_AUTOUPDATER=1`
  - Written to project-local `.claude/settings.local.json`
  - Prevents unexpected version changes in devcontainer

## Implementation Details

- Version passed to install script: `curl ... | bash -s -- $VERSION`
- Relative path: `../../../.claude/settings.local.json` from feature directory
- Merge strategy: jq if available, sed fallback
```

**Step 3: Commit**

```bash
git add CLAUDE.md docs/claude-version-control.md
git commit -m "docs: add version control documentation"
```

---

## Summary

This plan implements Claude Code version pinning with:
- Version option captured and passed to install script
- Auto-updates disabled for non-latest versions
- Project-local settings (not shared between containers)
- Comprehensive documentation

Total: 7 tasks, ~15-20 minutes implementation time.
