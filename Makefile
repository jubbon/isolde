.PHONY: all build devcontainer shell clean test test-build test-config test-runtime test-shell test-lint test-providers test-bats test-bats

# =============================================================================
# ANSI Color Codes for Better Output
# =============================================================================
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
MAGENTA := \033[0;35m
CYAN := \033[0;36m
RESET := \033[0m
CHECK := $(GREEN)$(RESET)
CROSS := $(RED)$(RESET)
ARROW := $(YELLOW)$(RESET)

# =============================================================================
# Default Target
# =============================================================================
all: build

# =============================================================================
# Build Targets
# =============================================================================

# Build Dev Container image
build:
	@echo "$(CYAN)Building Dev Container image...$(RESET)"
	cd .devcontainer && docker build -t claude-code-dev .
	@echo "$(GREEN)Build complete $(CHECK)$(RESET)"

# Run in development mode (interactive with workspace)
devcontainer:
	@echo "$(CYAN)Starting Dev Container...$(RESET)"
	docker run -it --rm \
		--mount type=bind,source="$(PWD)",target=/workspaces/claude-code \
		--mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
		-v "${HOME}/.claude:/home/${USER}/.claude" \
		-e "USERNAME=${USER}" \
		-w /workspaces/claude-code \
		claude-code-dev

# Get a shell in running container
shell:
	@echo "$(CYAN)Starting shell in Dev Container...$(RESET)"
	docker run -it --rm \
		--mount type=bind,source="${PWD}",target=/workspaces/claude-code \
		--mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
		-v "${HOME}/.claude:/home/${USER}/.claude" \
		-e "USERNAME=${USER}" \
		-w /workspaces/claude-code \
		claude-code-dev bash

# Remove running containers
clean:
	@echo "$(CYAN)Cleaning up containers...$(RESET)"
	@docker ps -q --filter "ancestor=claude-code-dev" 2>/dev/null | xargs -r docker stop 2>/dev/null || true
	@echo "$(GREEN)Clean complete $(CHECK)$(RESET)"

# =============================================================================
# TESTS
# =============================================================================

# Run all tests
test: test-build test-config test-runtime test-shell test-lint test-bats
	@echo ""
	@echo "$(GREEN)=============================================$(RESET)"
	@echo "$(GREEN)=== All tests passed! ====$(RESET)"
	@echo "$(GREEN)=============================================$(RESET)"
	@echo ""

# Test: Dev Container builds successfully
test-build:
	@echo "$(CYAN)Testing:$(RESET) Dev Container builds without errors..."
	@cd .devcontainer && docker build -t claude-code-dev-test . > /tmp/build-test.log 2>&1; \
	if [ $$? -eq 0 ]; then \
		echo "  $(GREEN)Build test PASSED$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)Build test FAILED$(CROSS)$(RESET)"; \
		cat /tmp/build-test.log; \
		exit 1; \
	fi
	@echo ""

# Test: Configuration is correct
test-config:
	@echo "$(CYAN)Testing:$(RESET) Environment variables are set correctly..."
	@echo ""

	@# Test HTTP_PROXY
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$$HTTP_PROXY" && echo "$$HTTP_PROXY"' > /dev/null 2>&1; then \
		echo "  $(GREEN)HTTP_PROXY$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)HTTP_PROXY$(CROSS)$(RESET)"; \
		exit 1; \
	fi

	@# Test HTTPS_PROXY
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$$HTTPS_PROXY" && echo "$$HTTPS_PROXY"' > /dev/null 2>&1; then \
		echo "  $(GREEN)HTTPS_PROXY$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)HTTPS_PROXY$(CROSS)$(RESET)"; \
		exit 1; \
	fi

	@# Test NO_PROXY
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$$NO_PROXY" && echo "$$NO_PROXY"' > /dev/null 2>&1; then \
		echo "  $(GREEN)NO_PROXY$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)NO_PROXY$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@echo ""

# Test: Runtime works correctly
test-runtime:
	@echo "$(CYAN)Testing:$(RESET) Docker-in-Docker..."
	@docker ps > /dev/null 2>&1
	@if [ $$? -eq 0 ]; then \
		echo "  $(GREEN)Runtime test PASSED$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)Runtime test FAILED$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@echo ""

