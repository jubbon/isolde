//! # Isolde init command
//!
//! Initialize a new Isolde project with a configuration file.

use std::fs;
use std::path::PathBuf;

use colored::Colorize;
use isolde_core::config::{Config, TemplateInfo};
use isolde_core::{Error, Result};

use super::is_agent_implemented;

/// Options for the init command
#[derive(Debug, Clone)]
pub struct InitOptions {
    /// Template to use (optional)
    pub template: Option<String>,

    /// Preset to use (optional)
    pub preset: Option<String>,

    /// Skip confirmation prompts
    pub yes: bool,

    /// Current working directory
    pub cwd: PathBuf,

    /// Project name (for non-interactive mode)
    pub name: Option<String>,

    /// Language version (optional)
    pub lang_version: Option<String>,

    /// Coding agent to use (claude-code, codex, gemini, aider)
    pub agent: String,

    /// Agent CLI version to install
    pub agent_version: String,

    /// HTTP proxy URL (optional)
    pub http_proxy: Option<String>,

    /// HTTPS proxy URL (optional)
    pub https_proxy: Option<String>,
}

impl Default for InitOptions {
    fn default() -> Self {
        Self {
            template: None,
            preset: None,
            yes: false,
            cwd: PathBuf::from("."),
            name: None,
            lang_version: None,
            agent: "claude-code".to_string(),
            agent_version: "latest".to_string(),
            http_proxy: None,
            https_proxy: None,
        }
    }
}

/// Hardcoded default language versions (fallback when template-info.yaml is unavailable)
fn default_lang_version(template: &str) -> &'static str {
    match template {
        "python" => "3.12",
        "nodejs" => "22",
        "rust" => "latest",
        "go" => "latest",
        _ => "",
    }
}

