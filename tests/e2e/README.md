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
```

## Test Structure

```
tests/e2e/
├── features/              # Gherkin feature files
│   ├── python-template.feature
│   ├── nodejs-template.feature
│   ├── common-templates.feature
│   ├── vscode-compatibility.feature
│   └── devcontainers-cli.feature
├── step_definitions/      # Python step implementations
│   ├── generator_steps.py
│   ├── container_steps.py
│   ├── validation_steps.py
│   ├── vscode_steps.py
│   └── devcontainer_cli_steps.py
└── support/              # Support code
    ├── environment.py    # Setup/teardown
    └── generators.py     # Generator abstraction
```

## Generator Abstraction

Tests are agnostic to the generation mechanism. The `GeneratorInterface` allows:

- `ShellScriptGenerator` - Uses `init-project.sh` script
- `ClaudeBinaryGenerator` - Future Rust binary implementation

## Writing New Tests

1. Add scenario to `.feature` file in `features/`
2. Implement steps in `step_definitions/*.py`
3. Use existing step patterns when possible

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
- Skip CLI tests: `behave --tags=~cli`

### ModuleNotFoundError: No module named 'generators'

This indicates the sys.path is incorrect in step definitions. Ensure:
1. Step files are in `tests/e2e/features/steps/`
2. sys.path uses absolute path: `os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support'))`

### Python version mismatch

The Python version check allows minor differences (e.g., 3.11.2 vs 3.12). Warnings are shown but tests pass.

### uv/pytest/Jupyter not found

These tools are installed via `postCreateCommand` which doesn't run during Docker build. These checks are skipped with warnings during E2E tests.
