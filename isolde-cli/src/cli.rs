//! # CLI command definitions for Isolde
//!
//! This module defines all command-line interface commands using clap's derive API.

use clap::{Parser, Subcommand};
use colored::Colorize;

/// Isolde - ISOLated Development Environment template system
#[derive(Parser, Debug)]
#[command(name = "isolde")]
#[command(author = "Isolde Contributors")]
#[command(version = env!("ISOLDE_VERSION"))]
#[command(about = "Create isolated development environments from templates", long_about = None)]
#[command(after_help = "For more help, visit: https://github.com/dmanakulikov/isolde")]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI commands
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Initialize a new Isolde project from a template or preset
    ///
    /// Creates a new project directory with the chosen template's devcontainer configuration.
    /// The project will have two separate git repositories: one for the user code and one
    /// for the devcontainer configuration.
    ///
    /// # Examples
    ///
    /// Create from preset:
    ///   isolde init --preset python-ml
    ///
    /// Create from template:
    ///   isolde init my-project --template nodejs --lang-version=22
    ///
    /// Interactive mode:
    ///   isolde init
    Init {
        /// Project name (defaults to current directory if not specified)
        #[arg(value_name = "NAME")]
        name: Option<String>,

        /// Template to use (e.g., python, nodejs, rust)
        #[arg(short, long, value_name = "TEMPLATE")]
        template: Option<String>,

        /// Preset to use (e.g., python-ml, node-api, rust-cli)
        #[arg(short = 'P', long, value_name = "PRESET")]
        preset: Option<String>,

        /// Language version (e.g., "3.12" for Python, "22" for Node.js)
        #[arg(long, value_name = "VERSION")]
        lang_version: Option<String>,

        /// Claude Code version to install
        #[arg(long, value_name = "VERSION", default_value = "latest")]
        claude_version: String,

        /// Claude provider (anthropic, openai, azure)
        #[arg(long, value_name = "PROVIDER")]
        claude_provider: Option<String>,

        /// HTTP proxy URL for enterprise environments
        #[arg(long, value_name = "URL")]
        http_proxy: Option<String>,

        /// HTTPS proxy URL for enterprise environments
        #[arg(long, value_name = "URL")]
        https_proxy: Option<String>,

        /// Target directory (defaults to current directory)
        #[arg(long, value_name = "PATH")]
        target_dir: Option<String>,

        /// List available templates and exit
        #[arg(long, conflicts_with = "preset")]
        list_templates: bool,

        /// List available presets and exit
        #[arg(long, conflicts_with = "template")]
        list_presets: bool,
    },

    /// Synchronize project with the latest template changes
    ///
    /// Updates the project's devcontainer configuration to match the latest version
    /// of the template. This is useful when templates are updated with new features
    /// or bug fixes.
    ///
    /// # Examples
    ///
    /// Preview changes:
    ///   isolde sync --dry-run
    ///
    /// Force update:
    ///   isolde sync --force
    Sync {
        /// Preview changes without applying them
        #[arg(long)]
        dry_run: bool,

        /// Apply changes even if there are conflicts
        #[arg(long)]
        force: bool,

        /// Specific template version to sync to
        #[arg(long, value_name = "VERSION")]
        version: Option<String>,
    },

    /// Pull a template or preset from a remote repository
    ///
    /// Downloads templates or presets from GitHub repositories, allowing you to
    /// use community-maintained configurations.
    ///
    /// # Examples
    ///
    /// Pull from GitHub:
    ///   isolde pull my-template --repo username/templates
    ///
    /// Pull specific branch:
    ///   isolde pull custom-preset --repo username/isolde-presets --ref develop
    Pull {
        /// Name of the template or preset to pull
        #[arg(value_name = "NAME")]
        name: String,

        /// Repository in OWNER/REPO format
        #[arg(long, value_name = "OWNER/REPO")]
        repo: String,

        /// Git reference (branch, tag, or commit)
        #[arg(long, value_name = "REF")]
        r#ref: Option<String>,

        /// Destination directory
        #[arg(short, long, value_name = "PATH")]
        output: Option<String>,

        /// Verify checksum after download
        #[arg(long)]
        verify: bool,
    },

    /// Validate project configuration
    ///
    /// Checks that the project's devcontainer configuration is valid and follows
    /// best practices.
    ///
    /// # Examples
    ///
    /// Quick validation:
    ///   isolde validate --quick
    ///
    /// Verbose output:
    ///   isolde validate --verbose
    Validate {
        /// Quick validation (skip expensive checks)
        #[arg(short, long)]
        quick: bool,

        /// Show detailed validation results
        #[arg(short, long)]
        verbose: bool,

        /// Fail on warnings (not just errors)
        #[arg(long)]
        warnings_as_errors: bool,

        /// Output format (text, json, sarif)
        #[arg(long, value_name = "FORMAT", default_value = "text")]
        format: String,

        /// Check specific files or directories
        #[arg(short, long, value_name = "PATH")]
        path: Vec<String>,
    },

    /// Show differences between project and template
    ///
    /// Compares the current project's devcontainer configuration with the
    /// original template to see what has changed.
    ///
    /// # Examples
    ///
    /// Show all differences:
    ///   isolde diff
    ///
    /// Show specific file:
    ///   isolde diff --file devcontainer.json
    Diff {
        /// Show differences for specific file only
        #[arg(long, value_name = "FILE")]
        file: Option<String>,

        /// Output format (diff, json, color)
        #[arg(long, value_name = "FORMAT", default_value = "color")]
        format: String,

        /// Number of context lines
        #[arg(short, long, value_name = "LINES", default_value = "3")]
        context: usize,
    },

    /// Run diagnostics on the Isolde installation
    ///
    /// Checks that Isolde is properly installed and configured, and provides
    /// troubleshooting information.
    ///
    /// # Examples
    ///
    /// Run diagnostics:
    ///   isolde doctor
    ///
    /// Fix issues automatically:
    ///   isolde doctor --fix
    Doctor {
        /// Attempt to fix issues automatically
        #[arg(long)]
        fix: bool,

        /// Show detailed diagnostic information
        #[arg(short, long)]
        verbose: bool,

        /// Check specific component only
        #[arg(long, value_name = "COMPONENT")]
        component: Option<String>,

        /// Generate diagnostic report
        #[arg(long, value_name = "FILE")]
        report: Option<String>,
    },

    /// Show version information
    ///
    /// Displays the version of Isolde and its dependencies.
    ///
    /// # Examples
    ///
    /// Show version:
    ///   isolde version
    ///
    /// Show detailed version info:
    ///   isolde version --verbose
    Version {
        /// Show detailed version information
        #[arg(short, long)]
        verbose: bool,

        /// Output format (text, json)
        #[arg(long, value_name = "FORMAT", default_value = "text")]
        format: String,
    },
}

