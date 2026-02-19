#!/bin/bash
#
# test-plugins.sh - Unit tests for plugins.sh library
#

# Import the library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../scripts/lib/plugins.sh"

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test helper functions
test_start() {
    TESTS_RUN=$((TESTS_RUN + 1))
    echo "Running: $1"
}

test_pass() {
    TESTS_PASSED=$((TESTS_PASSED + 1))
    echo "  ✓ PASSED"
}

test_fail() {
    TESTS_FAILED=$((TESTS_FAILED + 1))
    echo "  ✗ FAILED: $1"
}

assert_equals() {
    local expected="$1"
    local actual="$2"
    local message="${3:-Assertion failed}"

    if [ "$expected" = "$actual" ]; then
        return 0
    else
        test_fail "$message (expected: '$expected', got: '$actual')"
        return 1
    fi
}

assert_not_empty() {
    local value="$1"
    local message="${2:-Value should not be empty}"

    if [ -n "$value" ]; then
        return 0
    else
        test_fail "$message"
        return 1
    fi
}

assert_contains() {
    local haystack="$1"
    local needle="$2"
    local message="${3:-Value should contain substring}"

    if echo "$haystack" | grep -q "$needle"; then
        return 0
    else
        test_fail "$message (needle: '$needle' not in haystack)"
        return 1
    fi
}

# Setup test environment
setup_test_env() {
    TEST_HOME=$(mktemp -d)
    TEST_PROJECT=$(mktemp -d)

    # Create mock installed_plugins.json
    mkdir -p "$TEST_HOME/.claude/plugins"
    cat > "$TEST_HOME/.claude/plugins/installed_plugins.json" << 'EOF'
{
  "version": 2,
  "plugins": {
    "superpowers@superpowers-marketplace": [
      {
        "scope": "user",
        "installPath": "/home/user/.claude/plugins/cache/superpowers-marketplace/superpowers/4.2.0"
      }
    ],
    "oh-my-claudecode@omc": [
      {
        "scope": "user",
        "installPath": "/home/user/.claude/plugins/cache/omc/oh-my-claudecode/4.1.8"
      }
    ],
    "frontend-design@claude-plugins-official": [
      {
        "scope": "user",
        "installPath": "/home/user/.claude/plugins/cache/claude-plugins-official/frontend-design/2.0"
      }
    ]
  }
}
EOF
}

cleanup_test_env() {
    rm -rf "$TEST_HOME" "$TEST_PROJECT"
}

# Tests
test_discover_installed_plugins() {
    test_start "discover_installed_plugins"

    local result=$(discover_installed_plugins "$TEST_HOME")

    assert_contains "$result" "superpowers@superpowers-marketplace" "Should find superpowers plugin" || return
    assert_contains "$result" "oh-my-claudecode@omc" "Should find oh-my-claudecode plugin" || return
    assert_contains "$result" "frontend-design@claude-plugins-official" "Should find frontend-design plugin" || return

    test_pass
}

test_find_plugin_identifier_exact_match() {
    test_start "find_plugin_identifier (exact match)"

    local result=$(find_plugin_identifier "superpowers" "$TEST_HOME")

    assert_equals "superpowers@superpowers-marketplace" "$result" "Should find full identifier for superpowers"

    test_pass
}

test_find_plugin_identifier_partial_match() {
    test_start "find_plugin_identifier (partial match)"

    local result=$(find_plugin_identifier "oh-my" "$TEST_HOME")

    assert_contains "$result" "oh-my-claudecode@omc" "Should find plugin with partial match"

    test_pass
}

test_find_plugin_identifier_not_found() {
    test_start "find_plugin_identifier (not found)"

    local result=$(find_plugin_identifier "nonexistent-plugin" "$TEST_HOME")

    assert_equals "" "$result" "Should return empty string for unknown plugin"

    test_pass
}

test_parse_activate_plugins() {
    test_start "parse_activate_plugins"

    local result=$(parse_activate_plugins '["superpowers","tdd","frontend"]')

    assert_contains "$result" "superpowers" "Should parse superpowers" || return
    assert_contains "$result" "tdd" "Should parse tdd" || return
    assert_contains "$result" "frontend" "Should parse frontend" || return

    test_pass
}

test_parse_activate_plugins_empty() {
    test_start "parse_activate_plugins (empty)"

    local result=$(parse_activate_plugins "")

    assert_equals "" "$result" "Should return empty for empty input"

    test_pass
}

