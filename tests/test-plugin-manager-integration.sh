#!/bin/bash
#
# Integration tests for plugin-manager feature
#

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
source "$PROJECT_ROOT/scripts/lib/utils.sh"
source "$PROJECT_ROOT/scripts/lib/plugins.sh"

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test helper functions
test_start() {
    TESTS_RUN=$((TESTS_RUN + 1))
    TEST_NAME="$1"
    echo "=== Test $TESTS_RUN: $TEST_NAME ==="
}

test_pass() {
    TESTS_PASSED=$((TESTS_PASSED + 1))
    echo "✓ PASSED: $TEST_NAME"
    echo ""
}

test_fail() {
    TESTS_FAILED=$((TESTS_FAILED + 1))
    echo "✗ FAILED: $TEST_NAME"
    echo "  Reason: $1"
    echo ""
}

# Setup test environment
setup_test_env() {
    export TEST_PROJECT_DIR="/tmp/test-plugin-integration"
    export TEST_HOME="/tmp/test-plugin-home"
    rm -rf "$TEST_PROJECT_DIR" "$TEST_HOME"
    mkdir -p "$TEST_PROJECT_DIR/.claude"
    mkdir -p "$TEST_HOME/.claude/plugins"
}

# ============================================
# Integration Tests
# ============================================

test_start "Plugin discovery from installed_plugins.json"
setup_test_env
# Create a mock installed_plugins.json
cat > "$TEST_HOME/.claude/plugins/installed_plugins.json" << 'EOF'
{
  "plugins": {
    "oh-my-claudecode@superpowers-marketplace": {
      "name": "oh-my-claudecode",
      "marketplace": "superpowers-marketplace",
      "identifier": "oh-my-claudecode@superpowers-marketplace",
      "scope": "user",
      "version": "4.2.0",
      "path": "/home/user/.claude/plugins/cache/superpowers-marketplace/oh-my-claudecode/4.2.0"
    },
    "tdd@superpowers-marketplace": {
      "name": "tdd",
      "marketplace": "superpowers-marketplace",
      "identifier": "tdd@superpowers-marketplace",
      "scope": "local",
      "version": "1.0.0",
      "path": "/home/user/.claude/plugins/cache/superpowers-marketplace/tdd/1.0.0"
    },
    "security-review@built-in": {
      "name": "security-review",
      "marketplace": "built-in",
      "identifier": "security-review@built-in",
      "scope": "user",
      "version": "1.0.0",
      "path": "/usr/local/lib/claude-code/plugins/security-review"
    }
  }
}
EOF

DISCOVERED=$(discover_installed_plugins "$TEST_HOME")
if echo "$DISCOVERED" | grep -q "oh-my-claudecode@superpowers-marketplace"; then
    test_pass
else
    test_fail "Plugin discovery failed. Got: $DISCOVERED"
fi

test_start "Find plugin identifier by short name"
IDENTIFIER=$(find_plugin_identifier "oh-my-claudecode" "$TEST_HOME")
if [ "$IDENTIFIER" == "oh-my-claudecode@superpowers-marketplace" ]; then
    test_pass
else
    test_fail "Expected 'oh-my-claudecode@superpowers-marketplace', got '$IDENTIFIER'"
fi

test_start "Write claude settings with plugins"
setup_test_env
cat > "$TEST_HOME/.claude/plugins/installed_plugins.json" << 'EOF'
{
  "plugins": {
    "oh-my-claudecode@superpowers-marketplace": {
      "name": "oh-my-claudecode",
      "identifier": "oh-my-claudecode@superpowers-marketplace"
    },
    "tdd@superpowers-marketplace": {
      "name": "tdd",
      "identifier": "tdd@superpowers-marketplace"
    }
  }
}
EOF

# Parse plugin lists from JSON arrays
ACTIVATE_LIST=$(parse_activate_plugins '["oh-my-claudecode", "tdd"]')
DEACTIVATE_LIST=$(parse_deactivate_plugins '[]')

# Build enabled plugins JSON object
ENABLED_PLUGINS=$(build_enabled_plugins "$TEST_HOME" "$ACTIVATE_LIST" "$DEACTIVATE_LIST")

# Write settings
write_claude_settings "$TEST_PROJECT_DIR" "$ENABLED_PLUGINS"

if [ -f "$TEST_PROJECT_DIR/.claude/settings.json" ]; then
    CONTENT=$(cat "$TEST_PROJECT_DIR/.claude/settings.json")
    # Check that plugins are in enabledPlugins
    if echo "$CONTENT" | jq -e '.enabledPlugins["oh-my-claudecode@superpowers-marketplace"] == true' > /dev/null 2>&1 && \
       echo "$CONTENT" | jq -e '.enabledPlugins["tdd@superpowers-marketplace"] == true' > /dev/null 2>&1; then
        test_pass
    else
        test_fail "Plugins not correctly added to enabledPlugins. Content: $CONTENT"
    fi
else
    test_fail "settings.json not created"
fi

test_start "Build enabled plugins with activate and deactivate lists"
setup_test_env
cat > "$TEST_HOME/.claude/plugins/installed_plugins.json" << 'EOF'
{
  "plugins": {
    "oh-my-claudecode@superpowers": {
      "name": "oh-my-claudecode",
      "identifier": "oh-my-claudecode@superpowers"
    },
    "security-review@built-in": {
      "name": "security-review",
      "identifier": "security-review@built-in"
    },
    "tdd@superpowers": {
      "name": "tdd",
      "identifier": "tdd@superpowers"
    }
  }
}
EOF

