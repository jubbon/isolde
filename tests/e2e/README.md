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
