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

.PHONY: all build rebuild test clean lint install help

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
# Default to Rust build for v2, fallback to Docker for v1
all: build/test build install

# =============================================================================
# Main Entry Points - High-Level Targets
# =============================================================================

# Build target (release profile)
build: build/release

# Rebuild target (clean then release build)
rebuild: build/clean build/release

# Test target (unit tests + CI parity)
test: build/test lint test-build test-config test-runtime test-providers test-e2e

# Lint target (all fast checks)
lint: lint-json lint-bats build/lint

# Clean target (containers and build artifacts)
clean: clean-containers build/clean
