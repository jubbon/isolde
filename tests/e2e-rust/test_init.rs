//! # E2E Tests for `isolde init` command
//!
//! Tests project initialization with templates and presets.

use std::fs;
use std::path::PathBuf;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Wait for isolde init implementation"]
    fn test_basic_init_sync() {
        // Test: Init project with default template, verify sync works
        let project = TestProject::new("test-basic-init");

        // TODO: Run isolde init with default template
        // Command::new("isolde")
        //     .args(["init", &project.project_name])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");

        // Verify project directory was created
        assert!(project.path().exists(), "Project directory should exist");

        // Verify isolde.yaml was created
        assert!(project.file_exists("isolde.yaml"), "isolde.yaml should exist");
    }

    #[test]
    #[ignore = "Wait for isolde init implementation"]
    fn test_preset_init() {
        // Test: Init with preset, verify correct config
        let project = TestProject::new("test-preset-init");

        // TODO: Run isolde init with preset
        // Command::new("isolde")
        //     .args(["init", &project.project_name, "--preset", "python-ml"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");

        // Verify project directory was created
        assert!(project.path().exists(), "Project directory should exist");

        // Verify isolde.yaml contains preset config
        let config = project.read_file("isolde.yaml")
            .expect("Failed to read isolde.yaml");

        assert!(config.contains("template: python"), "Config should use python template");
        assert!(config.contains("3.12"), "Config should use Python 3.12");
    }

    #[test]
    #[ignore = "Wait for isolde init implementation"]
    fn test_init_with_custom_template() {
        // Test: Init with custom template
        let project = TestProject::new("test-custom-template");

        // TODO: Run isolde init with template
        // Command::new("isolde")
        //     .args(["init", &project.project_name, "--template", "nodejs", "--lang-version", "20"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");

        assert!(project.path().exists(), "Project directory should exist");
        assert!(project.file_exists("isolde.yaml"), "isolde.yaml should exist");
    }

    #[test]
    #[ignore = "Wait for isolde init implementation"]
    fn test_init_fails_for_existing_directory() {
        // Test: Init should fail if directory already exists
        let project = TestProject::new("test-existing-dir");

        // Create the directory first
        fs::create_dir_all(project.path()).expect("Failed to create directory");

        // TODO: Run isolde init, should fail
        // let result = Command::new("isolde")
        //     .args(["init", &project.project_name])
        //     .current_dir(project.temp_dir.path())
        //     .output();

        // assert!(result.is_err() || !result.unwrap().status.success(), "Init should fail for existing directory");
    }

    #[test]
    #[ignore = "Wait for isolde init implementation"]
    fn test_init_with_invalid_preset() {
        // Test: Init should fail gracefully with invalid preset
        let project = TestProject::new("test-invalid-preset");

        // TODO: Run isolde init with invalid preset
        // let result = Command::new("isolde")
        //     .args(["init", &project.project_name, "--preset", "nonexistent-preset"])
        //     .current_dir(project.temp_dir.path())
        //     .output();

        // assert!(result.is_err() || !result.unwrap().status.success(), "Init should fail for invalid preset");
    }
}
