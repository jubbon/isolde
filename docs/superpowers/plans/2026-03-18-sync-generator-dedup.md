# Sync/Generator Deduplication Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Eliminate devcontainer rendering duplication by extracting shared logic from `sync.rs` into `isolde-core/src/devcontainer.rs`, then removing `generator.rs` entirely.

**Architecture:** Extract pure rendering functions (no I/O, no CLI output) into a new `devcontainer` module in `isolde-core`. `sync.rs` becomes a thin CLI wrapper. `diff.rs` switches from the broken `Generator` struct to the same `devcontainer::*` functions. `generator.rs` is deleted.

**Tech Stack:** Rust, serde_json, isolde-core library, cargo test

**Spec:** `docs/superpowers/specs/2026-03-18-sync-generator-dedup-design.md`

---

## File Structure

| File | Action | Responsibility |
|---|---|---|
| `isolde-core/src/devcontainer.rs` | **Create** | Pure rendering functions for devcontainer.json, Dockerfile, CLAUDE.md; utility functions for feature copying and discovery |
| `isolde-core/src/lib.rs` | Modify | Add `pub mod devcontainer;`, remove `pub mod generator;` (session 6) |
| `isolde-cli/src/commands/sync.rs` | Modify | Thin CLI wrapper — calls `devcontainer::*`, handles file I/O and user output |
| `isolde-cli/src/commands/diff.rs` | Modify (session 6) | Replace `Generator` + broken renderers with `devcontainer::*` |
| `isolde-core/src/generator.rs` | **Delete** (session 6) | Removed entirely |

---

## Chunk 1: Session 5 — Extract devcontainer.rs and rewrite sync.rs

### Task 1: Verify baseline — all tests pass

**Files:** None (read-only)

- [ ] **Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass. Record the test count for later comparison.

- [ ] **Step 2: Run clippy**

Run: `cargo clippy`
Expected: No errors (warnings acceptable).

---

### Task 2: Create `devcontainer.rs` with HostAuthInfo and utility functions

**Files:**
- Create: `isolde-core/src/devcontainer.rs`
- Modify: `isolde-core/src/lib.rs`

- [ ] **Step 1: Create the module file with HostAuthInfo and utility functions**

Create `isolde-core/src/devcontainer.rs` with:

