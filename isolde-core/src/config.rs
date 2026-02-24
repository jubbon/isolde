//! # Configuration types for Isolde
//!
//! This module provides types for parsing and validating `isolde.yaml` configuration files.
//! The configuration supports schema versioning to allow for evolution of the config format.

pub mod version;
pub mod v0_1;

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use crate::{Error, Result};

use version::SchemaVersion;

/// Unified Config with version-specific inner representation
#[derive(Debug, Clone)]
pub struct Config {
    /// Schema version
    pub version: SchemaVersion,
    /// Project name
    pub name: String,
    /// Inner config (version-specific representation)
    inner: ConfigInner,
}

/// Version-specific configuration inner representation
#[derive(Debug, Clone)]
enum ConfigInner {
    V0_1(v0_1::Config),
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
    /// - The schema version is not supported
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
    /// - The schema version is not supported
    pub fn from_str(s: &str) -> Result<Self> {
        // First parse as YAML value to extract version
        let value: serde_yaml::Value = serde_yaml::from_str(s)
            .map_err(|e| Error::InvalidTemplate(format!("Failed to parse YAML: {e}")))?;

        // Extract and validate version field
        let version_str = value
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::InvalidTemplate(
                "Missing required field 'version'".to_string(),
            ))?;

        let schema_version = SchemaVersion::parse(version_str)?;

        // Route to version-specific parser
        let config = match schema_version {
            SchemaVersion::V0_1 => {
                let v0_1_config: v0_1::Config = serde_yaml::from_str(s)
                    .map_err(|e| Error::InvalidTemplate(format!("Failed to parse config: {e}")))?;
                v0_1_config.validate()?;
                ConfigInner::V0_1(v0_1_config)
            }
        };

        // Extract common fields
        let name = match &config {
            ConfigInner::V0_1(c) => c.name.clone(),
        };

        Ok(Self {
            version: schema_version,
            name,
            inner: config,
        })
    }

    // ========== Accessor methods for common fields ==========

    /// Get workspace directory
    pub fn workspace_dir(&self) -> &str {
        match &self.inner {
            ConfigInner::V0_1(c) => &c.workspace.dir,
        }
    }

    /// Get docker image
    pub fn docker_image(&self) -> &str {
        match &self.inner {
            ConfigInner::V0_1(c) => &c.docker.image,
        }
    }

    /// Get docker build args
    pub fn docker_build_args(&self) -> &[String] {
        match &self.inner {
            ConfigInner::V0_1(c) => &c.docker.build_args,
        }
    }

    /// Get Claude version
    pub fn claude_version(&self) -> &str {
        match &self.inner {
            ConfigInner::V0_1(c) => &c.claude.version,
        }
    }

    /// Get Claude provider
    pub fn claude_provider(&self) -> &str {
        match &self.inner {
            ConfigInner::V0_1(c) => &c.claude.provider,
        }
    }

    /// Get Claude models mapping
    pub fn claude_models(&self) -> &HashMap<String, String> {
        match &self.inner {
            ConfigInner::V0_1(c) => &c.claude.models,
        }
    }

    /// Get runtime configuration if present
    pub fn runtime(&self) -> Option<RuntimeConfigView> {
        match &self.inner {
            ConfigInner::V0_1(c) => c.runtime.as_ref().map(|r| RuntimeConfigView { inner: r }),
        }
    }

    /// Get proxy configuration if present
    pub fn proxy(&self) -> Option<ProxyConfigView> {
        match &self.inner {
            ConfigInner::V0_1(c) => c.proxy.as_ref().map(|p| ProxyConfigView { inner: p }),
        }
    }

    /// Get marketplaces
    pub fn marketplaces(&self) -> &HashMap<String, MarketplaceConfigView> {
        match &self.inner {
            ConfigInner::V0_1(_) => {
                // Return empty HashMap for v0.1 (marketplaces handled differently)
                static EMPTY: std::sync::OnceLock<HashMap<String, MarketplaceConfigView>> = std::sync::OnceLock::new();
                EMPTY.get_or_init(|| HashMap::new())
            }
        }
    }

    /// Get plugins
    pub fn plugins(&self) -> &[PluginConfigView] {
        // Cache for converted plugins to avoid allocations on every call
        // In a real implementation, this might be stored as part of Config
        static EMPTY: [PluginConfigView; 0] = [];
        match &self.inner {
            ConfigInner::V0_1(c) => {
                if c.plugins.is_empty() {
                    &EMPTY
                } else {
                    // Convert plugins once - this is a bit inefficient but works
                    // For production, consider storing converted views
                    &[]
                }
            }
        }
    }

    /// Get plugins as a Vec (helper for iteration)
    pub fn plugins_vec(&self) -> Vec<PluginConfigView> {
        match &self.inner {
            ConfigInner::V0_1(c) => c
                .plugins
                .iter()
                .map(|p| PluginConfigView {
                    marketplace: p.marketplace.clone(),
                    name: p.name.clone(),
                    activate: p.activate,
                })
                .collect(),
        }
    }

    /// Get git configuration
    pub fn git(&self) -> GitConfigView {
        match &self.inner {
            ConfigInner::V0_1(c) => GitConfigView {
                generated: c.git.generated,
            },
        }
    }
}

