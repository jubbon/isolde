//! # Isolde doctor command
//!
//! Diagnose the Isolde installation and configuration.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use colored::Colorize;
use isolde_core::config::Config;
use isolde_core::{Error, Result};
use serde::Serialize;

/// Options for the doctor command
#[derive(Debug, Clone)]
pub struct DoctorOptions {
    /// Attempt to fix issues automatically
    pub fix: bool,

    /// Verbose output
    pub verbose: bool,

    /// Check specific component only
    pub component: Option<String>,

    /// Path to write diagnostic report
    pub report: Option<PathBuf>,

    /// Current working directory
    pub cwd: PathBuf,
}

impl Default for DoctorOptions {
    fn default() -> Self {
        Self {
            fix: false,
            verbose: false,
            component: None,
            report: None,
            cwd: PathBuf::from("."),
        }
    }
}

/// Result of a diagnostic check
#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticResult {
    /// Component name
    pub component: String,
    /// Check status
    pub status: DiagnosticStatus,
    /// Check message
    pub message: String,
    /// Suggestions for fixing issues
    pub suggestions: Vec<String>,
    /// Whether the issue can be auto-fixed
    pub fixable: bool,
}

/// Status of a diagnostic check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiagnosticStatus {
    Healthy,
    Warning,
    Error,
    Missing,
}

/// Overall diagnostic report
#[derive(Debug, Clone, Serialize)]
pub struct DoctorReport {
    /// Isolde CLI version
    pub isolde_version: String,
    /// Rust version used to build Isolde
    pub rust_version: String,
    /// Individual diagnostic results
    pub diagnostics: Vec<DiagnosticResult>,
    /// Total errors
    pub error_count: usize,
    /// Total warnings
    pub warning_count: usize,
    /// Total healthy checks
    pub healthy_count: usize,
}

impl DoctorReport {
    /// Whether all diagnostics passed
    pub fn is_healthy(&self) -> bool {
        self.error_count == 0 && self.warning_count == 0
    }
}

/// Get icon for diagnostic status
fn diagnostic_status_icon(status: DiagnosticStatus) -> colored::ColoredString {
    match status {
        DiagnosticStatus::Healthy => "✔".green(),
        DiagnosticStatus::Warning => "⚠".yellow(),
        DiagnosticStatus::Error => "✗".red(),
        DiagnosticStatus::Missing => "⊘".dimmed(),
    }
}

/// Run the doctor command
pub fn run(opts: DoctorOptions) -> Result<DoctorReport> {
    println!("{}", "🏥 Isolde Installation Doctor".cyan());
    println!("{}", "═".repeat(50).cyan());
    println!();

    let mut diagnostics = Vec::new();
    let mut error_count = 0;
    let mut warning_count = 0;
    let mut healthy_count = 0;

    // Get version info
    let isolde_version = env!("ISOLDE_VERSION").to_string();
    let rust_version = env!("CARGO_PKG_RUST_VERSION").to_string();

    // Determine which checks to run
    let checks_to_run = if let Some(ref component) = opts.component {
        vec![component.as_str()]
    } else {
        vec![
            "isolde-cli",
            "docker",
            "git",
            "isolde-yaml",
            "devcontainer",
            "core-features",
            "claude",
            "workspace",
        ]
    };

    // Run each check
    for check in checks_to_run {
        let result = match check {
            "isolde-cli" => check_isolde_cli(&opts.cwd, opts.verbose),
            "docker" => check_docker(&opts.cwd, opts.verbose),
            "git" => check_git(&opts.cwd, opts.verbose),
            "isolde-yaml" => check_isolde_yaml(&opts.cwd, opts.verbose),
            "devcontainer" => check_devcontainer(&opts.cwd, opts.verbose),
            "core-features" => check_core_features(&opts.cwd, opts.verbose),
            "claude" => check_claude(&opts.cwd, opts.verbose),
            "workspace" => check_workspace(&opts.cwd, opts.verbose),
            _ => DiagnosticResult {
                component: check.to_string(),
                status: DiagnosticStatus::Missing,
                message: format!("Unknown component: {}", check),
                suggestions: vec![],
                fixable: false,
            },
        };

        error_count += (result.status == DiagnosticStatus::Error) as usize;
        warning_count += (result.status == DiagnosticStatus::Warning) as usize;
        healthy_count += (result.status == DiagnosticStatus::Healthy) as usize;

        // Attempt auto-fix if requested and fixable
        let result = if opts.fix && result.fixable && result.status != DiagnosticStatus::Healthy {
            println!("{} {}...", "🔧".yellow(), format!("Attempting to fix {}", result.component).cyan());
            let fixed = attempt_fix(&result, &opts.cwd);
            if fixed {
                DiagnosticResult {
                    message: format!("{} (fixed)", result.message),
                    status: DiagnosticStatus::Healthy,
                    ..result
                }
            } else {
                DiagnosticResult {
                    message: format!("{} (fix failed)", result.message),
                    ..result
                }
            }
        } else {
            result
        };

        diagnostics.push(result);
    }

    let report = DoctorReport {
        isolde_version,
        rust_version,
        diagnostics,
        error_count,
        warning_count,
        healthy_count,
    };

    // Print summary
    print_doctor_report(&report, opts.verbose);

    // Write report file if requested
    if let Some(ref report_path) = opts.report {
        write_report(&report, report_path)?;
    }

    Ok(report)
}

