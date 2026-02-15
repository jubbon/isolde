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
├── .devcontainer/           # Dev Container configuration
│   ├── devcontainer.json    # Main config (proxy, mounts, features)
│   ├── Dockerfile           # Base image (Debian + system deps)
│   ├── PROXY_ARCHITECTURE.md # Proxy architecture docs
│   └── features/
│       └── claude-code/     # Custom Claude Code feature
│           ├── devcontainer-feature.json
│           ├── install.sh
│           └── README.md
├── docs/                    # This documentation
├── .claude/                 # Claude Code local settings
├── .worktrees/              # Git worktree storage
├── CLAUDE.md                 # Project instructions for Claude Code
├── README.md                 # Project overview
└── Makefile                  # Build commands
```

## Getting Started

### For VS Code Users

1. Open this folder in VS Code
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Press `F1` → `Dev Containers: Reopen in Container`

### For CLI Users

```bash
# Build the image
make build

# Run in interactive mode
make devcontainer

# Get a shell inside the container
make shell
```

## Documentation Sections

- **[Architecture](architecture.md)** - Learn about the system design, component interactions, and technical decisions
- **[Setup Guide](setup.md)** - Step-by-step installation and configuration instructions
- **[Development](development.md)** - Workflow guide for contributors and developers
- **[Proxy Configuration](proxy.md)** - Understanding and configuring proxy settings
- **[LLM Providers](providers.md)** - Setting up and using different LLM providers

## Conventions

This project follows specific conventions documented in [CLAUDE.md](../CLAUDE.md):

- **Atomic Commits** - One logical change per commit
- **Conventional Commits** - Structured commit message format
- **Pre-commit Verification** - Build testing before commits
- **English Documentation** - All documentation in English

## Support

For issues or questions:

1. Check existing documentation in this directory
2. Review [CLAUDE.md](../CLAUDE.md) for project-specific guidance
3. Consult the [troubleshooting section](setup.md#troubleshooting) in setup guide

## License

MIT License - See project root for details.
