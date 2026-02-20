//! # Configuration types for Isolde
//!
//! This module provides types for parsing and validating `isolde.yaml` configuration files.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use crate::{Error, Result};

/// Main configuration for an Isolde project (isolde.yaml)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Project name
    pub name: String,

    /// Project version
    pub version: String,

    /// Workspace configuration
    pub workspace: WorkspaceConfig,

    /// Docker configuration
    pub docker: DockerConfig,

    /// Claude Code CLI configuration
    pub claude: ClaudeConfig,

    /// Runtime configuration (language, package manager, tools)
    pub runtime: Option<RuntimeConfig>,

    /// Proxy configuration for corporate networks
    pub proxy: Option<ProxyConfig>,

    /// Marketplace configurations
    #[serde(default)]
    pub marketplaces: HashMap<String, MarketplaceConfig>,

    /// Plugin configurations
    #[serde(default)]
    pub plugins: Vec<PluginConfig>,

    /// Git configuration
    #[serde(default)]
    pub git: GitConfig,
}

impl Config {
    /// Parse configuration from a file path
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The YAML is invalid
    /// - Required fields are missing
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::InvalidTemplate(format!("Failed to read config file: {e}")))?;
        Self::from_str(&content)
    }

    /// Parse configuration from a YAML string
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The YAML is invalid
    /// - Required fields are missing
    pub fn from_str(s: &str) -> Result<Self> {
        let config: Config = serde_yaml::from_str(s)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The name is empty
    /// - The version is empty
    /// - Required nested fields are invalid
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::InvalidTemplate("Project name cannot be empty".to_string()));
        }

        if self.version.is_empty() {
            return Err(Error::InvalidTemplate("Project version cannot be empty".to_string()));
        }

        // Validate workspace directory
        if self.workspace.dir.is_empty() {
            return Err(Error::InvalidTemplate("Workspace directory cannot be empty".to_string()));
        }

        // Validate Docker image
        if self.docker.image.is_empty() {
            return Err(Error::InvalidTemplate("Docker image cannot be empty".to_string()));
        }

        // Validate Claude config
        self.claude.validate()?;

        // Validate plugins reference existing marketplaces
        for plugin in &self.plugins {
            if !self.marketplaces.contains_key(&plugin.marketplace) {
                return Err(Error::InvalidTemplate(format!(
                    "Plugin '{}' references unknown marketplace '{}'",
                    plugin.name, plugin.marketplace
                )));
            }
        }

        Ok(())
    }
}

/// Workspace configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    /// Directory for the workspace (relative to project root)
    pub dir: String,
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
    pub models: HashMap<String, String>,
}

impl ClaudeConfig {
    /// Validate Claude configuration
    fn validate(&self) -> Result<()> {
        if !VALID_CLAUDE_PROVIDERS.contains(&self.provider.as_str()) {
            return Err(Error::InvalidTemplate(format!(
                "Invalid Claude provider '{}'. Must be one of: {}",
                self.provider,
                VALID_CLAUDE_PROVIDERS.join(", ")
            )));
        }
        Ok(())
    }
}

/// Valid Claude API providers
const VALID_CLAUDE_PROVIDERS: &[&str] = &["anthropic", "openai", "bedrock", "vertex", "azure"];

fn default_claude_version() -> String {
    "latest".to_string()
}

fn default_claude_provider() -> String {
    "anthropic".to_string()
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
#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
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

/// Template metadata from template-info.yaml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template version
    pub version: String,
    /// Default language version
    pub lang_version_default: String,
    /// Available features
    #[serde(default)]
    pub features: Vec<FeatureInfo>,
    /// Supported language versions
    #[serde(default)]
    pub supported_versions: Vec<String>,
}

/// Feature information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureInfo {
    /// Feature name
    pub name: String,
    /// Feature description
    pub description: String,
}

/// Preset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// Preset name
    pub name: String,
    /// Template to use
    pub template: String,
    /// Language version
    pub lang_version: String,
    /// Features to include
    #[serde(default)]
    pub features: Vec<String>,
    /// Claude plugins to include
    #[serde(default)]
    pub claude_plugins: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ISOLDE_YAML: &str = r#"
name: my-app
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
    opus: claude-3-5-sonnet-20241022
