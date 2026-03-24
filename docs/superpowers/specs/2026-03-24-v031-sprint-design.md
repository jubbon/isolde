# v0.3.1 Sprint Design

**Date:** 2026-03-24
**Status:** Approved
**Release:** v0.3.1 on March 27, 2026

## Overview

Patch release focused on bug fixes and code quality improvements found during the project audit on March 24, 2026. No new features — stability only.

## Scope

- 13 bugs from the audit report
- 1 tech debt item (version deduplication)
- 3 days × 1.5 hours = 4.5 hours total

## Version: v0.3.1

SemVer patch — only bug fixes, no new features or breaking changes.

## Version Deduplication

**Problem:** Version is duplicated in `Cargo.toml` (`workspace.package.version = "0.3.0"`) and `VERSION` file (`0.3.0`). Both must be kept in sync manually.

**Solution:** Remove `VERSION` file. Use `Cargo.toml` as single source of truth. Update:
- `isolde-cli/build.rs` — use `env!("CARGO_PKG_VERSION")` or emit `ISOLDE_VERSION` from `CARGO_PKG_VERSION` instead of reading `VERSION` file. Remove `rerun-if-changed=../VERSION` directive. Remove `semver` validation (Cargo already validates the version).
- `isolde-cli/Cargo.toml` — remove `semver` from `build-dependencies` if no longer needed
- `mk/release.mk` — extract version from `Cargo.toml` instead of `VERSION` file
- `isolde-cli/src/version_info.rs` — verify which env var it reads (`ISOLDE_VERSION` vs `CARGO_PKG_VERSION`)
- `.github/workflows/release.yml` — already reads from `Cargo.toml`, no change needed

## Day 1 (March 25) — 1.5h: Critical bugs

### 1. Version deduplication (25 min)
- Delete `VERSION` file
- Update `build.rs`: replace file read with `env!("CARGO_PKG_VERSION")`, remove `rerun-if-changed=../VERSION`, remove semver validation
- Check `version_info.rs` for env var name, keep `ISOLDE_VERSION` sourced from `CARGO_PKG_VERSION` if needed
- Remove `semver` from `build-dependencies` in `isolde-cli/Cargo.toml` if no longer used
- Update `mk/release.mk` to extract version from `Cargo.toml`
- **Important:** Delete VERSION and update build.rs atomically in one commit to avoid build breakage

### 2. `plugins()` always returns empty slice (15 min)
- **File:** `isolde-core/src/config.rs:176-191`
- **Bug:** `plugins()` returns `&[]` even when plugins exist. `plugins_vec()` works correctly.
- **Fix:** Remove `plugins()` method, rename `plugins_vec()` to `plugins()`, update callers. Return `Vec<PluginView>` instead of `&[PluginView]`.

### 3. `sync` hardcodes `project/` directory (15 min)
- **File:** `isolde-cli/src/commands/sync.rs:80`
- **Bug:** `let project_dir = opts.cwd.join("project")` ignores `config.workspace_dir()`
- **Fix:** Replace with `opts.cwd.join(config.workspace_dir())`

### 4. Guard in `sync` for unimplemented agents (15 min)
- **File:** `isolde-cli/src/commands/sync.rs`
- **Bug:** `sync` generates broken devcontainer for `agent: aider` or `agent: gemini` (no install.sh)
- **Fix:** Move `is_agent_implemented()` from `init.rs` to a shared location (e.g., `isolde-core` or `commands/mod.rs`), then call it in sync command to return error or warning

### 5. Input validation: project name + docker image (25 min)
- **File:** `isolde-core/src/config/v0_1/mod.rs` (in `validate()`)
- **Bug:** No character validation — `../` in project name alters mount target, newlines in docker image inject Dockerfile instructions
- **Fix:** Add `validate_project_name()` (alphanumeric + `-` + `_`, max 64 chars) and `validate_docker_image()` (alphanumeric + `.` `/` `:` `-` `_` `@`, no newlines) to the `validate()` method. Use `Error::InvalidTemplate` for consistency.

## Day 2 (March 26) — 1.5h: High/Medium bugs

