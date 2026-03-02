//! # Isolde build command
//!
//! Build the devcontainer image for a project.

use std::path::PathBuf;

use colored::Colorize;
use isolde_core::container;
use isolde_core::{Error, Result};

/// Options for the build command
#[derive(Debug, Clone)]
pub struct BuildOptions {
    /// Path to project directory (default: current directory)
    pub workspace_folder: Option<PathBuf>,

    /// Don't use cache when building
    pub no_cache: bool,

    /// Image name and tag
    pub image_name: Option<String>,

    /// Current working directory
    pub cwd: PathBuf,

    /// Enable verbose output
    pub verbose: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            workspace_folder: None,
            no_cache: false,
            image_name: None,
            cwd: PathBuf::from("."),
            verbose: false,
        }
    }
}

/// Run the build command
pub fn run(opts: BuildOptions) -> Result<()> {
    let workspace = opts.workspace_folder.unwrap_or_else(|| opts.cwd.clone());

    println!("{}", "🔨 Building devcontainer image...".cyan());
    println!("{}", "─".repeat(50).dimmed());

    // Check if .devcontainer exists
    if !workspace.join(".devcontainer").exists() {
        return Err(Error::Other(
            ".devcontainer directory not found. Run 'isolde sync' first.".to_string(),
        ));
    }

    if opts.verbose {
        println!("{}", format!("Workspace: {}", workspace.display()).dimmed());
        if opts.no_cache {
            println!("{}", "No cache: enabled".dimmed());
        }
        if let Some(ref name) = opts.image_name {
            println!("{}", format!("Image name: {}", name).dimmed());
        }
        println!();
    }

    // Run the build
    let result = container::build(&workspace, opts.no_cache, opts.image_name)
        .map_err(|e| Error::Other(format!("Build failed: {}", e)))?;

    println!("{}", "─".repeat(50).dimmed());

    if result.success {
        println!(
            "\n{} {}",
            "✨".green(),
            "Build complete!".green().bold()
        );
        println!(
            "{} {}",
            "Image:".dimmed(),
            result.image_name.cyan()
        );
        println!(
            "{}",
            "Run 'isolde run' to start the container.".dimmed()
        );
    } else {
        println!(
            "\n{} {}",
            "✗".red(),
            "Build failed!".red().bold()
        );
        if !result.output.is_empty() {
            println!("{}", "\nBuild output:".dimmed());
            println!("{}", result.output);
        }
        return Err(Error::Other("Build failed".to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_options_default() {
        let opts = BuildOptions::default();
        assert!(!opts.no_cache);
        assert!(opts.image_name.is_none());
    }
}
