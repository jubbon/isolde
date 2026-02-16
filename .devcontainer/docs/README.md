# Claude Code Dev Container - Documentation

Welcome to the official documentation for Claude Code Dev Container.

## Quick Links

| Topic | Description |
|--------|-------------|
| [Architecture](architecture.md) | Project architecture and design |
| [Setup Guide](setup.md) | Installation and configuration |
| [Development](development.md) | Contributing and workflow |
| [Proxy Configuration](proxy.md) | Proxy settings and architecture |
| [LLM Providers](providers.md) | Multi-provider LLM support |
| [Version Control](claude-version-control.md) | Managing Claude Code versions |

## Overview

Claude Code Dev Container is a Docker-based development environment that provides isolated containerized environments for using Claude Code CLI. The project supports multiple LLM providers and includes proxy support for enterprise environments.

### Key Features

- **Pre-installed Claude Code CLI** - Ready to use immediately
- **Docker-in-Docker** - Full container capabilities within devcontainer
- **Multi-Provider Support** - Anthropic, Z.ai, and custom LLM providers
- **Proxy Configuration** - Flexible proxy settings for enterprise networks
- **VS Code Integration** - Seamless development experience

## Project Structure

```
.
├── .devcontainer/           # Dev Container configuration (for this repo)
│   ├── devcontainer.json    # Main config (proxy, mounts, features)
│   ├── Dockerfile           # Base image (Debian + system deps)
│   ├── PROXY_ARCHITECTURE.md # Proxy architecture docs
│   ├── docs/                # This documentation
│   └── features/
│       └── claude-code/     # Custom Claude Code feature
│           ├── devcontainer-feature.json
│           ├── install.sh
│           └── README.md
├── core/                    # Shared components for templates
│   ├── features/            # Reusable devcontainer features
│   └── base-Dockerfile      # Base container image
├── templates/               # Language templates
│   ├── python/
│   ├── nodejs/
│   ├── rust/
│   ├── go/
│   └── generic/
├── scripts/                 # Project creation tools
│   ├── init-project.sh      # Main script
│   └── lib/                 # Helper libraries
├── docs/                    # Template system documentation
│   ├── usage.md
│   ├── templates.md
│   ├── presets.md
│   └── backlog.md
├── .claude/                 # Claude Code local settings
├── CLAUDE.md                # Project instructions for Claude Code
├── README.md                # Project overview
└── presets.yaml             # Built-in presets
```

## Getting Started

### For VS Code Users

1. Open this folder in VS Code
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Press `F1` → `Dev Containers: Reopen in Container`

### For CLI Users

```bash
# From the repository root
# Build the image
docker build -t claude-code-dev .devcontainer

# Run with current workspace mounted
docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v "$PWD:/workspaces/$(basename "$PWD")" \
  claude-code-dev
```

### Creating New Projects

Use the template system to create new projects:

```bash
# Interactive wizard
./scripts/init-project.sh

# With preset
./scripts/init-project.sh my-app --preset=python-ml

# With template
./scripts/init-project.sh my-api --template=nodejs --lang-version=22
```

See [Template System Documentation](../../docs/) for more details.

## Documentation Sections

- **[Architecture](architecture.md)** - Learn about the system design, component interactions, and technical decisions
- **[Setup Guide](setup.md)** - Step-by-step installation and configuration instructions
- **[Development](development.md)** - Workflow guide for contributors and developers
- **[Proxy Configuration](proxy.md)** - Understanding and configuring proxy settings
- **[LLM Providers](providers.md)** - Setting up and using different LLM providers

## Conventions

This project follows specific conventions documented in [CLAUDE.md](../CLAUDE.md) (in .devcontainer directory):

- **Atomic Commits** - One logical change per commit
- **Conventional Commits** - Structured commit message format
- **Pre-commit Verification** - Build testing before commits
- **English Documentation** - All documentation in English

## Support

For issues or questions:

1. Check existing documentation in this directory
2. Review [CLAUDE.md](../CLAUDE.md) for project-specific guidance
3. Consult the [troubleshooting section](setup.md#troubleshooting) in setup guide
4. See [Template System Documentation](../../docs/) for template-related issues

## License

MIT License - See project root for details.
