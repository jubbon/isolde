#!/bin/bash
#
# init-project.sh - Create a new project using Claude Code devcontainer templates
#
# Usage:
#   ./init-project.sh [project-name] [options]
#
# Options:
#   --template=TEMPLATE     Template to use (python, nodejs, rust, go, generic)
#   --lang-version=VERSION  Language version (e.g., 3.12, 22, latest)
#   --preset=PRESET         Use a predefined preset
#   --workspace=PATH        Workspace directory (default: ~/workspace)
#   --claude-version=VER    Claude Code version (default: latest)
#   --claude-provider=PROV  LLM provider (e.g., z.ai, anthropic)
#   --claude-models=MODS    Model mapping (haiku:model,sonnet:model,opus:model)
#   --proxy=URL             HTTP proxy URL
#   --no-proxy              Disable proxy
#   --list-templates        List available templates
#   --list-presets          List available presets
#   -h, --help              Show this help
#
# Examples:
#   ./init-project.sh                          # Interactive mode
#   ./init-project.sh myapp --template=python
#   ./init-project.sh ml-app --preset=python-ml
#   ./init-project.sh api --lang=python --version=3.12 --proxy=http://proxy:8080
#

set -e

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source library functions
source "$SCRIPT_DIR/lib/utils.sh"
source "$SCRIPT_DIR/lib/ui.sh"
source "$SCRIPT_DIR/lib/templates.sh"
source "$SCRIPT_DIR/lib/presets.sh"
source "$SCRIPT_DIR/lib/git.sh"

# Default values
WORKSPACE="${WORKSPACE:-$(pwd)}"
PROJECT_NAME=""
TEMPLATE=""
TEMPLATE_LANG_VERSION=""
PRESET=""
CLAUDE_VERSION="${CLAUDE_VERSION:-latest}"
CLAUDE_PROVIDER="${CLAUDE_PROVIDER:-}"
CLAUDE_MODELS="${CLAUDE_MODELS:-}"
HTTP_PROXY="${HTTP_PROXY:-}"
HTTPS_PROXY="${HTTPS_PROXY:-}"
NO_PROXY="${NO_PROXY:-localhost,127.0.0.1,.local}"
PROXY_ENABLED="true"
AUTO_CONFIRM=false

# Display help
show_help() {
    grep '^#' "$0" | grep -v '#!' | sed 's/^# //' | sed 's/^#//'
    exit 0
}

# Parse command line arguments
parse_args() {
    while [ $# -gt 0 ]; do
        case "$1" in
            --template=*)
                TEMPLATE="${1#*=}"
                shift
                ;;
            --lang-version=*)
                TEMPLATE_LANG_VERSION="${1#*=}"
                shift
                ;;
            --preset=*)
                PRESET="${1#*=}"
                shift
                ;;
            --workspace=*)
                WORKSPACE="${1#*=}"
                shift
                ;;
            --claude-version=*)
                CLAUDE_VERSION="${1#*=}"
                shift
                ;;
            --claude-provider=*)
                CLAUDE_PROVIDER="${1#*=}"
                shift
                ;;
            --claude-models=*)
                CLAUDE_MODELS="${1#*=}"
                shift
                ;;
            --proxy=*)
                HTTP_PROXY="${1#*=}"
                HTTPS_PROXY="${1#*=}"
                shift
                ;;
            --no-proxy)
                PROXY_ENABLED="false"
                shift
                ;;
            --http-proxy=*)
                HTTP_PROXY="${1#*=}"
                shift
                ;;
            --https-proxy=*)
                HTTPS_PROXY="${1#*=}"
                shift
                ;;
            --no-proxy=*)
                NO_PROXY="${1#*=}"
                shift
                ;;
            --list-templates)
                show_header "Available Templates"
                for tmpl in $(list_templates); do
                    show_template_details "$tmpl"
                done
                exit 0
                ;;
            --list-presets)
                list_available_presets
                exit 0
                ;;
            -h|--help)
                show_help
                ;;
            --yes|-y)
                AUTO_CONFIRM=true
                shift
                ;;
            -*)
                log_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
            *)
                if [ -z "$PROJECT_NAME" ]; then
                    PROJECT_NAME="$1"
                else
                    log_error "Unexpected argument: $1"
                    exit 1
                fi
                shift
                ;;
        esac
    done
}

