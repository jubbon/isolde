#!/usr/bin/env bats
# Tests for claude-code install.sh script

@test "install.sh exists and is readable" {
	[ -f "../.devcontainer/features/claude-code/install.sh" ]
}

@test "install.sh is executable" {
	[ -x "../.devcontainer/features/claude-code/install.sh" ]
}

@test "install.sh has proper shebang" {
	head -n1 "../.devcontainer/features/claude-code/install.sh" | grep -q '#!' || true
}

@test "install.sh has error handling set -e" {
	grep -q "set -e" "../.devcontainer/features/claude-code/install.sh" || true
}

@test "install.sh defines create_provider_dir function" {
	grep -q "create_provider_dir" "../.devcontainer/features/claude-code/install.sh" || true
}

@test "install.sh defines configure_provider function" {
	grep -q "configure_provider" "../.devcontainer/features/claude-code/install.sh" || true
}

@test "install.sh defines download_claude function" {
	grep -q "download_claude" "../.devcontainer/features/claude-code/install.sh" || true
}

@test "install.sh has _is_sourced check" {
	grep -q "_is_sourced" "../.devcontainer/features/claude-code/install.sh" || true
}
