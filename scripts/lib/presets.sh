#!/bin/bash
#
# Preset-related functions for init-project.sh
#
# Note: This script is sourced by init-project.sh
# SCRIPT_DIR and utility functions are already available

# Load preset configuration
load_preset() {
    local preset="$1"
    local templates_root="$(get_templates_root)"
    local presets_file="$templates_root/presets.yaml"

    if [ ! -f "$presets_file" ]; then
        log_error "Presets file not found: $presets_file"
        return 1
    fi

    if ! preset_exists "$preset"; then
        log_error "Preset not found: $preset"
        return 1
    fi

    log_debug "Loading preset: $preset"

    # Export preset values as environment variables
    export PRESET_TEMPLATE=$(get_preset_value "$preset" "template")
    export PRESET_LANG_VERSION=$(get_preset_value "$preset" "lang_version")
    export PRESET_FEATURES=$(get_preset_value "$preset" "features")
    export PRESET_CLAUDE_PLUGINS=$(get_preset_value "$preset" "claude_plugins")

    log_debug "Preset loaded - template: $PRESET_TEMPLATE, version: $PRESET_LANG_VERSION"
}

# Apply preset to project
apply_preset() {
    local preset="$1"
    local project_dir="$2"

    load_preset "$preset" || return 1

    # Set template version from preset
    if [ -n "$PRESET_LANG_VERSION" ]; then
        export TEMPLATE_LANG_VERSION="$PRESET_LANG_VERSION"
    fi

    # Apply template
    apply_template "$PRESET_TEMPLATE" "$project_dir" "${PROJECT_NAME}" || return 1

    # Create CLAUDE.md with preset plugins if specified
    if [ -n "$PRESET_CLAUDE_PLUGINS" ]; then
        create_claude_md_with_plugins "$project_dir" "$PRESET_CLAUDE_PLUGINS"
    fi

    return 0
}

# List all available presets
list_available_presets() {
    local templates_root="$(get_templates_root)"
    local presets_file="$templates_root/presets.yaml"

    if [ ! -f "$presets_file" ]; then
        log_warn "No presets file found"
        return
    fi

    echo ""
    echo "Available presets:"
    echo ""

    while IFS= read -r line; do
        if [[ "$line" =~ ^[[:space:]]*([a-z-]+):[[:space:]]*$ ]]; then
            local preset_name="${BASH_REMATCH[1]}"
            local preset_desc=$(grep -A1 "^  $preset_name:" "$presets_file" | grep "description:" | sed 's/.*description: *//')

            if [ -n "$preset_desc" ]; then
                echo "  $preset_name - $preset_desc"
            else
                echo "  $preset_name"
            fi
        fi
    done < "$presets_file"
    echo ""
}

# Display preset details
show_preset_details() {
    local preset="$1"

    if ! preset_exists "$preset"; then
        log_error "Preset not found: $preset"
        return 1
    fi

    load_preset "$preset"

    echo ""
    echo -e "${GREEN}Preset: $preset${NC}"
    echo "Template: $PRESET_TEMPLATE"
    echo "Language version: $PRESET_LANG_VERSION"

    if [ -n "$PRESET_FEATURES" ]; then
        echo "Features: $PRESET_FEATURES"
    fi

    if [ -n "$PRESET_CLAUDE_PLUGINS" ]; then
        echo "Claude plugins: $PRESET_CLAUDE_PLUGINS"
    fi
    echo ""
}

# Create CLAUDE.md with plugin recommendations
create_claude_md_with_plugins() {
    local project_dir="$1"
    local plugins="$2"
    local claude_md="$project_dir/.claude/CLAUDE.md"

    mkdir -p "$(dirname "$claude_md")"

    cat > "$claude_md" << EOF
# Claude Code Configuration for ${PROJECT_NAME}

## Recommended Plugins

The following Claude Code plugins are recommended for this project:

EOF

    # Parse comma-separated plugins
    IFS=',' read -ra plugin_array <<< "$plugins"
    for plugin in "${plugin_array[@]}"; do
        plugin=$(echo "$plugin" | xargs) # trim whitespace
        case "$plugin" in
            oh-my-claudecode)
                echo "- **oh-my-claudecode**: Multi-agent orchestration layer for complex tasks" >> "$claude_md"
                ;;
            tdd)
                echo "- **TDD**: Test-driven development workflow enforcement" >> "$claude_md"
                ;;
            security-review)
                echo "- **security-review**: Automated security vulnerability scanning" >> "$claude_md"
                ;;
            *)
                echo "- **$plugin**: Custom plugin" >> "$claude_md"
                ;;
        esac
    done

    cat >> "$claude_md" << EOF

## Installation

Install recommended plugins:

\`\`\`bash
claude-code skill install oh-my-claudecode
\`\`\`

## Project Structure

- \`project/\` - Main project directory (git repository)
- \`.devcontainer/\` - Devcontainer configuration (git repository)
- \`.claude/\` - Claude Code configuration (local, not in git)
EOF

    log_debug "Created CLAUDE.md with plugins: $plugins"
}

# Validate preset
validate_preset() {
    local preset="$1"

    if ! preset_exists "$preset"; then
        log_error "Preset '$preset' does not exist"
        return 1
    fi

    local template=$(get_preset_template "$preset")
    if ! template_exists "$template"; then
        log_error "Preset '$preset' references invalid template: $template"
        return 1
    fi

    return 0
}

# Get preset description
get_preset_description() {
    local preset="$1"
    local templates_root="$(get_templates_root)"
    local presets_file="$templates_root/presets.yaml"

    if [ ! -f "$presets_file" ]; then
        echo ""
        return 1
    fi

    # Find preset and extract description
    local in_preset=0
    while IFS= read -r line; do
        if [ "$in_preset" -eq 1 ]; then
            if [[ "$line" =~ description:[[:space:]]+(.+)$ ]]; then
                echo "${BASH_REMATCH[1]}"
                return 0
            fi
            if [[ "$line" =~ ^[[:space:]]*[a-z]+:[[:space:]]*$ ]]; then
                break
            fi
        fi
        if [[ "$line" =~ ^[[:space:]]*$preset:[[:space:]]*$ ]]; then
            in_preset=1
        fi
    done < "$presets_file"

    echo ""
    return 1
}
