# LLM Providers

Guide to configuring and using different LLM providers with Isolde.

## Overview

The devcontainer supports multiple LLM providers through a flexible configuration system:

```
~/.claude/providers/
├── anthropic/    # Default (uses ~/.claude/auth)
├── z.ai/         # Zhipu AI
└── custom/       # Your custom provider
```

## Available Providers

### Anthropic (Default)

**Configuration:**
```bash
mkdir -p ~/.claude
echo "sk-ant-api-key-here" > ~/.claude/auth
chmod 600 ~/.claude/auth
```

**devcontainer.json:**
```json
{
  "features": {
    "./features/claude-code": {
      // No provider option needed - this is default
    }
  }
}
```

**Environment Variables:**
- `ANTHROPIC_AUTH_TOKEN` - Set from `~/.claude/auth`
- `ANTHROPIC_BASE_URL` - Not set (uses default)

**API Endpoint:** `https://api.anthropic.com` (default)

### Z.ai (Zhipu AI)

**Configuration:**
```bash
mkdir -p ~/.claude/providers/z.ai
echo "your-zhipu-api-token" > ~/.claude/providers/z.ai/auth
echo "https://open.bigmodel.cn/api/paas/v4/" > ~/.claude/providers/z.ai/base_url
chmod 600 ~/.claude/providers/z.ai/*
```

**devcontainer.json:**
```json
{
  "features": {
    "./features/claude-code": {
      "provider": "z.ai"
    }
  }
}
```

**Environment Variables:**
- `ANTHROPIC_AUTH_TOKEN` - Set from `~/.claude/providers/z.ai/auth`
- `ANTHROPIC_BASE_URL` - Set from `~/.claude/providers/z.ai/base_url`

### Custom Provider

**Configuration:**
```bash
# Create provider directory
mkdir -p ~/.claude/providers/myprovider

# Add API token
echo "your-api-key" > ~/.claude/providers/myprovider/auth

# Add custom base URL (optional)
echo "https://api.example.com/v1" > ~/.claude/providers/myprovider/base_url

# Set permissions
chmod 600 ~/.claude/providers/myprovider/*
```

**devcontainer.json:**
```json
{
  "features": {
    "./features/claude-code": {
      "provider": "myprovider"
    }
  }
}
```

**Environment Variables:**
- `ANTHROPIC_AUTH_TOKEN` - Set from `~/.claude/providers/myprovider/auth`
- `ANTHROPIC_BASE_URL` - Set from `~/.claude/providers/myprovider/base_url` (if file exists)

## Provider Loading Mechanism

### How It Works

1. **Build Time** - Feature creates provider marker file
   ```bash
   ~/.config/devcontainer/provider
   ```

2. **First Start** - `postCreateCommand` updates `~/.bashrc`
   ```bash
   local provider="$(cat ~/.config/devcontainer/provider)"
   configure_claude_provider "$provider"
   ```

3. **Every Shell** - `~/.bashrc` sources provider function
   ```bash
   configure_claude_provider() {
       local provider=$1
       local provider_dir="$HOME/.claude/providers/$provider"

       if [ -f "$provider_dir/auth" ]; then
           export ANTHROPIC_AUTH_TOKEN="$(cat "$provider_dir/auth")"
       fi

       if [ -f "$provider_dir/base_url" ]; then
           export ANTHROPIC_BASE_URL="$(cat "$provider_dir/base_url")"
       fi
   }
   ```

4. **Claude Code** - Reads environment variables for API calls

### Configuration Flow

```
┌────────────────────────────────────────────────────────────┐
│  devcontainer.json                                       │
│  { "provider": "z.ai" }                               │
└──────────────────┬─────────────────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────────────────┐
│  install.sh (build time)                                  │
│  echo "z.ai" > ~/.config/devcontainer/provider             │
└──────────────────┬─────────────────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────────────────┐
│  postCreateCommand (first start)                          │
│  >> ~/.bashrc                                            │
│  configure_claude_provider() function                    │
└──────────────────┬─────────────────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────────────────┐
│  ~/.bashrc (every shell)                                 │
│  local provider="z.ai"                                   │
│  configure_claude_provider "$provider"                    │
│  └──> export ANTHROPIC_AUTH_TOKEN="..."                  │
│  └──> export ANTHROPIC_BASE_URL="..."                    │
└──────────────────┬─────────────────────────────────────────┘
                   │
                   ▼
┌────────────────────────────────────────────────────────────┐
│  Claude Code CLI                                         │
│  Reads $ANTHROPIC_AUTH_TOKEN                            │
│  Reads $ANTHROPIC_BASE_URL                              │
│  Makes API call to provider                              │
└────────────────────────────────────────────────────────────┘
```

