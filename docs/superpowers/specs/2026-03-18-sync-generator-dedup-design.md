# Sync/Generator Deduplication Design

**Date:** 2026-03-18
**Sprint:** v0.3.0, Sessions 5-6
**Status:** Reviewed

## Problem

`isolde-cli/src/commands/sync.rs` and `isolde-core/src/generator.rs` both generate devcontainer artifacts (devcontainer.json, Dockerfile, features copy) with different implementations. This causes:

1. **Functional divergence** — `sync.rs` correctly detects host auth files and creates volume directories; `generator.rs` hardcodes auth detection to false and skips volume setup entirely.
2. **Code duplication** — `copy_dir_recursive()` is 100% duplicated; `copy_core_features()` is ~95% duplicated; feature directory discovery uses two incompatible strategies.
3. **Maintenance burden** — bug fixes in one path (e.g., the `--lang-version` fix in session 3) must be applied to both.

### Current Architecture

```
isolde init  --> init.rs: generates isolde.yaml only (correct)
isolde sync  --> sync.rs: generates .devcontainer/ from isolde.yaml (correct, but logic trapped in CLI)
isolde diff  --> diff.rs: uses Generator from generator.rs to compute expected files (broken — uses wrong rendering path)
```

**Key finding:** `init.rs` already does NOT use `Generator`. It generates `isolde.yaml` and tells the user to run `isolde sync`. The `Generator` struct is only used by `diff.rs`.

**Additional finding:** `diff.rs` has a **third** set of broken rendering functions (lines 583-674): simplified copies of `generate_devcontainer_json()`, `generate_dockerfile()`, `generate_claude_md()`, plus `generate_readme()` and two `.gitignore` generators. These produce incomplete output (e.g., devcontainer.json only includes `common-utils`, missing agent features, mounts, proxy, plugins). This is the most dangerous duplication — `isolde diff` currently shows incorrect expected content.

## Design

### Principle

One rendering path for devcontainer artifacts. All commands that need to generate or compare devcontainer files use the same core functions.

### New Module: `isolde-core/src/devcontainer.rs`

Extract pure rendering functions from `sync.rs` into `isolde-core`. These functions take `&Config` (and optional host state) and return `Result<String>` — no filesystem writes, no CLI output.

```rust
// isolde-core/src/devcontainer.rs

/// Information about host auth files (for full isolation mode)
pub struct HostAuthInfo {
    pub credentials_exist: bool,
    pub providers_exist: bool,
    pub provider_file_exists: bool,
}

impl HostAuthInfo {
    /// Detect auth files from the current host environment
    pub fn detect() -> Self { ... }

    /// No auth files (for testing or when detection is not needed)
    pub fn none() -> Self { ... }
}

/// Render devcontainer.json from config
pub fn render_devcontainer_json(config: &Config, host_auth: &HostAuthInfo) -> Result<String> { ... }

/// Render Dockerfile from config
pub fn render_dockerfile(config: &Config) -> Result<String> { ... }

/// Render CLAUDE.md from config
pub fn render_claude_md(config: &Config) -> Result<String> { ... }

/// Find the core features directory (env var, relative paths, XDG, system-wide)
pub fn find_core_features_dir() -> Result<PathBuf> { ... }

/// Copy all core features to the destination directory
pub fn copy_core_features(dest: &Path) -> Result<()> { ... }

/// Copy a directory recursively
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> { ... }

/// List relative paths of all artifacts that sync would generate.
/// Paths are relative to the project root (e.g., ".devcontainer/devcontainer.json").
/// Used by diff.rs to determine expected file set. The caller (diff.rs) is
/// responsible for checking which paths already exist on disk to classify
/// them as create vs. modify.
pub fn expected_artifacts(config: &Config) -> Vec<String> { ... }
```

### Changes to Existing Files

#### `isolde-core/src/lib.rs`
- Add `pub mod devcontainer;`

