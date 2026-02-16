#!/bin/bash
#
# Proxy Utility Functions for Devcontainer Features
# Source this file to access proxy configuration from shared state
#

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get HTTP proxy from shared state file
# Returns: empty string if not configured
get_proxy_http() {
    local state_file="${HOME}/.config/devcontainer/proxy"
    if [ -f "$state_file" ]; then
        grep '^HTTP_PROXY=' "$state_file" 2>/dev/null | cut -d'=' -f2 | tr -d "'\''\''"
    fi
}

# Get HTTPS proxy from shared state file
# Returns: empty string if not configured
get_proxy_https() {
    local state_file="${HOME}/.config/devcontainer/proxy"
    if [ -f "$state_file" ]; then
        grep '^HTTPS_PROXY=' "$state_file" 2>/dev/null | cut -d'=' -f2 | tr -d "'\''\''"
    fi
}

# Get NO_PROXY from shared state file
# Returns: default value if not configured
get_proxy_no_proxy() {
    local state_file="${HOME}/.config/devcontainer/proxy"
    if [ -f "$state_file" ]; then
        grep '^NO_PROXY=' "$state_file" 2>/dev/null | cut -d'=' -f2 | tr -d "'\''\''"
    fi
}

# Export proxy variables for curl usage
# Usage: eval "$(proxy_export_for_curl)"
proxy_export_for_curl() {
    local http_proxy
    local https_proxy

    http_proxy="$(get_proxy_http)"
    https_proxy="$(get_proxy_https)"

    if [ -n "$http_proxy" ]; then
        echo "export http_proxy='$http_proxy'"
        echo "export https_proxy='${https_proxy:-$http_proxy}'"
        echo "export HTTP_PROXY='$http_proxy'"
        echo "export HTTPS_PROXY='${https_proxy:-$http_proxy}'"
    fi
}

# Validate proxy URL format
# Usage: validate_proxy <url> <var_name>
# Returns: 0 if valid, 1 if invalid
validate_proxy() {
    local url="$1"
    local var_name="$2"

    if [ -z "$url" ]; then
        return 0  # Empty is valid (no proxy)
    fi

    # Basic URL validation: must start with http:// or https://
    if [[ ! "$url" =~ ^https?:// ]]; then
        echo -e "${RED}[ERROR]${NC} Invalid $var_name URL: must start with http:// or https://"
        return 1
    fi

    # Extract host and port for basic validation
    local host_port="${url#*://}"
    if [ -z "$host_port" ]; then
        echo -e "${RED}[ERROR]${NC} Invalid $var_name URL: missing host and/or port"
        return 1
    fi

    return 0
}

# Test proxy connectivity
# Usage: test_proxy <http_proxy> <https_proxy>
# Returns: 0 if reachable, 1 if failed
test_proxy_connectivity() {
    local http_proxy="$1"
    local https_proxy="$2"

    if [ -z "$http_proxy" ] && [ -z "$https_proxy" ]; then
        return 0  # No proxy to test
    fi

    # Try to connect to proxy
    local test_url="https://www.google.com"
    local test_proxy="${https_proxy:-$http_proxy}"

    echo -e "${GREEN}[INFO]${NC} Testing proxy connectivity to $test_proxy..."

    if curl -s -x "$test_proxy" --connect-timeout 5 "$test_url" >/dev/null 2>&1; then
        echo -e "${GREEN}[INFO]${NC} Proxy is reachable"
        return 0
    else
        echo -e "${YELLOW}[WARN]${NC} Proxy may not be reachable (continuing anyway)"
        return 0  # Don't fail, just warn
    fi
}