/// Check Isolde CLI installation
fn check_isolde_cli(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let version = env!("ISOLDE_VERSION");

    let mut suggestions = Vec::new();

    // Check if isolde command is available (we're running it, so it must be)
    let in_path = Command::new("isolde")
        .arg("--version")
        .output()
        .is_ok();

    if !in_path {
        suggestions.push("Install Isolde CLI using: cargo install isolde-cli".to_string());
        suggestions.push("Or run from the repository: cargo run --release".to_string());
    }

    DiagnosticResult {
        component: "isolde-cli".to_string(),
        status: if in_path { DiagnosticStatus::Healthy } else { DiagnosticStatus::Warning },
        message: format!("Isolde CLI v{}", version),
        suggestions,
        fixable: false,
    }
}

/// Check Docker installation
fn check_docker(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let result = Command::new("docker")
        .arg("--version")
        .output();

    let (status, message, mut suggestions) = match result {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            let v_str = version.trim().to_string();

            // Check if daemon is running
            let daemon_result = Command::new("docker")
                .args(["info", "--format", "{{.ServerVersion}}"])
                .output();

            match daemon_result {
                Ok(daemon_output) if daemon_output.status.success() => {
                    let daemon_version = String::from_utf8_lossy(&daemon_output.stdout);
                    (
                        DiagnosticStatus::Healthy,
                        format!("Docker {} (daemon {})", v_str, daemon_version.trim()),
                        vec![],
                    )
                }
                _ => (
                    DiagnosticStatus::Warning,
                    format!("Docker {} installed, daemon not running", v_str),
                    vec!["Start Docker Desktop or the Docker daemon".to_string()],
                ),
            }
        }
        Err(_) => (
            DiagnosticStatus::Error,
            "Docker not found".to_string(),
            vec![
                "Install Docker Desktop from https://www.docker.com/products/docker-desktop/".to_string(),
                "Or install Docker Engine for your platform".to_string(),
            ],
        ),
    };

    suggestions.push("Docker is required for devcontainer functionality".to_string());

    DiagnosticResult {
        component: "docker".to_string(),
        status,
        message,
        suggestions,
        fixable: false,
    }
}

/// Check Git installation
fn check_git(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let result = Command::new("git")
        .arg("--version")
        .output();

    let (status, message, suggestions) = match result {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            (
                DiagnosticStatus::Healthy,
                version.trim().to_string(),
                vec![],
            )
        }
        Err(_) => (
            DiagnosticStatus::Error,
            "Git not found".to_string(),
            vec![
                "Install Git from https://git-scm.com/downloads".to_string(),
                "Git is required for project initialization".to_string(),
            ],
        ),
    };

    DiagnosticResult {
        component: "git".to_string(),
        status,
        message,
        suggestions,
        fixable: false,
    }
}

/// Check isolde.yaml configuration
fn check_isolde_yaml(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let config_path = cwd.join("isolde.yaml");

    if !config_path.exists() {
        return DiagnosticResult {
            component: "isolde-yaml".to_string(),
            status: DiagnosticStatus::Warning,
            message: "isolde.yaml not found".to_string(),
            suggestions: vec![
                "Run 'isolde init' to create a configuration file".to_string(),
            ],
            fixable: false,
        };
    }

    match Config::from_file(&config_path) {
        Ok(config) => {
            let mut suggestions = vec![];

            // Check for deprecated settings
            if config.docker.image.contains("devcontainers/base") {
                suggestions.push("Consider using a more specific base image".to_string());
            }

            DiagnosticResult {
                component: "isolde-yaml".to_string(),
                status: if suggestions.is_empty() {
                    DiagnosticStatus::Healthy
                } else {
                    DiagnosticStatus::Warning
                },
                message: format!("Configuration valid for project '{}'", config.name),
                suggestions,
                fixable: false,
            }
        }
        Err(e) => DiagnosticResult {
            component: "isolde-yaml".to_string(),
            status: DiagnosticStatus::Error,
            message: format!("Configuration error: {}", e),
            suggestions: vec![
                "Check isolde.yaml syntax".to_string(),
                "Ensure all required fields are present".to_string(),
                "Run 'isolde validate' for detailed diagnostics".to_string(),
            ],
            fixable: false,
        },
    }
}