/// Try to find the templates directory
fn find_templates_dir() -> Option<PathBuf> {
    if let Ok(env_path) = std::env::var("ISOLDE_TEMPLATES") {
        let p = PathBuf::from(&env_path);
        if p.exists() {
            return Some(p);
        }
    }

    for rel in &["templates", "../templates", "../../templates"] {
        let p = PathBuf::from(rel);
        if p.exists() {
            return Some(p);
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            if let Some(prefix) = exe_dir.parent() {
                let share_path = prefix.join("share").join("isolde").join("templates");
                if share_path.exists() {
                    return Some(share_path);
                }
            }
            let p = exe_dir.join("templates");
            if p.exists() {
                return Some(p);
            }
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(&home).join(".local/share/isolde/templates");
        if p.exists() {
            return Some(p);
        }
    }

    None
}

/// Load template metadata from template-info.yaml
fn load_template_info(template: &str) -> Option<TemplateInfo> {
    let templates_dir = find_templates_dir()?;
    let info_path = templates_dir.join(template).join("template-info.yaml");
    let content = fs::read_to_string(info_path).ok()?;
    serde_yaml::from_str(&content).ok()
}

/// Resolve the effective language version for a template
fn resolve_lang_version(template: &str, user_version: Option<&str>) -> String {
    if let Some(v) = user_version {
        return v.to_string();
    }

    if let Some(info) = load_template_info(template) {
        return info.lang_version_default;
    }

    default_lang_version(template).to_string()
}

/// Warn if the language version is not in the template's supported_versions list
fn warn_unsupported_version(template: &str, version: &str) {
    if version.is_empty() {
        return;
    }

    if let Some(info) = load_template_info(template) {
        if !info.supported_versions.contains(&version.to_string()) {
            eprintln!(
                "{} {}",
                "⚠".yellow(),
                format!(
                    "Language version '{}' is not in the officially supported list for '{}' template: {}",
                    version, template, info.supported_versions.join(", ")
                ).yellow()
            );
            eprintln!(
                "{}",
                "  The project will be created, but the version may not work correctly.".dimmed()
            );
        }
    }
}

/// Generate configuration from a template name
fn generate_config_from_template(
    project_name: &str,
    template: &str,
    agent: &str,
    agent_version: &str,
    lang_version: &str,
) -> String {
    let (docker_image, runtime_section) = match template {
        "python" => (
            format!("mcr.microsoft.com/devcontainers/python:{}", lang_version),
            format!("runtime:\n  language: python\n  version: \"{}\"\n  package_manager: uv\n  tools: []\n", lang_version),
        ),
        "nodejs" => (
            format!("mcr.microsoft.com/devcontainers/javascript-node:{}", lang_version),
            format!("runtime:\n  language: nodejs\n  version: \"{}\"\n  package_manager: pnpm\n  tools: []\n", lang_version),
        ),
        "rust" => (
            "mcr.microsoft.com/devcontainers/rust:latest".to_string(),
            format!("runtime:\n  language: rust\n  version: \"{}\"\n  package_manager: cargo\n  tools: []\n", lang_version),
        ),
        "go" => (
            format!("mcr.microsoft.com/devcontainers/go:{}", lang_version),
            format!("runtime:\n  language: go\n  version: \"{}\"\n  package_manager: go\n  tools: []\n", lang_version),
        ),
        _ => (
            "mcr.microsoft.com/devcontainers/base:ubuntu".to_string(),
            String::new(),
        ),
    };

    let agent_options_section = agent_options_yaml(agent);

    format!(
        r#"# Isolde Configuration for {name}
# Generated from template: {template}
version: "0.1"
template: {template}

name: {name}
workspace:
  dir: ./project

docker:
  image: {image}
  build_args: []

# Coding agent configuration
agent:
  name: {agent}
  version: {agent_version}
  options:
{agent_options}

{runtime}
# Git configuration
git:
  generated: ignored
"#,
        name = project_name,
        template = template,
        image = docker_image,
        agent = agent,
        agent_version = agent_version,
        agent_options = agent_options_section,
        runtime = runtime_section,
    )
}

/// Generate a default isolde.yaml configuration
fn generate_default_config(project_name: &str, agent: &str, agent_version: &str) -> String {
    let agent_options_section = agent_options_yaml(agent);

    format!(
        r#"# Isolde Configuration for {name}
# Generated by isolde init
version: "0.1"

name: {name}
workspace:
  dir: ./project

docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
  build_args: []

# Coding agent configuration
agent:
  name: {agent}
  version: {agent_version}
  options:
{agent_options}

# Runtime configuration (optional)
# runtime:
#   language: python
#   version: "3.12"
#   package_manager: uv
#   tools: []

# Proxy configuration for corporate networks (optional)
# proxy:
#   http: http://proxy.corp.com:8080
#   https: http://proxy.corp.com:8080
#   no_proxy: localhost,127.0.0.1,.local

# Marketplace configurations (optional)
# marketplaces:
#   omc:
#     url: https://github.com/oh-my-claudecode/marketplace

# Plugin configurations (optional)
# plugins:
#   - marketplace: omc
#     name: oh-my-claudecode
#     activate: true

# Git configuration
git:
  generated: ignored
"#,
        name = project_name,
        agent = agent,
        agent_version = agent_version,
        agent_options = agent_options_section,
    )
}

/// Generate agent-specific options YAML block (indented for embedding in isolde.yaml)
fn agent_options_yaml(agent: &str) -> String {
    match agent {
        "claude-code" => {
            "    provider: anthropic\n    # models: # Uncomment to pin specific model versions instead of using defaults\n    #   haiku: claude-3-5-haiku-20241022\n    #   sonnet: claude-3-5-sonnet-20241022\n    #   opus: claude-3-5-sonnet-20241022".to_string()
        }
        "codex" => {
            // Codex uses OPENAI_API_KEY from the environment; no special options needed
            "    {}".to_string()
        }
        _ => {
            "    {}".to_string()
        }
    }
}

/// Generate configuration from a preset
fn generate_config_from_preset(
    project_name: &str,
    preset_name: &str,
    lang_version_override: Option<&str>,
) -> Result<String> {
    // Try to load presets.yaml from the current directory or template repository
    let preset_yaml = load_presets_yaml()?;

    // Parse the preset configuration
    let preset = find_preset(&preset_yaml, preset_name)?;

    // Use CLI --lang-version override if provided, otherwise use preset's version
    let version = lang_version_override.unwrap_or(&preset.lang_version);

    // Generate config based on preset (presets always use claude-code agent)
    let agent_options_section = agent_options_yaml("claude-code");

    // Build marketplaces + plugins sections based on whether plugins exist
    let (marketplaces_section, plugins_section) = if preset.claude_plugins.is_empty() {
        (String::new(), "plugins: []\n".to_string())
    } else {
        let marketplaces = "marketplaces:\n  omc:\n    url: https://github.com/oh-my-claudecode/marketplace\n\n".to_string();
        let plugins = format!(
            "plugins:\n{}\n",
            preset
                .claude_plugins
                .iter()
                .map(|p| format!("  - marketplace: omc\n    name: {}\n    activate: true", p))
                .collect::<Vec<_>>()
                .join("\n")
        );
        (marketplaces, plugins)
    };

    let config = format!(
        r#"# Isolde Configuration for {name}
# Generated from preset: {preset}
version: "0.1"

name: {name}
workspace:
  dir: ./project

docker:
  image: mcr.microsoft.com/devcontainers/base:ubuntu
  build_args: []

# Coding agent configuration
agent:
  name: claude-code
  version: latest
  options:
{agent_options}

runtime:
  language: {lang}
  version: "{version}"
  package_manager: auto
  tools: {tools}

# Plugin configurations
{marketplaces}{plugins}
# Git configuration
git:
  generated: ignored
"#,
        name = project_name,
        preset = preset_name,
        agent_options = agent_options_section,
        lang = preset.template,
        version = version,
        tools = serde_yaml::to_string(&preset.features).unwrap_or_else(|_| "[]".to_string()),
        marketplaces = marketplaces_section,
        plugins = plugins_section,
    );

    Ok(config)
}

/// Search upward from a directory for presets.yaml
fn search_presets_upward(start: &std::path::Path) -> Option<PathBuf> {
    let mut dir = start;
    loop {
        let candidate = dir.join("presets.yaml");
        if candidate.exists() {
            return Some(candidate);
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => return None,
        }
    }
}

/// Load presets.yaml from the current directory, binary location, or by searching upward
fn load_presets_yaml() -> Result<String> {
    // Try current directory first
    let local_path = PathBuf::from("presets.yaml");
    if local_path.exists() {
        return fs::read_to_string(&local_path).map_err(|e| {
            Error::FileError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to read presets.yaml: {}", e),
            ))
        });
    }

    // Search upward from current directory
    if let Ok(current_dir) = std::env::current_dir() {
        if let Some(found) = search_presets_upward(&current_dir) {
            return fs::read_to_string(&found).map_err(|e| {
                Error::FileError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Failed to read presets.yaml: {}", e),
                ))
            });
        }
    }

    // Search upward from the binary's location (handles cases where cwd is outside the repo)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Some(found) = search_presets_upward(exe_dir) {
                return fs::read_to_string(&found).map_err(|e| {
                    Error::FileError(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Failed to read presets.yaml: {}", e),
                    ))
                });
            }
        }
    }

    Err(Error::InvalidTemplate(
        "presets.yaml not found. Please run this command from the Isolde repository root."
            .to_string(),
    ))
}