```rust
//! # Devcontainer artifact rendering
//!
//! Pure functions that generate devcontainer configuration files from an Isolde Config.
//! Used by `isolde sync` (CLI) and `isolde diff` (comparison).

use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{AgentOptionValue, Config, IsolationLevel};
use crate::error::{Error, Result};

/// Information about host auth files (for full isolation mode mount generation).
#[derive(Debug, Clone)]
pub struct HostAuthInfo {
    /// Whether ~/.claude/.credentials.json exists
    pub credentials_exist: bool,
    /// Whether ~/.claude/providers/ directory exists
    pub providers_exist: bool,
    /// Whether ~/.claude/provider file exists
    pub provider_file_exists: bool,
}

impl HostAuthInfo {
    /// Detect auth files from the current host environment.
    pub fn detect() -> Self {
        let home = std::env::var("HOME").unwrap_or_default();
        if home.is_empty() {
            return Self::none();
        }
        let base = Path::new(&home).join(".claude");
        Self {
            credentials_exist: base.join(".credentials.json").exists(),
            providers_exist: base.join("providers").exists(),
            provider_file_exists: base.join("provider").exists(),
        }
    }

    /// No auth files detected (for testing or non-full isolation).
    pub fn none() -> Self {
        Self {
            credentials_exist: false,
            providers_exist: false,
            provider_file_exists: false,
        }
    }
}

/// Copy a directory recursively.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst).map_err(|e| {
        Error::FileError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create directory {}: {}", dst.display(), e),
        ))
    })?;

    for entry in fs::read_dir(src)
        .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?
    {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| {
                Error::FileError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to copy {} to {}: {}",
                        src_path.display(),
                        dst_path.display(),
                        e
                    ),
                ))
            })?;
        }
    }

    Ok(())
}

/// Find the core features directory.
///
/// Search order: ISOLDE_CORE_FEATURES env var, relative paths from cwd,
/// paths relative to executable, XDG data dirs, system-wide paths.
pub fn find_core_features_dir() -> Result<PathBuf> {
    let mut possible_paths = vec![];

    if let Ok(env_path) = std::env::var("ISOLDE_CORE_FEATURES") {
        possible_paths.push(PathBuf::from(env_path));
    }

    possible_paths.push(PathBuf::from("core/features"));
    possible_paths.push(PathBuf::from("../core/features"));
    possible_paths.push(PathBuf::from("../../core/features"));

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Some(prefix) = exe_dir.parent() {
                possible_paths.push(
                    prefix
                        .join("share")
                        .join("isolde")
                        .join("core")
                        .join("features"),
                );
            }
            possible_paths.push(exe_dir.join("core").join("features"));
            possible_paths.push(
                exe_dir
                    .join("../core/features")
                    .canonicalize()
                    .unwrap_or_else(|_| PathBuf::from("../core/features")),
            );
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        possible_paths.push(
            PathBuf::from(home.clone())
                .join(".local/share/isolde/core/features"),
        );
        possible_paths.push(PathBuf::from(home).join(".isolde/core/features"));
    }

    possible_paths.push(PathBuf::from("/usr/local/share/isolde/core/features"));
    possible_paths.push(PathBuf::from("/opt/isolde/core/features"));

    for path in possible_paths {
        if path.exists() {
            return Ok(path);
        }
    }

    Err(Error::PathNotFound(PathBuf::from("core/features")))
}

/// Copy all core features to the destination directory.
///
/// Each subdirectory under `core/features/` is copied to `dest/<feature-name>/`.
/// Existing feature directories at the destination are removed first.
pub fn copy_core_features(dest: &Path) -> Result<()> {
    let core_features = find_core_features_dir()?;

    for entry in fs::read_dir(&core_features)
        .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?
    {
        let entry = entry?;
        let feature_path = entry.path();

        if feature_path.is_dir() {
            let feature_name = feature_path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| Error::Other("Invalid feature name".to_string()))?;

            let feature_dest = dest.join(feature_name);

            if feature_dest.exists() {
                fs::remove_dir_all(&feature_dest).map_err(|e| {
                    Error::FileError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to remove {}: {}", feature_dest.display(), e),
                    ))
                })?;
            }

            copy_dir_recursive(&feature_path, &feature_dest)?;
        }
    }

    Ok(())
}
```

- [ ] **Step 2: Register the module in lib.rs**

In `isolde-core/src/lib.rs`, add `pub mod devcontainer;` after `pub mod config;`.

- [ ] **Step 3: Verify compilation**

Run: `cargo build`
Expected: Compiles with no errors.

- [ ] **Step 4: Add tests for utility functions**

Append to `isolde-core/src/devcontainer.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_auth_info_none() {
        let info = HostAuthInfo::none();
        assert!(!info.credentials_exist);
        assert!(!info.providers_exist);
        assert!(!info.provider_file_exists);
    }

    #[test]
    fn test_copy_dir_recursive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let src = temp_dir.path().join("src");
        let dst = temp_dir.path().join("dst");

        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("file1.txt"), "content1").unwrap();
        fs::create_dir_all(src.join("subdir")).unwrap();
        fs::write(src.join("subdir/file2.txt"), "content2").unwrap();

        copy_dir_recursive(&src, &dst).unwrap();

        assert!(dst.join("file1.txt").exists());
        assert_eq!(fs::read_to_string(dst.join("file1.txt")).unwrap(), "content1");
        assert!(dst.join("subdir/file2.txt").exists());
        assert_eq!(fs::read_to_string(dst.join("subdir/file2.txt")).unwrap(), "content2");
    }
}
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p isolde-core devcontainer`
Expected: 2 new tests pass.

- [ ] **Step 6: Commit**

```
git add isolde-core/src/devcontainer.rs isolde-core/src/lib.rs
git commit -m "refactor: add devcontainer module with HostAuthInfo and utility functions"
```

---

### Task 3: Add render_devcontainer_json to devcontainer.rs

**Files:**
- Modify: `isolde-core/src/devcontainer.rs`

- [ ] **Step 1: Add the render function**

Add to `isolde-core/src/devcontainer.rs` (before `#[cfg(test)]`):

