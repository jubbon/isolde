#!/bin/bash
#
# Claude Code CLI Installer for Dev Containers
# This script installs Anthropic Claude Code CLI
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

# Proxy variables from feature options (for Claude Code installation ONLY)
# These are explicitly passed to the feature, not inherited from global ENV
FEATURE_HTTP_PROXY="${http_proxy:-$HTTP_PROXY}"
FEATURE_HTTPS_PROXY="${https_proxy:-$HTTPS_PROXY}"

# Export proxy variables for curl (only for Claude Code installation)
export http_proxy="$FEATURE_HTTP_PROXY"
export https_proxy="$FEATURE_HTTPS_PROXY"
export HTTP_PROXY="$FEATURE_HTTP_PROXY"
export HTTPS_PROXY="$FEATURE_HTTPS_PROXY"

# Check if proxy is set
if [ -n "$FEATURE_HTTP_PROXY" ] || [ -n "$FEATURE_HTTPS_PROXY" ]; then
    log_info "Proxy detected for Claude Code installation - HTTP_PROXY=$FEATURE_HTTP_PROXY, HTTPS_PROXY=$FEATURE_HTTPS_PROXY"
fi

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

    if ! id "$TARGET_USER" &>/dev/null; then
        log_error "User '$TARGET_USER' does not exist. Creating..."
        useradd --create-home --shell /bin/bash "$TARGET_USER" || true
        TARGET_HOME="/home/$TARGET_USER"
    fi

    log_info "Installing Claude Code CLI for user: $TARGET_USER (HOME: $TARGET_HOME)"

    # Download and install as the target user with proxy environment
    # Pass proxy variables explicitly via env (using feature options)
    if [ -n "$FEATURE_HTTP_PROXY" ] || [ -n "$FEATURE_HTTPS_PROXY" ]; then
        log_info "Using proxy - passing to install command"
        su - "$TARGET_USER" -c "
            export HTTP_PROXY='$FEATURE_HTTP_PROXY'
            export HTTPS_PROXY='$FEATURE_HTTPS_PROXY'
            export http_proxy='$FEATURE_HTTP_PROXY'
            export https_proxy='$FEATURE_HTTPS_PROXY'
            curl -vL --http1.1 --tlsv1.2 https://claude.ai/install.sh | bash
        "
    else
        su - "$TARGET_USER" -c 'curl -vL --http1.1 --tlsv1.2 https://claude.ai/install.sh | bash'
    fi
else
    # Running as non-root, install for current user
    log_info "Installing Claude Code CLI for current user: $(whoami)"

    # Download and install Claude Code CLI (proxy vars already exported)
    curl -fsSL https://claude.ai/install.sh | bash
fi

log_info "Claude Code CLI installation completed!"

# Provider configuration
PROVIDER="${provider:-}"
PROVIDERS_DIR="$HOME/.claude/providers"

# Add ~/.local/bin to PATH for all users
# This ensures claude is available regardless of which user runs the container
if [ "$(id -u)" -eq 0 ]; then
    cat > /etc/profile.d/claude-code-path.sh << 'EOF'
# Add Claude Code CLI to PATH
export PATH="$HOME/.local/bin:$PATH"

# Provider configuration
PROVIDER="__PROVIDER_PLACEHOLDER__"
PROVIDERS_DIR="$HOME/.claude/providers"

configure_provider() {
    local provider="$1"
    local provider_dir="$PROVIDERS_DIR/$provider"
    local auth_file="$provider_dir/auth"
    local url_file="$provider_dir/base_url"

    if [ -z "$provider" ]; then
        # Default: use Anthropic defaults
        if [ -f "$HOME/.claude/auth" ]; then
            export ANTHROPIC_AUTH_TOKEN="$(cat "$HOME/.claude/auth")"
        fi
        return 0
    fi

    # Check if provider directory exists
    if [ ! -d "$provider_dir" ]; then
        echo "[WARN] Provider directory not found: $provider_dir" >&2
        return 1
    fi

    # Load auth token
    if [ -f "$auth_file" ]; then
        export ANTHROPIC_AUTH_TOKEN="$(cat "$auth_file")"
    else
        echo "[WARN] Auth file not found: $auth_file" >&2
        return 1
    fi

    # Load base_url (optional)
    if [ -f "$url_file" ]; then
        export ANTHROPIC_BASE_URL="$(cat "$url_file" | tr -d '\n\r ')"
    fi

    echo "[INFO] Configured provider: $provider"
    return 0
}

# Auto-configure provider on shell startup
configure_provider "$PROVIDER"
EOF

    # Replace placeholder with actual provider value
    sed -i "s|__PROVIDER_PLACEHOLDER__|${PROVIDER}|g" /etc/profile.d/claude-code-path.sh
    chmod +x /etc/profile.d/claude-code-path.sh

    if [ -n "$PROVIDER" ]; then
        log_info "Configured LLM provider: $PROVIDER (reads from $PROVIDERS_DIR/$PROVIDER/)"
    else
        log_info "Using default Anthropic provider"
    fi
    log_info "Added ~/.local/bin to system PATH"
fi
