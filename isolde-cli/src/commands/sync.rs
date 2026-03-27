//! # Isolde sync command
//!
//! Generate devcontainer and Claude configuration from isolde.yaml.

use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use isolde_core::config::{Config, IsolationLevel};
use isolde_core::devcontainer::{self, HostAuthInfo};
use isolde_core::{Error, Result};

/// Options for the sync command
#[derive(Debug, Clone)]
pub struct SyncOptions {
    /// Dry run - don't write files
    pub dry_run: bool,

    /// Force regeneration even if files exist
    pub force: bool,

    /// Current working directory
    pub cwd: PathBuf,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            dry_run: false,
            force: false,
            cwd: PathBuf::from("."),
        }
    }
}

/// Run the sync command
pub fn run(opts: SyncOptions) -> Result<()> {
    let config_path = opts.cwd.join("isolde.yaml");

    // Check if isolde.yaml exists
    if !config_path.exists() {
        return Err(Error::InvalidTemplate(
            "isolde.yaml not found. Run 'isolde init' first.".to_string(),
        ));
    }

    println!("{}", "🔄 Syncing Isolde configuration...".cyan());
    println!("{}", "─".repeat(50).dimmed());

    // Load and validate configuration
    print!("{} ", "Loading configuration...".dimmed());
    let config = Config::from_file(&config_path)?;
    println!("{}", "✔".green());

    // Guard: warn about unimplemented agents
    let agent = config.agent_name();
    if !super::is_agent_implemented(agent) {
        eprintln!(
            "{} {}",
            "⚠".yellow(),
            format!(
                "Agent '{}' is not yet implemented — the generated devcontainer will not install the agent CLI.",
                agent
            ).yellow()
        );
    }

    // Create output directories
    let devcontainer_dir = opts.cwd.join(".devcontainer");
    let claude_dir = opts.cwd.join(".claude");
    let features_dir = devcontainer_dir.join("features");

    let project_dir = opts.cwd.join(config.workspace_dir());

    if !opts.dry_run {
        fs::create_dir_all(&devcontainer_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        fs::create_dir_all(&claude_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        fs::create_dir_all(&features_dir)
            .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        if !project_dir.exists() {
            fs::create_dir_all(&project_dir)
                .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        }

        // Create volume directories for isolation
        isolde_core::volumes::ensure_volumes(&opts.cwd, &config)?;
    }

    // Only probe host auth files when full isolation needs them
    let host_auth = if config.isolation() == IsolationLevel::Full {
        HostAuthInfo::detect()
    } else {
        HostAuthInfo::none()
    };

    // Warn if full isolation and no auth files found
    if config.isolation() == IsolationLevel::Full && !host_auth.credentials_exist {
        eprintln!(
            "{} {}",
            "⚠".yellow(),
            "No auth credentials found at ~/.claude/.credentials.json".yellow()
        );
        eprintln!(
            "{}",
            "  Run 'claude login' and then 'isolde sync' again to mount auth into the container."
                .dimmed()
        );
    }

    // Generate devcontainer.json
    print!("{} ", "Generating devcontainer.json...".dimmed());
    let devcontainer_json = devcontainer::render_devcontainer_json(&config, &host_auth)?;
    if !opts.dry_run {
        let output_path = devcontainer_dir.join("devcontainer.json");
        write_file(&output_path, &devcontainer_json, opts.force)?;
    }
    println!("{}", "✔".green());

    // Generate Dockerfile
    print!("{} ", "Generating Dockerfile...".dimmed());
    let dockerfile = devcontainer::render_dockerfile(&config)?;
    if !opts.dry_run {
        let output_path = devcontainer_dir.join("Dockerfile");
        write_file(&output_path, &dockerfile, opts.force)?;
    }
    println!("{}", "✔".green());

    // Generate CLAUDE.md
    print!("{} ", "Generating CLAUDE.md...".dimmed());
    let claude_md = devcontainer::render_claude_md(&config)?;
    if !opts.dry_run {
        let output_path = claude_dir.join("CLAUDE.md");
        write_file(&output_path, &claude_md, opts.force)?;
    }
    println!("{}", "✔".green());

    // Copy core features
    print!("{} ", "Copying core features...".dimmed());
    if !opts.dry_run {
        match devcontainer::copy_core_features(&features_dir) {
            Ok(()) => {}
            Err(_) => {
                println!(
                    "{}",
                    "⚠ Core features not found - skipping feature copy".yellow()
                );
                println!(
                    "{}",
                    "  Features should be manually installed or bundled with the binary.".dimmed()
                );
            }
        }
    }
    println!("{}", "✔".green());

    println!("{}", "─".repeat(50).dimmed());
    println!("\n{} {}", "✨".green(), "Sync complete!".green().bold());
    println!(
        "{}",
        "Run 'docker build -t <image-name> .devcontainer' to build the devcontainer.".dimmed()
    );

    Ok(())
}

/// Write file to disk, handling force option
fn write_file(path: &Path, content: &str, force: bool) -> Result<()> {
    if path.exists() && !force {
        println!(
            "  Skipped {} (already exists)",
            path.display().to_string().yellow()
        );
        return Ok(());
    }

    fs::write(path, content).map_err(|e| {
        Error::FileError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to write {}: {}", path.display(), e),
        ))
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_options_default() {
        let opts = SyncOptions::default();
        assert!(!opts.dry_run);
        assert!(!opts.force);
    }
}
