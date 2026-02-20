//! # Isolde validate command
//!
//! Validate project configuration and generated artifacts.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use colored::Colorize;
use isolde_core::config::Config;
use isolde_core::{Error, Result};
use serde::Serialize;

/// Options for the validate command
#[derive(Debug, Clone)]
pub struct ValidateOptions {
    /// Quick validation (skip expensive checks)
    pub quick: bool,

    /// Verbose output
    pub verbose: bool,

    /// Fail on warnings
    pub warnings_as_errors: bool,

    /// Output format
    pub format: ValidateFormat,

    /// Specific paths to check
    pub paths: Vec<PathBuf>,
}

/// Output format for validation results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidateFormat {
    Text,
    Json,
    Sarif,
}

impl ValidateFormat {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "text" => Some(Self::Text),
            "json" => Some(Self::Json),
            "sarif" => Some(Self::Sarif),
            _ => None,
        }
    }
}

impl Default for ValidateOptions {
    fn default() -> Self {
        Self {
            quick: false,
            verbose: false,
            warnings_as_errors: false,
            format: ValidateFormat::Text,
            paths: Vec::new(),
        }
    }
}

/// Result of a single validation check
#[derive(Debug, Clone, Serialize)]
pub struct CheckResult {
    /// Check name
    pub name: String,
    /// Check description
    pub description: String,
    /// Check status
    pub status: CheckStatus,
    /// Additional details
    pub details: Vec<String>,
}

/// Status of a validation check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CheckStatus {
    Passed,
    Failed,
    Warning,
    Skipped,
}

/// Overall validation report
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether configuration is valid
    pub config_valid: bool,
    /// Whether build succeeded (if checked)
    pub build_success: Option<bool>,
    /// Whether Claude is installed (if checked)
    pub claude_installed: Option<bool>,
    /// Individual check results
    pub checks: Vec<CheckResult>,
    /// Total errors
    pub error_count: usize,
    /// Total warnings
    pub warning_count: usize,
}

impl ValidationReport {
    /// Whether validation passed (no errors, and warnings are treated as errors if configured)
    pub fn passed(&self, warnings_as_errors: bool) -> bool {
        if self.error_count > 0 {
            return false;
        }
        if warnings_as_errors && self.warning_count > 0 {
            return false;
        }
        true
    }
}

