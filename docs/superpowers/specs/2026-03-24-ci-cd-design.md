# CI/CD Pipeline Design

**Date:** 2026-03-24
**Status:** Approved

## Overview

Replace the existing `.github/workflows/test.yml` (Docker-only tests) with two workflows:
- `ci.yml` — Rust CI on every push/PR
- `release.yml` — Build release binary and publish GitHub Release on tag or manual trigger

Target platform: Linux x86_64 only.

## Workflow 1: `ci.yml`

### Triggers

```yaml
on:
  push:
    branches: [dev, main]
  pull_request:
    branches: [main, dev]
  workflow_dispatch:

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true
```

### Shared Configuration

```yaml
env:
  CARGO_TERM_COLOR: always
```

### Rust Toolchain Setup (shared across jobs)

All Rust jobs use:
```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    toolchain: "1.75.0"
    components: rustfmt, clippy
- uses: Swatinem/rust-cache@v2
  with:
    shared-key: "isolde"
```

### Jobs

#### `lint`
- **Runs on:** `ubuntu-latest`
- **Timeout:** 10 minutes
- **Steps:**
  1. `actions/checkout@v4`
  2. Install Rust 1.75 with `rustfmt` and `clippy` components
  3. `cargo fmt --check`
  4. `cargo clippy --all-targets -- -D warnings`
  5. Install shellcheck, lint shell scripts in `core/features/`
  6. Install jq, validate JSON files in `core/features/` and `templates/`

Note: The old `test.yml` linted `.devcontainer/` — the new workflow lints `core/features/` and `templates/` since `.devcontainer/` only exists in generated projects, not in this repo.
Note: Bats tests from the old `test.yml` are dropped — they reference `.devcontainer/tests/` which does not exist in the repo.

#### `test`
- **Runs on:** `ubuntu-latest`
- **Timeout:** 15 minutes
- **Steps:**
  1. `actions/checkout@v4`
  2. Install Rust 1.75 + cache
  3. `cargo test`

#### `build`
- **Runs on:** `ubuntu-latest`
- **Timeout:** 15 minutes
- **Steps:**
  1. `actions/checkout@v4`
  2. Install Rust 1.75 + cache
  3. `cargo build --release`
  4. Upload `target/release/isolde` via `actions/upload-artifact@v4` (retention: 7 days)

Note: Docker tests from the old `test.yml` are NOT migrated. They reference `.devcontainer/` at the repo root which does not exist — those tests were designed for a generated project, not the isolde source repo. Proper integration tests (generate a project, then test it) should be designed separately.

## Workflow 2: `release.yml`

### Triggers

```yaml
on:
  push:
    tags: ['v*']
  workflow_dispatch:
    inputs:
      tag:
        description: 'Tag to release (e.g. v0.3.0)'
        required: true
        type: string

permissions:
  contents: write
```

### Version Resolution

All jobs that need the tag/version use this pattern:
```yaml
- name: Determine version
  run: |
    if [ "${{ github.event_name }}" = "workflow_dispatch" ]; then
      echo "TAG=${{ inputs.tag }}" >> "$GITHUB_ENV"
    else
      echo "TAG=${GITHUB_REF_NAME}" >> "$GITHUB_ENV"
    fi
- name: Set version without v prefix
  run: echo "VERSION=${TAG#v}" >> "$GITHUB_ENV"
```

All checkout steps use:
```yaml
- uses: actions/checkout@v4
  with:
    ref: ${{ env.TAG }}
```

### Jobs

#### `test`
- **Runs on:** `ubuntu-latest`
- **Timeout:** 15 minutes
- **Steps:**
  1. Determine version (sets `TAG` and `VERSION` env vars)
  2. `actions/checkout@v4` with `ref: ${{ env.TAG }}`
  3. Install Rust 1.75 + cache
  4. Verify tag matches Cargo.toml version:
     ```bash
     CARGO_VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
     if [ "$VERSION" != "$CARGO_VERSION" ]; then
       echo "::error::Tag $TAG does not match Cargo.toml version $CARGO_VERSION"
       exit 1
     fi
     ```
  5. `cargo test`

#### `build` (needs: test)
- **Runs on:** `ubuntu-latest`
- **Timeout:** 15 minutes
- **Steps:**
  1. Determine version
  2. `actions/checkout@v4` with `ref: ${{ env.TAG }}`
  3. Install Rust 1.75 + cache
  4. `cargo build --release`
  5. Package artifact:
     ```bash
     mkdir -p dist
     cp target/release/isolde dist/
     cp -r core/features dist/
     cp -r templates dist/
     cp presets.yaml dist/
     tar czf isolde-linux-x86_64.tar.gz -C dist .
     sha256sum isolde-linux-x86_64.tar.gz > isolde-linux-x86_64.tar.gz.sha256
     ```
  6. Upload `isolde-linux-x86_64.tar.gz` and `.sha256` via `actions/upload-artifact@v4`

#### `release` (needs: build)
- **Runs on:** `ubuntu-latest`
- **Timeout:** 10 minutes
- **Steps:**
  1. Determine version
  2. `actions/checkout@v4` with `ref: ${{ env.TAG }}` (needed for CHANGELOG.md)
  3. Download build artifacts
  4. Extract CHANGELOG section for the current version:
     ```bash
     # Extract section between ## [VERSION] and next ## [ header
     BODY=$(awk "/^## \[${VERSION}\]/{found=1; next} /^## \[/{found=0} found" CHANGELOG.md)
     echo "RELEASE_BODY<<EOF" >> "$GITHUB_ENV"
     echo "$BODY" >> "$GITHUB_ENV"
     echo "EOF" >> "$GITHUB_ENV"
     ```
  5. Create GitHub Release via `softprops/action-gh-release@v2`:
     ```yaml
     with:
       tag_name: ${{ env.TAG }}
       name: Isolde ${{ env.TAG }}
       body: ${{ env.RELEASE_BODY }}
       files: |
         isolde-linux-x86_64.tar.gz
         isolde-linux-x86_64.tar.gz.sha256
       make_latest: true
     ```

## Files Changed

| Action | File |
|--------|------|
| Delete | `.github/workflows/test.yml` |
| Create | `.github/workflows/ci.yml` |
| Create | `.github/workflows/release.yml` |

## Artifact Contents

`isolde-linux-x86_64.tar.gz` includes:
- `isolde` — release binary
- `core/features/` — devcontainer features (needed by `isolde sync`)
- `templates/` — language templates (needed by `isolde init`)
- `presets.yaml` — preset configurations

SHA256 checksum published alongside for verification.

## Decisions

1. **Docker tests dropped** — The old `test.yml` Docker tests reference `.devcontainer/` at repo root which doesn't exist. Rather than migrating broken tests with `continue-on-error`, they are omitted entirely. Proper integration testing (generate project → build container → verify) should be designed separately.
2. **Clippy included** — Clippy ships with Rust 1.75 and is already used in the local `make test` workflow. CI should match local development.
3. **Version consistency check** — Release workflow verifies tag matches `Cargo.toml` version to prevent mismatched releases.
4. **SHA256 checksum** — Published alongside release binary for integrity verification.
5. **awk over sed for CHANGELOG** — More robust extraction that handles the last section correctly without including trailing link references.
