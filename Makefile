# =============================================================================
# Isolde (ISOLated Development Environment) - Root Makefile
# =============================================================================
# This Makefile provides a unified interface for building images, running
# linting, and executing tests with full CI parity.
#
# Usage:
#   make            - Build the devcontainer image
#   make test       - Run all tests
#   make help       - Show all available targets
# =============================================================================

.PHONY: all build test clean lint install help

# Include modular makefiles
include mk/common.mk
include mk/build.mk
include mk/lint.mk
include mk/tests.mk
include mk/clean.mk
include mk/install.mk

# =============================================================================
# Default Target
# =============================================================================
all: build

# =============================================================================
# Main Entry Points - High-Level Targets
# =============================================================================
.PHONY: docker-build

# Build target (delegates to build.mk)
build: docker-build

# Test target (comprehensive CI parity)
test: lint test-build test-config test-runtime test-providers test-e2e

# Lint target (all fast checks)
lint: lint-shell lint-json lint-bats

# Clean target (containers only)
clean: clean-containers