/// Run the validate command
pub fn run(opts: ValidateOptions) -> Result<ValidationReport> {
    let cwd = std::env::current_dir()
        .map_err(|e| Error::Other(format!("Failed to get current directory: {}", e)))?;

    println!("{}", "🔍 Validating Isolde configuration...".cyan());
    println!("{}", "─".repeat(50).dimmed());

    let mut checks = Vec::new();
    let mut error_count = 0;
    let mut warning_count = 0;

    // Check 1: isolde.yaml exists
    let config_path = cwd.join("isolde.yaml");
    let check_config_exists = check_file_exists(
        &config_path,
        "isolde.yaml",
        "Configuration file",
        opts.verbose,
    );
    error_count += (check_config_exists.status == CheckStatus::Failed) as usize;
    warning_count += (check_config_exists.status == CheckStatus::Warning) as usize;
    checks.push(check_config_exists);

    // Only continue with config-dependent checks if config exists
    let config = if config_path.exists() {
        match Config::from_file(&config_path) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                let check = CheckResult {
                    name: "config-parse".to_string(),
                    description: "Parse configuration file".to_string(),
                    status: CheckStatus::Failed,
                    details: vec![e.to_string()],
                };
                error_count += 1;
                checks.push(check);
                None
            }
        }
    } else {
        None
    };

    // Check 2: Configuration validity
    if let Some(ref cfg) = config {
        let check = check_config_validity(cfg, opts.verbose);
        error_count += (check.status == CheckStatus::Failed) as usize;
        warning_count += (check.status == CheckStatus::Warning) as usize;
        checks.push(check.clone());
    }

    // Check 3: Docker is installed
    let check_docker = check_docker_installed(opts.verbose);
    error_count += (check_docker.status == CheckStatus::Failed) as usize;
    warning_count += (check_docker.status == CheckStatus::Warning) as usize;
    checks.push(check_docker.clone());

    // Check 4: Docker daemon is running
    if check_docker.status != CheckStatus::Failed {
        let check = check_docker_running(opts.verbose);
        error_count += (check.status == CheckStatus::Failed) as usize;
        warning_count += (check.status == CheckStatus::Warning) as usize;
        checks.push(check);
    }

    // Check 5: .devcontainer directory exists and is valid
    let check_devcontainer = check_devcontainer_valid(&cwd, opts.verbose);
    error_count += (check_devcontainer.status == CheckStatus::Failed) as usize;
    warning_count += (check_devcontainer.status == CheckStatus::Warning) as usize;
    checks.push(check_devcontainer.clone());

    // Check 6: Core features exist
    let check_features = check_core_features(&cwd, opts.verbose);
    error_count += (check_features.status == CheckStatus::Failed) as usize;
    warning_count += (check_features.status == CheckStatus::Warning) as usize;
    checks.push(check_features.clone());

    // Check 7: Git repositories
    let check_git = check_git_repos(&cwd, opts.verbose);
    error_count += (check_git.status == CheckStatus::Failed) as usize;
    warning_count += (check_git.status == CheckStatus::Warning) as usize;
    checks.push(check_git);

    // Check 8: Build test (skip in quick mode)
    let build_success = if !opts.quick && config.is_some() {
        let check = check_docker_build(&cwd, opts.verbose);
        error_count += (check.status == CheckStatus::Failed) as usize;
        warning_count += (check.status == CheckStatus::Warning) as usize;
        checks.push(check.clone());
        Some(check.status == CheckStatus::Passed)
    } else {
        if opts.quick {
            let check = CheckResult {
                name: "docker-build".to_string(),
                description: "Build Docker image".to_string(),
                status: CheckStatus::Skipped,
                details: vec!["Skipped in quick mode".to_string()],
            };
            checks.push(check);
        }
        None
    };

    // Check 9: Claude installation (skip in quick mode)
    let claude_installed = if !opts.quick {
        let check = check_claude_installed(opts.verbose);
        error_count += (check.status == CheckStatus::Failed) as usize;
        warning_count += (check.status == CheckStatus::Warning) as usize;
        checks.push(check.clone());
        Some(check.status == CheckStatus::Passed)
    } else {
        None
    };

    // Check 10: Custom path checks
    for path in &opts.paths {
        let check = check_path_valid(path, opts.verbose);
        error_count += (check.status == CheckStatus::Failed) as usize;
        warning_count += (check.status == CheckStatus::Warning) as usize;
        checks.push(check);
    }

    // Print results
    println!("{}", "─".repeat(50).dimmed());
    print_checks(&checks, opts.format);

    let config_valid = checks.iter().any(|c| c.name == "config-valid" && c.status == CheckStatus::Passed);
    let report = ValidationReport {
        config_valid,
        build_success,
        claude_installed,
        checks,
        error_count,
        warning_count,
    };

    Ok(report)
}

/// Get icon for check status
fn status_icon(status: CheckStatus) -> colored::ColoredString {
    match status {
        CheckStatus::Passed => "✔".green(),
        CheckStatus::Failed => "✗".red(),
        CheckStatus::Warning => "⚠".yellow(),
        CheckStatus::Skipped => "⊘".dimmed(),
    }
}

/// Get SARIF level for check status
fn status_to_sarif_level(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Failed => "error",
        CheckStatus::Warning => "warning",
        _ => "note",
    }
}

