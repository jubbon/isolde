# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Language Policy

**ALL documentation and code MUST be in English.** This project is open-sourced on GitHub.

- All documentation files (CLAUDE.md, README.md, docs/, etc.) must be written in English
- All commit messages must be in English
- All code comments should be in English
- All user-facing text must be in English
- This policy applies to all markdown, documentation, and text files in the repository

## Project Overview

**Isolde (ISOLated Development Environment)** - A template-based system for creating isolated development environments with Claude Code CLI support.

### Key Components

- **Template System** (`templates/`, `scripts/`) - Language templates and project creation scripts
- **Shared Features** (`core/features/`) - Reusable devcontainer features (claude-code, proxy)
- **Self Devcontainer** (`.devcontainer/`) - Devcontainer setup for developing this repository
- **Documentation** (`docs/`, `.devcontainer/docs/`) - User and developer documentation

## Documentation

### Template System Documentation
| Document | Description |
|----------|-------------|
| [docs/README.md](../docs/README.md) | Template system documentation index |
| [docs/usage.md](../docs/usage.md) | How to use init-project.sh |
| [docs/templates.md](../docs/templates.md) | Template customization |
| [docs/presets.md](../docs/presets.md) | Available and custom presets |
| [docs/backlog.md](../docs/backlog.md) | Planned features and improvements |

### Devcontainer Documentation
| Document | Description |
|----------|-------------|
| [.devcontainer/docs/README.md](README.md) | Devcontainer documentation index |
| [.devcontainer/docs/setup.md](setup.md) | Installation and configuration |
| [.devcontainer/docs/architecture.md](architecture.md) | System architecture and design |
| [.devcontainer/docs/development.md](development.md) | Contributing and workflow |
| [.devcontainer/docs/providers.md](providers.md) | Multi-provider LLM support |
| [.devcontainer/docs/proxy.md](proxy.md) | Proxy configuration |
| [.devcontainer/docs/claude-version-control.md](claude-version-control.md) | Version control options |

## Quick Reference

### Creating New Projects
```bash
# From repository root
./scripts/init-project.sh                    # Interactive wizard
./scripts/init-project.sh my-app --preset=python-ml
./scripts/init-project.sh api --template=nodejs --lang-version=22
```

### Development Workflow
```bash
# Build devcontainer image
docker build -t claude-code-dev .devcontainer

# Rebuild in VS Code: F1 → Dev Containers: Rebuild Container

# Verify inside container
claude --version
docker ps  # Test DinD
```

### Git Worktrees
```bash
# Add worktree for isolated feature branch
git worktree add ../feature-branch-name branch-name

# Remove worktree when done
git worktree remove ../feature-branch-name
```

### Testing
```bash
# Test container builds
docker build -t claude-code-dev .devcontainer

# Test shell scripts (if shellcheck is installed)
shellcheck scripts/init-project.sh
shellcheck .devcontainer/features/claude-code/install.sh

# Validate JSON files (if jq is available)
jq < .devcontainer/devcontainer.json
```

## Repository Structure

```
claude-code-templates/
├── core/                    # Shared components
│   ├── features/            # Reusable devcontainer features
│   │   ├── claude-code/     # Claude Code CLI installation
│   │   └── proxy/           # HTTP proxy configuration
│   └── base-Dockerfile      # Base container image
├── templates/               # Language templates
│   ├── python/
│   ├── nodejs/
│   ├── rust/
│   ├── go/
│   └── generic/
├── scripts/                 # Project creation tools
│   ├── init-project.sh      # Main script
│   └── lib/                 # Helper libraries
│       ├── git.sh           # Git operations
│       ├── presets.sh       # Preset management
│       ├── templates.sh     # Template processing
│       ├── ui.sh            # Interactive UI
│       └── utils.sh         # Utility functions
├── presets.yaml             # Built-in presets
├── docs/                    # Template system documentation
├── .devcontainer/           # Self devcontainer setup
│   ├── devcontainer.json    # Main config
│   ├── Dockerfile           # Base image
│   ├── docs/                # Devcontainer documentation
│   └── features/            # Feature implementations
├── .claude/                 # Claude Code local settings (not in git)
├── CLAUDE.md                # This file
└── README.md                # Project overview
```

