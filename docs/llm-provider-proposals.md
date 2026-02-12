# LLM Provider Configuration for Claude Code DevContainer Feature

**Research Date:** 2026-02-11
**Status:** Proposals for Multi-Provider Support

## Executive Summary

This document proposes several architectural approaches for configuring multiple LLM providers (Anthropic, Z.ai/GLM, OpenAI, Gemini, etc.) in Claude Code through a DevContainer Feature. The goal is to maintain Anthropic as the default provider while enabling seamless switching to alternative providers.

---

## Current State Analysis

### Claude Code Configuration Points

Based on official documentation, Claude Code supports configuration through multiple scopes:

| Scope | Location | Priority | Use Case |
|-------|----------|-----------|-----------|
| Managed | `/etc/claude-code/managed-settings.json` | Highest | IT/Team policies |
| User | `~/.claude/settings.json` | High | Personal preferences |
| Project | `.claude/settings.json` | Medium | Project-specific |
| Local | `.claude/settings.local.json` | Low | Local overrides |

### Environment Variables for Provider Configuration

Claude Code supports these key environment variables:

```bash
# Primary Provider Configuration
ANTHROPIC_API_KEY              # API authentication
ANTHROPIC_BASE_URL            # Custom endpoint (for proxy/gateway)

# Model Selection
CLAUDE_DEFAULT_MODEL           # Override default model
CLAUDE_CODE_MAX_OUTPUT_TOKENS  # Output limits
MAX_THINKING_TOKENS           # Thinking budget

# Proxy Configuration
HTTP_PROXY                    # HTTP proxy for installation
HTTPS_PROXY                   # HTTPS proxy for installation
```

### Current DevContainer Feature Structure

```json
{
  "id": "claude-code",
  "version": "1.0.0",
  "options": {
    "version": { "type": "string", "default": "latest" },
    "http_proxy": { "type": "string", "default": "" },
    "https_proxy": { "type": "string", "default": "" }
  }
}
```

---

## Provider Compatibility Research

### Z.ai (Zhipu AI / GLM Models)

**Endpoint:** `https://open.bigmodel.cn/api/paas/v4`
**Models:** GLM-4.5, GLM-4.6, GLM-4.7, GLM-4.7 Flash
**Compatibility:** OpenAI-compatible API

