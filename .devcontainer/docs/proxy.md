# Proxy Configuration

Complete guide to configuring proxy settings for Claude Code Dev Container in enterprise environments.

## Overview

Proxy settings are **separated by scope** to ensure they're used only where needed:

| Scope | Proxy Used | Reason |
|-------|-------------|---------|
| Dockerfile Build | ❌ No | Direct Debian package downloads |
| Feature Installation | ✅ Yes | Claude Code download requires proxy |
| Container Runtime | ✅ Yes | API calls to LLM provider |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Build Process                            │
│  ┌────────────────┐    ┌──────────────────────────────────┐ │
│  │   Dockerfile    │    │   Feature (install.sh)          │ │
│  │   NO PROXY     │    │   WITH PROXY                    │ │
│  │                │    │   └─> Downloads claude.ai        │ │
│  └────────────────┘    └──────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Runtime Container                            │
│  ┌────────────────────────────────────────────────────────┐  │
│  │              WITH PROXY                               │  │
│  │   └─> Claude Code API calls via proxy               │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Configuration

### Step 1: Add Proxy Feature

Add proxy feature to `.devcontainer/devcontainer.json`:

```json
{
  "features": {
    "./features/proxy": {
      "http_proxy": "http://proxy.example.com:8080",
      "https_proxy": "http://proxy.example.com:8080",
      "no_proxy": "localhost,127.0.0.1,.local,192.168.0.0/16",
      "enabled": true
    }
  }
}
```

### Step 2: Set Runtime Proxy

Add proxy variables to `.devcontainer/devcontainer.json`:

```json
{
  "name": "Claude Code Dev Container",
  "containerEnv": {
    "HTTP_PROXY": "http://proxy.example.com:8080",
    "HTTPS_PROXY": "http://proxy.example.com:8080",
    "NO_PROXY": "localhost,127.0.0.1,.local,192.168.0.0/16"
  }
}
```

**Environment Variables Explained:**

| Variable | Purpose |
|----------|---------|
| `HTTP_PROXY` | Proxy for HTTP requests |
| `HTTPS_PROXY` | Proxy for HTTPS requests |
| `NO_PROXY` | Direct connection exclusions |

**Common NO_PROXY Values:**
- `localhost` - Local services
- `127.0.0.1` - Loopback
- `.local` - Bonjour/mDNS
- `192.168.0.0/16` - Private network ranges

### Step 3: Configure Consuming Features (Optional)

The claude-code feature automatically reads proxy from shared state. If you're using a custom feature, source the state file:

```bash
# In your feature's install.sh
if [ -f ~/.config/devcontainer/proxy ]; then
    source ~/.config/devcontainer/proxy
fi
```

### Step 4: Build Container

```bash
# Rebuild to apply proxy settings
# VS Code: F1 → Dev Containers: Rebuild Container
# Or CLI:
make clean
make build
```

## Verification

Verify proxy configuration inside container:

```bash
# Check runtime variables
echo $HTTP_PROXY
echo $HTTPS_PROXY
echo $NO_PROXY

# Test proxy connectivity
curl -v https://claude.ai

# Test API call through proxy
claude "test connection"
```

## Troubleshooting

### Issue: Build Fails - Cannot Download Packages

**Symptoms:**
- `apt-get update` fails
- Package download timeouts

**Cause:** Proxy incorrectly configured for Dockerfile build

**Solution:** Dockerfile should NOT have proxy. Verify `.devcontainer/Dockerfile`:

```dockerfile
# CORRECT - No proxy ENV
FROM debian:12-slim
RUN apt-get update && apt-get install -y git

# INCORRECT - Remove these lines
# ENV HTTP_PROXY=http://proxy:8080
# ENV HTTPS_PROXY=http://proxy:8080
```

### Issue: Claude Code Installation Fails

**Symptoms:**
- `curl https://claude.ai/install.sh` times out
- install.sh script fails

**Cause:** Proxy feature not configured

**Solution:** Ensure proxy feature is enabled in `devcontainer.json`:

```json
{
  "features": {
    "./features/proxy": {
      "http_proxy": "http://proxy:8080",
      "https_proxy": "http://proxy:8080",
      "enabled": true
    }
  }
}
```

### Issue: Claude Code Cannot Reach API

**Symptoms:**
- `claude` command fails with network error
- API timeout errors

**Cause:** Runtime proxy not configured or blocked

**Solutions:**

1. Verify runtime proxy is set:
```bash
echo $HTTP_PROXY
```

2. Test proxy from host:
```bash
curl -x http://proxy:8080 https://api.anthropic.com
```

3. Check NO_PROXY doesn't exclude API endpoint:
```bash
echo $NO_PROXY | grep anthropic
# Remove API domain from NO_PROXY if present
```

4. Verify proxy allows HTTPS traffic:
```bash
# Some proxies block CONNECT method
curl -v -x http://proxy:8080 https://www.google.com
```

### Issue: Certain Connections Fail

**Symptoms:**
- Some services work, others don't
- Intermittent failures

**Cause:** NO_PROXY misconfiguration

**Solution:** Adjust exclusions in `devcontainer.json`:

```json
{
  "containerEnv": {
    "NO_PROXY": "localhost,127.0.0.1,.local,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16"
  }
}
```

**Private Network Ranges:**
- `10.0.0.0/8` - Class A private
- `172.16.0.0/12` - Class B private
- `192.168.0.0/16` - Class C private

## Advanced Configuration

### Authentication

If proxy requires authentication:

```json
{
  "containerEnv": {
    "HTTP_PROXY": "http://username:password@proxy.example.com:8080",
    "HTTPS_PROXY": "http://username:password@proxy.example.com:8080"
  }
}
```

**Security Note:** Consider using environment-specific configuration or secret management for production use.

### Proxy Auto-Config (PAC)

For environments with PAC files:

1. **Convert PAC to proxy URL** - Extract direct proxy URL
2. **Configure in devcontainer.json** - Use direct URL

### Multiple Proxies

Different proxies for different protocols:

```json
{
  "containerEnv": {
    "HTTP_PROXY": "http://http-proxy.example.com:8080",
    "HTTPS_PROXY": "http://https-proxy.example.com:8080"
  },
  "features": {
    "./features/proxy": {
      "http_proxy": "http://http-proxy.example.com:8080",
      "https_proxy": "http://https-proxy.example.com:8080"
    }
  }
}
```

### Conditional Proxy

Only use proxy in certain environments:

```json
// Use ${localEnv:PROXY_URL} to read from host environment
{
  "containerEnv": {
    "HTTP_PROXY": "${localEnv:HTTP_PROXY}",
    "HTTPS_PROXY": "${localEnv:HTTPS_PROXY}"
  }
}
```

## Reference

See [PROXY_ARCHITECTURE.md](../.devcontainer/PROXY_ARCHITECTURE.md) in the repository for implementation details.

## Related Documentation

- [Setup Guide](setup.md) - General installation instructions
- [Architecture](architecture.md) - System design overview
- [LLM Providers](providers.md) - Provider-specific configuration
