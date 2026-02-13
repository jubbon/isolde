.PHONY: all build devcontainer shell clean test

# Default target
all: build

# Build the Dev Container image
build:
	@printf "Building Dev Container image..."
	cd .devcontainer && docker build -t claude-code-dev .
	@printf "Build complete ✓"

# Run in development mode (interactive with workspace)
devcontainer:
	@printf "Starting Dev Container..."
	docker run -it --rm \
		--mount type=bind,source="$(PWD)",target=/workspaces/claude-code \
		--mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
		-v "${HOME}/.claude:/home/${USER}/.claude" \
		-e "USERNAME=${USER}" \
		-w /workspaces/claude-code \
		claude-code-dev

# Get a shell in running container
shell:
	@printf "Starting shell in Dev Container..."
	docker run -it --rm \
		--mount type=bind,source="${PWD}",target=/workspaces/claude-code \
		--mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
		-v "${HOME}/.claude:/home/${USER}/.claude" \
		-w /workspaces/claude-code \
		claude-code-dev bash

# Remove running containers
clean:
	@printf "Cleaning up containers..."
	docker ps -q --filter "ancestor=claude-code-dev" | xargs -r docker stop
	@printf "Clean complete ✓"

# =============================================================================
# TESTS
# =============================================================================

.PHONY: test test-build test-config test-runtime

# Run all tests
test: test-build test-config test-runtime
	@printf ""
	@printf "=== ════════════════════════════════════════ ==="
	@printf "===          All tests passed!                     ==="
	@printf "=== ════════════════════════════════════════ ==="
	@printf ""

# Test: Dev Container builds successfully
test-build:
	@printf "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@printf "TEST: Build"
	@printf "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@printf "Testing: Dev Container builds without errors..."
	@printf ""
	cd .devcontainer && \
		docker build -t claude-code-dev . > /tmp/build-test.log 2>&1
	cd .devcontainer && docker build -t claude-code-dev-test . > /tmp/build-test.log 2>&1
	@if [ $$? -eq 0 ]; then \
		printf "✓ Build test PASSED"; \
		printf ""; \
	else \
		printf "✗ Build test FAILED"; \
		cat /tmp/build-test.log; \
		printf ""; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
		exit 1; \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
	fi
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTP_PROXY" \&\& printf "$HTTP_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
		printf "✓ HTTP_PROXY is set"; \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
	else \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
		printf "✗ HTTP_PROXY is NOT set"; \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
		cat /tmp/test-config.log; \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
		rm /tmp/test-config.log; \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
		exit 1; \
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
	fi
	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& echo "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set\n"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& echo "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set\n"; \
	else \
		printf "✗ NO_PROXY is NOT set\n"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test HTTPS_PROXY
	@printf "Checking HTTPS_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$HTTPS_PROXY" \&\& printf "$HTTPS_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ HTTPS_PROXY is set"; \
	else \
		printf "✗ HTTPS_PROXY is NOT set"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi

	# Test NO_PROXY
	@printf "Checking NO_PROXY..."
	@if docker run --rm -e HTTP_PROXY -e HTTPS_PROXY -e NO_PROXY claude-code-dev bash -c 'test -n "$NO_PROXY" \&\& printf "$NO_PROXY"' \> /tmp/test-config.log 2\>\&1; then \
		printf "✓ NO_PROXY is set"; \
	else \
		printf "✗ NO_PROXY is NOT set"; \
		cat /tmp/test-config.log; \
		rm /tmp/test-config.log; \
		exit 1; \
	fi
	@printf ""

# Test: Runtime works correctly
test-runtime:
	@printf "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@printf "TEST: Runtime"
	@printf "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@printf "Testing: Claude Code CLI is functional..."
	@printf ""

	# Test Docker-in-Docker
	@printf "Checking Docker-in-Docker..."
	@if docker run --rm \
		--mount type=bind,source=/var/run/docker.sock,target=/var/run/docker.sock \
		claude-code-dev \
		docker ps > /tmp/test-runtime.log 2>&1; then \
		printf "✓ Docker-in-Docker works"; \
		rm /tmp/test-runtime.log; \
	else \
		printf "✗ Docker-in-Docker FAILED"; \
		cat /tmp/test-runtime.log; \
		rm /tmp/test-runtime.log; \
		exit 1; \
	fi
	@printf ""

	# Clean up test image
	@printf "Cleaning up test image..."
	docker rmi claude-code-dev-test > /dev/null 2>&1 || true

# Clean up test image
	@printf "Cleaning up test image..."
	docker rmi claude-code-dev-test > /dev/null 2>&1 || true
