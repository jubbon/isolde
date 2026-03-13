# Backlog

This file tracks planned features, improvements, and technical debt for the Isolde CLI.

## Completed

- [x] **Standalone Rust CLI** - Migrated from shell scripts to a Rust workspace (`isolde-core` + `isolde-cli`)
- [x] **Unit test suite** - 103 tests covering core library and CLI
- [x] **Fix symlinks issue** - Copy features instead of creating symlinks (Docker cannot follow symlinks outside build context)
- [x] **Duplicate proxy settings** - Add proxy settings to both `proxy` and `claude-code` features
- [x] **Add Node.js to all templates** - Node.js and npx are required by Claude Code
- [x] **Match host paths inside container** - Use full host path as `workspaceFolder`
- [x] **Dev branch workflow** - Development on `dev`, stable releases on `main`
- [x] **Isolation levels** - Configurable state sharing between host and container (`none`, `session`, `workspace`, `full`)
- [x] **Container management commands** - `isolde build`, `run`, `stop`, `ps`, `logs`, `exec`
- [x] **Sync command** - `isolde sync` generates devcontainer config from `isolde.yaml`
- [x] **Validation and diagnostics** - `isolde validate`, `isolde diff`, `isolde doctor`

## In Progress

- [ ] **v0.3.0 sprint** - Documentation fixes, functional bug fixes, code quality improvements (Mar 13 - Apr 3, 2026)

## Planned Features

### High Priority
- [ ] **Stub agent warnings** - Warn when selecting unimplemented agents (codex, gemini, aider) instead of failing silently
- [ ] **Fix --lang-version** - All templates should respect `--lang-version` parameter (currently some ignore it)
- [ ] **Template placeholder validation** - Detect and warn about unresolved `{{...}}` placeholders after rendering
- [ ] **Sync/generator deduplication** - `sync.rs` partially re-implements `generator.rs`; extract shared logic

### Medium Priority
- [ ] **Self-update command** - `isolde --self-update` to update the CLI binary from the latest release
- [ ] **Codex agent support** - Implement `install.sh` and CLI integration for OpenAI Codex
- [ ] **Rollback on generation failure** - Clean up partial files if project generation fails mid-way
- [ ] **Remove proxy duplication** - Share proxy settings between features without duplication
- [ ] **Better error messages** - Improve error reporting when feature resolution fails
- [ ] **Auto-update templates** - Mechanism to update existing projects with new template changes
- [ ] **Custom template directory** - Allow users to specify custom template locations

### Low Priority
- [ ] **Gemini agent support** - Implement Google Gemini CLI agent
- [ ] **Aider agent support** - Implement Aider AI pair programming agent
- [ ] **Interactive preset editor** - TUI for creating/editing presets
- [ ] **Template versioning** - Support multiple versions of templates
- [ ] **Export/import configurations** - Share project configurations across machines

## Technical Debt

- [ ] **Remove unwrap() from production code** - Replace with proper error handling (`?`, `.map_err()`)
- [ ] **Template engine hardening** - Validate all placeholders are resolved, return errors for missing required values
- [ ] **Revive E2E tests** - Fix ignored tests in `isolde-cli/tests/e2e_tests.rs`
- [ ] **Container module tests** - Add unit tests for `container.rs` parsing functions

## Ideas

- [ ] **Multi-container support** - Docker Compose based templates
- [ ] **GPU support templates** - Preconfigured for CUDA/ROCm
- [ ] **Service integration** - Built-in database, Redis, etc.
- [ ] **CI/CD templates** - GitHub Actions, GitLab CI configs
- [ ] **Pre-built binaries** - Distribute release binaries via GitHub Releases
