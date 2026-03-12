//! # isolde.yaml schema version 0.1
//!
//! This module contains the configuration structure for schema version 0.1.
//! This is the initial schema version for isolde.yaml.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// A value in agent options: either a plain string or a nested string map.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AgentOptionValue {
    Str(String),
    Map(HashMap<String, String>),
}

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

    /// Coding agent configuration
    #[serde(default = "default_agent_config")]
    pub agent: AgentConfig,

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

    /// Isolation level for devcontainer state sharing
    #[serde(default)]
    pub isolation: IsolationLevel,
}

fn default_agent_config() -> AgentConfig {
    AgentConfig {
        name: default_agent_name(),
        version: default_agent_version(),
        options: Default::default(),
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

/// Coding agent configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentConfig {
    /// Agent name (e.g., "claude-code", "codex", "gemini", "aider")
    #[serde(default = "default_agent_name")]
    pub name: String,

    /// Agent CLI version (e.g., "latest", "stable", or specific version)
    #[serde(default = "default_agent_version")]
    pub version: String,

    /// Agent-specific options (free-form key-value pairs)
    #[serde(default)]
    pub options: HashMap<String, AgentOptionValue>,
}

fn default_agent_name() -> String {
    "claude-code".to_string()
}

fn default_agent_version() -> String {
    "latest".to_string()
}

impl AgentConfig {
    /// Validate agent configuration
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::Error::InvalidTemplate(
                "Agent name cannot be empty".to_string(),
            ));
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

/// Isolation level for devcontainer state sharing
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IsolationLevel {
    /// No isolation — mount entire host ~/.claude
    None,
    /// Isolate sessions and telemetry (default)
    #[default]
    Session,
    /// Isolate sessions, telemetry, and plugins
    Workspace,
    /// Full isolation — only share auth credentials
    Full,
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

        // Validate agent config
        self.agent.validate()?;

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
agent:
  name: claude-code
  version: latest
  options: {}
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.version, "0.1");
        assert_eq!(config.name, "test-project");
        assert_eq!(config.workspace.dir, "./project");
        assert_eq!(config.docker.image, "ubuntu:latest");
        assert_eq!(config.agent.name, "claude-code");
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
        assert_eq!(config.agent.name, "claude-code"); // default
        assert_eq!(config.agent.version, "latest"); // default
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
agent:
  name: claude-code
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
agent:
  name: claude-code
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
agent:
  name: claude-code
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_empty_agent_name() {
        let yaml = r#"
version: "0.1"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
agent:
  name: ""
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_other_agents() {
        let yaml = r#"
version: "0.1"
name: test
workspace:
  dir: .
docker:
  image: ubuntu:latest
agent:
  name: codex
  version: latest
  options: {}
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(config.validate().is_ok());
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

    #[test]
    fn test_isolation_level_default() {
        let yaml = r#"
version: "0.1"
name: test
docker:
  image: ubuntu:latest
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.isolation, IsolationLevel::Session);
    }

    #[test]
    fn test_isolation_level_none() {
        let yaml = r#"
version: "0.1"
name: test
docker:
  image: ubuntu:latest
isolation: none
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.isolation, IsolationLevel::None);
    }

    #[test]
    fn test_isolation_level_workspace() {
        let yaml = r#"
version: "0.1"
name: test
docker:
  image: ubuntu:latest
isolation: workspace
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.isolation, IsolationLevel::Workspace);
    }

    #[test]
    fn test_isolation_level_full() {
        let yaml = r#"
version: "0.1"
name: test
docker:
  image: ubuntu:latest
isolation: full
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.isolation, IsolationLevel::Full);
    }

    #[test]
    fn test_agent_options() {
        let yaml = r#"
version: "0.1"
name: test
docker:
  image: ubuntu:latest
agent:
  name: claude-code
  version: stable
  options:
    provider: anthropic
    models:
      haiku: claude-3-5-haiku-20241022
      sonnet: claude-3-5-sonnet-20241022
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.agent.name, "claude-code");
        assert_eq!(config.agent.version, "stable");
        assert_eq!(config.agent.options.get("provider"), Some(&AgentOptionValue::Str("anthropic".to_string())));
        if let Some(AgentOptionValue::Map(m)) = config.agent.options.get("models") {
            assert_eq!(m.get("haiku").map(String::as_str), Some("claude-3-5-haiku-20241022"));
            assert_eq!(m.get("sonnet").map(String::as_str), Some("claude-3-5-sonnet-20241022"));
        } else {
            panic!("models should be AgentOptionValue::Map");
        }
    }
}
