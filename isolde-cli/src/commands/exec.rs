//! # Isolde exec command
//!
//! Execute a command in a running devcontainer.

use std::path::PathBuf;

use colored::Colorize;
use isolde_core::container;
use isolde_core::{Error, Result};

/// Options for the exec command
#[derive(Debug, Clone)]
pub struct ExecOptions {
    /// Command to execute
    pub command: Vec<String>,

    /// Path to project directory
    pub workspace_folder: Option<PathBuf>,

    /// Current working directory
    pub cwd: PathBuf,

    /// Enable verbose output
    pub verbose: bool,
}

impl Default for ExecOptions {
    fn default() -> Self {
        Self {
            command: vec![],
            workspace_folder: None,
            cwd: PathBuf::from("."),
            verbose: false,
        }
    }
}

/// Run the exec command
pub fn run(opts: ExecOptions) -> Result<()> {
    let workspace = opts.workspace_folder.unwrap_or_else(|| opts.cwd.clone());

    if opts.command.is_empty() {
        return Err(Error::Other("Command cannot be empty".to_string()));
    }

    // Check if .devcontainer exists
    if !workspace.join(".devcontainer").exists() {
        return Err(Error::Other(
            ".devcontainer directory not found. Run 'isolde sync' first.".to_string(),
        ));
    }

    if opts.verbose {
        println!("{}", format!("Workspace: {}", workspace.display()).dimmed());
        println!("{}", format!("Command: {}", opts.command.join(" ")).dimmed());
        println!();
    }

    // Execute command (interactive by default)
    let exit_status = match container::exec(&workspace, &opts.command, true) {
        Ok(status) => status,
        Err(e) => {
            println!("Error: Failed to execute command in container: {}", e);
            println!("Make sure the container is running. Start with 'isolde run'.");
            std::process::exit(1);
        }
    };

    if !exit_status.success() {
        let code = exit_status.code().unwrap_or(-1);
        println!("Error: Container command exited with code {}. Make sure the container is running.", code);
        std::process::exit(code);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_options_default() {
        let opts = ExecOptions::default();
        assert!(opts.command.is_empty());
    }
}
