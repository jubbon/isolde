# Isolde Documentation

Welcome to the documentation for Isolde (ISOLated Development Environment).

This system provides templates and tools for quickly setting up isolated development environments with Claude Code CLI support.

## Quick Links

| Document | Description |
|----------|-------------|
| [Usage Guide](usage.md) | Detailed usage instructions for isolde.sh |
| [Template Reference](templates.md) | Template customization and metadata |
| [Preset Reference](presets.md) | Available presets and creating custom ones |
| [Backlog](backlog.md) | Planned features and improvements |

## Overview

This template system provides:

- **Multiple Language Templates** - Python, Node.js, Rust, Go, and generic
- **Pre-configured Presets** - Common setups for ML, web, API, CLI development
- **Multi-Provider LLM Support** - Anthropic, Z.ai, and custom providers
- **Proxy Configuration** - Enterprise network support
- **Interactive Wizard** - Easy project creation with prompts

## Quick Start

```bash
# From the repository root
./scripts/isolde.sh

# Or with a preset
./scripts/isolde.sh my-ml-app --preset=python-ml

# Or with specific template
./scripts/isolde.sh my-api --template=nodejs --lang-version=22
```

## Development

### Using the Makefile

The root-level Makefile provides convenient targets for building, testing, and development:

```bash
# Build the devcontainer image
make

# Run all tests (CI parity)
make test

# Run linting checks
make lint

# Run specific test categories
make test-e2e              # E2E tests (Docker-based)
make test-build            # Container builds
make test-config           # Environment configuration

# Cleanup
make clean                 # Remove containers
make clean-all             # Full cleanup

# Show all available targets
make help
```

## Project Structure

After creating a project, your workspace will look like:

```
~/workspace/my-project/
├── project/              # Git repository for your code
│   ├── README.md
│   └── .gitignore
├── .devcontainer/        # Git repository for devcontainer config
│   ├── devcontainer.json
│   ├── Dockerfile
│   ├── features/         # Copied from template repository
│   └── README.md
└── .claude/              # Claude Code config (not in git)
```

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

## Documentation

- [Usage Guide](usage.md) - Detailed usage instructions
- [Template Reference](templates.md) - Template customization
- [Preset Reference](presets.md) - Available presets and creating custom ones
- [Architecture](architecture.md) - System design

## Repository Structure

```
claude-code-templates/
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
│   ├── isolde.sh    # Main script
│   └── lib/               # Helper libraries
├── presets.yaml           # Built-in presets
└── docs/                  # Documentation
```

## Requirements

- Docker
- VS Code with Dev Containers extension
- Bash shell
- Git

## License

MIT License - see LICENSE file for details.
