# HTTP Proxy Devcontainer Feature

Configures HTTP proxy for devcontainer features and runtime.

## Overview

This feature creates a shared proxy configuration state file at `~/.config/devcontainer/proxy` that other features can consume. This provides:

1. **Centralized configuration** - Set proxy once, use everywhere
2. **Reusable across features** - Any feature can read the proxy state
3. **Runtime environment** - Proxy vars available in container environment
4. **Optional apt proxy** - Can configure apt package manager

## Usage

### Basic Configuration

```json
{
  "features": {
    "./features/proxy": {
      "http_proxy": "http://proxy.example.com:8080",
      "enabled": true
    }
  }
}
```

### Full Configuration

```json
{
  "features": {
    "./features/proxy": {
      "http_proxy": "http://proxy.example.com:8080",
      "https_proxy": "http://proxy.example.com:8080",
      "no_proxy": "localhost,127.0.0.1,.local,*.internal",
      "apt_proxy": true,
      "enabled": true
    }
  }
}
```

### Disable Proxy

```json
{
  "features": {
    "./features/proxy": {
      "enabled": false
    }
  }
}
```

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `http_proxy` | string | `""` | HTTP proxy URL |
| `https_proxy` | string | `""` | HTTPS proxy URL (defaults to `http_proxy` if not set) |
| `no_proxy` | string | `"localhost,127.0.0.1,.local"` | Comma-separated bypass list |
| `apt_proxy` | boolean | `false` | Configure apt to use proxy |
| `enabled` | boolean | `true` | Enable proxy configuration |

## State File

The feature creates `~/.config/devcontainer/proxy` with the following format:

```bash
# DevContainer Proxy Configuration
HTTP_PROXY='http://proxy.example.com:8080'
HTTPS_PROXY='http://proxy.example.com:8080'
NO_PROXY='localhost,127.0.0.1,.local'
```

## Consuming from Other Features

Features can source the state file or use the utility functions:

```bash
# Option 1: Source state file directly
if [ -f ~/.config/devcontainer/proxy ]; then
    source ~/.config/devcontainer/proxy
fi

# Option 2: Use utility functions
source /path/to/features/proxy/proxy.sh
HTTP_PROXY=$(get_proxy_http)
HTTPS_PROXY=$(get_proxy_https)
```

## Example: claude-code Feature

The claude-code feature reads proxy from the shared state:

```bash
# In claude-code/install.sh
if [ -f ~/.config/devcontainer/proxy ]; then
    source ~/.config/devcontainer/proxy
    export HTTP_PROXY HTTPS_PROXY
fi
```

## Runtime Environment

For proxy to be available at container runtime, also configure `containerEnv`:

```json
{
  "containerEnv": {
    "HTTP_PROXY": "http://proxy.example.com:8080",
    "HTTPS_PROXY": "http://proxy.example.com:8080",
    "NO_PROXY": "localhost,127.0.0.1,.local"
  },
  "features": {
    "./features/proxy": {
      "http_proxy": "http://proxy.example.com:8080"
    }
  }
}
```
