#!/bin/bash
#
# plugins.sh - Claude Code Plugin Management Library
# Provides functions for managing Claude Code plugins in devcontainers
#

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Discover all installed plugins from Claude Code's installed_plugins.json
# Args: target_home
# Returns: List of "name@marketplace" entries (one per line)
# Example output:
#   superpowers@superpowers-marketplace
#   oh-my-claudecode@omc
#   frontend-design@claude-plugins-official
discover_installed_plugins() {
    local target_home="$1"
    local installed_json="$target_home/.claude/plugins/installed_plugins.json"

    if [ ! -f "$installed_json" ]; then
        log_warn "Plugin registry not found at $installed_json"
        return 1
    fi

    if command -v jq >/dev/null 2>&1; then
        # Use jq for reliable parsing
        jq -r '.plugins | keys[]' "$installed_json" 2>/dev/null
    else
        # Fallback: grep for keys (less reliable but works)
        grep -E '^\s*"[^"]+@[^"]+":\s*\[' "$installed_json" | sed 's/.*"\([^"]*\)".*/\1/'
    fi
}

# Find full plugin identifier (name@marketplace) for a short plugin name
# Args: plugin_name, target_home
# Returns: Full identifier (name@marketplace) or empty string if not found
# Example: find_plugin_identifier "superpowers" → "superpowers@superpowers-marketplace"
find_plugin_identifier() {
    local plugin_name="$1"
    local target_home="$2"
    local installed_json="$target_home/.claude/plugins/installed_plugins.json"

    if [ ! -f "$installed_json" ]; then
        echo ""
        return 1
    fi

    if command -v jq >/dev/null 2>&1; then
        # Try exact match first (plugin_name@...)
        local exact_match=$(jq -r --arg name "$plugin_name" \
            '.plugins | keys[] | select(startswith($name + "@"))' \
            "$installed_json" 2>/dev/null | head -1)

        if [ -n "$exact_match" ]; then
            echo "$exact_match"
            return 0
        fi

        # Try contains match (fallback)
        local contains_match=$(jq -r --arg name "$plugin_name" \
            '.plugins | keys[] | select(contains($name))' \
            "$installed_json" 2>/dev/null | head -1)

        if [ -n "$contains_match" ]; then
            echo "$contains_match"
            return 0
        fi
    else
        # Fallback: grep-based search
        # Try exact match first
        local exact_match=$(grep -E "\"${plugin_name}@[^\"]+\":" "$installed_json" 2>/dev/null | head -1 | sed 's/.*"\([^"]*\)".*/\1/')
        if [ -n "$exact_match" ]; then
            echo "$exact_match"
            return 0
        fi

        # Try contains match
        local contains_match=$(grep -i "\"[^"]*${plugin_name}[^"]*@" "$installed_json" 2>/dev/null | head -1 | sed 's/.*"\([^"]*\)".*/\1/')
        if [ -n "$contains_match" ]; then
            echo "$contains_match"
            return 0
        fi
    fi

    echo ""
    return 1
}

# Parse activate_plugins JSON array from devcontainer option
# Args: ACTIVATE_PLUGINS string (JSON array or empty)
# Returns: List of plugin names (one per line)
# Example: parse_activate_plugins '["superpowers","tdd"]' → "superpowers\ntdd"
parse_activate_plugins() {
    local activate_str="$1"

    if [ -z "$activate_str" ]; then
        return 0
    fi

    if command -v jq >/dev/null 2>&1; then
        echo "$activate_str" | jq -r '.[]?' 2>/dev/null
    else
        # Fallback: parse JSON array with sed
        echo "$activate_str" | sed 's/^\[//;s/\]$//;s/","/\n"/g;s/"//g'
    fi
}

# Parse deactivate_plugins JSON array from devcontainer option
# Args: DEACTIVATE_PLUGINS string (JSON array or empty)
# Returns: List of plugin names (one per line)
parse_deactivate_plugins() {
    local deactivate_str="$1"

    if [ -z "$deactivate_str" ]; then
        return 0
    fi

    if command -v jq >/dev/null 2>&1; then
        echo "$deactivate_str" | jq -r '.[]?' 2>/dev/null
    else
        # Fallback: parse JSON array with sed
        echo "$deactivate_str" | sed 's/^\[//;s/\]$//;s/","/\n"/g;s/"//g'
    fi
}