/// Check devcontainer configuration
fn check_devcontainer(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let devcontainer_dir = cwd.join(".devcontainer");

    if !devcontainer_dir.exists() {
        return DiagnosticResult {
            component: "devcontainer".to_string(),
            status: DiagnosticStatus::Warning,
            message: "Devcontainer directory not found".to_string(),
            suggestions: vec![
                "Run 'isolde sync' to generate devcontainer configuration".to_string(),
            ],
            fixable: true,
        };
    }

    let devcontainer_json = devcontainer_dir.join("devcontainer.json");
    let dockerfile = devcontainer_dir.join("Dockerfile");
    let features_dir = devcontainer_dir.join("features");

    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    if !devcontainer_json.exists() {
        issues.push("devcontainer.json missing".to_string());
        suggestions.push("devcontainer.json is required for VS Code devcontainers".to_string());
    } else {
        // Validate JSON
        if let Ok(content) = fs::read_to_string(&devcontainer_json) {
            if serde_json::from_str::<serde_json::Value>(&content).is_err() {
                issues.push("devcontainer.json contains invalid JSON".to_string());
                suggestions.push("Fix JSON syntax errors in devcontainer.json".to_string());
            } else if verbose {
                suggestions.push("devcontainer.json is valid".to_string());
            }
        }
    }

    if !dockerfile.exists() {
        issues.push("Dockerfile missing".to_string());
        suggestions.push("Dockerfile is required for custom build".to_string());
    }

    if !features_dir.exists() {
        issues.push("features directory missing".to_string());
        suggestions.push("Core features should be copied to features/".to_string());
    }

    let (status, message) = if issues.is_empty() {
        (
            DiagnosticStatus::Healthy,
            "Devcontainer configuration complete".to_string(),
        )
    } else {
        (
            DiagnosticStatus::Warning,
            format!("Issues found: {}", issues.join(", ")),
        )
    };

    DiagnosticResult {
        component: "devcontainer".to_string(),
        status,
        message,
        suggestions,
        fixable: true,
    }
}

