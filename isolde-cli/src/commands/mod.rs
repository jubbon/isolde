//! # Isolde CLI Commands
//!
//! This module contains all command implementations for the Isolde CLI.

pub mod build;
pub mod diff;
pub mod doctor;
pub mod exec;
pub mod init;
pub mod logs;
pub mod ps;
pub mod run;
pub mod stop;
pub mod sync;
pub mod validate;

pub use build::{run as run_build, BuildOptions};
pub use diff::{run as run_diff, DiffFormat, DiffOptions};
pub use doctor::{run as run_doctor, DoctorOptions};
pub use exec::{run as run_exec, ExecOptions};
pub use init::{run as run_init, InitOptions};
pub use logs::{run as run_logs, LogsOptions};
pub use ps::{run as run_ps, PsOptions};
pub use run::{run as run_run, RunOptions};
pub use stop::{run as run_stop, StopOptions};
pub use sync::{run as run_sync, SyncOptions};
pub use validate::{run as run_validate, ValidateFormat, ValidateOptions};

/// Check if an agent has a working install.sh implementation
pub fn is_agent_implemented(agent: &str) -> bool {
    matches!(agent, "claude-code" | "codex" | "opencode")
}

/// Print a warning about stale or missing sync.
pub fn warn_sync_freshness(workspace: &std::path::Path) {
    use colored::Colorize;
    if let Some(warning) = isolde_core::sync_check::check_sync_freshness(workspace) {
        match warning {
            isolde_core::sync_check::SyncWarning::NeverSynced => {
                eprintln!(
                    "{} {}",
                    "⚠".yellow(),
                    "No devcontainer configuration found. Run `isolde sync` first.".yellow()
                );
            }
            isolde_core::sync_check::SyncWarning::ConfigNewer => {
                eprintln!(
                    "{} {}",
                    "⚠".yellow(),
                    "isolde.yaml has been modified since last sync. Run `isolde sync` to update."
                        .yellow()
                );
            }
        }
    }
}

/// Hint message for container-not-running errors
pub const CONTAINER_NOT_RUNNING_HINT: &str =
    "Make sure the container is running. Start with 'isolde run'.";
