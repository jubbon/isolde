//! # Integration E2E Tests for Isolde
//!
//! Tests for pull, diff, doctor, and marketplace commands.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper struct for test project management
struct TestProject {
    temp_dir: TempDir,
    project_name: String,
}

impl TestProject {
    /// Create a new test project
    fn new(project_name: &str) -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp dir"),
            project_name: project_name.to_string(),
        }
    }

    /// Get the project path
    fn path(&self) -> PathBuf {
        self.temp_dir.path().join(&self.project_name)
    }

    /// Check if a file exists
    fn file_exists(&self, relative_path: &str) -> bool {
        self.path().join(relative_path).exists()
    }

    /// Read file contents
    fn read_file(&self, relative_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(self.path().join(relative_path))
    }

    /// Create a minimal isolde.yaml config
    fn create_minimal_config(&self) {
        let config = r#"
name: test-project
template: python
lang_version: "3.12"
"#;
        fs::create_dir_all(project.path()).expect("Failed to create project dir");
        fs::write(self.path().join("isolde.yaml"), config)
            .expect("Failed to write isolde.yaml");
    }

    /// Create a modified devcontainer.json
    fn create_modified_devcontainer(&self) {
        let devcontainer = r#"{
  "name": "Test Container - MODIFIED",
  "image": "mcr.microsoft.com/devcontainers/python:3.12",
  "customizations": {
    "vscode": {
      "extensions": ["ms-python.python"]
    }
  }
}"#;
        let devcontainer_path = self.path().join(".devcontainer");
        fs::create_dir_all(&devcontainer_path).expect("Failed to create .devcontainer");
        fs::write(devcontainer_path.join("devcontainer.json"), devcontainer)
            .expect("Failed to write devcontainer.json");
    }
}

/// Check if Docker is available
fn docker_available() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check if internet is available
fn internet_available() -> bool {
    Command::new("curl")
        .args(["-s", "--head", "--connect-timeout", "2", "https://github.com"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Wait for isolde diff implementation"]
    fn test_diff_shows_changes() {
        // Test: Verify diff output shows changes
        let project = TestProject::new("test-diff-changes");

        project.create_minimal_config();
        project.create_modified_devcontainer();

        // TODO: Run isolde diff
        // let output = Command::new("isolde")
        //     .args(["diff"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde diff");

        // let stdout = String::from_utf8_lossy(&output.stdout);
        // assert!(output.status.success(), "Diff should succeed");

        // Verify diff shows differences
        // assert!(stdout.contains("MODIFIED") || stdout.contains("-") || stdout.contains("+"),
        //         "Diff should show changes");
    }

    #[test]
    #[ignore = "Wait for isolde diff implementation"]
    fn test_diff_no_changes() {
        // Test: Verify diff reports no changes when in sync
        let project = TestProject::new("test-diff-no-changes");

        project.create_minimal_config();

        // TODO: First sync to create devcontainer, then run diff
        // Command::new("isolde")
        //     .args(["sync"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde sync");

        // let output = Command::new("isolde")
        //     .args(["diff"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde diff");

        // let stdout = String::from_utf8_lossy(&output.stdout);
        // assert!(stdout.contains("no changes") || stdout.contains("up to date") || stdout.contains("in sync"),
        //         "Diff should report no changes");
    }

    #[test]
    #[ignore = "Wait for isolde pull implementation"]
    fn test_pull_from_github() {
        // Test: Verify config pull works from GitHub
        let project = TestProject::new("test-pull-github");

        if !internet_available() {
            println!("Internet not available, skipping pull test");
            return;
        }

        project.create_minimal_config();

        // TODO: Run isolde pull with a valid GitHub URL
        // let output = Command::new("isolde")
        //     .args(["pull", "https://github.com/jubbon/isolde"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde pull");

        // assert!(output.status.success(), "Pull should succeed from GitHub");

        // Verify files were updated
        // assert!(project.file_exists("isolde.yaml"), "Config should exist after pull");
    }

    #[test]
    #[ignore = "Wait for isolde doctor implementation"]
    fn test_doctor_checks_environment() {
        // Test: Verify doctor checks environment
        let project = TestProject::new("test-doctor-env");

        // TODO: Run isolde doctor
        // let output = Command::new("isolde")
        //     .args(["doctor"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde doctor");

        // let stdout = String::from_utf8_lossy(&output.stdout);
        // assert!(output.status.success(), "Doctor should succeed");

        // Verify doctor checks key components
        // assert!(stdout.contains("Docker") || stdout.contains("docker"),
        //         "Doctor should check Docker");
        // assert!(stdout.contains("isolde") || stdout.contains("version"),
        //         "Doctor should report Isolde version");
    }

    #[test]
    #[ignore = "Wait for isolde marketplace implementation"]
    fn test_marketplace_plugins() {
        // Test: Verify plugin listing works
        if !internet_available() {
            println!("Internet not available, skipping marketplace test");
            return;
        }

        // TODO: Run isolde marketplace list
        // let output = Command::new("isolde")
        //     .args(["marketplace", "list"])
        //     .output()
        //     .expect("Failed to run isolde marketplace list");

        // assert!(output.status.success(), "Marketplace list should succeed");

        // let stdout = String::from_utf8_lossy(&output.stdout);
        // Verify some plugins are listed
        // assert!(stdout.contains("oh-my-claudecode") || stdout.contains("plugin"),
        //         "Marketplace should list plugins");
    }

    #[test]
    #[ignore = "Wait for isolde marketplace implementation"]
    fn test_marketplace_search() {
        // Test: Verify plugin search works
        if !internet_available() {
            println!("Internet not available, skipping marketplace search test");
            return;
        }

        // TODO: Run isolde marketplace search
        // let output = Command::new("isolde")
        //     .args(["marketplace", "search", "tdd"])
        //     .output()
        //     .expect("Failed to run isolde marketplace search");

        // assert!(output.status.success(), "Marketplace search should succeed");

        // let stdout = String::from_utf8_lossy(&output.stdout);
        // Verify search results are shown
        // assert!(!stdout.is_empty(), "Search should return results");
    }

    #[test]
    #[ignore = "Wait for full implementation"]
    fn test_full_workflow() {
        // Test: Complete workflow from init to sync to validate
        let project = TestProject::new("test-full-workflow");

        if !docker_available() {
            println!("Docker not available, skipping full workflow test");
            return;
        }

        // TODO: Implement full workflow
        // 1. isolde init
        // let init_output = Command::new("isolde")
        //     .args(["init", &project.project_name, "--template", "python"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");
        // assert!(init_output.status.success(), "Init should succeed");

        // 2. isolde sync
        // let sync_output = Command::new("isolde")
        //     .args(["sync"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde sync");
        // assert!(sync_output.status.success(), "Sync should succeed");

        // 3. isolde validate
        // let validate_output = Command::new("isolde")
        //     .args(["validate", "--build"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde validate");
        // assert!(validate_output.status.success(), "Validate --build should succeed");

        // Verify final state
        assert!(project.file_exists("isolde.yaml"), "Config should exist");
        assert!(project.file_exists(".devcontainer/devcontainer.json"), "Devcontainer should exist");
    }
}