# Interactive wizard
run_wizard() {
    show_header "Claude Code Devcontainer Project Creator"

    echo "This wizard will help you create a new project with a devcontainer."
    echo ""

    # Project name
    while [ -z "$PROJECT_NAME" ]; do
        show_prompt "Project name" "" PROJECT_NAME
        PROJECT_NAME=$(sanitize_name "$PROJECT_NAME")
        if ! validate_project_name "$PROJECT_NAME"; then
            PROJECT_NAME=""
        fi
    done

    # Preset or custom
    if show_confirm "Use a preset configuration?" "y"; then
        # Select preset
        local presets=($(list_presets))
        local preset
        preset=$(show_menu "Select a preset" "${presets[@]}")
        PRESET="$preset"
    else
        # Select template
        local templates=($(list_templates))
        local template
        template=$(show_menu "Select template" "${templates[@]}")
        TEMPLATE="$template"

        # Language version (if applicable)
        local versions=$(get_template_versions "$TEMPLATE")
        if [ -n "$versions" ]; then
            show_prompt "Language version" "$(get_template_info "$TEMPLATE" "lang_version_default")" TEMPLATE_LANG_VERSION
        fi
    fi

    # Claude Code configuration
    echo ""
    show_section "Claude Code Configuration"

    show_prompt "Claude Code version" "latest" CLAUDE_VERSION
    show_prompt "LLM provider (leave empty for Anthropic)" "" CLAUDE_PROVIDER
    show_prompt "Model mapping (haiku:model,sonnet:model,opus:model)" "" CLAUDE_MODELS

    # Proxy configuration
    echo ""
    show_section "Proxy Configuration"

    if show_confirm "Configure HTTP proxy?" "n"; then
        show_prompt "HTTP proxy URL" "" HTTP_PROXY
        show_prompt "HTTPS proxy URL" "$HTTP_PROXY" HTTPS_PROXY
        show_prompt "No proxy hosts" "localhost,127.0.0.1,.local" NO_PROXY
    else
        PROXY_ENABLED="false"
    fi

    # Confirm
    echo ""
    show_header "Project Summary"
    echo "Project name: $PROJECT_NAME"
    if [ -n "$PRESET" ]; then
        echo "Preset: $PRESET"
    else
        echo "Template: $TEMPLATE"
        if [ -n "$TEMPLATE_LANG_VERSION" ]; then
            echo "Language version: $TEMPLATE_LANG_VERSION"
        fi
    fi
    echo "Claude version: $CLAUDE_VERSION"
    if [ -n "$CLAUDE_PROVIDER" ]; then
        echo "LLM provider: $CLAUDE_PROVIDER"
    fi
    if [ "$PROXY_ENABLED" = "true" ]; then
        echo "Proxy: $HTTP_PROXY"
    fi
    echo ""

    if ! show_confirm "Create project?" "y"; then
        log_info "Project creation cancelled"
        exit 0
    fi
}

# Validate configuration
validate_config() {
    local errors=0

    # Validate project name
    if [ -z "$PROJECT_NAME" ]; then
        log_error "Project name is required"
        ((errors++))
    elif ! validate_project_name "$PROJECT_NAME"; then
        ((errors++))
    fi

    # Validate preset or template
    if [ -n "$PRESET" ]; then
        if ! validate_preset "$PRESET"; then
            ((errors++))
        fi
        # Get template from preset
        TEMPLATE=$(get_preset_template "$PRESET")
        TEMPLATE_LANG_VERSION=$(get_preset_lang_version "$PRESET")
    elif [ -n "$TEMPLATE" ]; then
        if ! template_exists "$TEMPLATE"; then
            log_error "Template not found: $TEMPLATE"
            ((errors++))
        fi
        # Validate language version
        if [ -n "$TEMPLATE_LANG_VERSION" ]; then
            if ! validate_template_version "$TEMPLATE" "$TEMPLATE_LANG_VERSION"; then
                ((errors++))
            fi
        fi
    else
        log_error "Either --template or --preset is required"
        ((errors++))
    fi

    # Check workspace exists
    if [ ! -d "$WORKSPACE" ]; then
        log_error "Workspace directory does not exist: $WORKSPACE"
        ((errors++))
    fi

    # Check if project already exists
    local project_dir="$WORKSPACE/$PROJECT_NAME"
    if [ -d "$project_dir" ]; then
        log_error "Project directory already exists: $project_dir"
        ((errors++))
    fi

    return $errors
}

# Create project
create_project() {
    local project_dir="$WORKSPACE/$PROJECT_NAME"

    show_section "Creating Project"
    log_info "Project directory: $project_dir"

    # Create project directory structure
    mkdir -p "$project_dir" || {
        log_error "Failed to create project directory"
        return 1
    }

    # Apply template or preset
    if [ -n "$PRESET" ]; then
        log_info "Applying preset: $PRESET"
        apply_preset "$PRESET" "$project_dir" || return 1
    else
        log_info "Applying template: $TEMPLATE"
        apply_template "$TEMPLATE" "$project_dir" "$PROJECT_NAME" || return 1
    fi

    # Initialize git repositories
    log_info "Initializing git repositories"
    init_project_repo "$project_dir" "$PROJECT_NAME" || return 1
    init_devcontainer_repo "$project_dir" "$PROJECT_NAME" || return 1

    # Create .gitignore
    create_project_gitignore "$project_dir"

    # Verify setup
    if ! verify_git_repos "$project_dir"; then
        log_error "Git repository verification failed"
        return 1
    fi

    return 0
}

# Display completion message
show_completion() {
    show_header "Project Created Successfully"

    echo ""
    echo -e "${GREEN}✓${NC} Project ${GREEN}$PROJECT_NAME${NC} created in:"
    echo -e "${BLUE}$WORKSPACE/$PROJECT_NAME${NC}"
    echo ""
    echo "Project structure:"
    echo -e "  ${BLUE}project/${NC}         - Your main project code (git repository)"
    echo -e "  ${BLUE}.devcontainer/${NC}   - Devcontainer configuration (git repository)"
    echo ""
    echo "Next steps:"
    echo "  1. Open in VS Code:"
    echo -e "     ${YELLOW}code $WORKSPACE/$PROJECT_NAME${NC}"
    echo ""
    echo "  2. When prompted, reopen in the container"
    echo ""
    echo "  3. Start coding!"
    echo ""
}

# Main execution
main() {
    # Parse arguments
    parse_args "$@"

    # Run wizard if no preset/template specified
    if [ -z "$PRESET" ] && [ -z "$TEMPLATE" ]; then
        run_wizard
    fi

    # Validate configuration
    if ! validate_config; then
        log_error "Configuration validation failed"
        exit 1
    fi

    # Create project
    if ! create_project; then
        log_error "Failed to create project"
        exit 1
    fi

    # Show completion message
    show_completion
}

# Run main
main "$@"