```rust
/// Render devcontainer.json content from configuration.
///
/// Builds a complete devcontainer.json with: common-utils, conditional Node.js,
/// language-specific features, proxy, agent feature, plugin manager, mounts,
/// and feature install order.
pub fn render_devcontainer_json(config: &Config, host_auth: &HostAuthInfo) -> Result<String> {
    let mut features = serde_json::Map::new();

    // common-utils: match host UID/GID for bind-mounted directories
    features.insert(
        "ghcr.io/devcontainers/features/common-utils:2".to_string(),
        serde_json::json!({
            "installZsh": false,
            "installOhMyZsh": false,
            "upgradePackages": false,
            "username": "${localEnv:USER}",
            "userUid": "automatic",
            "userGid": "automatic"
        }),
    );

    // Node.js for agents that need npm (claude-code, codex).
    // Skip if the project language is already nodejs/javascript.
    let agent_needs_node = matches!(config.agent_name(), "claude-code" | "codex");
    let lang_is_node = config
        .runtime()
        .map(|r| matches!(r.language(), "nodejs" | "javascript"))
        .unwrap_or(false);

    if agent_needs_node && !lang_is_node {
        features.insert(
            "ghcr.io/devcontainers/features/node:1".to_string(),
            serde_json::json!({
                "version": "lts",
                "nodeGypDependencies": true,
                "npxInstallCachedPackages": true
            }),
        );
    }

    // Language-specific features
    if let Some(runtime) = config.runtime() {
        match runtime.language() {
            "python" => {
                features.insert(
                    "ghcr.io/devcontainers/features/python:1".to_string(),
                    serde_json::json!({
                        "version": runtime.version(),
                        "installTools": true
                    }),
                );
            }
            "nodejs" | "javascript" => {
                features.insert(
                    "ghcr.io/devcontainers/features/node:1".to_string(),
                    serde_json::json!({
                        "version": runtime.version()
                    }),
                );
            }
            "rust" => {
                features.insert(
                    "ghcr.io/devcontainers/features/rust:1".to_string(),
                    serde_json::json!({
                        "version": runtime.version()
                    }),
                );
            }
            "go" => {
                features.insert(
                    "ghcr.io/devcontainers/features/go:1".to_string(),
                    serde_json::json!({
                        "version": runtime.version()
                    }),
                );
            }
            _ => {}
        }
    }

    // Proxy feature
    if let Some(proxy) = config.proxy() {
        features.insert(
            "./features/proxy".to_string(),
            serde_json::json!({
                "http_proxy": proxy.http(),
                "https_proxy": proxy.https(),
                "no_proxy": proxy.no_proxy(),
                "enabled": true
            }),
        );
    }

    // Coding agent feature
    let mut agent_opts = serde_json::Map::new();
    agent_opts.insert(
        "version".to_string(),
        serde_json::Value::String(config.agent_version().to_string()),
    );
    for (key, value) in config.agent_options() {
        let json_val = match value {
            AgentOptionValue::Str(s) => serde_json::Value::String(s.clone()),
            AgentOptionValue::Map(m) => {
                let csv = m
                    .iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<_>>()
                    .join(",");
                serde_json::Value::String(csv)
            }
        };
        agent_opts.insert(key.clone(), json_val);
    }
    if let Some(proxy) = config.proxy() {
        if let Some(h) = proxy.http() {
            agent_opts.insert("http_proxy".to_string(), serde_json::Value::String(h.clone()));
        }
        if let Some(h) = proxy.https() {
            agent_opts.insert("https_proxy".to_string(), serde_json::Value::String(h.clone()));
        }
    }
    let agent_feature_path = format!("./features/{}", config.agent_name());
    features.insert(agent_feature_path, serde_json::Value::Object(agent_opts));

    // Plugin manager feature
    let plugins = config.plugins_vec();
    if !plugins.is_empty() {
        let activate: Vec<&str> = plugins.iter().filter(|p| p.activate).map(|p| p.name.as_str()).collect();
        let deactivate: Vec<&str> = plugins.iter().filter(|p| !p.activate).map(|p| p.name.as_str()).collect();

        features.insert(
            "./features/plugin-manager".to_string(),
            serde_json::json!({
                "activate_plugins": activate,
                "deactivate_plugins": deactivate
            }),
        );
    }

    // Feature install order
    let mut override_order: Vec<String> = vec![];
    if config.proxy().is_some() {
        override_order.push("./features/proxy".to_string());
    }
    override_order.push(format!("./features/{}", config.agent_name()));
    if !plugins.is_empty() {
        override_order.push("./features/plugin-manager".to_string());
    }

    // Mounts (uses host auth info for full isolation)
    let mounts = crate::mounts::generate_mounts(
        config,
        host_auth.credentials_exist,
        host_auth.providers_exist,
        host_auth.provider_file_exists,
    );

    let devcontainer = serde_json::json!({
        "name": format!("{} - Isolde Environment", config.name),
        "build": {
            "dockerfile": "Dockerfile",
            "context": "..",
            "args": {
                "USERNAME": "${localEnv:USER}"
            }
        },
        "features": features,
        "overrideFeatureInstallOrder": override_order,
        "customizations": {
            "vscode": {
                "extensions": ["anthropic.claude-code"],
                "settings": {
                    "terminal.integrated.defaultProfile.linux": "bash"
                }
            }
        },
        "mounts": mounts,
        "remoteUser": "${localEnv:USER}",
        "workspaceFolder": format!("/workspaces/{}", config.name)
    });

    serde_json::to_string_pretty(&devcontainer)
        .map_err(|e| Error::Other(format!("Failed to serialize devcontainer.json: {}", e)))
}
```

