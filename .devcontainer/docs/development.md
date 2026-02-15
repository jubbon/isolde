# Development Guide

Guide for contributing to and developing the Claude Code Dev Container project.

## Development Workflow

### Prerequisites

- Docker installed and running
- Git configured
- Basic understanding of Dev Containers

### Getting Started

```bash
# Fork and clone repository
git clone https://github.com/your-username/claude-code-devcontainer.git
cd claude-code-devcontainer

# Create a feature branch
git checkout -b feat/my-new-feature

# Start development container
make devcontainer
```

## Project Structure

```
.
├── .devcontainer/              # Dev Container definition
│   ├── devcontainer.json       # Main configuration
│   ├── Dockerfile              # Base image
│   ├── PROXY_ARCHITECTURE.md   # Proxy documentation
│   ├── docs/                  # Documentation (this folder)
│   └── features/
│       └── claude-code/         # Claude Code feature
│           ├── devcontainer-feature.json
│           ├── install.sh
│           └── README.md
├── .claude/                    # Claude Code settings
├── CLAUDE.md                   # Project instructions
├── README.md                   # Project overview
└── Makefile                    # Build commands
```

## Making Changes

### 1. Modify Container Image

**Dockerfile Changes:**
```bash
# Edit .devcontainer/Dockerfile
vim .devcontainer/Dockerfile

# Test build
make build
```

**Feature Changes:**
```bash
# Edit feature files
vim .devcontainer/features/claude-code/install.sh

# Rebuild container
# In VS Code: F1 → Dev Containers: Rebuild Container
# Or CLI:
docker build -t claude-code-dev .devcontainer
```

### 2. Test Your Changes

```bash
# Run container with changes
make devcontainer

# Verify inside container
docker exec -it claude-code-dev bash
claude --version
docker ps  # Test DinD
```

### 3. Commit Standards

This project enforces **atomic commits**. See [CLAUDE.md](../CLAUDE.md) for full standards.

**Pre-commit Verification:**
```bash
# Build test - REQUIRED before committing
cd .devcontainer && docker build -t claude-code-dev .

# Environment verification
make devcontainer
# Inside container:
echo $HTTP_PROXY
echo $ANTHROPIC_AUTH_TOKEN
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

## Testing Strategy

### Unit Testing

Test individual components:

```bash
# Test installation script
bash .devcontainer/features/claude-code/install.sh

# Test with different providers
# Modify devcontainer.json provider option
# Rebuild and verify
```

### Integration Testing

Test full container build:

```bash
# Full build test
make build

# Runtime test
make devcontainer

# Inside container:
# - Verify Claude Code works
# - Verify provider configuration
# - Verify Docker-in-Docker
# - Verify proxy settings
```

### Provider Testing

Test with each provider:

| Provider | Test Command |
|----------|--------------|
| Anthropic | `echo $ANTHROPIC_AUTH_TOKEN` |
| Z.ai | `echo $ANTHROPIC_BASE_URL` |
| Custom | Check provider directory exists |

## Code Style

### Shell Scripts (install.sh)

- Use `set -euo pipefail` for error handling
- Quote variables: `"$VAR"` not `$VAR`
- Comment non-obvious logic
- Function names: `verb_noun()` style

### JSON Configuration

- Use 2-space indentation
- Trailing commas where allowed
- Comments in JSON5 if needed
- Validate with `jq` or VS Code

### Documentation

- English language only
- Markdown format
- Code fences with language
- Relative links: `[Link](other-file.md)`

## Testing

For comprehensive testing documentation, see [testing.md](testing.md).

### Quick Test Reference

The project includes a multi-layered testing system:

**Run all tests:**
```bash
make test              # Run all Makefile tests
make test-bats          # Run Bats unit tests
```

**Run individual test categories:**
```bash
make test-build       # Test container builds
make test-config      # Test environment configuration
make test-runtime     # Test Claude Code CLI functionality
make test-shell       # Test shell script syntax (requires shellcheck)
make test-lint        # Test JSON validity (requires jq)
make test-bats        # Run Bats unit tests (requires bats)
```

### Test Categories

| Test Category | Purpose | Command |
|---------------|---------|---------|
| **Build Test** | Verifies container builds without errors | `make test-build` |
| **Config Test** | Verifies environment variables are set correctly | `make test-config` |
| **Runtime Test** | Verifies Claude Code CLI is functional | `make test-runtime` |
| **Shell Test** | Verifies shell script syntax with shellcheck | `make test-shell` |
| **Lint Test** | Verifies JSON files are valid | `make test-lint` |
| **Bats Test** | Runs unit tests in `tests/` directory | `make test-bats` |

### Adding New Tests

To add a new test category:

1. **Add target to Makefile:**
```makefile
test-new-category:
	@echo "Testing: New category..."
	# Test logic here
	@if [ $$? -eq 0 ]; then \
		echo "✓ New category test PASSED"; \
	else \
		echo "✗ New category test FAILED"; \
		exit 1; \
	fi
```

2. **Add to test target dependencies:**
```makefile
test: test-build test-config test-runtime test-new-category
```

3. **Document in this file** - Add to test categories table above

## Common Tasks

### Adding a New Provider Option

1. Update `devcontainer-feature.json`:
```json
{
  "options": [
    {
      "id": "provider",
      "type": "string",
      "default": "anthropic",
      "description": "LLM provider name"
    }
  ]
}
```

2. Update `install.sh` to handle provider
3. Update documentation in `docs/providers.md`
4. Test with provider configured
5. Commit: `feat: add support for new-provider`

### Adding a New Feature

1. Create feature directory:
```bash
mkdir .devcontainer/features/new-feature
```

2. Create `devcontainer-feature.json`
3. Create `install.sh`
4. Document in `docs/architecture.md`
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

## Release Process

### Version Bump

Update version in relevant files:
- `.devcontainer/features/claude-code/devcontainer-feature.json`
- `README.md` if needed
- `docs/setup.md` if user-facing

### Changelog

Maintain `CHANGELOG.md` with entries:
```markdown
## [1.2.0] - 2025-02-13
### Added
- Multi-provider support
- Proxy configuration options

### Fixed
- Docker socket permission issues
```

### Tag Release

```bash
git tag -a v1.2.0 -m "Release v1.2.0"
git push origin v1.2.0
```

## Review Guidelines

### Code Review Checklist

- [ ] Build succeeds without errors
- [ ] Pre-commit verification completed
- [ ] Commit messages follow format
- [ ] Documentation updated
- [ ] Tests pass locally
- [ ] No unrelated changes included
- [ ] Atomic commits maintained

### Documentation Review Checklist

- [ ] English language used
- [ ] Code examples tested
- [ ] Links are valid
- [ ] Architect section accurate
- [ ] Troubleshooting covers common issues

## Getting Help

### Internal Resources

- [CLAUDE.md](../CLAUDE.md) - Project-specific instructions
- [Architecture](architecture.md) - System design details
- [Proxy Configuration](proxy.md) - Proxy architecture

### External Resources

- [Dev Containers Spec](https://devcontainers.github.io/implementors/spec/)
- [Claude Code Docs](https://code.claude.com/docs)
- [Docker Documentation](https://docs.docker.com/)

### Reporting Issues

When reporting issues, include:
1. `devcontainer.json` configuration
2. Provider being used
3. Error messages or logs
4. Steps to reproduce
5. Environment details (OS, Docker version)