/// Print check results based on format
fn print_checks(checks: &[CheckResult], format: ValidateFormat) {
    match format {
        ValidateFormat::Text => {
            for check in checks {
                println!("{} {}", status_icon(check.status), check.description.bold());
                for detail in &check.details {
                    println!("    {}", detail.dimmed());
                }
            }
        }
        ValidateFormat::Json => {
            let json = serde_json::to_string_pretty(checks).unwrap_or_default();
            println!("{}", json);
        }
        ValidateFormat::Sarif => {
            let sarif = serde_json::json!({
                "version": "2.1.0",
                "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
                "runs": [{
                    "tool": {
                        "driver": {
                            "name": "isolde",
                            "version": env!("ISOLDE_VERSION"),
                            "informationUri": "https://github.com/dmanakulikov/isolde"
                        }
                    },
                    "results": checks.iter().filter_map(|check| {
                        if matches!(check.status, CheckStatus::Failed | CheckStatus::Warning) {
                            Some(serde_json::json!({
                                "ruleId": check.name,
                                "level": status_to_sarif_level(check.status),
                                "message": {
                                    "text": check.details.first().unwrap_or(&check.description)
                                }
                            }))
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>()
                }]
            });
            println!("{}", serde_json::to_string_pretty(&sarif).unwrap_or_default());
        }
    }
}

/// Check if a file exists
fn check_file_exists(path: &Path, name: &str, description: &str, verbose: bool) -> CheckResult {
    let status = if path.exists() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed
    };

    let mut details = Vec::new();
    if verbose || status == CheckStatus::Failed {
        details.push(format!(
            "Path: {}",
            path.display().to_string().cyan()
        ));
    }
    if status == CheckStatus::Failed {
        details.push("File not found".to_string());
    }

    CheckResult {
        name: name.to_string(),
        description: description.to_string(),
        status,
        details,
    }
}

/// Check configuration validity
fn check_config_validity(config: &Config, verbose: bool) -> CheckResult {
    let mut status = CheckStatus::Passed;
    let mut details = Vec::new();

    if verbose {
        details.push(format!("Project name: {}", config.name.cyan()));
        details.push(format!("Docker image: {}", config.docker.image.cyan()));
        details.push(format!("Claude provider: {}", config.claude.provider.cyan()));
    }

    // Validate provider
    let valid_providers = ["anthropic", "openai", "bedrock", "vertex", "azure"];
    if !valid_providers.contains(&config.claude.provider.as_str()) {
        status = CheckStatus::Failed;
        details.push(format!("Invalid provider: {}", config.claude.provider));
    }

    // Check for deprecated settings
    if config.docker.image.contains("devcontainers/base") {
        details.push("Note: Using base devcontainer image".to_string());
    }

    CheckResult {
        name: "config-valid".to_string(),
        description: "Configuration validity".to_string(),
        status,
        details,
    }
}

/// Check if Docker is installed
fn check_docker_installed(verbose: bool) -> CheckResult {
    let result = Command::new("docker")
        .arg("--version")
        .output();

    let (status, details) = match result {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            (CheckStatus::Passed, vec![version.trim().to_string()])
        }
        Err(e) => (
            CheckStatus::Failed,
            vec![
                "Docker is not installed or not in PATH".to_string(),
                format!("Error: {}", e),
            ],
        ),
    };

    CheckResult {
        name: "docker-installed".to_string(),
        description: "Docker installation".to_string(),
        status,
        details,
    }
}

/// Check if Docker daemon is running
fn check_docker_running(verbose: bool) -> CheckResult {
    let result = Command::new("docker")
        .args(["info", "--format", "{{.ServerVersion}}"])
        .output();

    let (status, details) = match result {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            (CheckStatus::Passed, vec![format!("Docker version: {}", version.trim())])
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            (
                CheckStatus::Failed,
                vec![
                    "Docker daemon is not running".to_string(),
                    format!("Error: {}", stderr.trim()),
                ],
            )
        }
        Err(e) => (
            CheckStatus::Failed,
            vec![
                "Failed to check Docker status".to_string(),
                format!("Error: {}", e),
            ],
        ),
    };

    CheckResult {
        name: "docker-running".to_string(),
        description: "Docker daemon status".to_string(),
        status,
        details,
    }
}

/// Check devcontainer directory
fn check_devcontainer_valid(cwd: &Path, verbose: bool) -> CheckResult {
    let devcontainer_dir = cwd.join(".devcontainer");

    if !devcontainer_dir.exists() {
        return CheckResult {
            name: "devcontainer-dir".to_string(),
            description: "Devcontainer directory".to_string(),
            status: CheckStatus::Warning,
            details: vec![
                "Run 'isolde sync' to create devcontainer configuration".to_string(),
            ],
        };
    }

    let mut status = CheckStatus::Passed;
    let mut details = Vec::new();

    // Check for devcontainer.json
    let devcontainer_json = devcontainer_dir.join("devcontainer.json");
    if !devcontainer_json.exists() {
        status = CheckStatus::Failed;
        details.push("Missing devcontainer.json".to_string());
    } else {
        details.push("devcontainer.json: found".to_string());

        // Try to validate JSON
        if let Ok(content) = fs::read_to_string(&devcontainer_json) {
            if serde_json::from_str::<serde_json::Value>(&content).is_err() {
                status = CheckStatus::Failed;
                details.push("devcontainer.json: invalid JSON".to_string());
            } else if verbose {
                details.push("devcontainer.json: valid JSON".to_string());
            }
        }
    }

    // Check for Dockerfile
    let dockerfile = devcontainer_dir.join("Dockerfile");
    if !dockerfile.exists() {
        status = CheckStatus::Warning;
        details.push("Missing Dockerfile".to_string());
    } else if verbose {
        details.push("Dockerfile: found".to_string());
    }

    // Check for features directory
    let features_dir = devcontainer_dir.join("features");
    if !features_dir.exists() {
        status = CheckStatus::Warning;
        details.push("Missing features directory".to_string());
    } else if verbose {
        details.push("features directory: found".to_string());
    }

    CheckResult {
        name: "devcontainer-dir".to_string(),
        description: "Devcontainer configuration".to_string(),
        status,
        details,
    }
}

