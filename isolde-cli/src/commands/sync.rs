//! # Isolde sync command
//!
//! Generate devcontainer and Claude configuration from isolde.yaml.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use colored::Colorize;
use isolde_core::config::{Config, GitGeneratedHandling};
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

    if !opts.dry_run {
        fs::create_dir_all(&devcontainer_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create .devcontainer: {}", e))))?;
        fs::create_dir_all(&claude_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create .claude: {}", e))))?;
        fs::create_dir_all(&features_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create features: {}", e))))?;
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

    // Initialize git repos
    print!("{} ", "Initializing git repositories...".dimmed());
    if !opts.dry_run {
        init_git_repos(&opts.cwd, &config)?;
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

    // Add common utils
    features.insert(
        "ghcr.io/devcontainers/features/common-utils:2".to_string(),
        serde_json::json!({
            "installZsh": false,
            "installOhMyZsh": false,
            "upgradePackages": false
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
    if let Some(runtime) = &config.runtime {
        match runtime.language.as_str() {
            "python" => {
                features.insert(
                    format!("ghcr.io/devcontainers/features/python:1"),
                    serde_json::json!({
                        "version": runtime.version,
                        "installTools": true
                    }),
                );
            }
            "nodejs" | "javascript" => {
                features.insert(
                    format!("ghcr.io/devcontainers/features/node:1"),
                    serde_json::json!({
                        "version": runtime.version
                    }),
                );
            }
            "rust" => {
                features.insert(
                    "ghcr.io/devcontainers/features/rust:1".to_string(),
                    serde_json::json!({
                        "version": runtime.version
                    }),
                );
            }
            "go" => {
                features.insert(
                    "ghcr.io/devcontainers/features/go:1".to_string(),
                    serde_json::json!({
                        "version": runtime.version
                    }),
                );
            }
            _ => {}
        }
    }

    // Add proxy feature if configured
    if config.proxy.is_some() {
        let proxy = config.proxy.as_ref().unwrap();
        features.insert(
            "./features/proxy".to_string(),
            serde_json::json!({
                "http_proxy": proxy.http,
                "https_proxy": proxy.https,
                "no_proxy": proxy.no_proxy,
                "enabled": true
            }),
        );
    }

    // Add Claude Code feature
    let claude_models = if config.claude.models.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&config.claude.models).ok())
    };

    features.insert(
        "./features/claude-code".to_string(),
        serde_json::json!({
            "version": config.claude.version,
            "provider": config.claude.provider,
            "models": claude_models,
            "http_proxy": config.proxy.as_ref().and_then(|p| p.http.clone()),
            "https_proxy": config.proxy.as_ref().and_then(|p| p.https.clone())
        }),
    );

    // Add plugin manager feature if plugins are configured
    if !config.plugins.is_empty() {
        let activate: Vec<&str> = config
            .plugins
            .iter()
            .filter(|p| p.activate)
            .map(|p| p.name.as_str())
            .collect();
        let deactivate: Vec<&str> = config
            .plugins
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
    if config.proxy.is_some() {
        override_order.push("./features/proxy");
    }
    override_order.push("./features/claude-code");
    if !config.plugins.is_empty() {
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
            "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind,consistency=cached",
            "source=${localEnv:HOME}/.claude.json,target=/home/${localEnv:USER}/.claude.json,type=bind,consistency=cached",
            "source=${localEnv:HOME}/.config/devcontainer/machine-id,target=/etc/machine-id,type=bind,consistency=cached"
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
ARG USER_UID=1000
ARG USER_GID=1000

# Set DEBIAN_FRONTEND for non-interactive apt
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    wget \
    vim \
    jq \
    build-essential \
    ca-certificates \
    gosu \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspaces

# Pre-install sudo
RUN apt-get update && apt-get install -y sudo && rm -rf /var/lib/apt/lists/*

# Create user with sudo access
RUN groupadd --gid ${{USER_GID}} ${{USERNAME}} \
    && useradd --uid ${{USER_UID}} --gid ${{USERNAME}} --shell /bin/bash --create-home ${{USERNAME}} \
    && echo "${{USERNAME}} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set ownership for workspace directory
RUN chown -R ${{USERNAME}}:${{USERNAME}} /workspaces

USER ${{USERNAME}}
"#,
        config.docker.image
    );

    Ok(dockerfile)
}

/// Generate CLAUDE.md from configuration
fn generate_claude_md(config: &Config) -> Result<String> {
    let mut content = format!(
        r#"# Claude Code Configuration for {}

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**{}** - Version {}

This is an Isolde-managed isolated development environment.

## Configuration

- **Docker Image**: {}
- **Claude Provider**: {}
- **Workspace Directory**: {}
"#,
        config.name,
        config.name,
        config.version,
        config.docker.image,
        config.claude.provider,
        config.workspace.dir
    );

    if let Some(runtime) = &config.runtime {
        content.push_str(&format!(
            r#"
## Runtime Environment

- **Language**: {} {}
- **Package Manager**: {}
"#,
            runtime.language, runtime.version, runtime.package_manager
        ));

        if !runtime.tools.is_empty() {
            content.push_str("\n### Tools\n");
            for tool in &runtime.tools {
                content.push_str(&format!("- {}\n", tool));
            }
        }
    }

    if !config.plugins.is_empty() {
        content.push_str("\n## Claude Plugins\n");
        for plugin in &config.plugins {
            let status = if plugin.activate { "✓" } else { "✗" };
            content.push_str(&format!(
                "- {} {} (from marketplace: {})\n",
                status, plugin.name, plugin.marketplace
            ));
        }
    }

    if !config.marketplaces.is_empty() {
        content.push_str("\n## Marketplaces\n");
        for (name, marketplace) in &config.marketplaces {
            content.push_str(&format!("- {}: {}\n", name, marketplace.url));
        }
    }

    if config.proxy.is_some() {
        let proxy = config.proxy.as_ref().unwrap();
        content.push_str(
            r#"
## Proxy Configuration

This project is configured to work with a corporate proxy.
"#,
        );
        if let Some(http) = &proxy.http {
            content.push_str(&format!("- HTTP Proxy: {}\n", http));
        }
        if let Some(https) = &proxy.https {
            content.push_str(&format!("- HTTPS Proxy: {}\n", https));
        }
        if let Some(no_proxy) = &proxy.no_proxy {
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
        println!(
            "{}",
            format!("  Skipped {} (already exists)", path.display()).yellow()
        );
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
    // Get the path to the core features directory
    // In a real implementation, this would be determined from the isolde installation
    let core_features = match find_core_features_dir() {
        Ok(path) => path,
        Err(_) => {
            // Features not found - this is okay, they're optional
            println!("{}", "⚠ Core features not found - skipping feature copy".yellow());
            println!("{}", "  Features should be manually installed or bundled with the binary.".dimmed());
            return Ok(());
        }
    };

    // Copy each feature directory
    for entry in fs::read_dir(&core_features)
        .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read core features: {}", e))))?
    {
        let entry = entry?;
        let feature_path = entry.path();

        if feature_path.is_dir() {
            let feature_name = feature_path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| Error::Other("Invalid feature name".to_string()))?;

            let dest = features_dir.join(feature_name);

            // Remove existing directory if force
            if dest.exists() {
                fs::remove_dir_all(&dest).map_err(|e| {
                    Error::FileError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to remove {}: {}", dest.display(), e),
                    ))
                })?;
            }

            // Copy the feature directory
            copy_dir_recursive(&feature_path, &dest)?;
        }
    }

    Ok(())
}

/// Find the core features directory
fn find_core_features_dir() -> Result<PathBuf> {
    // Try several possible locations
    let possible_paths = vec![
        PathBuf::from("core/features"),
        PathBuf::from("../core/features"),
        PathBuf::from("../../core/features"),
        // In production, we would also check the isolde installation directory
    ];

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
        .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read {}: {}", src.display(), e))))?
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

/// Initialize git repositories for project and devcontainer
fn init_git_repos(cwd: &Path, config: &Config) -> Result<()> {
    use std::process::Command;

    // Determine the project directory
    let project_dir = if config.workspace.dir == "." {
        cwd.to_path_buf()
    } else {
        cwd.join(&config.workspace.dir)
    };

    // Create project directory if it doesn't exist
    if !project_dir.exists() {
        fs::create_dir_all(&project_dir).map_err(|e| {
            Error::FileError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create project directory {}: {}", project_dir.display(), e),
            ))
        })?;
    }

    // Initialize project git repo if it doesn't exist
    let project_git = project_dir.join(".git");
    if !project_git.exists() {
        print!(
            "{}",
            format!("  Initializing git repo in {}...\n", project_dir.display()).dimmed()
        );
        Command::new("git")
            .args(["init"])
            .current_dir(&project_dir)
            .status()
            .map_err(|e| Error::Other(format!("Failed to initialize git repo: {}", e)))?;
    }

    // Initialize devcontainer git repo
    let devcontainer_dir = cwd.join(".devcontainer");
    let devcontainer_git = devcontainer_dir.join(".git");
    if !devcontainer_git.exists() {
        print!(
            "{}",
            format!("  Initializing git repo in {}...\n", devcontainer_dir.display()).dimmed()
        );
        Command::new("git")
            .args(["init"])
            .current_dir(&devcontainer_dir)
            .status()
            .map_err(|e| Error::Other(format!("Failed to initialize git repo: {}", e)))?;
    }

    // Handle generated files based on git config
    match config.git.generated {
        GitGeneratedHandling::Ignored => {
            // Add generated files to .gitignore
            let gitignore_path = cwd.join(".gitignore");
            let mut gitignore_content = String::new();
            if gitignore_path.exists() {
                gitignore_content = fs::read_to_string(&gitignore_path).unwrap_or_default();
            }

            let entries = vec![".devcontainer/", ".claude/"];
            for entry in entries {
                if !gitignore_content.contains(entry) {
                    gitignore_content.push_str(entry);
                    gitignore_content.push('\n');
                }
            }

            fs::write(&gitignore_path, gitignore_content).map_err(|e| {
                Error::FileError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write .gitignore: {}", e),
                ))
            })?;
        }
        GitGeneratedHandling::Committed => {
            // Files are committed as-is
        }
        GitGeneratedHandling::LinguistGenerated => {
            // Add to gitattributes
            let gitattributes_path = cwd.join(".gitattributes");
            let mut attrs_content = String::new();
            if gitattributes_path.exists() {
                attrs_content = fs::read_to_string(&gitattributes_path).unwrap_or_default();
            }

            let entries = vec![
                ".devcontainer/ linguist-generated",
                ".claude/ linguist-generated",
            ];
            for entry in entries {
                if !attrs_content.contains(entry) {
                    attrs_content.push_str(entry);
                    attrs_content.push('\n');
                }
            }

            fs::write(&gitattributes_path, attrs_content).map_err(|e| {
                Error::FileError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write .gitattributes: {}", e),
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
name: test
version: 1.0.0
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
name: test-project
version: 1.0.0
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
name: test
version: 1.0.0
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
