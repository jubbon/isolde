# Usage Guide

## Interactive Mode

The simplest way to create a project is using the interactive wizard:

```bash
# From the repository root
./scripts/isolde.sh
```

The wizard will guide you through:
1. Project name
2. Template or preset selection
3. Language version (if applicable)
4. Claude Code configuration
5. Proxy settings (optional)

## Command-Line Mode

### Basic Usage

```bash
# From the repository root
# Create a project with default template (generic)
./scripts/isolde.sh my-project

# Create with a specific template
./scripts/isolde.sh my-app --template=python

# Create with a preset
./scripts/isolde.sh ml-app --preset=python-ml
```

### Language Version

```bash
# Python 3.12
./scripts/isolde.sh my-app --template=python --lang-version=3.12

# Node.js 20
./scripts/isolde.sh my-api --template=nodejs --lang-version=20
```

### Workspace Location

By default, projects are created in `~/workspace`. Use `--workspace` to specify a different location:

```bash
./scripts/isolde.sh my-app --template=python --workspace=~/projects
```

### Claude Code Configuration

```bash
# Specific version
./scripts/isolde.sh my-app --template=python --claude-version=1.2.41

# Custom provider
./scripts/isolde.sh my-app --template=python --claude-provider=z.ai

# Custom models
./scripts/isolde.sh my-app --template=python --claude-models="haiku:glm-4.5-air,sonnet:glm-4.7"
```

### Proxy Configuration

```bash
# With proxy
./scripts/isolde.sh my-app --template=python --proxy=http://proxy.example.com:8080

# Separate HTTP/HTTPS proxies
./scripts/isolde.sh my-app --template=python --http-proxy=http://proxy:8080 --https-proxy=https://proxy:8443

# Disable proxy
./scripts/isolde.sh my-app --template=python --no-proxy
```

### Auto-Confirm

Skip confirmation prompts:

```bash
./scripts/isolde.sh my-app --template=python --yes
```

## Listing Options

```bash
# List available templates
./scripts/isolde.sh --list-templates

# List available presets
./scripts/isolde.sh --list-presets
```

## Environment Variables

You can set defaults via environment variables:

```bash
export WORKSPACE=~/projects
export CLAUDE_VERSION=latest
export CLAUDE_PROVIDER=z.ai
export HTTP_PROXY=http://proxy:8080
```

## After Project Creation

### Opening in VS Code

```bash
# From anywhere
code ~/workspace/my-project

# From the project directory
cd ~/workspace/my-project
code .
```

VS Code will detect the devcontainer and prompt to reopen in it.

### Rebuilding the Container

1. Press `F1` in VS Code
2. Select "Dev Containers: Rebuild Container"

### Updating the Devcontainer

The devcontainer configuration is a separate git repository:

```bash
cd ~/workspace/my-project/.devcontainer
git pull origin main
```

Then rebuild the container.

## Troubleshooting

### Container Won't Start

1. Check Docker is running: `docker ps`
2. Check the devcontainer log (output panel in VS Code)
3. Try rebuilding the container

### Git Repository Issues

Each project has two git repositories:
- `project/` - Your code
- `.devcontainer/` - Devcontainer config

Check status separately:

```bash
cd ~/workspace/my-project/project && git status
cd ../.devcontainer && git status
```

### Proxy Issues

Verify proxy variables in the container:

```bash
echo $HTTP_PROXY
echo $HTTPS_PROXY
echo $NO_PROXY
```

## Advanced: Custom Presets

Create custom presets in `~/.devcontainer-presets.yaml`:

```yaml
presets:
  my-custom:
    template: python
    description: My custom Python setup
    lang_version: "3.11"
    features:
      - uv
      - pytest
    claude_plugins: oh-my-claudecode,tdd
```

Then use it:

```bash
./scripts/isolde.sh my-app --preset=my-custom
```