- [ ] **Step 2: Add test**

Add to the `tests` module:

```rust
    fn test_config(yaml: &str) -> Config {
        Config::from_str(yaml).unwrap()
    }

    fn minimal_config() -> Config {
        test_config(
            r#"
version: "0.1"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#,
        )
    }

    #[test]
    fn test_render_devcontainer_json_basic() {
        let config = minimal_config();
        let result = render_devcontainer_json(&config, &HostAuthInfo::none()).unwrap();
        assert!(result.contains("\"features\""));
        assert!(result.contains("claude-code"));
        assert!(result.contains("common-utils"));
    }

    #[test]
    fn test_render_devcontainer_json_with_python() {
        let config = test_config(
            r#"
version: "0.1"
name: test
docker:
  image: ubuntu:latest
runtime:
  language: python
  version: "3.12"
  package_manager: uv
"#,
        );
        let result = render_devcontainer_json(&config, &HostAuthInfo::none()).unwrap();
        assert!(result.contains("ghcr.io/devcontainers/features/python:1"));
        assert!(result.contains("3.12"));
    }
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p isolde-core devcontainer`
Expected: 4 tests pass.

- [ ] **Step 4: Commit**

```
git add isolde-core/src/devcontainer.rs
git commit -m "refactor: add render_devcontainer_json to devcontainer module"
```

---

### Task 4: Add render_dockerfile and render_claude_md to devcontainer.rs

**Files:**
- Modify: `isolde-core/src/devcontainer.rs`

- [ ] **Step 1: Add render_dockerfile**

Add to `isolde-core/src/devcontainer.rs` (before `#[cfg(test)]`):

