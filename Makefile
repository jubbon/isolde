.PHONY: all build devcontainer shell clean test

# Default target
all: build

# Build the Dev Container image
build:
	@echo "Building Dev Container image..."
	cd .devcontainer && docker build -t claude-code-dev .
	@echo "Build complete ✓"

# Run in development mode (interactive with workspace)
devcontainer:
	@echo "Starting Dev Container..."
	docker run -it --rm \
		--mount type=bind,source="$(PWD)",target=/workspaces/claude-code \
		--mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
		-v "${HOME}/.claude:/home/${USER}/.claude" \
		-e "USERNAME=${USER}" \
		-w /workspaces/claude-code \
		claude-code-dev

# Get a shell in running container
shell:
	@echo "Starting shell in Dev Container..."
	docker run -it --rm \
		--mount type=bind,source="${PWD}",target=/workspaces/claude-code \
		--mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
		-v "${HOME}/.claude:/home/${USER}/.claude" \
		-e "USERNAME=${USER}" \
		-w /workspaces/claude-code \
		claude-code-dev bash

# Remove running containers
clean:
	@echo "Cleaning up containers..."
	docker ps -q --filter "ancestor=claude-code-dev" | xargs -r docker stop
	@echo "Clean complete ✓"

# =============================================================================
# TESTS
# =============================================================================

.PHONY: test test-build test-config test-runtime

# Run all tests
test: test-build test-config test-runtime
	@echo ""
	@echo "=== All tests passed! ===="
	@echo ""

# Test: Dev Container builds successfully
test-build:
	@echo "Testing: Dev Container builds without errors..."
	cd .devcontainer && docker build -t claude-code-dev-test . > /tmp/build-test.log 2>&1
	@if [ $$? -eq 0 ]; then \
		echo "Build test PASSED";
	else \
		echo "Build test FAILED";
		cat /tmp/build-test.log;
	fi
	@echo ""

# Test: Configuration is correct
test-config:
	@echo "Testing: Environment variables are set correctly..."
	@echo ""

	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTP_PROXY" && echo "$HTTP_PROXY"'; then \
		echo "Config test PASSED"; \
	else \
		echo "Config test FAILED"; \
	fi

	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" && echo "$HTTPS_PROXY"'; then \
		echo "Config test PASSED"; \
	else \
		echo "Config test FAILED"; \
	fi

	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" && echo "$NO_PROXY"'; then \
		echo "Config test PASSED"; \
	else \
		echo "Config test FAILED"; \
	fi

# Test: Runtime works correctly
test-runtime:
	@echo "Testing: Docker-in-Docker..."
	@if docker run --rm --mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock claude-code-dev docker ps; then \
		echo "Runtime test PASSED"; \
	else \
		echo "Runtime test FAILED"; \
	fi
