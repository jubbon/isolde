# =============================================================================
# Installation Targets
# =============================================================================

# Installation directory (legacy, for uninstall target)
ISOLDE_HOME ?= $(HOME)/.isolde

.PHONY: install uninstall install-info

## Install Isolde to ~/.local/bin/
install: install-local

## Install Isolde to ~/.local/bin/
install-local: rust-build install-features
	@echo "$(CYAN)Installing $(BINARY_NAME) to ~/.local/bin...$(RESET)"
	@mkdir -p $(HOME)/.local/bin
	@cp -f $(BINARY_PATH) $(HOME)/.local/bin/$(BINARY_NAME)
	@chmod +x $(HOME)/.local/bin/$(BINARY_NAME)
	@echo "$(GREEN)$(BINARY_NAME) installed to ~/.local/bin/$(RESET)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(RESET)"
	@echo "  1. Verify: $(CYAN)isolde --version$(RESET)"
	@echo "  2. Create a project: $(CYAN)isolde init my-project$(RESET)"
	@if ! echo $$PATH | grep -q "$$HOME/.local/bin"; then \
		echo ""; \
		echo "$(YELLOW)Note:$(RESET) ~/.local/bin is not in your PATH."; \
		echo "Add this to your shell config (~/.bashrc or ~/.zshrc):"; \
		echo "  $(CYAN)export PATH=\"$$HOME/.local/bin:$$PATH\"$(RESET)"; \
	fi

## Install core features to ~/.local/share/isolde/
install-features:
	@echo "$(CYAN)Installing core features to ~/.local/share/isolde/...$(RESET)"
	@mkdir -p $(HOME)/.local/share/isolde/core/features
	@if [ -d "core/features" ]; then \
		for feature in core/features/*; do \
			if [ -d "$$feature" ]; then \
				feature_name=$$(basename "$$feature"); \
				echo "  Copying $$feature_name..."; \
				cp -r "$$feature" $(HOME)/.local/share/isolde/core/features/; \
			fi; \
		done; \
		echo "$(GREEN)Core features installed.$(RESET)"; \
	else \
		echo "$(YELLOW)Warning: core/features directory not found.$(RESET)"; \
		echo "  Features will need to be copied manually."; \
	fi

## Install Isolde via cargo install (to ~/.cargo/bin/)
install-cargo:
	@echo "$(CYAN)Installing $(BINARY_NAME) via cargo install...$(RESET)"
	@cargo install --path $(CARGO_DIR) --force
	@echo "$(GREEN)$(BINARY_NAME) installed to ~/.cargo/bin/$(RESET)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(RESET)"
	@echo "  1. Verify: $(CYAN)isolde --version$(RESET)"
	@echo "  2. Create a project: $(CYAN)isolde init my-project$(RESET)"

## Uninstall Isolde
uninstall:
	@echo "$(YELLOW)Removing Isolde...$(RESET)"
	@rm -f $(HOME)/.local/bin/$(BINARY_NAME)
	@rm -rf $(HOME)/.local/share/isolde
	@rm -rf $(ISOLDE_HOME)
	@echo "$(GREEN)Isolde uninstalled.$(RESET)"
	@echo ""
	@echo "$(YELLOW)Removed:$(RESET)"
	@echo "  - ~/.local/bin/$(BINARY_NAME)"
	@echo "  - ~/.local/share/isolde/"
	@echo "  - $(ISOLDE_HOME)"

## Show installation information
install-info:
	@echo "$(CYAN)Isolde Installation Information$(RESET)"
	@echo ""
	@if [ -f "$(HOME)/.local/bin/$(BINARY_NAME)" ]; then \
		echo "Status: $(GREEN)Installed$(RESET)"; \
		echo "Location: ~/.local/bin/$(BINARY_NAME)"; \
		echo "Version: $$($(HOME)/.local/bin/$(BINARY_NAME) --version 2>/dev/null)"; \
	else \
		echo "Status: $(YELLOW)Not installed$(RESET)"; \
	fi
	@echo ""
	@echo "$(ARROW) $(GREEN)Installation commands:$(RESET)"
	@echo "  make install          - Install Isolde to ~/.local/bin/"
	@echo "  make install-cargo    - Install via cargo to ~/.cargo/bin/"
	@echo "  make uninstall        - Remove Isolde installation"
	@echo "  make install-info     - Show installation status"