/// Check core features
fn check_core_features(cwd: &Path, verbose: bool) -> CheckResult {
    let features_dir = cwd.join(".devcontainer/features");

    if !features_dir.exists() {
        return CheckResult {
            name: "core-features".to_string(),
            description: "Core features".to_string(),
            status: CheckStatus::Warning,
            details: vec![
                "Core features directory not found".to_string(),
                "Run 'isolde sync' to copy core features".to_string(),
            ],
        };
    }

    let expected_features = ["claude-code", "proxy", "plugin-manager"];
    let mut missing = Vec::new();
    let mut found = Vec::new();

    for feature in &expected_features {
        let feature_path = features_dir.join(feature);
        if feature_path.exists() {
            found.push(feature.to_string());
        } else {
            missing.push(feature.to_string());
        }
    }

    let status = if missing.is_empty() {
        CheckStatus::Passed
    } else {
        CheckStatus::Warning
    };

    let mut details = Vec::new();
    if verbose || !missing.is_empty() {
        if !found.is_empty() {
            details.push(format!("Found features: {}", found.join(", ").cyan()));
        }
        if !missing.is_empty() {
            details.push(format!("Missing features: {}", missing.join(", ").yellow()));
        }
    }

    CheckResult {
        name: "core-features".to_string(),
        description: "Core features availability".to_string(),
        status,
        details,
    }
}

/// Check git repositories
fn check_git_repos(cwd: &Path, verbose: bool) -> CheckResult {
    let mut status = CheckStatus::Passed;
    let mut details = Vec::new();

    // Check for .git in current directory
    let git_dir = cwd.join(".git");
    if git_dir.exists() {
        details.push("Project git repository: initialized".to_string());
    } else if verbose {
        details.push("Project git repository: not initialized".to_string());
    }

    // Check for devcontainer git repo
    let devcontainer_git = cwd.join(".devcontainer/.git");
    if devcontainer_git.exists() {
        details.push("Devcontainer git repository: initialized".to_string());
    } else {
        status = CheckStatus::Warning;
        details.push("Devcontainer git repository: not initialized".to_string());
        details.push("Run 'isolde sync' to initialize repositories".to_string());
    }

    CheckResult {
        name: "git-repos".to_string(),
        description: "Git repositories".to_string(),
        status,
        details,
    }
}

/// Check Docker build (expensive operation)
fn check_docker_build(cwd: &Path, verbose: bool) -> CheckResult {
    let devcontainer_dir = cwd.join(".devcontainer");
    let dockerfile = devcontainer_dir.join("Dockerfile");

    if !dockerfile.exists() {
        return CheckResult {
            name: "docker-build".to_string(),
            description: "Docker build test".to_string(),
            status: CheckStatus::Skipped,
            details: vec!["Dockerfile not found, skipping build test".to_string()],
        };
    }

    if verbose {
        println!("    {} Building Docker image (this may take a while)...", "⏳".dimmed());
    }

    let result = Command::new("docker")
        .args([
            "build",
            "-q",
            "-f",
            dockerfile.to_str().unwrap(),
            "-t",
            "isolde-validate-test",
            &devcontainer_dir.to_str().unwrap(),
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            // Clean up the test image
            let _ = Command::new("docker")
                .args(["rmi", "isolde-validate-test", "-f"])
                .status();

            CheckResult {
                name: "docker-build".to_string(),
                description: "Docker build test".to_string(),
                status: CheckStatus::Passed,
                details: vec!["Docker image built successfully".to_string()],
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            CheckResult {
                name: "docker-build".to_string(),
                description: "Docker build test".to_string(),
                status: CheckStatus::Failed,
                details: vec![
                    "Docker build failed".to_string(),
                    format!("Error: {}", stderr.lines().next().unwrap_or("unknown error")),
                ],
            }
        }
        Err(e) => CheckResult {
            name: "docker-build".to_string(),
            description: "Docker build test".to_string(),
            status: CheckStatus::Failed,
            details: vec![
                "Failed to run docker build".to_string(),
                format!("Error: {}", e),
            ],
        },
    }
}

/// Check Claude installation
fn check_claude_installed(verbose: bool) -> CheckResult {
    let result = Command::new("claude")
        .arg("--version")
        .output();

    let (status, details) = match result {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            (CheckStatus::Passed, vec![version.trim().to_string()])
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            (
                CheckStatus::Warning,
                vec![
                    "Claude Code CLI not found or not working".to_string(),
                    format!("Error: {}", stderr.trim()),
                    "Claude will be installed in the devcontainer".to_string(),
                ],
            )
        }
        Err(_) => (
            CheckStatus::Warning,
            vec![
                "Claude Code CLI not found".to_string(),
                "Claude will be installed in the devcontainer".to_string(),
            ],
        ),
    };

    CheckResult {
        name: "claude-installed".to_string(),
        description: "Claude Code CLI installation".to_string(),
        status,
        details,
    }
}

