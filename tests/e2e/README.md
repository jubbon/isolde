# E2E Tests for Devcontainer Templates

Behavior-Driven Development (BDD) tests using Python's Behave framework to validate devcontainer templates.

## Prerequisites

- Docker installed and running
- Python 3.8+
- Node.js 20+ (for Dev Containers CLI tests)

## Installation

```bash
# Install Python dependencies
pip install -r requirements.txt

# Install Dev Containers CLI (optional, for CLI-specific tests)
npm install -g @devcontainers/cli
```

## Running Tests

### Using the Test Runner Script

```bash
# Run all tests (excluding CLI tests)
./run-tests.sh

# Run all tests including CLI tests
./run-tests.sh --all

# Run only CLI tests
./run-tests.sh --cli

# Run specific test categories
./run-tests.sh --presets      # Preset coverage tests
./run-tests.sh --versions     # Multi-version language tests
./run-tests.sh --edge         # Edge case and negative tests
./run-tests.sh --config       # Configuration option tests
./run-tests.sh --concurrent   # Concurrent operation tests

# Run with verbose output
./run-tests.sh --verbose

# Run specific scenario by name
./run-tests.sh --name "Basic Python template"

# Show help
./run-tests.sh --help
```

### Using Behave Directly

```bash
# Run all tests
behave

# Run specific feature
behave features/python-template.feature

# Run with pretty format
behave --format pretty

# Run specific scenario by name
behave --name "Basic Python template"

# Skip Dev Containers CLI tests (requires @devcontainers/cli)
behave --tags=~cli

# Run by tag
behave --tags=preset
behave --tags=version
behave --tags=edge-case
behave --tags=config
behave --tags=concurrent
```

## Test Categories

### 1. Template Tests (Existing)
- `python-template.feature` - Python template with ML and web presets
- `nodejs-template.feature` - Node.js template with TypeScript
- `common-templates.feature` - Rust and Go templates

### 2. Preset Coverage Tests
- `presets.feature` - Tests for previously untested presets:
  - `rust-cli` - Rust CLI application development
  - `go-service` - Go microservice development
  - `minimal` - Minimal devcontainer for any project
  - `fullstack` - Full-stack development with Node.js

### 3. Multi-Version Language Tests
- `multi-version.feature` - Alternative language versions:
  - Python 3.10, 3.11
  - Node.js 18, 20
  - Go 1.21, 1.22
  - Rust stable

### 4. Edge Cases and Negative Tests
- `edge-cases.feature` - Edge case handling:
  - Invalid template/preset/version names
  - Project names with spaces, dashes, underscores, numbers
  - Very long project names
  - Empty project names
  - Unicode characters in project names
  - Creating projects in existing directories

### 5. Configuration Tests
- `configuration.feature` - Configuration options:
  - HTTP/HTTPS proxy settings
  - Claude provider selection
  - Claude version specification
  - Combined configuration options

### 6. Concurrent Operations Tests
- `concurrent-operations.feature` - Concurrent project creation:
  - Multiple projects in same workspace
  - Different templates simultaneously
  - Duplicate name handling

### 7. Other Tests
- `vscode-compatibility.feature` - VS Code compatibility checks
- `devcontainers-cli.feature` - Dev Containers CLI integration

## Test Structure

```
tests/e2e/
├── features/              # Gherkin feature files
│   ├── python-template.feature
│   ├── nodejs-template.feature
│   ├── common-templates.feature
│   ├── presets.feature           # New: Preset coverage
│   ├── multi-version.feature     # New: Language versions
│   ├── edge-cases.feature        # New: Edge cases
│   ├── configuration.feature     # New: Configuration options
│   ├── concurrent-operations.feature  # New: Concurrent operations
│   ├── vscode-compatibility.feature
│   └── devcontainers-cli.feature
├── step_definitions/      # Python step implementations
│   ├── generator_steps.py
│   ├── container_steps.py
│   ├── validation_steps.py
│   ├── vscode_steps.py
│   ├── devcontainer_cli_steps.py
│   ├── preset_steps.py          # New: Preset-specific steps
│   ├── edge_case_steps.py       # New: Edge case steps
│   ├── config_steps.py          # New: Configuration steps
│   └── concurrent_steps.py      # New: Concurrent operation steps
└── support/              # Support code
    ├── environment.py    # Setup/teardown
    └── generators.py     # Generator abstraction
```

## Generator Abstraction

Tests are agnostic to the generation mechanism. The `GeneratorInterface` allows:

- `ShellScriptGenerator` - Uses `init-project.sh` script
- `ClaudeBinaryGenerator` - Future Rust binary implementation

The generator now supports additional options:
- `--http-proxy` - HTTP proxy configuration
- `--https-proxy` - HTTPS proxy configuration
- `--claude-provider` - Claude provider selection
- `--claude-version` - Specific Claude version

## Writing New Tests

1. Add scenario to `.feature` file in `features/`
2. Implement steps in `step_definitions/*.py`
3. Use existing step patterns when possible
4. Tag scenarios appropriately (@preset, @version, @edge-case, @config, @concurrent)

## Cleanup

Test containers and images are automatically cleaned up after each scenario. If cleanup fails:

```bash
# Remove test images
docker images | grep e2e- | awk '{print $3}' | xargs docker rmi -f

# Clean temp directories
rm -rf /tmp/e2e-*
```

## CI Integration

Tests run automatically in GitHub Actions via `.github/workflows/test.yml`.

## Troubleshooting

### Tests fail with "project path not found"

This usually means the generator created the project in the wrong location. Check:
1. The workspace path in `environment.py`
2. The generator implementation in `support/generators.py`

### Container build fails

1. Check the devcontainer path exists
2. Verify Docker is running: `docker ps`
3. Check build logs in test output

### Dev Containers CLI tests fail

1. Install CLI: `npm install -g @devcontainers/cli`
2. Verify installation: `devcontainer --version`
3. Run CLI tests separately: `behave --tags=cli`

### Cleanup failed

Manually clean resources:
```bash
# Remove test images
docker images | grep e2e- | awk '{print $3}' | xargs docker rmi -f

# Clean temp directories
rm -rf /tmp/e2e-*

# Stop any running devcontainers
devcontainer list --verbose
```

### Tests are slow

- Run specific features: `behave features/python-template.feature`
- Run specific scenarios: `behave --name "scenario name"`
- Run specific categories: `./run-tests.sh --presets`
- Skip CLI tests: `behave --tags=~cli`

### ModuleNotFoundError: No module named 'generators'

This indicates the sys.path is incorrect in step definitions. Ensure:
1. Step files are in `tests/e2e/features/steps/`
2. sys.path uses absolute path: `os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support'))`

### Python version mismatch

The Python version check allows minor differences (e.g., 3.11.2 vs 3.12). Warnings are shown but tests pass.

### uv/pytest/Jupyter not found

These tools are installed via `postCreateCommand` which doesn't run during Docker build. These checks are skipped with warnings during E2E tests.

### Edge case tests fail unexpectedly

Some edge case tests use non-interactive mode (newlines for defaults). Invalid inputs may be accepted as defaults rather than failing. This is expected behavior.