```rust
/// Render Dockerfile content from configuration.
pub fn render_dockerfile(config: &Config) -> Result<String> {
    let dockerfile = format!(
        r#"ARG BASE_IMAGE={}
FROM ${{BASE_IMAGE}}

# User arguments with defaults
ARG USERNAME=user

# Set DEBIAN_FRONTEND for non-interactive apt
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /workspaces

# If the base image has a 'vscode' user at UID 1000 and the requested user is different,
# reassign vscode to a high UID so that updateRemoteUserUID can give UID 1000 to USERNAME.
# This ensures bind-mounted host directories (owned by UID 1000) appear with the correct
# owner inside the container without needing chown on the bind mount.
RUN if [ "${{USERNAME}}" != "vscode" ] && id vscode >/dev/null 2>&1; then \
        userdel -r vscode 2>/dev/null || true; \
        groupdel vscode 2>/dev/null || true; \
    fi
"#,
        config.docker_image()
    );

    Ok(dockerfile)
}

/// Render CLAUDE.md content from configuration.
pub fn render_claude_md(config: &Config) -> Result<String> {
    let mut content = format!(
        r#"# Claude Code Configuration for {}

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**{}** - Schema Version {}

This is an Isolde-managed isolated development environment.

## Configuration

- **Docker Image**: {}
- **Agent**: {}
- **Workspace Directory**: {}
"#,
        config.name,
        config.name,
        config.version,
        config.docker_image(),
        config.agent_name(),
        config.workspace_dir()
    );

    if let Some(runtime) = config.runtime() {
        content.push_str(&format!(
            r#"
## Runtime Environment

- **Language**: {} {}
- **Package Manager**: {}
"#,
            runtime.language(),
            runtime.version(),
            runtime.package_manager()
        ));

        if !runtime.tools().is_empty() {
            content.push_str("\n### Tools\n");
            for tool in runtime.tools() {
                content.push_str(&format!("- {}\n", tool));
            }
        }
    }

    let plugins = config.plugins_vec();
    if !plugins.is_empty() {
        content.push_str("\n## Claude Plugins\n");
        for plugin in &plugins {
            let status = if plugin.activate { "✓" } else { "✗" };
            content.push_str(&format!(
                "- {} {} (from marketplace: {})\n",
                status, plugin.name, plugin.marketplace
            ));
        }
    }

    if let Some(proxy) = config.proxy() {
        content.push_str(
            r#"

## Proxy Configuration

This project is configured to work with a corporate proxy.
"#,
        );
        if let Some(http) = proxy.http() {
            content.push_str(&format!("- HTTP Proxy: {}\n", http));
        }
        if let Some(https) = proxy.https() {
            content.push_str(&format!("- HTTPS Proxy: {}\n", https));
        }
        if let Some(no_proxy) = proxy.no_proxy() {
            content.push_str(&format!("- No Proxy: {}\n", no_proxy));
        }
    }

    content.push_str(
        r#"

## Development Workflow

1. **Build the devcontainer**:
   ```bash
   docker build -t <image-name> .devcontainer
   ```

2. **Start development**:
   - Use VS Code Dev Containers or
   - Run: `docker run -it --rm -v $(pwd):/workspaces/<project> <image-name>`

3. **Use Claude Code**:
   ```bash
   claude "help me understand this codebase"
   ```

## Notes

- Configuration is managed by Isolde
- Run `isolde sync` to regenerate configuration
- Do not manually edit generated files in `.devcontainer/`
"#,
    );

    Ok(content)
}
```

- [ ] **Step 2: Add tests**

Add to the `tests` module:

```rust
    #[test]
    fn test_render_dockerfile() {
        let config = minimal_config();
        let dockerfile = render_dockerfile(&config).unwrap();
        assert!(dockerfile.contains("ARG BASE_IMAGE=ubuntu:latest"));
        assert!(dockerfile.contains("FROM ${BASE_IMAGE}"));
        assert!(dockerfile.contains("ARG USERNAME=user"));
    }

    #[test]
    fn test_render_claude_md() {
        let config = test_config(
            r#"
version: "0.1"
name: test-project
docker:
  image: ubuntu:latest
runtime:
  language: python
  version: "3.12"
  package_manager: uv
"#,
        );
        let claude_md = render_claude_md(&config).unwrap();
        assert!(claude_md.contains("test-project"));
        assert!(claude_md.contains("python 3.12"));
    }
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p isolde-core devcontainer`
Expected: 6 tests pass.

- [ ] **Step 4: Commit**

```
git add isolde-core/src/devcontainer.rs
git commit -m "refactor: add render_dockerfile and render_claude_md to devcontainer module"
```

---

### Task 5: Add expected_artifacts function

**Files:**
- Modify: `isolde-core/src/devcontainer.rs`

- [ ] **Step 1: Add expected_artifacts**

Add to `isolde-core/src/devcontainer.rs` (before `#[cfg(test)]`):

```rust
/// List relative paths of all artifacts that `isolde sync` would generate.
///
/// Paths are relative to the project root.
/// The caller is responsible for checking which paths already exist on disk
/// to classify them as "would create" vs. "would modify".
pub fn expected_artifacts(config: &Config) -> Vec<String> {
    let mut paths = vec![
        ".devcontainer/devcontainer.json".to_string(),
        ".devcontainer/Dockerfile".to_string(),
        ".claude/CLAUDE.md".to_string(),
    ];

    // Core feature directories — list known features that would be copied.
    // We list them statically since this function is pure (no filesystem access).
    for feature in &["claude-code", "codex", "gemini", "aider", "proxy", "plugin-manager"] {
        paths.push(format!(".devcontainer/features/{}", feature));
    }

    paths
}
```

- [ ] **Step 2: Add test**

