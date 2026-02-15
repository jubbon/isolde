#!/usr/bin/env bats
# Tests for JSON configuration files

@test "devcontainer.json is valid JSON" {
	jq -r .name ../.devcontainer/devcontainer.json >/dev/null 2>&1
}

@test "devcontainer-feature.json is valid JSON" {
	jq -r .name ../.devcontainer/features/claude-code/devcontainer-feature.json >/dev/null 2>&1
}

@test "devcontainer.json has required name field" {
	name=$(jq -r .name ../.devcontainer/devcontainer.json 2>/dev/null)
	[ "$name" = "Claude Code Environment" ]
}

@test "devcontainer.json has build section" {
	has_build=$(jq -r .build ../.devcontainer/devcontainer.json 2>/dev/null)
	[ "$has_build" != "null" ]
}

@test "devcontainer.json has features section" {
	has_features=$(jq -r .features ../.devcontainer/devcontainer.json 2>/dev/null)
	[ "$has_features" != "null" ]
}

@test "devcontainer.json has claude-code feature" {
	has_claude=$(jq -r '.features["./features/claude-code"]' ../.devcontainer/devcontainer.json 2>/dev/null)
	[ "$has_claude" != "null" ]
}

@test "devcontainer-feature.json defines provider option" {
	has_provider=$(jq -r '.options[] | select(.id == "provider")' ../.devcontainer/features/claude-code/devcontainer-feature.json 2>/dev/null)
	[ "$has_provider" != "null" ]
}

@test "devcontainer-feature.json defines version option" {
	has_version=$(jq -r '.options[] | select(.id == "version")' ../.devcontainer/features/claude-code/devcontainer-feature.json 2>/dev/null)
	[ "$has_version" != "null" ]
}