runtime:
  language: python
  version: "3.12"
  package_manager: uv
  tools: []
proxy:
  http: http://proxy.corp.com:8080
  https: http://proxy.corp.com:8080
  no_proxy: localhost,127.0.0.1,.local
marketplaces:
  omc:
    url: https://github.com/oh-my-claudecode/marketplace
plugins:
  - marketplace: omc
    name: oh-my-claudecode
    activate: true
git:
  generated: ignored
"#;

    #[test]
    fn test_config_from_str_valid() {
        let config = Config::from_str(VALID_ISOLDE_YAML).unwrap();
        assert_eq!(config.name, "my-app");
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.workspace.dir, "./project");
        assert_eq!(config.docker.image, "mcr.microsoft.com/devcontainers/base:ubuntu");
        assert_eq!(config.claude.provider, "anthropic");
    }

    #[test]
    fn test_config_from_str_minimal() {
        let yaml = r#"
name: minimal-app
version: 0.1.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config = Config::from_str(yaml).unwrap();
        assert_eq!(config.name, "minimal-app");
        assert_eq!(config.claude.version, "latest"); // default
        assert!(config.runtime.is_none());
        assert!(config.proxy.is_none());
        assert!(config.plugins.is_empty());
    }

    #[test]
    fn test_config_from_str_missing_name() {
        let yaml = r#"
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_str_missing_workspace() {
        let yaml = r#"
name: test
version: 1.0.0
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_str_missing_docker() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
claude:
  provider: anthropic
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_str_missing_claude() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_from_str_invalid_provider() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: invalid_provider
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid Claude provider"));
    }

    #[test]
    fn test_config_from_str_empty_name() {
        let yaml = r#"
name: ""
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name cannot be empty"));
    }

    #[test]
    fn test_config_from_str_unknown_marketplace() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
marketplaces:
  omc:
    url: https://example.com
plugins:
  - marketplace: unknown
    name: some-plugin
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unknown marketplace"));
    }

    #[test]
    fn test_config_validate_runtime() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
runtime:
  language: rust
  version: "1.75"
  package_manager: cargo
  tools:
    - rustfmt
    - clippy
"#;
        let config = Config::from_str(yaml).unwrap();
        let runtime = config.runtime.unwrap();
        assert_eq!(runtime.language, "rust");
        assert_eq!(runtime.version, "1.75");
        assert_eq!(runtime.tools.len(), 2);
    }

    #[test]
    fn test_config_default_git_handling() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config = Config::from_str(yaml).unwrap();
        assert_eq!(config.git.generated, GitGeneratedHandling::Ignored);
    }

    #[test]
    fn test_template_info_deserialize() {
        let yaml = r#"
name: Python
description: Python development environment
version: "1.0.0"
lang_version_default: "3.12"
features: []
supported_versions: ["3.12", "3.11", "3.10"]
"#;
        let info: TemplateInfo = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(info.name, "Python");
        assert_eq!(info.lang_version_default, "3.12");
    }

    #[test]
    fn test_serialize_config() {
        let config = Config::from_str(VALID_ISOLDE_YAML).unwrap();
        let serialized = serde_yaml::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.version, deserialized.version);
    }

    #[test]
    fn test_claude_models_map() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
  models:
    haiku: custom-haiku-model
    sonnet: custom-sonnet-model
    opus: custom-opus-model
"#;
        let config = Config::from_str(yaml).unwrap();
        assert_eq!(config.claude.models.len(), 3);
        assert_eq!(config.claude.models.get("haiku").unwrap(), "custom-haiku-model");
    }

    #[test]
    fn test_proxy_config_optional() {
        let yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
proxy:
  http: http://proxy.example.com:8080
"#;
        let config = Config::from_str(yaml).unwrap();
        let proxy = config.proxy.unwrap();
        assert_eq!(proxy.http.unwrap(), "http://proxy.example.com:8080");
        assert!(proxy.https.is_none());
        assert!(proxy.no_proxy.is_none());
    }

    #[test]
    fn test_valid_claude_providers() {
        for provider in VALID_CLAUDE_PROVIDERS {
            let yaml = format!(r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: {}
"#, provider);

            let result = Config::from_str(&yaml);
            assert!(result.is_ok(), "Provider {} should be valid", provider);
        }
    }
}
