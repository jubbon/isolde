# Claude Code Devcontainer Templates

A template-based system for creating devcontainer projects with Claude Code CLI support.

## Overview

This repository provides templates and tools for quickly setting up isolated development environments for multiple projects. Each project gets:

- A dedicated devcontainer with custom configurations
- Language-specific tooling and dependencies
- Isolated git repositories for project code and devcontainer config
- Pre-configured Claude Code CLI with multi-provider support

## Quick Start

```bash
# Clone the templates repository
git clone https://github.com/your-org/claude-code-templates.git
cd claude-code-templates

# Run the interactive wizard
./scripts/init-project.sh

# Or create a project with a preset
./scripts/init-project.sh my-ml-app --preset=python-ml

# Or create with specific template
./scripts/init-project.sh my-api --template=nodejs --lang-version=22
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
│   ├── features/         # Symlinks to template repository
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
│   ├── init-project.sh    # Main script
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