# Write enabledPlugins to project .claude/settings.json
# Args: project_dir, enabled_plugins_object (JSON string)
# Example: write_claude_settings "/workspace/myproject" '{"plugin@market":true}'
write_claude_settings() {
    local project_dir="$1"
    local enabled_plugins="$2"
    local settings_file="$project_dir/.claude/settings.json"

    # Ensure .claude directory exists
    mkdir -p "$(dirname "$settings_file")"

    if [ ! -f "$settings_file" ]; then
        # Create new file with enabledPlugins
        cat > "$settings_file" << EOF
{
  "enabledPlugins": $enabled_plugins
}
EOF
        log_info "Created .claude/settings.json with enabledPlugins"
    else
        # Merge with existing file
        merge_claude_settings "$project_dir" "$enabled_plugins"
    fi
}

# Merge new enabledPlugins with existing settings.json
# Args: project_dir, new_enabled_plugins (JSON object)
# Preserves other sections like permissions, mcpServers, etc.
merge_claude_settings() {
    local project_dir="$1"
    local new_plugins="$2"
    local settings_file="$project_dir/.claude/settings.json"

    if [ ! -f "$settings_file" ]; then
        write_claude_settings "$project_dir" "$new_plugins"
        return $?
    fi

    if command -v jq >/dev/null 2>&1; then
        # Use jq for proper merge
        local tmp_file=$(mktemp)
        if jq --argjson new "$new_plugins" \
            '.enabledPlugins = $new | .enabledPlugins |= with_entries(select(.value == true) // .value = false)' \
            "$settings_file" > "$tmp_file" 2>/dev/null; then
            mv "$tmp_file" "$settings_file"
            log_info "Merged enabledPlugins into existing .claude/settings.json"
        else
            rm -f "$tmp_file"
            log_error "Failed to merge settings.json"
            return 1
        fi
    else
        # Fallback: sed-based merge (replace existing enabledPlugins section)
        local tmp_file=$(mktemp)

        # Check if enabledPlugins exists
        if grep -q '"enabledPlugins"' "$settings_file"; then
            # Replace existing enabledPlugins section
            sed '/"enabledPlugins"/,/\}/d' "$settings_file" > "$tmp_file"
            # Add new enabledPlugins before closing brace
            sed -i '$ s/}/,\n  "enabledPlugins": '"$new_plugins"'\n}/' "$tmp_file"
            mv "$tmp_file" "$settings_file"
            log_info "Replaced enabledPlugins in .claude/settings.json (fallback mode)"
        else
            # Append new enabledPlugins before closing brace
            sed -i '$ s/}/,\n  "enabledPlugins": '"$new_plugins"'\n}/' "$settings_file"
            log_info "Added enabledPlugins to .claude/settings.json (fallback mode)"
        fi
    fi

    return 0
}

