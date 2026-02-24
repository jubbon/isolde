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
claude: {...}          # Claude Code CLI configuration
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

### claude

- **Type:** `object`
- **Required:** Yes
- **Description:** Claude Code CLI configuration

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `version` | `string` | No | `"latest"` | Claude Code CLI version |
| `provider` | `string` | No | `"anthropic"` | Claude API provider |
| `models` | `object` | No | `{}` | Model name mappings |

**Valid providers:** `anthropic`, `openai`, `bedrock`, `vertex`, `azure`

```yaml
claude:
  version: latest
  provider: anthropic
  models:
    haiku: claude-3-5-haiku-20241022
    sonnet: claude-3-5-sonnet-20241022
    opus: claude-3-5-sonnet-20241022
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

claude:
  version: latest
  provider: anthropic
  models:
    haiku: claude-3-5-haiku-20241022
    sonnet: claude-3-5-sonnet-20241022
    opus: claude-3-5-sonnet-20241022

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
5. **claude.provider must be valid** - Must be one of: anthropic, openai, bedrock, vertex, azure
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
| 0.1 | 2025-02-24 | Initial schema version |
