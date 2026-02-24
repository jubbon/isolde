//! # isolde.yaml schema version 0.1
//!
//! This module contains the configuration structure for schema version 0.1.
//! This is the initial schema version for isolde.yaml.

use serde::{Deserialize, Serialize};

/// Main configuration for an Isolde project (isolde.yaml) - Schema v0.1
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Schema version (must be "0.1" for this schema)
    #[serde(rename = "version")]
    pub version: String,

    /// Project name
    pub name: String,

    /// Workspace configuration
    #[serde(default = "default_workspace_config")]
    pub workspace: WorkspaceConfig,

    /// Docker configuration
    pub docker: DockerConfig,

    /// Claude Code CLI configuration
    #[serde(default = "default_claude_config")]
    pub claude: ClaudeConfig,

    /// Runtime configuration (language, package manager, tools)
    pub runtime: Option<RuntimeConfig>,

    /// Proxy configuration for corporate networks
    pub proxy: Option<ProxyConfig>,

    /// Marketplace configurations
    #[serde(default)]
    pub marketplaces: MarketplacesConfig,

    /// Plugin configurations
    #[serde(default)]
    pub plugins: Vec<PluginConfig>,

    /// Git configuration
    #[serde(default)]
    pub git: GitConfig,
}

fn default_claude_config() -> ClaudeConfig {
    ClaudeConfig {
        version: default_claude_version(),
        provider: default_claude_provider(),
        models: Default::default(),
    }
}

/// Workspace configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    /// Directory for the workspace (relative to project root)
    #[serde(default = "default_workspace_dir")]
    pub dir: String,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            dir: default_workspace_dir(),
        }
    }
}

fn default_workspace_dir() -> String {
    "./project".to_string()
}

fn default_workspace_config() -> WorkspaceConfig {
    WorkspaceConfig::default()
}

/// Docker configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DockerConfig {
    /// Base Docker image
    pub image: String,

    /// Build arguments for Docker
    #[serde(default)]
    pub build_args: Vec<String>,
}

/// Claude Code CLI configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClaudeConfig {
    /// Claude Code CLI version
    #[serde(default = "default_claude_version")]
    pub version: String,

    /// Claude API provider
    #[serde(default = "default_claude_provider")]
    pub provider: String,

    /// Model mappings for different Claude models
    #[serde(default)]
    pub models: ModelsConfig,
}

/// Models configuration as a HashMap-like structure
pub type ModelsConfig = std::collections::HashMap<String, String>;

fn default_claude_version() -> String {
    "latest".to_string()
}

fn default_claude_provider() -> String {
    "anthropic".to_string()
}

impl ClaudeConfig {
    /// Validate Claude configuration
    pub fn validate(&self) -> crate::Result<()> {
        const VALID_CLAUDE_PROVIDERS: &[&str] = &["anthropic", "openai", "bedrock", "vertex", "azure"];

        if !VALID_CLAUDE_PROVIDERS.contains(&self.provider.as_str()) {
            return Err(crate::Error::InvalidTemplate(format!(
                "Invalid Claude provider '{}'. Must be one of: {}",
                self.provider,
                VALID_CLAUDE_PROVIDERS.join(", ")
            )));
        }
        Ok(())
    }
}

/// Runtime configuration (language, package manager, tools)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuntimeConfig {
    /// Programming language
    pub language: String,

    /// Language version
    pub version: String,

    /// Package manager
    pub package_manager: String,

    /// Additional tools to install
    #[serde(default)]
    pub tools: Vec<String>,
}

/// Proxy configuration for corporate networks
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyConfig {
    /// HTTP proxy URL
    pub http: Option<String>,

    /// HTTPS proxy URL
    pub https: Option<String>,

    /// No proxy hosts
    pub no_proxy: Option<String>,
}

/// Marketplace configurations
pub type MarketplacesConfig = std::collections::HashMap<String, MarketplaceConfig>;

/// Marketplace configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketplaceConfig {
    /// Marketplace URL
    pub url: String,
}

/// Plugin configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginConfig {
    /// Marketplace to fetch plugin from
    pub marketplace: String,

    /// Plugin name
    pub name: String,

    /// Whether to activate the plugin
    #[serde(default = "default_plugin_activate")]
    pub activate: bool,
}

fn default_plugin_activate() -> bool {
    true
}

/// Git configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitConfig {
    /// How to handle generated files in git
    #[serde(default = "default_git_generated")]
    pub generated: GitGeneratedHandling,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            generated: default_git_generated(),
        }
    }
}

/// How to handle generated files in git
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GitGeneratedHandling {
    /// Add generated files to .gitignore
    #[default]
    Ignored,
    /// Commit generated files
    Committed,
    /// Add to gitattributes with linguist-generated
    LinguistGenerated,
}

fn default_git_generated() -> GitGeneratedHandling {
    GitGeneratedHandling::Ignored
}

impl Config {
    /// Validate the v0.1 configuration
    pub fn validate(&self) -> crate::Result<()> {
        // Check schema version
        if self.version != "0.1" {
            return Err(crate::Error::InvalidTemplate(format!(
                "Invalid schema version '{}'. Expected '0.1'",
                self.version
            )));
        }

        // Validate name
        if self.name.is_empty() {
            return Err(crate::Error::InvalidTemplate(
                "Project name cannot be empty".to_string(),
            ));
        }

        // Validate workspace directory
        if self.workspace.dir.is_empty() {
            return Err(crate::Error::InvalidTemplate(
                "Workspace directory cannot be empty".to_string(),
            ));
        }

        // Validate Docker image
        if self.docker.image.is_empty() {
            return Err(crate::Error::InvalidTemplate(
                "Docker image cannot be empty".to_string(),
            ));
        }

        // Validate Claude config
        self.claude.validate()?;

        // Validate plugins reference existing marketplaces
        for plugin in &self.plugins {
            if !self.marketplaces.contains_key(&plugin.marketplace) {
                return Err(crate::Error::InvalidTemplate(format!(
                    "Plugin '{}' references unknown marketplace '{}'",
                    plugin.name, plugin.marketplace
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialize_valid() {
        let yaml = r#"
version: "0.1"
name: test-project
workspace:
  dir: ./project
docker:
  image: ubuntu:latest
  build_args: []
claude:
  version: latest
  provider: anthropic
  models: {}
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.version, "0.1");
        assert_eq!(config.name, "test-project");
        assert_eq!(config.workspace.dir, "./project");
        assert_eq!(config.docker.image, "ubuntu:latest");
    }

    #[test]
    fn test_config_deserialize_minimal() {
        let yaml = r#"
version: "0.1"
name: minimal
docker:
  image: ubuntu:latest
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.version, "0.1");
        assert_eq!(config.name, "minimal");
        assert_eq!(config.workspace.dir, "./project"); // default
        assert_eq!(config.claude.provider, "anthropic"); // default
    }

    #[test]
    fn test_config_validate_success() {
        let yaml = r#"
version: "0.1"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_wrong_schema_version() {
        let yaml = r#"
version: "99.9"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_empty_name() {
        let yaml = r#"
version: "0.1"
name: ""
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_invalid_provider() {
        let yaml = r#"
version: "0.1"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: invalid_provider
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_plugin_activate_default() {
        let yaml = r#"
marketplace: omc
name: test-plugin
"#;
        let plugin: PluginConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(plugin.activate, true);
    }

    #[test]
    fn test_git_config_default() {
        let yaml = r#"
"#;
        let git: GitConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(git.generated, GitGeneratedHandling::Ignored);
    }
}
