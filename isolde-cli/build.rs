/// Build script for Isolde CLI
///
/// Reads the VERSION file from the project root and makes it available
/// at compile time via the ISOLDE_VERSION environment variable.
///
/// Also captures build metadata:
/// - Build timestamp (UTC)
/// - Git commit SHA (short and full)
/// - Git branch (if available)
/// - Git tag (if HEAD is at a tag)
/// - Build profile (debug/release)

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // Get the project root directory (where VERSION file is located)
    let project_root = Path::new("../VERSION");
    let version_path = if project_root.exists() {
        project_root.to_path_buf()
    } else {
        // Fallback: try current directory (for standalone builds)
        Path::new("VERSION").to_path_buf()
    };

    // Read version from VERSION file
    let version = fs::read_to_string(&version_path)
        .expect("Failed to read VERSION file")
        .trim()
        .to_string();

    // Validate version format (basic semver check)
    if !version.parse::<semver::Version>().is_ok() {
        panic!(
            "Invalid version format in VERSION file: '{}'. Expected semver format (e.g., 1.0.0)",
            version
        );
    }

    // Set cargo:rustc-env variable so it's available at compile time
    println!("cargo:rustc-env=ISOLDE_VERSION={}", version);

    // Also rerun build script if VERSION file changes
    println!("cargo:rerun-if-changed=../VERSION");

    // Capture build timestamp (UTC) using date command
    let build_timestamp = get_date_output();
    println!("cargo:rustc-env=ISOLDE_BUILD_TIMESTAMP={}", build_timestamp);

    // Capture build profile (debug/release)
    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=ISOLDE_BUILD_PROFILE={}", profile);

    // Capture git information
    let git_commit_sha = get_git_output(&["rev-parse", "HEAD"]);
    let git_commit_short = git_commit_sha
        .as_ref()
        .map(|s| s.chars().take(7).collect())
        .unwrap_or_else(|| "unknown".to_string());

    if let Some(sha) = &git_commit_sha {
        println!("cargo:rustc-env=ISOLDE_GIT_COMMIT_SHA={}", sha);
        println!("cargo:rustc-env=ISOLDE_GIT_COMMIT_SHORT={}", git_commit_short);
    } else {
        println!("cargo:rustc-env=ISOLDE_GIT_COMMIT_SHA=unknown");
        println!("cargo:rustc-env=ISOLDE_GIT_COMMIT_SHORT=unknown");
    }

    // Get git branch
    let git_branch = get_git_output(&["rev-parse", "--abbrev-ref", "HEAD"]);
    let branch_name = git_branch
        .as_ref()
        .filter(|b| b.as_str() != "HEAD")
        .map(String::as_str)
        .unwrap_or("unknown");
    println!("cargo:rustc-env=ISOLDE_GIT_BRANCH={}", branch_name);

    // Get git tag (if HEAD is at a tag)
    let git_tag = get_git_output(&["describe", "--tags", "--exact-match", "HEAD"]);
    let tag_name = git_tag.as_deref().unwrap_or("");
    println!("cargo:rustc-env=ISOLDE_GIT_TAG={}", tag_name);

    // Get rust version
    let rust_version = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let rust_version_output = Command::new(&rust_version)
        .arg("--version")
        .output()
        .ok();
    let rust_version_str = rust_version_output
        .as_ref()
        .and_then(|o| String::from_utf8(o.stdout.clone()).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=ISOLDE_RUST_VERSION={}", rust_version_str);
}

/// Run a git command and return stdout if successful
fn get_git_output(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// Get current UTC timestamp using date command
fn get_date_output() -> String {
    Command::new("date")
        .args(["-u", "+%Y-%m-%d %H:%M:%S UTC"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
