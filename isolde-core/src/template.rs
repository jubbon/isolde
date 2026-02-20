//! # Template engine for Isolde
//!
//! This module provides a simple template engine based on string replacement
//! for rendering configuration files from templates.

use crate::config::Config;
use crate::error::{Error, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Simple template engine using {{variable}} placeholder replacement
pub struct TemplateEngine {
    /// Template content indexed by name
    templates: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine with built-in templates
    ///
    /// # Errors
    ///
    /// Returns an error if built-in templates cannot be loaded
    pub fn new() -> Result<Self> {
        let mut templates = HashMap::new();

        // Register built-in templates
        templates.insert(
            "devcontainer.json".to_string(),
            include_str!("../templates/devcontainer.json.tera").to_string(),
        );
        templates.insert(
            "Dockerfile".to_string(),
            include_str!("../templates/Dockerfile.tera").to_string(),
        );
        templates.insert(
            "claude-config.json".to_string(),
            include_str!("../templates/claude-config.json.tera").to_string(),
        );

        Ok(Self { templates })
    }

    /// Create a new template engine and load templates from a directory
    ///
    /// # Errors
    ///
    /// Returns an error if the templates directory cannot be read
    pub fn from_dir<P: AsRef<Path>>(templates_dir: P) -> Result<Self> {
        let dir = templates_dir.as_ref();
        if !dir.exists() {
            return Err(Error::PathNotFound(dir.to_path_buf()));
        }

        let mut templates = HashMap::new();

        // Read all .tera and .template files
        for entry in fs::read_dir(dir)
            .map_err(|e| Error::IoError(format!("Failed to read templates directory {dir:?}: {e}")))?
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let ext = path.extension().and_then(|s| s.to_str());
                if ext == Some("tera") || ext == Some("template") {
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .ok_or_else(|| Error::InvalidTemplate(format!("Invalid template name: {:?}", path)))?;

                    let content = fs::read_to_string(&path)
                        .map_err(|e| Error::IoError(format!("Failed to read template {:?}: {e}", path)))?;

                    templates.insert(name.to_string(), content);
                }
            }
        }

        if templates.is_empty() {
            return Err(Error::InvalidTemplate(format!(
                "No templates found in {dir:?}"
            )));
        }

        Ok(Self { templates })
    }

    /// Register a template file from the filesystem
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read
    pub fn register_template_file(&mut self, name: &str, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(Error::PathNotFound(path.to_path_buf()));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| Error::IoError(format!("Failed to read template '{name}': {e}")))?;

        self.templates.insert(name.to_string(), content);
        Ok(())
    }

    /// Render a template with the given context variables
    ///
    /// # Errors
    ///
    /// Returns an error if the template is not found
    pub fn render_template(&self, name: &str, context: &TemplateContext) -> Result<String> {
        let template = self.templates.get(name).ok_or_else(|| {
            Error::InvalidTemplate(format!("Template '{name}' not found. Available: {:?}", self.templates.keys().collect::<Vec<_>>()))
        })?;

        Ok(render_template_simple(template, context))
    }

    /// Build template context from an Isolde configuration
    ///
    /// This converts the high-level configuration into template variables
    pub fn build_context(config: &Config) -> TemplateContext {
        // Build models JSON string for devcontainer
        let models_json = serde_json::to_string(&config.claude.models).unwrap_or_default();

        // Plugin activation lists
        let active_plugins: Vec<String> = config
            .plugins
            .iter()
            .filter(|p| p.activate)
            .map(|p| p.name.clone())
            .collect();
        let inactive_plugins: Vec<String> = config
            .plugins
            .iter()
            .filter(|p| !p.activate)
            .map(|p| p.name.clone())
            .collect();

        TemplateContext {
            project_name: config.name.clone(),
            docker_image: config.docker.image.clone(),
            lang_version: config.runtime.as_ref().map(|r| r.version.clone()),
            claude_version: config.claude.version.clone(),
            claude_provider: config.claude.provider.clone(),
            claude_models_json: models_json,
            proxy_http: config.proxy.as_ref().and_then(|p| p.http.clone()),
            proxy_https: config.proxy.as_ref().and_then(|p| p.https.clone()),
            proxy_no_proxy: config.proxy.as_ref().and_then(|p| p.no_proxy.clone()),
            proxy_enabled: config.proxy.is_some(),
            claude_activate_plugins: active_plugins,
            claude_deactivate_plugins: inactive_plugins,
        }
    }

    /// Render a template using an Isolde configuration
    ///
    /// This is a convenience method that builds the context from the config
    /// and then renders the template.
    ///
    /// # Errors
    ///
    /// Returns an error if the template is not found
    pub fn render_with_config(&self, name: &str, config: &Config) -> Result<String> {
        let context = Self::build_context(config);
        self.render_template(name, &context)
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default template engine")
    }
}

