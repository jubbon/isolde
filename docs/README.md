# Isolde Documentation

Welcome to the Isolde documentation. Isolde (ISOLated Development Environment) is a Rust CLI tool for creating isolated development environments with coding agent support.

## 📖 Documentation by Audience

### For Users
Creating projects and using Isolde templates and presets.
- [Quick Start](user/quick-start.md) - Get started in 5 minutes
- [Usage Guide](user/usage.md) - Detailed command reference
- [Templates](user/templates.md) - Available language templates
- [Presets](user/presets.md) - Pre-configured combinations

### Devcontainer Configuration
Advanced configuration for development containers.
- [Setup](devcontainer/setup.md) - Container setup and build
- [Providers](devcontainer/providers.md) - LLM provider configuration
- [Proxy](devcontainer/proxy.md) - HTTP proxy configuration
- [Version Control](devcontainer/version-control.md) - Managing agent CLI versions

### For Contributors
Contributing to Isolde itself.
- [Architecture](contributor/architecture.md) - System design and components
- [Development](contributor/development.md) - Development workflow
- [Testing](contributor/testing.md) - Test coverage and CI/CD

### Design Documents
Planning and design discussions.
- [Plans Index](plans/) - All design documents
- [Backlog](backlog.md) - Feature roadmap

## Quick Links

- [Project README](../README.md) - Main project README
- [CLAUDE.md](../CLAUDE.md) - Instructions for Claude Code

## Quick Start

```bash
# Install from source
git clone <repository-url>
cd isolde
cargo install --path .

# Create a new project
isolde init my-app --preset python-ml

# Generate devcontainer configuration
cd my-app
isolde sync

# Build and run the container (requires devcontainers CLI)
isolde build
isolde run

# Or use the interactive wizard
isolde init
```

See [Quick Start](user/quick-start.md) for more details.

## Container Management

Isolde provides built-in commands for building and managing devcontainers:

- `isolde build` - Build the devcontainer image
- `isolde run` - Start and enter the container
- `isolde exec` - Execute commands in a running container
- `isolde stop` - Stop a running container
- `isolde ps` - List running containers
- `isolde logs` - View container logs

These commands require the [devcontainers CLI](https://github.com/devcontainers/cli) to be installed.

```bash
# Install from source
git clone <repository-url>
cd isolde
cargo install --path .

# Create a new project
isolde init my-app --preset python-ml

# Or use the interactive wizard
isolde init
```

See [Quick Start](user/quick-start.md) for more details.