# Build enabledPlugins JSON object from activate and deactivate lists
# Args: target_home, activate_list (newline-separated), deactivate_list (newline-separated)
# Returns: JSON object with plugin identifiers as keys and boolean values
# Example: build_enabled_plugins "~" "superpowers\ntdd" "autopilot" → '{"superpowers@...":true,"tdd@...":true,"autopilot@...":false}'
build_enabled_plugins() {
    local target_home="$1"
    local activate_list="$2"
    local deactivate_list="$3"

    # Collect all plugins into arrays for jq processing
    local activate_args=""
    local deactivate_args=""

    # Process activate list
    if [ -n "$activate_list" ]; then
        while IFS= read -r plugin_name; do
            [ -z "$plugin_name" ] && continue

            local full_id=$(find_plugin_identifier "$plugin_name" "$target_home")
            if [ -n "$full_id" ]; then
                activate_args="$activate_args --arg id_$plugin_name \"$full_id\""
                log_info "  ✓ Activating: $full_id"
            else
                log_warn "  ? Plugin not found: $plugin_name (skipping)"
            fi
        done <<< "$activate_list"
    fi

    # Process deactivate list
    if [ -n "$deactivate_list" ]; then
        while IFS= read -r plugin_name; do
            [ -z "$plugin_name" ] && continue

            local full_id=$(find_plugin_identifier "$plugin_name" "$target_home")
            if [ -n "$full_id" ]; then
                deactivate_args="$deactivate_args --arg id_$plugin_name \"$full_id\""
                log_info "  ✗ Deactivating: $full_id"
            else
                log_warn "  ? Plugin not found: $plugin_name (skipping)"
            fi
        done <<< "$deactivate_list"
    fi

    # Build JSON using jq
    if command -v jq >/dev/null 2>&1; then
        # Build jq expression dynamically
        local jq_expr="{}"
        local first=true

        # Add activate plugins
        if [ -n "$activate_list" ]; then
            while IFS= read -r plugin_name; do
                [ -z "$plugin_name" ] && continue
                local full_id=$(find_plugin_identifier "$plugin_name" "$target_home")
                if [ -n "$full_id" ]; then
                    # Escape the ID for jq
                    local escaped_id=$(echo "$full_id" | sed 's/"/\\"/g')
                    if [ "$first" = true ]; then
                        jq_expr="{\"$escaped_id\": true}"
                        first=false
                    else
                        jq_expr="$jq_expr + {\"$escaped_id\": true}"
                    fi
                fi
            done <<< "$activate_list"
        fi

        # Add deactivate plugins
        if [ -n "$deactivate_list" ]; then
            while IFS= read -r plugin_name; do
                [ -z "$plugin_name" ] && continue
                local full_id=$(find_plugin_identifier "$plugin_name" "$target_home")
                if [ -n "$full_id" ]; then
                    local escaped_id=$(echo "$full_id" | sed 's/"/\\"/g')
                    if [ "$first" = true ]; then
                        jq_expr="{\"$escaped_id\": false}"
                        first=false
                    else
                        jq_expr="$jq_expr + {\"$escaped_id\": false}"
                    fi
                fi
            done <<< "$deactivate_list"
        fi

        # Execute jq expression
        jq -n "$jq_expr" 2>/dev/null || echo "{}"
    else
        # Fallback: manual JSON construction
        local result="{"
        local first=true

        if [ -n "$activate_list" ]; then
            while IFS= read -r plugin_name; do
                [ -z "$plugin_name" ] && continue
                local full_id=$(find_plugin_identifier "$plugin_name" "$target_home")
                if [ -n "$full_id" ]; then
                    if [ "$first" = true ]; then
                        result="$result\"$full_id\": true"
                        first=false
                    else
                        result="$result, \"$full_id\": true"
                    fi
                fi
            done <<< "$activate_list"
        fi

        if [ -n "$deactivate_list" ]; then
            while IFS= read -r plugin_name; do
                [ -z "$plugin_name" ] && continue
                local full_id=$(find_plugin_identifier "$plugin_name" "$target_home")
                if [ -n "$full_id" ]; then
                    if [ "$first" = true ]; then
                        result="$result\"$full_id\": false"
                        first=false
                    else
                        result="$result, \"$full_id\": false"
                    fi
                fi
            done <<< "$deactivate_list"
        fi

        result="$result}"
        echo "$result"
    fi
}

# Find project directory from feature location
# The feature is installed at <project>/.devcontainer/features/plugin-manager/
# So we need to go up two levels to reach the project root
# Args: feature_dir (current directory of install.sh)
# Returns: Project directory path
find_project_dir() {
    local feature_dir="$1"

    # Feature is at .devcontainer/features/plugin-manager/
    # Project is two levels up
    local project_dir="$(cd "$feature_dir/../../.." && pwd)"

    echo "$project_dir"
}

# Export functions for use in other scripts
export -f discover_installed_plugins
export -f find_plugin_identifier
export -f parse_activate_plugins
export -f parse_deactivate_plugins
export -f write_claude_settings
export -f merge_claude_settings
export -f build_enabled_plugins
export -f find_project_dir
