//! # E2E Tests for Isolde CLI
//!
//! Integration tests that run the isolde binary and verify behavior.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
#[ignore = "Wait for isolde --version implementation"]
fn test_cli_version() {
    // Test: CLI should report version
    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("2.0.0"));
}

#[test]
#[ignore = "Wait for isolde --help implementation"]
fn test_cli_help() {
    // Test: CLI should show help
    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Isolde"))
        .stdout(predicate::str::contains("USAGE"));
}

#[test]
#[ignore = "Wait for isolde init command implementation"]
fn test_init_creates_config() {
    // Test: isolde init should create isolde.yaml
    let temp_dir = TempDir::new().unwrap();
    let project_name = "test-init-config";

    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("init")
        .arg(project_name)
        .arg("--template")
        .arg("python")
        .assert()
        .success();

    // Verify isolde.yaml was created
    let config_path = temp_dir.path().join(project_name).join("isolde.yaml");
    assert!(config_path.exists(), "isolde.yaml should be created");
}

#[test]
#[ignore = "Wait for isolde list-templates implementation"]
fn test_list_templates() {
    // Test: isolde --list-templates should show templates
    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.arg("--list-templates")
        .assert()
        .success()
        .stdout(predicate::str::contains("python"))
        .stdout(predicate::str::contains("nodejs"));
}

#[test]
#[ignore = "Wait for isolde list-presets implementation"]
fn test_list_presets() {
    // Test: isolde --list-presets should show presets
    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.arg("--list-presets")
        .assert()
        .success()
        .stdout(predicate::str::contains("python-ml"))
        .stdout(predicate::str::contains("node-api"));
}

#[test]
#[ignore = "Wait for isolde validate implementation"]
fn test_validate_missing_config() {
    // Test: isolde validate should fail without config
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("validate")
        .assert()
        .failure()
        .stderr(predicate::str::contains("isolde.yaml"));
}

#[test]
#[ignore = "Wait for isolde sync implementation"]
fn test_sync_without_init() {
    // Test: isolde sync should fail without init
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("sync")
        .assert()
        .failure()
        .stderr(predicate::str::contains("isolde.yaml"));
}

#[test]
#[ignore = "Wait for isolde diff implementation"]
fn test_diff_command() {
    // Test: isolde diff should work after init
    let temp_dir = TempDir::new().unwrap();
    let project_name = "test-diff";

    // First init a project
    let mut init_cmd = Command::cargo_bin("isolde").unwrap();
    init_cmd.current_dir(temp_dir.path())
        .arg("init")
        .arg(project_name)
        .arg("--template")
        .arg("python")
        .assert()
        .success();

    let project_path = temp_dir.path().join(project_name);

    // Then run diff
    let mut diff_cmd = Command::cargo_bin("isolde").unwrap();
    diff_cmd.current_dir(&project_path)
        .arg("diff")
        .assert()
        .success();
}

#[test]
#[ignore = "Wait for isolde doctor implementation"]
fn test_doctor_command() {
    // Test: isolde doctor should check environment
    let mut cmd = Command::cargo_bin("isolde").unwrap();
    cmd.arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Docker").or(predicate::str::contains("Environment")));
}
