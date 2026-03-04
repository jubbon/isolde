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
        let agent_name = config.agent_name().to_string();
        let agent_feature_path = format!("./features/{}", agent_name);
        let include_plugin_manager = agent_name == "claude-code";

        // Build agent options JSON object
        let agent_options_json = build_agent_options_json(config);

        // Build feature install order JSON array
        let mut install_order = vec![
            "./features/proxy".to_string(),
            format!("./features/{}", agent_name),
        ];
        if include_plugin_manager {
            install_order.push("./features/plugin-manager".to_string());
        }
        let feature_install_order_json =
            serde_json::to_string(&install_order).unwrap_or_else(|_| "[]".to_string());

        // Plugin activation lists (only relevant for claude-code)
        let plugins = config.plugins_vec();
        let active_plugins: Vec<String> = plugins
            .iter()
            .filter(|p| p.activate)
            .map(|p| p.name.clone())
            .collect();
        let inactive_plugins: Vec<String> = plugins
            .iter()
            .filter(|p| !p.activate)
            .map(|p| p.name.clone())
            .collect();

        TemplateContext {
            project_name: config.name.clone(),
            docker_image: config.docker_image().to_string(),
            lang_version: config.runtime().map(|r| r.version().to_string()),
            agent_name,
            agent_feature_path,
            agent_options_json,
            include_plugin_manager,
            feature_install_order_json,
            proxy_http: config.proxy().and_then(|p| p.http().cloned()),
            proxy_https: config.proxy().and_then(|p| p.https().cloned()),
            proxy_no_proxy: config.proxy().and_then(|p| p.no_proxy().cloned()),
            proxy_enabled: config.proxy().is_some(),
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

/// Build the agent options JSON object string from config
fn build_agent_options_json(config: &Config) -> String {
    use serde_json::{Map, Value};

    let mut map = Map::new();

    // Add version first
    map.insert(
        "version".to_string(),
        Value::String(config.agent_version().to_string()),
    );

    // Add all agent options
    // Note: devcontainer-feature.json declares all options as "type": "string",
    // so Map values must be serialized back to a comma-separated string.
    for (key, value) in config.agent_options() {
        use crate::config::AgentOptionValue;
        let json_val = match value {
            AgentOptionValue::Str(s) => Value::String(s.clone()),
            AgentOptionValue::Map(m) => {
                let csv = m
                    .iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<_>>()
                    .join(",");
                Value::String(csv)
            }
        };
        map.insert(key.clone(), json_val);
    }

    // Inject proxy settings if configured
    if let Some(proxy) = config.proxy() {
        if let Some(http) = proxy.http() {
            map.insert("http_proxy".to_string(), Value::String(http.clone()));
        }
        if let Some(https) = proxy.https() {
            map.insert("https_proxy".to_string(), Value::String(https.clone()));
        }
    }

    serde_json::to_string_pretty(&Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}


/// Simple template renderer that replaces {{variable}} placeholders
fn render_template_simple(template: &str, context: &TemplateContext) -> String {
    let mut result = template.to_string();

    // Basic scalar replacements
    result = result.replace("{{project_name}}", &context.project_name);
    result = result.replace("{{docker_image}}", &context.docker_image);
    result = result.replace("{{agent_name}}", &context.agent_name);
    result = result.replace("{{agent_feature_path}}", &context.agent_feature_path);
    result = result.replace("{{agent_options_json}}", &context.agent_options_json);
    result = result.replace("{{feature_install_order_json}}", &context.feature_install_order_json);

    // Optional values with defaults
    let lang_version = context.lang_version.as_deref().unwrap_or("");
    result = result.replace("{{lang_version}}", lang_version);

    // Static feature paths
    result = result.replace("{{features_proxy}}", "./features/proxy");
    result = result.replace("{{features_plugin_manager}}", "./features/plugin-manager");

    // Proxy configuration
    let proxy_http = context.proxy_http.as_deref().unwrap_or("");
    let proxy_https = context.proxy_https.as_deref().unwrap_or("");
    let proxy_no_proxy = context.proxy_no_proxy.as_deref().unwrap_or("");
    result = result.replace("{{proxy_http}}", proxy_http);
    result = result.replace("{{proxy_https}}", proxy_https);
    result = result.replace("{{proxy_no_proxy}}", proxy_no_proxy);
    result = result.replace(
        "{{proxy_enabled}}",
        if context.proxy_enabled { "true" } else { "false" },
    );

    // Plugin manager block (conditional - only for claude-code agent)
    if context.include_plugin_manager {
        let activate = format_plugin_list(&context.claude_activate_plugins);
        let deactivate = format_plugin_list(&context.claude_deactivate_plugins);
        let block = format!(
            ",\n    \"./features/plugin-manager\": {{\n      \"activate_plugins\": [{}],\n      \"deactivate_plugins\": [{}]\n    }}",
            activate, deactivate
        );
        result = result.replace("{{plugin_manager_block}}", &block);
    } else {
        result = result.replace("{{plugin_manager_block}}", "");
    }

    result
}

/// Format a list of plugin names as a JSON array inline string
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

    /// Agent name (e.g., "claude-code", "codex", "gemini", "aider")
    pub agent_name: String,

    /// Agent feature path (e.g., "./features/claude-code")
    pub agent_feature_path: String,

    /// Agent options as a JSON object string (version + options + proxy)
    pub agent_options_json: String,

    /// Whether to include the plugin manager feature (only for claude-code)
    pub include_plugin_manager: bool,

    /// Feature install order as a JSON array string
    pub feature_install_order_json: String,

    /// HTTP proxy URL
    pub proxy_http: Option<String>,

    /// HTTPS proxy URL
    pub proxy_https: Option<String>,

    /// No proxy hosts
    pub proxy_no_proxy: Option<String>,

    /// Whether proxy is enabled
    #[serde(default)]
    pub proxy_enabled: bool,

    /// Active plugins (used by plugin-manager feature, claude-code only)
    #[serde(default)]
    pub claude_activate_plugins: Vec<String>,

    /// Inactive plugins (used by plugin-manager feature, claude-code only)
    #[serde(default)]
    pub claude_deactivate_plugins: Vec<String>,
}

impl TemplateContext {
    /// Create a new template context with minimal required fields (defaults to claude-code agent)
    pub fn new(project_name: String, docker_image: String) -> Self {
        Self {
            project_name,
            docker_image,
            lang_version: None,
            agent_name: "claude-code".to_string(),
            agent_feature_path: "./features/claude-code".to_string(),
            agent_options_json: "{\"version\": \"latest\"}".to_string(),
            include_plugin_manager: true,
            feature_install_order_json: "[\"./features/proxy\",\"./features/claude-code\",\"./features/plugin-manager\"]".to_string(),
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
    use std::path::PathBuf;

    fn create_test_config() -> Config {
        Config::from_str(
            r#"
version: "0.1"
name: test-project
workspace:
  dir: ./project
docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
  build_args:
    - USERNAME=user
agent:
  name: claude-code
  version: latest
  options:
    provider: anthropic
    models:
      haiku: claude-3-5-haiku-20241022
      sonnet: claude-3-5-sonnet-20241022
      opus: claude-3-5-sonnet-20241022
"#,
        )
        .unwrap()
    }

    fn create_codex_config() -> Config {
        Config::from_str(
            r#"
version: "0.1"
name: test-codex
docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
agent:
  name: codex
  version: latest
  options: {}
"#,
        )
        .unwrap()
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
    fn test_build_context_claude_code() {
        let config = create_test_config();
        let context = TemplateEngine::build_context(&config);

        assert_eq!(context.project_name, "test-project");
        assert_eq!(context.docker_image, "mcr.microsoft.com/devcontainers/base:ubuntu");
        assert_eq!(context.agent_name, "claude-code");
        assert_eq!(context.agent_feature_path, "./features/claude-code");
        assert!(context.include_plugin_manager);
        assert!(!context.proxy_enabled);
        assert!(context.agent_options_json.contains("\"version\""));
        assert!(context.agent_options_json.contains("\"provider\""));
    }

    #[test]
    fn test_build_context_codex() {
        let config = create_codex_config();
        let context = TemplateEngine::build_context(&config);

        assert_eq!(context.agent_name, "codex");
        assert_eq!(context.agent_feature_path, "./features/codex");
        assert!(!context.include_plugin_manager);
        assert!(context.feature_install_order_json.contains("./features/codex"));
        assert!(!context.feature_install_order_json.contains("plugin-manager"));
    }

    #[test]
    fn test_template_context_new() {
        let ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );

        assert_eq!(ctx.project_name, "my-project");
        assert_eq!(ctx.docker_image, "ubuntu:latest");
        assert_eq!(ctx.agent_name, "claude-code");
        assert_eq!(ctx.agent_feature_path, "./features/claude-code");
        assert!(ctx.include_plugin_manager);
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

    #[test]
    fn test_render_agent_feature_path() {
        let template = "feature: {{agent_feature_path}}";
        let ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );

        let result = render_template_simple(template, &ctx);
        assert_eq!(result, "feature: ./features/claude-code");
    }

    #[test]
    fn test_render_plugin_manager_block_included() {
        let template = "{{agent_feature_path}}: {}{{plugin_manager_block}}";
        let mut ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );
        ctx.include_plugin_manager = true;

        let result = render_template_simple(template, &ctx);
        assert!(result.contains("plugin-manager"));
        assert!(result.contains("activate_plugins"));
    }

    #[test]
    fn test_render_plugin_manager_block_excluded() {
        let template = "{{agent_feature_path}}: {}{{plugin_manager_block}}";
        let mut ctx = TemplateContext::new(
            "my-project".to_string(),
            "ubuntu:latest".to_string(),
        );
        ctx.include_plugin_manager = false;
        ctx.agent_feature_path = "./features/codex".to_string();

        let result = render_template_simple(template, &ctx);
        assert!(!result.contains("plugin-manager"));
        assert_eq!(result, "./features/codex: {}");
    }

    #[test]
    fn test_from_dir_loads_templates() {
        let temp_dir = tempfile::tempdir().unwrap();
        let templates_dir = temp_dir.path();

        // Create sample template files
        fs::write(templates_dir.join("test.tera"), "Hello {{name}}").unwrap();
        fs::write(templates_dir.join("other.template"), "World {{value}}").unwrap();

        let engine = TemplateEngine::from_dir(templates_dir).unwrap();

        assert_eq!(engine.templates.len(), 2);
        assert!(engine.templates.contains_key("test"));
        assert!(engine.templates.contains_key("other"));
    }

    #[test]
    fn test_from_dir_empty_dir_returns_error() {
        let temp_dir = tempfile::tempdir().unwrap();
        let templates_dir = temp_dir.path();

        let result = TemplateEngine::from_dir(templates_dir);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidTemplate(_))));
    }

    #[test]
    fn test_register_template_file_invalid_path() {
        let mut engine = TemplateEngine::new().unwrap();
        let non_existent = PathBuf::from("/non/existent/path.tera");

        let result = engine.register_template_file("test", &non_existent);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::PathNotFound(_))));
    }

    #[test]
    fn test_render_template_not_found_error() {
        let engine = TemplateEngine::new().unwrap();
        let ctx = TemplateContext::new("test".to_string(), "ubuntu:latest".to_string());

        let result = engine.render_template("nonexistent", &ctx);
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidTemplate(_))));
    }

    #[test]
    fn test_from_dir_nonexistent_path() {
        let result = TemplateEngine::from_dir("/nonexistent/path");
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::PathNotFound(_))));
    }

    #[test]
    fn test_format_plugin_list_empty() {
        let template = "{{claude_activate_plugins}}";
        let ctx = TemplateContext::new("test".to_string(), "ubuntu:latest".to_string());

        // Note: {{claude_activate_plugins}} is not a direct replacement - it's used
        // inside the plugin_manager_block expansion. This template variable won't be replaced.
        let result = render_template_simple(template, &ctx);
        assert_eq!(result, "{{claude_activate_plugins}}");
    }

    #[test]
    fn test_render_with_config_integration() {
        let config = create_test_config();
        let engine = TemplateEngine::new().unwrap();

        let result = engine.render_with_config("devcontainer.json", &config);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("./features/claude-code"));
        assert!(rendered.contains("./features/plugin-manager"));
    }

    #[test]
    fn test_render_with_codex_config_no_plugin_manager() {
        let config = create_codex_config();
        let engine = TemplateEngine::new().unwrap();

        let result = engine.render_with_config("devcontainer.json", &config);
        assert!(result.is_ok());
        let rendered = result.unwrap();
        assert!(rendered.contains("./features/codex"));
        assert!(!rendered.contains("./features/plugin-manager"));
    }

    #[test]
    fn test_build_agent_options_json_models_map() {
        let config = create_test_config();
        let json = build_agent_options_json(&config);
        let val: serde_json::Value = serde_json::from_str(&json).unwrap();
        // models must be a string (comma-separated) because devcontainer-feature.json type is "string"
        let models_str = val.get("models").unwrap().as_str().unwrap();
        assert!(models_str.contains("haiku:claude-3-5-haiku-20241022"), "got: {models_str}");
        assert!(models_str.contains("sonnet:claude-3-5-sonnet-20241022"), "got: {models_str}");
    }
}
