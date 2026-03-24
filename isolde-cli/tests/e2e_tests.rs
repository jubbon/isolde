//! # E2E Tests for Isolde CLI
//!
//! Integration tests that run the isolde binary and verify behavior.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_version() {
    // Test: CLI should report version
    Command::cargo_bin("isolde")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("isolde"));
}

#[test]
fn test_cli_help() {
    // Test: CLI should show help
    Command::cargo_bin("isolde")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("isolde"))
        .stdout(predicate::str::contains("Usage").or(predicate::str::contains("USAGE")));
}

#[test]
fn test_init_creates_config() {
    // Test: isolde init should create isolde.yaml in the current directory
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("isolde")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("test-init-config")
        .arg("--template")
        .arg("python")
        .arg("--agent")
        .arg("claude-code")
        // Pass --yes to skip interactive confirmation prompt
        .arg("--yes")
        .assert()
        .success();

    // init creates isolde.yaml in cwd (not in a subdirectory named after the project)
    let config_path = temp_dir.path().join("isolde.yaml");
    assert!(config_path.exists(), "isolde.yaml should be created in cwd");
}

#[test]
fn test_list_templates() {
    // Test: isolde init --list-templates should show templates
    Command::cargo_bin("isolde")
        .unwrap()
        .arg("init")
        .arg("--list-templates")
        .assert()
        .success()
        .stdout(predicate::str::contains("python"));
}

#[test]
fn test_list_presets() {
    // Test: isolde init --list-presets should show presets
    Command::cargo_bin("isolde")
        .unwrap()
        .arg("init")
        .arg("--list-presets")
        .assert()
        .success()
        .stdout(predicate::str::contains("python-ml").or(predicate::str::contains("node-api")));
}

#[test]
fn test_validate_missing_config() {
    // Test: isolde validate should fail without config
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("isolde")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("validate")
        .assert()
        .failure()
        // validate prints "isolde.yaml" to stdout (text format) and exits non-zero
        .stdout(predicate::str::contains("isolde.yaml"));
}

#[test]
fn test_sync_without_init() {
    // Test: isolde sync should fail without isolde.yaml
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("isolde")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("sync")
        .assert()
        .failure()
        .stderr(predicate::str::contains("isolde.yaml"));
}

#[test]
fn test_diff_command() {
    // Test: isolde diff in a dir without config should error gracefully
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("isolde")
        .unwrap()
        .current_dir(temp_dir.path())
        .arg("diff")
        .assert()
        .failure();
}

#[test]
fn test_doctor_command() {
    // Test: isolde doctor should run without panic (may exit non-zero if docker not present)
    Command::cargo_bin("isolde")
        .unwrap()
        .arg("doctor")
        .assert()
        // doctor may exit 1 if environment is unhealthy, but must not panic
        .stdout(
            predicate::str::contains("Docker")
                .or(predicate::str::contains("Environment"))
                .or(predicate::str::contains("Isolde"))
                .or(predicate::str::contains("devcontainer"))
                .or(predicate::str::contains("doctor")),
        );
}
