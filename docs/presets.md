# Preset Reference

## Overview

Presets are pre-configured combinations of templates, language versions, and features. They provide a quick way to create projects with common setups.

## Built-in Presets

### python-ml

**Description**: Python machine learning environment with Jupyter support

| Setting | Value |
|---------|-------|
| Template | python |
| Language Version | 3.12 |
| Features | uv, jupyter, pytest, numpy, pandas |
| Claude Plugins | oh-my-claudecode, tdd |

```bash
./scripts/isolde.sh ml-app --preset=python-ml
```

### python-web

**Description**: Python web development environment

| Setting | Value |
|---------|-------|
| Template | python |
| Language Version | 3.11 |
| Features | uv, pytest, ruff, black |
| Claude Plugins | oh-my-claudecode |

```bash
./scripts/isolde.sh web-app --preset=python-web
```

### node-api

**Description**: Node.js API development with TypeScript

| Setting | Value |
|---------|-------|
| Template | nodejs |
| Language Version | 22 |
| Features | typescript, eslint, vitest, prettier |
| Claude Plugins | oh-my-claudecode, security-review |

```bash
./scripts/isolde.sh api --preset=node-api
```

### rust-cli

**Description**: Rust CLI application development

| Setting | Value |
|---------|-------|
| Template | rust |
| Language Version | latest |
| Features | cargo, clippy, rustfmt |
| Claude Plugins | oh-my-claudecode |

```bash
./scripts/isolde.sh mycli --preset=rust-cli
```

### go-service

**Description**: Go microservice development

| Setting | Value |
|---------|-------|
| Template | go |
| Language Version | latest |
| Features | modules, golangci-lint |
| Claude Plugins | oh-my-claudecode |

```bash
./scripts/isolde.sh service --preset=go-service
```

### minimal

**Description**: Minimal devcontainer for any project

| Setting | Value |
|---------|-------|
| Template | generic |
| Language Version | (none) |
| Features | (none) |
| Claude Plugins | oh-my-claudecode |

```bash
./scripts/isolde.sh simple --preset=minimal
```

### fullstack

**Description**: Full-stack development with Node.js backend

| Setting | Value |
|---------|-------|
| Template | nodejs |
| Language Version | 22 |
| Features | typescript, eslint, vitest, prettier |
| Claude Plugins | oh-my-claudecode, tdd, security-review |

```bash
./scripts/isolde.sh fullstack --preset=fullstack
```

## Preset Configuration File

Presets are defined in `presets.yaml`:

```yaml
presets:
  python-ml:
    template: python
    description: Python machine learning environment
    lang_version: "3.12"
    features:
      - uv
      - jupyter
      - pytest
    claude_plugins: oh-my-claudecode,tdd
```

## Custom Presets

### User-Level Presets

Create custom presets in `~/.devcontainer-presets.yaml`:

```yaml
presets:
  my-python-api:
    template: python
    description: My Python API setup
    lang_version: "3.11"
    features:
      - uv
      - fastapi
      - pytest
      - pytest-cov
    claude_plugins: oh-my-claudecode,tdd,security-review

  my-react-app:
    template: nodejs
    description: My React app setup
    lang_version: "20"
    features:
      - typescript
      - eslint
      - vitest
      - prettier
    claude_plugins: oh-my-claudecode
```

Use them like built-in presets:

```bash
./scripts/isolde.sh my-api --preset=my-python-api
```

### Project-Level Presets

You can also add a `presets.yaml` in your project's `.devcontainer/` directory for project-specific presets.

## Preset Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template` | string | Yes | Template name (python, nodejs, etc.) |
| `description` | string | No | Human-readable description |
| `lang_version` | string | No | Language version |
| `features` | array | No | List of feature names |
| `claude_plugins` | string | No | Comma-separated Claude plugin names |

## Claude Plugins

Presets can recommend Claude Code plugins:

| Plugin | Description |
|--------|-------------|
| `oh-my-claudecode` | Multi-agent orchestration |
| `tdd` | Test-driven development enforcement |
| `security-review` | Security vulnerability scanning |

Plugins are documented in the created `CLAUDE.md` file for reference.

## Creating Presets from Existing Projects

To create a preset from an existing project:

1. Review the project's `.devcontainer/devcontainer.json`
2. Note the language version and features used
3. Add an entry to `presets.yaml` (or user presets)

Example from an existing Python project:

```yaml
presets:
  existing-project-style:
    template: python
    description: Based on my existing project
    lang_version: "3.10"  # From devcontainer.json
    features:
      - uv
      - pytest
      - black
      - mypy
    claude_plugins: oh-my-claudecode
```

## Preset Inheritance

You can reference another preset as a base:

```yaml
presets:
  base-python:
    template: python
    lang_version: "3.12"
    features:
      - uv
      - pytest

  python-extended:
    extends: base-python
    features:
      - jupyter
      - ipython
    claude_plugins: oh-my-claudecode
```

## Preset Validation

When using a preset, the following validations are performed:

1. Preset exists in `presets.yaml` or user presets
2. Referenced template exists
3. Language version is supported by the template

If validation fails, an error message explains the issue.
