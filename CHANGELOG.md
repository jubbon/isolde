# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/jubbon/isolde/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/jubbon/isolde/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/jubbon/isolde/releases/tag/v0.1.0
