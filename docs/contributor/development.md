# Development Guide

Guide for contributing to and developing the Isolde (ISOLated Development Environment) project.

## Development Workflow

### Prerequisites

- Docker installed and running
- Rust toolchain (rustc, cargo)
- Git configured
- Basic understanding of Dev Containers

### Getting Started

```bash
# Clone repository
git clone <repository-url>
cd isolde

# Create a feature branch
git checkout -b feat/my-new-feature

# Build the CLI
make rust-build
# or
cargo build --release
```

## Project Structure

```
.
├── isolde-core/               # Core library
│   ├── src/
│   │   ├── templates.rs       # Template loading and processing
│   │   ├── git.rs             # Git operations
│   │   ├── config.rs          # Configuration and presets
│   │   └── features.rs        # Feature copying
│   └── Cargo.toml
├── isolde-cli/                # CLI binary
│   ├── src/
│   │   └── main.rs            # Entry point with clap
│   └── Cargo.toml
├── .devcontainer/             # Dev Container for development
│   ├── devcontainer.json      # Main configuration
│   ├── Dockerfile             # Base image
│   └── features/
│       └── claude-code/       # Claude Code feature
├── core/                      # Shared components
│   └── features/              # Reusable devcontainer features
│       ├── claude-code/
│       ├── proxy/
│       └── plugin-manager/
├── templates/                 # Language templates
│   ├── python/
│   ├── nodejs/
│   ├── rust/
│   ├── go/
│   └── generic/
├── mk/                        # Makefile modules
├── docs/                      # Documentation
├── tests/                     # E2E tests
├── presets.yaml               # Preset configurations
├── Cargo.toml                 # Workspace config
├── CLAUDE.md                  # Project instructions
└── README.md                  # Project overview
```

## Making Changes

### 1. Modify CLI Code

**Rust Code Changes:**
```bash
# Edit source files
vim isolde-cli/src/main.rs
vim isolde-core/src/templates.rs

# Build and test
cargo build
cargo test
cargo clippy

# Run directly
cargo run -- --help
```

### 2. Test Your Changes

```bash
# Build release binary
make rust-build

# Test with a sample project
./target/release/isolde init test-project --template python

# Verify project structure
ls -la ~/workspace/test-project/
```

### 3. Commit Standards

This project enforces **atomic commits**. See [CLAUDE.md](../../CLAUDE.md) for full standards.

**Pre-commit Verification:**
```bash
# Format code
make rust-fmt
# or
cargo fmt

# Run linter
make rust-lint
# or
cargo clippy

# Run tests
make rust-test
# or
cargo test

# Build release
make rust-build
```

**Commit Message Format:**
```bash
# Type: lowercase description
git commit -m "feat: add custom provider option"
git commit -m "fix: resolve proxy timeout issue"
git commit -m "docs: update installation instructions"
```

**Types:** `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

### 4. Submit Changes

```bash
# Push to fork
git push origin feat/my-new-feature

# Create pull request via GitHub or gh CLI
gh pr create --title "feat: add custom provider option"
```

## Rust Development

### Building

```bash
# Debug build (faster)
cargo build

# Release build (optimized)
cargo build --release

# Specific crate
cargo build -p isolde-core
cargo build -p isolde-cli
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p isolde-core

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_template_loading
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

### Adding Dependencies

```bash
# Add runtime dependency
cargo add serde -p isolde-core

# Add dev dependency
cargo add --dev assert_cmd -p isolde-cli

# Update lockfile
cargo check
```

## Template Development

### Template Structure

Each template directory contains:
```
templates/python/
├── template-info.yaml         # Template metadata
└── .devcontainer/
    ├── devcontainer.json      # Container configuration
    └── Dockerfile             # Base image
```

### Creating a New Template

1. Create template directory:
```bash
mkdir templates/my-language
```

2. Create `template-info.yaml`:
```yaml
name: My Language
description: Development environment for My Language
version: 1.0.0
lang_version_default: "1.0"
features:
  - name: tool1
    description: Essential tool
supported_versions:
  - "1.0"
  - "2.0"
```

3. Create `.devcontainer/devcontainer.json` with placeholders

4. Test the template:
```bash
cargo run -- init test --template my-language
```

### Template Placeholders