## Project Structure (Created Projects)

Projects created with `init-project.sh` have this structure:

```
~/workspace/my-project/
├── project/              # Git repository for your code
│   ├── README.md
│   └── .gitignore
├── .devcontainer/        # Git repository for devcontainer config
│   ├── devcontainer.json
│   ├── Dockerfile
│   ├── features/         # Copied from core/features/
│   └── README.md
└── .claude/              # Claude Code config (not in git)
```

## Available Templates

| Template | Description | Versions |
|----------|-------------|----------|
| **python** | Python with uv, pytest, ruff | 3.12, 3.11, 3.10 |
| **nodejs** | Node.js with TypeScript, ESLint, Vitest | 22, 20, 18 |
| **rust** | Rust with cargo, clippy | latest, stable |
| **go** | Go with golangci-lint | latest, 1.22, 1.21 |
| **generic** | Minimal container for any project | - |

## Available Presets

| Preset | Template | Description |
|--------|----------|-------------|
| **python-ml** | Python 3.12 | ML with Jupyter, numpy, pandas |
| **python-web** | Python 3.11 | Web with pytest, ruff |
| **node-api** | Node.js 22 | API with TypeScript, Vitest |
| **rust-cli** | Rust | CLI with clippy |
| **go-service** | Go | Microservice with golangci-lint |
| **minimal** | Generic | Minimal setup |
| **fullstack** | Node.js 22 | Full-stack with security plugins |

## Version Control

Control which Claude Code CLI version is installed via `devcontainer.json`:

| Version | Behavior |
|---------|-----------|
| `latest` (default) | Most recent release, auto-updates enabled |
| `stable` | Latest stable release, auto-updates disabled |
| `1.2.41` | Specific version, auto-updates disabled |

```json
{
  "features": {
    "./features/claude-code": {
      "version": "latest"
    }
  }
}
```

## Multi-Provider LLM Support

### Provider Directory Structure
```
~/.claude/providers/
├── anthropic/    # Default (uses ~/.claude/auth)
├── z.ai/         # Zhipu AI
└── custom/       # Your custom provider
    ├── auth       # API token
    └── base_url   # API endpoint (optional)
```

### Configuration in devcontainer.json
```json
{
  "features": {
    "./features/claude-code": {
      "provider": "z.ai",
      "models": "haiku:model,sonnet:model,opus:model"
    }
  }
}
```

## Proxy Architecture

Proxy settings are separated by scope:

| Scope | Proxy Used | Reason |
|-------|-------------|---------|
| Dockerfile Build | No | Direct Debian package downloads |
| Feature Installation | Yes | Claude Code download requires proxy |
| Container Runtime | Yes | API calls to LLM provider |

## Git Commit Standards

**CRITICAL: All commits must be atomic.**

### Pre-Commit Verification

**Before committing, verify the devcontainer builds and environment is correct:**

1. **Build test:**
```bash
docker build -t claude-code-dev .devcontainer
```

2. **Environment verification** (after container starts):
```bash
# Verify critical variables are set:
echo $HTTP_PROXY
echo $HTTPS_PROXY
echo $NO_PROXY

# Verify Anthropic provider variables:
echo $ANTHROPIC_AUTH_TOKEN
echo $ANTHROPIC_BASE_URL  # if custom provider is used
```

If build fails or environment variables are not correctly set, fix the issues before committing.

### Atomic Commits Rule

- **One logical change per commit** - Each commit must contain only logically related changes
- **No unrelated changes** - Never bundle unrelated fixes, refactoring, or features in a single commit
- **Minimal scope** - A commit should do exactly one thing and do it completely

### Commit Message Format

- **English only** - All commit messages must be written in English
- **Conventional commits** - Use format: `<type>: <description>`
  - Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
  - Description: lowercase, no period at end
- **Imperative mood** - Use "add" not "added", "fix" not "fixed"

