# Contributing to Isolde

Thank you for your interest in contributing to Isolde! This guide will help you get started.

## Prerequisites

- **Rust toolchain** — install via [rustup](https://rustup.rs/) (edition 2021, see `Cargo.toml` for MSRV)
- **Docker** — for running and testing devcontainers
- **Git**

## Getting Started

```bash
# Clone the repository
git clone https://github.com/jubbon/isolde.git
cd isolde

# Build
cargo build

# Run tests
make test

# Install locally
make install
```

## Development Workflow

1. Create a branch from `dev` (not `main`)
2. Make your changes
3. Run checks before committing:

```bash
# Format
cargo fmt

# Lint
cargo clippy

# Test
cargo test

# All checks at once
make build/check
```

4. Commit using [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` — new feature
   - `fix:` — bug fix
   - `docs:` — documentation changes
   - `refactor:` — code restructuring without behavior change
   - `test:` — adding or fixing tests
   - `chore:` — maintenance tasks

5. Open a PR against `dev`

## Project Structure

```
isolde/
├── isolde-core/    # Core library (template processing, config, container management)
├── isolde-cli/     # CLI binary (clap-based)
├── core/features/  # Devcontainer features (claude-code, proxy, etc.)
├── templates/      # Language templates (python, nodejs, rust, go, generic)
└── presets.yaml    # Preset configurations
```

See [CLAUDE.md](CLAUDE.md) for detailed architecture documentation.

## Testing

```bash
# Rust unit tests + clippy
make test

# Docker container tests (requires Docker)
make test-docker

# E2E tests (requires Docker)
make test-e2e

# Everything
make test-all
```

## Adding a Template

See [docs/contributor/adding-templates.md](docs/contributor/adding-templates.md) for a step-by-step guide.

## Language Policy

All documentation, commit messages, and code comments must be in **English**.

## Questions?

Open an [issue](https://github.com/jubbon/isolde/issues) to discuss your idea before submitting a PR.
