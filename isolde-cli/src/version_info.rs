//! # Build information and version display
//!
//! This module provides structured access to build metadata captured at compile time,
//! including version, git information, build timestamp, and profile.

use colored::Colorize;
use serde::Serialize;

/// Build metadata captured at compile time
#[derive(Debug, Clone, Serialize)]
pub struct BuildInfo {
    pub version: &'static str,
    pub build_timestamp: &'static str,
    pub commit_sha: &'static str,
    pub commit_short: &'static str,
    pub git_branch: &'static str,
    pub git_tag: &'static str,
    pub build_profile: &'static str,
    pub rust_version: &'static str,
}

impl BuildInfo {
    /// Get build info from compile-time environment variables
    pub fn get() -> Self {
        BuildInfo {
            version: env!("ISOLDE_VERSION"),
            build_timestamp: env!("ISOLDE_BUILD_TIMESTAMP"),
            commit_sha: env!("ISOLDE_GIT_COMMIT_SHA"),
            commit_short: env!("ISOLDE_GIT_COMMIT_SHORT"),
            git_branch: env!("ISOLDE_GIT_BRANCH"),
            git_tag: env!("ISOLDE_GIT_TAG"),
            build_profile: env!("ISOLDE_BUILD_PROFILE"),
            rust_version: env!("ISOLDE_RUST_VERSION"),
        }
    }

    /// Display version info at verbosity level 0 (just version)
    pub fn display_basic(&self) {
        println!("Isolde {}", self.version);
    }

    /// Display version info at verbosity level 1 (-v)
    pub fn display_verbose1(&self) {
        println!("Isolde {}", self.version);
        println!("Built: {}", self.build_timestamp);
    }

    /// Display version info at verbosity level 2 (-vv)
    pub fn display_verbose2(&self) {
        println!("Isolde {}", self.version);
        println!("Built: {}", self.build_timestamp);
        println!("Type: {}", self.build_profile);
        if self.commit_short != "unknown" {
            println!("Commit: {}", self.commit_short);
        }
    }

    /// Display version info at verbosity level 3 (-vvv)
    pub fn display_verbose3(&self) {
        println!("Isolde {}", self.version);
        println!("Built: {}", self.build_timestamp);
        println!("Build Type: {}", self.build_profile);

        // Show git info
        if self.commit_sha != "unknown" {
            println!();
            println!("{}", "Git:".bold());
            println!("  Branch: {}", self.git_branch);
            println!("  Commit: {}", self.commit_sha);
            if !self.git_tag.is_empty() {
                println!("  Tag: {}", self.git_tag);
            }
        }

        // Show toolchain info
        println!();
        println!("{}", "Toolchain:".bold());
        println!("  Rust: {}", self.rust_version);
        println!("  Profile: {}", self.build_profile);
    }

    /// Display as JSON
    pub fn display_json(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_info_get() {
        let info = BuildInfo::get();
        assert!(!info.version.is_empty());
    }
}