### Commit Chain Ordering

When multiple changes are needed, commits must form a proper dependency chain:
- **Dependencies first** - If change B depends on change A, commit A before B
- **Buildable state** - Each commit should leave the codebase in a working state when possible
- **Reversible** - Each commit should be independently revertible without breaking unrelated functionality

## Development Workflow

1. **Initial Setup**: Open in VS Code and use Dev Containers extension
2. **Provider Configuration**: Set `provider` option in `devcontainer.json`
3. **API Tokens**: Store in `~/.claude/providers/{provider}/auth`
4. **Feature Development**: Use worktrees for isolated development branches
5. **Testing**: Verify container builds and environment variables

## Important Notes

### Features are Copied, Not Symlinked

Features are **copied** from `core/features/` to project directories because Docker cannot follow symlinks outside the build context. Each project gets its own copy of the features.

### No Makefile

This project does **not** have a Makefile. Use Docker commands directly:
- Build: `docker build -t claude-code-dev .devcontainer`
- Rebuild: VS Code → F1 → Dev Containers: Rebuild Container

### Script Libraries

The `scripts/lib/` directory contains shared functions for:
- **git.sh** - Git repository initialization and management
- **presets.sh** - Preset loading and validation
- **templates.sh** - Template processing and substitution
- **ui.sh** - Interactive menus and prompts
- **utils.sh** - Common utility functions

When adding new features to `init-project.sh`, first check if a helper function already exists in these libraries.

### Template Placeholders

Templates support these placeholders in `devcontainer.json`:
- `{{PROJECT_NAME}}` - Project name
- `{{PYTHON_VERSION}}`, `{{NODE_VERSION}}`, etc. - Language versions
- `{{FEATURES_CLAUDE_CODE}}`, `{{FEATURES_PROXY}}` - Feature paths
- `{{CLAUDE_VERSION}}`, `{{CLAUDE_PROVIDER}}`, `{{CLAUDE_MODELS}}` - Claude config
- `{{HTTP_PROXY}}`, `{{HTTPS_PROXY}}`, `{{NO_PROXY}}` - Proxy settings

## Troubleshooting

### Provider Issues
- Check if provider directory exists: `ls -la ~/.claude/providers/`
- Verify API token is present: `cat ~/.claude/providers/{provider}/auth`
- Ensure ANTHROPIC_AUTH_TOKEN is set: `echo $ANTHROPIC_AUTH_TOKEN`

### Proxy Issues
- Confirm proxy is configured for runtime: `echo $HTTP_PROXY`
- Check feature installation used correct proxy: `curl -v https://claude.ai/install.sh`
- Verify no proxy conflicts in build vs runtime

### Container Won't Start
- Check Docker is running: `docker ps`
- Verify `devcontainer.json` syntax: `cat .devcontainer/devcontainer.json | jq`
- Review build logs in VS Code Output panel

## Script Development

When modifying `scripts/init-project.sh` or library files:

1. **Test interactively**: Run `./scripts/init-project.sh test-project --template=python`
2. **Verify output**: Check the created project structure
3. **Test all templates**: Ensure changes work with all templates
4. **Update documentation**: If adding new options, update relevant docs

### Adding a New Template

1. Create directory: `mkdir templates/my-language/.devcontainer`
2. Create `template-info.yaml` with metadata
3. Create `Dockerfile` based on `core/base-Dockerfile`
4. Create `devcontainer.json` with placeholders
5. Add template processing logic to `scripts/lib/templates.sh` if needed
6. Test: `./scripts/init-project.sh test --template=my-language`

### Adding a New Preset

1. Add entry to `presets.yaml`
2. Test: `./scripts/init-project.sh test --preset=my-preset`
3. Update `docs/presets.md` if user-facing

## Related Projects

- [Dev Containers Specification](https://devcontainers.github.io/implementors/spec/)
- [Claude Code Documentation](https://code.claude.com/docs)
- [oh-my-claudecode](https://github.com/anthropics/oh-my-claudecode) - Multi-agent orchestration (configured in `.claude/CLAUDE.md`)
