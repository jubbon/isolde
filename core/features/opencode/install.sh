#!/usr/bin/env bash
#
# OpenCode CLI Installer for Dev Containers
# This script installs OpenCode AI coding assistant
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
VERSION="${VERSION:-latest}"
USERNAME="${USERNAME:-"${_REMOTE_USER:-"automatic"}"}"

log_info "Installing OpenCode CLI (version: ${VERSION})..."

# Check that Node.js and npm are available
if ! command -v npm >/dev/null 2>&1; then
    log_error "npm is not available. Node.js is required to install OpenCode CLI."
    log_error "Add the Node.js feature (ghcr.io/devcontainers/features/node) to your devcontainer.json."
    exit 1
fi

log_info "Using npm $(npm --version) at $(command -v npm)"

# Determine package spec (versioned or latest)
if [ "${VERSION}" = "latest" ]; then
    PACKAGE_SPEC="opencode-ai"
else
    PACKAGE_SPEC="opencode-ai@${VERSION}"
fi

log_info "Installing OpenCode CLI: ${PACKAGE_SPEC}"

# Check if running as root or with sudo
if [ "$(id -u)" -eq 0 ]; then
    # Running as root, determine target user
    if [ "${USERNAME}" = "automatic" ]; then
        # Fallback: find first user with UID >= 1000
        TARGET_USER=$(getent passwd | awk -F: '$3 >= 1000 {print $1; exit}')
        if [ -z "${TARGET_USER}" ]; then
            TARGET_USER="user"
        fi
    else
        TARGET_USER="${USERNAME}"
    fi

    if ! id "${TARGET_USER}" &>/dev/null; then
        log_error "User '${TARGET_USER}' does not exist."
        exit 1
    fi

    log_info "Installing OpenCode CLI for user: ${TARGET_USER}"
    su - "${TARGET_USER}" -c "npm install -g '${PACKAGE_SPEC}'"
else
    log_info "Installing OpenCode CLI for current user: $(whoami)"
    npm install -g "${PACKAGE_SPEC}"
fi

# Verify installation
if command -v opencode &>/dev/null; then
    log_info "OpenCode installed successfully: $(opencode --version 2>/dev/null || echo 'version unknown')"
else
    log_warn "opencode binary not found in PATH after installation. You may need to open a new shell."
fi

log_info "OpenCode CLI is ready."
