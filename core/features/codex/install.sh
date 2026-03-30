#!/usr/bin/env bash
#
# Codex CLI Installer for Dev Containers
# This script installs OpenAI Codex CLI
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Version option from feature (default: latest)
VERSION_OPTION="${VERSION:-latest}"

# Check if running as root or with sudo
if [ "$(id -u)" -eq 0 ]; then
    # Running as root, install for user from host
    # Use USERNAME (from devcontainer build arg) or _REMOTE_USER or default to first non-root user
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

# Proxy variables for Codex CLI installation
# Priority: 1) Shared state file (preferred)  2) Direct options  3) Global ENV
# NOTE: Must read state file AFTER determining TARGET_USER and TARGET_HOME
STATE_FILE="$TARGET_HOME/.config/devcontainer/proxy"
if [ -f "$STATE_FILE" ]; then
    # shellcheck source=/dev/null
    source "$STATE_FILE"
    FEATURE_HTTP_PROXY="$HTTP_PROXY"
    FEATURE_HTTPS_PROXY="$HTTPS_PROXY"
    log_info "Using proxy from shared state file"
elif [ -n "$OPTION_HTTP_PROXY" ] || [ -n "$OPTION_HTTPS_PROXY" ]; then
    # Use direct options for build-time downloads (Docker build phase)
    FEATURE_HTTP_PROXY="$OPTION_HTTP_PROXY"
    FEATURE_HTTPS_PROXY="$OPTION_HTTPS_PROXY"
    log_info "Using proxy from direct options (build-time)"
else
    log_info "Proxy state file not found at $STATE_FILE, using global ENV"
    FEATURE_HTTP_PROXY="${HTTP_PROXY:-}"
    FEATURE_HTTPS_PROXY="${HTTPS_PROXY:-}"
fi

log_info "Codex CLI version: $VERSION_OPTION"

# Check if proxy is set
if [ -n "$FEATURE_HTTP_PROXY" ] || [ -n "$FEATURE_HTTPS_PROXY" ]; then
    log_info "Proxy detected for Codex CLI installation - HTTP_PROXY=$FEATURE_HTTP_PROXY, HTTPS_PROXY=$FEATURE_HTTPS_PROXY"
fi

# Check that Node.js and npm are available
if ! command -v npm >/dev/null 2>&1; then
    log_error "npm is not available. Node.js is required to install Codex CLI."
    log_error "Add the Node.js feature (ghcr.io/devcontainers/features/node) to your devcontainer.json."
    exit 1
fi

log_info "Using npm $(npm --version) at $(command -v npm)"

# Determine package spec (versioned or latest)
if [ "$VERSION_OPTION" = "latest" ]; then
    PACKAGE_SPEC="@openai/codex"
else
    PACKAGE_SPEC="@openai/codex@${VERSION_OPTION}"
fi

log_info "Installing Codex CLI: $PACKAGE_SPEC"

# Build npm install command with proxy if needed
if [ -n "$FEATURE_HTTPS_PROXY" ]; then
    NPM_PROXY="$FEATURE_HTTPS_PROXY"
elif [ -n "$FEATURE_HTTP_PROXY" ]; then
    NPM_PROXY="$FEATURE_HTTP_PROXY"
else
    NPM_PROXY=""
fi

if [ "$(id -u)" -eq 0 ]; then
    if ! id "$TARGET_USER" &>/dev/null; then
        log_error "User '$TARGET_USER' does not exist."
        exit 1
    fi

    log_info "Installing Codex CLI for user: $TARGET_USER (HOME: $TARGET_HOME)"

    if [ -n "$NPM_PROXY" ]; then
        log_info "Using proxy: $NPM_PROXY"
        su - "$TARGET_USER" -c "npm install -g '$PACKAGE_SPEC' --https-proxy '$NPM_PROXY' --proxy '$NPM_PROXY'"
    else
        su - "$TARGET_USER" -c "npm install -g '$PACKAGE_SPEC'"
    fi
else
    log_info "Installing Codex CLI for current user: $(whoami)"

    if [ -n "$NPM_PROXY" ]; then
        log_info "Using proxy: $NPM_PROXY"
        npm install -g "$PACKAGE_SPEC" --https-proxy "$NPM_PROXY" --proxy "$NPM_PROXY"
    else
        npm install -g "$PACKAGE_SPEC"
    fi
fi

log_info "Codex CLI installation completed!"

# Verify installation
if command -v codex >/dev/null 2>&1; then
    log_info "Codex CLI version: $(codex --version)"
else
    log_warn "codex binary not found in PATH after installation. You may need to open a new shell."
fi

log_info "Codex CLI is ready. Set OPENAI_API_KEY in your environment to use it."
