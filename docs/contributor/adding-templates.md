# Adding Templates

How to add new language templates to Isolde.

## Template Structure

Each template lives in `templates/{name}/`:

```
templates/python/
├── .devcontainer/
│   └── devcontainer.json    # Container configuration (with placeholders)
└── template-info.yaml        # Template metadata
```

## template-info.yaml

Defines template metadata:

```yaml
name: Python                    # Human-readable name
description: Python development environment with uv, pytest, and ruff
version: 1.0.0
lang_version_default: "3.12"    # Default version
features:
  - name: uv
    description: Fast Python package installer
supported_versions:
  - "3.12"
  - "3.11"
  - "3.10"
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Human-readable template name |
| `description` | string | Yes | Brief description of the template |
| `version` | string | Yes | Template version (semver) |
| `lang_version_default` | string | Yes | Default language version |
| `features` | array | No | List of additional features |
| `supported_versions` | array | Yes | Valid language versions |

## devcontainer.json Placeholders

Templates use placeholders for substitution during project creation:

| Placeholder | Description |
|-------------|-------------|
| `{{PROJECT_NAME}}` | Project name |
| `{{PYTHON_VERSION}}`, `{{NODE_VERSION}}`, etc. | Language version from `--lang-version` |
| `{{FEATURES_CLAUDE_CODE}}` | Replaced with `./features/claude-code` |
| `{{FEATURES_PROXY}}` | Replaced with `./features/proxy` |
| `{{CLAUDE_VERSION}}` | Claude Code CLI version (from `--claude-version`) |
| `{{CLAUDE_PROVIDER}}` | Claude provider (from `--claude-provider`) |
| `{{HTTP_PROXY}}`, `{{HTTPS_PROXY}}` | Proxy settings from options |

### Example devcontainer.json

```json
{
  "name": "{{PROJECT_NAME}}",
  "image": "python:{{PYTHON_VERSION}}-slim",
  "features": {
    "{{FEATURES_CLAUDE_CODE}}": {
      "version": "{{CLAUDE_VERSION}}",
      "provider": "{{CLAUDE_PROVIDER}}"
    }
  },
  "forwardPorts": [8000]
}
```

Substitutions happen in `apply_template_substitutions()` via `sed` during project creation.

## Core Features

Reusable features in `core/features/`:

- `claude-code/` - Claude Code CLI installation with multi-provider support
- `proxy/` - HTTP proxy configuration for enterprise networks

### Feature Path Resolution

Features are **copied** (not symlinked) to each project's `.devcontainer/features/` directory because:
- Docker's build context cannot follow symlinks outside the build directory
- Each project needs its own copy of the feature files

In created projects, features are referenced as `./features/claude-code` (relative to `.devcontainer/`).

## Adding a New Template

### Step 1: Create Template Directory

```bash
mkdir -p templates/{name}/.devcontainer
```

### Step 2: Create template-info.yaml

```bash
cat > templates/{name}/template-info.yaml << 'EOF'
name: Your Language
description: Brief description
version: 1.0.0
lang_version_default: "1.0"
features:
  - name: example-feature
    description: What this feature does
supported_versions:
  - "1.0"
  - "0.9"
EOF
```

### Step 3: Create devcontainer.json

Create `templates/{name}/.devcontainer/devcontainer.json` with placeholders:

```json
{
  "name": "{{PROJECT_NAME}} - Your Language Environment",

  "build": {
    "dockerfile": "Dockerfile",
    "context": "..",
    "args": {
      "USERNAME": "${localEnv:USER}",
      "LANGUAGE_VERSION": "{{LANGUAGE_VERSION}}"
    }
  },

  "features": {
    "{{FEATURES_CLAUDE_CODE}}": {
      "version": "{{CLAUDE_VERSION}}",
      "provider": "{{CLAUDE_PROVIDER}}",
      "models": "{{CLAUDE_MODELS}}",
      "http_proxy": "{{HTTP_PROXY}}",
      "https_proxy": "{{HTTPS_PROXY}}"
    },
    "{{FEATURES_PROXY}}": {
      "http_proxy": "{{HTTP_PROXY}}",
      "https_proxy": "{{HTTPS_PROXY}}",
      "no_proxy": "{{NO_PROXY}}",
      "enabled": {{PROXY_ENABLED}}
    }
  },

  "customizations": {
    "vscode": {
      "extensions": [
        "anthropic.claude-code"
      ],
      "settings": {
        "terminal.integrated.defaultProfile.linux": "bash"
      }
    }
  },

  "mounts": [
    "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind,consistency=cached",
    "source=${localEnv:HOME}/.claude.json,target=/home/${localEnv:USER}/.claude.json,type=bind,consistency=cached",
    "source=${localEnv:HOME}/.config/devcontainer/machine-id,target=/etc/machine-id,type=bind,consistency=cached"
  ],

  "remoteUser": "${localEnv:USER}",
  "workspaceFolder": "/workspaces/{{PROJECT_NAME}}"
}
```

### Step 4: Create Dockerfile

Create `templates/{name}/.devcontainer/Dockerfile`:

```dockerfile
ARG BASE_IMAGE=debian:bookworm-slim
FROM ${BASE_IMAGE}

