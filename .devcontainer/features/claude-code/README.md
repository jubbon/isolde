# Isolde Feature

Installs Anthropic Claude Code CLI with multi-provider support for the Isolde project.

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `version` | string | `latest` | Claude Code version |
| `provider` | string | `""` | LLM provider name (e.g., `z.ai`, `anthropic`) |
| `models` | string | `""` | Model mapping: `haiku:model,sonnet:model,opus:model` |
| `http_proxy` | string | `""` | HTTP proxy for build-time downloads only |
| `https_proxy` | string | `""` | HTTPS proxy for build-time downloads only |

## Proxy Configuration

The claude-code feature supports a hybrid proxy approach:

### Build-Time (Docker Image Build)
During the Docker build phase, proxy settings are needed for downloading the Claude Code installer:
- **Source:** Direct `http_proxy`/`https_proxy` options in devcontainer.json
- **Required when:** Building behind a corporate proxy
- **Example:**
  ```json
  {
    "features": {
      "./features/claude-code": {
        "http_proxy": "http://proxy.example.com:8080",
        "https_proxy": "http://proxy.example.com:8080"
      }
    }
  }
  ```

### Runtime (Container Running)
During container runtime, proxy settings are used for Claude Code API calls:
- **Source:** Shared state file at `~/.config/devcontainer/proxy` (created by `./features/proxy`)
- **Recommended:** Use the `./features/proxy` feature for consistent proxy management

### Proxy Priority
The install script uses this priority order:
1. Shared state file (`~/.config/devcontainer/proxy`) - preferred
2. Direct options (`http_proxy`, `https_proxy`) - build-time fallback
3. Global environment variables (`HTTP_PROXY`, `HTTPS_PROXY`) - ultimate fallback

## Provider Configuration

### How it works

When `provider` is specified, the feature:

1. **During build:** Creates `~/.config/devcontainer/provider` with the provider name
2. **At first startup:** `postCreateCommand` updates `~/.bashrc` to read from that file
3. **Every shell:** Credentials are loaded from:

```
~/.claude/providers/{provider}/
├── auth       # API token
└── base_url   # API endpoint URL
```

**Note:** The provider file is stored in `~/.config/devcontainer/` (container-local) to avoid race conditions when `~/.claude` is mounted between multiple containers.

### Available Providers

#### Anthropic (Default)
No provider configuration needed. Uses:
- `~/.claude/auth` for API token
- Default Anthropic endpoint

#### Z.ai (Zhipu AI)

**Setup:**
```bash
mkdir -p ~/.claude/providers/z.ai
echo "your-api-token" > ~/.claude/providers/z.ai/auth
echo "https://api.z.ai/api/anthropic" > ~/.claude/providers/z.ai/base_url
```

**Usage in devcontainer.json:**
```json
{
  "features": {
    "./features/claude-code": {
      "provider": "z.ai"
    }
  }
}
```

### Adding New Providers

Create a directory under `~/.claude/providers/`:

```bash
mkdir -p ~/.claude/providers/myprovider
echo "your-api-key" > ~/.claude/providers/myprovider/auth
echo "https://api.example.com/v1" > ~/.claude/providers/myprovider/base_url
```

Then use in devcontainer.json:
```json
{
  "features": {
    "./features/claude-code": {
      "provider": "myprovider"
    }
  }
}
```

## Complete Example

```json
{
  "name": "My Project",
  "features": {
    "./features/claude-code": {
      "provider": "z.ai",
      "version": "latest"
    }
  },
  "mounts": [
    "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind"
  ]
}
```

## Environment Variables

After container start, the following variables are automatically set:

- `ANTHROPIC_AUTH_TOKEN` — loaded from provider's `auth` file
- `ANTHROPIC_BASE_URL` — loaded from provider's `base_url` file (optional)

## Troubleshooting

### Environment variables not set

**Problem:** `ANTHROPIC_AUTH_TOKEN` or `ANTHROPIC_BASE_URL` are empty in new shells.

**Solution:**

1. Check if `~/.config/devcontainer/provider` file exists:
   ```bash
   cat ~/.config/devcontainer/provider
   ```

2. Check if `~/.bashrc` was updated correctly:
   ```bash
   grep "local provider=" ~/.bashrc
   ```
   Should show: `local provider="$(cat ~/.config/devcontainer/provider)"`

3. If not, rebuild the devcontainer to trigger `postCreateCommand` again.

- `ANTHROPIC_AUTH_TOKEN` — loaded from provider's `auth` file
- `ANTHROPIC_BASE_URL` — loaded from provider's `base_url` file (optional)