/// Find and extract a preset from the YAML
fn find_preset(yaml: &str, preset_name: &str) -> Result<PresetData> {
    use isolde_core::config::Preset;

    // Parse the presets YAML
    let value: serde_yaml::Value = serde_yaml::from_str(yaml)
        .map_err(|e| Error::InvalidTemplate(format!("Failed to parse presets.yaml: {}", e)))?;

    let presets = value
        .get("presets")
        .and_then(|v| v.as_mapping())
        .ok_or_else(|| Error::InvalidTemplate("No presets found in presets.yaml".to_string()))?;

    let preset_value = presets
        .get(serde_yaml::Value::String(preset_name.to_string()))
        .ok_or_else(|| Error::PresetNotFound(preset_name.to_string()))?;

    // Try to parse as Preset
    let preset: Preset = serde_yaml::from_value(preset_value.clone()).map_err(|e| {
        Error::InvalidTemplate(format!("Failed to parse preset '{}': {}", preset_name, e))
    })?;

    Ok(PresetData {
        template: preset.template.clone(),
        lang_version: preset.lang_version.clone(),
        features: preset.features.clone(),
        claude_plugins: preset.claude_plugins.clone(),
    })
}

/// Simplified preset data structure
#[derive(Debug, Clone)]
struct PresetData {
    template: String,
    lang_version: String,
    features: Vec<String>,
    claude_plugins: Vec<String>,
}

/// Valid template names
const VALID_TEMPLATES: &[&str] = &["python", "nodejs", "rust", "go", "minimal", "generic"];

