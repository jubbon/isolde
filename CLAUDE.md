# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Language Policy

**ALL documentation MUST be in English.** This project is intended to be open-sourced on GitHub.

- All documentation files (CLAUDE.md, README.md, docs/, etc.) must be written in English
- All commit messages must be in English
- All code comments should be in English
- This policy applies to all markdown, documentation, and text files in the repository

## Project Overview

**Isolde (ISOLated Development Environment)** - A Rust CLI tool for creating isolated development environments with Claude Code CLI support. The project uses a workspace structure:

1. **isolde-core** - Core library with template processing, git operations, and configuration
2. **isolde-cli** - Command-line interface built with clap
3. **Template System** (`templates/`, `presets.yaml`) - Language templates and preset configurations
4. **Core Features** (`core/features/`) - Reusable devcontainer features

## Commands

### Building the CLI
```bash
# Development build (faster)
make build/dev
# or
cargo build

# Release build
make build/release
# or
cargo build --release

# Run directly
cargo run -- --help
```

### Creating New Projects
```bash
# Build and install first
make install

# Interactive wizard
isolde init

# Direct command with template
isolde init my-app --template python

# Using presets
isolde init my-app --preset python-ml

# List available templates and presets
isolde init --list-templates
isolde init --list-presets
```

**Note:** The `pull` command is currently disabled (requires async/network support). Use `sync` to update from local templates.

### Sync Command

`isolde sync` generates devcontainer configuration from `isolde.yaml` in the current directory:

```bash
isolde sync               # Generate configuration
isolde sync --dry-run     # Preview without writing
isolde sync --force       # Overwrite existing files
```

This generates:
- `.devcontainer/devcontainer.json` — devcontainer spec with features and mounts
- `.devcontainer/Dockerfile` — Dockerfile with base image and user setup
- `.devcontainer/features/` — copies of `core/features/*` (required because Docker cannot follow symlinks)
- `.claude/CLAUDE.md` — project-specific guidance for Claude Code
- `project/` — workspace directory for bind mount (created if absent)

### Validation and Diagnostics
```bash
# Validate project configuration
isolde validate
isolde validate --quick
isolde validate --verbose
isolde validate --format text|json|sarif

# Show differences from template
isolde diff
isolde diff --file FILE
isolde diff --format diff|json|color

# Run diagnostics
isolde doctor
isolde doctor --fix
isolde doctor --verbose
isolde doctor --component COMPONENT
```

### Container Management

After running `isolde init` and `isolde sync`, you can build and run the devcontainer:

```bash
# Build the devcontainer image
isolde build

# Build without cache
isolde build --no-cache

# Build with custom image name
isolde build --image-name myproject:latest

# Run the devcontainer (start and enter shell)
isolde run

# Run without attaching (background mode)
isolde run --detach

# Execute a command in a running container
isolde exec python --version
isolde exec pytest
isolde exec bash -c "echo hello"

# Stop a running container
isolde stop

# List running containers
isolde ps

# List all containers (including stopped)
isolde ps --all

# View container logs
isolde logs

# Follow logs (like tail -f)
isolde logs --follow

# Show last 50 lines
isolde logs --tail 50
```