/// Check a custom path
fn check_path_valid(path: &Path, verbose: bool) -> CheckResult {
    let status = if path.exists() {
        CheckStatus::Passed
    } else {
        CheckStatus::Failed
    };

    let mut details = Vec::new();
    details.push(format!("Path: {}", path.display().to_string().cyan()));
    if status == CheckStatus::Failed {
        details.push("Path does not exist".to_string());
    } else if verbose {
        if path.is_dir() {
            details.push("Directory exists".to_string());
        } else {
            details.push("File exists".to_string());
        }
    }

    CheckResult {
        name: format!("path-check-{}", path.display()),
        description: format!("Path check: {}", path.display()),
        status,
        details,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_options_default() {
        let opts = ValidateOptions::default();
        assert!(!opts.quick);
        assert!(!opts.verbose);
        assert!(!opts.warnings_as_errors);
        assert_eq!(opts.format, ValidateFormat::Text);
    }

    #[test]
    fn test_validate_format_from_str() {
        assert_eq!(ValidateFormat::from_str("text"), Some(ValidateFormat::Text));
        assert_eq!(ValidateFormat::from_str("TEXT"), Some(ValidateFormat::Text));
        assert_eq!(ValidateFormat::from_str("json"), Some(ValidateFormat::Json));
        assert_eq!(ValidateFormat::from_str("sarif"), Some(ValidateFormat::Sarif));
        assert_eq!(ValidateFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_validation_report_passed() {
        let report = ValidationReport {
            config_valid: true,
            build_success: None,
            claude_installed: None,
            checks: vec![],
            error_count: 0,
            warning_count: 0,
        };
        assert!(report.passed(false));
        assert!(report.passed(true));

        let report_with_warnings = ValidationReport {
            config_valid: true,
            build_success: None,
            claude_installed: None,
            checks: vec![],
            error_count: 0,
            warning_count: 1,
        };
        assert!(report_with_warnings.passed(false));
        assert!(!report_with_warnings.passed(true));

        let report_with_errors = ValidationReport {
            config_valid: false,
            build_success: None,
            claude_installed: None,
            checks: vec![],
            error_count: 1,
            warning_count: 0,
        };
        assert!(!report_with_errors.passed(false));
        assert!(!report_with_errors.passed(true));
    }

    #[test]
    fn test_check_file_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let existing_file = temp_dir.path().join("existing.txt");
        fs::write(&existing_file, "test").unwrap();

        let non_existing_file = temp_dir.path().join("non-existing.txt");

        let check_existing = check_file_exists(&existing_file, "test", "Test file", false);
        assert_eq!(check_existing.status, CheckStatus::Passed);

        let check_non_existing = check_file_exists(&non_existing_file, "test", "Test file", false);
        assert_eq!(check_non_existing.status, CheckStatus::Failed);
    }

    #[test]
    fn test_check_config_validity() {
        let config_yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: anthropic
"#;
        let config = Config::from_str(config_yaml).unwrap();
        let check = check_config_validity(&config, false);
        assert_eq!(check.status, CheckStatus::Passed);
    }

    #[test]
    fn test_check_config_validity_invalid_provider() {
        let config_yaml = r#"
name: test
version: 1.0.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
claude:
  provider: vertex
"#;
        let config = Config::from_str(config_yaml).unwrap();
        let check = check_config_validity(&config, false);
        assert_eq!(check.status, CheckStatus::Passed);
    }
}