RESULT=$(build_enabled_plugins "$TEST_HOME" "oh-my-claudecode
tdd" "security-review")
if echo "$RESULT" | jq -e '."oh-my-claudecode@superpowers" == true and ."tdd@superpowers" == true and ."security-review@built-in" == false' > /dev/null 2>&1; then
    test_pass
else
    test_fail "Plugin build logic incorrect: $RESULT"
fi

test_start "Preset parsing returns correct JSON arrays"
source "$PROJECT_ROOT/scripts/lib/presets.sh"
ACTIVATE=$(get_preset_claude_plugins_activate "python-ml")
DEACTIVATE=$(get_preset_claude_plugins_deactivate "python-ml")

if echo "$ACTIVATE" | jq -e '. == ["oh-my-claudecode", "tdd"]' > /dev/null 2>&1 && \
   echo "$DEACTIVATE" | jq -e '. == []' > /dev/null 2>&1; then
    test_pass
else
    test_fail "Preset parsing incorrect. Activate: $ACTIVATE, Deactivate: $DEACTIVATE"
fi

test_start "Parse activate plugins from JSON array"
RESULT=$(parse_activate_plugins '["oh-my-claudecode", "tdd"]')
if echo "$RESULT" | grep -q "oh-my-claudecode" && echo "$RESULT" | grep -q "tdd"; then
    test_pass
else
    test_fail "Parse activate failed: $RESULT"
fi

test_start "Parse deactivate plugins from JSON array"
RESULT=$(parse_deactivate_plugins '["security-review"]')
if echo "$RESULT" | grep -q "security-review"; then
    test_pass
else
    test_fail "Parse deactivate failed: $RESULT"
fi

test_start "Fullstack preset has 3 activate plugins"
ACTIVATE=$(get_preset_claude_plugins_activate "fullstack")
if echo "$ACTIVATE" | jq -e 'length == 3' > /dev/null 2>&1; then
    test_pass
else
    test_fail "Expected 3 activate plugins for fullstack, got: $ACTIVATE"
fi

test_start "Minimal preset has oh-my-claudecode"
ACTIVATE=$(get_preset_claude_plugins_activate "minimal")
DEACTIVATE=$(get_preset_claude_plugins_deactivate "minimal")
if echo "$ACTIVATE" | jq -e '. == ["oh-my-claudecode"]' > /dev/null 2>&1 && \
   echo "$DEACTIVATE" | jq -e '. == []' > /dev/null 2>&1; then
    test_pass
else
    test_fail "Minimal preset incorrect. Activate: $ACTIVATE, Deactivate: $DEACTIVATE"
fi

test_start "Plugin manager feature files exist"
if [ -f "$PROJECT_ROOT/core/features/plugin-manager/devcontainer-feature.json" ] && \
   [ -f "$PROJECT_ROOT/core/features/plugin-manager/install.sh" ] && \
   [ -f "$PROJECT_ROOT/core/features/plugin-manager/README.md" ]; then
    test_pass
else
    test_fail "Plugin manager feature files missing"
fi

test_start "Plugin manager feature metadata is valid"
METADATA="$PROJECT_ROOT/core/features/plugin-manager/devcontainer-feature.json"
if jq -e '.id == "plugin-manager"' "$METADATA" > /dev/null 2>&1; then
    test_pass
else
    test_fail "Feature metadata invalid"
fi

test_start "Find project directory from feature path"
# Create test structure
setup_test_env
mkdir -p "$TEST_PROJECT_DIR/../project-subdir/.devcontainer/features/plugin-manager"
REAL_DIR=$(find_project_dir "$TEST_PROJECT_DIR/../project-subdir/.devcontainer/features/plugin-manager")
# Should return the project directory (normalized path)
# /tmp/test-plugin-integration/../project-subdir normalizes to /tmp/project-subdir
if [ "$REAL_DIR" == "/tmp/project-subdir" ]; then
    test_pass
else
    test_fail "find_project_dir failed. Expected: /tmp/project-subdir, got: $REAL_DIR"
fi

test_start "Handle empty activate/deactivate arrays"
setup_test_env
cat > "$TEST_HOME/.claude/plugins/installed_plugins.json" << 'EOF'
{
  "plugins": {
    "existing@marketplace": {
      "name": "existing",
      "identifier": "existing@marketplace"
    }
  }
}
EOF

RESULT=$(build_enabled_plugins "$TEST_HOME" "" "")
if echo "$RESULT" | jq -e 'length == 0 or (.existing == true or .existing == false)' > /dev/null 2>&1; then
    test_pass
else
    test_fail "Empty arrays not handled correctly: $RESULT"
fi

test_start "Plugin not found warning"
setup_test_env
cat > "$TEST_HOME/.claude/plugins/installed_plugins.json" << 'EOF'
{
  "plugins": {}
}
EOF

RESULT=$(build_enabled_plugins "$TEST_HOME" "nonexistent-plugin" "")
# Should return empty object or object without the non-existent plugin
if echo "$RESULT" | jq -e 'length == 0' > /dev/null 2>&1; then
    test_pass
else
    test_fail "Non-existent plugin not handled: $RESULT"
fi

# ============================================
# Summary
# ============================================

echo "========================================"
echo "Integration Test Summary"
echo "========================================"
echo "Tests run: $TESTS_RUN"
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo "========================================"

if [ $TESTS_FAILED -eq 0 ]; then
    echo "All tests passed! ✓"
    exit 0
else
    echo "Some tests failed! ✗"
    exit 1
fi
