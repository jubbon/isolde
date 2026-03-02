//! # Devcontainer integration module
//!
//! This module provides integration with the devcontainers CLI tool
//! for building and managing development containers.

use crate::{Error, Result};
use command_group::CommandGroup;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use which::which;

/// Check if devcontainer CLI is installed
pub fn check_devcontainer_cli() -> Result<()> {
    which("devcontainer")
        .map_err(|_| Error::Other(
            "devcontainer CLI not found. Please install it from: https://github.com/devcontainers/cli".to_string()
        ))?;
    Ok(())
}

/// Build result from devcontainer build
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Image name that was built
    pub image_name: String,
    /// Build exit status
    pub success: bool,
    /// Build output
    pub output: String,
}

/// Container information
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    /// Container ID
    pub container_id: String,
    /// Container name
    pub container_name: String,
    /// Container status (running, exited, etc.)
    pub status: String,
    /// Workspace folder
    pub workspace_folder: String,
}

/// Build devcontainer image
///
/// # Arguments
///
/// * `workspace` - Path to the workspace folder (where .devcontainer is located)
/// * `no_cache` - Whether to skip cache during build
/// * `image_name` - Optional custom image name
///
/// # Examples
///
/// ```no_run
/// use isolde_core::container;
/// # use std::path::Path;
///
/// let result = container::build(Path::new("."), false, None)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn build(workspace: &Path, no_cache: bool, image_name: Option<String>) -> Result<BuildResult> {
    check_devcontainer_cli()?;

    if !workspace.join(".devcontainer").exists() {
        return Err(Error::Other(
            ".devcontainer directory not found. Run 'isolde sync' first.".to_string()
        ));
    }

    let mut cmd = Command::new("devcontainer");
    cmd.arg("build")
        .arg("--workspace-folder")
        .arg(workspace);

    if no_cache {
        cmd.arg("--no-cache");
    }

    if let Some(ref name) = image_name {
        cmd.arg("--image-name").arg(name);
    }

    // Capture output
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let output = cmd.spawn()
        .and_then(|mut child| child.wait_with_output())
        .map_err(|e| Error::Other(format!("Failed to run devcontainer build: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined_output = format!("{}\n{}", stdout, stderr);

    // Try to extract image name from output
    let image_name = image_name.unwrap_or_else(|| {
        extract_image_name(&combined_output).unwrap_or_else(|| "dev-container:latest".to_string())
    });

    Ok(BuildResult {
        image_name,
        success: output.status.success(),
        output: combined_output,
    })
}

/// Start devcontainer (up)
///
/// # Arguments
///
/// * `workspace` - Path to the workspace folder
/// * `detach` - If true, start container without attaching
pub fn up(workspace: &Path, detach: bool) -> Result<ContainerInfo> {
    check_devcontainer_cli()?;

    if !workspace.join(".devcontainer").exists() {
        return Err(Error::Other(
            ".devcontainer directory not found. Run 'isolde sync' first.".to_string()
        ));
    }

    let mut cmd = Command::new("devcontainer");
    cmd.arg("up")
        .arg("--workspace-folder")
        .arg(workspace);

    if detach {
        cmd.arg("--detach");
    }

    // For non-detach mode, we want the terminal to be interactive
    if !detach {
        // Don't capture output - let it go to the terminal
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        cmd.group_spawn()
            .map_err(|e| Error::Other(format!("Failed to spawn devcontainer up: {}", e)))?;
    } else {
        // For detach mode, spawn and wait for completion
        cmd.spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| Error::Other(format!("Failed to run devcontainer up: {}", e)))?;
    }

    // Get container info by parsing devcontainer output
    let container_info = get_container_info(workspace)?;

    Ok(container_info)
}

/// Execute command in running container
///
/// # Arguments
///
/// * `workspace` - Path to the workspace folder
/// * `command` - Command and arguments to execute
/// * `interactive` - If true, run in interactive mode
pub fn exec(workspace: &Path, command: &[String], interactive: bool) -> Result<ExitStatus> {
    check_devcontainer_cli()?;

    if command.is_empty() {
        return Err(Error::Other("Command cannot be empty".to_string()));
    }

    let mut cmd = Command::new("devcontainer");
    cmd.arg("exec")
        .arg("--workspace-folder")
        .arg(workspace);

    if interactive {
        cmd.arg("--tty");
    }

    cmd.arg("--").args(command);

    if interactive {
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        cmd.group_spawn();
    }

    cmd.status()
        .map_err(|e| Error::Other(format!("Failed to run devcontainer exec: {}", e)))
}

/// Stop running container
///
/// # Arguments
///
/// * `workspace` - Path to the workspace folder
pub fn stop(workspace: &Path) -> Result<()> {
    check_devcontainer_cli()?;

    let output = Command::new("devcontainer")
        .arg("stop")
        .arg("--workspace-folder")
        .arg(workspace)
        .output()
        .map_err(|e| Error::Other(format!("Failed to run devcontainer stop: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Other(format!("Failed to stop container: {}", stderr)));
    }

    Ok(())
}

/// List all devcontainers
///
/// Returns a list of container information
pub fn ps() -> Result<Vec<ContainerInfo>> {
    check_devcontainer_cli()?;

    let output = Command::new("devcontainer")
        .arg("ps")
        .arg("--format")
        .arg("json")
        .output()
        .map_err(|e| Error::Other(format!("Failed to run devcontainer ps: {}", e)))?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_container_list(&stdout)
}

/// Get container logs
///
/// # Arguments
///
/// * `workspace` - Path to the workspace folder
/// * `follow` - If true, follow log output (like tail -f)
/// * `tail` - Number of lines to show from the end
pub fn logs(workspace: &Path, follow: bool, tail: usize) -> Result<String> {
    check_devcontainer_cli()?;

    let mut cmd = Command::new("devcontainer");
    cmd.arg("logs")
        .arg("--workspace-folder")
        .arg(workspace);

    if follow {
        cmd.arg("--follow");
    }

    cmd.arg("--tail").arg(tail.to_string());

    if follow {
        // For follow mode, run in foreground and inherit output
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());
        cmd.status()
            .map_err(|e| Error::Other(format!("Failed to follow logs: {}", e)))?;
        Ok("".to_string())
    } else {
        let output = cmd.output()
            .map_err(|e| Error::Other(format!("Failed to get logs: {}", e)))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// Get container information for a workspace
fn get_container_info(workspace: &Path) -> Result<ContainerInfo> {
    let containers = ps()?;

    let workspace_str = workspace.to_string_lossy().to_string();

    // Find matching container by workspace path
    let container = containers.iter()
        .find(|c| c.workspace_folder == workspace_str)
        .ok_or_else(|| Error::Other("No running container found for workspace".to_string()))?;

    Ok(container.clone())
}

/// Extract image name from build output
fn extract_image_name(output: &str) -> Option<String> {
    // Look for patterns like "Built image: xxx" or "Successfully built xxx"
    for line in output.lines() {
        if line.contains("Built image:") || line.contains("Successfully built") || line.contains("=> => writing image") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(name) = parts.last() {
                let name = name.trim_end_matches('.');
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
    }

    // Try to parse from "=> => naming to" pattern
    for line in output.lines() {
        if line.contains("naming to") {
            let parts: Vec<&str> = line.split("naming to").collect();
            if parts.len() > 1 {
                let name = parts[1].trim().trim_end_matches('.');
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
    }

    None
}

/// Parse container list from JSON output
fn parse_container_list(json: &str) -> Result<Vec<ContainerInfo>> {
    #[derive(Debug, serde::Deserialize)]
    struct ContainerJson {
        #[serde(rename = "containerID")]
        container_id: String,
        #[serde(rename = "containerName")]
        container_name: String,
        state: String,
        #[serde(rename = "workspaceFolder")]
        workspace_folder: String,
    }

    let containers: Vec<ContainerJson> = serde_json::from_str(json)
        .unwrap_or_default();

    Ok(containers.into_iter()
        .map(|c| ContainerInfo {
            container_id: c.container_id,
            container_name: c.container_name,
            status: c.state,
            workspace_folder: c.workspace_folder,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_image_name_from_output() {
        let output = r#"
#1 [1/5] FROM docker.io/library/ubuntu:22.04
#1 DONE 0.0s
#2 [2/5] RUN apt-get update
#2 DONE 1.5s
#3 [3/5] RUN useradd test
#3 DONE 0.3s
=> => naming to myproject-dev:latest
"#;
        let name = extract_image_name(output);
        assert_eq!(name, Some("myproject-dev:latest".to_string()));
    }

    #[test]
    fn test_extract_image_name_from_built_output() {
        let output = "Built image: myproject-dev:latest";
        let name = extract_image_name(output);
        assert_eq!(name, Some("myproject-dev:latest".to_string()));
    }

    #[test]
    fn test_extract_image_name_none() {
        let output = "Some random output without image name";
        let name = extract_image_name(output);
        assert_eq!(name, None);
    }

    #[test]
    fn test_parse_container_list_empty() {
        let containers = parse_container_list("[]").unwrap();
        assert!(containers.is_empty());
    }

    #[test]
    fn test_parse_container_list_single() {
        let json = r#"[{
            "containerID": "abc123",
            "containerName": "devcontainer-myproject",
            "state": "running",
            "workspaceFolder": "/home/user/myproject"
        }]"#;
        let containers = parse_container_list(json).unwrap();
        assert_eq!(containers.len(), 1);
        assert_eq!(containers[0].container_id, "abc123");
        assert_eq!(containers[0].status, "running");
    }
}
