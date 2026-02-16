# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Claude Code Devcontainer Templates** - A template-based system for creating isolated development environments with Claude Code CLI support. The project has two main components:

1. **Template System** (`templates/`, `scripts/`, `presets.yaml`) - Language templates and `init-project.sh` script for creating new projects
2. **Self Devcontainer** (`.devcontainer/`) - Devcontainer setup for developing this repository itself

## Commands

### Creating New Projects
```bash
# From repository root
./scripts/init-project.sh                    # Interactive wizard
./scripts/init-project.sh my-app --preset=python-ml
./scripts/init-project.sh api --template=nodejs --lang-version=22

# List options
./scripts/init-project.sh --list-templates
./scripts/init-project.sh --list-presets
```

### Development Workflow
```bash
# Build devcontainer image
docker build -t claude-code-dev .devcontainer

# Rebuild in VS Code: F1 → Dev Containers: Rebuild Container
# Verify: claude --version && docker ps
```

### Testing
```bash
# Test container builds
docker build -t claude-code-dev .devcontainer

# Test shell scripts (requires shellcheck)
shellcheck scripts/init-project.sh
shellcheck scripts/lib/*.sh

# Validate JSON files (requires jq)
jq < .devcontainer/devcontainer.json
```

## Architecture

### Template Application Flow

When `init-project.sh` creates a project:

1. **Template Selection** - User selects template or preset
2. **Copy Devcontainer** - `templates/{name}/.devcontainer/` copied to project
3. **Feature Copy** - `core/features/*` copied to `.devcontainer/features/` (Docker cannot follow symlinks outside build context)
4. **Substitution** - Placeholders in `devcontainer.json` replaced via `sed`
5. **Git Init** - Two separate git repos created: `project/` and `.devcontainer/`

### Script Library Architecture

`scripts/lib/` contains modular shell libraries sourced by `init-project.sh`:

| File | Responsibility |
|------|----------------|
| `templates.sh` | Template loading, validation, substitution, copying |
| `presets.sh` | Preset loading from YAML, validation |
| `git.sh` | Dual git repo initialization (project + devcontainer) |
| `ui.sh` | Interactive menus, prompts, colored output |
| `utils.sh` | YAML parsing, logging, string sanitization |

**Key Pattern**: Functions use `get_templates_root()` to locate `templates/` and `core/` directories relative to `SCRIPT_DIR`.

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

Substitutions happen in `apply_template_substitutions()` via `sed`.

### Dual Git Repository Pattern

Created projects have **two separate git repositories**:

```
~/workspace/my-project/
├── project/              # Git repo #1 - user code
│   └── .git/
└── .devcontainer/        # Git repo #2 - devcontainer config
    └── .git/
```

This separation allows:
- Independent version control of code vs. devcontainer config
- Easy updates to devcontainer from template repository
- Clean git history for user code

### Core Features

`core/features/` contains reusable devcontainer features:
- `claude-code/` - Claude Code CLI installation with multi-provider support
- `proxy/` - HTTP proxy configuration for enterprise networks

Features are **copied** (not symlinked) to each project because Docker's build context cannot follow symlinks outside the build directory.

## Important Notes

### No Makefile
This project uses Docker directly, not Make. Build commands: `docker build -t claude-code-dev .devcontainer`

### Feature Path Resolution
In created projects, features are referenced as `./features/claude-code` (relative to `.devcontainer/`), not via absolute paths from the template repository.

### Preset Format
`presets.yaml` defines preset configurations. When adding a new preset, follow the existing format and include `template`, `lang_version`, `features`, and optional `claude_plugins`.

### Script Development
When modifying `scripts/init-project.sh` or library files:
1. Test with: `./scripts/init-project.sh test-project --template=python`
2. Verify created project structure
3. Test all templates if changes affect template processing
4. Update relevant documentation in `docs/`

## Documentation Locations

- Template system: `docs/README.md`, `docs/usage.md`, `docs/templates.md`, `docs/presets.md`
- Devcontainer setup: `.devcontainer/docs/README.md`, `.devcontainer/docs/setup.md`
- Architecture: `.devcontainer/docs/architecture.md`
- Provider config: `.devcontainer/docs/providers.md`
- Development: `.devcontainer/docs/development.md`
