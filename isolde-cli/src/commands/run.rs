//! # Isolde run command
//!
//! Run the devcontainer (start and enter shell).

use std::path::PathBuf;

use colored::Colorize;
use isolde_core::container;
use isolde_core::state::{ContainerState, ContainerStatus};
use isolde_core::{Error, Result};

/// Options for the run command
#[derive(Debug, Clone)]
pub struct RunOptions {
    /// Path to project directory
    pub workspace_folder: Option<PathBuf>,

    /// Don't attach to container (start only)
    pub detach: bool,

    /// Current working directory
    pub cwd: PathBuf,

    /// Enable verbose output
    pub verbose: bool,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            workspace_folder: None,
            detach: false,
            cwd: PathBuf::from("."),
            verbose: false,
        }
    }
}

/// Run the run command
pub fn run(opts: RunOptions) -> Result<()> {
    let workspace = opts.workspace_folder.unwrap_or_else(|| opts.cwd.clone());

    // Check if workspace folder exists (if explicitly specified)
    if !workspace.exists() {
        println!("Error: Workspace folder not found: {}", workspace.display());
        println!("Please provide a valid workspace folder path.");
        std::process::exit(1);
    }

    // Check if .devcontainer exists
    if !workspace.join(".devcontainer").exists() {
        return Err(Error::Other(
            ".devcontainer directory not found. Run 'isolde sync' first.".to_string(),
        ));
    }

    if opts.verbose {
        println!("{}", format!("Workspace: {}", workspace.display()).dimmed());
        if opts.detach {
            println!("{}", "Detach mode: enabled".dimmed());
        }
        println!();
    }

    println!("{}", "🚀 Starting devcontainer...".cyan());
    println!("{}", "─".repeat(50).dimmed());

    // Start the container
    let container_info = container::up(&workspace, opts.detach)
        .map_err(|e| Error::Other(format!("Failed to start container: {}", e)))?;

    // Update state
    if let Ok(mut state) = ContainerState::load(&workspace) {
        state = state
            .with_container_id(container_info.container_id.clone())
            .with_status(ContainerStatus::Running);
        let _ = state.save(&workspace);
    }

    println!("{}", "─".repeat(50).dimmed());

    if opts.detach {
        println!(
            "\n{} {}",
            "✨".green(),
            "Container started!".green().bold()
        );
        println!(
            "{} {}",
            "Container:".dimmed(),
            container_info.container_name.cyan()
        );
        println!(
            "{} {}",
            "ID:".dimmed(),
            container_info.container_id[..12].cyan()
        );
        println!(
            "{}",
            "Run 'isolde exec' to execute commands in the container.".dimmed()
        );
    } else {
        // If we were attached, the shell is now running
        // State was updated before entering the shell
        println!(
            "\n{} {}",
            "✨".green(),
            "Started shell in container".green().bold()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_options_default() {
        let opts = RunOptions::default();
        assert!(!opts.detach);
        assert!(opts.workspace_folder.is_none());
    }
}
