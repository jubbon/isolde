//! # Devcontainer generator from YAML config
//!
//! Orchestrates creating devcontainer artifacts from an `isolde.yaml` configuration.
//! Rendering logic is delegated to the [`devcontainer`](crate::devcontainer) module;
//! this module handles file creation, dry-run reports, and init-specific artifacts
//! (`.claude/config.json`, project `README.md`).

use crate::config::Config;
use crate::devcontainer;
use crate::error::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};

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
}

impl Generator {
    /// Create a new generator from a config
    pub fn new(config: Config) -> Result<Self> {
        let isolde_root = Self::find_isolde_root()?;
        Ok(Self {
            config,
            isolde_root,
        })
    }

    /// Find the Isolde installation root by searching upward for `templates/` and `core/`
    fn find_isolde_root() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()
            .map_err(|e| Error::Other(format!("Failed to get current directory: {e}")))?;

        let mut dir = current_dir.as_path();
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
    pub fn generate(&self, output_dir: &Path) -> Result<GenerateReport> {
        let mut files_created = Vec::new();
        let files_modified = Vec::new();

        // Create workspace directory
        let workspace_dir = output_dir.join(self.config.workspace_dir());
        fs::create_dir_all(&workspace_dir)?;

        // Create .devcontainer directory
        let devcontainer_dir = output_dir.join(".devcontainer");
        fs::create_dir_all(&devcontainer_dir)?;

        // Generate devcontainer.json (delegated to devcontainer module)
        let host_auth = devcontainer::HostAuthInfo::detect();
        let devcontainer_json =
            devcontainer::render_devcontainer_json(&self.config, &host_auth)?;
        let devcontainer_json_path = devcontainer_dir.join("devcontainer.json");
        fs::write(&devcontainer_json_path, devcontainer_json)?;
        files_created.push(devcontainer_json_path);

        // Generate Dockerfile (delegated to devcontainer module)
        let dockerfile = devcontainer::render_dockerfile(&self.config)?;
        let dockerfile_path = devcontainer_dir.join("Dockerfile");
        fs::write(&dockerfile_path, dockerfile)?;
        files_created.push(dockerfile_path);

        // Copy core features
        let features_dir = devcontainer_dir.join("features");
        let copied_features = self.copy_core_features(&features_dir)?;
        files_created.extend(copied_features);

        // Generate .claude/config.json (init-specific, not part of sync)
        let claude_config = self.render_claude_config()?;
        let claude_dir = workspace_dir.join(".claude");
        fs::create_dir_all(&claude_dir)?;
        let claude_config_path = claude_dir.join("config.json");
        fs::write(&claude_config_path, claude_config)?;
        files_created.push(claude_config_path);

        // Generate README.md for project (init-specific)
        let readme = self.render_project_readme()?;
        let readme_path = workspace_dir.join("README.md");
        fs::write(&readme_path, readme)?;
        files_created.push(readme_path);

        Ok(GenerateReport {
            files_created,
            files_modified,
        })
    }

    /// Perform a dry run to see what would be generated
    pub fn dry_run(&self, output_dir: &Path) -> Result<DryRunReport> {
        let mut would_create = Vec::new();
        let mut would_modify = Vec::new();

        let workspace_dir = output_dir.join(self.config.workspace_dir());
        let devcontainer_dir = output_dir.join(".devcontainer");
        let features_dir = devcontainer_dir.join("features");
        let claude_dir = workspace_dir.join(".claude");

        // Check devcontainer.json
        let devcontainer_json_path = devcontainer_dir.join("devcontainer.json");
        if devcontainer_json_path.exists() {
            would_modify.push(devcontainer_json_path);
        } else {
            would_create.push(devcontainer_json_path);
        }

        // Check Dockerfile
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

    /// Copy core features from the Isolde installation.
    /// Uses [`devcontainer::copy_dir_recursive`] for the actual recursive copy.
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

                if dest.exists() {
                    fs::remove_dir_all(&dest)?;
                }

                devcontainer::copy_dir_recursive(&path, &dest)?;
                copied_files.push(dest);
            }
        }

        Ok(copied_files)
    }

    /// Render Claude Code configuration (init-specific: `.claude/config.json`)
    fn render_claude_config(&self) -> Result<String> {
        use crate::config::AgentOptionValue;
        let provider = self.config.agent_option_str("provider").unwrap_or("");
        let mut config = serde_json::json!({
            "provider": provider,
        });

        if let Some(AgentOptionValue::Map(m)) = self.config.agent_options().get("models") {
            let models_obj: serde_json::Map<String, serde_json::Value> = m
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            if !models_obj.is_empty() {
                config["models"] = serde_json::Value::Object(models_obj);
            }
        }

        serde_json::to_string_pretty(&config)
            .map_err(|e| Error::Other(format!("Failed to serialize Claude config: {}", e)))
    }

    /// Render project README content (init-specific)
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
- `.devcontainer/` - Devcontainer configuration
- `.claude/` - Claude Code configuration

## DevContainer

This project uses a devcontainer for development. The configuration is in the
`.devcontainer/` directory.

To rebuild the container:
1. Press F1 in VS Code
2. Select "Dev Containers: Rebuild Container"
"#,
            self.config.name
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config::from_str(
            r#"
version: "0.1"
name: test-app
workspace:
  dir: ./project
docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
  build_args: []
agent:
  name: claude-code
  version: latest
  options:
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
    fn test_render_devcontainer_json() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let host_auth = devcontainer::HostAuthInfo::detect();
        let result = devcontainer::render_devcontainer_json(&generator.config, &host_auth);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("test-app"));
        assert!(rendered.contains("claude-code"));
        assert!(rendered.contains("common-utils"));
    }

    #[test]
    fn test_render_dockerfile() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let dockerfile = devcontainer::render_dockerfile(&generator.config).unwrap();
        assert!(dockerfile.contains("ARG BASE_IMAGE=mcr.microsoft.com/devcontainers/base:ubuntu"));
        assert!(dockerfile.contains("FROM ${BASE_IMAGE}"));
        assert!(dockerfile.contains("ARG USERNAME=user"));
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
    fn test_render_project_readme() {
        let config = create_test_config();
        let generator = Generator::new(config).unwrap();
        let readme = generator.render_project_readme().unwrap();

        assert!(readme.contains("# test-app"));
        assert!(readme.contains("Dev Containers: Rebuild Container"));
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
        assert!(features_dir.join("feature1/install.sh").exists());
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

        let report = generator.generate(output_dir).unwrap();

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
    }

    #[test]
    fn test_dry_run_all_cases() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Setup mock isolde root
        let mock_root = temp_dir.path().join("isolde");
        fs::create_dir_all(mock_root.join("core/features/test-feature")).unwrap();

        let config = create_test_config();
        let mut generator = Generator::new(config).unwrap();
        generator.isolde_root = mock_root;

        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&output_dir).unwrap();

        // First run - should show all creates
        let report = generator.dry_run(&output_dir).unwrap();
        assert!(!report.would_create.is_empty());
        assert!(report.would_modify.is_empty());

        // Create one file
        fs::create_dir_all(output_dir.join(".devcontainer")).unwrap();
        fs::write(output_dir.join(".devcontainer/devcontainer.json"), "{}").unwrap();

        // Second run - should show modify
        let report2 = generator.dry_run(&output_dir).unwrap();
        assert!(report2.would_modify.iter().any(|p| p.ends_with("devcontainer.json")));
    }
}
