# Codex CLI Feature

Installs OpenAI Codex CLI for use as a coding agent inside a devcontainer.

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `version` | string | `latest` | Codex CLI version to install (e.g. `0.1.0`) |
| `http_proxy` | string | `""` | HTTP proxy URL for build-time npm install |
| `https_proxy` | string | `""` | HTTPS proxy URL for build-time npm install |

## Requirements

Codex CLI is distributed as an npm package and requires Node.js. Add the Node.js feature to your devcontainer before this feature:

```json
{
  "features": {
    "ghcr.io/devcontainers/features/node:1": {},
    "./features/codex": {}
  }
}
```

Isolde automatically includes the Node.js feature when `agent: codex` is configured in `isolde.yaml`.

## Usage

In your `isolde.yaml`:

```yaml
agent:
  name: codex
  version: latest
  options: {}
```

Then run `isolde sync` to generate the devcontainer configuration.

## Authentication

Codex CLI uses the `OPENAI_API_KEY` environment variable. Set it in your shell before starting the container, or add it to your devcontainer environment:

```json
{
  "containerEnv": {
    "OPENAI_API_KEY": "${localEnv:OPENAI_API_KEY}"
  }
}
```

## Proxy Configuration

The feature supports a hybrid proxy approach:

### Proxy Priority

1. Shared state file (`~/.config/devcontainer/proxy`) — written by the `./features/proxy` feature at runtime
2. Direct options (`http_proxy`, `https_proxy`) — build-time fallback
3. Global environment variables (`HTTP_PROXY`, `HTTPS_PROXY`) — ultimate fallback

### Build-Time Proxy Example

```json
{
  "features": {
    "./features/codex": {
      "http_proxy": "http://proxy.example.com:8080",
      "https_proxy": "http://proxy.example.com:8080"
    }
  }
}
```
