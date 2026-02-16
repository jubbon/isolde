#!/bin/bash
#
# Proxy Configuration Installer for Dev Containers
# Creates shared state at ~/.config/devcontainer/proxy for other features
#

set -e

FEATURE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source utility functions
source "$FEATURE_DIR/proxy.sh"

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

# Get options from devcontainers (uppercase conversion)
ENABLED="${ENABLED:-true}"
HTTP_PROXY_VAR="${HTTP_PROXY:-}"
HTTPS_PROXY_VAR="${HTTPS_PROXY:-}"
NO_PROXY_VAR="${NO_PROXY:-localhost,127.0.0.1,.local}"
APT_PROXY="${APT_PROXY:-false}"

log_info "Proxy feature configuration - enabled: $ENABLED"

# Check if enabled
if [ "$ENABLED" != "true" ]; then
    log_info "Proxy feature is disabled"
    exit 0
fi

# Validate proxy URLs
if ! validate_proxy "$HTTP_PROXY_VAR" "http_proxy"; then
    log_error "Invalid http_proxy configuration"
    exit 1
fi

if ! validate_proxy "$HTTPS_PROXY_VAR" "https_proxy"; then
    log_error "Invalid https_proxy configuration"
    exit 1
fi

# Determine final proxy values
FINAL_HTTP_PROXY="$HTTP_PROXY_VAR"
FINAL_HTTPS_PROXY="${HTTPS_PROXY_VAR:-$HTTP_PROXY_VAR}"
FINAL_NO_PROXY="$NO_PROXY_VAR"

# Test proxy connectivity (optional, don't fail)
if [ -n "$FINAL_HTTP_PROXY" ] || [ -n "$FINAL_HTTPS_PROXY" ]; then
    test_proxy_connectivity "$FINAL_HTTP_PROXY" "$FINAL_HTTPS_PROXY"
fi

# Determine target user and home directory
if [ "$(id -u)" -eq 0 ]; then
    if [ -n "$USERNAME" ]; then
        TARGET_USER="$USERNAME"
    elif [ -n "$_REMOTE_USER" ]; then
        TARGET_USER="$_REMOTE_USER"
    elif [ -n "$_DEV_CONTAINERS_IMAGE_USER" ]; then
        TARGET_USER="$_DEV_CONTAINERS_IMAGE_USER"
    else
        TARGET_USER=$(getent passwd | awk -F: '$3 >= 1000 {print $1; exit}')
        if [ -z "$TARGET_USER" ]; then
            TARGET_USER="user"
        fi
    fi
    TARGET_HOME=$(getent passwd "$TARGET_USER" | cut -d: -f6)
else
    TARGET_USER="$(whoami)"
    TARGET_HOME="$HOME"
fi

# Create state directory
STATE_DIR="$TARGET_HOME/.config/devcontainer"
mkdir -p "$STATE_DIR"

# Create state file with proxy configuration
STATE_FILE="$STATE_DIR/proxy"
cat > "$STATE_FILE" << EOF
# DevContainer Proxy Configuration
# Source this file to export proxy variables
HTTP_PROXY='$FINAL_HTTP_PROXY'
HTTPS_PROXY='$FINAL_HTTPS_PROXY'
NO_PROXY='$FINAL_NO_PROXY'
EOF

# Fix ownership
if [ "$(id -u)" -eq 0 ]; then
    chown -R "$TARGET_USER:$TARGET_USER" "$STATE_DIR"
fi

log_info "Created proxy state file: $STATE_FILE"

# Configure apt proxy if requested
if [ "$APT_PROXY" = "true" ] && [ -n "$FINAL_HTTP_PROXY" ]; then
    APT_CONF_DIR="/etc/apt/apt.conf.d"
    APT_CONF_FILE="$APT_CONF_DIR/proxy.conf"

    if [ "$(id -u)" -eq 0 ]; then
        mkdir -p "$APT_CONF_DIR"
        cat > "$APT_CONF_FILE" << EOF
Acquire::http::Proxy "$FINAL_HTTP_PROXY";
EOF
        log_info "Created apt proxy configuration: $APT_CONF_FILE"
    else
        log_warn "Cannot configure apt proxy (not running as root)"
    fi
fi

# Create runtime environment export script for containerEnv
# This allows proxy feature to populate containerEnv automatically
PROFILE_D_DIR="/etc/profile.d"
PROFILE_SCRIPT="$PROFILE_D_DIR/devcontainer-proxy.sh"

if [ "$(id -u)" -eq 0 ] && [ -n "$FINAL_HTTP_PROXY" ]; then
    mkdir -p "$PROFILE_D_DIR"
    cat > "$PROFILE_SCRIPT" << EOF
# DevContainer Proxy Configuration
# Source this file to use proxy in shell sessions
export HTTP_PROXY='$FINAL_HTTP_PROXY'
export HTTPS_PROXY='$FINAL_HTTPS_PROXY'
export NO_PROXY='$FINAL_NO_PROXY'
export http_proxy='$FINAL_HTTP_PROXY'
export https_proxy='$FINAL_HTTPS_PROXY'
EOF
    chmod +x "$PROFILE_SCRIPT"
    log_info "Created runtime proxy export script: $PROFILE_SCRIPT"
fi

# Log summary
if [ -n "$FINAL_HTTP_PROXY" ]; then
    log_info "Proxy configured - HTTP: $FINAL_HTTP_PROXY, HTTPS: $FINAL_HTTPS_PROXY"
    log_info "Other features can source $STATE_FILE to use proxy"
else
    log_info "No proxy configured (empty http_proxy)"
fi
