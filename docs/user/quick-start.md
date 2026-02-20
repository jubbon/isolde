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

# Build and install
cargo install --path .

# Verify installation
isolde --version
```

### Using Cargo (future)

```bash
cargo install isolde
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
isolde list-templates

# List all presets
isolde list-presets
```

## What You Get

Each created project has:
- A `project/` directory for your code and devcontainer configuration
- A single git repository that includes both code and config
- Pre-installed language tools and Claude Code CLI

## Next Steps

- [Usage Guide](usage.md) - Detailed command reference
- [Templates](templates.md) - Available language templates
- [Presets](presets.md) - Pre-configured combinations
- [Troubleshooting](troubleshooting.md) - Common issues
