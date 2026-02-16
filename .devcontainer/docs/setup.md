# Setup Guide

Complete guide for setting up and configuring Claude Code Dev Container.

## Prerequisites

### Required

- **Docker** - Installed and running on host machine
- **VS Code** (recommended) - With Dev Containers extension
- **API Token** - For your chosen LLM provider

### Optional

- **Git** - For version control and worktrees
- **Docker Group** - Add user to `docker` group for socket-less access

## Prevent Setup Wizard from Running

**Problem:** After rebuilding the Dev Container, Claude Code's setup wizard runs repeatedly, asking about color scheme, file system trust, and other initial configuration.

**Solution:** Create a persistent machine-id on your host machine and mount it into the container.

### One-Time Host Setup

Run these commands on your **host machine** (not in the container):

```bash
# Create directory for persistent container identity
mkdir -p ~/.config/devcontainer

# Generate and save a persistent machine-id
uuidgen > ~/.config/devcontainer/machine-id
```

**That's it!** The `devcontainer.json` already includes the mount configuration:
```json
"source=${localEnv:HOME}/.config/devcontainer/machine-id,target=/etc/machine-id,type=bind,consistency=cached"
```

### Verification

After rebuilding the container:

```bash
# Inside container - verify machine-id persists
cat /etc/machine-id

# Start Claude Code - setup wizard should NOT run
claude
```

## Installation Methods

### Method 1: VS Code (Recommended)

1. **Open in VS Code**
   ```bash
   cd /path/to/claude-code-devcontainer
   code .
   ```

2. **Install Extension**
   - Install [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

3. **Reopen in Container**
   - Press `F1`
   - Select: `Dev Containers: Reopen in Container`

4. **Wait for Build**
   - First build takes 2-5 minutes
   - Subsequent starts are faster

### Method 2: CLI

```bash
# Clone or navigate to project
cd /path/to/claude-code-devcontainer

# Build the image
make build

# Run with current workspace mounted
make devcontainer

# Open a new shell in running container
make shell

# Clean up when done
make clean
```

## Configuration

### 1. LLM Provider

#### Default: Anthropic

No configuration needed. Just place your API token:

```bash
mkdir -p ~/.claude
echo "sk-ant-api-key-here" > ~/.claude/auth
```

#### Custom Provider (e.g., Z.ai)

**Step 1:** Create provider directory
```bash
mkdir -p ~/.claude/providers/z.ai
```

**Step 2:** Add API token
```bash
echo "your-api-token" > ~/.claude/providers/z.ai/auth
```

**Step 3:** Add base URL (optional)
```bash
echo "https://api.z.ai/api/anthropic" > ~/.claude/providers/z.ai/base_url
```

**Step 4:** Configure in `.devcontainer/devcontainer.json`
```json
{
  "features": {
    "./features/claude-code": {
      "provider": "z.ai"
    }
  }
}
```

#### Enterprise Proxy

Add proxy settings to `.devcontainer/devcontainer.json`:

```json
{
  "containerEnv": {
    "HTTP_PROXY": "http://proxy.example.com:8080",
    "HTTPS_PROXY": "http://proxy.example.com:8080",
    "NO_PROXY": "localhost,127.0.0.1,.local"
  },
  "features": {
    "./features/claude-code": {
      "http_proxy": "http://proxy.example.com:8080",
      "https_proxy": "http://proxy.example.com:8080"
    }
  }
}
```

See [Proxy Configuration](proxy.md) for detailed proxy setup.

### 2. Version Control

Choose which Claude Code version to install via `.devcontainer/devcontainer.json`:

```json
{
  "features": {
    "./features/claude-code": {
      "version": "latest"    // Auto-updates (default)
      // "version": "stable"   // Latest stable, no auto-update
      // "version": "1.2.41"   // Specific version
    }
  }
}
```

See [Version Control](claude-version-control.md) for details.

### 3. Model Configuration

Set default models via environment variables in `.devcontainer/devcontainer.json`:

```json
{
  "containerEnv": {
    "ANTHROPIC_DEFAULT_HAIKU_MODEL": "claude-3-5-haiku-20241022",
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "claude-3-5-sonnet-20241022",
    "ANTHROPIC_DEFAULT_OPUS_MODEL": "claude-3-5-sonnet-20241022"
  }
}
```

## Verification

After container starts, verify configuration:

```bash
# Check Claude Code is installed
claude --version

# Verify provider environment variables
echo $ANTHROPIC_AUTH_TOKEN
echo $ANTHROPIC_BASE_URL

# Test API connection
claude "Hello, world!"

# Check Docker-in-Docker
docker ps
```

## Troubleshooting

### Container Won't Start

**Symptom:** Build fails or container exits immediately

**Solutions:**
1. Check Docker is running: `docker ps`
2. Verify `devcontainer.json` syntax: `cat .devcontainer/devcontainer.json | jq`
3. Review build logs in VS Code Output panel

### Provider Not Working

**Symptom:** `ANTHROPIC_AUTH_TOKEN` or `ANTHROPIC_BASE_URL` are empty

**Solutions:**
```bash
# 1. Check provider file exists
cat ~/.config/devcontainer/provider

# 2. Check bashrc was updated
grep "provider=" ~/.bashrc

# 3. Rebuild container to trigger postCreateCommand
# In VS Code: F1 → Dev Containers: Rebuild Container
```

### Proxy Issues

**Symptom:** Claude Code can't reach API or download fails

**Solutions:**
1. Verify proxy is accessible from host: `curl -x http://proxy:port https://claude.ai`
2. Check `NO_PROXY` includes necessary exclusions
3. Ensure proxy is configured for both build and runtime
4. See [Proxy Configuration](proxy.md) for detailed debugging

### Permission Denied

**Symptom:** Can't access Docker socket inside container

**Solutions:**
```bash
# Add user to docker group on host
sudo usermod -aG docker $USER

# Log out and back in for group change to take effect
```

### VS Code Extensions Missing

**Symptom:** Expected extensions not installed

**Solutions:**
1. Check `.devcontainer/devcontainer.json` extensions list
2. Manually install in container: Press `Ctrl+Shift+X`
3. Rebuild container: `F1 → Dev Containers: Rebuild Container`

## Advanced Setup

### Custom Features

Add custom Dev Container features in `.devcontainer/features/`:

```bash
mkdir .devcontainer/features/my-feature
```

Create `devcontainer-feature.json`:
```json
{
  "id": "my-feature",
  "name": "My Custom Feature",
  "version": "1.0.0",
  "options": [
    {
      "id": "myoption",
      "type": "string",
      "default": "default-value"
    }
  ]
}
```

See [Dev Containers Feature Spec](https://devcontainers.github.io/implementors/features/).

### Persistent Configuration

Create `.claude/settings.local.json`:

```json
{
  "permissions": {
    "allowedTools": ["bash", "docker", "git"]
  }
}
```

### Git Worktrees

For isolated development branches:

```bash
# Add worktree
git worktree add ../feature-branch feature-branch

# Remove worktree when done
git worktree remove ../feature-branch
```

## Next Steps

- Read [Architecture](architecture.md) to understand system design
- Configure [LLM Providers](providers.md) for your needs
- Review [Development](development.md) for contribution workflow
- Set up [Proxy Configuration](proxy.md) if behind corporate firewall
