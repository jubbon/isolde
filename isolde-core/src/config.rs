//! # Configuration types for Isolde
//!
//! This module provides types for parsing and validating `isolde.yaml` configuration files.
//! The configuration supports schema versioning to allow for evolution of the config format.

pub mod v0_1;
pub mod version;

pub use v0_1::AgentOptionValue;
pub use v0_1::AgentPermissions;
pub use v0_1::IsolationLevel;

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

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
            .ok_or_else(|| {
                Error::InvalidTemplate("Missing required field 'version'".to_string())
            })?;

        let schema_version = SchemaVersion::parse(version_str)?;

        // Route to version-specific parser
        let config = match schema_version {
            SchemaVersion::V0_1 => {
                let mut v0_1_config: v0_1::Config = serde_yaml::from_str(s)
                    .map_err(|e| Error::InvalidTemplate(format!("Failed to parse config: {e}")))?;
                v0_1_config.normalize();
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

    /// Get the first agent config (helper for deprecated single-agent accessors).
    fn first_agent(&self) -> Option<&v0_1::AgentConfig> {
        match &self.inner {
            ConfigInner::V0_1(c) => c.agents.as_ref()?.first(),
        }
    }

    // DEPRECATED: use agents() instead. Returns first agent for backward compatibility.
    pub fn agent_name(&self) -> &str {
        self.first_agent()
            .map(|a| a.name.as_str())
            .unwrap_or("claude-code")
    }

    // DEPRECATED: use agents() instead. Returns first agent for backward compatibility.
    pub fn agent_version(&self) -> &str {
        self.first_agent()
            .map(|a| a.version.as_str())
            .unwrap_or("latest")
    }

    // DEPRECATED: use agents() instead. Returns first agent for backward compatibility.
    pub fn agent_options(&self) -> &BTreeMap<String, AgentOptionValue> {
        static EMPTY: std::sync::OnceLock<BTreeMap<String, AgentOptionValue>> =
            std::sync::OnceLock::new();
        self.first_agent()
            .map(|a| &a.options)
            .unwrap_or_else(|| EMPTY.get_or_init(BTreeMap::new))
    }

    // DEPRECATED: use agents() instead. Returns first agent for backward compatibility.
    pub fn agent_option_str(&self, key: &str) -> Option<&str> {
        match self.agent_options().get(key) {
            Some(AgentOptionValue::Str(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Get all configured agents
    pub fn agents(&self) -> Vec<AgentConfigView> {
        match &self.inner {
            ConfigInner::V0_1(c) => c
                .agents
                .as_ref()
                .map(|agents| {
                    agents
                        .iter()
                        .map(|a| AgentConfigView { inner: a })
                        .collect()
                })
                .unwrap_or_default(),
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
                static EMPTY: std::sync::OnceLock<HashMap<String, MarketplaceConfigView>> =
                    std::sync::OnceLock::new();
                EMPTY.get_or_init(HashMap::new)
            }
        }
    }

    /// Get plugins
    pub fn plugins(&self) -> Vec<PluginConfigView> {
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

    /// Get isolation level
    pub fn isolation(&self) -> v0_1::IsolationLevel {
        match &self.inner {
            ConfigInner::V0_1(c) => c.isolation,
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

/// Agent configuration view
#[derive(Debug, Clone)]
pub struct AgentConfigView<'a> {
    inner: &'a v0_1::AgentConfig,
}

impl<'a> AgentConfigView<'a> {
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn version(&self) -> &str {
        &self.inner.version
    }

    pub fn options(&self) -> &BTreeMap<String, AgentOptionValue> {
        &self.inner.options
    }

    pub fn option_str(&self, key: &str) -> Option<&str> {
        match self.inner.options.get(key) {
            Some(AgentOptionValue::Str(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn permissions(&self) -> Option<&v0_1::AgentPermissions> {
        self.inner.permissions.as_ref()
    }
}

/// Git configuration view
#[derive(Debug, Clone, Copy)]
pub struct GitConfigView {
    pub generated: v0_1::GitGeneratedHandling,
}

// ========== Template metadata ==========

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
    /// Preset name (may be omitted when preset is a YAML map value keyed by name)
    #[serde(default)]
    pub name: String,
    /// Template to use
    pub template: String,
    /// Language version
    pub lang_version: String,
    /// Features to include
    #[serde(default)]
    pub features: Vec<String>,
    /// Claude plugins to include (only applied when agent is claude-code)
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
agent:
  name: claude-code
  version: latest
  options:
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
        assert_eq!(
            config.docker_image(),
            "mcr.microsoft.com/devcontainers/base:ubuntu"
        );
        assert_eq!(config.agent_name(), "claude-code");
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
        assert_eq!(config.agent_name(), "claude-code"); // default
        assert_eq!(config.agent_version(), "latest"); // default
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
agent:
  name: claude-code
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing required field 'version'"));
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
agent:
  name: claude-code
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unsupported schema version"));
    }

    #[test]
    fn test_config_from_str_missing_name() {
        let yaml = r#"
version: "0.1"
workspace:
  dir: ./project
docker:
  image: ubuntu:latest
agent:
  name: claude-code
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
agent:
  name: claude-code
"#;
        let result = Config::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_accessor_methods() {
        let config = Config::from_str(VALID_ISOLDE_YAML_V0_1).unwrap();

        // Test all accessor methods
        assert_eq!(config.workspace_dir(), "./project");
        assert_eq!(
            config.docker_image(),
            "mcr.microsoft.com/devcontainers/base:ubuntu"
        );
        assert_eq!(config.docker_build_args(), &[] as &[String]);
        assert_eq!(config.agent_name(), "claude-code");
        assert_eq!(config.agent_version(), "latest");
        assert_eq!(
            config.agent_options().get("provider"),
            Some(&AgentOptionValue::Str("anthropic".to_string()))
        );

        // Test runtime
        let runtime = config.runtime().unwrap();
        assert_eq!(runtime.language(), "python");
        assert_eq!(runtime.version(), "3.12");
        assert_eq!(runtime.package_manager(), "uv");

        // Test proxy
        let proxy = config.proxy().unwrap();
        assert_eq!(
            proxy.http(),
            Some(&"http://proxy.corp.com:8080".to_string())
        );
        assert_eq!(
            proxy.https(),
            Some(&"http://proxy.corp.com:8080".to_string())
        );

        // Test git
        use crate::config::v0_1::GitGeneratedHandling;
        assert_eq!(config.git().generated, GitGeneratedHandling::Ignored);

        // Test isolation (default)
        assert_eq!(config.isolation(), IsolationLevel::Session);
    }

    #[test]
    fn test_config_plugins() {
        let config = Config::from_str(VALID_ISOLDE_YAML_V0_1).unwrap();
        let plugins = config.plugins();
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

    #[test]
    fn test_config_agents_accessor() {
        let yaml = r#"
version: "0.1"
name: test-app
docker:
  image: ubuntu:latest
agents:
  - name: claude-code
    version: latest
  - name: opencode
    version: latest
"#;
        let config = Config::from_str(yaml).unwrap();
        let agents = config.agents();
        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0].name(), "claude-code");
        assert_eq!(agents[1].name(), "opencode");
    }

    #[test]
    fn test_config_agents_accessor_single_agent_migration() {
        let yaml = r#"
version: "0.1"
name: test-app
docker:
  image: ubuntu:latest
agent:
  name: codex
  version: latest
"#;
        let config = Config::from_str(yaml).unwrap();
        let agents = config.agents();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name(), "codex");
        // Backward compat
        assert_eq!(config.agent_name(), "codex");
    }

    #[test]
    fn test_config_agents_accessor_defaults() {
        let yaml = r#"
version: "0.1"
name: test-app
docker:
  image: ubuntu:latest
"#;
        let config = Config::from_str(yaml).unwrap();
        let agents = config.agents();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name(), "claude-code");
    }

    #[test]
    fn test_config_with_codex_agent() {
        let yaml = r#"
version: "0.1"
name: my-app
docker:
  image: ubuntu:latest
agent:
  name: codex
  version: latest
  options: {}
"#;
        let config = Config::from_str(yaml).unwrap();
        assert_eq!(config.agent_name(), "codex");
        assert_eq!(config.agent_version(), "latest");
    }
}
