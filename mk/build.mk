# =============================================================================
# Build Targets
# =============================================================================
# Builds the Isolde Rust CLI

# Cargo binary name
BINARY_NAME = isolde
# Cargo workspace directory
CARGO_DIR = $(CURDIR)

# Build directories
TARGET_DIR = $(CARGO_DIR)/target
RELEASE_DIR = $(TARGET_DIR)/release
BINARY_PATH = $(RELEASE_DIR)/$(BINARY_NAME)

# Rust build flags
CARGO_BUILD_FLAGS = --release
CARGO_FEATURES =

.PHONY: build/app build/dev build/test build/test-verbose build/lint build/check build/fmt build/clean build/install build/update build/info

## Build the release binary
build/app:
	@echo "$(CYAN)Building Isolde (release)...$(RESET)"
	@cd $(CARGO_DIR) && cargo build $(CARGO_BUILD_FLAGS) $(if $(CARGO_FEATURES),--features $(CARGO_FEATURES),)
	@echo "$(GREEN)Build complete: $(BINARY_PATH)$(RESET)"
	@echo "$(YELLOW)Binary size: $$(du -h $(BINARY_PATH) | cut -f1)$(RESET)"

## Build the binary in dev mode (faster compilation)
build/dev:
	@echo "$(CYAN)Building Isolde (dev)...$(RESET)"
	@cd $(CARGO_DIR) && cargo build
	@echo "$(GREEN)Dev build complete: $(TARGET_DIR)/debug/$(BINARY_NAME)$(RESET)"

## Run tests
build/test:
	@echo "$(CYAN)Running tests...$(RESET)"
	@cd $(CARGO_DIR) && cargo test --all
	@echo "$(GREEN)Tests passed$(RESET)"

## Run tests with output
build/test-verbose:
	@echo "$(CYAN)Running tests (verbose)...$(RESET)"
	@cd $(CARGO_DIR) && cargo test --all -- --nocapture
	@echo "$(GREEN)Tests passed$(RESET)"

## Run clippy linter
build/lint:
	@echo "$(CYAN)Running linter (clippy)...$(RESET)"
	@cd $(CARGO_DIR) && cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)Clippy checks passed$(RESET)"

## Check code formatting and lint
build/check: build/lint
	@echo "$(CYAN)Checking code formatting...$(RESET)"
	@cd $(CARGO_DIR) && cargo fmt --all -- --check
	@echo "$(GREEN)Code formatting check passed$(RESET)"

## Format code
build/fmt:
	@echo "$(CYAN)Formatting code...$(RESET)"
	@cd $(CARGO_DIR) && cargo fmt --all

## Clean build artifacts
build/clean:
	@echo "$(CYAN)Cleaning build artifacts...$(RESET)"
	@cd $(CARGO_DIR) && cargo clean
	@echo "$(GREEN)Build artifacts cleaned$(RESET)"

## Install binary locally via cargo
build/install: build/app
	@echo "$(CYAN)Installing $(BINARY_NAME)...$(RESET)"
	@cargo install --path $(CARGO_DIR) --force
	@echo "$(GREEN)$(BINARY_NAME) installed to ~/.cargo/bin/$(RESET)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(RESET)"
	@echo "  1. Verify: $(CYAN)isolde --version$(RESET)"
	@echo "  2. Create a project: $(CYAN)isolde init my-project$(RESET)"

## Update dependencies
build/update:
	@echo "$(CYAN)Updating dependencies...$(RESET)"
	@cd $(CARGO_DIR) && cargo update
	@echo "$(GREEN)Dependencies updated$(RESET)"

## Show build information
build/info:
	@echo "$(CYAN)Isolde Build Information$(RESET)"
	@echo ""
	@echo "Binary: $(BINARY_NAME)"
	@echo "Target: $(BINARY_PATH)"
	@if [ -f "$(BINARY_PATH)" ]; then \
		echo "Status: $(GREEN)Built$(RESET)"; \
		echo "Size: $$(du -h $(BINARY_PATH) | cut -f1)"; \
		echo "Modified: $$(stat -c %y $(BINARY_PATH) 2>/dev/null || stat -f %Sm $(BINARY_PATH))"; \
	else \
		echo "Status: $(YELLOW)Not built$(RESET)"; \
	fi
	@echo ""
	@echo "$(ARROW) $(GREEN)Build commands:$(RESET)"
	@echo "  make build/app          - Build release binary"
	@echo "  make build/dev          - Build dev binary (faster)"
	@echo "  make build/test         - Run tests"
	@echo "  make build/lint         - Run clippy"
	@echo "  make build/check        - Check code (format + lint)"
	@echo "  make build/fmt          - Format code"
	@echo "  make build/clean        - Clean build artifacts"
	@echo "  make build/install      - Install to ~/.cargo/bin/"
	@echo "  make build/info         - Show build information"
