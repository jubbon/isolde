# Proxy Configuration Architecture

## Overview
Proxy settings are managed by a standalone `proxy` feature that creates shared state for other features to consume.

## Components

### 1. Proxy Feature (`./features/proxy`)
- **Files**:
  - `.devcontainer/features/proxy/devcontainer-feature.json` - Feature manifest
  - `.devcontainer/features/proxy/install.sh` - Creates shared state file
  - `.devcontainer/features/proxy/proxy.sh` - Utility functions for consuming features
  - `.devcontainer/features/proxy/README.md` - Feature documentation
- **Behavior**: Creates `~/.config/devcontainer/proxy` state file with proxy configuration
- **Shared State**: `HTTP_PROXY`, `HTTPS_PROXY`, `NO_PROXY` variables
- **Optional**: Apt proxy configuration (`/etc/apt/apt.conf.d/proxy.conf`)

### 2. Consuming Features (e.g., claude-code)
- **File**: `.devcontainer/features/claude-code/install.sh`
- **Behavior**: Sources `~/.config/devcontainer/proxy` if exists
- **Fallback**: Uses deprecated direct options for backward compatibility

### 3. Container Runtime
- **File**: `.devcontainer/devcontainer.json` → `containerEnv`
- **Behavior**: Proxy environment variables set in running container
- **Reason**: Claude Code needs proxy for API calls at runtime

## Configuration Flow

```
devcontainer.json
├── containerEnv → HTTP_PROXY, HTTPS_PROXY, NO_PROXY (runtime)
└── features
    ├── ./features/proxy
    │   ├── http_proxy → Creates state file
    │   ├── https_proxy →
    │   └── no_proxy →
    └── ./features/claude-code
        └── (reads from ~/.config/devcontainer/proxy)
```

## State File Format

`~/.config/devcontainer/proxy`:
```bash
# DevContainer Proxy Configuration
HTTP_PROXY='http://proxy.example.com:8080'
HTTPS_PROXY='http://proxy.example.com:8080'
NO_PROXY='localhost,127.0.0.1,.local'
```

## Feature Interaction

1. **Proxy feature runs first** - Creates shared state file
2. **Other features consume** - Source state file or use utility functions
3. **Runtime vars available** - Via containerEnv in devcontainer.json

## Backward Compatibility

The claude-code feature uses a hybrid proxy approach:

### Build-Time (Docker Build Phase)
During container image build, when the shared state file doesn't exist yet:
- Uses direct `http_proxy`/`https_proxy` options from devcontainer.json
- These options are required for downloading Claude Code installer behind a proxy
- Falls back to global environment variables if options not set

### Runtime (Container Running)
After container starts, when shared state file exists:
- Reads from shared state file at `~/.config/devcontainer/proxy` (created by `./features/proxy`)
- This is the recommended approach for runtime proxy configuration

### Proxy Priority in install.sh
1. Shared state file (preferred - created by proxy feature)
2. Direct options (build-time fallback - required for Docker builds)
3. Global ENV (ultimate fallback)

**Note:** Direct proxy options in devcontainer.json are necessary for Docker builds even when using the proxy feature, because each feature runs in isolated layers during the build process.
