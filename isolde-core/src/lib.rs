//! # Isolde Core
//!
//! Core library for Isolde v2 - ISOLated Development Environment template system.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod config;
pub mod template;
pub mod generator;
pub mod container;
pub mod state;
pub mod mounts;
pub mod volumes;

// Re-export common types
pub use error::{Error, Result};

/// Isolde core library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, IsolationLevel};
    use tempfile::TempDir;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    /// Verify that every `.isolde/volumes/` source referenced by `generate_mounts`
    /// has a corresponding directory or file created by `ensure_volumes`.
    #[test]
    fn test_mount_sources_match_created_volumes() {
        for level in &["session", "workspace", "full"] {
            let yaml = format!(
                "version: \"0.1\"\nname: test-app\ndocker:\n  image: ubuntu:latest\nisolation: {level}"
            );
            let config = Config::from_str(&yaml).unwrap();

            // Create volumes
            let tmp = TempDir::new().unwrap();
            volumes::ensure_volumes(tmp.path(), &config).unwrap();

            // Collect what exists under .isolde/volumes/
            let volumes_dir = tmp.path().join(".isolde/volumes");
            let mut created: Vec<String> = Vec::new();
            if volumes_dir.exists() {
                for entry in std::fs::read_dir(&volumes_dir).unwrap() {
                    let entry = entry.unwrap();
                    created.push(entry.file_name().to_string_lossy().to_string());
                }
            }

            // Extract mount sources that reference .isolde/volumes/
            let mount_list = mounts::generate_mounts(&config, false, false, false);
            for mount in &mount_list {
                if let Some(rest) = mount.strip_prefix("source=./.isolde/volumes/") {
                    let volume_name = rest.split(',').next().unwrap();
                    assert!(
                        created.iter().any(|c| c == volume_name),
                        "Isolation '{level}': mount references .isolde/volumes/{volume_name} \
                         but ensure_volumes did not create it. Created: {created:?}"
                    );
                }
            }
        }
    }

    /// None isolation should have no volumes and no volume-referencing mounts.
    #[test]
    fn test_none_isolation_no_volumes_no_mounts() {
        let yaml = "version: \"0.1\"\nname: test-app\ndocker:\n  image: ubuntu:latest\nisolation: none";
        let config = Config::from_str(yaml).unwrap();
        assert_eq!(config.isolation(), IsolationLevel::None);

        let tmp = TempDir::new().unwrap();
        volumes::ensure_volumes(tmp.path(), &config).unwrap();
        assert!(!tmp.path().join(".isolde").exists());

        let mount_list = mounts::generate_mounts(&config, false, false, false);
        assert!(!mount_list.iter().any(|m| m.contains(".isolde/volumes")));
    }
}