test_parse_deactivate_plugins() {
    test_start "parse_deactivate_plugins"

    local result=$(parse_deactivate_plugins '["autopilot","ralph"]')

    assert_contains "$result" "autopilot" "Should parse autopilot" || return
    assert_contains "$result" "ralph" "Should parse ralph" || return

    test_pass
}

test_build_enabled_plugins() {
    test_start "build_enabled_plugins"

    local activate="superpowers
oh-my-claudecode"
    local deactivate="frontend-design"

    local result=$(build_enabled_plugins "$TEST_HOME" "$activate" "$deactivate")

    # Result should be valid JSON
    assert_not_empty "$result" "Result should not be empty" || return

    if command -v jq >/dev/null 2>&1; then
        # Verify JSON structure
        local superpowers_value=$(echo "$result" | jq -r '."superpowers@superpowers-marketplace"')
        assert_equals "true" "$superpowers_value" "superpowers should be true" || return

        local ohmy_value=$(echo "$result" | jq -r '."oh-my-claudecode@omc"')
        assert_equals "true" "$ohmy_value" "oh-my-claudecode should be true" || return

        local frontend_value=$(echo "$result" | jq -r '."frontend-design@claude-plugins-official"')
        assert_equals "false" "$frontend_value" "frontend-design should be false" || return
    fi

    test_pass
}

test_write_claude_settings_new_file() {
    test_start "write_claude_settings (new file)"

    local enabled_plugins='{"test@example-market":true}'

    write_claude_settings "$TEST_PROJECT" "$enabled_plugins"

    if [ -f "$TEST_PROJECT/.claude/settings.json" ]; then
        local content=$(cat "$TEST_PROJECT/.claude/settings.json")
        assert_contains "$content" "enabledPlugins" "Should contain enabledPlugins" || return
        assert_contains "$content" "test@example-market" "Should contain plugin ID" || return
        test_pass
    else
        test_fail "Settings file was not created"
    fi
}

test_merge_claude_settings_existing() {
    test_start "merge_claude_settings (existing file)"

    # Create existing settings with other sections
    cat > "$TEST_PROJECT/.claude/settings.json" << 'EOF'
{
  "permissions": {
    "defaultMode": "bypassPermissions"
  },
  "env": {
    "TEST_VAR": "value"
  }
}
EOF

    local new_plugins='{"test@example-market":true}'

    merge_claude_settings "$TEST_PROJECT" "$new_plugins"

    if [ -f "$TEST_PROJECT/.claude/settings.json" ]; then
        local content=$(cat "$TEST_PROJECT/.claude/settings.json")
        assert_contains "$content" "permissions" "Should preserve permissions" || return
        assert_contains "$content" "defaultMode" "Should preserve defaultMode" || return
        assert_contains "$content" "enabledPlugins" "Should add enabledPlugins" || return
        test_pass
    else
        test_fail "Settings file was deleted"
    fi
}

test_find_project_dir() {
    test_start "find_project_dir"

    # Create a mock feature directory structure
    local mock_feature="$TEST_PROJECT/.devcontainer/features/plugin-manager"
    mkdir -p "$mock_feature"

    local result=$(find_project_dir "$mock_feature")

    assert_equals "$TEST_PROJECT" "$result" "Should find project root"

    test_pass
}

# Main test runner
main() {
    echo "======================================"
    echo "Running plugins.sh unit tests"
    echo "======================================"
    echo ""

    setup_test_env

    # Run all tests
    test_discover_installed_plugins
    test_find_plugin_identifier_exact_match
    test_find_plugin_identifier_partial_match
    test_find_plugin_identifier_not_found
    test_parse_activate_plugins
    test_parse_activate_plugins_empty
    test_parse_deactivate_plugins
    test_build_enabled_plugins
    test_write_claude_settings_new_file
    test_merge_claude_settings_existing
    test_find_project_dir

    cleanup_test_env

    # Summary
    echo ""
    echo "======================================"
    echo "Test Results:"
    echo "  Run:     $TESTS_RUN"
    echo "  Passed:  $TESTS_PASSED"
    echo "  Failed:  $TESTS_FAILED"
    echo "======================================"

    if [ $TESTS_FAILED -eq 0 ]; then
        echo "✓ All tests passed!"
        exit 0
    else
        echo "✗ Some tests failed"
        exit 1
    fi
}

# Run tests if script is executed directly
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    main "$@"
fi