/// Validate language version for the given template
fn validate_lang_version(template: Option<&str>, version: &str) -> Result<()> {
    // "latest" and empty string are always valid (use template defaults)
    if version == "latest" || version.is_empty() {
        return Ok(());
    }

    let is_valid = match template {
        Some("python") => {
            // Python versions: 3.x or 3.x.y where x is a reasonable minor version
            version.starts_with("3.") && {
                let minor = version.trim_start_matches("3.");
                let minor_num: Option<u32> = minor.split('.').next().and_then(|s| s.parse().ok());
                minor_num.map_or(false, |n| n <= 20)
            }
        }
        Some("nodejs") => {
            // Node.js versions: single number (18, 20, 22, etc.) or major.minor
            let major: Option<u32> = version.split('.').next().and_then(|s| s.parse().ok());
            major.map_or(false, |n| n >= 14 && n <= 30)
        }
        Some("rust") => {
            // Rust: stable, nightly, beta, or semver
            matches!(version, "stable" | "nightly" | "beta") || {
                let parts: Vec<&str> = version.split('.').collect();
                parts.len() >= 2 && parts[0].parse::<u32>().is_ok()
            }
        }
        Some("go") => {
            // Go versions: 1.x or 1.x.y
            version.starts_with("1.") && {
                let minor = version.trim_start_matches("1.");
                let minor_num: Option<u32> = minor.split('.').next().and_then(|s| s.parse().ok());
                minor_num.map_or(false, |n| n <= 30)
            }
        }
        _ => {
            // For unknown templates or minimal, do a basic sanity check
            // Version numbers shouldn't have major version > 50
            let major: Option<u32> = version.split('.').next().and_then(|s| s.parse().ok());
            major.map_or(true, |n| n <= 50)
        }
    };

    if is_valid {
        Ok(())
    } else {
        Err(Error::Other(format!(
            "Invalid language version '{}' for template '{}'",
            version,
            template.unwrap_or("unknown")
        )))
    }
}

