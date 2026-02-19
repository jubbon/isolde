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

# Rust build flags
CARGO_BUILD_FLAGS = --release
CARGO_FEATURES =

.PHONY: rust-build rust-clean rust-test rust-install rust-dev-build

## Build the Rust release binary
rust-build:
	@echo "$(CYAN)Building Isolde v2 (Rust)...$(RESET)"
	@cd $(CARGO_DIR) && cargo build $(CARGO_BUILD_FLAGS) $(if $(CARGO_FEATURES),--features $(CARGO_FEATURES),)
	@echo "$(GREEN)Build complete: $(BINARY_PATH)$(RESET)"
	@echo "$(YELLOW)Binary size: $$(du -h $(BINARY_PATH) | cut -f1)$(RESET)"

## Build the Rust binary (dev mode, faster compilation)
rust-dev-build:
	@echo "$(CYAN)Building Isolde v2 (Rust, dev mode)...$(RESET)"
	@cd $(CARGO_DIR) && cargo build
	@echo "$(GREEN)Dev build complete: $(TARGET_DIR)/debug/$(BINARY_NAME)$(RESET)"

## Run Rust tests
rust-test:
	@echo "$(CYAN)Running Rust tests...$(RESET)"
	@cd $(CARGO_DIR) && cargo test --all
	@echo "$(GREEN)Tests passed$(RESET)"

## Run Rust tests with output
rust-test-verbose:
	@echo "$(CYAN)Running Rust tests (verbose)...$(RESET)"
	@cd $(CARGO_DIR) && cargo test --all -- --nocapture
	@echo "$(GREEN)Tests passed$(RESET)"

## Run Rust clippy linter
rust-lint:
	@echo "$(CYAN)Running Rust linter (clippy)...$(RESET)"
	@cd $(CARGO_DIR) && cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)Clippy checks passed$(RESET)"

## Check Rust code (format and lint)
rust-check: rust-lint
	@echo "$(CYAN)Checking Rust code formatting...$(RESET)"
	@cd $(CARGO_DIR) && cargo fmt --all -- --check
	@echo "$(GREEN)Code formatting check passed$(RESET)"

## Format Rust code
rust-fmt:
	@echo "$(CYAN)Formatting Rust code...$(RESET)"
	@cd $(CARGO_DIR) && cargo fmt --all

## Clean Rust build artifacts
rust-clean:
	@echo "$(CYAN)Cleaning Rust build artifacts...$(RESET)"
	@cd $(CARGO_DIR) && cargo clean
	@echo "$(GREEN)Rust artifacts cleaned$(RESET)"

## Install Rust binary locally
rust-install: rust-build
	@echo "$(CYAN)Installing $(BINARY_NAME)...$(RESET)"
	@cargo install --path $(CARGO_DIR) --force
	@echo "$(GREEN)$(BINARY_NAME) installed to ~/.cargo/bin/$(RESET)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(RESET)"
	@echo "  1. Verify: $(CYAN)isolde --version$(RESET)"
	@echo "  2. Create a project: $(CYAN)isolde init my-project$(RESET)"

## Update Rust dependencies
rust-update:
	@echo "$(CYAN)Updating Rust dependencies...$(RESET)"
	@cd $(CARGO_DIR) && cargo update
	@echo "$(GREEN)Dependencies updated$(RESET)"

## Show Rust build info
rust-info:
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
	@echo "  make rust-build         - Build release binary"
	@echo "  make rust-dev-build     - Build dev binary (faster)"
	@echo "  make rust-test          - Run tests"
	@echo "  make rust-lint          - Run clippy"
	@echo "  make rust-check         - Check code (format + lint)"
	@echo "  make rust-fmt           - Format code"
	@echo "  make rust-clean         - Clean build artifacts"
	@echo "  make rust-install       - Install to ~/.cargo/bin/"
	@echo "  make rust-info          - Show build information"
