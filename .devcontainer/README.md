# Isolde

Docker-based development environment for [Claude Code CLI](https://code.claude.com) with Dev Containers support. Features multi-provider LLM support and enterprise proxy configuration.

## Quick Start

### VS Code (Recommended)

1. Open this folder in VS Code
2. Install [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Press `F1` → `Dev Containers: Reopen in Container`

That's it! Claude Code is ready to use.

### CLI Usage

```bash
# Build the image
make build

# Run in interactive mode
make devcontainer

# Get a shell inside the container
make shell
```

## Features

- ✅ Claude Code CLI pre-installed
- ✅ Docker-in-Docker support
- ✅ Multi-provider LLM support (Anthropic, Z.ai, custom)
- ✅ Enterprise proxy configuration
- ✅ Version control options
- ✅ Common utilities (git, vim, jq, curl, wget)

## Documentation

For detailed documentation, see the [docs/](docs/) directory:

| Topic | Description |
|-------|-------------|
| [Setup Guide](docs/setup.md) | Installation and configuration |
| [Architecture](docs/architecture.md) | System design and components |
| [Development](docs/development.md) | Contributing workflow |
| [Proxy Configuration](docs/proxy.md) | Enterprise proxy setup |
| [LLM Providers](docs/providers.md) | Provider configuration |
| [Version Control](docs/claude-version-control.md) | Managing Claude Code versions |

## Project Structure

```
.devcontainer/
├── devcontainer.json       # Main config (proxy, mounts, features)
├── Dockerfile              # Base image definition
├── PROXY_ARCHITECTURE.md  # Proxy architecture docs
├── docs/                  # Project documentation
├── features/
│   └── claude-code/         # Custom Claude Code feature
│       ├── devcontainer-feature.json
│       ├── install.sh
│       └── README.md
├── CLAUDE.md              # Project instructions for Claude Code
├── README.md               # This file
└── Makefile                # Build commands
```

## Configuration

### LLM Provider

Configure your LLM provider in `devcontainer.json`:

```json
{
  "features": {
    "./features/claude-code": {
      "provider": "anthropic"  // or "z.ai", or custom
      "version": "latest"
    }
  }
}
```

See [docs/providers.md](docs/providers.md) for detailed setup.

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

See [docs/proxy.md](docs/proxy.md) for details.

### Version Control

Choose which Claude Code version to install:

```json
{
  "features": {
    "./features/claude-code": {
      "version": "latest"    // or "stable", or "1.2.41"
    }
  }
}
```

See [docs/claude-version-control.md](docs/claude-version-control.md) for details.

## Make Commands

| Command | Description |
|---------|-------------|
| `make help` | Show available commands |
| `make build` | Build the Docker image |
| `make devcontainer` | Run container with current workspace |
| `make shell` | Get shell in running container |
| `make clean` | Remove running containers |

## Docker-in-Docker

The container mounts `/var/run/docker.sock` for Docker-in-Docker support:

- Build containers within the dev container
- Run docker commands without sudo
- Test Docker-based projects in isolation

Ensure your user has Docker permissions on the host machine.

## Development

This project follows specific conventions documented in [CLAUDE.md](.devcontainer/CLAUDE.md):

- **Atomic commits** - One logical change per commit
- **Conventional commits** - Structured commit message format
- **Pre-commit verification** - Build testing before commits
- **English documentation** - All docs in English

See [docs/development.md](docs/development.md) for contributing guidelines.

## License

MIT
