# Isolde

**ISOLated Development Environment** - A template-based system for creating isolated development environments with Claude Code CLI support.

## Overview

Isolde provides templates and tools for quickly setting up isolated development environments. Each project gets a dedicated devcontainer with language-specific tooling, isolated git repositories, and pre-configured Claude Code CLI with multi-provider support.

---

## Quick Start

### For Users: Create a New Project

#### Installation

**Quick Install (Recommended)**
```bash
curl -fsSL https://raw.githubusercontent.com/jubbon/isolde/main/scripts/install/install.sh | bash
source ~/.bashrc  # or source ~/.zshrc
isolde --version
```

**From Source**
```bash
git clone https://github.com/jubbon/isolde.git
cd isolde
./scripts/isolde.sh
```

#### Create Projects

```bash
# Interactive wizard
isolde

# With a preset
isolde my-ml-app --preset=python-ml

# With a specific template
isolde my-api --template=nodejs --lang-version=22

# List options
isolde --list-templates
isolde --list-presets
```

### For Contributors: Development Setup

#### VS Code (Recommended)

1. Open this folder in VS Code
2. Install [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Press `F1` → `Dev Containers: Reopen in Container`

#### CLI

```bash
# Build the image
make build

# Run in interactive mode
make devcontainer

# Get a shell inside the container
make shell
```

---

## Features

- ✅ **Template-based project creation** - Language-specific devcontainers with pre-configured tooling
- ✅ **Claude Code CLI pre-installed** - Multi-provider LLM support (Anthropic, Z.ai, custom)
- ✅ **Dual git repositories** - Separate version control for code and devcontainer config
- ✅ **Docker-in-Docker support** - Build containers within the dev container
- ✅ **Enterprise proxy configuration** - HTTP/HTTPS proxy support for corporate networks
- ✅ **Version control options** - Pin specific Claude Code versions or track latest
- ✅ **Isolated development** - Each project in its own container with dedicated dependencies

---

## Available Templates

| Template | Description | Versions |
|----------|-------------|----------|
| **python** | Python with uv, pytest, ruff | 3.12, 3.11, 3.10 |
| **nodejs** | Node.js with TypeScript, ESLint, Vitest | 22, 20, 18 |
| **rust** | Rust with cargo, clippy | latest, stable |
| **go** | Go with golangci-lint | latest, 1.22, 1.21 |
| **generic** | Minimal container for any project | - |

## Available Presets

| Preset | Template | Description |
|--------|----------|-------------|
| **python-ml** | Python 3.12 | ML with Jupyter, numpy, pandas |
| **python-web** | Python 3.11 | Web with pytest, ruff |
| **node-api** | Node.js 22 | API with TypeScript, Vitest |
| **rust-cli** | Rust | CLI with clippy |
| **go-service** | Go | Microservice with golangci-lint |
| **minimal** | Generic | Minimal setup |
| **fullstack** | Node.js 22 | Full-stack with security plugins |

---

## Documentation

**Main Documentation**: See [docs/README.md](docs/README.md) for the complete documentation index.

### For Users
| Document | Description |
|----------|-------------|
| [Usage Guide](docs/user/usage.md) | Detailed usage instructions |
| [Template Reference](docs/user/templates.md) | Template customization |
| [Preset Reference](docs/user/presets.md) | Available presets and creating custom ones |
| [Backlog](docs/backlog.md) | Planned features and improvements |

### For Contributors
| Document | Description |
|----------|-------------|
| [Devcontainer Setup](docs/devcontainer/setup.md) | Installation and configuration |
| [Architecture](docs/contributor/architecture.md) | System architecture and design |
| [Development](docs/contributor/development.md) | Contributing and workflow |
| [Testing](docs/contributor/testing.md) | Test documentation |
| [LLM Providers](docs/devcontainer/providers.md) | Multi-provider LLM support |
| [Proxy Configuration](docs/devcontainer/proxy.md) | Enterprise proxy setup |
| [Version Control](docs/devcontainer/version-control.md) | Managing Claude Code versions |

---

## Repository Structure

```
isolde/
├── core/                  # Shared components
│   ├── features/          # Reusable devcontainer features
│   │   ├── claude-code/   # Claude Code CLI installation
│   │   └── proxy/         # HTTP proxy configuration
│   └── base-Dockerfile    # Base container image
├── templates/             # Language templates
│   ├── python/
│   ├── nodejs/
│   ├── rust/
│   ├── go/
│   └── generic/
├── scripts/               # Project creation tools
│   ├── isolde.sh          # Main script
│   ├── install/           # Installation scripts
│   └── lib/               # Helper libraries
├── presets.yaml           # Built-in presets
├── docs/                  # Documentation
├── tests/                 # E2E tests
├── .devcontainer/         # Self devcontainer setup
├── mk/                    # Makefile modules
├── Makefile               # Root-level Makefile
├── VERSION                # Version file
├── CLAUDE.md              # Claude Code project instructions
└── README.md              # This file
```

### Created Project Structure

```
~/workspace/my-project/
├── project/              # Git repository for your code
│   ├── README.md
│   └── .gitignore
├── .devcontainer/        # Git repository for devcontainer config
│   ├── devcontainer.json
│   ├── Dockerfile
│   ├── features/
│   └── README.md
└── .claude/              # Claude Code config (not in git)
```

---

## Configuration

### LLM Provider

Configure your LLM provider in `devcontainer.json`:

```json
{
  "features": {
    "./features/claude-code": {
      "provider": "anthropic"
    }
  }
}
```

Available providers: `anthropic` (default), `z.ai`, or custom. See [docs/devcontainer/providers.md](docs/devcontainer/providers.md) for detailed setup.

### Proxy Configuration

For enterprise environments, configure proxy in `devcontainer.json`:

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

See [docs/devcontainer/proxy.md](docs/devcontainer/proxy.md) for details.

### Version Control

Choose which Claude Code version to install:

```json
{
  "features": {
    "./features/claude-code": {
      "version": "latest"
    }
  }
}
```

| Version | Behavior |
|---------|-----------|
| `latest` (default) | Most recent release, auto-updates enabled |
| `stable` | Latest stable release, auto-updates disabled |
| `1.2.41` | Specific version, auto-updates disabled |

See [docs/devcontainer/version-control.md](docs/devcontainer/version-control.md) for details.

---

## Development

### Make Commands

| Command | Description |
|---------|-------------|
| `make help` | Show available commands |
| `make build` | Build the Docker image |
| `make devcontainer` | Run container with current workspace |
| `make shell` | Get shell in running container |
| `make test` | Run all tests (CI parity) |
| `make lint` | Run linting checks |
| `make test-e2e` | E2E tests (Docker-based) |
| `make clean` | Remove running containers |
| `make clean-all` | Full cleanup |

### Quick Test Reference

```bash
# Test container builds
make build

# Verify shell scripts
make lint-shell

# Validate JSON files
make lint-json

# Run E2E tests
make test-e2e SCENARIO="Create Python project"
```

### Docker-in-Docker

The container mounts `/var/run/docker.sock` for Docker-in-Docker support:
- Build containers within the dev container
- Run docker commands without sudo
- Test Docker-based projects in isolation

Ensure your user has Docker permissions on the host machine.

### Commit Standards

This project follows specific conventions documented in [CLAUDE.md](CLAUDE.md):
- **Atomic commits** - One logical change per commit
- **Conventional commits** - Structured commit message format
- **Pre-commit verification** - Build testing before commits
- **English documentation** - All docs in English

### Updating

**If installed via quick install:**
```bash
isolde --self-update
```

**If running from source:**
```bash
cd isolde
git pull origin main
```

---

## Requirements

- Docker
- VS Code with Dev Containers extension (recommended)
- Bash shell
- Git

---

## License

MIT License - see LICENSE file for details.