/// Check core features
fn check_core_features(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let features_dir = cwd.join(".devcontainer/features");

    if !features_dir.exists() {
        return DiagnosticResult {
            component: "core-features".to_string(),
            status: DiagnosticStatus::Warning,
            message: "Core features directory not found".to_string(),
            suggestions: vec![
                "Run 'isolde sync' to copy core features".to_string(),
            ],
            fixable: true,
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

    let (status, message, suggestions) = if missing.is_empty() {
        (
            DiagnosticStatus::Healthy,
            format!("All core features present: {}", found.join(", ")),
            vec![],
        )
    } else {
        (
            DiagnosticStatus::Warning,
            format!("Missing core features: {}", missing.join(", ")),
            vec![
                format!("Run 'isolde sync' to copy missing features"),
                "Ensure core/ directory exists in Isolde installation".to_string(),
            ],
        )
    };

    DiagnosticResult {
        component: "core-features".to_string(),
        status,
        message,
        suggestions,
        fixable: true,
    }
}

/// Check Claude installation
fn check_claude(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let result = Command::new("claude")
        .arg("--version")
        .output();

    let (status, message, suggestions) = match result {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            (
                DiagnosticStatus::Healthy,
                version.trim().to_string(),
                vec![],
            )
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            (
                DiagnosticStatus::Warning,
                "Claude CLI not working".to_string(),
                vec![
                    format!("Error: {}", stderr.trim()),
                    "Claude will be installed in the devcontainer".to_string(),
                ],
            )
        }
        Err(_) => (
            DiagnosticStatus::Warning,
            "Claude CLI not installed".to_string(),
            vec![
                "Install with: npm install -g @anthropic-ai/claude-code".to_string(),
                "Or it will be installed in the devcontainer".to_string(),
            ],
        ),
    };

    DiagnosticResult {
        component: "claude".to_string(),
        status,
        message,
        suggestions,
        fixable: false,
    }
}

/// Check workspace configuration
fn check_workspace(cwd: &Path, verbose: bool) -> DiagnosticResult {
    let config_path = cwd.join("isolde.yaml");

    let config = if config_path.exists() {
        Config::from_file(&config_path).ok()
    } else {
        None
    };

    if let Some(ref cfg) = config {
        let workspace_dir = cwd.join(&cfg.workspace.dir);

        let (status, message, suggestions) = if !workspace_dir.exists() {
            (
                DiagnosticStatus::Warning,
                format!("Workspace directory '{}' does not exist", cfg.workspace.dir),
                vec![
                    format!("Create the directory: mkdir -p {}", cfg.workspace.dir),
                    "Run 'isolde sync' to set up the workspace".to_string(),
                ],
            )
        } else {
            // Check for git repo
            let git_dir = workspace_dir.join(".git");
            if !git_dir.exists() {
                (
                    DiagnosticStatus::Warning,
                    format!("Workspace '{}' exists but not a git repository", cfg.workspace.dir),
                    vec![
                        "Initialize git: git init".to_string(),
                        "Or run 'isolde sync' to initialize repositories".to_string(),
                    ],
                )
            } else {
                (
                    DiagnosticStatus::Healthy,
                    format!("Workspace '{}' configured", cfg.workspace.dir),
                    vec![],
                )
            }
        };

        DiagnosticResult {
            component: "workspace".to_string(),
            status,
            message,
            suggestions,
            fixable: true,
        }
    } else {
        DiagnosticResult {
            component: "workspace".to_string(),
            status: DiagnosticStatus::Missing,
            message: "Cannot check workspace without isolde.yaml".to_string(),
            suggestions: vec![
                "Run 'isolde init' to create configuration".to_string(),
            ],
            fixable: false,
        }
    }
}

/// Attempt to fix an issue
fn attempt_fix(result: &DiagnosticResult, cwd: &Path) -> bool {
    match result.component.as_str() {
        "devcontainer" => {
            // Try running isolde sync
            let output = Command::new("isolde")
                .args(["sync"])
                .current_dir(cwd)
                .output();

            match output {
                Ok(o) if o.status.success() => true,
                _ => false,
            }
        }
        "core-features" => {
            // Try running isolde sync
            let output = Command::new("isolde")
                .args(["sync"])
                .current_dir(cwd)
                .output();

            match output {
                Ok(o) if o.status.success() => true,
                _ => false,
            }
        }
        _ => false,
    }
}

/// Print the doctor report
fn print_doctor_report(report: &DoctorReport, verbose: bool) {
    println!("{}", "Version Information:".bold());
    println!("  Isolde CLI: {}", report.isolde_version.cyan());
    println!("  Rust: {}", report.rust_version.dimmed());
    println!();

    println!("{}", "Diagnostic Results:".bold());

    for diagnostic in &report.diagnostics {
        println!(
            "  {} {} - {}",
            diagnostic_status_icon(diagnostic.status),
            diagnostic.component.bold(),
            diagnostic.message
        );

        if !diagnostic.suggestions.is_empty() && (verbose || diagnostic.status != DiagnosticStatus::Healthy) {
            for suggestion in &diagnostic.suggestions {
                println!("    {}", suggestion.dimmed());
            }
        }
    }

    println!();
    println!("{}", "═".repeat(50).cyan());

    if report.is_healthy() {
        println!(
            "{} {}",
            "✨".green(),
            "All systems operational!".green().bold()
        );
    } else {
        let icon = if report.error_count > 0 {
            "✗".red()
        } else {
            "⚠".yellow()
        };
        println!(
            "{} {} error(s), {} warning(s)",
            icon,
            report.error_count.to_string().red().bold(),
            report.warning_count.to_string().yellow().bold()
        );
    }

    if !report.is_healthy() {
        println!();
        println!("{}", "Run with --fix to attempt automatic fixes.".yellow());
        println!("{}", "Run with --verbose for more details.".dimmed());
    }
}

/// Write report to file
fn write_report(report: &DoctorReport, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(report)
        .map_err(|e| Error::Other(format!("Failed to serialize report: {}", e)))?;

    fs::write(path, json)
        .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to write report: {}", e))))?;

    println!();
    println!(
        "{} {}",
        "📄".cyan(),
        format!("Report written to {}", path.display()).cyan()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doctor_options_default() {
        let opts = DoctorOptions::default();
        assert!(!opts.fix);
        assert!(!opts.verbose);
        assert!(opts.component.is_none());
        assert!(opts.report.is_none());
    }

    #[test]
    fn test_diagnostic_status_ord() {
        assert_eq!(DiagnosticStatus::Healthy as i32, 0);
    }

    #[test]
    fn test_doctor_report_is_healthy() {
        let report = DoctorReport {
            isolde_version: "1.0.0".to_string(),
            rust_version: "1.75".to_string(),
            diagnostics: vec![],
            error_count: 0,
            warning_count: 0,
            healthy_count: 0,
        };
        assert!(report.is_healthy());

        let report_with_warnings = DoctorReport {
            isolde_version: "1.0.0".to_string(),
            rust_version: "1.75".to_string(),
            diagnostics: vec![],
            error_count: 0,
            warning_count: 1,
            healthy_count: 0,
        };
        assert!(!report_with_warnings.is_healthy());
    }
}