```rust
    #[test]
    fn test_expected_artifacts() {
        let config = minimal_config();
        let artifacts = expected_artifacts(&config);
        assert!(artifacts.contains(&".devcontainer/devcontainer.json".to_string()));
        assert!(artifacts.contains(&".devcontainer/Dockerfile".to_string()));
        assert!(artifacts.contains(&".claude/CLAUDE.md".to_string()));
    }
```

- [ ] **Step 3: Run tests, commit**

Run: `cargo test -p isolde-core devcontainer`
Expected: 7 tests pass.

```
git add isolde-core/src/devcontainer.rs
git commit -m "refactor: add expected_artifacts to devcontainer module"
```

---

### Task 6: Rewrite sync.rs to use devcontainer module

**Files:**
- Modify: `isolde-cli/src/commands/sync.rs`

- [ ] **Step 1: Update imports in sync.rs**

Replace the imports section (lines 1-10) with:

```rust
//! # Isolde sync command
//!
//! Generate devcontainer and Claude configuration from isolde.yaml.

use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use isolde_core::config::{Config, IsolationLevel};
use isolde_core::devcontainer::{self, HostAuthInfo};
use isolde_core::{Error, Result};
```

- [ ] **Step 2: Replace rendering calls in run()**

Replace the body of `run()` to use devcontainer module. The function keeps its signature and all CLI output / file-writing logic, but delegates rendering:

In `run()`, replace:
```rust
    let devcontainer_json = generate_devcontainer(&config)?;
```
with:
```rust
    let host_auth = HostAuthInfo::detect();

    // Warn if full isolation and no auth files found
    if config.isolation() == IsolationLevel::Full && !host_auth.credentials_exist {
        eprintln!(
            "{} {}",
            "⚠".yellow(),
            "No auth credentials found at ~/.claude/.credentials.json".yellow()
        );
        eprintln!(
            "{}",
            "  Run 'claude login' and then 'isolde sync' again to mount auth into the container."
                .dimmed()
        );
    }

    let devcontainer_json = devcontainer::render_devcontainer_json(&config, &host_auth)?;
```

Replace:
```rust
    let dockerfile = generate_dockerfile(&config)?;
```
with:
```rust
    let dockerfile = devcontainer::render_dockerfile(&config)?;
```

Replace:
```rust
    let claude_md = generate_claude_md(&config)?;
```
with:
```rust
    let claude_md = devcontainer::render_claude_md(&config)?;
```

Replace:
```rust
    copy_core_features(&features_dir)?;
```
with:
```rust
    devcontainer::copy_core_features(&features_dir)?;
```

Replace the stub agent warning block (lines 55-72) — update `find_core_features_dir()` call:
```rust
    if !matches!(agent, "claude-code") {
        let has_install_sh = devcontainer::find_core_features_dir()
            .ok()
            .map(|dir| dir.join(agent).join("install.sh").exists())
            .unwrap_or(false);
```

- [ ] **Step 3: Delete all private rendering functions from sync.rs**

Delete these functions entirely (they now live in devcontainer.rs):
- `generate_devcontainer()` (lines 146-347)
- `generate_dockerfile()` (lines 349-376)
- `generate_claude_md()` (lines 378-486)
- `copy_core_features()` (lines 505-544)
- `find_core_features_dir()` (lines 546-593)
- `copy_dir_recursive()` (lines 595-625)

Keep:
- `SyncOptions` struct and `Default` impl
- `run()` function (now using devcontainer::*)
- `write_file()` helper
- `#[cfg(test)] mod tests` block

- [ ] **Step 4: Move the auth warning from inside generate_devcontainer to run()**

The auth warning (lines 307-317 in the original sync.rs) was inside `generate_devcontainer()`. It's already been placed before the `render_devcontainer_json` call in step 2 above. Verify the warning block is in `run()`, not duplicated.

- [ ] **Step 5: Update sync.rs tests**

