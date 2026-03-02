# =============================================================================
# Installation Targets
# =============================================================================

# Installation directory (legacy, for uninstall target)
ISOLDE_HOME ?= $(HOME)/.isolde

.PHONY: install uninstall install-info

## Install Isolde to ~/.local/bin/
install: install-local

## Install Isolde to ~/.local/bin/
install-local: rust-build
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
	@echo "$(YELLOW)Removing Isolde from $(ISOLDE_HOME)...$(RESET)"
	@rm -rf $(ISOLDE_HOME)
	@echo "$(GREEN)Isolde uninstalled.$(RESET)"
	@echo ""
	@echo "$(YELLOW)Note:$(RESET) Remove the following lines from your shell config:"
	@echo "  export ISOLDE_HOME=\"$(ISOLDE_HOME)\""
	@echo "  export PATH=\"\$$PATH:\$$ISOLDE_HOME\""

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
