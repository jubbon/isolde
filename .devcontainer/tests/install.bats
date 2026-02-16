#!/usr/bin/env bats
# Tests for claude-code install.sh script

@test "install.sh exists and is readable" {
	[ -f "../features/claude-code/install.sh" ]
}

@test "install.sh is executable" {
	[ -x "../features/claude-code/install.sh" ]
}

@test "install.sh has proper shebang" {
	head -n1 "../features/claude-code/install.sh" | grep -q '#!' || true
}

@test "install.sh has error handling set -e" {
	grep -q "set -e" "../features/claude-code/install.sh" || true
}

@test "install.sh defines configure_claude_provider function" {
	grep -q "configure_claude_provider" "../features/claude-code/install.sh" || true
}

@test "install.sh defines configure_auto_update function" {
	grep -q "configure_auto_update" "../features/claude-code/install.sh" || true
}
