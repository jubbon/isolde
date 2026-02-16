#!/bin/bash
#
# Utility functions for init-project.sh
#

# Colors for output (only define once to allow re-sourcing)
if [ -z "$COLORS_DEFINED" ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    NC='\033[0m'
    COLORS_DEFINED=1
fi

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

log_debug() {
    if [ "${DEBUG:-0}" = "1" ]; then
        echo -e "${BLUE}[DEBUG]${NC} $1"
    fi
}

# Error handling
die() {
    log_error "$1"
    exit 1
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check required commands
check_required_commands() {
    local missing=()
    for cmd in "$@"; do
        if ! command_exists "$cmd"; then
            missing+=("$cmd")
        fi
    done

    if [ ${#missing[@]} -gt 0 ]; then
        die "Missing required commands: ${missing[*]}"
    fi
}

# Sanitize project name (convert to valid directory name)
sanitize_name() {
    local name="$1"
    # Convert to lowercase, replace spaces and special chars with hyphens
    echo "$name" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | tr -cd '[:alnum:]-'
}

# Validate project name
validate_project_name() {
    local name="$1"
    if [ -z "$name" ]; then
        log_error "Project name cannot be empty"
        return 1
    fi
    if echo "$name" | grep -q '[^a-zA-Z0-9_-]'; then
        log_error "Project name can only contain letters, numbers, hyphens, and underscores"
        return 1
    fi
    return 0
}

# Check if directory exists
dir_exists() {
    [ -d "$1" ]
}

# Create directory if it doesn't exist
ensure_dir() {
    local dir="$1"
    if ! dir_exists "$dir"; then
        mkdir -p "$dir" || die "Failed to create directory: $dir"
    fi
}

# Get absolute path
get_absolute_path() {
    local path="$1"
    if [ -d "$path" ]; then
        (cd "$path" && pwd)
    else
        local dir_name
        local base_name
        dir_name=$(cd "$(dirname "$path")" && pwd)
        base_name=$(basename "$path")
        echo "${dir_name}/${base_name}"
    fi
}

# Check if running in a git repository
is_git_repo() {
    git rev-parse --git-dir >/dev/null 2>&1
}

# Get the templates repository root
get_templates_root() {
    local script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
    echo "$script_dir"
}

# Read YAML value (simple grep-based parser for our specific use case)
read_yaml_value() {
    local file="$1"
    local key="$2"
    local value
    value=$(grep "^${key}:" "$file" | sed 's/^[^:]*: *//')
    # Strip surrounding quotes if present
    value="${value%\"}"
    value="${value#\"}"
    echo "$value"
}

# Check if a template exists
template_exists() {
    local template="$1"
    local templates_root="$(get_templates_root)"
    [ -d "$templates_root/templates/$template" ]
}

# List available templates
list_templates() {
    local templates_root="$(get_templates_root)"
    find "$templates_root/templates" -maxdepth 1 -mindepth 1 -type d -exec basename {} \; | sort
}

# Get template info
get_template_info() {
    local template="$1"
    local field="$2"
    local templates_root="$(get_templates_root)"
    local info_file="$templates_root/templates/$template/template-info.yaml"

    if [ ! -f "$info_file" ]; then
        echo ""
        return 1
    fi

    read_yaml_value "$info_file" "$field"
}

# Check if preset exists
preset_exists() {
    local preset="$1"
    local templates_root="$(get_templates_root)"
    local presets_file="$templates_root/presets.yaml"

    if [ ! -f "$presets_file" ]; then
        return 1
    fi

    grep -q "^  $preset:" "$presets_file"
}

# Get preset value
get_preset_value() {
    local preset="$1"
    local field="$2"
    local templates_root="$(get_templates_root)"
    local presets_file="$templates_root/presets.yaml"

    if [ ! -f "$presets_file" ]; then
        echo ""
        return 1
    fi

    # Parse preset section (simple grep-based)
    local in_preset=0
    while IFS= read -r line; do
        if [ "$in_preset" -eq 1 ]; then
            # Check if we hit another preset (starts with optional space + word + colon, but not indented field)
            # Preset names are at column 0 or minimal indentation, fields have 2+ spaces
            if [[ "$line" =~ ^[[:space:]]*[a-z]+[-a-z]*:[[:space:]]*$ ]] && [[ ! "$line" =~ ^[[:space:]][[:space:]] ]]; then
                # Next preset found
                break
            fi
            if [[ "$line" =~ ^[[:space:]]*$field:[[:space:]]+(.+)$ ]]; then
                local value="${BASH_REMATCH[1]}"
                # Strip surrounding quotes if present
                value="${value%\"}"
                value="${value#\"}"
                echo "$value"
                return 0
            fi
        fi
        if [[ "$line" =~ ^[[:space:]]*$preset:[[:space:]]*$ ]]; then
            in_preset=1
        fi
    done < "$presets_file"

    echo ""
    return 1
}

# List available presets
list_presets() {
    local templates_root="$(get_templates_root)"
    local presets_file="$templates_root/presets.yaml"

    if [ ! -f "$presets_file" ]; then
        return
    fi

    # Match only preset names (2-space indent, ends with colon, no value on same line)
    grep -E '^  [a-z-]+:$' "$presets_file" | sed 's/^[[:space:]]*//' | sed 's/:.*$//' | sort
}

# Resolve feature path
resolve_feature_path() {
    local feature="$1"
    local templates_root="$(get_templates_root)"
    local core_features="$templates_root/core/features"

    if [ -d "$core_features/$feature" ]; then
        echo "$core_features/$feature"
    else
        echo "$feature"
    fi
}

# Count path components for symlink calculation
count_path_components() {
    local path="$1"
    echo "$path" | tr '/' '\n' | wc -l
}

# Calculate relative path
get_relative_path() {
    local from="$1"
    local to="$2"

    # Normalize paths
    from=$(get_absolute_path "$from")
    to=$(get_absolute_path "$to")

    # Remove common prefix
    local common=""
    while [ -n "$from" ] && [ -n "$to" ] && [ "${from:0:1}" = "${to:0:1}" ]; do
        local segment_from="${from%%/*}"
        local segment_to="${to%%/*}"

        if [ "$segment_from" = "$segment_to" ]; then
            common="${common}/${segment_from}"
            from="${from#*/}"
            to="${to#*/}"
        else
            break
        fi
    done

    # Count remaining segments in "from"
    local up_count=$(echo "$from" | tr '/' '\n' | grep -c '^')

    # Build relative path
    local result=""
    for ((i=0; i<up_count; i++)); do
        result="${result}../"
    done

    result="${result}${to}"
    echo "${result#./}"
}
