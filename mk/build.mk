# =============================================================================
# Rust Build Targets
# =============================================================================
# Builds the Isolde v2 Rust CLI utility

# Cargo binary name
BINARY_NAME = isolde
# Cargo workspace directory
CARGO_DIR = $(CURDIR)

# Build directories
TARGET_DIR = $(CARGO_DIR)/target
RELEASE_DIR = $(TARGET_DIR)/release
BINARY_PATH = $(RELEASE_DIR)/$(BINARY_NAME)

CARGO_FEATURES =

.PHONY: build/release build/clean build/test build/dev

## Build the Rust release binary
build/release:
	@echo "$(CYAN)Building Isolde v2 (Rust)...$(RESET)"
	@cd $(CARGO_DIR) && cargo build --release $(if $(CARGO_FEATURES),--features $(CARGO_FEATURES),)
	@echo "$(GREEN)Build complete: $(BINARY_PATH)$(RESET)"
	@echo "$(YELLOW)Binary size: $$(du -h $(BINARY_PATH) | cut -f1)$(RESET)"

## Build the Rust binary (dev mode, faster compilation)
build/dev:
	@echo "$(CYAN)Building Isolde v2 (Rust, dev mode)...$(RESET)"
	@cd $(CARGO_DIR) && cargo build $(if $(CARGO_FEATURES),--features $(CARGO_FEATURES),)
	@echo "$(GREEN)Dev build complete: $(TARGET_DIR)/debug/$(BINARY_NAME)$(RESET)"

## Run Rust tests
build/test:
	@echo "$(CYAN)Running Rust tests...$(RESET)"
	@cd $(CARGO_DIR) && cargo test --all
	@echo "$(GREEN)Tests passed$(RESET)"

## Run Rust tests with output
build/test-verbose:
	@echo "$(CYAN)Running Rust tests (verbose)...$(RESET)"
	@cd $(CARGO_DIR) && cargo test --all -- --nocapture
	@echo "$(GREEN)Tests passed$(RESET)"

## Run Rust clippy linter
build/lint:
	@echo "$(CYAN)Running Rust linter (clippy)...$(RESET)"
	@cd $(CARGO_DIR) && cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)Clippy checks passed$(RESET)"

## Check Rust code (format and lint)
build/check: build/lint
	@echo "$(CYAN)Checking Rust code formatting...$(RESET)"
	@cd $(CARGO_DIR) && cargo fmt --all -- --check
	@echo "$(GREEN)Code formatting check passed$(RESET)"

## Format Rust code
build/fmt:
	@echo "$(CYAN)Formatting Rust code...$(RESET)"
	@cd $(CARGO_DIR) && cargo fmt --all

## Clean Rust build artifacts
build/clean:
	@echo "$(CYAN)Cleaning Rust build artifacts...$(RESET)"
	@cd $(CARGO_DIR) && cargo clean
	@echo "$(GREEN)Rust artifacts cleaned$(RESET)"

## Update Rust dependencies
build/update:
	@echo "$(CYAN)Updating Rust dependencies...$(RESET)"
	@cd $(CARGO_DIR) && cargo update
	@echo "$(GREEN)Dependencies updated$(RESET)"

## Show Rust build info
build/info:
	@echo "$(CYAN)Isolde v2 Rust Build Information$(RESET)"
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
	@echo "$(ARROW) $(GREEN)Rust commands:$(RESET)"
	@echo "  make build/release      - Build release binary"
	@echo "  make build/dev          - Build dev binary (faster)"
	@echo "  make build/test          - Run tests"
	@echo "  make build/lint          - Run clippy"
	@echo "  make build/check         - Check code (format + lint)"
	@echo "  make build/fmt           - Format code"
	@echo "  make build/clean         - Clean build artifacts"
	@echo "  make build/info          - Show build information"
