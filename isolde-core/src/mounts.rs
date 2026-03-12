//! # Mount generation for devcontainer isolation levels
//!
//! Generates the correct bind-mount list for `devcontainer.json` based on the
//! configured [`IsolationLevel`].

use crate::config::{Config, IsolationLevel};

/// Generate the mount list for devcontainer.json based on isolation level.
///
/// # Arguments
///
/// * `config` - The parsed isolde.yaml configuration
/// * `host_auth_exists` - Whether `~/.claude/.credentials.json` exists on the host
/// * `host_providers_exist` - Whether `~/.claude/providers/` exists on the host
/// * `host_provider_file_exists` - Whether `~/.claude/provider` exists on the host
///   (stores the active auth provider name, e.g. "z.ai")
pub fn generate_mounts(
    config: &Config,
    host_auth_exists: bool,
    host_providers_exist: bool,
    host_provider_file_exists: bool,
) -> Vec<String> {
    let name = &config.name;

    // Mounts shared by all levels: project workspace + project-local .claude
    let project_mount = format!(
        "source=./project,target=/workspaces/{name},type=bind,consistency=cached"
    );
    let project_claude_mount = format!(
        "source=./.claude,target=/workspaces/{name}/.claude,type=bind,consistency=cached"
    );
    let machine_id_mount =
        "source=${localEnv:HOME}/.config/devcontainer/machine-id,target=/etc/machine-id,type=bind,consistency=cached"
            .to_string();

    match config.isolation() {
        IsolationLevel::None => {
            // Mount entire host ~/.claude + ~/.claude.json
            vec![
                project_mount,
                project_claude_mount,
                "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind,consistency=cached".to_string(),
                "source=${localEnv:HOME}/.claude.json,target=/home/${localEnv:USER}/.claude.json,type=bind,consistency=cached".to_string(),
                machine_id_mount,
            ]
        }
        IsolationLevel::Session => {
            // Host ~/.claude + overlay sessions, statsig, omc-config with local volumes
            let mut mounts = vec![
                project_mount,
                project_claude_mount,
                "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind,consistency=cached".to_string(),
                "source=${localEnv:HOME}/.claude.json,target=/home/${localEnv:USER}/.claude.json,type=bind,consistency=cached".to_string(),
                machine_id_mount,
            ];
            // Overlay more-specific paths with local volumes
            mounts.push(
                "source=./.isolde/volumes/claude-sessions,target=/home/${localEnv:USER}/.claude/projects,type=bind,consistency=cached".to_string(),
            );
            mounts.push(
                "source=./.isolde/volumes/claude-statsig,target=/home/${localEnv:USER}/.claude/statsig,type=bind,consistency=cached".to_string(),
            );
            mounts.push(
                "source=./.isolde/volumes/omc-config.json,target=/home/${localEnv:USER}/.claude/.omc-config.json,type=bind,consistency=cached".to_string(),
            );
            mounts
        }
        IsolationLevel::Workspace => {
            // Same as session + overlay plugins
            let mut mounts = vec![
                project_mount,
                project_claude_mount,
                "source=${localEnv:HOME}/.claude,target=/home/${localEnv:USER}/.claude,type=bind,consistency=cached".to_string(),
                "source=${localEnv:HOME}/.claude.json,target=/home/${localEnv:USER}/.claude.json,type=bind,consistency=cached".to_string(),
                machine_id_mount,
            ];
            mounts.push(
                "source=./.isolde/volumes/claude-sessions,target=/home/${localEnv:USER}/.claude/projects,type=bind,consistency=cached".to_string(),
            );
            mounts.push(
                "source=./.isolde/volumes/claude-statsig,target=/home/${localEnv:USER}/.claude/statsig,type=bind,consistency=cached".to_string(),
            );
            mounts.push(
                "source=./.isolde/volumes/omc-config.json,target=/home/${localEnv:USER}/.claude/.omc-config.json,type=bind,consistency=cached".to_string(),
            );
            mounts.push(
                "source=./.isolde/volumes/claude-plugins,target=/home/${localEnv:USER}/.claude/plugins,type=bind,consistency=cached".to_string(),
            );
            mounts
        }
        IsolationLevel::Full => {
            // Don't mount host ~/.claude at all; use local volume as claude home.
            // Mount host ~/.claude.json (app state needed for Claude Code to recognise auth).
            let mut mounts = vec![
                project_mount,
                project_claude_mount,
                "source=./.isolde/volumes/claude-home,target=/home/${localEnv:USER}/.claude,type=bind,consistency=cached".to_string(),
                "source=${localEnv:HOME}/.claude.json,target=/home/${localEnv:USER}/.claude.json,type=bind,consistency=cached".to_string(),
                machine_id_mount,
            ];
            // Conditionally mount auth files from host if they exist
            if host_auth_exists {
                mounts.push(
                    "source=${localEnv:HOME}/.claude/.credentials.json,target=/home/${localEnv:USER}/.claude/.credentials.json,type=bind,consistency=cached".to_string(),
                );
            }
            if host_providers_exist {
                mounts.push(
                    "source=${localEnv:HOME}/.claude/providers,target=/home/${localEnv:USER}/.claude/providers,type=bind,consistency=cached".to_string(),
                );
            }
            if host_provider_file_exists {
                mounts.push(
                    "source=${localEnv:HOME}/.claude/provider,target=/home/${localEnv:USER}/.claude/provider,type=bind,consistency=cached".to_string(),
                );
            }
            mounts
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(isolation: &str) -> Config {
        let yaml = format!(
            r#"
version: "0.1"
name: test-app
docker:
  image: ubuntu:latest
{isolation_field}
"#,
            isolation_field = if isolation.is_empty() {
                String::new()
            } else {
                format!("isolation: {isolation}")
            }
        );
        Config::from_str(&yaml).unwrap()
    }

    #[test]
    fn test_none_produces_5_mounts() {
        let config = test_config("none");
        let mounts = generate_mounts(&config, false, false, false);
        assert_eq!(mounts.len(), 5);
        assert!(mounts[0].contains("source=./project"));
        assert!(mounts[2].contains("source=${localEnv:HOME}/.claude,"));
        assert!(mounts[3].contains(".claude.json"));
        assert!(mounts[4].contains("machine-id"));
    }

    #[test]
    fn test_session_produces_8_mounts() {
        let config = test_config("session");
        let mounts = generate_mounts(&config, false, false, false);
        assert_eq!(mounts.len(), 8);
        assert!(mounts.iter().any(|m| m.contains("claude-sessions")));
        assert!(mounts.iter().any(|m| m.contains("claude-statsig")));
        assert!(mounts.iter().any(|m| m.contains("omc-config.json")));
    }

    #[test]
    fn test_workspace_produces_9_mounts() {
        let config = test_config("workspace");
        let mounts = generate_mounts(&config, false, false, false);
        assert_eq!(mounts.len(), 9);
        assert!(mounts.iter().any(|m| m.contains("claude-plugins")));
    }

    #[test]
    fn test_full_without_auth_produces_5_mounts() {
        let config = test_config("full");
        let mounts = generate_mounts(&config, false, false, false);
        assert_eq!(mounts.len(), 5);
        assert!(!mounts.iter().any(|m| m.contains("source=${localEnv:HOME}/.claude,")));
        assert!(mounts.iter().any(|m| m.contains("claude-home")));
    }

    #[test]
    fn test_full_with_credentials_only() {
        let config = test_config("full");
        let mounts = generate_mounts(&config, true, false, false);
        assert_eq!(mounts.len(), 6);
        assert!(mounts.iter().any(|m| m.contains(".credentials.json")));
    }

    #[test]
    fn test_full_with_all_auth_files() {
        let config = test_config("full");
        let mounts = generate_mounts(&config, true, true, true);
        assert_eq!(mounts.len(), 8);
        assert!(mounts.iter().any(|m| m.contains(".credentials.json")));
        assert!(mounts.iter().any(|m| m.contains("target=/home/${localEnv:USER}/.claude/providers,")));
        assert!(mounts.iter().any(|m| m.contains("target=/home/${localEnv:USER}/.claude/provider,")));
    }

    #[test]
    fn test_full_with_provider_file_only() {
        let config = test_config("full");
        let mounts = generate_mounts(&config, false, false, true);
        assert_eq!(mounts.len(), 6);
        assert!(mounts.iter().any(|m| m.contains("target=/home/${localEnv:USER}/.claude/provider,")));
    }

    #[test]
    fn test_default_isolation_is_session() {
        let config = test_config("");
        assert_eq!(config.isolation(), IsolationLevel::Session);
        let mounts = generate_mounts(&config, false, false, false);
        assert_eq!(mounts.len(), 8);
    }

    #[test]
    fn test_project_name_in_mounts() {
        let config = test_config("none");
        let mounts = generate_mounts(&config, false, false, false);
        assert!(mounts[0].contains("test-app"));
    }

    // --- Negative assertions ---

    #[test]
    fn test_none_has_no_local_volumes() {
        let config = test_config("none");
        let mounts = generate_mounts(&config, false, false, false);
        assert!(
            !mounts.iter().any(|m| m.contains(".isolde/volumes")),
            "None isolation should not reference .isolde/volumes"
        );
    }

    #[test]
    fn test_session_has_no_plugins_mount() {
        let config = test_config("session");
        let mounts = generate_mounts(&config, false, false, false);
        assert!(
            !mounts.iter().any(|m| m.contains("claude-plugins")),
            "Session isolation should not mount claude-plugins"
        );
    }

    #[test]
    fn test_full_does_not_mount_host_claude_dir() {
        let config = test_config("full");
        let mounts = generate_mounts(&config, false, false, false);
        // Use trailing comma to distinguish from .claude.json and .claude/ subpaths
        assert!(
            !mounts
                .iter()
                .any(|m| m.contains("source=${localEnv:HOME}/.claude,")),
            "Full isolation should not mount host ~/.claude directly"
        );
    }

    // --- Missing Full auth combinations ---

    #[test]
    fn test_full_with_providers_dir_only() {
        let config = test_config("full");
        let mounts = generate_mounts(&config, false, true, false);
        assert_eq!(mounts.len(), 6);
        assert!(mounts
            .iter()
            .any(|m| m.contains("target=/home/${localEnv:USER}/.claude/providers,")));
    }

    #[test]
    fn test_full_with_credentials_and_providers() {
        let config = test_config("full");
        let mounts = generate_mounts(&config, true, true, false);
        assert_eq!(mounts.len(), 7);
        assert!(mounts.iter().any(|m| m.contains(".credentials.json")));
        assert!(mounts
            .iter()
            .any(|m| m.contains("target=/home/${localEnv:USER}/.claude/providers,")));
    }
}
