# Root-Level Makefile Design

**Date:** 2025-02-16
**Status:** Approved

## Overview

A modular root-level Makefile that orchestrates container builds, linting, and testing with full CI parity. The design uses includes for maintainability and provides a single entry point for all development workflows.

## File Structure

```
/
├── Makefile               # Main entry point, orchestrates everything
├── mk/
│   ├── common.mk         # Shared variables (colors, paths, utility targets)
│   ├── build.mk          # Docker image build targets
│   ├── lint.mk           # Fast checks (shellcheck, jq, bats)
│   ├── tests.mk          # All test targets (container, e2e, providers)
│   └── clean.mk          # Cleanup targets (containers, artifacts)
```

## Design Principles

1. **Modularity** - Each `.mk` file is self-contained and independently understandable
2. **CI Parity** - `make test` runs exactly what GitHub Actions runs
3. **Graceful Degradation** - Missing tools warn locally, fail in CI
4. **YAGNI** - Only targets needed for current workflow, no speculative features

## Module Specifications

### `mk/common.mk`

Shared variables and utilities:

```makefile
# Image names
IMAGE_NAME ?= claude-code-dev
TEST_IMAGE_PREFIX = claude-code-dev-test

# Paths
DEVCONTAINER_DIR = .devcontainer
E2E_DIR = tests/e2e
SCRIPTS_DIR = scripts

# Docker
DOCKER_BUILDKIT = 1
DOCKER_BUILD_CONTEXT = $(DEVCONTAINER_DIR)

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
CYAN := \033[0;36m
RESET := \033[0m
```

### `mk/build.mk`

Container image creation:

| Target | Description |
|--------|-------------|
| `build` | Build the devcontainer image |
| `build-provider` | Build with specific provider (`PROVIDER=z.ai`) |
| `rebuild` | Force no-cache rebuild |

### `mk/lint.mk`

Fast static checks:

| Target | Description |
|--------|-------------|
| `lint` | Run all lint checks |
| `lint-shell` | shellcheck all `.sh` files |
| `lint-json` | jq validate all `.json` files |
| `lint-bats` | Run Bats unit tests |

### `mk/tests.mk`

All test categories:

| Target | Description |
|--------|-------------|
| `test` | Run ALL tests (CI parity) |
| `test-build` | Container builds successfully |
| `test-config` | Environment variables (HTTP_PROXY, etc.) |
| `test-runtime` | Docker-in-Docker works |
| `test-providers` | Provider directory creation |
| `test-e2e` | Behave E2E tests (Docker-based, `--tags=~cli`) |
| `test-e2e-all` | E2E including CLI tests |
| `test-e2e-cli` | E2E CLI tests only |

**E2E Variables:**
- `VERBOSE=1` - Show verbose Behave output
- `SCENARIO="name"` - Run specific scenario

### `mk/clean.mk`

Cleanup operations:

| Target | Description |
|--------|-------------|
| `clean` | Remove running containers |
| `clean-images` | Remove built images |
| `clean-all` | Full cleanup (containers + images + artifacts) |
| `clean-e2e` | Remove E2E test artifacts |

### Main `Makefile`

Top-level orchestration:

```makefile
.PHONY: all build test clean lint help

include mk/common.mk
include mk/build.mk
include mk/lint.mk
include mk/tests.mk
include mk/clean.mk

# Default target
all: build

# Main entry points
build: docker-build
test: lint test-build test-config test-runtime test-providers test-e2e
lint: lint-shell lint-json lint-bats
clean: clean-containers
```

## Usage Examples

```bash
# Default: build image
make

# Run all tests (CI parity)
make test

# Quick development cycle
make build && make test-e2e SCENARIO="Create Python project"

# Clean everything
make clean-all

# Show available targets
make help
```

## CI Integration

The GitHub Actions workflow (`.github/workflows/test.yml`) can be simplified to use the Makefile:

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: make test
```

## Implementation Notes

1. **Tool Detection** - Check for `shellcheck`, `jq`, `bats` before use
2. **CI Mode** - Fail hard if tools missing when `CI` env var is set
3. **E2E Integration** - Delegate to `tests/e2e/run-tests.sh` with appropriate flags
4. **Docker BuildKit** - Always use for better performance
