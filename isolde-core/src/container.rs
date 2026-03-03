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
/// * `detach` - If true, start container without attaching to shell
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
        // Skip the postAttachCommand (which typically starts the shell)
        // This allows us to start the container without attaching
        cmd.arg("--skip-post-attach");
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
        // For detach mode, wait for container to start
        cmd.spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| Error::Other(format!("Failed to run devcontainer up: {}", e)))?;

        // After up completes, start a keepalive process to keep container running
        // This is needed because the container would otherwise exit after postStartCommand
        let container_info = get_container_info(workspace)?;

        // Start a sleep process in the background to keep the container alive
        let keepalive_cmd = Command::new("devcontainer")
            .arg("exec")
            .arg("--workspace-folder")
            .arg(workspace)
            .arg("--")
            .args(["sh", "-c", "nohup sleep infinity >/dev/null 2>&1 &"])
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| Error::Other(format!("Failed to start keepalive: {}", e)))?;

        if !keepalive_cmd.success() {
            return Err(Error::Other("Failed to start keepalive process".to_string()));
        }

        // Small delay to ensure keepalive is running
        std::thread::sleep(std::time::Duration::from_millis(500));

        return Ok(container_info);
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
    // Use docker ps directly since devcontainers CLI 0.83.0 doesn't have ps command
    let output = Command::new("docker")
        .arg("ps")
        .arg("--format")
        .arg("json")
        .arg("--filter")
        .arg("label=devcontainer.container_id")  // Filter for devcontainers
        .output()
        .map_err(|e| Error::Other(format!("Failed to run docker ps: {}", e)))?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_docker_container_list(&stdout)
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
    let workspace_canonical = std::fs::canonicalize(workspace)
        .unwrap_or_else(|_| workspace.to_path_buf());
    let workspace_canonical_str = workspace_canonical.to_string_lossy().to_string();

    // First try to find by workspace_folder (for newer devcontainers CLI)
    if let Some(container) = containers.iter()
        .find(|c| !c.workspace_folder.is_empty() &&
                      (c.workspace_folder == workspace_str ||
                       c.workspace_folder == workspace_canonical_str))
    {
        return Ok(container.clone());
    }

    // For Docker ps output, we need to inspect each container to find the workspace mount
    for container in &containers {
        if let Ok(ws_folder) = get_workspace_folder_from_docker(&container.container_id) {
            if ws_folder == workspace_str || ws_folder == workspace_canonical_str {
                // Found matching container, fill in the workspace_folder
                return Ok(ContainerInfo {
                    workspace_folder: ws_folder,
                    ..container.clone()
                });
            }
        }
    }

    // If only one container exists, use it (for simple cases)
    if containers.len() == 1 {
        let container = &containers[0];
        return Ok(ContainerInfo {
            workspace_folder: workspace_str,
            ..container.clone()
        });
    }

    Err(Error::Other("No running container found for workspace".to_string()))
}

/// Get workspace folder from docker inspect
fn get_workspace_folder_from_docker(container_id: &str) -> Result<String> {
    let output = Command::new("docker")
        .arg("inspect")
        .arg(container_id)
        .output()
        .map_err(|e| Error::Other(format!("Failed to inspect container: {}", e)))?;

    if !output.status.success() {
        return Err(Error::Other("Failed to inspect container".to_string()));
    }

    // Docker API uses PascalCase field names
    #[allow(non_snake_case)]
    #[derive(Debug, serde::Deserialize)]
    struct Mount {
        Source: Option<String>,
        Destination: String,
    }

    // Docker API uses PascalCase field names
    #[allow(non_snake_case)]
    #[derive(Debug, serde::Deserialize)]
    struct InspectResult {
        Mounts: Vec<Mount>,
    }

    let inspect_results: Vec<InspectResult> = serde_json::from_slice(&output.stdout)
        .map_err(|_| Error::Other("Failed to parse inspect output".to_string()))?;

    if let Some(result) = inspect_results.first() {
        // Find the mount that contains "workspace" in the destination
        if let Some(mount) = result.Mounts.iter()
            .find(|m| m.Destination.contains("workspace") || m.Destination.contains("workspaces"))
        {
            if let Some(source) = &mount.Source {
                return Ok(source.clone());
            }
        }
    }

    Err(Error::Other("Workspace mount not found".to_string()))
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

/// Parse container list from Docker ps JSON format
/// Docker format differs from devcontainers CLI format
fn parse_docker_container_list(json: &str) -> Result<Vec<ContainerInfo>> {
    if json.trim().is_empty() {
        return Ok(vec![]);
    }

    #[derive(Debug, serde::Deserialize)]
    struct DockerContainerJson {
        #[serde(rename = "ID")]
        id: String,
        #[serde(rename = "Names")]
        names: String,
        #[serde(rename = "State")]
        state: String,
    }

    let containers: Vec<DockerContainerJson> = serde_json::from_str(json)
        .unwrap_or_default();

    Ok(containers.into_iter()
        .map(|c| ContainerInfo {
            container_id: c.id,
            container_name: c.names,
            status: c.state,
            workspace_folder: String::new(), // Will be filled by get_workspace_folder when needed
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

}