/// Simple template renderer that replaces {{variable}} placeholders
fn render_template_simple(template: &str, context: &TemplateContext) -> String {
    let mut result = template.to_string();

    // Basic scalar replacements
    result = result.replace("{{project_name}}", &context.project_name);
    result = result.replace("{{docker_image}}", &context.docker_image);
    result = result.replace("{{claude_version}}", &context.claude_version);
    result = result.replace("{{claude_provider}}", &context.claude_provider);
    result = result.replace("{{claude_models_json}}", &context.claude_models_json);

    // Optional values with defaults
    let lang_version = context.lang_version.as_deref().unwrap_or("");
    result = result.replace("{{lang_version}}", lang_version);

    // Feature paths
    result = result.replace("{{features_claude_code}}", "./features/claude-code");
    result = result.replace("{{features_proxy}}", "./features/proxy");
    result = result.replace("{{features_plugin_manager}}", "./features/plugin-manager");

    // Proxy configuration
    let proxy_http = context.proxy_http.as_deref().unwrap_or("");
    let proxy_https = context.proxy_https.as_deref().unwrap_or("");
    let proxy_no_proxy = context.proxy_no_proxy.as_deref().unwrap_or("");
    result = result.replace("{{proxy_http}}", proxy_http);
    result = result.replace("{{proxy_https}}", proxy_https);
    result = result.replace("{{proxy_no_proxy}}", proxy_no_proxy);

    // Plugin lists
    let activate_list = format_plugin_list(&context.claude_activate_plugins);
    let deactivate_list = format_plugin_list(&context.claude_deactivate_plugins);
    result = result.replace("{{claude_activate_plugins}}", &activate_list);
    result = result.replace("{{claude_deactivate_plugins}}", &deactivate_list);

    result
}

/// Format a list of plugin names as a JSON array string
fn format_plugin_list(plugins: &[String]) -> String {
    plugins
        .iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Template context for rendering
///
/// This struct provides a structured way to pass template variables
/// that may not come from a full Config.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateContext {
    /// Project name
    pub project_name: String,

    /// Docker image
    pub docker_image: String,

    /// Language version (e.g., PYTHON_VERSION, NODE_VERSION)
    pub lang_version: Option<String>,

    /// Claude Code version
    #[serde(default = "default_claude_version")]
    pub claude_version: String,

    /// Claude provider
    #[serde(default = "default_claude_provider")]
    pub claude_provider: String,

    /// Claude models as JSON string
    #[serde(default)]
    pub claude_models_json: String,

    /// HTTP proxy URL
    pub proxy_http: Option<String>,

    /// HTTPS proxy URL
    pub proxy_https: Option<String>,

    /// No proxy hosts
    pub proxy_no_proxy: Option<String>,

    /// Whether proxy is enabled
    #[serde(default)]
    pub proxy_enabled: bool,

    /// Active plugins
    #[serde(default)]
    pub claude_activate_plugins: Vec<String>,

    /// Inactive plugins
    #[serde(default)]
    pub claude_deactivate_plugins: Vec<String>,
}

fn default_claude_version() -> String {
    "latest".to_string()
}

fn default_claude_provider() -> String {
    "anthropic".to_string()
}

