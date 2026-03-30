//! Check whether generated files are fresh relative to isolde.yaml.

use std::path::Path;

/// Warning about sync freshness
#[derive(Debug, PartialEq, Eq)]
pub enum SyncWarning {
    /// devcontainer.json does not exist — never synced
    NeverSynced,
    /// isolde.yaml is newer than devcontainer.json
    ConfigNewer,
}

/// Check if generated files are up-to-date with isolde.yaml.
/// Returns None if everything is fresh, Some(warning) otherwise.
pub fn check_sync_freshness(cwd: &Path) -> Option<SyncWarning> {
    let config_path = cwd.join("isolde.yaml");
    let devcontainer_path = cwd.join(".devcontainer/devcontainer.json");

    if !devcontainer_path.exists() {
        return Some(SyncWarning::NeverSynced);
    }

    let config_mtime = std::fs::metadata(&config_path).ok()?.modified().ok()?;
    let devcontainer_mtime = std::fs::metadata(&devcontainer_path).ok()?.modified().ok()?;

    if config_mtime > devcontainer_mtime {
        Some(SyncWarning::ConfigNewer)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_never_synced() {
        let tmp = tempfile::TempDir::new().unwrap();
        fs::write(tmp.path().join("isolde.yaml"), "version: '0.1'").unwrap();
        assert_eq!(
            check_sync_freshness(tmp.path()),
            Some(SyncWarning::NeverSynced)
        );
    }

    #[test]
    fn test_config_newer() {
        let tmp = tempfile::TempDir::new().unwrap();
        let dc_dir = tmp.path().join(".devcontainer");
        fs::create_dir_all(&dc_dir).unwrap();
        fs::write(dc_dir.join("devcontainer.json"), "{}").unwrap();
        // Small delay to ensure mtime difference
        thread::sleep(Duration::from_millis(1100));
        fs::write(tmp.path().join("isolde.yaml"), "version: '0.1'").unwrap();
        assert_eq!(
            check_sync_freshness(tmp.path()),
            Some(SyncWarning::ConfigNewer)
        );
    }

    #[test]
    fn test_fresh() {
        let tmp = tempfile::TempDir::new().unwrap();
        fs::write(tmp.path().join("isolde.yaml"), "version: '0.1'").unwrap();
        thread::sleep(Duration::from_millis(1100));
        let dc_dir = tmp.path().join(".devcontainer");
        fs::create_dir_all(&dc_dir).unwrap();
        fs::write(dc_dir.join("devcontainer.json"), "{}").unwrap();
        assert_eq!(check_sync_freshness(tmp.path()), None);
    }
}