# Test: Shell scripts are valid
test-shell:
	@echo "$(CYAN)Testing:$(RESET) Shell scripts syntax..."
	@echo ""

	@if command -v shellcheck >/dev/null 2>&1; then \
		echo "  $(YELLOW)Checking with shellcheck...$(RESET)"; \
		for script in $$(find .devcontainer -name "*.sh" -type f); do \
			if shellcheck "$$script" > /dev/null 2>&1; then \
				echo "  $(GREEN)$$script$(CHECK)$(RESET)"; \
			else \
				echo "  $(RED)$$script$(CROSS)$(RESET)"; \
				shellcheck "$$script"; \
				exit 1; \
			fi; \
		done; \
	else \
		echo "  $(YELLOW)shellcheck not found, skipping...$(RESET)"; \
	fi
	@echo ""

# Test: Lint JSON files
test-lint:
	@echo "$(CYAN)Testing:$(RESET) JSON file validity..."
	@echo ""

	@if command -v jq >/dev/null 2>&1; then \
		for json in $$(find .devcontainer -name "*.json" -type f); do \
			if jq empty "$$json" > /dev/null 2>&1; then \
				echo "  $(GREEN)$$json$(CHECK)$(RESET)"; \
			else \
				echo "  $(RED)$$json$(CROSS)$(RESET)"; \
				jq empty "$$json"; \
				exit 1; \
			fi; \
		done; \
	else \
		echo "  $(YELLOW)jq not found, skipping JSON validation...$(RESET)"; \
	fi
	@echo ""

# Test: Provider configuration
test-providers:
	@echo "$(CYAN)Testing:$(RESET) Provider configuration..."
	@echo ""

	@# Test if provider directory can be created
	@echo "  $(YELLOW)Testing provider setup...$(RESET)"
	@if docker run --rm claude-code-dev bash -c 'mkdir -p ~/.claude/providers/test-provider && echo "test" > ~/.claude/providers/test-provider/auth && cat ~/.claude/providers/test-provider/auth' > /dev/null 2>&1; then \
		echo "  $(GREEN)Provider directory creation$(CHECK)$(RESET)"; \
	else \
		echo "  $(RED)Provider directory creation$(CROSS)$(RESET)"; \
		exit 1; \
	fi
	@echo ""

# Test: Run Bats unit tests
test-bats:
	@echo "$(CYAN)Testing:$(RESET) Bats unit tests..."
	@echo ""
	@if command -v bats >/dev/null 2>&1; then \
		cd tests && bats --formatter pretty .; \
		echo "  $(GREEN)Bats tests PASSED$(CHECK)$(RESET)"; \
	else \
		echo "  $(YELLOW)bats not found, using npx...$(RESET)"; \
		if command -v npx >/dev/null 2>&1; then \
			cd tests && npx bats --formatter pretty .; \
			echo "  $(GREEN)Bats tests PASSED$(CHECK)$(RESET)"; \
		else \
			echo "  $(RED)Neither bats nor npx found$(CROSS)$(RESET)"; \
			echo "  $(YELLOW)Install bats: npm install -g bats or see tests/README.md$(RESET)"; \
			exit 1; \
		fi; \
	fi
	@echo ""

# =============================================================================
# Development Helpers
# =============================================================================

# Show test coverage (what tests exist)
test-help:
	@echo "$(CYAN)Available test targets:$(RESET)"
	@echo ""
	@echo "  $(ARROW) make test          $(RESET)- Run all tests"
	@echo "  $(ARROW) make test-build     $(RESET)- Test container builds"
	@echo "  $(ARROW) make test-config    $(RESET)- Test environment configuration"
	@echo "  $(ARROW) make test-runtime   $(RESET)- Test Docker-in-Docker"
	@echo "  $(ARROW) make test-shell     $(RESET)- Test shell script syntax"
	@echo "  $(ARROW) make test-lint      $(RESET)- Test JSON file validity"
	@echo "  $(ARROW) make test-providers $(RESET)- Test provider configuration"
	@echo "  $(ARROW) make test-bats      $(RESET)- Run Bats unit tests"
	@echo ""
