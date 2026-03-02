//! # Container state persistence
//!
//! This module provides state persistence for tracking container information.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Container state file name
pub const STATE_FILE: &str = ".isolde/state.json";

/// Container state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerState {
    /// Container ID
    pub container_id: String,

    /// Container name
    pub container_name: String,

    /// Image name
    pub image_name: String,

    /// Container status
    pub status: ContainerStatus,

    /// Workspace folder path
    pub workspace_folder: String,

    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Container status enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerStatus {
    /// Container is running
    #[serde(rename = "running")]
    Running,

    /// Container is stopped
    #[serde(rename = "stopped")]
    Stopped,

    /// Container is being built
    #[serde(rename = "building")]
    Building,

    /// Container has errored
    #[serde(rename = "error")]
    Error,
}

impl ContainerState {
    /// Create a new container state
    pub fn new(
        container_id: String,
        container_name: String,
        image_name: String,
        workspace_folder: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            container_id,
            container_name,
            image_name,
            status: ContainerStatus::Building,
            workspace_folder,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the container status
    pub fn with_status(mut self, status: ContainerStatus) -> Self {
        self.status = status;
        self.updated_at = chrono::Utc::now();
        self
    }

    /// Update the container ID
    pub fn with_container_id(mut self, container_id: String) -> Self {
        self.container_id = container_id;
        self.updated_at = chrono::Utc::now();
        self
    }

    /// Save state to file
    pub fn save(&self, workspace: &Path) -> Result<()> {
        let state_dir = workspace.join(".isolde");
        let state_file = state_dir.join("state.json");

        // Create .isolde directory if it doesn't exist
        if !state_dir.exists() {
            fs::create_dir_all(&state_dir).map_err(|e| {
                Error::FileError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create .isolde directory: {}", e),
                ))
            })?;
        }

        let json = serde_json::to_string_pretty(self).map_err(|e| {
            Error::Other(format!("Failed to serialize state: {}", e))
        })?;

        fs::write(&state_file, json).map_err(|e| {
            Error::FileError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to write state file: {}", e),
            ))
        })?;

        Ok(())
    }

    /// Load state from file
    pub fn load(workspace: &Path) -> Result<Self> {
        let state_file = workspace.join(STATE_FILE);

        if !state_file.exists() {
            return Err(Error::PathNotFound(state_file));
        }

        let content = fs::read_to_string(&state_file).map_err(|e| {
            Error::FileError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read state file: {}", e),
            ))
        })?;

        serde_json::from_str(&content).map_err(|e| {
            Error::Other(format!("Failed to parse state file: {}", e))
        })
    }

    /// Remove state file
    pub fn remove(workspace: &Path) -> Result<()> {
        let state_file = workspace.join(STATE_FILE);

        if !state_file.exists() {
            return Ok(());
        }

        fs::remove_file(&state_file).map_err(|e| {
            Error::FileError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to remove state file: {}", e),
            ))
        })?;

        Ok(())
    }

    /// Check if state file exists
    pub fn exists(workspace: &Path) -> bool {
        workspace.join(STATE_FILE).exists()
    }
}

/// Get state directory path
pub fn state_dir(workspace: &Path) -> PathBuf {
    workspace.join(".isolde")
}

/// Get state file path
pub fn state_file(workspace: &Path) -> PathBuf {
    workspace.join(STATE_FILE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_new() {
        let state = ContainerState::new(
            "abc123".to_string(),
            "devcontainer-test".to_string(),
            "test-image:latest".to_string(),
            "/home/user/test".to_string(),
        );

        assert_eq!(state.container_id, "abc123");
        assert_eq!(state.status, ContainerStatus::Building);
        assert_eq!(state.container_name, "devcontainer-test");
    }

    #[test]
    fn test_container_state_with_status() {
        let state = ContainerState::new(
            "abc123".to_string(),
            "devcontainer-test".to_string(),
            "test-image:latest".to_string(),
            "/home/user/test".to_string(),
        );

        let running_state = state.with_status(ContainerStatus::Running);
        assert_eq!(running_state.status, ContainerStatus::Running);
    }

    #[test]
    fn test_container_status_serialization() {
        let status = ContainerStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""running""#);

        let deserialized: ContainerStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ContainerStatus::Running);
    }

    #[test]
    fn test_container_state_serialization() {
        let state = ContainerState::new(
            "abc123".to_string(),
            "devcontainer-test".to_string(),
            "test-image:latest".to_string(),
            "/home/user/test".to_string(),
        );

        let json = serde_json::to_string_pretty(&state).unwrap();
        assert!(json.contains("container_id"));
        assert!(json.contains("abc123"));

        let deserialized: ContainerState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.container_id, "abc123");
    }

    #[test]
    fn test_state_dir_path() {
        let workspace = PathBuf::from("/home/user/project");
        let dir = state_dir(&workspace);
        assert_eq!(dir, PathBuf::from("/home/user/project/.isolde"));
    }

    #[test]
    fn test_state_file_path() {
        let workspace = PathBuf::from("/home/user/project");
        let file = state_file(&workspace);
        assert_eq!(file, PathBuf::from("/home/user/project/.isolde/state.json"));
    }
}