### 6. `process::exit()` → `Err()` (35 min)
- **Files:** `isolde-cli/src/commands/run.rs:49`, `exec.rs:65,72`, `logs.rs:72`, `diff.rs:230`
- **Bug:** Bypasses destructors, makes functions untestable
- **Fix:** Add `Error::ExitCode(i32)` variant to error type. Replace all `process::exit()` calls with `Err(Error::ExitCode(code))`. Update `main.rs` to pattern-match on `ExitCode` and call `process::exit()` there.

### 7. `up()` non-detach: skip container info after shell (20 min)
- **File:** `isolde-core/src/container.rs:132-175`
- **Bug:** In non-detach mode (line 137), `group_spawn()` launches the container shell without waiting. Execution then falls through to `get_container_info()` at line 173 before the container is ready.
- **Fix:** In non-detach mode, await the group process before querying container info, or skip the `get_container_info()` call entirely since the user already has the shell.

### 8. `generate_config_from_preset` missing `marketplaces:` + empty plugins (20 min)
- **File:** `isolde-cli/src/commands/init.rs:326-370`
- **Bug:** (a) Generated YAML includes `marketplace: omc` per plugin but no `marketplaces:` section, failing validation. (b) When plugins list is empty, generated YAML has a bare `plugins:` block with no value, producing odd YAML.
- **Fix:** (a) When plugins non-empty, prepend `marketplaces:\n  omc:\n    url: <url>` before the `plugins:` block. (b) When plugins empty, emit `plugins: []` instead of an empty block.

### 9. `HashMap` → `BTreeMap` for stable JSON output (15 min)
- **Bug:** `agent_options()` returns `&HashMap<String, AgentOptionValue>`, iteration order is non-deterministic
- **Fix:** Change the `options` field in `AgentConfig` struct (`isolde-core/src/config/v0_1/mod.rs`) from `HashMap` to `BTreeMap`. Update `agent_options()` return type in `isolde-core/src/config.rs`. Call sites in `devcontainer.rs` and `template.rs` require no change.

## Day 3 (March 27) — 1.5h: Quality + release

### 10. `expected_artifacts()` — dynamic list from config (15 min)
- **File:** `isolde-core/src/devcontainer.rs:508-522`
- **Bug:** Hardcoded feature list ignores actual agent config, causes false positives in `isolde diff`
- **Fix:** Use config to determine which features should be present (agent feature + proxy + plugin-manager if applicable)

### 11. Typo "Isolle" + fix `--force` error message (10 min)
- **Files:** `isolde-cli/src/commands/diff.rs:597`, `isolde-cli/src/commands/init.rs:541`
- **Bug:** Typo "Isolle" instead of "Isolde"; error message suggests `--force` flag that doesn't exist
- **Fix:** Fix typo. Change init error message to: "isolde.yaml already exists. Delete it and re-run `isolde init` to regenerate."

### 12. `validate` ignores `cwd` (15 min)
- **File:** `isolde-cli/src/commands/validate.rs:119`
- **Bug:** Always uses `std::env::current_dir()`, inconsistent with other commands
- **Fix:** Add `cwd: PathBuf` to `ValidateOptions`, use it instead of `current_dir()`

### 13. Dead code cleanup (15 min)
- **Files:**
  - `isolde-core/src/config.rs:296-394` — remove unused legacy types (verify not in public API first; mark `#[deprecated]` if exposed)
  - `isolde-cli/src/cli.rs:441` — remove unused `print_error` function
- **Note:** `.tera` template files and `DiffLineType::Header` are actively used — do NOT delete them.

### 14. Release v0.3.1 (15 min)
- Update `Cargo.toml` version to `0.3.1`
- Add `## [0.3.1]` entry to `CHANGELOG.md`
- Merge dev → main (direct merge, pet project)
- Tag v0.3.1, push, verify release workflow

## Deferred to next sprint

- Docs rewrite (`docs/user/`, `docs/contributor/`)
- E2E test: init+sync workflow
- state.rs save/load round-trip tests
- devcontainer.rs render tests (proxy, codex, nodejs, plugins)
- Security: `curl | bash` integrity check, `source` state file validation, shell injection in `su -c`
- Gemini/Aider agent implementation
