# =============================================================================
# Lint Targets - Fast Static Checks
# =============================================================================
.PHONY: lint lint-shell lint-json lint-bats

# Run all lint checks
lint: lint-shell lint-json lint-bats
	@echo ""
	@echo "$(GREEN)=== All lint checks passed ===$(RESET)"
	@echo ""

# Check shell scripts with shellcheck
lint-shell:
	@echo "$(CYAN)Checking shell scripts with shellcheck...$(RESET)"
	@if [ -z "$(call command_exists,shellcheck)" ]; then \
		if [ $(CI_MODE) -eq 1 ]; then \
			echo "$(RED)Error: shellcheck not found in CI$(RESET)"; \
			exit 1; \
		else \
			echo "$(YELLOW)shellcheck not found, skipping...$(RESET)"; \
			echo "$(YELLOW)Install: sudo apt-get install shellcheck$(RESET)"; \
		fi \
	else \
		errors=0; \
		for script in $$(find $(DEVCONTAINER_DIR) $(SCRIPTS_DIR) -name "*.sh" -type f 2>/dev/null); do \
			if shellcheck "$$script" > /dev/null 2>&1; then \
				echo "  $(GREEN)$$script$(CHECK)$(RESET)"; \
			else \
				echo "  $(RED)$$script$(CROSS)$(RESET)"; \
				shellcheck "$$script"; \
				errors=$$((errors + 1)); \
			fi \
		done; \
		if [ $$errors -gt 0 ]; then \
			echo "$(RED)$$errors script(s) failed linting$(RESET)"; \
			exit 1; \
		fi \
	fi

# Validate JSON files with jq
lint-json:
	@echo "$(CYAN)Validating JSON files with jq...$(RESET)"
	@if [ -z "$(call command_exists,jq)" ]; then \
		if [ $(CI_MODE) -eq 1 ]; then \
			echo "$(RED)Error: jq not found in CI$(RESET)"; \
			exit 1; \
		else \
			echo "$(YELLOW)jq not found, skipping...$(RESET)"; \
			echo "$(YELLOW)Install: sudo apt-get install jq$(RESET)"; \
		fi \
	else \
		errors=0; \
		for json in $$(find . -name "*.json" -type f 2>/dev/null | grep -v -E '(node_modules|templates/.*\.devcontainer|\.omc|\.claude|\.devcontainer/devcontainer\.json|test-fix-verification)'); do \
			if jq empty "$$json" > /dev/null 2>&1; then \
				echo "  $(GREEN)$$json$(CHECK)$(RESET)"; \
			else \
				echo "  $(RED)$$json$(CROSS)$(RESET)"; \
				jq empty "$$json"; \
				errors=$$((errors + 1)); \
			fi \
		done; \
		if [ $$errors -gt 0 ]; then \
			echo "$(RED)$$errors file(s) failed validation$(RESET)"; \
			exit 1; \
		fi \
	fi

# Run Bats unit tests
lint-bats:
	@echo "$(CYAN)Running Bats unit tests...$(RESET)"
	@if [ -z "$(call command_exists,bats)" ] && [ -z "$(call command_exists,npx)" ]; then \
		if [ $(CI_MODE) -eq 1 ]; then \
			echo "$(RED)Error: Neither bats nor npx found in CI$(RESET)"; \
			exit 1; \
		else \
			echo "$(YELLOW)Neither bats nor npx found, skipping...$(RESET)"; \
			echo "$(YELLOW)Install: npm install -g bats$(RESET)"; \
		fi \
	else \
		if [ -n "$(call command_exists,bats)" ]; then \
			cd $(DEVCONTAINER_DIR)/tests && bats --formatter pretty .; \
		else \
			echo "$(YELLOW)Using npx bats...$(RESET)"; \
			cd $(DEVCONTAINER_DIR)/tests && npx bats --formatter pretty .; \
		fi \
	fi
