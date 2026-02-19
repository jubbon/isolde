# =============================================================================
# Common Variables and Utilities
# =============================================================================

# Image names
IMAGE_NAME ?= claude-code-dev
TEST_IMAGE_PREFIX = claude-code-dev-test

# Paths
DEVCONTAINER_DIR = images/devcontainer
E2E_DIR = tests/e2e
SCRIPTS_DIR = scripts

# Docker
DOCKER_BUILDKIT = 1
DOCKER_BUILD_CONTEXT = $(DEVCONTAINER_DIR)
export DOCKER_BUILDKIT

# CI Detection
ifdef CI
  CI_MODE = 1
else
  CI_MODE = 0
endif

# =============================================================================
# ANSI Color Codes
# =============================================================================
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
CYAN := \033[0;36m
RESET := \033[0m
CHECK := $(GREEN)$(RESET)
CROSS := $(RED)$(RESET)
ARROW := $(YELLOW)$(RESET)

# =============================================================================
# Utility Functions
# =============================================================================

# Check if command exists
command_exists = $(shell command -v $(1) 2>/dev/null)

# =============================================================================
# Common Targets
# =============================================================================

.PHONY: help
help: ## Show this help message
	@echo "$(CYAN)Available targets:$(RESET)"
	@echo ""
	@echo "$(ARROW) $(GREEN)Build targets:$(RESET)"
	@echo "  make build          - Build devcontainer image"
	@echo "  make build-provider - Build with specific provider (PROVIDER=name)"
	@echo "  make rebuild        - Force rebuild without cache"
	@echo ""
	@echo "$(ARROW) $(GREEN)Lint targets:$(RESET)"
	@echo "  make lint           - Run all lint checks"
	@echo "  make lint-shell     - Check shell scripts with shellcheck"
	@echo "  make lint-json      - Validate JSON files with jq"
	@echo "  make lint-bats      - Run Bats unit tests"
	@echo ""
	@echo "$(ARROW) $(GREEN)Test targets:$(RESET)"
	@echo "  make test           - Run all tests (CI parity)"
	@echo "  make test-build     - Test container builds"
	@echo "  make test-config    - Test environment configuration"
	@echo "  make test-runtime   - Test Docker-in-Docker"
	@echo "  make test-providers - Test provider configuration"
	@echo "  make test-e2e       - Run E2E tests (Docker-based)"
	@echo "  make test-e2e-all   - Run E2E tests including CLI"
	@echo "  make test-e2e-cli   - Run E2E CLI tests only"
	@echo ""
	@echo "$(ARROW) $(GREEN)Clean targets:$(RESET)"
	@echo "  make clean          - Remove running containers"
	@echo "  make clean-images   - Remove built images"
	@echo "  make clean-all      - Full cleanup"
	@echo "  make clean-e2e      - Remove E2E test artifacts"
	@echo ""
	@echo "$(ARROW) $(GREEN)Installation targets:$(RESET)"
	@echo "  make install        - Install Isolde to ~/.isolde/"
	@echo "  make uninstall      - Remove Isolde installation"
	@echo "  make install-info   - Show installation status"
	@echo ""
	@echo "$(ARROW) $(GREEN)Variables:$(RESET)"
	@echo "  VERBOSE=1           - Verbose E2E test output"
	@echo "  SCENARIO='name'     - Run specific E2E scenario"
	@echo "  PROVIDER=name       - Provider for build-provider target"
