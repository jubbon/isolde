# Proxy Configuration Architecture

## Overview
Proxy settings are now separated by scope to ensure they're used only where needed.

## Scopes

### 1. Dockerfile Build (NO PROXY)
- **File**: `.devcontainer/Dockerfile`
- **Behavior**: `apt-get update/install` commands run **without proxy**
- **Reason**: Debian package downloads should use direct connection

### 2. Feature Installation (WITH PROXY)
- **Files**:
  - `.devcontainer/features/claude-code/devcontainer-feature.json` (defines options)
  - `.devcontainer/features/claude-code/install.sh` (uses options)
  - `.devcontainer/devcontainer.json` (passes options to feature)
- **Behavior**: Claude Code installation via `curl https://claude.ai/install.sh` uses proxy
- **Reason**: Claude Code download requires proxy for network access

### 3. Container Runtime (WITH PROXY)
- **File**: `.devcontainer/devcontainer.json` → `containerEnv`
- **Behavior**: Proxy environment variables are set in the running container
- **Reason**: Claude Code needs proxy for API calls when running

## Configuration Flow

```
devcontainer.json
├── build.args → Only USERNAME (proxy args removed)
├── containerEnv → HTTP_PROXY, HTTPS_PROXY, NO_PROXY (runtime)
└── features
    └── ./features/claude-code
        ├── http_proxy → passed to install.sh
        └── https_proxy → passed to install.sh
```

## Changes Made

| File | Change |
|------|--------|
| Dockerfile | Removed `ENV HTTP_PROXY=...` (was global, affected all RUN commands) |
| devcontainer.json | Removed proxy from `build.args`, added to feature options |
| devcontainer-feature.json | Added `http_proxy`, `https_proxy` options |
| install.sh | Uses feature options instead of global ENV |