impl TemplateContext {
    /// Create a new template context with minimal required fields
    pub fn new(project_name: String, docker_image: String) -> Self {
        Self {
            project_name,
            docker_image,
            lang_version: None,
            claude_version: default_claude_version(),
            claude_provider: default_claude_provider(),
            claude_models_json: "{}".to_string(),
            proxy_http: None,
            proxy_https: None,
            proxy_no_proxy: None,
            proxy_enabled: false,
            claude_activate_plugins: Vec::new(),
            claude_deactivate_plugins: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ClaudeConfig, DockerConfig, WorkspaceConfig};

    fn create_test_config() -> Config {
        Config {
            name: "test-project".to_string(),
            version: "1.0.0".to_string(),
            workspace: WorkspaceConfig {
                dir: "./project".to_string(),
            },
            docker: DockerConfig {
                image: "mcr.microsoft.com/devcontainers/base:ubuntu".to_string(),
                build_args: vec!["USERNAME=user".to_string()],
            },
            claude: ClaudeConfig {
                version: "latest".to_string(),
                provider: "anthropic".to_string(),
                models: {
                    let mut map = HashMap::new();
                    map.insert("haiku".to_string(), "claude-3-5-haiku-20241022".to_string());
                    map.insert("sonnet".to_string(), "claude-3-5-sonnet-20241022".to_string());
                    map.insert("opus".to_string(), "claude-3-5-sonnet-20241022".to_string());
                    map
                },
            },
            runtime: None,
            proxy: None,
            marketplaces: HashMap::new(),
            plugins: vec![],
            git: Default::default(),
        }
    }

    #[test]
    fn test_template_engine_new() {
        let engine = TemplateEngine::new();
        assert!(engine.is_ok());
        if let Ok(e) = engine {
            assert_eq!(e.templates.len(), 3);
        }
    }

    #[test]
    fn test_template_engine_default() {
        let engine = TemplateEngine::default();
        assert_eq!(engine.templates.len(), 3);
    }

    #[test]
    fn test_build_context() {
        let config = create_test_config();
        let context = TemplateEngine::build_context(&config);

        assert_eq!(context.project_name, "test-project");
        assert_eq!(context.docker_image, "mcr.microsoft.com/devcontainers/base:ubuntu");
        assert_eq!(context.claude_version, "latest");
        assert_eq!(context.claude_provider, "anthropic");
        assert!(!context.proxy_enabled);
    }

    #[test]
    fn test_template_context_new() {
        let ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );

        assert_eq!(ctx.project_name, "my-project");
        assert_eq!(ctx.docker_image, "ubuntu:latest");
        assert_eq!(ctx.claude_version, "latest");
        assert_eq!(ctx.claude_provider, "anthropic");
        assert!(!ctx.proxy_enabled);
        assert!(ctx.claude_activate_plugins.is_empty());
    }

    #[test]
    fn test_template_context_with_proxy() {
        let mut ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );
        ctx.proxy_http = Some("http://proxy.example.com:8080".to_string());
        ctx.proxy_https = Some("http://proxy.example.com:8080".to_string());
        ctx.proxy_no_proxy = Some("localhost,127.0.0.1".to_string());
        ctx.proxy_enabled = true;

        assert!(ctx.proxy_enabled);
        assert_eq!(ctx.proxy_http, Some("http://proxy.example.com:8080".to_string()));
    }

    #[test]
    fn test_template_context_with_plugins() {
        let mut ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );
        ctx.claude_activate_plugins = vec!["plugin-a".to_string(), "plugin-b".to_string()];
        ctx.claude_deactivate_plugins = vec!["plugin-c".to_string()];

        assert_eq!(ctx.claude_activate_plugins.len(), 2);
        assert_eq!(ctx.claude_deactivate_plugins.len(), 1);
    }

    #[test]
    fn test_render_template_simple() {
        let template = "Image: {{docker_image}}, Name: {{project_name}}";
        let ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );

        let result = render_template_simple(template, &ctx);
        assert_eq!(result, "Image: ubuntu:latest, Name: my-project");
    }

    #[test]
    fn test_render_template_with_lang_version() {
        let template = "{{project_name}}-v{{lang_version}}";
        let mut ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );
        ctx.lang_version = Some("3.12".to_string());

        let result = render_template_simple(template, &ctx);
        assert_eq!(result, "my-project-v3.12");
    }
}
