//! # E2E Tests for `isolde sync` command
//!
//! Tests devcontainer generation and sync operations.

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

    /// Get the devcontainer path
    fn devcontainer_path(&self) -> PathBuf {
        self.path().join(".devcontainer")
    }

    /// Check if a file exists
    fn file_exists(&self, relative_path: &str) -> bool {
        self.path().join(relative_path).exists()
    }

    /// Check if devcontainer file exists
    fn devcontainer_file_exists(&self, file_name: &str) -> bool {
        self.devcontainer_path().join(file_name).exists()
    }

    /// Read file contents
    fn read_file(&self, relative_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(self.path().join(relative_path))
    }

    /// Check if the project is a git repo
    fn is_git_repo(&self) -> bool {
        self.path().join(".git").exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Wait for isolde sync implementation"]
    fn test_sync_generates_devcontainer() {
        // Test: Verify devcontainer files are generated
        let project = TestProject::new("test-sync-devcontainer");

        // TODO: Run isolde init then sync
        // Command::new("isolde")
        //     .args(["init", &project.project_name, "--template", "python"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");

        // Command::new("isolde")
        //     .args(["sync"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde sync");

        // Verify .devcontainer directory exists
        assert!(project.devcontainer_path().exists(), ".devcontainer directory should exist");

        // Verify core devcontainer files
        assert!(project.devcontainer_file_exists("devcontainer.json"), "devcontainer.json should exist");
        assert!(project.devcontainer_file_exists("Dockerfile"), "Dockerfile should exist");
    }

    #[test]
    #[ignore = "Wait for isolde sync implementation"]
    fn test_sync_applies_substitutions() {
        // Test: Verify template substitutions are applied
        let project = TestProject::new("test-sync-substitutions");

        // TODO: Run isolde init with specific values
        // Command::new("isolde")
        //     .args(["init", &project.project_name, "--template", "python", "--lang-version", "3.11"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");

        // Command::new("isolde")
        //     .args(["sync"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde sync");

        // Verify substitutions in devcontainer.json
        let devcontainer_json = project.read_file(".devcontainer/devcontainer.json")
            .expect("Failed to read devcontainer.json");

        assert!(devcontainer_json.contains(&project.project_name), "Project name should be substituted");
        assert!(devcontainer_json.contains("3.11"), "Python version should be substituted");
        assert!(!devcontainer_json.contains("{{PROJECT_NAME}}"), "Template placeholders should be replaced");
    }

    #[test]
    #[ignore = "Wait for isolde sync implementation"]
    fn test_sync_copies_features() {
        // Test: Verify features are copied to project
        let project = TestProject::new("test-sync-features");

        // TODO: Run isolde init then sync
        // Command::new("isolde")
        //     .args(["init", &project.project_name, "--template", "python"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");

        // Command::new("isolde")
        //     .args(["sync"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde sync");

        // Verify features directory exists
        assert!(project.devcontainer_file_exists("features/claude-code"), "claude-code feature should be copied");
        assert!(project.devcontainer_file_exists("features/proxy"), "proxy feature should be copied");
    }

    #[test]
    #[ignore = "Wait for isolde sync implementation"]
    fn test_sync_with_preset() {
        // Test: Verify sync works with preset configuration
        let project = TestProject::new("test-sync-preset");

        // TODO: Run isolde init with preset then sync
        // Command::new("isolde")
        //     .args(["init", &project.project_name, "--preset", "python-ml"])
        //     .current_dir(project.temp_dir.path())
        //     .output()
        //     .expect("Failed to run isolde init");

        // Command::new("isolde")
        //     .args(["sync"])
        //     .current_dir(project.path())
        //     .output()
        //     .expect("Failed to run isolde sync");

        // Verify devcontainer was generated
        assert!(project.devcontainer_path().exists(), ".devcontainer should exist");

        // Verify ML-specific features are included
        let devcontainer_json = project.read_file(".devcontainer/devcontainer.json")
            .expect("Failed to read devcontainer.json");

        assert!(devcontainer_json.contains("jupyter"), "Should include jupyter feature");
    }
}