Update the test imports and calls. The tests should now use `devcontainer::*` or be simplified since rendering is tested in devcontainer.rs. Update:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_options_default() {
        let opts = SyncOptions::default();
        assert!(!opts.dry_run);
        assert!(!opts.force);
    }
}
```

The rendering tests (`test_generate_dockerfile`, `test_generate_claude_md`, `test_generate_devcontainer`) are now in `devcontainer.rs` — remove them from `sync.rs`.

- [ ] **Step 6: Verify full test suite**

Run: `cargo test`
Expected: All tests pass. Some test count may shift (moved from sync to devcontainer) but no failures.

- [ ] **Step 7: Verify clippy**

Run: `cargo clippy`
Expected: No errors.

- [ ] **Step 8: Commit**

```
git add isolde-cli/src/commands/sync.rs
git commit -m "refactor: sync.rs delegates rendering to isolde-core::devcontainer"
```

---

## Chunk 2: Session 6 — Rewrite diff.rs, delete generator.rs, cleanup

### Task 7: Rewrite diff.rs to use devcontainer module

**Files:**
- Modify: `isolde-cli/src/commands/diff.rs`

- [ ] **Step 1: Update imports**

Replace:
```rust
use isolde_core::generator::Generator;
```
with:
```rust
use isolde_core::devcontainer::{self, HostAuthInfo};
```

- [ ] **Step 2: Rewrite run() — replace Generator::dry_run with expected_artifacts**

Replace the `match generator_result` block (lines 160-248) with logic that:
1. Calls `devcontainer::expected_artifacts(&config)` to get expected file paths
2. For each path, checks if it exists on disk → classify as would_create or would_modify
3. Scans `.devcontainer/` for files not in expected set → would_delete (orphans)

```rust
    // Compute expected artifacts
    let expected = devcontainer::expected_artifacts(&config);
    let mut expected_abs: Vec<PathBuf> = Vec::new();

    for rel_path in &expected {
        let abs_path = opts.cwd.join(rel_path);
        if abs_path.exists() {
            would_modify.push(abs_path.clone());
        } else {
            would_create.push(abs_path.clone());
        }
        expected_abs.push(abs_path);
    }

    // If a specific file is requested, only diff that file
    if let Some(ref file) = opts.file {
        let file_path = resolve_file_path(&opts.cwd, file);
        let diff = generate_file_diff(&file_path, &opts)?;
        file_diffs.push(diff);
    } else {
        for path in &would_create {
            file_diffs.push(FileDiff {
                path: path.clone(),
                status: DiffStatus::Created,
                lines: vec![],
            });
        }
        for path in &would_modify {
            let diff = generate_file_diff(path, &opts)?;
            file_diffs.push(diff);
        }
    }

    // Find orphaned files
    let expected_set: std::collections::HashSet<PathBuf> = expected_abs.into_iter().collect();
    let devcontainer_dir = opts.cwd.join(".devcontainer");
    if devcontainer_dir.exists() {
        for entry in walk_dir(&devcontainer_dir)? {
            if !expected_set.contains(&entry) {
                let relative = entry.strip_prefix(&opts.cwd).unwrap_or(&entry);
                if !would_delete.iter().any(|p| p == &entry) {
                    would_delete.push(entry.clone());
                }
                if !unchanged.iter().any(|p| p == relative) {
                    // not orphan and not in expected = unchanged
                }
            } else {
                let relative = entry.strip_prefix(&opts.cwd).unwrap_or(&entry);
                if !would_create.contains(&entry) && !would_modify.contains(&entry) {
                    unchanged.push(relative.to_path_buf());
                }
            }
        }
    }
