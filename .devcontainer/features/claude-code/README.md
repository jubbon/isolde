# Claude Code DevContainer Feature

Installs Anthropic Claude Code CLI with multi-provider support.

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `version` | string | `latest` | Claude Code version |
| `provider` | string | `""` | LLM provider name (e.g., `z.ai`, `anthropic`) |
| `http_proxy` | string | `""` | HTTP proxy for installation |
| `https_proxy` | string | `""` | HTTPS proxy for installation |

## Provider Configuration

### How it works

When `provider` is specified, the feature reads configuration from:

```
~/.claude/providers/{provider}/
├── auth       # API token
└── base_url   # API endpoint URL
```

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
