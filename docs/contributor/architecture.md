# Architecture

This document describes the architecture of the Isolde (ISOLated Development Environment) project.

## System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         Host Machine                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │    VS Code   │  │  Docker CLI  │  │     Git     │    │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘    │
└─────────┼──────────────────┼──────────────────┼────────────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Rust CLI (isolde)                          │
│  ┌────────────────────────────────────────────────────────┐  │
│  │              isolde-cli (clap parser)                 │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │  │
│  │  │   init   │  │   sync   │  │  list-templates  │ │  │
│  │  └──────────┘  └──────────┘  └──────────────────┘ │  │
│  └────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────┐  │
│  │              isolde-core (business logic)             │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌─────────────┐ │ │
│  │  │  templates   │  │     git      │  │   config    │ │ │
│  │  │    module    │  │   module     │  │   module    │ │ │
│  │  └──────────────┘  └──────────────┘  └─────────────┘ │ │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Created Project                             │
│  ┌─────────────────────┐    ┌────────────────────────────────┐ │
│  │   project/          │    │   .devcontainer/               │ │
│  │   (user code)       │    │   (container config)           │ │
│  │                     │    │                                │ │
│  │   └─ .git/          │    │   ├─ devcontainer.json         │ │
│  │                     │    │   ├─ Dockerfile                │ │
│  └─────────────────────┘    │   ├─ features/                 │ │
│                             │   │   ├─ claude-code/          │ │
│                             │   │   ├─ proxy/                │ │
│                             │   │   └─ plugin-manager/       │ │
│                             │   └─ .git/                     │ │
│                             └────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Dev Container                               │
│  ┌────────────────────────────────────────────────────────┐  │
│  │          Claude Code CLI (user workspace)            │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────────────┐ │  │
│  │  │  Config  │  │  Mounts  │  │  Provider Config  │ │  │
│  │  └──────────┘  └──────────┘  └──────────────────┘ │  │
│  └────────────────────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────────────────────┐  │
│  │              Docker-in-Docker                         │  │
│  │          (/var/run/docker.sock)                       │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    LLM Provider API                          │
│            (Anthropic / Z.ai / Custom)                        │
└─────────────────────────────────────────────────────────────────┘
```

## Components

### 1. Rust CLI

**Location:** `isolde-cli/` and `isolde-core/`

#### isolde-cli

| File | Purpose |
|------|---------|
| `src/main.rs` | Entry point with clap command-line argument parsing |
| `Cargo.toml` | CLI dependencies and metadata |

**Key Responsibilities:**
- Parse command-line arguments
- Display help and usage information
- Delegate to isolde-core for business logic
- Handle error reporting

#### isolde-core

| Module | Purpose |
|--------|---------|
| `templates.rs` | Template loading, validation, substitution, copying |
| `git.rs` | Dual git repo initialization (project + devcontainer) |
| `config.rs` | Configuration and preset loading from YAML |
| `features.rs` | Feature copying and management |

**Key Responsibilities:**
- Template processing engine
- Git operations for dual repository pattern
- Configuration management
- Feature file handling

### 2. Template System

**Location:** `templates/`

Each template contains:
```
templates/python/
├── template-info.yaml         # Template metadata
└── .devcontainer/
    ├── devcontainer.json      # Container configuration with placeholders
    └── Dockerfile             # Base image definition
```

### 3. Core Features

**Location:** `core/features/`

| Feature | Purpose |
|---------|---------|
| `claude-code/` | Claude Code CLI installation with multi-provider support |
| `proxy/` | HTTP proxy configuration for enterprise networks |
| `plugin-manager/` | Plugin activation and management |

Features are **copied** (not symlinked) to each project because Docker's build context cannot follow symlinks outside the build directory.

### 4. Dev Container Configuration

**Location:** `.devcontainer/`

| File | Purpose |
|------|---------|
| `devcontainer.json` | Main configuration - mounts, environment, features |
| `Dockerfile` | Base image definition |
| `PROXY_ARCHITECTURE.md` | Proxy architecture documentation |
| `docs/` | Devcontainer-specific documentation |

## Data Flow

### Template Application Flow

```
isolde init my-app --template python
    │
    ▼
1. CLI parses arguments (clap)
    │
    ▼
2. isolde-core::config loads template-info.yaml
    │
    ▼
