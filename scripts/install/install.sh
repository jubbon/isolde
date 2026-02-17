#!/bin/bash
# install.sh - Install Isolde to ~/.isolde/
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/jubbon/isolde/main/scripts/install/install.sh | bash
#
# Environment variables:
#   ISOLDE_HOME    Installation directory (default: ~/.isolde)
#   ISOLDE_REPO_URL  Repository URL (default: https://github.com/jubbon/isolde.git)
#   ISOLDE_BRANCH    Branch to clone (default: main)

set -e

# Configuration
ISOLDE_HOME="${ISOLDE_HOME:-$HOME/.isolde}"
REPO_URL="${ISOLDE_REPO_URL:-https://github.com/jubbon/isolde.git}"
BRANCH="${ISOLDE_BRANCH:-main}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Clone or update repository
install_or_update() {
    if [ -d "$ISOLDE_HOME/.git" ]; then
        log_info "Updating existing installation..."
        git -C "$ISOLDE_HOME" fetch origin "$BRANCH" 2>/dev/null || true
        git -C "$ISOLDE_HOME" checkout "$BRANCH" 2>/dev/null || true
        git -C "$ISOLDE_HOME" pull origin "$BRANCH" 2>/dev/null || log_warn "Git pull failed, continuing..."
    else
        log_info "Installing Isolde to $ISOLDE_HOME..."
        # Remove existing directory if it exists but isn't a git repo
        if [ -d "$ISOLDE_HOME" ]; then
            log_warn "Removing existing non-git installation directory..."
            rm -rf "$ISOLDE_HOME"
        fi
        git clone --depth 1 --branch "$BRANCH" "$REPO_URL" "$ISOLDE_HOME"
    fi
}

# Create wrapper script
create_wrapper() {
    log_info "Creating wrapper script..."

    # Copy wrapper from install directory
    if [ -f "$ISOLDE_HOME/scripts/install/isolde-wrapper.sh" ]; then
        cp "$ISOLDE_HOME/scripts/install/isolde-wrapper.sh" "$ISOLDE_HOME/isolde"
        chmod +x "$ISOLDE_HOME/isolde"
        log_info "Wrapper script created"
    else
        log_error "Wrapper template not found at $ISOLDE_HOME/scripts/install/isolde-wrapper.sh"
        return 1
    fi
}

# Create VERSION file
create_version_file() {
    log_info "Creating VERSION file..."
    if [ -d "$ISOLDE_HOME/.git" ]; then
        git -C "$ISOLDE_HOME" describe --tags --always 2>/dev/null > "$ISOLDE_HOME/VERSION" \
            || echo "unknown" > "$ISOLDE_HOME/VERSION"
    else
        echo "unknown" > "$ISOLDE_HOME/VERSION"
    fi
}

# Add to PATH
add_to_path() {
    local shell_config=""

    # Detect shell
    if [ -n "$ZSH_VERSION" ]; then
        shell_config="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        shell_config="$HOME/.bashrc"
    else
        # Default to .bashrc
        shell_config="$HOME/.bashrc"
    fi

    # Check if already configured
    if grep -q 'ISOLDE_HOME' "$shell_config" 2>/dev/null; then
        log_info "PATH already configured in $shell_config"
        return 0
    fi

    log_info "Adding Isolde to PATH in $shell_config..."

    # Add to shell config
    {
        echo ""
        echo "# Isolde - ISOLated Development Environment"
        echo "export ISOLDE_HOME=\"$ISOLDE_HOME\""
        echo "export PATH=\"\$PATH:\$ISOLDE_HOME\""
    } >> "$shell_config"

    log_info "Added to $shell_config"
}

# Show completion message
show_completion() {
    echo ""
    echo -e "${GREEN}Isolde installed successfully!${NC}"
    echo ""
    echo "Installation directory: $ISOLDE_HOME"
    echo "Version: $(cat "$ISOLDE_HOME/VERSION" 2>/dev/null || echo "unknown")"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo "  1. Reload your shell:"
    echo -e "     ${BLUE}source $shell_config${NC}"
    echo "     (or restart your terminal)"
    echo ""
    echo "  2. Verify installation:"
    echo -e "     ${BLUE}isolde --version${NC}"
    echo ""
    echo "  3. Create a project:"
    echo -e "     ${BLUE}isolde my-project --template=python${NC}"
    echo ""
    echo "For more information:"
    echo "  isolde --help"
    echo "  isolde --list-templates"
    echo "  isolde --list-presets"
    echo ""
}

# Main installation
main() {
    echo ""
    echo -e "${BLUE}Isolde Installation${NC}"
    echo "===================="
    echo ""

    # Check requirements
    if ! command -v git >/dev/null 2>&1; then
        log_error "git is required but not installed"
        exit 1
    fi

    if ! command -v docker >/dev/null 2>&1; then
        log_warn "docker is recommended but not installed"
    fi

    # Install
    install_or_update
    create_wrapper
    create_version_file

    # Track shell config for completion message
    local shell_config=""
    if [ -n "$ZSH_VERSION" ]; then
        shell_config="$HOME/.zshrc"
    else
        shell_config="$HOME/.bashrc"
    fi

    add_to_path
    show_completion
}

# Run main
main "$@"
