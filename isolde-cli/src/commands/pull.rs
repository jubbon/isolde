//! # Isolde pull command
//!
//! Fetch configuration from a GitHub repository and create isolde.yaml.
//!
//! NOTE: This command is disabled in builds without async/network support.

use std::path::{Path, PathBuf};

use colored::Colorize;
use isolde_core::{Error, Result};

/// Default GitHub repository for Isolde templates
const DEFAULT_ISOLDE_REPO: &str = "https://github.com/jubbon/isolde";

/// Options for the pull command
#[derive(Debug, Clone)]
pub struct PullOptions {
    /// Name of the template/preset to pull
    pub name: String,

    /// Repository URL (optional, defaults to main Isolde repo)
    pub repo: Option<String>,

    /// Git reference (branch, tag, or commit)
    pub r#ref: Option<String>,

    /// Run sync after pulling
    pub sync: bool,

    /// Current working directory
    pub cwd: PathBuf,
}

impl PullOptions {
    /// Create a new PullOptions instance
    pub fn new(name: String) -> Self {
        Self {
            name,
            repo: None,
            r#ref: None,
            sync: false,
            cwd: PathBuf::from("."),
        }
    }

    /// Set the repository URL
    pub fn repo(mut self, repo: String) -> Self {
        self.repo = Some(repo);
        self
    }

    /// Set the git reference
    pub fn r#ref(mut self, r#ref: String) -> Self {
        self.r#ref = Some(r#ref);
        self
    }

    /// Set whether to run sync after pulling
    pub fn sync(mut self, sync: bool) -> Self {
        self.sync = sync;
        self
    }

    /// Set the current working directory
    pub fn cwd(mut self, cwd: PathBuf) -> Self {
        self.cwd = cwd;
        self
    }
}

/// Run the pull command
///
/// Note: This command is disabled in builds without async/network support.
pub fn run(opts: PullOptions) -> Result<()> {
    let _ = opts; // Suppress unused warning

    eprintln!("{}", "❌ Pull command is disabled".red());
    eprintln!("The pull command requires async/network support which is disabled in this build.");
    eprintln!("");
    eprintln!("To pull a configuration manually:");
    eprintln!("1. Visit the repository: {}", DEFAULT_ISOLDE_REPO.dimmed());
    eprintln!("2. Browse to the desired template/preset");
    eprintln!("3. Download the isolde.yaml file");
    eprintln!("4. Run: isolde sync");
    eprintln!("");

    Err(Error::Other("Pull command disabled".to_string()))
}

/// List available presets from a repository
///
/// Note: This function is disabled without async/network support.
pub fn list_presets(repo: Option<String>, r#ref: Option<String>) -> Result<Vec<String>> {
    let _ = (repo, r#ref);
    eprintln!("{}", "⚠ Warning: list-presets requires async/network support".yellow());
    Ok(vec![])
}

/// List available templates from a repository
///
/// Note: This function is disabled without async/network support.
pub fn list_templates(repo: Option<String>, r#ref: Option<String>) -> Result<Vec<String>> {
    let _ = (repo, r#ref);
    eprintln!("{}", "⚠ Warning: list-templates requires async/network support".yellow());
    Ok(vec![])
}
