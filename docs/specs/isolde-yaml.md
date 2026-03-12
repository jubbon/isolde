# isolde.yaml Schema Specification

This document describes the schema specification for `isolde.yaml` configuration files.

## Version

**Current schema version:** `0.1`

The `version` field identifies the schema format used by the configuration file. This allows the Isolde CLI to support multiple schema versions simultaneously, enabling backward compatibility and future evolution of the configuration format.

## File Structure

```yaml
# Comments are allowed anywhere
version: "0.1"         # REQUIRED: Schema version (must be "0.1")
name: project-name     # REQUIRED: Project identifier
workspace: {...}       # Workspace configuration
docker: {...}          # Docker configuration
agent: {...}           # Coding agent configuration
isolation: session     # Isolation level (none/session/workspace/full)
runtime: {...}         # OPTIONAL: Runtime environment
proxy: {...}           # OPTIONAL: Proxy configuration
marketplaces: {...}   # OPTIONAL: Marketplace definitions
plugins: [...]         # OPTIONAL: Plugin configurations
git: {...}            # Git configuration
```

## Fields

### version

- **Type:** `string`
- **Required:** Yes
- **Format:** `"X.Y"` where X and Y are non-negative integers
- **Valid values:** `"0.1"`
- **Description:** Schema version identifier

```yaml
version: "0.1"
```

### name

- **Type:** `string`
- **Required:** Yes
- **Format:** Must be a valid project name (alphanumeric, hyphens, underscores)
- **Description:** Human-readable project identifier

```yaml
name: my-awesome-project
```

### workspace

- **Type:** `object`
- **Required:** Yes
- **Description:** Workspace directory configuration

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `dir` | `string` | No | `"./project"` | Relative path to workspace directory |

```yaml
workspace:
  dir: ./project
```

### docker

- **Type:** `object`
- **Required:** Yes
- **Description:** Docker container configuration

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `image` | `string` | Yes | - | Base Docker image |
| `build_args` | `array[string]` | No | `[]` | Docker build arguments |

```yaml
docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
  build_args:
    - USERNAME=user
```

### agent

- **Type:** `object`
- **Required:** Yes
- **Description:** Coding agent configuration. Exactly one agent per project.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | `string` | No | `"claude-code"` | Agent identifier (`claude-code`, `codex`, `gemini`, `aider`) |
| `version` | `string` | No | `"latest"` | Agent CLI version to install |
| `options` | `object` | No | `{}` | Agent-specific key-value options |

**Agent-specific options for `claude-code`:**

| Option | Description |
|--------|-------------|
| `provider` | Claude API provider (e.g. `anthropic`, `z.ai`) |
| `models` | Comma-separated `role:model` pairs, e.g. `"haiku:claude-3-5-haiku,sonnet:claude-3-5-sonnet"` |

```yaml
# Coding agent configuration
agent:
  name: claude-code
  version: latest
  options:
    provider: anthropic
    models: "haiku:claude-3-5-haiku-20241022,sonnet:claude-3-5-sonnet-20241022,opus:claude-3-5-sonnet-20241022"
```

### runtime

- **Type:** `object`
- **Required:** No
- **Description:** Runtime environment configuration

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `language` | `string` | Yes | - | Programming language |
| `version` | `string` | Yes | - | Language version |
| `package_manager` | `string` | Yes | - | Package manager name |
| `tools` | `array[string]` | No | `[]` | Additional tools to install |

```yaml
runtime:
  language: python
  version: "3.12"
  package_manager: uv
  tools:
    - pytest
    - ruff
```

### proxy

- **Type:** `object`
- **Required:** No
- **Description:** HTTP proxy configuration for corporate networks

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `http` | `string` | No | - | HTTP proxy URL |
| `https` | `string` | No | - | HTTPS proxy URL |
| `no_proxy` | `string` | No | - | No-proxy hosts list |

```yaml
proxy:
  http: http://proxy.corp.com:8080
  https: http://proxy.corp.com:8080
  no_proxy: localhost,127.0.0.1,.local
```

### marketplaces

