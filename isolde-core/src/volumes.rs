//! # Volume directory preparation for isolation levels
//!
//! Creates the required `.isolde/volumes/` directories and placeholder files
//! based on the configured [`IsolationLevel`].

use std::fs;
use std::path::Path;

use crate::config::{Config, IsolationLevel};
use crate::{Error, Result};

/// Ensure the required `.isolde/volumes/` directories and placeholder files
/// exist for the given isolation level.
///
/// Directories are created if missing. Placeholder files (`claude.json`,
/// `omc-config.json`) are created as `{}` only if they don't already exist.
pub fn ensure_volumes(project_dir: &Path, config: &Config) -> Result<()> {
    let level = config.isolation();
    if level == IsolationLevel::None {
        return Ok(());
    }

    let volumes_dir = project_dir.join(".isolde").join("volumes");

    // Directories needed per level
    let dirs: &[&str] = match level {
        IsolationLevel::None => unreachable!(),
        IsolationLevel::Session => &[
            "claude-sessions",
            "claude-statsig",
        ],
        IsolationLevel::Workspace => &[
            "claude-sessions",
            "claude-statsig",
            "claude-plugins",
        ],
        IsolationLevel::Full => &[
            "claude-sessions",
            "claude-statsig",
            "claude-plugins",
            "claude-home",
        ],
    };

    for dir in dirs {
        let path = volumes_dir.join(dir);
        fs::create_dir_all(&path).map_err(|e| {
            Error::FileError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create {}: {}", path.display(), e),
            ))
        })?;
    }

    // Placeholder files needed per level
    let files: &[&str] = match level {
        IsolationLevel::None => unreachable!(),
        IsolationLevel::Session | IsolationLevel::Workspace => &[
            "omc-config.json",
        ],
        IsolationLevel::Full => &[
            "omc-config.json",
        ],
    };

    for file in files {
        let path = volumes_dir.join(file);
        if !path.exists() {
            fs::write(&path, "{}").map_err(|e| {
                Error::FileError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create {}: {}", path.display(), e),
                ))
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_config(isolation: &str) -> Config {
        let yaml = format!(
            r#"
version: "0.1"
name: test-app
docker:
  image: ubuntu:latest
isolation: {isolation}
"#
        );
        Config::from_str(&yaml).unwrap()
    }

    #[test]
    fn test_none_creates_nothing() {
        let tmp = TempDir::new().unwrap();
        let config = test_config("none");
        ensure_volumes(tmp.path(), &config).unwrap();
        assert!(!tmp.path().join(".isolde").exists());
    }

    #[test]
    fn test_session_creates_dirs_and_placeholder() {
        let tmp = TempDir::new().unwrap();
        let config = test_config("session");
        ensure_volumes(tmp.path(), &config).unwrap();

        let vol = tmp.path().join(".isolde/volumes");
        assert!(vol.join("claude-sessions").is_dir());
        assert!(vol.join("claude-statsig").is_dir());
        assert!(vol.join("omc-config.json").is_file());
        assert!(!vol.join("claude-plugins").exists());
        assert!(!vol.join("claude-home").exists());
    }

    #[test]
    fn test_workspace_creates_plugins_dir() {
        let tmp = TempDir::new().unwrap();
        let config = test_config("workspace");
        ensure_volumes(tmp.path(), &config).unwrap();

        let vol = tmp.path().join(".isolde/volumes");
        assert!(vol.join("claude-sessions").is_dir());
        assert!(vol.join("claude-statsig").is_dir());
        assert!(vol.join("claude-plugins").is_dir());
        assert!(vol.join("omc-config.json").is_file());
    }

    #[test]
    fn test_full_creates_all() {
        let tmp = TempDir::new().unwrap();
        let config = test_config("full");
        ensure_volumes(tmp.path(), &config).unwrap();

        let vol = tmp.path().join(".isolde/volumes");
        assert!(vol.join("claude-sessions").is_dir());
        assert!(vol.join("claude-statsig").is_dir());
        assert!(vol.join("claude-plugins").is_dir());
        assert!(vol.join("claude-home").is_dir());
        assert!(vol.join("omc-config.json").is_file());
        // claude.json is no longer created — host ~/.claude.json is mounted directly
        assert!(!vol.join("claude.json").exists());
    }

    #[test]
    fn test_idempotent_does_not_overwrite_placeholder() {
        let tmp = TempDir::new().unwrap();
        let config = test_config("session");
        ensure_volumes(tmp.path(), &config).unwrap();

        // Write custom content to placeholder
        let path = tmp.path().join(".isolde/volumes/omc-config.json");
        fs::write(&path, r#"{"key":"value"}"#).unwrap();

        // Run again — should not overwrite
        ensure_volumes(tmp.path(), &config).unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, r#"{"key":"value"}"#);
    }

    #[test]
    fn test_placeholder_content_is_empty_json() {
        let tmp = TempDir::new().unwrap();
        let config = test_config("session");
        ensure_volumes(tmp.path(), &config).unwrap();

        let content = fs::read_to_string(
            tmp.path().join(".isolde/volumes/omc-config.json"),
        )
        .unwrap();
        assert_eq!(content, "{}");
    }
}