#### `isolde-cli/src/commands/sync.rs`
- Remove: `generate_devcontainer()`, `generate_dockerfile()`, `generate_claude_md()`, `copy_core_features()`, `find_core_features_dir()`, `copy_dir_recursive()` (all private functions)
- Keep: `SyncOptions`, `run()`, `write_file()` (CLI-specific orchestration and output)
- `run()` calls `isolde_core::devcontainer::*` for rendering, handles file writing and CLI output locally

**Before:**
```rust
let devcontainer_json = generate_devcontainer(&config)?;
```

**After:**
```rust
let host_auth = isolde_core::devcontainer::HostAuthInfo::detect();
let devcontainer_json = isolde_core::devcontainer::render_devcontainer_json(&config, &host_auth)?;
```

#### `isolde-core/src/generator.rs`
- **Remove entirely** — delete the file and remove `pub mod generator;` from `lib.rs`
- `Generator` struct, `DryRunReport`, `GenerateReport` — all removed
- `render_claude_config()`, `render_project_readme()` — dead code (only called from `Generator::generate()` which is being removed; `init.rs` does not use Generator). Delete.
- All generator.rs tests — delete (rendering tests are replaced by devcontainer.rs tests)

#### `isolde-cli/src/commands/diff.rs`
- Remove: `generate_devcontainer_json()`, `generate_dockerfile()`, `generate_claude_md()`, `generate_readme()` (lines 583-674) — broken simplified copies, replaced by `devcontainer.rs` calls
- Remove: `use isolde_core::generator::Generator` import
- Remove: all `Generator::new()` calls (lines 152, 271)
- Keep: `generate_devcontainer_gitignore()`, `generate_project_gitignore()` — stay in diff.rs (diff-specific, not generated by sync)
- Rewrite `generate_expected_content()` (line 296) to dispatch to `devcontainer::render_*` functions
- Rewrite `run()` to compute expected file list from `devcontainer::expected_artifacts()` and classify create/modify by checking filesystem existence locally (instead of `Generator::dry_run()`)
- Rewrite `generate_file_diff()` to use `devcontainer::render_*` instead of `Generator`

#### `isolde-cli/src/commands/init.rs`
- No changes. Already generates only `isolde.yaml`.

### What Moves Where

| Function | From | To |
|---|---|---|
| `generate_devcontainer()` | `sync.rs` | `devcontainer.rs` as `render_devcontainer_json()` |
| `generate_dockerfile()` | `sync.rs` | `devcontainer.rs` as `render_dockerfile()` |
| `generate_claude_md()` | `sync.rs` | `devcontainer.rs` as `render_claude_md()` |
| `find_core_features_dir()` | `sync.rs` | `devcontainer.rs` |
| `copy_core_features()` | `sync.rs` | `devcontainer.rs` |
| `copy_dir_recursive()` | `sync.rs` + `generator.rs` | `devcontainer.rs` (single copy) |
| `HostAuthInfo::detect()` | inline in `sync.rs` | `devcontainer.rs` as struct |
| `generate_devcontainer_json()` | `diff.rs` (broken copy) | Delete — replaced by `devcontainer.rs` |
| `generate_dockerfile()` | `diff.rs` (broken copy) | Delete — replaced by `devcontainer.rs` |
| `generate_claude_md()` | `diff.rs` (broken copy) | Delete — replaced by `devcontainer.rs` |
| `generate_readme()` | `diff.rs` | Delete — sync does not generate README.md |
| `generate_devcontainer_gitignore()` | `diff.rs` | Keep in `diff.rs` (diff-specific) |
| `generate_project_gitignore()` | `diff.rs` | Keep in `diff.rs` (diff-specific) |
| `DryRunReport` | `generator.rs` | Delete — `diff.rs` classifies create/modify locally |
| `GenerateReport` | `generator.rs` | Delete |
| `Generator` struct | `generator.rs` | Delete entirely |
| `render_claude_config()` | `generator.rs` | Delete (dead code — only called from Generator::generate()) |
| `render_project_readme()` | `generator.rs` | Delete (dead code) |
| `generator.rs` file | `isolde-core` | Delete entirely, remove `pub mod generator;` from `lib.rs` |

