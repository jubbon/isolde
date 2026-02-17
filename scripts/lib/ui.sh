#!/bin/bash
#
# User interface functions for isolde.sh
#
# Note: This script is sourced by isolde.sh
# SCRIPT_DIR and utility functions are already available

# Display a header
show_header() {
    local text="$1"
    local width=60
    local padding=$(( (width - ${#text}) / 2 ))

    echo ""
    echo -e "${BLUE}$(printf '=%.0s' $(seq 1 $width))${NC}"
    printf "%${padding}s%s\n" "" "$text"
    echo -e "${BLUE}$(printf '=%.0s' $(seq 1 $width))${NC}"
    echo ""
}

# Display a prompt
show_prompt() {
    local prompt="$1"
    local default="$2"
    local result_var="$3"

    if [ -n "$default" ]; then
        prompt="$prompt [$default]"
    fi

    echo -n "$prompt: "
    read -r response

    if [ -z "$response" ] && [ -n "$default" ]; then
        response="$default"
    fi

    eval "$result_var='$response'"
}

# Display a yes/no prompt
show_confirm() {
    local prompt="$1"
    local default="${2:-n}"

    if [ "$default" = "y" ]; then
        prompt="$prompt [Y/n]"
    else
        prompt="$prompt [y/N]"
    fi

    while true; do
        echo -n "$prompt: "
        read -r response

        if [ -z "$response" ]; then
            response="$default"
        fi

        case "$response" in
            [Yy]|[Yy][Ee][Ss])
                return 0
                ;;
            [Nn]|[Nn][Oo])
                return 1
                ;;
            *)
                log_error "Please respond with 'yes' or 'no'"
                ;;
        esac
    done
}

# Display a menu
show_menu() {
    local title="$1"
    shift
    local options=("$@")
    local default="${options[0]}"

    # Output menu display to stderr so only the selection goes to stdout
    echo "" >&2
    echo "$title:" >&2
    echo "" >&2

    local i=1
    for opt in "${options[@]}"; do
        if [ "$opt" = "$default" ]; then
            echo "  $i) $opt (default)" >&2
        else
            echo "  $i) $opt" >&2
        fi
        ((i++))
    done
    echo "" >&2

    while true; do
        echo -n "Select option [1-${#options[@]}]: " >&2
        read -r response

        if [ -z "$response" ]; then
            response="1"
        fi

        if [[ "$response" =~ ^[0-9]+$ ]] && [ "$response" -ge 1 ] && [ "$response" -le "${#options[@]}" ]; then
            echo "${options[$((response-1))]}"
            return 0
        else
            log_error "Invalid selection. Please enter a number between 1 and ${#options[@]}" >&2
        fi
    done
}

# Display a list
show_list() {
    local title="$1"
    shift
    local items=("$@")

    echo ""
    echo "$title:"
    echo ""

    for item in "${items[@]}"; do
        echo "  - $item"
    done
    echo ""
}

# Display a section
show_section() {
    local title="$1"
    echo ""
    echo -e "${GREEN}==>${NC} $title"
}

# Display progress
show_progress() {
    local current="$1"
    local total="$2"
    local message="$3"

    local percent=$((current * 100 / total))
    local bar_width=40
    local filled=$((bar_width * current / total))

    printf "\r[%${filled}s>%*s] %d%% - %s" "" $((bar_width - filled)) "" "$percent" "$message"
}

# Complete progress display
show_progress_complete() {
    local message="$1"
    printf "\r[%40s] 100%% - %s\n" "" "$message"
}

# Display success
show_success() {
    local message="$1"
    echo ""
    echo -e "${GREEN}✓${NC} $message"
}

# Display warning
show_warning() {
    local message="$1"
    echo ""
    echo -e "${YELLOW}⚠${NC} $message"
}

# Display error
show_error() {
    local message="$1"
    echo ""
    echo -e "${RED}✗${NC} $message"
}

# Display info box
show_info() {
    local message="$1"
    echo ""
    echo -e "${BLUE}ℹ${NC} $message"
}

# Display table
show_table() {
    local delimiter="${1:-|}"
    shift
    local rows=("$@")

    # Calculate column widths
    local col_widths=()
    local num_cols=0

    for row in "${rows[@]}"; do
        IFS="$delimiter" read -ra cols <<< "$row"
        if [ ${#cols[@]} -gt $num_cols ]; then
            num_cols=${#cols[@]}
        fi

        local i=0
        for col in "${cols[@]}"; do
            local len=${#col}
            if [ $i -ge ${#col_widths[@]} ]; then
                col_widths[$i]=$len
            elif [ $len -gt ${col_widths[$i]} ]; then
                col_widths[$i]=$len
            fi
            ((i++))
        done
    done

    # Print table
    for row in "${rows[@]}"; do
        IFS="$delimiter" read -ra cols <<< "$row"
        local i=0
        for col in "${cols[@]}"; do
            printf " %-${col_widths[$i]}s " "$col"
            ((i++))
        done
        echo ""
    done
}

# Spinner for long operations
start_spinner() {
    local message="$1"
    local spin_chars='|/-\'
    local delay=0.1

    # Create temp file for spinner control
    SPINNER_FILE=$(mktemp)
    echo "1" > "$SPINNER_FILE"

    (
        while [ "$(cat "$SPINNER_FILE")" = "1" ]; do
            for char in ${spin_chars}; do
                printf "\r%s $char" "$message"
                sleep "$delay"
            done
        done
        printf "\r" # Clear spinner
    ) &

    SPINNER_PID=$!
}

stop_spinner() {
    if [ -n "$SPINNER_FILE" ] && [ -f "$SPINNER_FILE" ]; then
        echo "0" > "$SPINNER_FILE"
        wait $SPINNER_PID 2>/dev/null
        rm -f "$SPINNER_FILE"
    fi
}
