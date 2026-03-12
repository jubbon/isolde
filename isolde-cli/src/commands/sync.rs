//! # Isolde sync command
//!
//! Generate devcontainer and Claude configuration from isolde.yaml.

use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use isolde_core::config::Config;
use isolde_core::{Error, Result};

/// Options for the sync command
#[derive(Debug, Clone)]
pub struct SyncOptions {
    /// Dry run - don't write files
    pub dry_run: bool,

    /// Force regeneration even if files exist
    pub force: bool,

    /// Current working directory
    pub cwd: PathBuf,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            dry_run: false,
            force: false,
            cwd: PathBuf::from("."),
        }
    }
}

/// Run the sync command
pub fn run(opts: SyncOptions) -> Result<()> {
    let config_path = opts.cwd.join("isolde.yaml");

    // Check if isolde.yaml exists
    if !config_path.exists() {
        return Err(Error::InvalidTemplate(
            "isolde.yaml not found. Run 'isolde init' first.".to_string(),
        ));
    }

    println!("{}", "🔄 Syncing Isolde configuration...".cyan());
    println!("{}", "─".repeat(50).dimmed());

    // Load and validate configuration
    print!("{} ", "Loading configuration...".dimmed());
    let config = Config::from_file(&config_path)?;
    println!("{}", "✔".green());

    // Create output directories
    let devcontainer_dir = opts.cwd.join(".devcontainer");
    let claude_dir = opts.cwd.join(".claude");
    let features_dir = devcontainer_dir.join("features");

    let project_dir = opts.cwd.join("project");

    if !opts.dry_run {
        fs::create_dir_all(&devcontainer_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        fs::create_dir_all(&claude_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        fs::create_dir_all(&features_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        if !project_dir.exists() {
            fs::create_dir_all(&project_dir)
                .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        }
    }

    // Generate devcontainer.json
    print!("{} ", "Generating devcontainer.json...".dimmed());
    let devcontainer_json = generate_devcontainer(&config)?;
    if !opts.dry_run {
        let output_path = devcontainer_dir.join("devcontainer.json");
        write_file(&output_path, &devcontainer_json, opts.force)?;
    }
    println!("{}", "✔".green());

    // Generate Dockerfile
    print!("{} ", "Generating Dockerfile...".dimmed());
    let dockerfile = generate_dockerfile(&config)?;
    if !opts.dry_run {
        let output_path = devcontainer_dir.join("Dockerfile");
        write_file(&output_path, &dockerfile, opts.force)?;
    }
    println!("{}", "✔".green());

    // Generate CLAUDE.md
    print!("{} ", "Generating CLAUDE.md...".dimmed());
    let claude_md = generate_claude_md(&config)?;
    if !opts.dry_run {
        let output_path = claude_dir.join("CLAUDE.md");
        write_file(&output_path, &claude_md, opts.force)?;
    }
    println!("{}", "✔".green());

    // Copy core features
    print!("{} ", "Copying core features...".dimmed());
    if !opts.dry_run {
        copy_core_features(&features_dir)?;
    }
    println!("{}", "✔".green());

    println!("{}", "─".repeat(50).dimmed());
    println!(
        "\n{} {}",
        "✨".green(),
        "Sync complete!".green().bold()
    );
    println!(
        "{}",
        "Run 'docker build -t <image-name> .devcontainer' to build the devcontainer.".dimmed()
    );

    Ok(())
}

/// Generate devcontainer.json from configuration
fn generate_devcontainer(config: &Config) -> Result<String> {
    let mut features = serde_json::Map::new();

    // Add common utils - userUid/userGid "automatic" makes devcontainers match the host user's UID/GID,
    // so bind-mounted directories owned by the host user appear with the correct owner inside the container.
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

    // Add Node.js (for Claude Code)
    features.insert(
        "ghcr.io/devcontainers/features/node:1".to_string(),
        serde_json::json!({
            "version": "lts",
            "nodeGypDependencies": true,
            "npxInstallCachedPackages": true
        }),
    );

    // Add language-specific features
    if let Some(runtime) = config.runtime() {
        match runtime.language() {
            "python" => {
                features.insert(
                    format!("ghcr.io/devcontainers/features/python:1"),
                    serde_json::json!({
                        "version": runtime.version(),
                        "installTools": true
                    }),
                );
            }
            "nodejs" | "javascript" => {
                features.insert(
                    format!("ghcr.io/devcontainers/features/node:1"),
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

    // Add proxy feature if configured
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

    // Add coding agent feature
    let mut agent_opts = serde_json::Map::new();
    agent_opts.insert("version".to_string(), serde_json::Value::String(config.agent_version().to_string()));
    for (key, value) in config.agent_options() {
        use isolde_core::config::AgentOptionValue;
        // devcontainer-feature.json declares all options as "type": "string",
        // so Map values must be serialized back to a comma-separated string.
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
        if let Some(h) = proxy.http() { agent_opts.insert("http_proxy".to_string(), serde_json::Value::String(h.clone())); }
        if let Some(h) = proxy.https() { agent_opts.insert("https_proxy".to_string(), serde_json::Value::String(h.clone())); }
    }
    let agent_feature_path = format!("./features/{}", config.agent_name());
    features.insert(agent_feature_path, serde_json::Value::Object(agent_opts));

    // Add plugin manager feature if plugins are configured
    let plugins = config.plugins_vec();
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

    // Build feature install order
    let mut override_order = vec![];
    if config.proxy().is_some() {
        override_order.push("./features/proxy");
    }
    override_order.push("./features/claude-code");
    if !plugins.is_empty() {
        override_order.push("./features/plugin-manager");
    }

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
                "extensions": [
                    "anthropic.claude-code"
                ],
                "settings": {
                    "terminal.integrated.defaultProfile.linux": "bash"
                }
            }
        },
        "mounts": [
            format!("source=./project,target=/workspaces/{},type=bind,consistency=cached", config.name),
            format!("source=./.claude,target=/workspaces/{}/.claude,type=bind,consistency=cached", config.name),
            "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind,consistency=cached".to_string(),
            "source=${localEnv:HOME}/.claude.json,target=/home/${localEnv:USER}/.claude.json,type=bind,consistency=cached".to_string(),
            "source=${localEnv:HOME}/.config/devcontainer/machine-id,target=/etc/machine-id,type=bind,consistency=cached".to_string()
        ],
        "remoteUser": "${localEnv:USER}",
        "workspaceFolder": format!("/workspaces/{}", config.name)
    });

    serde_json::to_string_pretty(&devcontainer)
        .map_err(|e| Error::Other(format!("Failed to serialize devcontainer.json: {}", e)))
}

/// Generate Dockerfile from configuration
fn generate_dockerfile(config: &Config) -> Result<String> {
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

/// Generate CLAUDE.md from configuration
fn generate_claude_md(config: &Config) -> Result<String> {
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

/// Write file to disk, handling force option
fn write_file(path: &Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        println!("  Skipped {} (already exists)", path.display().to_string().yellow());
        return Ok(());
    }

    fs::write(path, content).map_err(|e| {
        Error::FileError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write {}: {}", path.display(), e),
        ))
    })?;

    Ok(())
}

/// Copy core features from the isolde repository
fn copy_core_features(features_dir: &Path) -> Result<()> {
    let core_features = match find_core_features_dir() {
        Ok(path) => path,
        Err(_) => {
            println!("{}", "⚠ Core features not found - skipping feature copy".yellow());
            println!("{}", "  Features should be manually installed or bundled with the binary.".dimmed());
            return Ok(());
        }
    };

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

            let dest = features_dir.join(feature_name);

            if dest.exists() {
                fs::remove_dir_all(&dest).map_err(|e| {
                    Error::FileError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to remove {}: {}", dest.display(), e),
                    ))
                })?;
            }

            copy_dir_recursive(&feature_path, &dest)?;
        }
    }

    Ok(())
}

