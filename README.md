# Isolde

ISOLated Development Environment - A template-based system for creating isolated development environments with Claude Code CLI support.

## Overview

This repository provides templates and tools for quickly setting up isolated development environments. Each project gets:

- A dedicated devcontainer with custom configurations
- Language-specific tooling and dependencies
- Isolated git repositories for project code and devcontainer config
- Pre-configured Claude Code CLI with multi-provider support

## Installation

### Quick Install (Recommended)

Install Isolde system-wide using the installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/jubbon/isolde/main/scripts/install/install.sh | bash
```

After installation, reload your shell:

```bash
source ~/.bashrc  # or source ~/.zshrc
```

Then verify:

```bash
isolde --version
isolde --help
```

### From Source

Clone and run directly from the repository:

```bash
git clone https://github.com/jubbon/isolde.git
cd isolde
./scripts/isolde.sh
```

### Updating

If installed via the quick install method:

```bash
isolde --self-update
```

If running from source, pull the latest changes:

```bash
cd isolde
git pull origin main
```

## Quick Start

Once installed, create a new project:

```bash
# Interactive wizard
isolde

# With a preset
isolde my-ml-app --preset=python-ml

# With a specific template
isolde my-api --template=nodejs --lang-version=22

# List available templates
isolde --list-templates

# List available presets
isolde --list-presets
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

**Main Documentation**: See [docs/README.md](docs/README.md) for the complete documentation index organized by audience.

### Template System Documentation
- [Usage Guide](docs/user/usage.md) - Detailed usage instructions
- [Template Reference](docs/user/templates.md) - Template customization
- [Preset Reference](docs/user/presets.md) - Available presets and creating custom ones
- [Backlog](docs/backlog.md) - Planned features and improvements

### Devcontainer Documentation
- [Devcontainer Setup](docs/devcontainer/setup.md) - Installation and configuration
- [Architecture](docs/contributor/architecture.md) - System architecture and design
- [Development](docs/contributor/development.md) - Contributing and workflow
- [LLM Providers](docs/devcontainer/providers.md) - Multi-provider LLM support
- [Proxy Configuration](docs/devcontainer/proxy.md) - Proxy settings and architecture
- [Version Control](docs/devcontainer/version-control.md) - Managing Claude Code versions

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
│   │   ├── install.sh     # Installation script
│   │   └── isolde-wrapper.sh  # Wrapper for installed Isolde
│   └── lib/               # Helper libraries
├── presets.yaml           # Built-in presets
├── docs/                  # Template system documentation
├── tests/                 # E2E tests
│   └── e2e/               # Behave test scenarios
├── .devcontainer/         # Self devcontainer setup
│   └── features/          # Feature implementations
├── mk/                    # Makefile modules
│   ├── common.mk          # Shared variables
│   ├── build.mk           # Build targets
│   ├── lint.mk            # Linting targets
│   ├── tests.mk           # Test targets
│   └── clean.mk           # Cleanup targets
├── Makefile               # Root-level Makefile
├── VERSION                # Version file
└── README.md              # This file
```

## Requirements

- Docker
- VS Code with Dev Containers extension (recommended)
- Bash shell
- Git

## License

MIT License - see LICENSE file for details.