### Rendering Approach

Use **sync.rs's direct JSON construction** (not generator.rs's template substitution) as the canonical approach because:

1. It correctly handles all features: auth detection, volumes, conditional Node.js, language features, proxy, plugins
2. It produces structured JSON via `serde_json` — no risk of unresolved placeholders
3. It's the more complete and battle-tested path

The template substitution approach in `generator.rs` (embedded template + `build_substitution_map()`) is removed entirely.

### Tests

- **Move sync.rs tests** → `devcontainer.rs` tests (they test rendering logic, not CLI behavior)
- **Remove generator.rs tests** that test rendering/substitution (replaced by devcontainer.rs tests)
- **Keep generator.rs tests** for `render_claude_config()` and `render_project_readme()` if those functions remain
- **Update diff.rs** tests if any exist
- All 103+ existing tests must pass after refactoring

### Risks

1. **diff.rs rewrite scope** — `diff.rs` has three dependencies on the old code: `Generator` struct, `DryRunReport`, and its own broken rendering functions. All three must be replaced in one pass. This is the riskiest part. Mitigated by doing diff.rs rewrite as a dedicated session 6 task.
2. **Feature discovery path change** — `generator.rs` uses `find_isolde_root()` (searches for `templates/` + `core/`), while `sync.rs` uses `find_core_features_dir()` (more robust, with env var support). After the refactor, diff.rs will use `find_core_features_dir()` which may behave differently in edge cases where only `core/features/` exists without `templates/`.
3. **diff.rs no longer needs Generator fallback** — currently diff.rs falls back gracefully when `Generator::new()` fails (line 201). After refactoring, since `devcontainer::render_*` are pure functions that don't search for isolde_root, this failure mode only affects `expected_artifacts()` and feature copying. The fallback logic in diff.rs `run()` needs to be updated accordingly.

### Out of Scope

- Changing `init.rs` behavior (already correct)
- Changing `isolde.yaml` format
- Adding new features to devcontainer generation
- E2E tests (session 8)

## Implementation Plan (Sessions 5-6)

### Session 5 (~1.5h)
1. Create `isolde-core/src/devcontainer.rs` with `HostAuthInfo`, rendering functions, and utility functions — all extracted from `sync.rs`
2. Register module in `lib.rs`
3. Rewrite `sync.rs` to call `devcontainer::*` functions
4. Move sync.rs rendering tests to `devcontainer.rs`
5. `cargo test` — all tests pass

### Session 6 (~1.5h)
1. Rewrite `diff.rs` to use `devcontainer::*` instead of `Generator`:
   - Delete broken private rendering functions (lines 583-674)
   - Rewrite `generate_expected_content()` to dispatch to `devcontainer::render_*`
   - Rewrite `run()`: replace `Generator::dry_run()` with `devcontainer::expected_artifacts()` + local filesystem check for create/modify classification
   - Rewrite `generate_file_diff()`: remove `Generator::new()`, use `devcontainer::render_*`
   - Update fallback path (line 201) for when features dir is not available
2. Delete `isolde-core/src/generator.rs` entirely
3. Remove `pub mod generator;` from `isolde-core/src/lib.rs`
4. Update any remaining imports (`use isolde_core::generator::*`)
5. unwrap() cleanup in `devcontainer.rs` and `template.rs`
6. `cargo test && cargo clippy` — all clean

## Success Criteria

- [ ] Single rendering path: `devcontainer.rs` is the only place that generates devcontainer.json, Dockerfile, CLAUDE.md
- [ ] `sync.rs` is a thin CLI wrapper (~120-140 lines of non-test code, down from ~625)
- [ ] `generator.rs` is deleted
- [ ] `diff.rs` uses `devcontainer.rs` for all expected content generation; broken rendering copies removed
- [ ] All existing tests pass (test count may decrease due to removed generator.rs tests, but no regressions)
- [ ] `cargo clippy` clean
- [ ] No functional regression in `isolde sync`, `isolde diff`
