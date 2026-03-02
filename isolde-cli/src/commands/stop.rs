//! # Isolde stop command
//!
//! Stop a running devcontainer.

use std::path::PathBuf;

use colored::Colorize;
use isolde_core::container;
use isolde_core::state::{ContainerState, ContainerStatus};
use isolde_core::{Error, Result};

/// Options for the stop command
#[derive(Debug, Clone)]
pub struct StopOptions {
    /// Path to project directory
    pub workspace_folder: Option<PathBuf>,

    /// Force stop without graceful shutdown
    pub force: bool,

    /// Current working directory
    pub cwd: PathBuf,

    /// Enable verbose output
    pub verbose: bool,
}

impl Default for StopOptions {
    fn default() -> Self {
        Self {
            workspace_folder: None,
            force: false,
            cwd: PathBuf::from("."),
            verbose: false,
        }
    }
}

/// Run the stop command
pub fn run(opts: StopOptions) -> Result<()> {
    let workspace = opts.workspace_folder.unwrap_or_else(|| opts.cwd.clone());

    // Check if .devcontainer exists
    if !workspace.join(".devcontainer").exists() {
        return Err(Error::Other(
            ".devcontainer directory not found. Run 'isolde sync' first.".to_string(),
        ));
    }

    if opts.verbose {
        println!("{}", format!("Workspace: {}", workspace.display()).dimmed());
        if opts.force {
            println!("{}", "Force stop: enabled".dimmed());
        }
        println!();
    }

    println!("{}", "🛑 Stopping devcontainer...".cyan());
    println!("{}", "─".repeat(50).dimmed());

    // Stop the container
    container::stop(&workspace)
        .map_err(|e| Error::Other(format!("Failed to stop container: {}", e)))?;

    // Update state
    if ContainerState::exists(&workspace) {
        if let Ok(mut state) = ContainerState::load(&workspace) {
            state = state.with_status(ContainerStatus::Stopped);
            let _ = state.save(&workspace);
        }
    }

    println!("{}", "─".repeat(50).dimmed());
    println!(
        "\n{} {}",
        "✨".green(),
        "Container stopped!".green().bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_options_default() {
        let opts = StopOptions::default();
        assert!(!opts.force);
        assert!(opts.workspace_folder.is_none());
    }
}
