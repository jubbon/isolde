# Backlog

This file tracks planned features, improvements, and technical debt for the Isolde CLI.

## Completed

- [x] **Standalone Rust CLI** - Migrated from shell scripts to a Rust workspace (`isolde-core` + `isolde-cli`)
- [x] **Unit test suite** - 185 tests covering core library, CLI, and E2E scenarios
- [x] **Fix symlinks issue** - Copy features instead of creating symlinks (Docker cannot follow symlinks outside build context)
- [x] **Duplicate proxy settings** - Add proxy settings to both `proxy` and `claude-code` features
- [x] **Add Node.js to all templates** - Node.js and npx are required by Claude Code
- [x] **Match host paths inside container** - Use full host path as `workspaceFolder`
- [x] **Dev branch workflow** - Development on `dev`, stable releases on `main`
- [x] **Isolation levels** - Configurable state sharing between host and container (`none`, `session`, `workspace`, `full`)
- [x] **Container management commands** - `isolde build`, `run`, `stop`, `ps`, `logs`, `exec`
- [x] **Sync command** - `isolde sync` generates devcontainer config from `isolde.yaml`
- [x] **Validation and diagnostics** - `isolde validate`, `isolde diff`, `isolde doctor`
- [x] **Stub agent warnings** - Warn when selecting unimplemented agents (gemini, aider) instead of failing silently
- [x] **Fix --lang-version** - All templates respect `--lang-version` parameter
- [x] **Template placeholder validation** - Detect and warn about unresolved `{{...}}` placeholders after rendering
- [x] **Sync/generator deduplication** - Shared rendering logic extracted to `devcontainer.rs` module
- [x] **Codex agent support** - `install.sh` with npm-based installation, version pinning, and proxy support
- [x] **Rollback on generation failure** - `GenerationGuard` cleans up partial files if project generation fails
- [x] **E2E tests revived** - 9 active E2E tests covering CLI version, help, init, templates, presets, validate, sync, diff, doctor
- [x] **Container module tests** - Unit tests for `container.rs` parsing functions
- [x] **Remove unwrap() from production code** - Replaced with proper error handling

## In Progress

- [ ] **v0.3.0 sprint** - Documentation fixes, functional bug fixes, code quality improvements (Mar 13 - Mar 24, 2026)

## Planned Features

### Medium Priority
- [ ] **Self-update command** - `isolde --self-update` to update the CLI binary from the latest release
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

## Ideas

- [ ] **Multi-container support** - Docker Compose based templates
- [ ] **GPU support templates** - Preconfigured for CUDA/ROCm
- [ ] **Service integration** - Built-in database, Redis, etc.
- [ ] **CI/CD templates** - GitHub Actions, GitLab CI configs
- [ ] **Pre-built binaries** - Distribute release binaries via GitHub Releases
