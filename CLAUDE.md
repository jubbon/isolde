# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Documentation

For detailed project documentation, see the [.devcontainer/docs/](.devcontainer/docs/) directory:

| Document | Description |
|----------|-------------|
| [.devcontainer/docs/README.md](.devcontainer/docs/README.md) | Documentation index and navigation |
| [.devcontainer/docs/setup.md](.devcontainer/docs/setup.md) | Installation and configuration guide |
| [.devcontainer/docs/architecture.md](.devcontainer/docs/architecture.md) | System architecture and design |
| [.devcontainer/docs/development.md](.devcontainer/docs/development.md) | Contributing and development workflow |
| [.devcontainer/docs/proxy.md](.devcontainer/docs/proxy.md) | Proxy configuration for enterprise |
| [.devcontainer/docs/providers.md](.devcontainer/docs/providers.md) | LLM provider setup |
| [.devcontainer/docs/claude-version-control.md](.devcontainer/docs/claude-version-control.md) | Version control options |

## Quick Reference

### Development Commands
```bash
# Build Dev Container image
cd .devcontainer && docker build -t claude-code-dev .

# Run in development mode (interactive)
make devcontainer

# Get a shell in container
make shell

# Clean up running containers
make clean
```

### Git Worktrees
```bash
# Add worktree for isolated feature branch
git worktree add ../feature-branch-name branch-name

# Remove worktree when done
git worktree remove ../feature-branch-name
```

### Docker-in-Docker
```bash
# Build images
docker build -t my-app .

# Run containers
docker run -it my-app

# List containers
docker ps
```

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

See: [.devcontainer/docs/claude-version-control.md](.devcontainer/docs/claude-version-control.md)

## Architecture Overview

### Dev Container Structure
```
.devcontainer/
├── devcontainer.json       # Main config (proxy, mounts, features)
├── Dockerfile              # Base image (Debian + system deps)
└── features/
    └── claude-code/        # Custom Claude Code feature
        ├── devcontainer-feature.json
        ├── install.sh
        └── README.md
```

### Multi-Provider LLM Support

```
~/.claude/providers/
├── anthropic/    # Default (uses ~/.claude/auth)
├── z.ai/         # Zhipu AI
└── custom/       # Your custom provider
    ├── auth       # API token
    └── base_url   # API endpoint (optional)
```

**Provider Selection:** Set `provider` option in `devcontainer.json`

See: [.devcontainer/docs/providers.md](.devcontainer/docs/providers.md)

### Proxy Architecture

Proxy settings are separated by scope:

| Scope | Proxy Used | Reason |
|-------|-------------|---------|
| Dockerfile Build | No | Direct Debian package downloads |
| Feature Installation | Yes | Claude Code download requires proxy |
| Container Runtime | Yes | API calls to LLM provider |

See: [.devcontainer/docs/proxy.md](.devcontainer/docs/proxy.md)

## Development Workflow

1. **Initial Setup**: Use VS Code Dev Containers extension or run `make devcontainer`
2. **Provider Configuration**: Set `provider` option in `devcontainer.json`
3. **API Tokens**: Store in `~/.claude/providers/{provider}/auth`
4. **Feature Development**: Use worktrees for isolated development branches
5. **Testing**: Run tests within container environment

## Git Commit Standards

**CRITICAL: All commits must be atomic.**

### Pre-Commit Verification

**Before committing, verify the devcontainer builds and environment is correct:**

1. **Build test:**
```bash
cd .devcontainer && docker build -t claude-code-dev .
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

# Verify Anthropic model configuration:
echo $ANTHROPIC_MODELS
```

If build fails or environment variables are not correctly set, fix the issues before committing.

### Atomic Commits Rule

- **One logical change per commit** - Each commit must contain only logically related changes
- **No unrelated changes** - Never bundle unrelated fixes, refactoring, or features in a single commit
- **Minimal scope** - A commit should do exactly one thing and do it completely

### Commit Message Format

- **English only** - All commit messages must be written in English language
- **Conventional commits** - Use format: `<type>: <description>`
  - Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
  - Description: lowercase, no period at end
- **Imperative mood** - Use "add" not "added", "fix" not "fixed"

### Commit Chain Ordering

When multiple changes are needed, commits must form a proper dependency chain:
- **Dependencies first** - If change B depends on change A, commit A before B
- **Buildable state** - Each commit should leave the codebase in a working state when possible
- **Reversible** - Each commit should be independently revertible without breaking unrelated functionality

### Examples

**Good atomic commits:**
```
feat: add provider configuration system
fix: resolve proxy timeout issue
docs: update installation instructions
```

**Bad - multiple unrelated changes:**
```
feat: add provider config and fix proxy bug and update docs
```

**Bad - wrong order (depends on later commit):**
```
feat: use provider config function (requires function that doesn't exist yet)
feat: add provider config function (should be committed first)
```

## Troubleshooting

### Provider Issues
- Check if provider directory exists: `ls -la ~/.claude/providers/`
- Verify API token is present: `cat ~/.claude/providers/{provider}/auth`
- Ensure ANTHROPIC_AUTH_TOKEN is set: `echo $ANTHROPIC_AUTH_TOKEN`

See: [.devcontainer/docs/providers.md](.devcontainer/docs/providers.md#troubleshooting)

### Proxy Issues
- Confirm proxy is configured for runtime: `echo $HTTP_PROXY`
- Check feature installation used correct proxy: `curl -v https://claude.ai/install.sh`
- Verify no proxy conflicts in build vs runtime

See: [.devcontainer/docs/proxy.md](.devcontainer/docs/proxy.md#troubleshooting)

### Permission Issues
- Ensure user has Docker access on host
- Check `.claude` directory permissions: `ls -la ~/.claude/`
- Verify config file ownership after installation
