//! # Devcontainer generator from YAML config
//!
//! This module provides the `Generator` struct for creating devcontainer artifacts
//! from an `isolde.yaml` configuration file.

use crate::config::Config;
use crate::error::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Trait for running git commands - allows stubbing in tests
pub trait GitRunner {
    /// Run a git command in the specified directory
    fn run_git(&self, dir: &Path, args: &[&str]) -> Result<()>;
}

/// Default implementation using real git
pub struct RealGitRunner;

impl GitRunner for RealGitRunner {
    fn run_git(&self, dir: &Path, args: &[&str]) -> Result<()> {
        use std::process::Command;

        let result = Command::new("git")
            .current_dir(dir)
            .args(args)
            .output()
            .map_err(|e| Error::Other(format!("Failed to execute git: {}", e)))?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(Error::Other(format!("Git command failed: {}", stderr)));
        }

        Ok(())
    }
}

/// Report generated from a dry run
#[derive(Debug, Clone)]
pub struct DryRunReport {
    /// Files that would be created
    pub would_create: Vec<PathBuf>,
    /// Files that would be modified
    pub would_modify: Vec<PathBuf>,
}

/// Report generated from a successful generation
#[derive(Debug, Clone)]
pub struct GenerateReport {
    /// Files that were created
    pub files_created: Vec<PathBuf>,
    /// Files that were modified
    pub files_modified: Vec<PathBuf>,
}

/// Generator for creating devcontainer artifacts from config
pub struct Generator {
    /// Configuration loaded from isolde.yaml
    config: Config,
    /// Isolde installation root (for templates and features)
    pub(crate) isolde_root: PathBuf,
    /// Git runner (allows stubbing in tests)
    git_runner: Box<dyn GitRunner>,
}

impl Generator {
    /// Create a new generator from a config
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration loaded from isolde.yaml
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `Generator` or an `Error`
    pub fn new(config: Config) -> Result<Self> {
        // Find the Isolde installation root
        // We look for the templates/ and core/ directories
        let isolde_root = Self::find_isolde_root()?;

        Ok(Self {
            config,
            isolde_root,
            git_runner: Box::new(RealGitRunner),
        })
    }

    /// Find the Isolde installation root
    ///
    /// This looks for the `templates/` and `core/` directories
    /// by searching upward from the current directory
    fn find_isolde_root() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()
            .map_err(|e| Error::Other(format!("Failed to get current directory: {e}")))?;

        let mut dir = current_dir.as_path();