```

- [ ] **Step 3: Rewrite generate_file_diff — remove Generator dependency**

Replace `generate_file_diff` (lines 265-293):

```rust
fn generate_file_diff(path: &Path, opts: &DiffOptions) -> Result<FileDiff> {
    let current_content = fs::read_to_string(path).unwrap_or_default();

    let config = Config::from_file(&opts.cwd.join("isolde.yaml"))?;
    let expected_content = generate_expected_content(&config, path)?;

    let lines = compute_diff(&current_content, &expected_content, opts.context);

    let status = if current_content.is_empty() {
        DiffStatus::Created
    } else if expected_content.is_empty() {
        DiffStatus::Deleted
    } else if lines
        .iter()
        .any(|l| matches!(l.line_type, DiffLineType::Added | DiffLineType::Removed))
    {
        DiffStatus::Modified
    } else {
        DiffStatus::Unchanged
    };

    Ok(FileDiff {
        path: path.to_path_buf(),
        status,
        lines,
    })
}
```

- [ ] **Step 4: Rewrite generate_expected_content — dispatch to devcontainer::***

Replace `generate_expected_content` (lines 296-328):

```rust
fn generate_expected_content(config: &Config, path: &Path) -> Result<String> {
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    match file_name {
        "devcontainer.json" => {
            devcontainer::render_devcontainer_json(config, &HostAuthInfo::none())
        }
        "Dockerfile" => devcontainer::render_dockerfile(config),
        "CLAUDE.md" => devcontainer::render_claude_md(config),
        ".gitignore" => {
            if path.to_str().unwrap_or("").contains("devcontainer") {
                Ok(generate_devcontainer_gitignore())
            } else {
                Ok(generate_project_gitignore())
            }
        }
        _ => Ok(String::new()),
    }
}
```

Note: `generate_readme` case removed — sync does not generate README.md.

- [ ] **Step 5: Delete broken private rendering functions**

Delete from diff.rs (lines 581-674):
- `generate_devcontainer_json()`
- `generate_dockerfile()`
- `generate_claude_md()`
- `generate_readme()`

Keep:
- `generate_devcontainer_gitignore()` (diff-specific)
- `generate_project_gitignore()` (diff-specific)

- [ ] **Step 6: Remove find_orphaned_files function and its DryRunReport dependency**

Delete the `find_orphaned_files` function (lines 394-414) — its logic is now inlined in `run()`.

Remove the `use isolde_core::generator::Generator;` import if not already done.

- [ ] **Step 7: Run tests**

Run: `cargo test -p isolde-cli diff`
Expected: All diff tests pass.

- [ ] **Step 8: Commit**

```
git add isolde-cli/src/commands/diff.rs
git commit -m "refactor: diff.rs uses devcontainer module instead of Generator"
```

---

### Task 8: Delete generator.rs

**Files:**
- Delete: `isolde-core/src/generator.rs`
- Modify: `isolde-core/src/lib.rs`

- [ ] **Step 1: Remove module declaration from lib.rs**

In `isolde-core/src/lib.rs`, remove:
```rust
pub mod generator;
```

- [ ] **Step 2: Delete generator.rs**

```bash
rm isolde-core/src/generator.rs
```

- [ ] **Step 3: Search for any remaining references**

Run: `cargo build 2>&1`

Fix any compilation errors from remaining `use isolde_core::generator::*` imports. Known locations:
- `isolde-cli/src/commands/diff.rs` — should already be cleaned in Task 7

- [ ] **Step 4: Run full test suite**

Run: `cargo test`
Expected: All tests pass. Test count will decrease (generator.rs had ~15 tests).

- [ ] **Step 5: Commit**

```
git add -u
git commit -m "refactor: delete generator.rs — all rendering via devcontainer module"
```

---

### Task 9: unwrap() cleanup

**Files:**
- Modify: `isolde-core/src/devcontainer.rs` (if any unwraps introduced)
- Modify: `isolde-core/src/template.rs` (2 production unwraps per sprint plan)

- [ ] **Step 1: Find production unwraps**

Run: `cargo clippy -- -W clippy::unwrap_used 2>&1 | grep unwrap`

- [ ] **Step 2: Fix each unwrap**

Replace `unwrap()` with `?`, `.unwrap_or_default()`, or `.map_err()` as appropriate.

- [ ] **Step 3: Verify**

Run: `cargo test && cargo clippy`
Expected: All clean.

- [ ] **Step 4: Commit**

```
git add -u
git commit -m "fix: replace production unwrap() calls with proper error handling"
```

---

### Task 10: Final verification

**Files:** None (read-only)

- [ ] **Step 1: Full test suite**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 2: Clippy clean**

Run: `cargo clippy`
Expected: No errors.

- [ ] **Step 3: Verify sync works end-to-end**

Run: `cargo run -- sync --help`
Expected: Help text displayed correctly.

- [ ] **Step 4: Verify diff works**

Run: `cargo run -- diff --help`
Expected: Help text displayed correctly.

- [ ] **Step 5: Update sprint plan progress**

Mark Session 5 as complete in the sprint plan. Update session 6 status if also complete.

---

## Summary

| Session | Tasks | Key Deliverable |
|---|---|---|
| 5 (today) | Tasks 1-6 | `devcontainer.rs` created, `sync.rs` rewritten as thin wrapper |
| 6 (tomorrow) | Tasks 7-10 | `diff.rs` rewritten, `generator.rs` deleted, unwrap cleanup |
