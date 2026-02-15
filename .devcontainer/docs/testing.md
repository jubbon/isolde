# Testing Guide

Comprehensive guide for testing the Claude Code Dev Container project.

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
| **Makefile Tests** | Bash make | Fast integration tests (build, config, runtime) |
| **Bats Tests** | Bats | Unit tests for shell scripts and JSON |
| **CI/CD** | GitHub Actions | Automated testing on push/PR |

## Quick Start

```bash
# Run all tests
make test

# Run specific test category
make test-build     # Container builds
make test-config    # Environment variables
make test-runtime   # Docker-in-Docker
make test-shell     # Shell script syntax
make test-lint      # JSON validity
make test-bats      # Bats unit tests

# See all available tests
make test-help
```

## Test Frameworks

### Makefile Tests

Located in root `Makefile`, these provide fast feedback for common issues.

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

### Bats Tests

Located in `tests/` directory using [Bats](https://github.com/bats-core/bats-core).

**Advantages:**
- Unit-level testing
- Testable assertions
- Better failure messages

**Installing Bats:**
```bash
# macOS
brew install bats-core

# Linux (npm)
npm install -g bats

# Linux (source)
git clone https://github.com/bats-core/bats-core.git
cd bats-core && sudo ./install.sh /usr/local
```

**Run:**
```bash
cd tests
bats .                    # Run all
bats install.bats          # Run specific file
bats -v .                 # Verbose output
bats --pretty .            # Pretty formatting
```

## Running Tests

### Local Development

1. **Before committing changes:**
   ```bash
   make test              # Run all Makefile tests
   cd tests && bats .     # Run Bats tests
   ```

2. **Quick iteration:**
   ```bash
   make test-build       # Just test build
   make test-bats        # Just run Bats
   ```

3. **Full verification:**
   ```bash
   make clean && make build && make test
   cd tests && bats --timing .
   ```

### CI/CD

Tests run automatically on:
- Push to `main`/`master`
- Pull requests
- Manual trigger (Actions tab)

**View results:**
```
https://github.com/YOUR_USERNAME/claude-code-devcontainer/actions
```

## Writing Tests

### Makefile Tests

Add new test target to `Makefile`:

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

Then add to main `test` target:
```makefile
test: test-build test-config test-runtime test-new-feature
```

### Bats Tests

Create new `.bats` file in `tests/`:

```bash
#!/usr/bin/env bats
load bats/core

@test "description of test" {
	# Arrange
	local result="expected"

	# Act
	assert [ "$result" = "expected" ]

	# Assert (implicit)
}

@test "use helper functions" {
	assert_valid_json "path/to/file.json"
	assert_shellcheck_pass "path/to/script.sh"
}
```

**Available helpers** (in `tests/bats/core.bash`):
- `assert_valid_json <file>` - Check JSON validity
- `assert_shellcheck_pass <script>` - Run shellcheck
- `assert_contains <file> <string>` - Check file contains string
- `assert_executable <file>` - Check file is executable

## Best Practices

### Test Organization

1. **Unit tests in `tests/`** - Test individual functions/files
2. **Integration tests in Makefile** - Test end-to-end scenarios
3. **One test per concern** - Don't combine unrelated checks
4. **Clear test names** - Describe what is being tested

### Test Reliability

1. **Avoid external dependencies** - Tests should run offline
2. **Clean up after tests** - Don't leave containers/files behind
3. **Make tests fast** - Parallelize when possible
4. **Use timeouts** - Don't let tests hang forever

### CI/CD Best Practices

1. **Fast feedback** - Run lint before build
2. **Matrix testing** - Test multiple providers in parallel
3. **Clear failures** - Show what failed and why
4. **Document flakes** - Note intermittent failures

## Test Categories Reference

| Category | Makefile Target | Bats File | CI Job |
|----------|-----------------|------------|---------|
| Build | `test-build` | - | `build` |
| Configuration | `test-config` | - | `config` |
| Runtime | `test-runtime` | - | `runtime` |
| Shell Scripts | `test-shell` | `install.bats` | `lint` |
| JSON Files | `test-lint` | `json.bats` | `lint` |
| Providers | `test-providers` | `provider.bats` | `provider` |
| Unit Tests | `test-bats` | `*.bats` | `lint` |

## Troubleshooting

### Bats not found

```bash
# Install via npm
npm install -g bats

# Or add to project
npm install --save-dev bats
```

### Docker build fails

```bash
# Check logs
cat /tmp/build-test.log

# Build manually for more details
cd .devcontainer
docker build -t claude-code-dev-test .
```

### shellcheck errors

```bash
# Install shellcheck
sudo apt-get install shellcheck  # Debian/Ubuntu
brew install shellcheck               # macOS

# Check specific file
shellcheck .devcontainer/features/claude-code/install.sh
```

## Contributing Tests

When adding new features:

1. **Write tests first** (TDD approach)
2. **Ensure all tests pass** locally
3. **Update this doc** if adding test categories
4. **CI will verify** on push/PR

See [development.md](development.md) for full contribution guidelines.
