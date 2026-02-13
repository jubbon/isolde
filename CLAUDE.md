# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Claude Code Dev Container - a Docker-based development environment that provides isolated containerized environments for using Claude Code CLI. The project supports multiple LLM providers and includes proxy support for enterprise environments.

## Common Commands

### Development Setup
```bash
# Build the Dev Container image
cd .devcontainer && docker build -t claude-code-dev .

# Run in development mode (interactive)
make devcontainer

# Get a shell in the container
make shell

# Clean up running containers
make clean
```

### Version Control

Control which Claude Code CLI version is installed in your devcontainer.

| Version | Behavior |
|---------|-----------|
| `latest` (default) | Most recent release, auto-updates enabled |
| `stable` | Latest stable release, auto-updates disabled |
| `1.2.41` | Specific version, auto-updates disabled |

#### Use Specific Version
```json
{
  "features": {
    "claude-code": {
      "version": "1.2.41"
    }
  }
}
```

#### Use Latest (Default)
```json
{
  "features": {
    "claude-code": {
      // "version": "latest"  // or omit entirely
    }
  }
}
```

See also: [Version Control Documentation](docs/claude-version-control.md)

### Git Operations
```bash
# The project uses git with worktrees for feature branches
git worktree add ../feature-branch-name branch-name
git worktree remove ../feature-branch-name
```

### Docker Operations (Docker-in-Docker)
```bash
# Build images
docker build -t my-app .

# Run containers
docker run -it my-app

# List containers
docker ps

# Stop containers
docker stop <container-id>
```

## Architecture

### Dev Container Structure
- **`.devcontainer/`** - Dev Container configuration
  - `devcontainer.json` - Main configuration with proxy settings and features
  - `Dockerfile` - Base image definition (Debian + system deps)
  - `features/claude-code/` - Custom feature for Claude Code installation
    - `devcontainer-feature.json` - Feature definition with provider options
    - `install.sh` - Installation script with multi-provider support

### Multi-Provider LLM Support
The project supports multiple LLM providers through a configuration system:

1. **Provider Configuration Location**: `~/.claude/providers/{provider}/`
   - `auth` - API token file
   - `base_url` - API endpoint URL (optional)

2. **Current Provider Setup**:
   - Default: Anthropic (reads from `~/.claude/auth`)
   - Z.ai/GLM provider configured in example devcontainer.json
   - Custom providers can be added by creating the directory structure

3. **Provider Selection**:
   - Set in `devcontainer.json` via the `provider` option
   - Configuration is automatically written to `~/.bashrc` for shell sessions
   - The `configure_claude_provider()` function handles provider loading

### Proxy Architecture
Proxy settings are separated by scope:
- **Docker Build**: No proxy (direct package downloads)
- **Feature Installation**: Proxy used for Claude Code download
- **Container Runtime**: Proxy used for API calls

Configuration in `devcontainer.json`:
```json
"containerEnv": {
  "HTTP_PROXY": "http://192.168.1.21:2080",
  "HTTPS_PROXY": "http://192.168.1.21:2080",
  "NO_PROXY": "localhost,127.0.0.1,.local"
}
```

### User Mapping
The installation script handles both root and non-root users:
- Auto-detects target user from environment variables
- Creates user with sudo access if needed
- Sets proper ownership for configuration files
- Installs Claude Code in user's home directory

## Configuration Files

### Claude Code Settings
Local settings are in `.claude/settings.local.json` with permissions configured for:
- Bash commands (limited to specific safe operations)
- Docker operations
- Git operations
- Web search
- MCP plugins (context7, oh-my-claudecode)

### VS Code Integration
Recommended extensions:
- `anthropic.claude-code` - Claude Code extension
- `ms-azuretools.vscode-docker` - Docker support
- `ms-python.python` - Python support

## Important Implementation Details

### Bash Configuration
Provider configuration is added to `~/.bashrc` rather than `/etc/profile.d` because:
- VS Code terminals use non-login shells
- `~/.bashrc` is always sourced in interactive sessions
- Configuration includes a `configure_claude_provider()` function

### Provider Loading Logic
```bash
configure_claude_provider() {
    local provider=$1
    local provider_dir="$HOME/.claude/providers/$provider"

    if [ -z "$provider" ]; then
        # Use default Anthropic auth
        if [ -f "$HOME/.claude/auth" ]; then
            export ANTHROPIC_AUTH_TOKEN="$(cat "$HOME/.claude/auth")"
        fi
    else
        # Load from provider directory
        if [ -f "$provider_dir/auth" ]; then
            export ANTHROPIC_AUTH_TOKEN="$(cat "$provider_dir/auth")"
        fi
        if [ -f "$provider_dir/base_url" ]; then
            export ANTHROPIC_BASE_URL="$(cat "$provider_dir/base_url" | tr -d '\n\r ')"
        fi
    fi
}
```

### Docker Socket Mount
The container mounts `/var/run/docker.sock` for Docker-in-Docker support, enabling:
- Building containers within the dev container
- Running docker commands without sudo

## Development Workflow

1. **Initial Setup**: Use VS Code Dev Containers extension or run `make devcontainer`
2. **Provider Configuration**: Set `provider` option in `devcontainer.json` or configure manually
3. **API Tokens**: Store in `~/.claude/providers/{provider}/auth`
4. **Feature Development**: Use worktrees for isolated development branches
5. **Testing**: Run tests within the container environment

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
# Start container and verify environment variables
make devcontainer
# Or: docker run -it --rm claude-code-dev bash

# Inside container, verify critical variables are set:
echo $HTTP_PROXY
echo $HTTPS_PROXY
echo $NO_PROXY

# Verify Anthropic provider variables (after configure_claude_provider):
echo $ANTHROPIC_AUTH_TOKEN
echo $ANTHROPIC_BASE_URL  # if custom provider is used

# Verify Anthropic model configuration:
echo $ANTHROPIC_MODELS
```

If the build fails or environment variables are not correctly set, fix the issues before committing. Never commit broken configuration.

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

### Proxy Issues
- Confirm proxy is configured for runtime: `echo $HTTP_PROXY`
- Check feature installation used correct proxy: `curl -v https://claude.ai/install.sh`
- Verify no proxy conflicts in build vs runtime

### Permission Issues
- Ensure user has Docker access on host
- Check `.claude` directory permissions: `ls -la ~/.claude/`
- Verify config file ownership after installation