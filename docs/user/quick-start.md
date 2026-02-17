# Quick Start

Get started with Isolde in 5 minutes.

## What is Isolde?

Isolde (ISOLated Development Environment) is a template-based system for creating isolated development environments with Claude Code CLI support. Each project gets its own devcontainer with pre-configured tools, language runtimes, and Claude Code integration.

## Prerequisites

- Docker (for running devcontainers)
- Claude Code CLI (optional but recommended)

Verify Docker is installed:
```bash
docker --version
```

## Creating Your First Project

### Option 1: Interactive Wizard

```bash
cd /path/to/this/repo
./scripts/isolde.sh
```

Follow the prompts to select a template or preset.

### Option 2: Direct Command

```bash
./scripts/isolde.sh my-app --preset=python-ml
```

This creates a new project with Python, machine learning tools, and Claude Code pre-configured.

## What You Get

Each created project has:
- A `project/` directory for your code
- A `.devcontainer/` directory with container configuration
- Two separate git repositories (code and devcontainer)
- Pre-installed language tools and Claude Code CLI

## Next Steps

- [Usage Guide](usage.md) - Detailed command reference
- [Templates](templates.md) - Available language templates
- [Presets](presets.md) - Pre-configured combinations
- [Troubleshooting](troubleshooting.md) - Common issues
