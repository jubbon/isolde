# Architecture

This document describes the architecture of the Isolde (ISOLated Development Environment) project.

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         Host Machine                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │    VS Code   │  │  Docker CLI  │  │     Git     │    │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘    │
└─────────┼──────────────────┼──────────────────┼────────────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Dev Container                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │          Claude Code CLI (user workspace)            │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │  │
│  │  │  Config  │  │  Mounts  │  │  Provider Config  │ │  │
│  │  └──────────┘  └──────────┘  └──────────────────┘ │  │
│  └────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────┐  │
│  │              Docker-in-Docker                         │  │
│  │          (/var/run/docker.sock)                       │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    LLM Provider API                          │
│            (Anthropic / Z.ai / Custom)                        │
└─────────────────────────────────────────────────────────────────┘
```

## Components

### 1. Dev Container Configuration

**Location:** `.devcontainer/`

| File | Purpose |
|------|---------|
| `devcontainer.json` | Main configuration - mounts, environment, features |
| `Dockerfile` | Base image definition |
| `PROXY_ARCHITECTURE.md` | Proxy architecture documentation |
| `docs/` | Devcontainer-specific documentation |

**Key Responsibilities:**
- Define container image and build arguments
- Configure volume mounts for workspace and Claude Code config
- Set up environment variables (proxy, models)
- Specify custom features

### 2. Claude Code Feature

**Location:** `core/features/claude-code/` (copied to `.devcontainer/features/` in projects)

| File | Purpose |
|------|---------|
| `devcontainer-feature.json` | Feature definition and options schema |
| `install.sh` | Installation script with multi-provider support |
| `README.md` | Feature-specific documentation |

**Key Responsibilities:**
- Install Claude Code CLI
- Configure provider-specific settings
- Set up bash integration for provider switching
- Handle user mapping for non-root containers

**Provider Loading Flow:**
```
install.sh (build time)
    │
    ▼
Creates ~/.config/devcontainer/provider
    │
    ▼
postCreateCommand updates ~/.bashrc
    │
    ▼
~/.bashrc sources configure_claude_provider()
    │
    ▼
Each shell: loads from ~/.claude/providers/{provider}/
    ├── auth → ANTHROPIC_AUTH_TOKEN
    └── base_url → ANTHROPIC_BASE_URL
```

### 3. Provider System

**Location:** `~/.claude/providers/{provider}/`

Each provider directory contains:

```
~/.claude/providers/
├── anthropic/          (optional, uses ~/.claude/auth by default)
├── z.ai/
│   ├── auth            # API token
│   └── base_url        # API endpoint
└── custom/
    ├── auth
    └── base_url
```

**Configuration Loading:**
```bash
configure_claude_provider() {
    local provider=$1
    local provider_dir="$HOME/.claude/providers/$provider"

    if [ -z "$provider" ]; then
        # Use default Anthropic auth
        export ANTHROPIC_AUTH_TOKEN="$(cat "$HOME/.claude/auth")"
    else
        # Load from provider directory
        export ANTHROPIC_AUTH_TOKEN="$(cat "$provider_dir/auth")"
        [ -f "$provider_dir/base_url" ] && \
            export ANTHROPIC_BASE_URL="$(cat "$provider_dir/base_url")"
    fi
}
```

### 4. Workspace Integration

**Mounts Configuration:**
```json
"mounts": [
  // Claude Code config (shared across containers)
  "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind",

  // Docker socket for DinD
  "source=/var/run/docker.sock,target=/var/run/docker.sock,type=bind",
]
```

**Git Worktrees:**
- Stored in `.worktrees/` directory
- Isolated development branches
- Separate from main workspace

## Data Flow

### Container Startup Flow

```
1. VS Code/CLI initiates container
   │
2. Docker builds/starts container
   │
3. postCreateCommand executes
   │   └──> Writes ~/.config/devcontainer/provider
   │   └──> Updates ~/.bashrc with provider function
   │
4. User opens shell
   │
5. ~/.bashrc sources
   │   └──> configure_claude_provider() runs
   │       └──> Sets ANTHROPIC_AUTH_TOKEN
   │       └──> Sets ANTHROPIC_BASE_URL (if custom provider)
   │
6. Claude Code CLI ready with configured provider
```

### API Request Flow

```
Claude Code CLI
    │
    ▼
Reads ANTHROPIC_AUTH_TOKEN
    │
    ▼
Reads ANTHROPIC_BASE_URL (optional)
    │
    ▼
Makes API request to provider endpoint
    │
    ▼
[HTTP_PROXY] ────> Corporate Proxy ────> LLM Provider API
```

## Design Decisions

### Why ~/.config/devcontainer/provider?

The provider name is stored in `~/.config/devcontainer/` (container-local) rather than `~/.claude` to avoid race conditions when `~/.claude` is mounted between multiple containers.

### Why ~/.bashrc not /etc/profile.d?

VS Code terminals use non-login shells, which source `~/.bashrc` but not `/etc/profile.d`. Storing provider configuration in `~/.bashrc` ensures it's available in all interactive sessions.

### Why Separate Proxy Scopes?

Proxy settings are separated to ensure they're used only where needed:

1. **Dockerfile Build** - No proxy (direct Debian package downloads)
2. **Feature Installation** - With proxy (Claude Code download)
3. **Container Runtime** - With proxy (API calls)

See [PROXY_ARCHITECTURE.md](../PROXY_ARCHITECTURE.md) for details.

### Why Docker-in-Docker?

Mounting `/var/run/docker.sock` enables:
- Building containers within the dev container
- Running docker commands without sudo
- Testing Docker-based projects in isolation

## Security Considerations

### Credential Storage

- API tokens stored in user home directory (`~/.claude/`)
- File permissions respect user umask
- No credentials in container image or logs

### Docker Socket Access

- Socket mounted with same permissions as host user
- No privilege escalation within container
- Host user's Docker group membership required

### Proxy Configuration

- Proxy URLs in `devcontainer.json` (clear text in project)
- Consider environment-specific overrides for sensitive proxy credentials

## Extension Points

### Adding New Features

Create features under `core/features/` following the [Dev Containers Feature specification](https://devcontainers.github.io/implementors/features/).

### Adding New Providers

1. Create provider directory: `~/.claude/providers/{provider}/`
2. Add `auth` file with API token
3. Add `base_url` file with API endpoint (optional)
4. Configure in project's `devcontainer.json`:

```json
{
  "features": {
    "./features/claude-code": {
      "provider": "new-provider"
    }
  }
}
```

### Custom Base Images

Modify the template's Dockerfile or create a custom base image in `core/base-Dockerfile` to add:
- System dependencies
- Base image changes
- Global configuration