Supported placeholders in `devcontainer.json`:
- `{{PROJECT_NAME}}` - Project name
- `{{LANG_VERSION}}` - Language version
- `{{FEATURES_CLAUDE_CODE}}` - Claude Code feature path
- `{{FEATURES_PROXY}}` - Proxy feature path
- `{{CLAUDE_VERSION}}` - Claude version
- `{{CLAUDE_PROVIDER}}` - Claude provider
- `{{HTTP_PROXY}}` - HTTP proxy URL
- `{{HTTPS_PROXY}}` - HTTPS proxy URL

## Code Style

### Rust Code

- Use `cargo fmt` for formatting
- Follow Rust naming conventions
- Use `Result<T, E>` for error handling
- Document public APIs with rustdoc

### Shell Scripts (feature install.sh)

- Use `set -euo pipefail` for error handling
- Quote variables: `"$VAR"` not `$VAR`
- Comment non-obvious logic
- Function names: `verb_noun()` style

### JSON Configuration

- Use 2-space indentation
- Trailing commas where allowed
- Validate with `jq` or VS Code

### Documentation

- English language only
- Markdown format
- Code fences with language
- Relative links: `[Link](other-file.md)`

## Common Tasks

### Adding a New CLI Command

1. Edit `isolde-cli/src/main.rs`:
```rust
#[derive(Subcommand)]
enum Commands {
    #[command(name = "newcommand")]
    NewCommand {
        #[arg(short, long)]
        option: String,
    },
}

// Handle the command
Commands::NewCommand { option } => {
    // Your implementation
}
```

2. Add corresponding function in `isolde-core` if needed
3. Test with: `cargo run -- newcommand --option value`
4. Update documentation

### Adding a New Feature

1. Create feature directory in `core/features/`:
```bash
mkdir core/features/new-feature
```

2. Create `devcontainer-feature.json`
3. Create `install.sh`
4. Document in `docs/contributor/architecture.md`
5. Test and commit

### Debugging Container Issues

```bash
# View container logs
docker logs claude-code-dev

# Inspect running container
docker inspect claude-code-dev

# Interactive shell for debugging
docker exec -it claude-code-dev bash

# Check environment variables
docker exec claude-code-dev env | sort
```

## Testing

For comprehensive testing documentation, see [testing.md](testing.md).

### Quick Test Reference

```bash
# Run all tests
make test

# Run specific test category
make test-build     # Container builds
make test-config    # Environment variables
make test-runtime   # Docker-in-Docker
make test-providers # Provider configuration
make test-e2e       # E2E tests
```

## Release Process

### Version Bump

Update version in:
- `isolde-cli/Cargo.toml`
- `isolde-core/Cargo.toml`
- `README.md` if needed

### Changelog

Maintain `CHANGELOG.md` with entries:
```markdown
## [2.0.0] - 2025-02-20
### Added
- Rust CLI implementation
- Plugin manager support

### Changed
- Migrated from shell scripts to Rust
```

### Tag Release

```bash
git tag -a v2.0.0 -m "Release v2.0.0"
git push origin v2.0.0
```

## Review Guidelines

### Code Review Checklist

- [ ] Build succeeds without errors
- [ ] Tests pass locally
- [ ] Commit messages follow format
- [ ] Documentation updated
- [ ] No unrelated changes included
- [ ] Atomic commits maintained

### Documentation Review Checklist

- [ ] English language used
- [ ] Code examples tested
- [ ] Links are valid
- [ ] Architecture section accurate
- [ ] Troubleshooting covers common issues

## Getting Help

### Internal Resources

- [CLAUDE.md](../../CLAUDE.md) - Project-specific instructions
- [Architecture](architecture.md) - System design details
- [Proxy Configuration](../devcontainer/proxy.md) - Proxy architecture

### External Resources

- [Dev Containers Spec](https://devcontainers.github.io/implementors/spec/)
- [Claude Code Docs](https://code.claude.com/docs)
- [Docker Documentation](https://docs.docker.com/)
- [Rust Documentation](https://www.rust-lang.org/docs)

### Reporting Issues

When reporting issues, include:
1. Command that failed
2. Error messages or logs
3. Steps to reproduce
4. Environment details (OS, Rust version, Docker version)
