#!/usr/bin/env bats
# Tests for provider configuration

@test "providers directory can be created" {
	test_dir=$(mktemp -d)
	mkdir -p "$test_dir/custom"
	[ -d "$test_dir/custom" ]
	rm -rf "$test_dir"
}

@test "provider auth file can be created" {
	test_dir=$(mktemp -d)
	mkdir -p "$test_dir/custom"
	echo "test-token" > "$test_dir/custom/auth"
	[ -f "$test_dir/custom/auth" ]
	[ "$(cat "$test_dir/custom/auth")" = "test-token" ]
	rm -rf "$test_dir"
}

@test "provider base_url file can be created" {
	test_dir=$(mktemp -d)
	mkdir -p "$test_dir/custom"
	echo "https://api.example.com" > "$test_dir/custom/base_url"
	[ -f "$test_dir/custom/base_url" ]
	rm -rf "$test_dir"
}

@test "multiple providers can coexist" {
	test_dir=$(mktemp -d)
	mkdir -p "$test_dir/anthropic"
	mkdir -p "$test_dir/custom"
	mkdir -p "$test_dir/z.ai"
	[ -d "$test_dir/anthropic" ]
	[ -d "$test_dir/custom" ]
	[ -d "$test_dir/z.ai" ]
	rm -rf "$test_dir"
}

@test "provider names are valid directory names" {
	test_dir=$(mktemp -d)
	valid_names=("anthropic" "z.ai" "custom" "my-provider" "provider_123")
	for name in "${valid_names[@]}"; do
		mkdir -p "$test_dir/$name"
		[ -d "$test_dir/$name" ]
	done
	rm -rf "$test_dir"
}
