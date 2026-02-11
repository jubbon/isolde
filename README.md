# Claude Code Dev Container

Docker-based development environment for [Claude Code CLI](https://code.claude.com) with Dev Containers support.

## Quick Start

### VS Code / Cursor (Recommended)

1. Open this folder in VS Code
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Press `F1` → `Dev Containers: Reopen in Container`

That's it! Claude Code is ready to use.

### CLI Usage

```bash
# Build the image
make build

# Run in interactive mode
make devcontainer

# Get a shell
make shell
```

## Features

- ✅ Claude Code CLI pre-installed
- ✅ Docker-in-Docker support
- ✅ Python 3.11
- ✅ Common utilities (git, vim, jq, curl, wget)
- ✅ VS Code extensions pre-configured

## Project Structure

```
.
├── .devcontainer/           # Dev Container configuration
│   ├── devcontainer.json    # Main config file
│   ├── Dockerfile           # Container image definition
│   └── features/            # Custom Dev Container features
│       └── claude-code/
├── image/                   # Standalone base image
│   └── Dockerfile
├── config/                  # Configuration files
│   └── models.env           # Model configuration
├── Makefile                 # Build commands
└── README.md
```

## Configuration

### Model Configuration

Edit `config/models.env` or create `~/docker-claude-code.env`:

```bash
ANTHROPIC_BASE_URL=https://api.anthropic.com
ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-3-5-haiku-20241022
ANTHROPIC_DEFAULT_SONNET_MODEL=claude-3-5-sonnet-20241022
ANTHROPIC_DEFAULT_OPUS_MODEL=claude-3-5-sonnet-20241022
```

### Custom Features

Add custom features in `.devcontainer/features/`. See [Dev Container Features spec](https://devcontainers.github.io/implementors/features/) for details.

## Make Commands

| Command | Description |
|---------|-------------|
| `make help` | Show available commands |
| `make build` | Build the Docker image |
| `make devcontainer` | Run container with current workspace |
| `make shell` | Get shell in container |
| `make push` | Push image to registry |
| `make clean` | Remove running containers |

## Docker Socket Access

The container mounts `/var/run/docker.sock` for Docker-in-Docker support. Ensure your user has Docker permissions on the host.

## License

MIT