impl Commands {
    /// Returns a colored string representation of the command
    pub fn display(&self) -> String {
        match self {
            Commands::Init { .. } => "init".cyan().to_string(),
            Commands::Sync { .. } => "sync".cyan().to_string(),
            Commands::Pull { .. } => "pull".cyan().to_string(),
            Commands::Validate { .. } => "validate".cyan().to_string(),
            Commands::Diff { .. } => "diff".cyan().to_string(),
            Commands::Doctor { .. } => "doctor".cyan().to_string(),
            Commands::Version { .. } => "version".cyan().to_string(),
        }
    }

    /// Returns the command name as a static string
    pub fn name(&self) -> &'static str {
        match self {
            Commands::Init { .. } => "init",
            Commands::Sync { .. } => "sync",
            Commands::Pull { .. } => "pull",
            Commands::Validate { .. } => "validate",
            Commands::Diff { .. } => "diff",
            Commands::Doctor { .. } => "doctor",
            Commands::Version { .. } => "version",
        }
    }
}

/// Helper function to print a success message with color
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Helper function to print an error message with color
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message);
}

/// Helper function to print a warning message with color
pub fn print_warning(message: &str) {
    eprintln!("{} {}", "⚠".yellow(), message);
}

/// Helper function to print an info message with color
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::try_parse_from(["isolde", "--verbose"]).unwrap();
        assert!(cli.verbose);
    }

    #[test]
    fn test_init_command() {
        let cli = Cli::try_parse_from([
            "isolde",
            "init",
            "my-project",
            "--template",
            "python",
        ])
        .unwrap();
        assert!(cli.command.is_some());
    }

    #[test]
    fn test_sync_command() {
        let cli = Cli::try_parse_from(["isolde", "sync", "--dry-run"]).unwrap();
        assert!(cli.command.is_some());
    }

    #[test]
    fn test_pull_command() {
        let cli = Cli::try_parse_from([
            "isolde",
            "pull",
            "my-template",
            "--repo",
            "owner/repo",
        ])
        .unwrap();
        assert!(cli.command.is_some());
    }

    #[test]
    fn test_validate_command() {
        let cli = Cli::try_parse_from(["isolde", "validate", "--quick"]).unwrap();
        assert!(cli.command.is_some());
    }

    #[test]
    fn test_diff_command() {
        let cli = Cli::try_parse_from(["isolde", "diff"]).unwrap();
        assert!(cli.command.is_some());
    }

    #[test]
    fn test_doctor_command() {
        let cli = Cli::try_parse_from(["isolde", "doctor"]).unwrap();
        assert!(cli.command.is_some());
    }

    #[test]
    fn test_version_command() {
        let cli = Cli::try_parse_from(["isolde", "version"]).unwrap();
        assert!(cli.command.is_some());
    }

    #[test]
    fn test_command_display() {
        let commands = vec![
            Commands::Init {
                name: Some("test".to_string()),
                template: None,
                preset: None,
                lang_version: None,
                claude_version: "latest".to_string(),
                claude_provider: None,
                http_proxy: None,
                https_proxy: None,
                target_dir: None,
                list_templates: false,
                list_presets: false,
            },
            Commands::Sync {
                dry_run: false,
                force: false,
                version: None,
            },
            Commands::Pull {
                name: "test".to_string(),
                repo: "owner/repo".to_string(),
                r#ref: None,
                output: None,
                verify: false,
            },
            Commands::Validate {
                quick: false,
                verbose: false,
                warnings_as_errors: false,
                format: "text".to_string(),
                path: vec![],
            },
            Commands::Diff {
                file: None,
                format: "color".to_string(),
                context: 3,
            },
            Commands::Doctor {
                fix: false,
                verbose: false,
                component: None,
                report: None,
            },
            Commands::Version {
                verbose: false,
                format: "text".to_string(),
            },
        ];

        for cmd in commands {
            assert!(!cmd.name().is_empty());
            assert!(!cmd.display().is_empty());
        }
    }
}