- **Type:** `object`
- **Required:** No
- **Description:** Plugin marketplace configurations

```yaml
marketplaces:
  omc:
    url: https://github.com/oh-my-claudecode/marketplace
```

### plugins

- **Type:** `array[object]`
- **Required:** No
- **Description:** Plugin configurations

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `marketplace` | `string` | Yes | - | Marketplace name |
| `name` | `string` | Yes | - | Plugin name |
| `activate` | `boolean` | No | `true` | Whether to activate the plugin |

```yaml
plugins:
  - marketplace: omc
    name: oh-my-claudecode
    activate: true
```

### isolation

- **Type:** `string`
- **Required:** No
- **Default:** `"session"`
- **Valid values:** `none`, `session`, `workspace`, `full`
- **Description:** Controls how much host Claude Code state is shared with the devcontainer

| Component | `none` | `session` | `workspace` | `full` |
|---|---|---|---|---|
| Auth (`.credentials.json`, `providers/`, `provider`) | host | host | host | host |
| App state (`~/.claude.json`) | host | host | host | host |
| Settings, keybindings | host | host | host | container |
| Plugins (`plugins/`) | host | host | **container** | container |
| Sessions (`projects/`) | host | **container** | **container** | container |
| Telemetry (`statsig/`) | host | **container** | **container** | container |
| OMC config (`.omc-config.json`) | host | **container** | **container** | container |

Container-local state is persisted in `.isolde/volumes/` (bind-mounted).

**`none`** — current legacy behavior, mounts entire host `~/.claude` directory.

**`session`** (default) — host `~/.claude` is mounted, but sessions, telemetry, and OMC config are overlaid with local volumes. Good for clean session state while keeping plugins and settings from host.

**`workspace`** — same as `session`, plus plugins are container-local. Good for testing plugin configurations without affecting host.

**`full`** — does not mount host `~/.claude` at all. Uses a local `claude-home` volume. Auth files (`.credentials.json`, `providers/`, `provider`) and `~/.claude.json` are conditionally mounted from host if they exist. Good for reproducible "zero environment" testing.

```yaml
# Default (session isolation)
isolation: session

# No isolation (legacy behavior)
isolation: none

# Full isolation (only auth shared)
isolation: full
```

**Note:** For `full` isolation, if `~/.claude/.credentials.json` does not exist on the host, `isolde sync` prints a warning. Run `claude login` on the host first, then `isolde sync` again.

### git

- **Type:** `object`
- **Required:** No
- **Description:** Git configuration

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `generated` | `string` | No | `"ignored"` | How to handle generated files |

**Valid values:** `ignored`, `committed`, `linguist-generated`

```yaml
git:
  generated: ignored
```

## Complete Example

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

isolation: session

runtime:
  language: python
  version: "3.12"
  package_manager: uv
  tools:
    - pytest
    - ruff

proxy:
  http: http://proxy.corp.com:8080
  https: http://proxy.corp.com:8080
  no_proxy: localhost,127.0.0.1,.local

marketplaces:
  omc:
    url: https://github.com/oh-my-claudecode/marketplace

plugins:
  - marketplace: omc
    name: oh-my-claudecode
    activate: true

git:
  generated: ignored
```

## Validation Rules

1. **version field is required** - Missing version will cause an error
2. **version must be supported** - Unknown versions (e.g., "99.9") will cause an error
3. **name field is required** - Must be non-empty
4. **docker.image is required** - Must be non-empty
5. **agent.name must be non-empty** - Defaults to `claude-code` if omitted
6. **plugins must reference existing marketplaces** - Each plugin's marketplace must be defined in `marketplaces`

## Schema Evolution

When adding a new schema version:

1. Choose a new version number (e.g., "0.2", "1.0")
2. Create a new module in `isolde-core/src/config/v{version}_/`
3. Add parsing logic in `isolde-core/src/config.rs`
4. Add the variant to `SchemaVersion` enum
5. Document breaking changes
6. Update this specification

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 0.1 | 2025-02-24 | Initial schema version with `claude:` section |
| 0.1 | 2026-03-04 | Hard break: `claude:` replaced by generic `agent:` section |
