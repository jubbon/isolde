# =============================================================================
# Build Targets
# =============================================================================
.PHONY: build build-provider rebuild docker-build

# Default build target
build: docker-build
	@echo "$(GREEN)Build complete $(CHECK)$(RESET)"

# Main docker build
docker-build:
	@echo "$(CYAN)Building Dev Container image...$(RESET)"
	docker build -t $(IMAGE_NAME) $(DOCKER_BUILD_CONTEXT)
	@echo "$(GREEN)Image built: $(IMAGE_NAME)$(RESET)"

# Build with specific provider
build-provider:
	@if [ -z "$(PROVIDER)" ]; then \
		echo "$(RED)Error: PROVIDER variable not set$(RESET)"; \
		echo "$(YELLOW)Usage: make build-provider PROVIDER=z.ai$(RESET)"; \
		exit 1; \
	fi
	@echo "$(CYAN)Building Dev Container image with provider: $(PROVIDER)...$(RESET)"
	docker build \
		--build-arg PROVIDER=$(PROVIDER) \
		-t $(IMAGE_NAME)-$(PROVIDER) \
		$(DOCKER_BUILD_CONTEXT)
	@echo "$(GREEN)Image built: $(IMAGE_NAME)-$(PROVIDER)$(RESET)"

# Force rebuild without cache
rebuild:
	@echo "$(CYAN)Rebuilding Dev Container image (no cache)...$(RESET)"
	docker build --no-cache -t $(IMAGE_NAME) $(DOCKER_BUILD_CONTEXT)
	@echo "$(GREEN)Rebuild complete $(CHECK)$(RESET)"