# User arguments with defaults
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=1000
ARG LANGUAGE_VERSION=latest

# Set DEBIAN_FRONTEND for non-interactive apt
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    wget \
    vim \
    jq \
    build-essential \
    ca-certificates \
    gosu \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspaces

# Create user with sudo access
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USERNAME} --shell /bin/bash --create-home ${USERNAME} \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set ownership for workspace directory
RUN chown -R ${USERNAME}:${USERNAME} /workspaces

USER ${USERNAME}
```

### Step 5: Test Your Template

```bash
./scripts/isolde.sh test-project --template=your-language --lang-version=1.0.0
```

Verify:
1. Project structure is correct
2. Container builds successfully: `cd test-project/.devcontainer && docker build -t test .`
3. Substitutions were applied: `cat test-project/.devcontainer/devcontainer.json`

### Step 6: Add a Preset (Optional)

Add your template to `presets.yaml` for easy access:

```yaml
your-language-productive:
  template: your-language
  lang_version: "1.0.0"
  description: Productive your-language development setup
  features:
    - name: tool1
      description: Tool 1 description
```

## Testing Guidelines

### Validate Template Files

```bash
# Validate JSON
jq < templates/{name}/.devcontainer/devcontainer.json

# Check Dockerfile syntax
docker build --no-cache -f templates/{name}/.devcontainer/Dockerfile .

# Test shell scripts
shellcheck templates/{name}/.devcontainer/*.sh
```

### Test Project Creation

```bash
# Test with specific template
./scripts/isolde.sh test-project --template=your-language

# Test with preset
./scripts/isolde.sh test-project --preset=your-preset

# Verify created project
ls -la test-project/
ls -la test-project/.devcontainer/
```

### Test Container Build

```bash
cd test-project/.devcontainer
docker build -t test-your-language .
docker run --rm test-your-language <language-command> --version
```

## Custom Features

If your template needs custom features beyond `claude-code` and `proxy`, create them in `core/features/`:

```
core/features/your-feature/
├── devcontainer-feature.json    # Feature metadata
└── install.sh                   # Installation script
```

Example `devcontainer-feature.json`:

```json
{
  "id": "your-feature",
  "version": "1.0.0",
  "name": "Your Feature",
  "description": "Description of what this feature does",
  "options": {
    "version": {
      "type": "string",
      "default": "latest",
      "description": "Version to install"
    }
  },
  "entrypoint": "install.sh"
}
```

Reference your custom feature in `devcontainer.json`:

```json
"features": {
  "./features/your-feature": {
    "version": "latest"
  }
}
```

## Common Patterns

### Language-Specific Tools

Use `postCreateCommand` to install language tools:

```json
"postCreateCommand": "bash -c 'pip install --user uv pytest ruff'"
```

### VS Code Extensions

Add language-specific extensions:

```json
"customizations": {
  "vscode": {
    "extensions": [
      "anthropic.claude-code",
      "ms-python.python",
      "ms-python.pylint"
    ]
  }
}
```

## Documentation Updates

After creating your template, update:

1. `docs/user/templates.md` - Add template to the list
2. `presets.yaml` - Add any useful presets
3. `CLAUDE.md` - Update if new patterns are introduced
