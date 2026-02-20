/// Build script for Isolde CLI
///
/// Reads the VERSION file from the project root and makes it available
/// at compile time via the ISOLDE_VERSION environment variable.

use std::env;
use std::fs;
use std::path::Path;

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
}
