# Testing Guide

Comprehensive guide for testing the Isolde project.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Test Frameworks](#test-frameworks)
- [Running Tests](#running-tests)
- [Writing Tests](#writing-tests)
- [CI/CD](#cicd)
- [Best Practices](#best-practices)

## Overview

This project uses a multi-layered testing approach:

| Layer | Tool | Purpose |
|-------|------|---------|
| **Rust Tests** | cargo test | Unit and integration tests |
| **Makefile Tests** | Bash make | Fast integration tests (build, config, runtime) |
| **E2E Tests** | Rust + Docker | End-to-end CLI testing |
| **CI/CD** | GitHub Actions | Automated testing on push/PR |

## Quick Start

```bash
# Run all tests (CI parity)
make test

# Run specific test category
make test-build     # Container builds
make test-config    # Environment variables
make test-runtime   # Docker-in-Docker
make test-providers # Provider configuration
make test-e2e       # E2E tests (Rust CLI)

# Rust-specific tests
make rust-test      # Run Rust unit/integration tests

# See all available targets
make help
```

## Test Frameworks

### Rust Tests

Located in `isolde-core/src/` and `isolde-cli/src/` using Rust's built-in test framework.

**Advantages:**
- First-class Rust support
- Fast execution
- Easy to debug
- Good IDE integration

**Run:**
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p isolde-core
cargo test -p isolde-cli

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_template_loading
```

### Makefile Tests

Located in `mk/tests.mk`, these provide fast feedback for common issues.

**Advantages:**
- No dependencies beyond Docker
- Fast execution
- Simple to understand

**Run:**
```bash
make test              # All tests
make test-build        # Build test only
make test-config       # Config test only
```

### E2E Tests

Located in `tests/` directory using Rust and Docker.

**Advantages:**
- Tests full CLI workflow
- Real project creation
- Integration with Docker

**Run:**
```bash
# Run all E2E tests
make test-e2e

# Run specific scenario
SCENARIO='basic_init' make test-e2e

# Verbose output
VERBOSE=1 make test-e2e
```

## Running Tests

### Local Development

1. **Before committing changes:**
   ```bash
   make test              # Run all Makefile tests
   make rust-test         # Run Rust tests
   make test-e2e          # Run E2E tests
   ```

2. **Quick iteration:**
   ```bash
   make test-build       # Just test build
   cargo test -p isolde-core  # Just test core library
   ```

3. **Full verification:**
   ```bash
   make clean && make rust-build && make test
   ```

### CI/CD

Tests run automatically on:
- Push to `main`/`master`
- Pull requests
- Manual trigger (Actions tab)

## Writing Tests

### Rust Unit Tests

Add tests to the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_loading() {
        let template = load_template("python").unwrap();
        assert_eq!(template.name, "Python");
    }

    #[test]
    fn test_invalid_template() {
        let result = load_template("nonexistent");
        assert!(result.is_err());
    }
}
```

### Rust Integration Tests

Create tests in `tests/` directory:

```rust
// tests/cli_tests.rs
use assert_cmd::Command;

#[test]
fn test_init_command() {
    Command::cargo_bin("isolde")
        .unwrap()
        .args(["init", "test-project", "--template", "python"])
        .assert()
        .success();
}
```

### Makefile Tests

Add new test target to `mk/tests.mk`:

```makefile
# Test: Your new test
test-new-feature:
	@echo "$(CYAN)Testing:$(RESET) Your feature..."
	@if your_test_command; then \
		echo "  $(GREEN)Test PASSED$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)Test FAILED$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@echo ""
```

Then add to main `test` target in root `Makefile`:
```makefile
test: lint test-build test-config test-runtime test-providers test-e2e test-new-feature
```

## Best Practices

### Test Organization

1. **Unit tests in `src/`** - Test individual functions/modules
2. **Integration tests in `tests/`** - Test end-to-end scenarios
3. **E2E tests in `tests/`** - Test full CLI workflows
4. **One test per concern** - Don't combine unrelated checks
5. **Clear test names** - Describe what is being tested

### Test Reliability

1. **Avoid external dependencies** - Tests should run offline when possible
2. **Clean up after tests** - Don't leave containers/files behind
3. **Make tests fast** - Parallelize when possible
4. **Use timeouts** - Don't let tests hang forever

### CI/CD Best Practices

1. **Fast feedback** - Run lint before build
2. **Matrix testing** - Test multiple configurations in parallel
3. **Clear failures** - Show what failed and why
4. **Document flakes** - Note intermittent failures

## Test Categories Reference

| Category | Makefile Target | Cargo Test | CI Job |
|----------|-----------------|------------|---------|
| Unit/Integration | `rust-test` | `cargo test` | `rust-test` |
| Build | `test-build` | - | `build` |
| Configuration | `test-config` | - | `config` |
| Runtime | `test-runtime` | - | `runtime` |
| Providers | `test-providers` | - | `provider` |
| E2E Tests | `test-e2e` | `tests/` | `e2e-tests` |
| Lint | `lint` | `cargo clippy` | `lint` |

## Troubleshooting

### Docker build fails

```bash
# Check logs
cat /tmp/build-test.log

# Build manually for more details
cd .devcontainer
docker build -t claude-code-dev-test .
```

### Rust tests fail

```bash
# Run with output for debugging
cargo test -- --nocapture

# Run specific test with output
cargo test test_name -- --nocapture

# Run tests in verbose mode
cargo test -- --verbose
```

### E2E tests fail

```bash
# Run with verbose output
VERBOSE=1 make test-e2e

# Run specific scenario
SCENARIO='basic_init' VERBOSE=1 make test-e2e

# Check test artifacts
ls -la tests/e2e/
```

## Contributing Tests

When adding new features:

1. **Write tests first** (TDD approach)
2. **Ensure all tests pass** locally
3. **Update this doc** if adding test categories
4. **CI will verify** on push/PR

See [development.md](development.md) for full contribution guidelines.

## Test Data

Test templates and fixtures are located in:
- `templates/` - Real templates used for testing
- `tests/fixtures/` - Test-specific data files

## Mocking

For tests that require external dependencies, use mocking:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Server};

    #[test]
    fn test_with_mock_server() {
        let mut server = Server::new();

        let _mock = server.mock("GET", "/api")
            .with_status(200)
            .with_body("response")
            .create();

        // Test code using mock server URL
    }
}
```
