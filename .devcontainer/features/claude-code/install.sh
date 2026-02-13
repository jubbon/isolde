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

# Provider configuration (runs for both root and non-root)
# Devcontainers converts option names to uppercase, so 'provider' becomes 'PROVIDER'
log_info "DEBUG: provider option = '${PROVIDER:-NOT_SET}' (was: ${provider:-NOT_SET})"

# Try both PROVIDER (devcontainers standard) and provider (fallback)
if [ -n "${PROVIDER:-}" ]; then
    FINAL_PROVIDER="$PROVIDER"
elif [ -n "${provider:-}" ]; then
    FINAL_PROVIDER="$provider"
else
    FINAL_PROVIDER=""
fi

# Save provider name for postCreateCommand (container-local, not in mounted .claude)
if [ -n "$FINAL_PROVIDER" ]; then
    # Ensure .config exists with proper permissions
    if [ ! -d "$TARGET_HOME/.config" ]; then
        mkdir -p "$TARGET_HOME/.config"
        chown -R "$TARGET_USER:$TARGET_USER" "$TARGET_HOME/.config"
    fi
    mkdir -p "$TARGET_HOME/.config/devcontainer"
    echo "$FINAL_PROVIDER" > "$TARGET_HOME/.config/devcontainer/provider"
    chown -R "$TARGET_USER:$TARGET_USER" "$TARGET_HOME/.config/devcontainer"
    log_info "Fixed ownership for $TARGET_HOME/.config/devcontainer"
    log_info "Saved provider config: $FINAL_PROVIDER → $TARGET_HOME/.config/devcontainer/provider"
fi

# Save model configurations if they are set
if [ -n "${DEFAULT_HAIKU_MODEL:-}" ] || [ -n "${DEFAULT_SONNET_MODEL:-}" ] || [ -n "${DEFAULT_OPUS_MODEL:-}" ]; then
    if [ ! -d "$TARGET_HOME/.config" ]; then
        mkdir -p "$TARGET_HOME/.config"
        chown -R "$TARGET_USER:$TARGET_USER" "$TARGET_HOME/.config"
    fi
    mkdir -p "$TARGET_HOME/.config/devcontainer"

    # Create or update model config file
    if [ -f "$TARGET_HOME/.config/devcontainer/models" ]; then
        log_info "Updating existing model configuration"
    else
        log_info "Creating model configuration"
    fi

    # Save model configurations
    cat > "$TARGET_HOME/.config/devcontainer/models" << EOF
# Claude Code Model Configuration
ANTHROPIC_DEFAULT_HAIKU_MODEL="${DEFAULT_HAIKU_MODEL:-}"
ANTHROPIC_DEFAULT_SONNET_MODEL="${DEFAULT_SONNET_MODEL:-}"
ANTHROPIC_DEFAULT_OPUS_MODEL="${DEFAULT_OPUS_MODEL:-}"
EOF

    chown -R "$TARGET_USER:$TARGET_USER" "$TARGET_HOME/.config/devcontainer"
    log_info "Fixed ownership for model configuration"
    if [ -n "${DEFAULT_HAIKU_MODEL:-}" ]; then
        log_info "Saved Haiku model: ${DEFAULT_HAIKU_MODEL}"
    fi
    if [ -n "${DEFAULT_SONNET_MODEL:-}" ]; then
        log_info "Saved Sonnet model: ${DEFAULT_SONNET_MODEL}"
    fi
    if [ -n "${DEFAULT_OPUS_MODEL:-}" ]; then
        log_info "Saved Opus model: ${DEFAULT_OPUS_MODEL}"
    fi
fi

# Configure PATH and provider for target user
# Write to ~/.bashrc instead of /etc/profile.d (VS Code uses non-login shell)
# IMPORTANT: Insert at BEGINNING of .bashrc (before interactive check) to ensure vars are always available
if [ "$(id -u)" -eq 0 ]; then
    BASHRC_FILE="$TARGET_HOME/.bashrc"

    # Remove old configuration if exists
    sed -i '/# Claude Code CLI - START/,/# Claude Code CLI - END/d' "$BASHRC_FILE" 2>/dev/null || true

    # Create temp file with new configuration
    TEMP_RC=$(mktemp)
    cat > "$TEMP_RC" << 'EOF'

# Claude Code CLI - START
export PATH="$HOME/.local/bin:$PATH"

configure_claude_provider() {
    local provider=$1
    local provider_dir="$HOME/.claude/providers/$provider"

    if [ -z "$provider" ]; then
        # Default: use Anthropic defaults
        if [ -f "$HOME/.claude/auth" ]; then
            export ANTHROPIC_AUTH_TOKEN="$(cat "$HOME/.claude/auth")"
        fi
        return 0
    fi

    # Load from provider directory
    if [ -d "$provider_dir" ]; then
        if [ -f "$provider_dir/auth" ]; then
            export ANTHROPIC_AUTH_TOKEN="$(cat "$provider_dir/auth")"
        fi
        if [ -f "$provider_dir/base_url" ]; then
            export ANTHROPIC_BASE_URL="$(cat "$provider_dir/base_url" | tr -d '\n\r ')"
        fi
    fi
}
llm_provider=$(cat "$HOME/.config/devcontainer/provider")
configure_claude_provider $llm_provider

# Load model configurations if they exist
if [ -f "$HOME/.config/devcontainer/models" ]; then
    source "$HOME/.config/devcontainer/models"
fi
# Claude Code CLI - END
EOF

    # Append original bashrc content to temp file, then move over original
    cat "$BASHRC_FILE" >> "$TEMP_RC"
    mv "$TEMP_RC" "$BASHRC_FILE"
    chown "$TARGET_USER:$TARGET_USER" "$BASHRC_FILE"

    if [ -n "$FINAL_PROVIDER" ]; then
        log_info "Configured LLM provider: $FINAL_PROVIDER (reads from ~/.claude/providers/$FINAL_PROVIDER/)"
    else
        log_info "Using default Anthropic provider"
    fi
    log_info "Added ~/.local/bin to PATH in ~/.bashrc"
fi
