//! # Isolde logs command
//!
//! View devcontainer logs.

use std::path::PathBuf;

use colored::Colorize;
use isolde_core::container;
use isolde_core::{Error, Result};

/// Options for the logs command
#[derive(Debug, Clone)]
pub struct LogsOptions {
    /// Path to project directory
    pub workspace_folder: Option<PathBuf>,

    /// Follow log output
    pub follow: bool,

    /// Number of lines to show
    pub tail: usize,

    /// Current working directory
    pub cwd: PathBuf,

    /// Enable verbose output
    pub verbose: bool,
}

impl Default for LogsOptions {
    fn default() -> Self {
        Self {
            workspace_folder: None,
            follow: false,
            tail: 100,
            cwd: PathBuf::from("."),
            verbose: false,
        }
    }
}

/// Run the logs command
pub fn run(opts: LogsOptions) -> Result<()> {
    let workspace = opts.workspace_folder.unwrap_or_else(|| opts.cwd.clone());

    // Check if .devcontainer exists
    if !workspace.join(".devcontainer").exists() {
        return Err(Error::Other(
            ".devcontainer directory not found. Run 'isolde sync' first.".to_string(),
        ));
    }

    if opts.verbose {
        println!("{}", format!("Workspace: {}", workspace.display()).dimmed());
        println!("{}", format!("Tail: {} lines", opts.tail).dimmed());
        if opts.follow {
            println!("{}", "Follow mode: enabled".dimmed());
        }
        println!();
    }

    println!("{}", "📋 Container Logs".cyan());
    println!("{}", "─".repeat(50).dimmed());
    println!();

    // Get logs
    let logs = container::logs(&workspace, opts.follow, opts.tail)
        .map_err(|e| Error::Other(format!("Failed to get logs: {}", e)))?;

    if !logs.is_empty() {
        print!("{}", logs);
    }

    // If following, the function already outputs to stdout
    if opts.follow {
        println!();
        println!("{}", "─".repeat(50).dimmed());
        println!("{}", "Press Ctrl+C to stop following".dimmed());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logs_options_default() {
        let opts = LogsOptions::default();
        assert!(!opts.follow);
        assert_eq!(opts.tail, 100);
    }
}
