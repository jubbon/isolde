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
# Default to Rust build
all: build

# =============================================================================
# Main Entry Points - High-Level Targets
# =============================================================================

# Build target (release profile)
build: build/release

# Rebuild target (clean then release build)
rebuild: build/clean build/release

# Test target (Rust tests + lint)
test: build/test lint

# Lint target (all fast checks)
lint: lint-json build/lint

# Clean target (containers and build artifacts)
clean: clean-containers build/clean
