# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-03-30

### Added

- Multi-agent support: `agents:` list in `isolde.yaml` for multiple coding agents per container
- OpenCode agent: feature, template, and full integration (`core/features/opencode/`, `templates/opencode/`)
- Agent permissions: `permissions` block in agent config with `allowed_tools`, `allowed_commands`, `deny_commands`
- Settings file generation: `render_agent_settings()` produces `.claude/settings.json` or `opencode.json` from permissions
- Auto-sync freshness check: warns before `build`/`run` if `isolde.yaml` is newer than generated files
- "Do not edit" banners in all generated files (Dockerfile, devcontainer.json, CLAUDE.md)
- Render tests for proxy, codex, opencode, plugins, and multi-agent scenarios

### Changed

- `agent:` field deprecated in favor of `agents:` list (soft migration with deprecation warning)
- `render_devcontainer_json()` loops over all agents for features, Node.js dedup, and install order
- `render_claude_md()` shows all agents (singular "Agent" or plural "Agents")
- `expected_artifacts()` iterates agents for feature directories and settings files
- Sync command validates all configured agents, not just the first

### Security

- All `install.sh` scripts now use `set -euo pipefail`
- Added `source` file existence guards in proxy and claude-code features
- Fixed unquoted variable in generated bashrc (`configure_claude_provider`)

## [0.3.1] - 2026-03-27

### Fixed

- `expected_artifacts()` now uses config to determine features instead of hardcoded list
- Typo "Isolle" → "Isolde" in generated project README
- `init` error message no longer suggests non-existent `--force` flag
- `validate` command now respects working directory from options instead of hardcoding `current_dir()`

### Removed

- Unused legacy config types (duplicate of `v0_1` module types)
- Unused `print_error` helper from CLI

## [0.3.0] - 2026-03-24

### Added

- Codex agent support with install.sh, version pinning, and proxy support
- `--yes` flag for non-interactive `init` command
- Template placeholder validation after rendering (detects unresolved `{{...}}`)
- `GenerationGuard` for automatic rollback on generation failure
- CONTRIBUTING.md and GitHub issue templates
- CI/CD pipeline: Rust CI (lint, test, build) and release workflow with binary artifacts

### Changed

- Extracted shared rendering logic to `devcontainer.rs` module (sync/generator deduplication)
- Rewrote documentation: README, backlog, architecture, testing docs

### Fixed

- `exec` command: handle `group_spawn()` result instead of silently discarding it
- `run` command: safe container ID slicing to prevent panic on short IDs
- `ps` command: Docker JSONL parsing for Docker 20.10+ compatibility
- Stub agent warnings for unimplemented agents (gemini, aider)
- `--lang-version` flag respected by all templates
- E2E tests revived (9 active scenarios)

## [0.2.0] - 2026-03-05

### Added

- Container management commands: `build`, `run`, `exec`, `stop`, `ps`, `logs`
- Generic agent selection framework replacing hardcoded Claude Code
- Structured YAML map support for `agent.options.models` in `isolde.yaml`
- Schema versioning for `isolde.yaml` configuration
- Detailed `version` command with build metadata (`-v`/`-vv`/`-vvv`)
- `make install` target for installing binary to `~/.local/bin/`
- Comprehensive E2E test suite with Behave
- Project directory mount in `devcontainer.json` for all templates and sync

### Changed

- Replaced hardcoded Claude Code with generic agent system supporting multiple coding agents
- Renamed `mk/rust.mk` to `mk/build.mk` and standardized build target names
- Consolidated install targets and removed cargo install path
- Model configuration commented out by default in generated `isolde.yaml`

### Fixed

- Bind-mount ownership by delegating user creation to `common-utils` feature
- Project directory creation during `sync` if absent
- User/group handling when existing group conflicts in `claude-code` feature
- Proxy handling in feature install scripts
- Documentation inconsistencies aligned with actual CLI behavior

### Removed

- Obsolete artifacts and disabled modules
- Leftover `isolde.yaml` from project root

## [0.1.0] - 2026-02-17

### Added

- Rust CLI implementation of Isolde (replacing shell script prototype)
- Multi-project template system with language templates (Python, Node.js, Rust, Go, generic)
- Preset configurations via `presets.yaml`
- `init` command with interactive wizard and direct template/preset selection
- `sync` command to generate devcontainer config from `isolde.yaml`
- `validate` command for project configuration validation
- `diff` command to show differences from template
- `doctor` command for diagnostics
- Plugin manager with activation/deactivation support
- Multi-provider LLM support in `claude-code` feature (Anthropic, OpenRouter, AWS Bedrock, GCP Vertex)
- Standalone proxy feature with shared state for enterprise networks
- Feature install order control via `overrideFeatureInstallOrder`
- Version control for Claude Code feature installation
- Comprehensive project documentation
- GitHub Actions CI pipeline with shell/JSON linting, Bats tests, and Docker tests
- Makefile-based build system

### Changed

- Migrated from shell scripts to Rust workspace (`isolde-core` + `isolde-cli`)
- Reorganized devcontainer into `images/` directory structure
- Features copied (not symlinked) to projects for Docker build compatibility
- Proxy configuration unified with single source of truth

### Fixed

- Workspace folder path using VSCode standard `/workspaces`
- Provider variable issues in devcontainer feature
- Claude Code setup wizard prevention on container rebuild
- User detection and UID/GID handling in devcontainer

[Unreleased]: https://github.com/jubbon/isolde/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/jubbon/isolde/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/jubbon/isolde/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/jubbon/isolde/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jubbon/isolde/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jubbon/isolde/releases/tag/v0.1.0
