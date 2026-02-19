#!/bin/bash
#
# Claude Code Plugin Manager Installer for Dev Containers
# Manages built-in Claude Code plugins at project scope
#

set -e

# Get the feature directory
FEATURE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Try to source plugins.sh from multiple possible locations
# Priority:
# 1. From the feature directory itself (self-contained)
# 2. From the isolde repository (development)
# 3. From the devcontainer context (production - copied to project)
PLUGINS_LIB=""
if [ -f "$FEATURE_DIR/lib/plugins.sh" ]; then
    # Self-contained: plugin-manager/lib/plugins.sh
    PLUGINS_LIB="$FEATURE_DIR/lib/plugins.sh"
elif [ -f "$FEATURE_DIR/../../../scripts/lib/plugins.sh" ]; then
    # Development: scripts/lib/plugins.sh
    PLUGINS_LIB="$FEATURE_DIR/../../../scripts/lib/plugins.sh"
elif [ -f "$FEATURE_DIR/../../../../scripts/lib/plugins.sh" ]; then
    # Production: copied to .devcontainer/features/plugin-manager/
    PLUGINS_LIB="$FEATURE_DIR/../../../../scripts/lib/plugins.sh"
fi

if [ -z "$PLUGINS_LIB" ] || [ ! -f "$PLUGINS_LIB" ]; then
    echo "[ERROR] Cannot find plugins.sh library"
    echo "[ERROR] FEATURE_DIR: $FEATURE_DIR"
    exit 1
fi

# Source the library
source "$PLUGINS_LIB"

# Check if running as root or with sudo
if [ "$(id -u)" -eq 0 ]; then
    # Running as root, install for user from host
    if [ -n "$USERNAME" ]; then
        TARGET_USER="$USERNAME"
    elif [ -n "$_REMOTE_USER" ]; then
        TARGET_USER="$_REMOTE_USER"
    elif [ -n "$_DEV_CONTAINERS_IMAGE_USER" ]; then
        TARGET_USER="$_DEV_CONTAINERS_IMAGE_USER"
    else
        # Fallback: find first user with UID >= 1000
        TARGET_USER=$(getent passwd | awk -F: '$3 >= 1000 {print $1; exit}')
        if [ -z "$TARGET_USER" ]; then
            TARGET_USER="user"
        fi
    fi

    # Get home directory for target user
    TARGET_HOME=$(getent passwd "$TARGET_USER" | cut -d: -f6)
else
    # Running as non-root
    TARGET_USER="$(whoami)"
    TARGET_HOME="$HOME"
fi

log_info "Plugin Manager - configuring for user: $TARGET_USER"

# Verify Claude Code is installed
CLAUDE_BIN="$TARGET_HOME/.local/bin/claude"
if [ ! -f "$CLAUDE_BIN" ]; then
    # Check if claude is in PATH
    if ! command -v claude &>/dev/null; then
        log_error "Claude Code CLI not found."
        log_error "Please ensure the claude-code feature is installed first."
        log_error "The plugin-manager feature requires Claude Code CLI."
        exit 1
    fi
    CLAUDE_BIN="$(command -v claude)"
fi

log_info "Claude Code CLI found at: $CLAUDE_BIN"

# Read options from devcontainer
# Devcontainers converts option names to uppercase
ACTIVATE_PLUGINS_OPTION="${ACTIVATE_PLUGINS:-[]}"
DEACTIVATE_PLUGINS_OPTION="${DEACTIVATE_PLUGINS:-[]}"

log_info "Reading plugin configuration..."
log_info "  Activate: $ACTIVATE_PLUGINS_OPTION"
log_info "  Deactivate: $DEACTIVATE_PLUGINS_OPTION"

# Parse plugin lists
ACTIVATE_LIST=$(parse_activate_plugins "$ACTIVATE_PLUGINS_OPTION")
DEACTIVATE_LIST=$(parse_deactivate_plugins "$DEACTIVATE_PLUGINS_OPTION")

# Find project directory
# Feature is installed at <project>/.devcontainer/features/plugin-manager/
# Project root is two levels up from feature directory
PROJECT_DIR="$(cd "$FEATURE_DIR/../.." && pwd)"

log_info "Project directory: $PROJECT_DIR"

# Build enabledPlugins object
log_info "Building plugin configuration..."
ENABLED_PLUGINS=$(build_enabled_plugins "$TARGET_HOME" "$ACTIVATE_LIST" "$DEACTIVATE_LIST")

# Write to project .claude/settings.json
log_info "Writing .claude/settings.json..."
write_claude_settings "$PROJECT_DIR" "$ENABLED_PLUGINS"

# Fix ownership if running as root
if [ "$(id -u)" -eq 0 ]; then
    chown -R "$TARGET_USER:$TARGET_USER" "$PROJECT_DIR/.claude" 2>/dev/null || true
fi

log_info "Plugin Manager configuration complete!"
log_info "Plugins configured in: $PROJECT_DIR/.claude/settings.json"

# Show summary
if [ -n "$ACTIVATE_LIST" ]; then
    log_info "Activated plugins:"
    echo "$ACTIVATE_LIST" | while IFS= read -r plugin; do
        [ -n "$plugin" ] && echo "  + $plugin"
    done
fi

if [ -n "$DEACTIVATE_LIST" ]; then
    log_info "Deactivated plugins:"
    echo "$DEACTIVATE_LIST" | while IFS= read -r plugin; do
        [ -n "$plugin" ] && echo "  - $plugin"
    done
fi