Documentation:
- [GLM-4.7 Documentation](https://docs.z.ai/guides/llm/glm-4.7)
- [OpenAI Compatibility Guide](https://docs.bigmodel.cn/cn/guide/develop/openai/introduction)
- [API Reference](https://docs.z.ai/api-reference/introduction)

### OpenRouter (400+ Models Aggregator)

**Endpoint:** `https://openrouter.ai/api`
**Models:** 400+ models from multiple providers
**Compatibility:** OpenAI-compatible API

Documentation:
- [Quick Start](https://openrouter.ai/docs/quickstart)
- [Provider Routing](https://openrouter.ai/docs/guides/routing/provider-selection)
- [Claude Code Integration](https://openrouter.ai/docs/guides/guides/claude-code-integration)

### LiteLLM Proxy (100+ Providers)

**Endpoint:** Self-hosted or cloud
**Models:** 100+ providers unified
**Compatibility:** OpenAI-compatible API

Documentation:
- [Providers Documentation](https://docs.litellm.ai/docs/providers)
- [Proxy Configuration](https://docs.litellm.ai/docs/proxy/configs)
- [Getting Started](https://docs.litellm.ai/docs/)

---

## Proposed Solutions

### **Proposal 1: Direct Provider Selection via Environment Variables**

#### Architecture

Add provider-specific options to the DevContainer Feature that configure environment variables:

```json
{
  "id": "claude-code",
  "version": "2.0.0",
  "name": "Claude Code CLI with Multi-Provider Support",
  "options": {
    "llm_provider": {
      "type": "string",
      "enum": ["anthropic", "openrouter", "zai", "litellm", "custom"],
      "default": "anthropic",
      "description": "Primary LLM provider"
    },
    "anthropic_api_key": {
      "type": "string",
      "default": "",
      "description": "Anthropic API key (auto-loaded from ~/.claude/auth if empty)"
    },
    "anthropic_base_url": {
      "type": "string",
      "default": "https://api.anthropic.com",
      "description": "Anthropic API base URL"
    },
    "openrouter_api_key": {
      "type": "string",
      "default": "",
      "description": "OpenRouter API key"
    },
    "openrouter_base_url": {
      "type": "string",
      "default": "https://openrouter.ai/api/v1",
      "description": "OpenRouter base URL"
    },
    "zai_api_key": {
      "type": "string",
      "default": "",
      "description": "Z.ai (Zhipu AI) API key"
    },
    "zai_base_url": {
      "type": "string",
      "default": "https://open.bigmodel.cn/api/paas/v4",
      "description": "Z.ai base URL"
    },
    "custom_provider_name": {
      "type": "string",
      "default": "",
      "description": "Custom provider name (when llm_provider=custom)"
    },
    "custom_base_url": {
      "type": "string",
      "default": "",
      "description": "Custom provider base URL"
    },
    "custom_api_key": {
      "type": "string",
      "default": "",
      "description": "Custom provider API key"
    },
    "default_model": {
      "type": "string",
      "default": "",
      "description": "Override default model (e.g., claude-sonnet-4-5, glm-4.7)"
    }
  }
}
```

#### Implementation (install.sh)

```bash
#!/bin/bash

# Determine active provider
PROVIDER="${llm_provider:-anthropic}"
BASE_URL=""
API_KEY_VAR=""

case "$PROVIDER" in
  anthropic)
    BASE_URL="${anthropic_base_url:-https://api.anthropic.com}"
    API_KEY_VAR="${anthropic_api_key:-}"
    ;;
  openrouter)
    BASE_URL="${openrouter_base_url:-https://openrouter.ai/api/v1}"
    API_KEY_VAR="${openrouter_api_key:-}"
    ;;
  zai)
    BASE_URL="${zai_base_url:-https://open.bigmodel.cn/api/paas/v4}"
    API_KEY_VAR="${zai_api_key:-}"
    ;;
  custom)
    BASE_URL="${custom_base_url}"
    API_KEY_VAR="${custom_api_key}"
    ;;
esac

# Write environment configuration
cat > /etc/profile.d/claude-code-provider.sh << EOF
# Claude Code Provider Configuration
export ANTHROPIC_BASE_URL="${BASE_URL}"
export ANTHROPIC_API_KEY="${API_KEY_VAR}"
EOF

chmod +x /etc/profile.d/claude-code-provider.sh
```

#### Usage Example

```json
{
  "features": {
    "./features/claude-code": {
      "llm_provider": "zai",
      "zai_api_key": "${localEnv:ZAI_API_KEY}",
      "default_model": "glm-4.7"
    }
  }
}
```

---

### **Proposal 2: Profile-Based Configuration with Runtime Switching**

#### Architecture

Create a configuration management system that allows switching between pre-defined provider profiles:

```json
{
  "id": "claude-code",
  "version": "2.0.0",
  "options": {
    "provider_profiles": {
      "type": "object",
      "default": {
        "default": {
          "provider": "anthropic",
          "base_url": "https://api.anthropic.com",
          "model": "claude-sonnet-4-5-20250929"
        },
        "zai": {
          "provider": "zai",
          "base_url": "https://open.bigmodel.cn/api/paas/v4",
          "model": "glm-4.7"
        },
        "openrouter": {
          "provider": "openrouter",
          "base_url": "https://openrouter.ai/api/v1",
          "model": "anthropic/claude-sonnet-4"
        }
      }
    },
    "active_profile": {
      "type": "string",
      "default": "default",
      "description": "Active provider profile name"
    },
    "profile_config_path": {
      "type": "string",
      "default": "~/.claude/providers.json",
      "description": "Path to provider profiles configuration"
    }
  }
}
```

#### Implementation

```bash
#!/bin/bash

# Create provider profiles file
PROFILES_FILE="${profile_config_path:-~/.claude/providers.json}"
ACTIVE_PROFILE="${active_profile:-default}"

# Write profiles configuration
mkdir -p "$(dirname "$PROFILES_FILE")"
cat > "$PROFILES_FILE" << EOF
{
  "profiles": ${provider_profiles},
  "active": "${ACTIVE_PROFILE}"
}
EOF

# Create helper function for profile switching
cat > /usr/local/bin/claude-provider << 'EOF'
#!/bin/bash
PROFILES_FILE="${CLAUDE_PROVIDER_CONFIG:-~/.claude/providers.json}"

list_profiles() {
  jq -r '.profiles | keys[]' "$PROFILES_FILE"
}

set_profile() {
  local profile=$1
  local config=$(jq -r ".profiles[\"$profile\"]" "$PROFILES_FILE")
  local base_url=$(echo "$config" | jq -r '.base_url')
  local model=$(echo "$config" | jq -r '.model')

  export ANTHROPIC_BASE_URL="$base_url"
  export CLAUDE_DEFAULT_MODEL="$model"

  # Update active profile
  jq --arg p "$profile" '.active = $p' "$PROFILES_FILE" > "$PROFILES_FILE.tmp"
  mv "$PROFILES_FILE.tmp" "$PROFILES_FILE"

  echo "Switched to profile: $profile"
  echo "  Base URL: $base_url"
  echo "  Model: $model"
}

case "${1:-}" in
  list) list_profiles ;;
  set) set_profile "$2" ;;
  show) jq -r ".profiles[\"$(jq -r '.active' "$PROFILES_FILE")\"]" "$PROFILES_FILE" ;;
  *) echo "Usage: claude-provider {list|set|show}" ;;
esac
EOF
chmod +x /usr/local/bin/claude-provider
```

#### Usage

```bash
# List available profiles
claude-provider list

# Switch to Z.ai
claude-provider set zai

# Show current profile
claude-provider show
```

---

### **Proposal 3: Gateway/Proxy Approach with LiteLLM**

#### Architecture

Deploy a self-contained LiteLLM proxy container alongside the devcontainer, routing all LLM requests through a unified gateway:

```yaml
# docker-compose.yml for devcontainer
services:
  app:
    build: .
    depends_on:
      - llm-gateway
    environment:
      - ANTHROPIC_BASE_URL=http://llm-gateway:4000
      - LLM_GATEWAY_CONFIG=/workspace/.llm/config.yaml

  llm-gateway:
    image: ghcr.io/berriai/litellm:latest
    volumes:
      - ./litellm-config.yaml:/app/config.yaml
    ports:
      - "4000:4000"
    environment:
      - LITELLM_MASTER_KEY=${LITELLM_KEY}
```

#### LiteLLM Configuration

```yaml
# litellm-config.yaml
model_list:
  - model_name: claude-default
    litellm_params:
      model: anthropic/claude-sonnet-4-5-20250929
      api_key: os.environ/ANTHROPIC_API_KEY
      base_url: https://api.anthropic.com

  - model_name: glm-default
    litellm_params:
      model: openai/glm-4.7
      api_key: os.environ/ZAI_API_KEY
      base_url: https://open.bigmodel.cn/api/paas/v4

  - model_name: gpt-default
    litellm_params:
      model: openai/gpt-4o
      api_key: os.environ/OPENAI_API_KEY

router:
  routing_strategy: usage-based  # or cost-based, latency-based

litellm_settings:
  drop_params: true
  set_verbose: false
```

#### DevContainer Feature Options

```json
{
  "options": {
    "enable_gateway": {
      "type": "boolean",
      "default": false,
      "description": "Enable LiteLLM gateway for multi-provider routing"
    },
    "gateway_config": {
      "type": "string",
      "default": "",
      "description": "Path to LiteLLM configuration file"
    },
    "default_route": {
      "type": "string",
      "default": "claude-default",
      "description": "Default model route through gateway"
    }
  }
}
```

---

### **Proposal 4: Hybrid Approach with Provider Detection**

#### Architecture

Smart configuration that auto-detects available API keys and configures the most appropriate provider:

```json
{
  "options": {
    "auto_detect_providers": {
      "type": "boolean",
      "default": true,
      "description": "Auto-detect available providers from environment"
    },
    "provider_priority": {
      "type": "array",
      "default": ["anthropic", "zai", "openrouter", "openai"],
      "description": "Provider priority order for auto-detection"
    },
    "fallback_providers": {
      "type": "array",
      "default": [],
      "description": "Fallback providers if primary fails"
    }
  }
}
```

#### Implementation

```bash
#!/bin/bash

detect_providers() {
  local available=()
  local priority=(${provider_priority[@]})

  for provider in "${priority[@]}"; do
    case "$provider" in
      anthropic)
        if [[ -n "${ANTHROPIC_API_KEY:-}" ]] || [[ -f "$HOME/.claude/auth" ]]; then
          available+=("anthropic")
        fi
        ;;
      zai)
        if [[ -n "${ZAI_API_KEY:-}" ]]; then
          available+=("zai")
        fi
        ;;
      openrouter)
        if [[ -n "${OPENROUTER_API_KEY:-}" ]]; then
          available+=("openrouter")
        fi
        ;;
      openai)
        if [[ -n "${OPENAI_API_KEY:-}" ]]; then
          available+=("openai")
        fi
        ;;
    esac
  done

  echo "${available[@]}"
}

configure_provider() {
  local provider=$1

  case "$provider" in
    anthropic)
      export ANTHROPIC_BASE_URL="https://api.anthropic.com"
      ;;
    zai)
      export ANTHROPIC_BASE_URL="https://open.bigmodel.cn/api/paas/v4"
      export ANTHROPIC_API_KEY="${ZAI_API_KEY}"
      ;;
    openrouter)
      export ANTHROPIC_BASE_URL="https://openrouter.ai/api/v1"
      export ANTHROPIC_API_KEY="${OPENROUTER_API_KEY}"
      ;;
  esac
}

# Auto-detection
if [[ "${auto_detect_providers:-true}" == "true" ]]; then
  detected=($(detect_providers))
  if [[ ${#detected[@]} -gt 0 ]]; then
    configure_provider "${detected[0]}"
    log_info "Auto-configured provider: ${detected[0]}"
  fi
fi
```

---

## Comparison Matrix

| Proposal | Pros | Cons | Complexity |
|----------|-------|-------|------------|
| **1. Direct Environment Variables** | Simple, transparent, no runtime overhead | Requires rebuild for changes | Low |
| **2. Profile-Based** | Flexible, runtime switching, user-friendly | More complex setup | Medium |
| **3. Gateway/Proxy** | Unified interface, advanced routing, fallbacks | Additional container, resource overhead | High |
| **4. Hybrid Auto-Detect** | Zero-config for users, intelligent fallback | Less predictable, environment dependent | Medium |

---

## Recommended Implementation

**Primary Recommendation:** Proposal 2 (Profile-Based Configuration)

**Rationale:**
1. Balances flexibility with simplicity
2. Allows runtime switching without rebuild
3. Clear user expectations through profiles
4. Easy to extend with new providers
5. Maintains Anthropic as default

**Secondary Addition:** Proposal 4 (Auto-Detection)
- As an opt-in feature for ease of use
- Helps new users get started quickly

---

## Sample Complete Feature Definition

```json
{
  "id": "claude-code",
  "version": "2.0.0",
  "name": "Claude Code CLI with Multi-Provider LLM Support",
  "description": "Installs Claude Code CLI with support for multiple LLM providers (Anthropic, Z.ai, OpenRouter, custom)",
  "documentationURL": "https://code.claude.com/docs",
  "options": {
    "version": {
      "type": "string",
      "default": "latest",
      "description": "Claude Code version"
    },
    "provider": {
      "type": "string",
      "enum": ["anthropic", "zai", "openrouter", "custom", "auto"],
      "default": "anthropic",
      "description": "Primary LLM provider (auto = auto-detect from environment)"
    },
    "anthropic_api_key": {
      "type": "string",
      "default": "",
      "description": "Anthropic API key (defaults to ~/.claude/auth)"
    },
    "zai_api_key": {
      "type": "string",
      "default": "",
      "description": "Z.ai (Zhipu AI) API key for GLM models"
    },
    "openrouter_api_key": {
      "type": "string",
      "default": "",
      "description": "OpenRouter API key for 400+ models"
    },
    "custom_base_url": {
      "type": "string",
      "default": "",
      "description": "Custom provider base URL (when provider=custom)"
    },
    "custom_api_key": {
      "type": "string",
      "default": "",
      "description": "Custom provider API key"
    },
    "custom_model": {
      "type": "string",
      "default": "",
      "description": "Custom model identifier"
    },
    "default_model": {
      "type": "string",
      "default": "",
      "description": "Override default model (e.g., claude-sonnet-4-5, glm-4.7)"
    },
    "enable_profile_switching": {
      "type": "boolean",
      "default": true,
      "description": "Enable profile switching via claude-provider command"
    },
    "http_proxy": {
      "type": "string",
      "default": "",
      "description": "HTTP proxy for installation"
    },
    "https_proxy": {
      "type": "string",
      "default": "",
      "description": "HTTPS proxy for installation"
    }
  },
  "entrypoint": "install.sh"
}
```

---

## Implementation Roadmap

### Phase 1: Core Multi-Provider Support
- [ ] Update `devcontainer-feature.json` with provider options
- [ ] Modify `install.sh` for provider configuration
- [ ] Test with Anthropic (default)
- [ ] Test with Z.ai/GLM

### Phase 2: Profile Management
- [ ] Implement `claude-provider` CLI tool
- [ ] Create profile storage format
- [ ] Add profile switching functionality

### Phase 3: Advanced Features
- [ ] Auto-detection of available providers
- [ ] Fallback provider configuration
- [ ] Provider health checking

### Phase 4: Documentation & Examples
- [ ] Provider-specific setup guides
- [ ] Sample devcontainer.json configurations
- [ ] Troubleshooting guide

---

## References

### Official Documentation
- [Claude Code Settings](https://code.claude.com/docs/en/settings)
- [Dev Container Features Specification](https://devcontainers.github.io/implementors/features/)
- [Dev Container Specification](https://devcontainers.github.io/implementors/spec/)

### Provider Documentation
- [Z.ai API Reference](https://docs.z.ai/api-reference/introduction)
- [Z.ai GLM-4.7 Guide](https://docs.z.ai/guides/llm/glm-4.7)
- [Zhipu AI OpenAI Compatibility](https://docs.bigmodel.cn/cn/guide/develop/openai/introduction)
- [OpenRouter Quick Start](https://openrouter.ai/docs/quickstart)
- [OpenRouter Claude Code Integration](https://openrouter.ai/docs/guides/guides/claude-code-integration)
- [LiteLLM Providers](https://docs.litellm.ai/docs/providers)
- [LiteLLM Proxy Config](https://docs.litellm.ai/docs/proxy/configs)

### Community Resources
- [GitHub: Custom Endpoint Discussion](https://github.com/zed-industries/zed/discussions/37842)
- [Reddit: Custom Base URL](https://www.reddit.com/r/ClaudeAI/comments/1l88015/how_to_set_custom_base_url_in_claude_code_like/)
- [Medium: Custom LLM Providers in Claude Code](https://imfing.com/til/use-custom-llm-providers-in-claude-code/)

---

**Document Version:** 1.0
**Author:** Claude Code Research
**Last Updated:** 2026-02-11
