# Tests

This directory contains automated tests for the Claude Code Dev Container project.

## Test Framework

Tests are written using [Bats](https://github.com/bats-core/bats-core) - Bash Automated Testing System.

### Installing Bats

```bash
# On macOS
brew install bats-core

# On Linux
git clone https://github.com/bats-core/bats-core.git
cd bats-core
sudo ./install.sh /usr/local

# Or via npm
npm install -g bats
```

## Running Tests

### Run all tests
```bash
cd tests
bats .
```

### Run specific test file
```bash
bats install.bats
```

### Run with verbose output
```bash
bats -v .
```

### Run with pretty printing
```bash
bats --pretty .
```

## Test Files

| File | Purpose |
|------|---------|
| `install.bats` | Test install.sh script functionality |
| `json.bats` | Validate JSON configuration files |
| `provider.bats` | Test provider configuration system |

## Adding New Tests

Create a new `.bats` file in this directory:

```bash
#!/usr/bin/env bats
load bats/core

@test "description of your test" {
	# Test code here
	assert [ condition ]
}

@test "another test" {
	# Use helper functions
	assert_valid_json "path/to/file.json"
	assert_shellcheck_pass "path/to/script.sh"
}
```

## CI/CD Integration

These tests run automatically via GitHub Actions:
- On every push to main/master
- On every pull request
- Manual trigger via workflow dispatch

See `../../.github/workflows/test.yml` for configuration.
