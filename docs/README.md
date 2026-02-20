# Isolde Documentation

Welcome to the Isolde documentation. Isolde (ISOLated Development Environment) is a Rust CLI tool for creating isolated development environments with Claude Code CLI support.

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
- [Providers](devcontainer/providers.md) - Claude Code provider configuration
- [Proxy](devcontainer/proxy.md) - HTTP proxy configuration
- [Version Control](devcontainer/version-control.md) - Managing Claude versions

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

# Or use the interactive wizard
isolde init
```

See [Quick Start](user/quick-start.md) for more details.
