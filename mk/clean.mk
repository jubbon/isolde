# =============================================================================
# Clean Targets - Cleanup Operations
# =============================================================================
.PHONY: clean clean-containers clean-images clean-all clean-e2e

# Remove running containers
clean: clean-containers

clean-containers:
	@echo "$(CYAN)Cleaning up containers...$(RESET)"
	@docker ps -q --filter "ancestor=$(IMAGE_NAME)" 2>/dev/null | xargs -r docker stop 2>/dev/null || true
	@docker ps -q --filter "ancestor=$(TEST_IMAGE_PREFIX)" 2>/dev/null | xargs -r docker stop 2>/dev/null || true
	@echo "$(GREEN)Containers cleaned $(CHECK)$(RESET)"

# Remove built images
clean-images:
	@echo "$(CYAN)Removing built images...$(RESET)"
	@docker images -q $(IMAGE_NAME) 2>/dev/null | xargs -r docker rmi -f 2>/dev/null || true
	@docker images -q $(TEST_IMAGE_PREFIX) 2>/dev/null | xargs -r docker rmi -f 2>/dev/null || true
	@echo "$(GREEN)Images removed $(CHECK)$(RESET)"

# Full cleanup
clean-all: clean-containers clean-images clean-e2e
	@echo "$(CYAN)Running Docker system prune...$(RESET)"
	@docker system prune -f --volumes > /dev/null 2>&1 || true
	@echo "$(GREEN)Full cleanup complete $(CHECK)$(RESET)"

# Remove E2E test artifacts
clean-e2e:
	@echo "$(CYAN)Cleaning E2E test artifacts...$(RESET)"
	@rm -rf /tmp/e2e-* 2>/dev/null || true
	@rm -rf $(E2E_DIR)/reports/ 2>/dev/null || true
	@echo "$(GREEN)E2E artifacts cleaned $(CHECK)$(RESET)"
