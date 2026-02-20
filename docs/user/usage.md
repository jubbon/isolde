# Usage Guide

## Interactive Mode

The simplest way to create a project is using the interactive wizard:

```bash
isolde init
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
# Create a project with default template (generic)
isolde init my-project

# Create with a specific template
isolde init my-app --template python

# Create with a preset
isolde init ml-app --preset python-ml
```

### Language Version

```bash
# Python 3.12
isolde init my-app --template python --lang-version 3.12

# Node.js 20
isolde init my-api --template nodejs --lang-version 20
```

### Workspace Location

By default, projects are created in `~/workspace`. Use `--workspace` to specify a different location:

```bash
isolde init my-app --template python --workspace ~/projects
```

### Claude Code Configuration

```bash
# Specific version
isolde init my-app --template python --claude-version 1.2.41

# Custom provider
isolde init my-app --template python --claude-provider z.ai

# Custom models
isolde init my-app --template python --claude-models "haiku:glm-4.5-air,sonnet:glm-4.7"
```

### Proxy Configuration

```bash
# With proxy
isolde init my-app --template python --proxy http://proxy.example.com:8080

# Separate HTTP/HTTPS proxies
isolde init my-app --template python --http-proxy http://proxy:8080 --https-proxy https://proxy:8443

# Disable proxy
isolde init my-app --template python --no-proxy
```

### Auto-Confirm

Skip confirmation prompts:

```bash
isolde init my-app --template python --yes
```

## Listing Options

```bash
# List available templates
isolde list-templates

# List available presets
isolde list-presets
```

## Environment Variables

You can set defaults via environment variables:

```bash
export ISOLDE_WORKSPACE=~/projects
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

The devcontainer configuration is part of the project's git repository:

```bash
cd ~/workspace/my-project
git pull origin main
```

Then rebuild the container.

## Sync Command

The `isolde sync` command updates the devcontainer with the latest changes from the template:

```bash
cd ~/workspace/my-project
isolde sync
```

This will:
- Pull latest template changes
- Update features while preserving your customizations
- Rebuild the container if needed

## Troubleshooting

### Container Won't Start

1. Check Docker is running: `docker ps`
2. Check the devcontainer log (output panel in VS Code)
3. Try rebuilding the container

### Git Repository Issues

The project has a single git repository that includes both your code and devcontainer configuration:

```bash
cd ~/workspace/my-project
git status
```

### Proxy Issues

Verify proxy variables in the container:

```bash
echo $HTTP_PROXY
echo $HTTPS_PROXY
echo $NO_PROXY
```

## Advanced: Custom Presets

Create custom presets in `~/.isolde-presets.yaml`:

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
isolde init my-app --preset my-custom
```
