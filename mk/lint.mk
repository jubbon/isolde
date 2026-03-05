# =============================================================================
# Lint Targets - Fast Static Checks
# =============================================================================
.PHONY: lint lint-json

# Run all lint checks
lint: lint-json build/lint
	@echo ""
	@echo "$(GREEN)=== All lint checks passed ===$(RESET)"
	@echo ""

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