3. isolde-core::templates validates template
    │
    ▼
4. isolde-core::features copies core/features/* to .devcontainer/features/
    │
    ▼
5. isolde-core::templates applies substitutions to devcontainer.json
    │   - {{PROJECT_NAME}}
    │   - {{LANG_VERSION}}
    │   - {{FEATURES_CLAUDE_CODE}}
    │   - etc.
    │
    ▼
6. isolde-core::git initializes dual git repositories
    │   - project/.git/
    │   - .devcontainer/.git/
    │
    ▼
7. Project ready at ~/workspace/my-app/
```

### Container Startup Flow

```
1. VS Code/CLI initiates container
   │
2. Docker builds/starts container
   │
3. postCreateCommand executes
   │   └──> Writes ~/.config/devcontainer/provider
   │   └──> Updates ~/.bashrc with provider function
   │
4. User opens shell
   │
5. ~/.bashrc sources
   │   └──> configure_claude_provider() runs
   │       └──> Sets ANTHROPIC_AUTH_TOKEN
   │       └──> Sets ANTHROPIC_BASE_URL (if custom provider)
   │
6. Claude Code CLI ready with configured provider
```

### Provider Configuration Flow

```
install.sh (build time)
    │
    ▼
Creates ~/.config/devcontainer/provider
    │
    ▼
postCreateCommand updates ~/.bashrc
    │
    ▼
~/.bashrc sources configure_claude_provider()
    │
    ▼
Each shell: loads from ~/.claude/providers/{provider}/
    ├── auth → ANTHROPIC_AUTH_TOKEN
    └── base_url → ANTHROPIC_BASE_URL
```

## Design Decisions

### Why Rust?

The v2 implementation uses Rust instead of shell scripts for:
- **Type Safety**: Compile-time error checking prevents common bugs
- **Performance**: Faster execution, especially for template processing
- **Maintainability**: Easier to refactor and extend
- **Distribution**: Single binary distribution via cargo
- **Testing**: Built-in test framework with good tooling

### Why Dual Git Repositories?

Created projects have **two separate git repositories**:
- `project/` - User code repository
- `.devcontainer/` - Devcontainer configuration repository

This separation allows:
- Independent version control of code vs. devcontainer config
- Easy updates to devcontainer from template repository
- Clean git history for user code
- Separate release cycles for code and infrastructure

### Why Copy Features Instead of Symlinks?

Docker's build context cannot follow symlinks outside the build directory. Features must be copied into each project's `.devcontainer/features/` directory.

### Why ~/.config/devcontainer/provider?

The provider name is stored in `~/.config/devcontainer/` (container-local) rather than `~/.claude` to avoid race conditions when `~/.claude` is mounted between multiple containers.

## Code Organization

### Module Boundaries

```
isolde-cli (presentation layer)
    │
    │ depends on
    ▼
isolde-core (business logic layer)
    ├── templates (template processing)
    ├── git (repository operations)
    ├── config (configuration management)
    └── features (feature handling)
    │
    │ depends on
    ▼
templates/, core/features/ (data layer)
```

### Error Handling

All modules use `Result<T, E>` for error handling:
```rust
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TemplateNotFound(String),
    InvalidTemplate(String),
    GitError(git2::Error),
    IoError(std::io::Error),
}
```

## Extension Points

### Adding New Templates

Create a new directory under `templates/` with:
- `template-info.yaml` - Template metadata
- `.devcontainer/devcontainer.json` - Container configuration
- `.devcontainer/Dockerfile` - Base image

### Adding New Features

Create features under `core/features/` following the [Dev Containers Feature specification](https://devcontainers.github.io/implementors/features/).

### Adding New CLI Commands

Add new subcommands to `isolde-cli/src/main.rs`:
```rust
#[derive(Subcommand)]
enum Commands {
    #[command(name = "newcommand")]
    NewCommand {
        #[arg(short, long)]
        option: String,
    },
}
```

## Security Considerations

### Credential Storage

- API tokens stored in user home directory (`~/.claude/`)
- File permissions respect user umask
- No credentials in container image or logs

### Docker Socket Access

- Socket mounted with same permissions as host user
- No privilege escalation within container
- Host user's Docker group membership required

### Proxy Configuration

- Proxy URLs in `devcontainer.json` (clear text in project)
- Consider environment-specific overrides for sensitive proxy credentials