/// Find the core features directory
fn find_core_features_dir() -> Result<PathBuf> {
    let mut possible_paths = vec![];

    // 1. Check environment variable first
    if let Ok(env_path) = std::env::var("ISOLDE_CORE_FEATURES") {
        possible_paths.push(PathBuf::from(env_path));
    }

    // 2. Try relative paths from current directory (for development)
    possible_paths.push(PathBuf::from("core/features"));
    possible_paths.push(PathBuf::from("../core/features"));
    possible_paths.push(PathBuf::from("../../core/features"));

    // 3. Try to find relative to the isolde executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // Installed via make install to ~/.local/bin
            // Features should be in ~/.local/share/isolde/core/features
            if let Some(prefix) = exe_dir.parent() {
                let share_path = prefix.join("share").join("isolde").join("core").join("features");
                possible_paths.push(share_path);
            }

            // Or features might be next to the executable (for development)
            possible_paths.push(exe_dir.join("core").join("features"));
            possible_paths.push(exe_dir.join("../core/features").canonicalize().unwrap_or_else(|_| PathBuf::from("../core/features")));
        }
    }

    // 4. Check XDG data directories
    if let Ok(home) = std::env::var("HOME") {
        possible_paths.push(PathBuf::from(home.clone()).join(".local").join("share").join("isolde").join("core").join("features"));
        possible_paths.push(PathBuf::from(home).join(".isolde").join("core").join("features"));
    }

    // 5. Try /usr/local/share for system-wide installation
    possible_paths.push(PathBuf::from("/usr/local/share/isolde/core/features"));
    possible_paths.push(PathBuf::from("/opt/isolde/core/features"));

    for path in possible_paths {
        if path.exists() {
            return Ok(path);
        }
    }

    Err(Error::PathNotFound(PathBuf::from("core/features")))
}

/// Copy a directory recursively
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
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
                    format!("Failed to copy {} to {}: {}", src_path.display(), dst_path.display(), e),
                ))
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_dockerfile() {
        let config_yaml = r#"
version: "0.1"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config = Config::from_str(config_yaml).unwrap();
        let dockerfile = generate_dockerfile(&config).unwrap();
        assert!(dockerfile.contains("ARG BASE_IMAGE=ubuntu:latest"));
        assert!(dockerfile.contains("FROM ${BASE_IMAGE}"));
        assert!(dockerfile.contains("ARG USERNAME=user"));
    }

    #[test]
    fn test_generate_claude_md() {
        let config_yaml = r#"
version: "0.1"
name: test-project
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
runtime:
  language: python
  version: "3.12"
  package_manager: uv
"#;
        let config = Config::from_str(config_yaml).unwrap();
        let claude_md = generate_claude_md(&config).unwrap();
        assert!(claude_md.contains("test-project"));
        assert!(claude_md.contains("python 3.12"));
    }

    #[test]
    fn test_generate_devcontainer() {
        let config_yaml = r#"
version: "0.1"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config = Config::from_str(config_yaml).unwrap();
        let devcontainer = generate_devcontainer(&config).unwrap();
        assert!(devcontainer.contains("features"));
        assert!(devcontainer.contains("claude-code"));
    }

    #[test]
    fn test_sync_options_default() {
        let opts = SyncOptions::default();
        assert!(!opts.dry_run);
        assert!(!opts.force);
    }
}