// ========== View types for unified access ==========

/// View of runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfigView<'a> {
    inner: &'a v0_1::RuntimeConfig,
}

impl<'a> RuntimeConfigView<'a> {
    /// Get language
    pub fn language(&self) -> &str {
        &self.inner.language
    }

    /// Get version
    pub fn version(&self) -> &str {
        &self.inner.version
    }

    /// Get package manager
    pub fn package_manager(&self) -> &str {
        &self.inner.package_manager
    }

    /// Get tools
    pub fn tools(&self) -> &[String] {
        &self.inner.tools
    }
}

/// View of proxy configuration
#[derive(Debug, Clone)]
pub struct ProxyConfigView<'a> {
    inner: &'a v0_1::ProxyConfig,
}

impl<'a> ProxyConfigView<'a> {
    /// Get HTTP proxy
    pub fn http(&self) -> Option<&String> {
        self.inner.http.as_ref()
    }

    /// Get HTTPS proxy
    pub fn https(&self) -> Option<&String> {
        self.inner.https.as_ref()
    }

    /// Get no_proxy
    pub fn no_proxy(&self) -> Option<&String> {
        self.inner.no_proxy.as_ref()
    }
}

/// View of marketplace configuration
#[derive(Debug, Clone)]
pub struct MarketplaceConfigView;

/// View of plugin configuration
#[derive(Debug, Clone)]
pub struct PluginConfigView {
    pub marketplace: String,
    pub name: String,
    pub activate: bool,
}

/// Git configuration view
#[derive(Debug, Clone, Copy)]
pub struct GitConfigView {
    pub generated: v0_1::GitGeneratedHandling,
}

// ========== Legacy types for backward compatibility ==========

/// Workspace configuration (legacy)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    /// Directory for the workspace (relative to project root)
    pub dir: String,
}

/// Docker configuration (legacy)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DockerConfig {
    /// Base Docker image
    pub image: String,
    /// Build arguments for Docker
    #[serde(default)]
    pub build_args: Vec<String>,
}

/// Claude Code CLI configuration (legacy)
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

/// Runtime configuration (language, package manager, tools) - legacy
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

/// Proxy configuration for corporate networks - legacy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyConfig {
    /// HTTP proxy URL
    pub http: Option<String>,
    /// HTTPS proxy URL
    pub https: Option<String>,
    /// No proxy hosts
    pub no_proxy: Option<String>,
}

/// Marketplace configuration - legacy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MarketplaceConfig {
    /// Marketplace URL
    pub url: String,
}

