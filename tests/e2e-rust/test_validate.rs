//! # E2E Tests for `isolde validate` command
//!
//! Tests configuration validation and container build verification.

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

    /// Create a minimal isolde.yaml config
    fn create_minimal_config(&self) {
        let config = r#"
name: test-project
template: python
lang_version: "3.12"
"#;
        fs::write(self.path().join("isolde.yaml"), config)
            .expect("Failed to write isolde.yaml");
    }

    /// Create a devcontainer.json for testing
    fn create_devcontainer_json(&self) {
        let devcontainer = r#"{
  "name": "Test Container",
  "image": "mcr.microsoft.com/devcontainers/base:ubuntu"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Wait for isolde validate implementation"]
    fn test_validate_valid_config() {
        // Test: Validate should pass for valid config
        let project = TestProject::new("test-validate-valid");

        project.create_minimal_config();

        // TODO: Run isolde validate
        // let output = Command::new("isolde")
        //     .args(["validate"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde validate");

        // assert!(output.status.success(), "Validate should succeed for valid config");
    }

    #[test]
    #[ignore = "Wait for isolde validate implementation"]
    fn test_validate_missing_config() {
        // Test: Validate should fail for missing config
        let project = TestProject::new("test-validate-missing");

        // Don't create isolde.yaml

        // TODO: Run isolde validate, should fail
        // let output = Command::new("isolde")
        //     .args(["validate"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde validate");

        // assert!(!output.status.success(), "Validate should fail for missing config");
    }

    #[test]
    #[ignore = "Wait for isolde validate implementation"]
    fn test_validate_invalid_template() {
        // Test: Validate should fail for invalid template
        let project = TestProject::new("test-validate-invalid-template");

        let config = r#"
name: test-project
template: nonexistent-template
lang_version: "3.12"
"#;
        fs::create_dir_all(project.path()).expect("Failed to create project dir");
        fs::write(project.path().join("isolde.yaml"), config)
            .expect("Failed to write isolde.yaml");

        // TODO: Run isolde validate, should fail
        // let output = Command::new("isolde")
        //     .args(["validate"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde validate");

        // assert!(!output.status.success(), "Validate should fail for invalid template");
    }

    #[test]
    #[ignore = "Wait for isolde validate implementation"]
    fn test_validate_builds_container() {
        // Test: Verify container builds successfully
        let project = TestProject::new("test-validate-build");

        project.create_minimal_config();
        project.create_devcontainer_json();

        if !docker_available() {
            println!("Docker not available, skipping container build test");
            return;
        }

        // TODO: Run isolde validate with --build flag
        // let output = Command::new("isolde")
        //     .args(["validate", "--build"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde validate");

        // assert!(output.status.success(), "Validate --build should succeed");

        // Clean up the built image
        // Command::new("docker")
        //     .args(["rmi", "-f", "test-validate-build-devcontainer"])
        //     .output()
        //     .ok();
    }

    #[test]
    #[ignore = "Wait for isolde validate implementation"]
    fn test_validate_checks_syntax() {
        // Test: Validate should check YAML syntax
        let project = TestProject::new("test-validate-syntax");

        let invalid_yaml = r#"
name: test-project
template: python
lang_version: "3.12
"#;  // Missing closing quote

        fs::create_dir_all(project.path()).expect("Failed to create project dir");
        fs::write(project.path().join("isolde.yaml"), invalid_yaml)
            .expect("Failed to write isolde.yaml");

        // TODO: Run isolde validate, should fail
        // let output = Command::new("isolde")
        //     .args(["validate"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde validate");

        // assert!(!output.status.success(), "Validate should fail for invalid YAML syntax");
    }

    #[test]
    #[ignore = "Wait for isolde validate implementation"]
    fn test_validate_unsupported_version() {
        // Test: Validate should fail for unsupported language version
        let project = TestProject::new("test-validate-version");

        let config = r#"
name: test-project
template: python
lang_version: "2.7"
"#;

        fs::create_dir_all(project.path()).expect("Failed to create project dir");
        fs::write(project.path().join("isolde.yaml"), config)
            .expect("Failed to write isolde.yaml");

        // TODO: Run isolde validate, should fail
        // let output = Command::new("isolde")
        //     .args(["validate"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde validate");

        // assert!(!output.status.success(), "Validate should fail for unsupported Python version");
    }
}
