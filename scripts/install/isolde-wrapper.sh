#!/bin/bash
# isolde - Isolde distribution wrapper
#
# This wrapper detects whether Isolde is installed or running from source,
# and delegates to isolde.sh accordingly.
#
# Special commands handled by this wrapper:
#   --self-update    Update Isolde installation
#   --version        Show Isolde version

set -e

# Find Isolde installation directory
# Handles: ISOLDE_INSTALL_DIR (set by wrapper), ISOLDE_HOME (env var),
#          ~/.isolde (default install), or source directory
find_install_dir() {
    # Check if wrapper set the install directory
    if [ -n "${ISOLDE_INSTALL_DIR}" ] && [ -f "${ISOLDE_INSTALL_DIR}/isolde.sh" ]; then
        echo "${ISOLDE_INSTALL_DIR}"
        return 0
    fi

    # Check ISOLDE_HOME environment variable
    if [ -n "$ISOLDE_HOME" ] && [ -f "$ISOLDE_HOME/isolde.sh" ]; then
        echo "$ISOLDE_HOME"
        return 0
    fi

    # Check default installation directory
    if [ -f "$HOME/.isolde/isolde.sh" ]; then
        echo "$HOME/.isolde"
        return 0
    fi

    # Running from source - find isolde.sh relative to this script
    local script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

    # Check if we're in the install directory (installed scenario)
    if [ -f "$script_dir/isolde.sh" ]; then
        echo "$script_dir"
        return 0
    fi

    # Check if we're in scripts directory (source scenario)
    if [ -f "$script_dir/isolde.sh" ]; then
        echo "$script_dir"
        return 0
    fi

    # Check parent directories (might be in scripts/ subdirectory)
    if [ -f "$script_dir/../isolde.sh" ]; then
        echo "$(cd "$script_dir/.." && pwd)"
        return 0
    fi

    # Check scripts/ subdirectory
    if [ -f "$script_dir/scripts/isolde.sh" ]; then
        echo "$script_dir/scripts"
        return 0
    fi

    return 1
}

# Self-update: pull latest changes from git
isolde_self_update() {
    local install_dir="$(find_install_dir)" || {
        echo "Error: Cannot find Isolde installation directory"
        return 1
    }

    if [ ! -d "$install_dir/.git" ]; then
        echo "Error: Not a git installation (cannot self-update)"
        echo "Isolde was not installed via git - please reinstall from:"
        echo "  https://github.com/jubbon/isolde"
        return 1
    fi

    echo "Updating Isolde..."
    echo "Installation directory: $install_dir"
    echo ""

    # Fetch and pull
    git -C "$install_dir" fetch origin 2>/dev/null || {
        echo "Warning: Git fetch failed, attempting pull anyway..."
    }

    local current_branch=$(git -C "$install_dir" rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main")
    git -C "$install_dir" pull origin "$current_branch" || return 1

    # Update VERSION file
    git -C "$install_dir" describe --tags --always 2>/dev/null > "$install_dir/VERSION" \
        || echo "unknown" > "$install_dir/VERSION"

    echo ""
    echo "Isolde updated successfully!"
    echo "New version: $(cat "$install_dir/VERSION" 2>/dev/null || echo "unknown")"
}

# Show version from VERSION file or git
show_version() {
    local install_dir="$(find_install_dir)" || {
        echo "unknown (cannot find installation)"
        return 0
    }

    if [ -f "$install_dir/VERSION" ]; then
        cat "$install_dir/VERSION"
    elif [ -d "$install_dir/.git" ]; then
        git -C "$install_dir" describe --tags --always 2>/dev/null || echo "unknown"
    else
        echo "unknown"
    fi
}

# Main entry point
main() {
    # Handle special commands before delegating
    case "$1" in
        --self-update)
            isolde_self_update
            exit $?
            ;;
        --version|-v)
            show_version
            exit 0
            ;;
        --help|-h)
            # Delegate to isolde.sh for full help
            # But show quick help if isolde.sh not found
            local install_dir="$(find_install_dir)" 2>/dev/null || true
            if [ -n "$install_dir" ] && [ -f "$install_dir/isolde.sh" ]; then
                export ISOLDE_INSTALL_DIR="$install_dir"
                exec bash "$install_dir/isolde.sh" --help
            else
                echo "Isolde - ISOLated Development Environment"
                echo ""
                echo "Usage: isolde [project-name] [options]"
                echo ""
                echo "Wrapper commands:"
                echo "  --self-update    Update Isolde to latest version"
                echo "  --version        Show Isolde version"
                echo "  --help          Show full help"
                echo ""
                echo "Error: Cannot find isolde.sh - please reinstall Isolde"
                exit 1
            fi
            ;;
    esac

    # Find installation directory
    local install_dir="$(find_install_dir)" || {
        echo "Error: Cannot find Isolde installation"
        echo ""
        echo "Isolde may not be installed correctly. Please reinstall:"
        echo "  curl -fsSL https://raw.githubusercontent.com/jubbon/isolde/main/scripts/install/install.sh | bash"
        exit 1
    }

    # Set install directory for isolde.sh
    export ISOLDE_INSTALL_DIR="$install_dir"

    # Delegate to isolde.sh
    if [ -f "$install_dir/isolde.sh" ]; then
        exec bash "$install_dir/isolde.sh" "$@"
    else
        echo "Error: isolde.sh not found in $install_dir"
        echo "Please reinstall Isolde"
        exit 1
    fi
}

main "$@"