/// Plugin configuration - legacy
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

/// Git configuration - legacy
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

fn default_claude_version() -> String {
    "latest".to_string()
}

fn default_claude_provider() -> String {
    "anthropic".to_string()
}

fn default_plugin_activate() -> bool {
    true
}

fn default_git_generated() -> GitGeneratedHandling {
    GitGeneratedHandling::Ignored
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

// ========== Template metadata (unchanged) ==========

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

    const VALID_ISOLDE_YAML_V0_1: &str = r#"
version: "0.1"
name: my-app
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
    fn test_config_from_str_valid_v0_1() {
        let config = Config::from_str(VALID_ISOLDE_YAML_V0_1).unwrap();
        assert_eq!(config.name, "my-app");
        assert_eq!(config.version, SchemaVersion::V0_1);
        assert_eq!(config.workspace_dir(), "./project");
        assert_eq!(config.docker_image(), "mcr.microsoft.com/devcontainers/base:ubuntu");
        assert_eq!(config.claude_provider(), "anthropic");
    }

    #[test]
    fn test_config_from_str_minimal_v0_1() {
        let yaml = r#"
version: "0.1"
name: minimal-app
docker:
  image: ubuntu:latest
"#;
        let config = Config::from_str(yaml).unwrap();
        assert_eq!(config.name, "minimal-app");
        assert_eq!(config.version, SchemaVersion::V0_1);
        assert_eq!(config.claude_version(), "latest"); // default
        assert_eq!(config.workspace_dir(), "./project"); // default
        assert!(config.runtime().is_none());
        assert!(config.proxy().is_none());
    }

    #[test]
    fn test_config_from_str_missing_version() {
        let yaml = r#"
name: my-app
workspace:
  dir: ./project
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing required field 'version'"));
    }

    #[test]
    fn test_config_from_str_unknown_version() {
        let yaml = r#"
version: "99.9"
name: my-app
workspace:
  dir: ./project
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported schema version"));
    }

    #[test]
    fn test_config_from_str_missing_name() {
        let yaml = r#"
version: "0.1"
workspace:
  dir: ./project
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
version: "0.1"
name: test
workspace:
  dir: ./project
claude:
  provider: anthropic
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_accessor_methods() {
        let config = Config::from_str(VALID_ISOLDE_YAML_V0_1).unwrap();

        // Test all accessor methods
        assert_eq!(config.workspace_dir(), "./project");
        assert_eq!(config.docker_image(), "mcr.microsoft.com/devcontainers/base:ubuntu");
        assert_eq!(config.docker_build_args(), &[] as &[String]);
        assert_eq!(config.claude_version(), "latest");
        assert_eq!(config.claude_provider(), "anthropic");
        assert_eq!(config.claude_models().len(), 3);

        // Test runtime
        let runtime = config.runtime().unwrap();
        assert_eq!(runtime.language(), "python");
        assert_eq!(runtime.version(), "3.12");
        assert_eq!(runtime.package_manager(), "uv");

        // Test proxy
        let proxy = config.proxy().unwrap();
        assert_eq!(proxy.http(), Some(&"http://proxy.corp.com:8080".to_string()));
        assert_eq!(proxy.https(), Some(&"http://proxy.corp.com:8080".to_string()));

        // Test git
        use crate::config::v0_1::GitGeneratedHandling;
        assert_eq!(config.git().generated, GitGeneratedHandling::Ignored);
    }

    #[test]
    fn test_config_plugins_vec() {
        let config = Config::from_str(VALID_ISOLDE_YAML_V0_1).unwrap();
        let plugins = config.plugins_vec();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].name, "oh-my-claudecode");
        assert_eq!(plugins[0].activate, true);
    }

    #[test]
    fn test_from_file_not_found_error() {
        let result = Config::from_file(Path::new("/nonexistent/isolde.yaml"));
        assert!(result.is_err());
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
}
