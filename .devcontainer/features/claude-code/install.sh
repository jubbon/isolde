#!/bin/bash
#
# Claude Code CLI Installer for Dev Containers
# This script installs the Anthropic Claude Code CLI
#

set -e

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

# Check if running as root or with sudo
if [ "$(id -u)" -eq 0 ]; then
    # Running as root, install for the vscode user
    TARGET_USER="vscode"
    TARGET_HOME="/home/vscode"

    if ! id "$TARGET_USER" &>/dev/null; then
        log_error "User '$TARGET_USER' does not exist. Creating..."
        useradd --create-home --shell /bin/bash "$TARGET_USER" || true
    fi

    log_info "Installing Claude Code CLI for user: $TARGET_USER"

    # Download and install as the target user
    su - "$TARGET_USER" -c 'curl -fsSL https://claude.ai/install.sh | bash'
else
    # Running as non-root, install for current user
    log_info "Installing Claude Code CLI for current user: $(whoami)"

    # Check if proxy is set
    if [ -n "$HTTP_PROXY" ] || [ -n "$HTTPS_PROXY" ]; then
        log_info "Proxy detected - installation will use proxy settings"
    fi

    # Download and install Claude Code CLI
    curl -fsSL https://claude.ai/install.sh | bash
fi

log_info "Claude Code CLI installation completed!"
