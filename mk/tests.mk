# =============================================================================
# Test Targets - All Test Categories
# =============================================================================
.PHONY: test test-build test-config test-runtime test-providers
.PHONY: test-e2e test-e2e-all test-e2e-cli
.PHONY: test-e2e-fast test-e2e-medium test-e2e-full
.PHONY: test-e2e-layer-1 test-e2e-layer-2 test-e2e-layer-3
.PHONY: test-e2e-workflow test-e2e-clean test-e2e-help
.PHONY: test-e2e-container test-e2e-error test-e2e-validation test-e2e-doctor

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

# =============================================================================
# E2E Test Speed Targets (Fast, Medium, Full)
# =============================================================================

# E2E Tests - Fast tests (no rebuilds, basic scenarios)
test-e2e-fast:
	@echo "$(CYAN)Running E2E fast tests (no rebuilds)...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --fast $(if $(VERBOSE),--verbose,)

# E2E Tests - Fast + Medium tests
test-e2e-medium:
	@echo "$(CYAN)Running E2E medium tests (fast + medium)...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --tags "fast or medium" $(if $(VERBOSE),--verbose,)

# E2E Tests - Complete test suite
test-e2e-full:
	@echo "$(CYAN)Running E2E full test suite...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --all $(if $(VERBOSE),--verbose,)

# =============================================================================
# E2E Test Layer Targets (Three-Layer Testing)
# =============================================================================

# E2E Tests - Layer 1: Build template images only
test-e2e-layer-1:
	@echo "$(CYAN)Running E2E Layer 1: Build template images...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --tags=layer-1 $(if $(VERBOSE),--verbose,)

# E2E Tests - Layer 2: Test scenarios only
test-e2e-layer-2:
	@echo "$(CYAN)Running E2E Layer 2: Test scenarios...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --tags=layer-2 $(if $(VERBOSE),--verbose,)

# E2E Tests - Layer 3: Verification tests only
test-e2e-layer-3:
	@echo "$(CYAN)Running E2E Layer 3: Verification tests...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --tags=layer-3 $(if $(VERBOSE),--verbose,)

# E2E Tests - Layer 1 + Layer 3 (Build + Verify workflow)
test-e2e-workflow:
	@echo "$(CYAN)Running E2E workflow (Layer 1 + Layer 3)...$(RESET)"
	@cd $(E2E_DIR) && ./run-tests.sh --tags="layer-1 or layer-3" $(if $(VERBOSE),--verbose,)

# =============================================================================
# E2E Test Feature-Specific Targets (New)
# =============================================================================

# E2E Tests - Container lifecycle commands (build, run, exec, stop, ps, logs)
test-e2e-container:
	@echo "$(CYAN)Running E2E container lifecycle tests...$(RESET)"
	@cd $(E2E_DIR) && behave --tags=@container --format=progress $(if $(VERBOSE),--verbose,)

# E2E Tests - Error scenarios
test-e2e-error:
	@echo "$(CYAN)Running E2E error scenario tests...$(RESET)"
	@cd $(E2E_DIR) && behave --tags=@error --format=progress $(if $(VERBOSE),--verbose,)

# E2E Tests - Validation and diff commands
test-e2e-validation:
	@echo "$(CYAN)Running E2E validation command tests...$(RESET)"
	@cd $(E2E_DIR) && behave --tags=@validation --format=progress $(if $(VERBOSE),--verbose,)

# E2E Tests - Doctor command
test-e2e-doctor:
	@echo "$(CYAN)Running E2E doctor command tests...$(RESET)"
	@cd $(E2E_DIR) && behave --tags=@doctor --format=progress $(if $(VERBOSE),--verbose,)

# E2E Tests - Diff command
test-e2e-diff:
	@echo "$(CYAN)Running E2E diff command tests...$(RESET)"
	@cd $(E2E_DIR) && behave --tags=@diff --format=progress $(if $(VERBOSE),--verbose,)

# =============================================================================
# E2E Test Utility Targets
# =============================================================================

# E2E Tests - Clean test artifacts
test-e2e-clean:
	@echo "$(CYAN)Cleaning E2E test artifacts...$(RESET)"
	@rm -rf $(E2E_DIR)/reports/*.html $(E2E_DIR)/reports/*.json
	@rm -rf /tmp/e2e-*
	@docker images | grep e2e- | awk '{print $3}' | xargs -r docker rmi -f 2>/dev/null || true
	@echo "  $(GREEN)E2E artifacts cleaned$(CHECK)$(RESET)"

# E2E Tests - List E2E test options
test-e2e-help:
	@echo "$(CYAN)E2E Test Targets:$(RESET)"
	@echo ""
	@echo "$(ARROW) $(GREEN)Speed-based targets:$(RESET)"
	@echo "  make test-e2e-fast    - Fast tests (no rebuilds)"
	@echo "  make test-e2e-medium  - Fast + medium tests"
	@echo "  make test-e2e-full    - Complete test suite"
	@echo ""
	@echo "$(ARROW) $(GREEN)Layer-based targets:$(RESET)"
	@echo "  make test-e2e-layer-1   - Build template images only"
	@echo "  make test-e2e-layer-2   - Test scenarios only"
	@echo "  make test-e2e-layer-3   - Verification tests only"
	@echo "  make test-e2e-workflow  - Layer 1 + Layer 3 (Build + Verify)"
	@echo ""
	@echo "$(ARROW) $(GREEN)Command-specific targets:$(RESET)"
	@echo "  make test-e2e-container - Container lifecycle (build, run, exec, stop, ps, logs)"
	@echo "  make test-e2e-error     - Error scenario tests"
	@echo "  make test-e2e-validation - Validation and diff command tests"
	@echo "  make test-e2e-doctor    - Doctor command tests"
	@echo "  make test-e2e-diff      - Diff command tests"
	@echo ""
	@echo "$(ARROW) $(GREEN)Category targets:$(RESET)"
	@echo "  make test-e2e       - Docker-based tests (default)"
	@echo "  make test-e2e-all   - All tests including CLI"
	@echo "  make test-e2e-cli   - CLI tests only"
	@echo ""
	@echo "$(ARROW) $(GREEN)Utility targets:$(RESET)"
	@echo "  make test-e2e-clean - Clean test artifacts"
	@echo "  make test-e2e-help  - Show this help message"
	@echo ""
	@echo "$(ARROW) $(GREEN)Variables:$(RESET)"
	@echo "  VERBOSE=1           - Verbose test output"
	@echo "  SCENARIO='name'     - Run specific scenario"