**Note:** These commands require the [devcontainers CLI](https://github.com/devcontainers/cli) to be installed.

### Development Workflow
```bash
# Format code
make build/fmt
# or
cargo fmt

# Run linter
make build/lint
# or
cargo clippy

# Run tests
make build/test
# or
cargo test

# Run all checks
make build/check
```

### Testing
```bash
# Test container builds
make test-build

# Run E2E tests
make test-e2e

# Run specific E2E scenario
SCENARIO='basic_init' make test-e2e
```

## Architecture

### Rust Workspace Structure

```
isolde/
├── isolde-core/           # Core library
│   ├── src/
│   │   ├── templates.rs   # Template loading and processing
│   │   ├── git.rs         # Git operations
│   │   ├── config.rs      # Configuration and presets
│   │   └── features.rs    # Feature copying and management
│   └── Cargo.toml
├── isolde-cli/            # CLI binary
│   ├── src/
│   │   └── main.rs        # Entry point with clap CLI
│   └── Cargo.toml
├── Cargo.toml             # Workspace config
├── templates/             # Language templates
├── core/features/         # Devcontainer features
└── presets.yaml           # Preset configurations
```

### Template Application Flow

When `isolde init` creates a project:

1. **CLI Parsing** - clap parses command-line arguments
2. **Template Selection** - User selects template or preset via interactive prompt or args
3. **Template Loading** - `isolde-core::templates` loads `template-info.yaml`
4. **Feature Copy** - `core/features/*` copied to `.devcontainer/features/` (Docker cannot follow symlinks)
5. **Substitution** - Placeholders in `devcontainer.json` replaced via Rust template engine
6. **Git Init** - Git repository created for the project

**Note:** `.gitignore` files are **not** created automatically. Users manage their own `.gitignore` files based on their needs.

### User UID Handling in Generated Devcontainers

The base devcontainer image (`mcr.microsoft.com/devcontainers/base:ubuntu`) includes a `vscode` user at UID 1000. Without intervention, the local user (e.g. `dmanakulikov`, also UID 1000) would appear as `vscode` inside the container on bind-mounted directories, causing permission errors.

The generated Dockerfile and devcontainer.json address this:

1. **Dockerfile** — deletes `vscode` user/group if `USERNAME != vscode`, freeing UID 1000
2. **`common-utils` feature** — creates the local user with `userUid: "automatic"` and `userGid: "automatic"`, matching the host UID/GID
3. **`updateRemoteUserUID`** (devcontainers default: on) — adjusts the container user's UID to match the host after build

Result: bind-mounted `project/` directory has the same owner on host and inside the container.

### Template Metadata Format

Each template has `template-info.yaml`:
```yaml
name: Python
description: Python development environment with uv, pytest, and ruff
version: 1.0.0
lang_version_default: "3.12"
features:
  - name: uv
    description: Fast Python package installer
supported_versions:
  - "3.12"
  - "3.11"
  - "3.10"
```

### Template Substitutions

`devcontainer.json` templates support these placeholders:

| Placeholder | Description |
|-------------|-------------|
| `{{PROJECT_NAME}}` | Project name |
| `{{PYTHON_VERSION}}`, `{{NODE_VERSION}}`, etc. | Language version from `--lang-version` |
| `{{FEATURES_CLAUDE_CODE}}` | Replaced with `./features/claude-code` |
| `{{FEATURES_PROXY}}` | Replaced with `./features/proxy` |
| `{{CLAUDE_VERSION}}` | From `--claude-version` or `latest` |
| `{{CLAUDE_PROVIDER}}` | From `--claude-provider` |
| `{{HTTP_PROXY}}`, `{{HTTPS_PROXY}}` | From proxy options |

Substitutions happen in Rust via the `templates` module.

### Single Git Repository Pattern

Created projects have **a single git repository** that includes:
- User code
- Devcontainer configuration
- Template files

This approach allows:
- Simplified version control with code and config together
- Complete history of changes in one repository
- Easier collaboration with team members
- Simplified deployment and CI/CD workflows

### Core Features

`core/features/` contains reusable devcontainer features:
- `claude-code/` - Claude Code CLI installation with multi-provider support
- `proxy/` - HTTP proxy configuration for enterprise networks
- `plugin-manager/` - Plugin activation and management

Features are **copied** (not symlinked) to each project because Docker's build context cannot follow symlinks outside the build directory.

## Important Notes

### .gitignore Files
Isolde **does not create** `.gitignore` files automatically. Users must create and manage their own `.gitignore` files based on their project needs. This design choice gives users full control over what gets ignored in their repositories.

### Feature Path Resolution
In created projects, features are referenced as `./features/claude-code` (relative to `.devcontainer/`), not via absolute paths from the template repository.

### Preset Format
`presets.yaml` defines preset configurations. When adding a new preset, follow the existing format and include `template`, `lang_version`, `features`, and optional `claude_plugins`.

### CLI Development
When modifying the CLI:
1. Test with: `cargo run -- init test-project --template python`
2. Verify created project structure
3. Test all templates if changes affect template processing
4. Update relevant documentation in `docs/`

## Documentation Locations

- Template system: `docs/README.md`, `docs/user/quick-start.md`, `docs/user/usage.md`, `docs/user/templates.md`, `docs/user/presets.md`
- Devcontainer setup: `docs/devcontainer/setup.md`, `docs/devcontainer/providers.md`, `docs/devcontainer/proxy.md`, `docs/devcontainer/version-control.md`
- Architecture: `docs/contributor/architecture.md`
- Development: `docs/contributor/development.md`, `docs/contributor/testing.md`, `docs/contributor/adding-templates.md`