/// Run the init command
pub fn run(opts: InitOptions) -> Result<()> {
    let config_path = opts.cwd.join("isolde.yaml");

    // Check if isolde.yaml already exists
    if config_path.exists() {
        return Err(Error::Other(format!(
            "isolde.yaml already exists at {}. Use --force to overwrite.",
            config_path.display()
        )));
    }

    // Validate template name if provided
    if let Some(ref template) = opts.template {
        if !VALID_TEMPLATES.contains(&template.as_str()) {
            return Err(Error::InvalidTemplate(format!(
                "Unknown template '{}'. Valid templates: {}",
                template,
                VALID_TEMPLATES.join(", ")
            )));
        }
    }

    // Validate lang_version if provided
    if let Some(ref version) = opts.lang_version {
        if let Err(e) = validate_lang_version(opts.template.as_deref(), version) {
            return Err(e);
        }
    }

    // Determine project name
    let project_name = opts.name.unwrap_or_else(|| {
        opts.cwd
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-project")
            .to_string()
    });

    // Validate proxy URLs if provided
    if let Some(ref proxy_url) = opts.http_proxy {
        if !proxy_url.starts_with("http://") && !proxy_url.starts_with("https://") {
            return Err(Error::InvalidTemplate(format!(
                "Invalid proxy URL '{}': proxy URL must start with http:// or https://",
                proxy_url
            )));
        }
    }
    if let Some(ref proxy_url) = opts.https_proxy {
        if !proxy_url.starts_with("http://") && !proxy_url.starts_with("https://") {
            return Err(Error::InvalidTemplate(format!(
                "Invalid proxy URL '{}': proxy URL must start with http:// or https://",
                proxy_url
            )));
        }
    }

    // Warn about stub agents
    if !is_agent_implemented(&opts.agent) {
        eprintln!(
            "\n{} {}",
            "⚠".yellow(),
            format!(
                "Agent '{}' is experimental — its devcontainer feature has no install.sh yet.",
                opts.agent
            )
            .yellow()
        );
        eprintln!(
            "{}",
            "  The devcontainer will be created but the agent CLI won't be automatically installed.".dimmed()
        );

        if !opts.yes {
            print!("{}", "\nContinue anyway? [y/N] ".bold());
            use std::io::Write;
            let _ = std::io::stdout().flush();

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .map_err(|e| Error::Other(format!("Failed to read input: {}", e)))?;

            let input = input.trim().to_lowercase();
            if input != "y" && input != "yes" {
                println!("{}", "Aborted.".yellow());
                return Ok(());
            }
        }
    }

    // Resolve effective language version and validate
    if let Some(ref template) = opts.template {
        let effective_version = resolve_lang_version(template, opts.lang_version.as_deref());
        warn_unsupported_version(template, &effective_version);
    }

    // Generate configuration
    let config_content = if let Some(preset) = &opts.preset {
        print!(
            "{}",
            format!("Loading preset '{}'...\n", preset.cyan()).dimmed()
        );
        generate_config_from_preset(&project_name, preset, opts.lang_version.as_deref())?
    } else if let Some(ref template) = opts.template {
        let effective_version = resolve_lang_version(template, opts.lang_version.as_deref());
        generate_config_from_template(
            &project_name,
            template,
            &opts.agent,
            &opts.agent_version,
            &effective_version,
        )
    } else {
        generate_default_config(&project_name, &opts.agent, &opts.agent_version)
    };

    // Validate the configuration
    let config = Config::from_str(&config_content)?;

    // Show preview
    println!("\n{}", "Configuration preview:".bold());
    println!("{}", "─".repeat(50));
    println!("  Name: {}", config.name.cyan());
    println!("  Schema Version: {}", config.version);
    println!("  Workspace: {}", config.workspace_dir());
    println!("  Docker Image: {}", config.docker_image());
    println!("  Agent: {}", config.agent_name());
    if let Some(runtime) = config.runtime() {
        println!("  Runtime: {} {}", runtime.language(), runtime.version());
    }
    println!("{}", "─".repeat(50));

    // Ask for confirmation
    if !opts.yes {
        print!(
            "\n{}",
            "Create isolde.yaml with this configuration? [Y/n] ".bold()
        );
        use std::io::Write;
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| Error::Other(format!("Failed to read input: {}", e)))?;

        let input = input.trim().to_lowercase();
        if !input.is_empty() && input != "y" && input != "yes" {
            println!("{}", "Aborted.".yellow());
            return Ok(());
        }
    }

    // Write configuration file
    fs::write(&config_path, config_content).map_err(|e| {
        Error::FileError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write isolde.yaml: {}", e),
        ))
    })?;

    println!(
        "\n{} {}",
        "✔".green(),
        format!("Created isolde.yaml at {}", config_path.display()).green()
    );
    println!(
        "{}",
        "Run 'isolde sync' to generate the devcontainer configuration.".dimmed()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_default_config() {
        let config = generate_default_config("test-project", "claude-code", "latest");
        assert!(config.contains("name: test-project"));
        assert!(config.contains("workspace:"));
        assert!(config.contains("docker:"));
        assert!(config.contains("agent:"));
        assert!(config.contains("name: claude-code"));
        assert!(!config.contains("claude:"));
    }

    #[test]
    fn test_generate_default_config_codex() {
        let config = generate_default_config("test-project", "codex", "latest");
        assert!(config.contains("name: codex"));
        assert!(!config.contains("provider: anthropic"));
    }

    #[test]
    fn test_generate_config_from_template() {
        let config =
            generate_config_from_template("my-app", "python", "claude-code", "latest", "3.12");
        assert!(config.contains("name: my-app"));
        assert!(config.contains("agent:"));
        assert!(config.contains("name: claude-code"));
        assert!(config.contains("python"));
        assert!(config.contains("version: \"3.12\""));
    }

    #[test]
    fn test_generate_config_with_custom_lang_version() {
        let config =
            generate_config_from_template("my-app", "python", "claude-code", "latest", "3.11");
        assert!(config.contains("version: \"3.11\""));
        assert!(config.contains("mcr.microsoft.com/devcontainers/python:3.11"));
    }

    #[test]
    fn test_generate_config_nodejs_lang_version() {
        let config =
            generate_config_from_template("my-app", "nodejs", "claude-code", "latest", "20");
        assert!(config.contains("version: \"20\""));
        assert!(config.contains("mcr.microsoft.com/devcontainers/javascript-node:20"));
    }

    #[test]
    fn test_generate_config_go_lang_version() {
        let config = generate_config_from_template("my-app", "go", "claude-code", "latest", "1.21");
        assert!(config.contains("version: \"1.21\""));
        assert!(config.contains("mcr.microsoft.com/devcontainers/go:1.21"));
    }

    #[test]
    fn test_generate_config_rust_always_latest_image() {
        let config =
            generate_config_from_template("my-app", "rust", "claude-code", "latest", "stable");
        assert!(config.contains("version: \"stable\""));
        // Rust docker image always uses :latest regardless of lang_version
        assert!(config.contains("mcr.microsoft.com/devcontainers/rust:latest"));
    }

    #[test]
    fn test_is_agent_implemented() {
        assert!(is_agent_implemented("claude-code"));
        assert!(is_agent_implemented("codex"));
        assert!(!is_agent_implemented("gemini"));
        assert!(!is_agent_implemented("aider"));
    }

    #[test]
    fn test_default_lang_version() {
        assert_eq!(default_lang_version("python"), "3.12");
        assert_eq!(default_lang_version("nodejs"), "22");
        assert_eq!(default_lang_version("rust"), "latest");
        assert_eq!(default_lang_version("go"), "latest");
        assert_eq!(default_lang_version("unknown"), "");
    }

    #[test]
    fn test_init_options_default() {
        let opts = InitOptions::default();
        assert!(opts.template.is_none());
        assert!(opts.preset.is_none());
        assert!(!opts.yes);
        assert_eq!(opts.agent, "claude-code");
        assert_eq!(opts.agent_version, "latest");
    }
}
