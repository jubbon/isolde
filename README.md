# Isolde

**ISOLated Development Environment** - A template-based system for creating isolated development environments with coding agent support (Claude Code, Codex, Gemini, Aider).

> **Note:** This project is under active development. You may encounter bugs or incomplete features. If you find any issues, please [open an issue](https://github.com/jubbon/isolde/issues) — all bug reports and feedback are appreciated!

## Overview

Isolde provides templates and tools for quickly setting up isolated development environments. Each project gets a dedicated devcontainer with language-specific tooling, a single git repository, and a pre-configured coding agent CLI (Claude Code by default, with Codex, Gemini, and Aider support planned).

---

## Quick Start

### For Users: Create a New Project

#### Installation

Pre-built binaries are not yet available. Build from source using the instructions below.

**From Source**
```bash
git clone https://github.com/jubbon/isolde.git
cd isolde
make install
export PATH="$HOME/.local/bin:$PATH"
isolde --version
```

#### Create Projects

```bash
# Interactive wizard
isolde init

# With a preset
isolde init my-ml-app --preset python-ml

# With a specific template
isolde init my-api --template nodejs --lang-version 22

# List options
isolde init --list-templates
isolde init --list-presets
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
- ✅ **Coding agent integration** - Claude Code supported, Codex/Gemini/Aider planned
- ✅ **Single git repository** - Code and devcontainer config tracked together
- ✅ **Docker-in-Docker support** - Build containers within the dev container
- ✅ **Enterprise proxy configuration** - HTTP/HTTPS proxy support for corporate networks
- ✅ **Version control options** - Pin specific agent CLI versions or track latest
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

## Supported Coding Agents

| Agent | Status | Description |
|-------|--------|-------------|
| **Claude Code** | Supported | Anthropic CLI with multi-provider LLM support (Anthropic, Z.ai, custom) |
| **Codex** | Planned | OpenAI Codex CLI |
| **Gemini** | Planned | Google Gemini CLI |
| **Aider** | Planned | AI pair programming tool |

Claude Code is the default agent. Use `--agent <name>` to select a different agent:

```bash
isolde init my-app --template python --agent claude-code
```

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
| [Version Control](docs/devcontainer/version-control.md) | Managing agent CLI versions |

---

## Repository Structure

```
isolde/
├── isolde-core/           # Core library (template processing, git ops, config)
│   ├── src/
│   └── Cargo.toml
├── isolde-cli/            # CLI binary (clap-based)
│   ├── src/
│   └── Cargo.toml
├── core/                  # Shared components
│   ├── features/          # Reusable devcontainer features
│   │   ├── claude-code/   # Claude Code CLI (default agent)
│   │   ├── codex/         # OpenAI Codex CLI (planned)
│   │   ├── gemini/        # Google Gemini CLI (planned)
│   │   ├── aider/         # Aider AI pair programming (planned)
│   │   ├── proxy/         # HTTP proxy configuration
│   │   └── plugin-manager/ # Plugin activation (claude-code only)
│   └── base-Dockerfile    # Base container image
├── templates/             # Language templates
│   ├── python/
│   ├── nodejs/
│   ├── rust/
│   ├── go/
│   └── generic/
├── Cargo.toml             # Workspace configuration
├── Cargo.lock             # Dependency lock file
├── presets.yaml           # Built-in presets
├── docs/                  # Documentation
├── tests/                 # E2E tests
├── mk/                    # Makefile modules
├── Makefile               # Build and test commands
├── CLAUDE.md              # Claude Code project instructions
└── README.md              # This file
```

### Created Project Structure

```
~/workspace/my-project/   # Single git repository
├── .git/                 # Version control for code and config
├── .devcontainer/        # Devcontainer configuration
│   ├── devcontainer.json
│   ├── Dockerfile
│   └── features/         # Copied agent and proxy features
├── .claude/              # Claude Code config
│   └── CLAUDE.md         # Project-specific agent instructions
├── .isolde/              # Isolde state (volumes, isolation data)
│   └── volumes/
├── project/              # Workspace directory (bind-mounted into container)
└── isolde.yaml           # Isolde project configuration
```

---

## Configuration

### LLM Provider (Claude Code)

Configure the LLM provider for the Claude Code agent in `devcontainer.json`:

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

### Agent Version Control

Choose which agent CLI version to install (example for Claude Code):

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
| `make test` | Run Rust tests + lint |
| `make test-docker` | Run Docker container tests |
| `make test-all` | Run everything (Rust + Docker + E2E) |
| `make lint` | Run linting checks |
| `make test-e2e` | E2E tests (Docker-based) |
| `make clean` | Remove running containers |
| `make clean-all` | Full cleanup |

### Quick Test Reference

```bash
# Run Rust tests + lint
make test

# Validate JSON files
make lint-json

# Run Docker container tests
make test-docker

# Run E2E tests
make test-e2e SCENARIO="Create Python project"

# Run everything
make test-all
```

### Docker-in-Docker

The container mounts `/var/run/docker.sock` for Docker-in-Docker support:
- Build containers within the dev container
- Run docker commands without sudo
- Test Docker-based projects in isolation

Ensure your user has Docker permissions on the host machine.

### Contributing

Pull requests are welcome! Please check the [backlog](docs/backlog.md) for planned features or open an issue to discuss your idea before submitting a PR.

### Commit Standards

This project follows specific conventions documented in [CLAUDE.md](CLAUDE.md):
- **Atomic commits** - One logical change per commit
- **Conventional commits** - Structured commit message format
- **Pre-commit verification** - Build testing before commits
- **English documentation** - All docs in English

### Updating

```bash
cd isolde
git pull origin main
make install
```

---

## Requirements

- **Rust toolchain** (for building from source) — install via [rustup](https://rustup.rs/)
- **Docker** — for running devcontainers
- **[devcontainers CLI](https://github.com/devcontainers/cli)** — for `isolde build`, `isolde run`, etc.
- **Git**
- VS Code with Dev Containers extension (optional, recommended)

---

## License

MIT License - see LICENSE file for details.