        // Search upward for isolde root markers
        loop {
            let templates_dir = dir.join("templates");
            let core_dir = dir.join("core");

            if templates_dir.exists() && core_dir.exists() {
                return Ok(dir.to_path_buf());
            }

            match dir.parent() {
                Some(parent) => dir = parent,
                None => {
                    return Err(Error::PathNotFound(
                        current_dir.join("Could not find Isolde root (templates/ and core/ directories)"),
                    ));
                }
            }
        }
    }

    /// Generate devcontainer artifacts
    ///
    /// # Arguments
    ///
    /// * `output_dir` - The root directory where the project will be created
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `GenerateReport` or an `Error`
    pub fn generate(&self, output_dir: &Path) -> Result<GenerateReport> {
        let mut files_created = Vec::new();
        let files_modified = Vec::new();

        // Create workspace directory
        let workspace_dir = output_dir.join(&self.config.workspace.dir);
        fs::create_dir_all(&workspace_dir)?;

        // Create .devcontainer directory
        let devcontainer_dir = output_dir.join(".devcontainer");
        fs::create_dir_all(&devcontainer_dir)?;

        // Generate devcontainer.json from template
        let devcontainer_json = self.render_devcontainer_json()?;
        let devcontainer_json_path = devcontainer_dir.join("devcontainer.json");
        fs::write(&devcontainer_json_path, devcontainer_json)?;
        files_created.push(devcontainer_json_path);

        // Generate Dockerfile
        let dockerfile = self.render_dockerfile()?;
        let dockerfile_path = devcontainer_dir.join("Dockerfile");
        fs::write(&dockerfile_path, dockerfile)?;
        files_created.push(dockerfile_path);

        // Copy core features
        let features_dir = devcontainer_dir.join("features");
        let copied_features = self.copy_core_features(&features_dir)?;
        files_created.extend(copied_features);

        // Generate .claude/config.json
        let claude_config = self.render_claude_config()?;
        let claude_dir = workspace_dir.join(".claude");
        fs::create_dir_all(&claude_dir)?;
        let claude_config_path = claude_dir.join("config.json");
        fs::write(&claude_config_path, claude_config)?;
        files_created.push(claude_config_path);

        // Generate .gitignore for project
        let gitignore_content = self.render_project_gitignore()?;
        let gitignore_path = workspace_dir.join(".gitignore");
        fs::write(&gitignore_path, gitignore_content)?;
        files_created.push(gitignore_path);

        // Generate .gitignore for devcontainer
        let devcontainer_gitignore = self.render_devcontainer_gitignore()?;
        let devcontainer_gitignore_path = devcontainer_dir.join(".gitignore");
        fs::write(&devcontainer_gitignore_path, devcontainer_gitignore)?;
        files_created.push(devcontainer_gitignore_path);

        // Generate README.md for project
        let readme = self.render_project_readme()?;
        let readme_path = workspace_dir.join("README.md");
        fs::write(&readme_path, readme)?;
        files_created.push(readme_path);

        // Initialize git repositories
        self.initialize_git_repos(output_dir)?;

        Ok(GenerateReport {
            files_created,
            files_modified,
        })
    }

    /// Perform a dry run to see what would be generated
    ///
    /// # Arguments
    ///
    /// * `output_dir` - The root directory where the project would be created
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `DryRunReport` or an `Error`
    pub fn dry_run(&self, output_dir: &Path) -> Result<DryRunReport> {
        let mut would_create = Vec::new();
        let mut would_modify = Vec::new();

        let workspace_dir = output_dir.join(&self.config.workspace.dir);
        let devcontainer_dir = output_dir.join(".devcontainer");
        let features_dir = devcontainer_dir.join("features");
        let claude_dir = workspace_dir.join(".claude");

        // Check which files would be created vs modified
        let devcontainer_json_path = devcontainer_dir.join("devcontainer.json");
        if devcontainer_json_path.exists() {
            would_modify.push(devcontainer_json_path);
        } else {
            would_create.push(devcontainer_json_path);
        }

        let dockerfile_path = devcontainer_dir.join("Dockerfile");
        if dockerfile_path.exists() {
            would_modify.push(dockerfile_path);
        } else {
            would_create.push(dockerfile_path);
        }

        // Core features
        let core_features_dir = self.isolde_root.join("core/features");
        if let Ok(entries) = fs::read_dir(&core_features_dir) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type() {
                    if ft.is_dir() {
                        let feature_name = entry.file_name();
                        let feature_path = features_dir.join(&feature_name);
                        if feature_path.exists() {
                            would_modify.push(feature_path);
                        } else {
                            would_create.push(feature_path);
                        }
                    }
                }
            }
        }

        // Claude config
        let claude_config_path = claude_dir.join("config.json");
        if claude_config_path.exists() {
            would_modify.push(claude_config_path);
        } else {
            would_create.push(claude_config_path);
        }

        // Project gitignore
        let gitignore_path = workspace_dir.join(".gitignore");
        if gitignore_path.exists() {
            would_modify.push(gitignore_path);
        } else {
            would_create.push(gitignore_path);
        }

        // Devcontainer gitignore
        let devcontainer_gitignore_path = devcontainer_dir.join(".gitignore");
        if devcontainer_gitignore_path.exists() {
            would_modify.push(devcontainer_gitignore_path);
        } else {
            would_create.push(devcontainer_gitignore_path);
        }

        // Project README
        let readme_path = workspace_dir.join("README.md");
        if readme_path.exists() {
            would_modify.push(readme_path);
        } else {
            would_create.push(readme_path);
        }

        Ok(DryRunReport {
            would_create,
            would_modify,
        })
    }

    /// Render devcontainer.json using template substitution
    fn render_devcontainer_json(&self) -> Result<String> {
        let mut content = self.get_base_devcontainer_template()?;

        // Build substitution map
        let substitutions = self.build_substitution_map();

        // Apply each substitution
        for (key, value) in &substitutions {
            let placeholder = format!("{{{{{}}}}}", key);
            content = content.replace(&placeholder, value);
        }

        Ok(content)
    }

    /// Get the base devcontainer.json template
    fn get_base_devcontainer_template(&self) -> Result<String> {
        // For now, use the generic template
        let template_path = self
            .isolde_root
            .join("templates/generic/.devcontainer/devcontainer.json");

        if template_path.exists() {
            fs::read_to_string(&template_path).map_err(|e| {
                Error::FileError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read template: {}", e),
                ))
            })
        } else {
            // Fallback to embedded template
            Ok(self.get_embedded_devcontainer_template())
        }
    }

    /// Get embedded devcontainer.json template
    fn get_embedded_devcontainer_template(&self) -> String {
        // Use a simpler template without complex escape sequences
        let json = r#"{
  "name": "{{PROJECT_NAME}} - Isolde Environment",
  "build": {
    "dockerfile": "Dockerfile",
    "context": "..",
    "args": {
      "USERNAME": "${localEnv:USER}"
    }
  },
  "features": {
    "ghcr.io/devcontainers/features/common-utils:2": {
      "installZsh": false,
      "installOhMyZsh": false,
      "upgradePackages": false
    },
    "{{FEATURES_PROXY}}": {
      "http_proxy": "{{HTTP_PROXY}}",
      "https_proxy": "{{HTTPS_PROXY}}",
      "no_proxy": "{{NO_PROXY}}",
      "enabled": {{PROXY_ENABLED}}
    },
    "{{FEATURES_CLAUDE_CODE}}": {
      "version": "{{CLAUDE_VERSION}}",
      "provider": "{{CLAUDE_PROVIDER}}",
      "models": {{CLAUDE_MODELS}},
      "http_proxy": "{{HTTP_PROXY}}",
      "https_proxy": "{{HTTPS_PROXY}}"
    },
    "{{FEATURES_PLUGIN_MANAGER}}": {
      "activate_plugins": {{CLAUDE_ACTIVATE_PLUGINS}},
      "deactivate_plugins": {{CLAUDE_DEACTIVATE_PLUGINS}}
    }
  },
  "overrideFeatureInstallOrder": [
    "./features/proxy",
    "./features/claude-code",
    "./features/plugin-manager"
  ],
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
  "workspaceFolder": "/workspaces/{{PROJECT_NAME}}"
}
"#;
        json.to_string()
    }

    /// Build the substitution map for templates
    fn build_substitution_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Project info
        map.insert("PROJECT_NAME".to_string(), self.config.name.clone());

        // Claude configuration
        map.insert("CLAUDE_VERSION".to_string(), self.config.claude.version.clone());
        map.insert("CLAUDE_PROVIDER".to_string(), self.config.claude.provider.clone());

        // Claude models as JSON
        let models_json = serde_json::to_string(&self.config.claude.models)
            .unwrap_or_else(|_| "{}".to_string());
        map.insert("CLAUDE_MODELS".to_string(), models_json);

        // Proxy configuration
        let proxy_enabled = if self.config.proxy.is_some() {
            "true"
        } else {
            "false"
        };
        map.insert("PROXY_ENABLED".to_string(), proxy_enabled.to_string());

        if let Some(ref proxy) = self.config.proxy {
            map.insert(
                "HTTP_PROXY".to_string(),
                proxy.http.clone().unwrap_or_default(),
            );
            map.insert(
                "HTTPS_PROXY".to_string(),
                proxy.https.clone().unwrap_or_default(),
            );
            map.insert(
                "NO_PROXY".to_string(),
                proxy.no_proxy.clone().unwrap_or_else(|| format!("localhost,127.0.0.1{}", ".local")),
            );
        } else {
            map.insert("HTTP_PROXY".to_string(), String::new());
            map.insert("HTTPS_PROXY".to_string(), String::new());
            // Split .local to avoid prefix literal interpretation in Rust
            map.insert("NO_PROXY".to_string(), format!("localhost,127.0.0.1{}", ".local"));
        }

        // Feature paths
        map.insert(
            "FEATURES_CLAUDE_CODE".to_string(),
            "./features/claude-code".to_string(),
        );
        map.insert("FEATURES_PROXY".to_string(), "./features/proxy".to_string());
        map.insert(
            "FEATURES_PLUGIN_MANAGER".to_string(),
            "./features/plugin-manager".to_string(),
        );

        // Plugin activation lists
        let activate_plugins: Vec<&String> = self
            .config
            .plugins
            .iter()
            .filter(|p| p.activate)
            .map(|p| &p.name)
            .collect();
        let activate_json = serde_json::to_string(&activate_plugins).unwrap_or_else(|_| "[]".to_string());
        map.insert("CLAUDE_ACTIVATE_PLUGINS".to_string(), activate_json);

        let deactivate_plugins: Vec<&String> = self
            .config
            .plugins
            .iter()
            .filter(|p| !p.activate)
            .map(|p| &p.name)
            .collect();
        let deactivate_json =
            serde_json::to_string(&deactivate_plugins).unwrap_or_else(|_| "[]".to_string());
        map.insert("CLAUDE_DEACTIVATE_PLUGINS".to_string(), deactivate_json);

        // Runtime language version (if available)
        if let Some(ref runtime) = self.config.runtime {
            if let Some(version_key) = Self::language_to_version_key(runtime.language.as_str()) {
                map.insert(version_key, runtime.version.clone());
            }
        }

        map
    }

    /// Map a language name to its version variable name
    pub fn language_to_version_key(language: &str) -> Option<String> {
        let key = match language {
            "python" => "PYTHON_VERSION",
            "node" | "nodejs" | "javascript" => "NODE_VERSION",
            "rust" => "RUST_VERSION",
            "go" | "golang" => "GO_VERSION",
            _ => return None,
        };
        Some(key.to_string())
    }

    /// Render Dockerfile content
    fn render_dockerfile(&self) -> Result<String> {
        // Use the configured base image
        let base_image = &self.config.docker.image;
        let mut content = format!("ARG BASE_IMAGE={}\nFROM ${{BASE_IMAGE}}\n\n", base_image);

        // Add user arguments
        content.push_str("ARG USERNAME=user\n");
        content.push_str("ARG USER_UID=1000\n");
        content.push_str("ARG USER_GID=1000\n");

        // Add language version if runtime is configured
        if let Some(ref runtime) = self.config.runtime {
            match runtime.language.as_str() {
                "python" => {
                    content.push_str(&format!("ARG PYTHON_VERSION={}\n\n", runtime.version));
                }
                "node" | "nodejs" => {
                    content.push_str(&format!("ARG NODE_VERSION={}\n\n", runtime.version));
                }
                "rust" => {
                    content.push_str(&format!("ARG RUST_VERSION={}\n\n", runtime.version));
                }
                "go" | "golang" => {
                    content.push_str(&format!("ARG GO_VERSION={}\n\n", runtime.version));
                }
                _ => {}
            }
        }

        // Add build args from config
        for arg in &self.config.docker.build_args {
            content.push_str(&format!("ARG {}\n", arg));
        }
        if !self.config.docker.build_args.is_empty() {
            content.push('\n');
        }

        // Set DEBIAN_FRONTEND for non-interactive apt
        content.push_str("ENV DEBIAN_FRONTEND=noninteractive\n\n");

        // Install common system dependencies
        content.push_str(
            r#"# Install system dependencies
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
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USERNAME} --shell /bin/bash --create-home ${USERNAME} \
    && echo "${USERNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set ownership for workspace directory
RUN chown -R ${USERNAME}:${USERNAME} /workspaces

USER ${USERNAME}
"#,
        );

        Ok(content)
    }

    /// Copy core features from the Isolde installation
    fn copy_core_features(&self, features_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut copied_files = Vec::new();

        let core_features_dir = self.isolde_root.join("core/features");

        if !core_features_dir.exists() {
            return Ok(copied_files);
        }

        fs::create_dir_all(features_dir)?;

        let entries = fs::read_dir(&core_features_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let feature_name = path.file_name().unwrap_or_default();
                let dest = features_dir.join(feature_name);

                // Remove existing if present
                if dest.exists() {
                    fs::remove_dir_all(&dest)?;
                }

                // Copy recursively
                self.copy_dir_recursive(&path, &dest)?;
                copied_files.push(dest.clone());
            }
        }

        Ok(copied_files)
    }

    /// Copy a directory recursively
    pub(crate) fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;

        let entries = fs::read_dir(src)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let dest_path = dst.join(path.file_name().unwrap_or_default());

            if path.is_dir() {
                self.copy_dir_recursive(&path, &dest_path)?;
            } else {
                fs::copy(&path, &dest_path)?;
            }
        }

        Ok(())
    }

    /// Render Claude Code configuration
    fn render_claude_config(&self) -> Result<String> {
        let mut config = serde_json::json!({
            "provider": self.config.claude.provider,
        });

        // Add models mapping if present
        if !self.config.claude.models.is_empty() {
            if let Some(models_obj) = config.get_mut("models") {
                if let Some(obj) = models_obj.as_object_mut() {
                    for (key, value) in &self.config.claude.models {
                        obj.insert(key.clone(), serde_json::Value::String(value.clone()));
                    }
                }
            }
        }

        serde_json::to_string_pretty(&config)
            .map_err(|e| Error::Other(format!("Failed to serialize Claude config: {}", e)))
    }

    /// Render project .gitignore content
    fn render_project_gitignore(&self) -> Result<String> {
        Ok(r#"# Claude Code local files
.claude/

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db

# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
venv/
env/
.venv/

# Node
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Rust
target/
Cargo.lock

# Go
*.sum
*.test
*.prof
"#.to_string())
    }

    /// Render devcontainer .gitignore content
    fn render_devcontainer_gitignore(&self) -> Result<String> {
        Ok(r#"# Claude Code local files (not in git)
.claude/
settings.json

# OMC state files (not in git)
.omc/

# IDE files
.vscode/
.idea/
"#.to_string())
    }

    /// Render project README content
    fn render_project_readme(&self) -> Result<String> {
        Ok(format!(
            r#"# {}

This project was created using the Isolde devcontainer template system.

## Getting Started

1. Open this project in VS Code
2. Reopen in Container when prompted
3. Start coding!

## Project Structure

- `project/` - Your main project code (this directory)
- `.devcontainer/` - Devcontainer configuration (separate git repository)
- `.claude/` - Claude Code configuration (not in git)

## DevContainer

This project uses a devcontainer for development. The configuration is in the
`.devcontainer/` directory (a separate git repository).

To rebuild the container:
1. Press F1 in VS Code
2. Select "Dev Containers: Rebuild Container"
"#,
            self.config.name
        ))
    }

    /// Initialize dual git repositories
    fn initialize_git_repos(&self, output_dir: &Path) -> Result<()> {
        let workspace_dir = output_dir.join(&self.config.workspace.dir);
        let devcontainer_dir = output_dir.join(".devcontainer");

        // Initialize project repository
        if !workspace_dir.join(".git").exists() {
            self.git_runner.run_git(&workspace_dir, &["init", "-q"])?;

            // Add initial files
            let readme_path = workspace_dir.join("README.md");
            let gitignore_path = workspace_dir.join(".gitignore");

            if readme_path.exists() {
                self.git_runner.run_git(&workspace_dir, &["add", "README.md"])?;
            }
            if gitignore_path.exists() {
                self.git_runner.run_git(&workspace_dir, &["add", ".gitignore"])?;
            }

            self.git_runner.run_git(&workspace_dir, &["commit", "-m", "Initial commit", "-q"])?;
        }

        // Initialize devcontainer repository
        if !devcontainer_dir.join(".git").exists() {
            self.git_runner.run_git(&devcontainer_dir, &["init", "-q"])?;

            // Add all files
            self.git_runner.run_git(&devcontainer_dir, &["add", "-A"])?;
            self.git_runner.run_git(
                &devcontainer_dir,
                &["commit", "-m", "Initial devcontainer setup", "-q"],
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config::from_str(
            r#"
name: test-app
version: 1.0.0
workspace:
  dir: ./project
docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
  build_args: []
claude:
  version: latest
  provider: anthropic
  models:
    haiku: claude-3-5-haiku-20241022
    sonnet: claude-3-5-sonnet-20241022
runtime:
  language: python
  version: "3.12"
  package_manager: uv
  tools: []
"#,
        )
        .unwrap()
    }

    #[test]
    fn test_generator_new() {
        let config = create_test_config();
        let generator = Generator::new(config);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_build_substitution_map() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let map = generator.build_substitution_map();

        assert_eq!(map.get("PROJECT_NAME"), Some(&"test-app".to_string()));
        assert_eq!(map.get("CLAUDE_VERSION"), Some(&"latest".to_string()));
        assert_eq!(map.get("CLAUDE_PROVIDER"), Some(&"anthropic".to_string()));
        assert_eq!(map.get("PYTHON_VERSION"), Some(&"3.12".to_string()));
        assert_eq!(map.get("PROXY_ENABLED"), Some(&"false".to_string()));
    }

    #[test]
    fn test_render_dockerfile() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let dockerfile = generator.render_dockerfile().unwrap();

        assert!(dockerfile.contains("ARG BASE_IMAGE=mcr.microsoft.com/devcontainers/base:ubuntu"));
        assert!(dockerfile.contains("ARG PYTHON_VERSION=3.12"));
        assert!(dockerfile.contains("USER ${USERNAME}"));
    }

    #[test]
    fn test_render_claude_config() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let claude_config = generator.render_claude_config().unwrap();

        assert!(claude_config.contains("\"provider\""));
        assert!(claude_config.contains("anthropic"));
    }

    #[test]
    fn test_render_project_gitignore() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let gitignore = generator.render_project_gitignore().unwrap();

        assert!(gitignore.contains(".claude/"));
        assert!(gitignore.contains("node_modules/"));
    }

    #[test]
    fn test_render_devcontainer_gitignore() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let gitignore = generator.render_devcontainer_gitignore().unwrap();

        assert!(gitignore.contains(".claude/"));
        assert!(gitignore.contains(".omc/"));
        assert!(gitignore.contains("settings.json"));
    }

    #[test]
    fn test_render_project_readme() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let readme = generator.render_project_readme().unwrap();

        assert!(readme.contains("# test-app"));
        assert!(readme.contains("Dev Containers: Rebuild Container"));
    }

    #[test]
    fn test_substitution_with_proxy() {
        let mut config = create_test_config();
        config.proxy = Some(crate::config::ProxyConfig {
            http: Some("http://proxy.example.com:8080".to_string()),
            https: Some("http://proxy.example.com:8080".to_string()),
            no_proxy: Some("localhost,127.0.0.1".to_string()),
        });

        let generator = Generator::new(config).unwrap();
        let map = generator.build_substitution_map();

        assert_eq!(
            map.get("HTTP_PROXY"),
            Some(&"http://proxy.example.com:8080".to_string())
        );
        assert_eq!(
            map.get("HTTPS_PROXY"),
            Some(&"http://proxy.example.com:8080".to_string())
        );
        assert_eq!(map.get("PROXY_ENABLED"), Some(&"true".to_string()));
    }

    #[test]
    fn test_plugin_activation_lists() {
        let mut config = create_test_config();
        config.plugins = vec![
            crate::config::PluginConfig {
                marketplace: "omc".to_string(),
                name: "plugin1".to_string(),
                activate: true,
            },
            crate::config::PluginConfig {
                marketplace: "omc".to_string(),
                name: "plugin2".to_string(),
                activate: false,
            },
        ];

        let generator = Generator::new(config).unwrap();
        let map = generator.build_substitution_map();

        assert!(map.get("CLAUDE_ACTIVATE_PLUGINS").unwrap().contains("plugin1"));
        assert!(!map.get("CLAUDE_ACTIVATE_PLUGINS").unwrap().contains("plugin2"));
        assert!(map.get("CLAUDE_DEACTIVATE_PLUGINS").unwrap().contains("plugin2"));
        assert!(!map.get("CLAUDE_DEACTIVATE_PLUGINS").unwrap().contains("plugin1"));
    }

    #[test]
    fn test_language_to_version_key_mappings() {
        use crate::generator::Generator;

        assert_eq!(Generator::language_to_version_key("python"), Some("PYTHON_VERSION".to_string()));
        assert_eq!(Generator::language_to_version_key("node"), Some("NODE_VERSION".to_string()));
        assert_eq!(Generator::language_to_version_key("nodejs"), Some("NODE_VERSION".to_string()));
        assert_eq!(Generator::language_to_version_key("javascript"), Some("NODE_VERSION".to_string()));
        assert_eq!(Generator::language_to_version_key("rust"), Some("RUST_VERSION".to_string()));
        assert_eq!(Generator::language_to_version_key("go"), Some("GO_VERSION".to_string()));
        assert_eq!(Generator::language_to_version_key("golang"), Some("GO_VERSION".to_string()));
        assert_eq!(Generator::language_to_version_key("unknown"), None);
    }

    #[test]
    fn test_render_devcontainer_json_substitutions() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();

        let result = generator.render_devcontainer_json();
        assert!(result.is_ok());

        let rendered = result.unwrap();
        assert!(rendered.contains("test-app"));
        assert!(!rendered.contains("{{PROJECT_NAME}}"));
        assert!(rendered.contains("\"haiku\":"));
        assert!(rendered.contains("\"sonnet\":"));
    }

    #[test]
    fn test_copy_dir_recursive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let src = temp_dir.path().join("src");
        let dst = temp_dir.path().join("dst");

        // Create source structure
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("file1.txt"), "content1").unwrap();
        fs::create_dir_all(src.join("subdir")).unwrap();
        fs::write(src.join("subdir/file2.txt"), "content2").unwrap();

        let config = create_test_config();
        let generator = Generator::new(config).unwrap();

        generator.copy_dir_recursive(&src, &dst).unwrap();

        assert!(dst.exists());
        assert!(dst.join("file1.txt").exists());
        assert!(dst.join("subdir/file2.txt").exists());
    }

    #[test]
    fn test_copy_core_features_with_temp_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let features_dir = temp_dir.path().join("features");

        // Setup mock isolde root
        let mock_root = temp_dir.path().join("isolde");
        fs::create_dir_all(mock_root.join("core/features/feature1")).unwrap();
        fs::write(mock_root.join("core/features/feature1/install.sh"), "#!/bin/bash").unwrap();

        let config = create_test_config();
        let mut generator = Generator::new(config).unwrap();
        generator.isolde_root = mock_root;

        let copied = generator.copy_core_features(&features_dir).unwrap();

        assert!(!copied.is_empty());
        assert!(features_dir.join("feature1").exists());
    }

    /// Mock git runner for testing
    struct MockGitRunner {
        should_fail: bool,
    }

    impl MockGitRunner {
        fn new() -> Self {
            Self { should_fail: false }
        }

        fn failing() -> Self {
            Self { should_fail: true }
        }
    }

    impl GitRunner for MockGitRunner {
        fn run_git(&self, _dir: &Path, _args: &[&str]) -> Result<()> {
            if self.should_fail {
                Err(Error::Other("Mock git failure".to_string()))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn test_initialize_git_with_mock_runner() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = create_test_config();

        let mut generator = Generator::new(config).unwrap();
        generator.git_runner = Box::new(MockGitRunner::new());

        let result = generator.initialize_git_repos(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_git_command_fails_propagates_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config = create_test_config();

        let mut generator = Generator::new(config).unwrap();
        generator.git_runner = Box::new(MockGitRunner::failing());

        let result = generator.initialize_git_repos(temp_dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_full_workflow() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir = temp_dir.path();

        // Setup mock isolde root
        let mock_root = temp_dir.path().join("isolde");
        fs::create_dir_all(mock_root.join("core/features/claude-code")).unwrap();
        fs::create_dir_all(mock_root.join("core/features/proxy")).unwrap();
        fs::create_dir_all(mock_root.join("core/features/plugin-manager")).unwrap();
        fs::write(mock_root.join("core/features/claude-code/install.sh"), "#!/bin/bash\necho claude").unwrap();
        fs::write(mock_root.join("core/features/proxy/install.sh"), "#!/bin/bash\necho proxy").unwrap();
        fs::write(mock_root.join("core/features/plugin-manager/install.sh"), "#!/bin/bash\necho plugin").unwrap();

        let config = create_test_config();
        let mut generator = Generator::new(config).unwrap();
        generator.isolde_root = mock_root;
        generator.git_runner = Box::new(MockGitRunner::new());

        let report = generator.generate(output_dir).unwrap();

        // Verify files created
        assert!(!report.files_created.is_empty());

        let devcontainer_dir = output_dir.join(".devcontainer");
        assert!(devcontainer_dir.exists());
        assert!(devcontainer_dir.join("devcontainer.json").exists());
        assert!(devcontainer_dir.join("Dockerfile").exists());
        assert!(devcontainer_dir.join("features/claude-code").exists());

        let workspace_dir = output_dir.join("./project");
        assert!(workspace_dir.exists());
        assert!(workspace_dir.join(".claude/config.json").exists());
        assert!(workspace_dir.join("README.md").exists());
        assert!(workspace_dir.join(".gitignore").exists());
    }
}