## Switching Providers

### Temporary Switch

```bash
# Override for current session
export ANTHROPIC_AUTH_TOKEN="$(cat ~/.claude/providers/other-provider/auth)"
export ANTHROPIC_BASE_URL="$(cat ~/.claude/providers/other-provider/base_url)"
```

### Permanent Switch

1. Update `devcontainer.json` with new provider
2. Rebuild container
3. Verify in new shell:

```bash
echo $ANTHROPIC_AUTH_TOKEN
echo $ANTHROPIC_BASE_URL
```

## Troubleshooting

### Provider Variables Not Set

**Symptoms:**
```bash
echo $ANTHROPIC_AUTH_TOKEN
# (empty)
```

**Diagnosis:**
```bash
# 1. Check provider marker exists
cat ~/.config/devcontainer/provider

# 2. Check provider directory exists
ls -la ~/.claude/providers/

# 3. Check auth file exists
cat ~/.claude/providers/z.ai/auth

# 4. Check bashrc has function
grep configure_claude_provider ~/.bashrc
```

**Solutions:**

1. **Rebuild container** - Triggers `postCreateCommand` again
2. **Manually source bashrc** - `source ~/.bashrc`
3. **Check file permissions** - `chmod 644 ~/.claude/providers/*/auth`

### API Connection Failures

**Symptoms:**
```bash
claude "test"
# Error: connection refused / timeout
```

**Diagnosis:**
```bash
# Check base_url is correct
echo $ANTHROPIC_BASE_URL

# Test connectivity
curl -v $ANTHROPIC_BASE_URL

# Check proxy settings if behind firewall
echo $HTTP_PROXY
echo $HTTPS_PROXY
```

**Solutions:**

1. **Verify base_url format** - Include protocol (`https://`)
2. **Check proxy configuration** - See [Proxy Configuration](proxy.md)
3. **Verify API key validity** - Regenerate if needed
4. **Test from host** - Isolate if issue is container-specific

### Wrong Provider Active

**Symptoms:**
- Expected provider not being used
- Calls going to wrong endpoint

**Diagnosis:**
```bash
# Check which provider is configured
cat ~/.config/devcontainer/provider

# Verify environment variables
echo $ANTHROPIC_BASE_URL
```

**Solutions:**

1. **Update devcontainer.json** - Set correct provider
2. **Rebuild container** - Apply new configuration
3. **Clear stale environment** - Start fresh shell

## Provider Comparison

| Provider | Base URL | Notes |
|----------|-----------|-------|
| Anthropic | (default) | Official Claude API |
| Z.ai | `https://open.bigmodel.cn/api/paas/v4/` | Chinese AI provider |
| Custom | (configurable) | Any Anthropic-compatible API |

## Adding New Providers

### Template

```bash
# 1. Create provider directory
mkdir -p ~/.claude/providers/new-provider

# 2. Add authentication
cat > ~/.claude/providers/new-provider/auth << 'EOF'
your-api-token-here
EOF

# 3. Add base URL (if needed)
cat > ~/.claude/providers/new-provider/base_url << 'EOF'
https://api.example.com/v1
EOF

# 4. Set permissions
chmod 600 ~/.claude/providers/new-provider/*

# 5. Configure in devcontainer.json
# { "provider": "new-provider" }
```

### Provider Requirements

**Required:**
- `auth` file with API token

**Optional:**
- `base_url` file with API endpoint (defaults to Anthropic)

**Format:**
- Token: plain text API key
- URL: include protocol (`https://`)

## Related Documentation

- [Setup Guide](setup.md) - Installation instructions
- [Proxy Configuration](proxy.md) - Enterprise network setup
- [Architecture](architecture.md) - Provider system design
- [Version Control](claude-version-control.md) - Managing versions
