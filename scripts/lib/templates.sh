#!/bin/bash
#
# Template-related functions for init-project.sh
#
# Note: This script is sourced by init-project.sh
# SCRIPT_DIR and utility functions are already available

# Load template metadata
load_template_info() {
    local template="$1"
    local templates_root="$(get_templates_root)"
    local info_file="$templates_root/templates/$template/template-info.yaml"

    if [ ! -f "$info_file" ]; then
        log_error "Template info file not found: $info_file"
        return 1
    fi

    # Export template info as environment variables
    export TEMPLATE_NAME=$(read_yaml_value "$info_file" "name")
    export TEMPLATE_DESCRIPTION=$(read_yaml_value "$info_file" "description")
    export TEMPLATE_VERSION=$(read_yaml_value "$info_file" "version")
    export TEMPLATE_LANG_VERSION_DEFAULT=$(read_yaml_value "$info_file" "lang_version_default")
}

# Get available language versions for a template
get_template_versions() {
    local template="$1"
    local templates_root="$(get_templates_root)"
    local info_file="$templates_root/templates/$template/template-info.yaml"

    if [ ! -f "$info_file" ]; then
        echo ""
        return 1
    fi

    # Parse supported_versions from YAML (exclude devcontainer_features, strip quotes)
    awk '/^supported_versions:/,/^devcontainer_features:/ {print}' "$info_file" | grep '^  -' | sed 's/^  - //' | sed 's/"//g' | tr '\n' ' '
}

# Validate language version
validate_template_version() {
    local template="$1"
    local version="$2"
    local versions=$(get_template_versions "$template")

    if [ -z "$versions" ]; then
        return 0
    fi

    for v in $versions; do
        if [ "$v" = "$version" ]; then
            return 0
        fi
    done

    log_error "Invalid version '$version' for template '$template'. Valid versions: $versions"
    return 1
}

# Apply template to project directory
apply_template() {
    local template="$1"
    local project_dir="$2"
    local project_name="$3"
    local templates_root="$(get_templates_root)"
    local template_dir="$templates_root/templates/$template"

    if ! template_exists "$template"; then
        log_error "Template not found: $template"
        return 1
    fi

    log_debug "Applying template: $template"
    log_debug "Template directory: $template_dir"
    log_debug "Project directory: $project_dir"

    # Copy devcontainer files
    if [ -d "$template_dir/.devcontainer" ]; then
        cp -r "$template_dir/.devcontainer" "$project_dir/" || {
            log_error "Failed to copy .devcontainer directory"
            return 1
        }

        # Apply template substitutions
        apply_template_substitutions "$template" "$project_dir" "$project_name"
    else
        log_error "Template .devcontainer directory not found"
        return 1
    fi

    return 0
}

# Apply template substitutions to devcontainer.json
apply_template_substitutions() {
    local template="$1"
    local project_dir="$2"
    local project_name="$3"
    local devcontainer_json="$project_dir/.devcontainer/devcontainer.json"

    if [ ! -f "$devcontainer_json" ]; then
        return 0
    fi

    log_debug "Applying template substitutions"

    # Get template-specific values
    local lang_version="${TEMPLATE_LANG_VERSION:-$(get_template_info "$template" "lang_version_default")}"

    # Get feature paths (relative to project)
    local templates_root="$(get_templates_root)"
    local core_features="$templates_root/core/features"

    # Calculate relative paths to core features
    local features_dir="$project_dir/.devcontainer/features"
    mkdir -p "$features_dir"

    # Copy core features (Docker cannot follow symlinks outside build context)
    for feature in "$core_features"/*; do
        if [ -d "$feature" ]; then
            local feature_name=$(basename "$feature")
            cp -r "$feature" "$features_dir/$feature_name"
            log_debug "Copied feature: $features_dir/$feature_name"
        fi
    done

    # Calculate relative path for features
    local features_relative="../../../../templates/../../core/features"

    # Template substitutions
    sed -i "s|{{PROJECT_NAME}}|$project_name|g" "$devcontainer_json"
    sed -i "s|{{PYTHON_VERSION}}|$lang_version|g" "$devcontainer_json"
    sed -i "s|{{NODE_VERSION}}|$lang_version|g" "$devcontainer_json"
    sed -i "s|{{RUST_VERSION}}|$lang_version|g" "$devcontainer_json"
    sed -i "s|{{GO_VERSION}}|$lang_version|g" "$devcontainer_json"

    # Feature path substitutions
    sed -i "s|{{FEATURES_CLAUDE_CODE}}|./features/claude-code|g" "$devcontainer_json"
    sed -i "s|{{FEATURES_PROXY}}|./features/proxy|g" "$devcontainer_json"

    # Claude Code configuration
    sed -i "s|{{CLAUDE_VERSION}}|${CLAUDE_VERSION:-latest}|g" "$devcontainer_json"
    sed -i "s|{{CLAUDE_PROVIDER}}|${CLAUDE_PROVIDER:-}|g" "$devcontainer_json"
    sed -i "s|{{CLAUDE_MODELS}}|${CLAUDE_MODELS:-}|g" "$devcontainer_json"

    # Proxy configuration
    sed -i "s|{{HTTP_PROXY}}|${HTTP_PROXY:-}|g" "$devcontainer_json"
    sed -i "s|{{HTTPS_PROXY}}|${HTTPS_PROXY:-}|g" "$devcontainer_json"
    sed -i "s|{{NO_PROXY}}|${NO_PROXY:-localhost,127.0.0.1,.local}|g" "$devcontainer_json"
    sed -i "s|{{PROXY_ENABLED}}|${PROXY_ENABLED:-true}|g" "$devcontainer_json"

    log_debug "Template substitutions applied"
}

# List template features
list_template_features() {
    local template="$1"
    local templates_root="$(get_templates_root)"
    local info_file="$templates_root/templates/$template/template-info.yaml"

    if [ ! -f "$info_file" ]; then
        return
    fi

    # Parse features section
    awk '/^features:/,/^[a-z]+:/ {print}' "$info_file" | grep '^  - name:' | sed 's/^  - name: //' | tr '\n' ' '
}

# Display template details
show_template_details() {
    local template="$1"

    if ! template_exists "$template"; then
        log_error "Template not found: $template"
        return 1
    fi

    load_template_info "$template"

    echo ""
    echo -e "${GREEN}Template: $TEMPLATE_NAME${NC}"
    echo "Description: $TEMPLATE_DESCRIPTION"
    echo "Version: $TEMPLATE_VERSION"

    local versions=$(get_template_versions "$template")
    if [ -n "$versions" ]; then
        echo "Supported versions: $versions"
    fi

    local features=$(list_template_features "$template")
    if [ -n "$features" ]; then
        echo "Features: $features"
    fi
    echo ""
}

# Get template from preset
get_preset_template() {
    local preset="$1"
    get_preset_value "$preset" "template"
}

# Get lang_version from preset
get_preset_lang_version() {
    local preset="$1"
    get_preset_value "$preset" "lang_version"
}
