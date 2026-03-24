//! # Devcontainer artifact rendering
//!
//! Pure functions that generate devcontainer configuration files from an Isolde Config.
//! Used by `isolde sync` (CLI) and `isolde diff` (comparison).

use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{AgentOptionValue, Config};
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
        possible_paths.push(PathBuf::from(&home).join(".local/share/isolde/core/features"));
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

/// Render devcontainer.json content from configuration.
///
/// Builds a complete devcontainer.json with: common-utils, conditional Node.js,
/// language-specific features, proxy, agent feature, plugin manager, mounts,
/// and feature install order.
pub fn render_devcontainer_json(config: &Config, host_auth: &HostAuthInfo) -> Result<String> {
    let proxy = config.proxy();
    let plugins = config.plugins_vec();
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
    if let Some(proxy) = &proxy {
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
    if let Some(proxy) = &proxy {
        if let Some(h) = proxy.http() {
            agent_opts.insert(
                "http_proxy".to_string(),
                serde_json::Value::String(h.clone()),
            );
        }
        if let Some(h) = proxy.https() {
            agent_opts.insert(
                "https_proxy".to_string(),
                serde_json::Value::String(h.clone()),
            );
        }
    }
    let agent_feature_path = format!("./features/{}", config.agent_name());
    features.insert(agent_feature_path, serde_json::Value::Object(agent_opts));

    // Plugin manager feature
    if !plugins.is_empty() {
        let activate: Vec<&str> = plugins
            .iter()
            .filter(|p| p.activate)
            .map(|p| p.name.as_str())
            .collect();
        let deactivate: Vec<&str> = plugins
            .iter()
            .filter(|p| !p.activate)
            .map(|p| p.name.as_str())
            .collect();

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
    if proxy.is_some() {
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

/// List relative paths of all artifacts that `isolde sync` would generate.
///
/// Paths are relative to the project root.
/// The caller is responsible for checking which paths already exist on disk
/// to classify them as "would create" vs. "would modify".
pub fn expected_artifacts(_config: &Config) -> Vec<String> {
    let mut paths = vec![
        ".devcontainer/devcontainer.json".to_string(),
        ".devcontainer/Dockerfile".to_string(),
        ".claude/CLAUDE.md".to_string(),
    ];

    // Core feature directories — list known features that would be copied.
    // We list them statically since this function is pure (no filesystem access).
    for feature in &[
        "claude-code",
        "codex",
        "gemini",
        "aider",
        "proxy",
        "plugin-manager",
    ] {
        paths.push(format!(".devcontainer/features/{}", feature));
    }

    paths
}

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
        assert_eq!(
            fs::read_to_string(dst.join("file1.txt")).unwrap(),
            "content1"
        );
        assert!(dst.join("subdir/file2.txt").exists());
        assert_eq!(
            fs::read_to_string(dst.join("subdir/file2.txt")).unwrap(),
            "content2"
        );
    }

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

    #[test]
    fn test_expected_artifacts() {
        let config = minimal_config();
        let artifacts = expected_artifacts(&config);
        assert!(artifacts.contains(&".devcontainer/devcontainer.json".to_string()));
        assert!(artifacts.contains(&".devcontainer/Dockerfile".to_string()));
        assert!(artifacts.contains(&".claude/CLAUDE.md".to_string()));
    }
}
