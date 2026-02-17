# Template Reference

## Overview

Templates define the base configuration for different programming languages and frameworks. Each template includes:

- A Dockerfile with language-specific dependencies
- A devcontainer.json with VS Code settings
- Feature definitions
- Metadata in `template-info.yaml`

## Template Structure

```
templates/python/
├── template-info.yaml     # Template metadata
└── .devcontainer/
    ├── Dockerfile         # Container image
    └── devcontainer.json  # Devcontainer config
```

The `template-info.yaml` file defines template properties, supported versions, and features.

## Template Metadata

Each template has a `template-info.yaml`:

```yaml
name: Python
description: Python development environment with uv, pytest, and ruff
version: 1.0.0
lang_version_default: "3.12"
features:
  - name: uv
    description: Fast Python package installer
  - name: pytest
    description: Testing framework
  - name: ruff
    description: Fast linter and formatter
supported_versions:
  - "3.12"
  - "3.11"
  - "3.10"
devcontainer_features:
  - ghcr.io/devcontainers/features/python:1
  - ghcr.io/devcontainers/features/common-utils:2
```

## Placeholder Substitutions

The `devcontainer.json` template supports these placeholders:

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{PROJECT_NAME}}` | Project name | `my-app` |
| `{{PYTHON_VERSION}}` | Python version | `3.12` |
| `{{NODE_VERSION}}` | Node.js version | `22` |
| `{{RUST_VERSION}}` | Rust version | `latest` |
| `{{GO_VERSION}}` | Go version | `latest` |
| `{{FEATURES_CLAUDE_CODE}}` | Claude Code feature path | `./features/claude-code` |
| `{{FEATURES_PROXY}}` | Proxy feature path | `./features/proxy` |
| `{{CLAUDE_VERSION}}` | Claude Code version | `latest` |
| `{{CLAUDE_PROVIDER}}` | LLM provider | `z.ai` |
| `{{CLAUDE_MODELS}}` | Model mapping | `haiku:model,sonnet:model` |
| `{{HTTP_PROXY}}` | HTTP proxy URL | `http://proxy:8080` |
| `{{HTTPS_PROXY}}` | HTTPS proxy URL | `http://proxy:8080` |
| `{{NO_PROXY}}` | No proxy hosts | `localhost,127.0.0.1,.local` |
| `{{PROXY_ENABLED}}` | Proxy enabled | `true` |

## Built-in Templates

### Python

**Language**: Python
**Default Version**: 3.12
**Features**: uv, pytest, ruff

```bash
./scripts/isolde.sh my-app --template=python --lang-version=3.12
```

**DevContainer Features**:
- Python 3.12 with pip
- uv (fast package installer)
- pytest (testing)
- ruff (linting/formatting)

### Node.js

**Language**: JavaScript/TypeScript
**Default Version**: 22
**Features**: TypeScript, ESLint, Vitest

```bash
./scripts/isolde.sh my-app --template=nodejs --lang-version=22
```

**DevContainer Features**:
- Node.js 22 with npm
- TypeScript compiler
- ESLint (linting)
- Vitest (testing)

### Rust

**Language**: Rust
**Default Version**: latest
**Features**: cargo, clippy

```bash
./scripts/isolde.sh my-app --template=rust
```

**DevContainer Features**:
- Rust toolchain with cargo
- clippy (linting)
- rustfmt (formatting)

### Go

**Language**: Go
**Default Version**: latest
**Features**: modules, golangci-lint

```bash
./scripts/isolde.sh my-app --template=go
```

**DevContainer Features**:
- Go toolchain
- golangci-lint (linting)

### Generic

**Language**: None
**Default Version**: N/A
**Features**: None

```bash
./scripts/isolde.sh my-app --template=generic
```

**DevContainer Features**:
- Base Debian image
- Common utilities (git, curl, jq)

## Creating Custom Templates

1. Create a new directory in `templates/`:

```bash
mkdir templates/my-language
mkdir templates/my-language/.devcontainer
```

2. Create `template-info.yaml`:

```yaml
name: My Language
description: My custom language environment
version: 1.0.0
lang_version_default: "1.0"
features:
  - name: linter
    description: My linter
supported_versions:
  - "1.0"
  - "2.0"
devcontainer_features:
  - ghcr.io/devcontainers/features/common-utils:2
```

3. Create `.devcontainer/Dockerfile` based on `core/base-Dockerfile`:

```dockerfile
ARG BASE_IMAGE=debian:bookworm-slim
FROM ${BASE_IMAGE}

ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=1000
ARG MYLANG_VERSION=1.0

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    git \
    # Add your language's dependencies here
    && rm -rf /var/lib/apt/lists/*

# ... (user setup)
```

4. Create `.devcontainer/devcontainer.json` with placeholders

5. Add template processing logic to `scripts/lib/templates.sh` if needed

6. Test the template:

```bash
./scripts/isolde.sh test --template=my-language
```

## Template Features

Templates use devcontainer features for modular functionality. The `core/features/` directory contains:

### claude-code (`core/features/claude-code/`)

Installs Claude Code CLI with multi-provider support.

**Options**:
- `version`: `latest`, `stable`, or specific version
- `provider`: LLM provider (e.g., `z.ai`, `anthropic`)
- `models`: Model mapping string
- `http_proxy`, `https_proxy`: Proxy for build-time downloads

### proxy (`core/features/proxy/`)

Configures HTTP proxy for the container.

**Options**:
- `http_proxy`: HTTP proxy URL
- `https_proxy`: HTTPS proxy URL (defaults to http_proxy)
- `no_proxy`: Hosts to bypass proxy
- `apt_proxy`: Configure apt to use proxy
- `enabled`: Enable/disable proxy

## Feature Paths in Projects

Projects copy features from the core template repository:

```
my-project/.devcontainer/features/
├── claude-code/    # Copied from core/features/claude-code
└── proxy/          # Copied from core/features/proxy
```

Note: Features are copied rather than symlinked because Docker cannot follow symlinks outside the build context. Each project gets its own copy of the features.
