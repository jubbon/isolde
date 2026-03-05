# =============================================================================
# Release targets
# =============================================================================

.PHONY: release release/check release/tag

VERSION := $(shell cat VERSION 2>/dev/null || echo "unknown")

## Check release prerequisites
release/check:
	@echo "=== Release preflight for v$(VERSION) ==="
	@if [ "$(VERSION)" = "unknown" ]; then \
		echo "ERROR: VERSION file not found"; exit 1; \
	fi
	@if ! grep -q '^\#\# \[$(VERSION)\]' CHANGELOG.md 2>/dev/null; then \
		echo "ERROR: CHANGELOG.md has no entry for [$(VERSION)]"; exit 1; \
	fi
	@if [ -n "$$(git status --porcelain)" ]; then \
		echo "ERROR: Working tree is not clean. Commit or stash changes first."; exit 1; \
	fi
	@if git tag -l "v$(VERSION)" | grep -q "v$(VERSION)"; then \
		echo "ERROR: Tag v$(VERSION) already exists"; exit 1; \
	fi
	@echo "All checks passed."

## Create an annotated release tag
release/tag:
	git tag -a "v$(VERSION)" -m "Release v$(VERSION)"
	@echo "Created tag v$(VERSION)"

## Full release: check, test, tag
release: release/check test release/tag
	@echo ""
	@echo "=== Release v$(VERSION) complete ==="
	@echo "To publish, run:"
	@echo "  git push origin v$(VERSION)"
