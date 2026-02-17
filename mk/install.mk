# =============================================================================
# Installation Targets
# =============================================================================

# Installation directory
ISOLDE_HOME ?= $(HOME)/.isolde
INSTALL_SCRIPT = scripts/install/install.sh

.PHONY: install uninstall install-info

## Install Isolde to ~/.isolde/
install:
	@echo "$(CYAN)Installing Isolde to $(ISOLDE_HOME)...$(RESET)"
	@ISOLDE_HOME=$(ISOLDE_HOME) bash $(INSTALL_SCRIPT)
	@echo ""
	@echo "$(GREEN)Isolde installed successfully!$(RESET)"
	@echo ""
	@echo "$(YELLOW)Next steps:$(RESET)"
	@echo "  1. Reload your shell: $(CYAN)source ~/.bashrc$(RESET) (or restart terminal)"
	@echo "  2. Verify: $(CYAN)isolde --version$(RESET)"
	@echo "  3. Create a project: $(CYAN)isolde my-project --template=python$(RESET)"

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
