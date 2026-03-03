# Quick Start

Get started with Isolde in 5 minutes.

## What is Isolde?

Isolde (ISOLated Development Environment) is a Rust CLI tool for creating isolated development environments with Claude Code CLI support. Each project gets its own devcontainer with pre-configured tools, language runtimes, and Claude Code integration.

## Prerequisites

- Docker (for running devcontainers)
- Rust toolchain (for building the CLI)
- Claude Code CLI (optional but recommended)

Verify Docker is installed:
```bash
docker --version
```

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd isolde

# Build and install to ~/.local/bin/
make install

# Add ~/.local/bin to PATH if needed
export PATH="$HOME/.local/bin:$PATH"

# Verify installation
isolde --version
```

### Alternative Installation Methods

```bash
# Install via cargo (to ~/.cargo/bin/)
make install-cargo

# Or use cargo directly
cargo install --path .
```

## Creating Your First Project

### Option 1: Interactive Wizard

```bash
isolde init
```

Follow the prompts to select a template or preset.

### Option 2: Direct Command

```bash
isolde init my-app --preset python-ml
```

This creates a new project with Python, machine learning tools, and Claude Code pre-configured.

### Option 3: Template Selection

```bash
isolde init my-api --template nodejs
```

### List Available Options

```bash
# List all templates
isolde init --list-templates

# List all presets
isolde init --list-presets
```

## What You Get

Each created project has:
- A `project/` directory for your code and devcontainer configuration
- A single git repository that includes both code and config
- Pre-installed language tools and Claude Code CLI

## Building and Running

After creating a project, you'll need to build and run the devcontainer:

### Prerequisites

Install the [devcontainers CLI](https://github.com/devcontainers/cli):

```bash
npm install -g @devcontainers/cli
```

### Build the Container

```bash
cd my-app
isolde build
```

### Start the Container

```bash
isolde run
```

This will start the container and give you an interactive shell inside it.

### Run Commands

From another terminal, you can execute commands in the running container:

```bash
cd my-app
isolde exec python --version
isolde exec pytest
```

### Other Useful Commands

```bash
# Stop the container
isolde stop

# List running containers
isolde ps

# View container logs
isolde logs

# Follow logs (live)
isolde logs --follow
```

## Next Steps

- [Usage Guide](usage.md) - Detailed command reference
- [Templates](templates.md) - Available language templates
- [Presets](presets.md) - Pre-configured combinations
- [Troubleshooting](troubleshooting.md) - Common issues
