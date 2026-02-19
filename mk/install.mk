# =============================================================================
# Installation Targets
# =============================================================================

# Installation directory
ISOLDE_HOME ?= $(HOME)/.isolde
INSTALL_SCRIPT = scripts/install/install.sh

.PHONY: install uninstall install-info

## Install Isolde to ~/.isolde/ (v2 Rust binary via cargo)
install: rust-install
	@echo "$(GREEN)Isolde v2 installed successfully!$(RESET)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(RESET)"
	@echo "  1. Verify: $(CYAN)isolde --version$(RESET)"
	@echo "  2. Create a project: $(CYAN)isolde init my-project$(RESET)"
	@echo ""
	@echo "$(YELLOW)Note:$(RESET) Binary installed via cargo to ~/.cargo/bin/"
	@echo "      For shell script installation (v1), run: make install-v1"

## Install Isolde v1 (shell script version) to ~/.isolde/
install-v1:
	@echo "$(CYAN)Installing Isolde v1 (shell scripts) to $(ISOLDE_HOME)...$(RESET)"
	@ISOLDE_HOME=$(ISOLDE_HOME) bash $(INSTALL_SCRIPT)
	@echo ""
	@echo "$(GREEN)Isolde v1 installed successfully!$(RESET)"

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
	@echo "Installation directory: $(ISOLDE_HOME)"
	@if [ -d "$(ISOLDE_HOME)" ]; then \
		echo "Status: $(GREEN)Installed$(RESET)"; \
		if [ -f "$(ISOLDE_HOME)/VERSION" ]; then \
			echo "Version: $$(cat $(ISOLDE_HOME)/VERSION)"; \
		fi; \
	else \
		echo "Status: $(YELLOW)Not installed$(RESET)"; \
	fi
	@echo ""
	@echo "$(ARROW) $(GREEN)Installation commands:$(RESET)"
	@echo "  make install          - Install Isolde to ~/.isolde/"
	@echo "  make uninstall        - Remove Isolde installation"
	@echo "  make install-info     - Show installation status"
