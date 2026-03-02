//! # Isolde ps command
//!
//! List running devcontainers.

use colored::Colorize;
use isolde_core::container;
use isolde_core::{Error, Result};

/// Options for the ps command
#[derive(Debug, Clone)]
pub struct PsOptions {
    /// Show all containers (including stopped)
    pub all: bool,

    /// Enable verbose output
    pub verbose: bool,
}

impl Default for PsOptions {
    fn default() -> Self {
        Self {
            all: false,
            verbose: false,
        }
    }
}

/// Container display information
#[derive(Debug, Clone)]
pub struct ContainerDisplay {
    /// Container ID (shortened)
    pub id: String,
    /// Container name
    pub name: String,
    /// Container status
    pub status: String,
    /// Workspace folder
    pub workspace: String,
}

/// Run the ps command
pub fn run(opts: PsOptions) -> Result<Vec<ContainerDisplay>> {
    println!("{}", "📋 Devcontainers".cyan());
    println!("{}", "─".repeat(50).dimmed());

    let containers = container::ps()
        .map_err(|e| Error::Other(format!("Failed to list containers: {}", e)))?;

    if containers.is_empty() {
        println!("{}", "No running containers found.".dimmed());
        return Ok(vec![]);
    }

    // Filter by status if not --all
    let filtered: Vec<_> = if opts.all {
        containers.iter().collect()
    } else {
        containers.iter()
            .filter(|c| c.status == "running")
            .collect()
    };

    if filtered.is_empty() {
        println!("{}", "No running containers found.".dimmed());
        if opts.all {
            println!("{}", "Use --all to see stopped containers.".dimmed());
        }
        return Ok(vec![]);
    }

    // Display header
    println!(
        "{:<12}  {:<25}  {:<10}  {}",
        "ID".bold(),
        "NAME".bold(),
        "STATUS".bold(),
        "WORKSPACE".bold()
    );
    println!("{}", "─".repeat(80).dimmed());

    // Display each container
    let display_containers: Vec<ContainerDisplay> = filtered.iter().map(|c| {
        let id = if c.container_id.len() > 12 {
            format!("{}...", &c.container_id[..12])
        } else {
            c.container_id.clone()
        };

        let status_colored = match c.status.as_str() {
            "running" => c.status.green(),
            "exited" => c.status.dimmed(),
            "stopped" => c.status.dimmed(),
            _ => c.status.normal(),
        };

        println!(
            "{:<12}  {:<25}  {:<10}  {}",
            id.cyan(),
            c.container_name,
            status_colored,
            c.workspace_folder.dimmed()
        );

        ContainerDisplay {
            id,
            name: c.container_name.clone(),
            status: c.status.clone(),
            workspace: c.workspace_folder.clone(),
        }
    }).collect();

    println!();
    println!(
        "{} {}",
        filtered.len().to_string().cyan().bold(),
        if filtered.len() == 1 {
            "container".to_string()
        } else {
            "containers".to_string()
        }
    );

    Ok(display_containers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ps_options_default() {
        let opts = PsOptions::default();
        assert!(!opts.all);
    }
}
