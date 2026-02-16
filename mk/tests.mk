# =============================================================================
# Test Targets - All Test Categories
# =============================================================================
.PHONY: test test-build test-config test-runtime test-providers
.PHONY: test-e2e test-e2e-all test-e2e-cli

# Run all tests (CI parity)
test: lint test-build test-config test-runtime test-providers test-e2e
	@echo ""
	@echo "$(GREEN)=============================================$(RESET)"
	@echo "$(GREEN)=== All tests passed! ====$(RESET)"
	@echo "$(GREEN)=============================================$(RESET)"
	@echo ""

# Test: Container builds successfully
test-build:
	@echo "$(CYAN)Testing:$(RESET) Dev Container builds without errors..."
	@docker build -t $(TEST_IMAGE_PREFIX) $(DOCKER_BUILD_CONTEXT) > /tmp/build-test.log 2>&1; \
	if [ $$? -eq 0 ]; then \
		echo "  $(GREEN)Build test PASSED$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)Build test FAILED$(CROSS)$(RESET)"; \
		cat /tmp/build-test.log; \
		exit 1; \
	fi
	@echo ""

# Test: Environment configuration (proxy variables)
test-config:
	@echo "$(CYAN)Testing:$(RESET) Environment variables are set correctly..."
	@echo ""
	@# Test HTTP_PROXY
	@if docker run --rm -e HTTP_PROXY=value -e HTTPS_PROXY=value -e NO_PROXY=value $(TEST_IMAGE_PREFIX) bash -c 'test -n "$$HTTP_PROXY" && echo "$$HTTP_PROXY"' > /dev/null 2>&1; then \
		echo "  $(GREEN)HTTP_PROXY$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)HTTP_PROXY$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@# Test HTTPS_PROXY
	@if docker run --rm -e HTTP_PROXY=value -e HTTPS_PROXY=value -e NO_PROXY=value $(TEST_IMAGE_PREFIX) bash -c 'test -n "$$HTTPS_PROXY" && echo "$$HTTPS_PROXY"' > /dev/null 2>&1; then \
		echo "  $(GREEN)HTTPS_PROXY$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)HTTPS_PROXY$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@# Test NO_PROXY
	@if docker run --rm -e HTTP_PROXY=value -e HTTPS_PROXY=value -e NO_PROXY=value $(TEST_IMAGE_PREFIX) bash -c 'test -n "$$NO_PROXY" && echo "$$NO_PROXY"' > /dev/null 2>&1; then \
		echo "  $(GREEN)NO_PROXY$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)NO_PROXY$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@echo ""

# Test: Docker-in-Docker functionality
test-runtime:
	@echo "$(CYAN)Testing:$(RESET) Docker-in-Docker..."
	@if docker ps > /dev/null 2>&1; then \
		echo "  $(GREEN)Runtime test PASSED$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)Runtime test FAILED$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@echo ""

# Test: Provider configuration
test-providers:
	@echo "$(CYAN)Testing:$(RESET) Provider configuration..."
	@echo ""
	@# Test if provider directory can be created
	@echo "  $(YELLOW)Testing provider setup...$(RESET)"
	@if docker run --rm $(TEST_IMAGE_PREFIX) bash -c 'mkdir -p ~/.claude/providers/test-provider && echo "test" > ~/.claude/providers/test-provider/auth && cat ~/.claude/providers/test-provider/auth' > /dev/null 2>&1; then \
		echo "  $(GREEN)Provider directory creation$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)Provider directory creation$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@echo ""

# E2E Tests - Docker-based only (default)
test-e2e:
	@echo "$(CYAN)Running E2E tests (Docker-based)...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh $(if $(VERBOSE),--verbose,) $(if $(SCENARIO),--name "$(SCENARIO)",)

# E2E Tests - All tests including CLI
test-e2e-all:
	@echo "$(CYAN)Running E2E tests (all)...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --all $(if $(VERBOSE),--verbose,) $(if $(SCENARIO),--name "$(SCENARIO)",)

# E2E Tests - CLI tests only
test-e2e-cli:
	@echo "$(CYAN)Running E2E tests (CLI only)...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --cli $(if $(VERBOSE),--verbose,) $(if $(SCENARIO),--name "$(SCENARIO)",)
