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
4. Coding agent configuration
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

### Agent Configuration

```bash
# Specific agent (default: claude-code)
isolde init my-app --template python --agent claude-code

# Specific agent version
isolde init my-app --template python --agent-version 1.2.41

# Use a different agent
isolde init my-app --template python --agent codex
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
isolde init --list-templates

# List available presets
isolde init --list-presets
```

## Environment Variables

You can set defaults via environment variables:

```bash
export ISOLDE_WORKSPACE=~/projects
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

The `isolde sync` command regenerates devcontainer configuration from `isolde.yaml`:

```bash
cd ~/workspace/my-project
isolde sync
```

This will:
- Generate `.devcontainer/devcontainer.json` and `Dockerfile`
- Generate `.claude/CLAUDE.md` with project-specific guidance
- Copy `core/features/` (claude-code, proxy, plugin-manager) to `.devcontainer/features/`
- Create the `project/` workspace directory if it does not exist

Run `isolde sync` after editing `isolde.yaml` to apply changes, then rebuild the container.

## Container Management Commands

Isolde provides built-in commands for managing devcontainers using the [devcontainers CLI](https://github.com/devcontainers/cli). These commands require the devcontainer CLI to be installed.

### Prerequisites

Install the devcontainer CLI:

```bash
# On Linux/macOS
npm install -g @devcontainers/cli

# Verify installation
devcontainer --version
```

### Building the Container

Build the devcontainer image from your project configuration:

```bash
cd ~/workspace/my-project
isolde build
```

Build options:
```bash
# Build without cache
isolde build --no-cache

# Build with custom image name
isolde build --image-name myproject:latest

# Build in specific workspace
isolde build --workspace-folder /path/to/project
```

### Running the Container

Start the devcontainer and enter an interactive shell:

```bash
isolde run
```

Run options:
```bash
# Start without attaching (background mode)
isolde run --detach

# Run in specific workspace
isolde run --workspace-folder /path/to/project
```

### Executing Commands

Run a command inside a running container without starting a shell:

```bash
# Run a single command
isolde exec python --version

# Run tests
isolde exec pytest

# Run multiple commands
isolde exec bash -c "echo hello && ls"
```

### Stopping the Container

Stop a running container:

```bash
isolde stop
```

Stop options:
```bash
# Force stop without graceful shutdown
isolde stop --force

# Stop container in specific workspace
isolde stop --workspace-folder /path/to/project
```

### Listing Containers

List all running devcontainers:

```bash
isolde ps
```

List options:
```bash
# Show all containers (including stopped)
isolde ps --all
```

Output example:
```
📋 Devcontainers
────────────────────────────────────────────────────────────────────
ID            NAME                        STATUS      WORKSPACE
────────────────────────────────────────────────────────────────────
abc123...     devcontainer-myproject      running     /home/user/myproject

1 container
```

### Viewing Logs

View container logs:

```bash
isolde logs
```

Logs options:
```bash
# Follow logs (like tail -f)
isolde logs --follow

# Show last 50 lines
isolde logs --tail 50

# View logs for specific workspace
isolde logs --workspace-folder /path/to/project
```

### Typical Workflow

A typical development workflow with Isolde:

```bash
# 1. Create a project
isolde init my-app --template python
cd my-app

# 2. Generate devcontainer configuration
isolde sync

# 3. Build the container image
isolde build

# 4. Start the container
isolde run

# 5. Run commands in the container (from another terminal)
isolde exec pytest
isolde exec python --version

# 6. View logs if needed
isolde logs

# 7. Stop the container when done
isolde stop

# 8. List containers to verify
isolde ps
```

### State Management

Isolde maintains container state in `.isolde/state.json` in each project directory:

```json
{
  "container_id": "abc123def456",
  "container_name": "devcontainer-myproject",
  "image_name": "myproject-dev:latest",
  "status": "running",
  "workspace_folder": "/home/user/myproject",
  "created_at": "2026-03-02T10:30:00Z",
  "updated_at": "2026-03-02T10:30:00Z"
}
```

## Isolation Levels

Isolde supports configurable isolation levels that control how much host Claude Code state is shared with the devcontainer. Set the `isolation` field in `isolde.yaml`:

```yaml
isolation: session  # default
```

### Available Levels

| Level | Description | Use case |
|---|---|---|
| `none` | Mount entire host `~/.claude` | Legacy behavior, full state sharing |
| `session` | Isolate sessions and telemetry | **Default.** Clean session state, keep host plugins and settings |
| `workspace` | Isolate sessions, telemetry, and plugins | Test plugin configurations without affecting host |
| `full` | Only share auth credentials | Reproducible "zero environment" testing |

### What Gets Isolated

| Component | `none` | `session` | `workspace` | `full` |
|---|---|---|---|---|
| Auth credentials | host | host | host | host |
| App state (`~/.claude.json`) | host | host | host | host |
| Settings, keybindings | host | host | host | container |
| Plugins | host | host | **container** | container |
| Sessions | host | **container** | **container** | container |
| Telemetry | host | **container** | **container** | container |

Container-local state is persisted in `.isolde/volumes/` so it survives container rebuilds.

### Full Isolation Setup

For `full` isolation, auth files are mounted from the host only if they exist. If you haven't logged in yet:

```bash
# 1. Log in on the host first
claude login

# 2. Then sync to generate mounts with auth
isolde sync
```

### Migration Note

Existing projects without the `isolation` field default to `session` (not `none`). To keep the old behavior, add `isolation: none` to your `isolde.yaml`.

## isolde.yaml Format

Every Isolde project has an `isolde.yaml` configuration file:

```yaml
# Isolde Configuration for my-app
# Generated by isolde init
version: "0.1"

name: my-app
workspace:
  dir: ./project

docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
  build_args: []

# Coding agent configuration
agent:
  name: claude-code
  version: latest
  options:
    provider: anthropic
    models: "haiku:claude-3-5-haiku-20241022,sonnet:claude-3-5-sonnet-20241022,opus:claude-3-5-sonnet-20241022"

# Isolation level (none/session/workspace/full)
isolation: session

# Runtime configuration (optional)
# runtime:
#   language: python
#   version: "3.12"
#   package_manager: uv

# Proxy configuration (optional)
# proxy:
#   http: http://proxy.corp.com:8080
#   https: http://proxy.corp.com:8080

git:
  generated: ignored
```

**Schema Versioning:**
- The `version` field specifies the schema format (not project version)
- Current supported version: `"0.1"`
- The version field is **required**
- Missing or unknown versions will cause an error

## Troubleshooting

### Container Won't Start

1. Check Docker is running: `docker ps`
2. Check the devcontainer log (output panel in VS Code)
3. Try rebuilding the container

### isolde.yaml Errors

If you see an error about the schema version:
- Ensure `version: "0.1"` is present (with quotes)
- The version must be the first field after comments
- Check that the YAML syntax is correct

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

> **Note:** Custom presets always use the `claude-code` agent.

Then use it:

```bash
isolde init my-app --preset my-custom
```
