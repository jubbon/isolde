//! # Isolde CLI
//!
//! Command-line interface for Isolde v2 - ISOLated Development Environment template system.

#![warn(missing_docs)]
#![warn(clippy::all)]

mod cli;
mod commands;

use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

use cli::{Cli, Commands, print_error, print_info, print_warning};

/// Isolde CLI version (from VERSION file)
const VERSION: &str = env!("ISOLDE_VERSION");

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // Disable colored output if requested
    if args.no_color {
        colored::control::set_override(false);
    }

    // Execute the command
    match args.command {
        Some(command) => execute_command(command, args.verbose),
        None => print_usage(),
    }
}

/// Execute a command
fn execute_command(command: Commands, verbose: bool) -> anyhow::Result<()> {
    if verbose {
        print_info(&format!("Isolde CLI v{}", VERSION));
    }

    match command {
        Commands::Init {
            name,
            template,
            preset,
            lang_version,
            claude_version,
            claude_provider,
            http_proxy,
            https_proxy,
            target_dir,
            list_templates,
            list_presets,
        } => execute_init(
            name,
            template,
            preset,
            lang_version,
            claude_version,
            claude_provider,
            http_proxy,
            https_proxy,
            target_dir,
            list_templates,
            list_presets,
            verbose,
        ),

        Commands::Sync {
            dry_run,
            force,
            version,
        } => execute_sync(dry_run, force, version, verbose),

        Commands::Pull { .. } => {
            eprintln!("Error: Pull command is disabled in this build (requires async/network support)");
            eprintln!("Please manually download the configuration file from the marketplace.");
            std::process::exit(1);
        }

        Commands::Validate {
            quick,
            verbose: validate_verbose,
            warnings_as_errors,
            format,
            path,
        } => execute_validate(quick, validate_verbose, warnings_as_errors, format, path, verbose),

        Commands::Diff {
            file,
            format,
            context,
        } => execute_diff(file, format, context, verbose),

        Commands::Doctor {
            fix,
            verbose: doctor_verbose,
            component,
            report,
        } => execute_doctor(fix, doctor_verbose, component, report, verbose),

        Commands::Version {
            verbose: version_verbose,
            format,
        } => execute_version(version_verbose, format),
    }
}

/// Execute the init command
fn execute_init(
    name: Option<String>,
    template: Option<String>,
    preset: Option<String>,
    lang_version: Option<String>,
    claude_version: String,
    claude_provider: Option<String>,
    http_proxy: Option<String>,
    https_proxy: Option<String>,
    target_dir: Option<String>,
    list_templates: bool,
    list_presets: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    if list_templates {
        print_info("Available templates:");
        println!("  {}  Python development environment with uv, pytest, and ruff", "python".cyan());
        println!("  {}  Node.js development environment with pnpm and TypeScript", "nodejs".green());
        println!("  {}  Rust development environment with cargo and rustfmt", "rust".red());
        println!();
        print_info("Use --template or --preset to select one");
        return Ok(());
    }

    if list_presets {
        print_info("Available presets:");
        println!("  {}  Python with ML libraries (numpy, pandas, scikit-learn)", "python-ml".cyan());
        println!("  {}  Node.js API server with Express and TypeScript", "node-api".green());
        println!("  {}  Rust CLI application with clap and tracing", "rust-cli".red());
        println!();
        print_info("Use --preset to select one");
        return Ok(());
    }

    // For now, init creates isolde.yaml in the current directory
    let cwd = target_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let opts = commands::InitOptions {
        template,
        preset,
        yes: false,
        cwd,
        name: name.clone(),
    };

    commands::run_init(opts).map_err(|e| anyhow::anyhow!(e))
}

/// Execute the sync command
fn execute_sync(
    dry_run: bool,
    force: bool,
    version: Option<String>,
    verbose: bool,
) -> anyhow::Result<()> {
    if verbose {
        if let Some(v) = &version {
            print_info(&format!("Target version: {}", v));
        }
        if force {
            print_warning("Force mode enabled - conflicts will be overwritten");
        }
    }

    let opts = commands::SyncOptions {
        dry_run,
        force,
        cwd: PathBuf::from("."),
    };

    commands::run_sync(opts).map_err(|e| anyhow::anyhow!(e))
}

/// Execute the pull command
///
/// Note: This command is disabled in builds without async/network support.
fn execute_pull(
    name: String,
    repo: String,
    r#ref: Option<String>,
    output: Option<String>,
    verify: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let _ = (name, repo, r#ref, output, verify, verbose);

    // Pull command disabled - requires async/network support
    eprintln!("Error: Pull command is disabled in this build.");
    eprintln!("Please manually download the configuration file from the marketplace.");
    std::process::exit(1);
}

/// Execute the validate command
fn execute_validate(
    quick: bool,
    validate_verbose: bool,
    warnings_as_errors: bool,
    format: String,
    path: Vec<String>,
    verbose: bool,
) -> anyhow::Result<()> {
    let format_enum = commands::ValidateFormat::from_str(&format)
        .unwrap_or(commands::ValidateFormat::Text);

    let opts = commands::ValidateOptions {
        quick,
        verbose: validate_verbose,
        warnings_as_errors,
        format: format_enum,
        paths: path.into_iter().map(PathBuf::from).collect(),
    };

    let report = commands::run_validate(opts).map_err(|e| anyhow::anyhow!(e))?;

    // Exit with error code if validation failed
    if !report.passed(warnings_as_errors) {
        std::process::exit(1);
    }

    Ok(())
}

/// Execute the diff command
fn execute_diff(
    file: Option<String>,
    format: String,
    context: usize,
    verbose: bool,
) -> anyhow::Result<()> {
    let format_enum = commands::DiffFormat::from_str(&format)
        .unwrap_or(commands::DiffFormat::Color);

    let opts = commands::DiffOptions {
        file,
        format: format_enum,
        context,
        cwd: PathBuf::from("."),
    };

    commands::run_diff(opts).map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

/// Execute the doctor command
fn execute_doctor(
    fix: bool,
    doctor_verbose: bool,
    component: Option<String>,
    report: Option<String>,
    verbose: bool,
) -> anyhow::Result<()> {
    let opts = commands::DoctorOptions {
        fix,
        verbose: doctor_verbose,
        component,
        report: report.map(PathBuf::from),
        cwd: PathBuf::from("."),
    };

    let result = commands::run_doctor(opts).map_err(|e| anyhow::anyhow!(e))?;

    // Exit with error code if diagnostics found errors
    if result.error_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Execute the version command
fn execute_version(version_verbose: bool, format: String) -> anyhow::Result<()> {
    match format.as_str() {
        "json" => {
            let json = serde_json::json!({
                "name": "isolde",
                "version": VERSION,
                "rust_version": env!("CARGO_PKG_RUST_VERSION"),
            });
            println!("{}", json);
        }
        _ => {
            println!("Isolde {}", VERSION);
            if version_verbose {
                println!("Release: {}", env!("ISOLDE_VERSION"));
                println!("Rust: {}", env!("CARGO_PKG_RUST_VERSION"));
            }
        }
    }
    Ok(())
}

/// Print usage information
fn print_usage() -> anyhow::Result<()> {
    println!("{}", "Isolde - ISOLated Development Environment".cyan().bold());
    println!();
    println!("Usage: {} {} {}", "isolde".green(), "[OPTIONS]".yellow(), "<COMMAND>".cyan());
    println!();
    println!("{}", "Available Commands:".bold());
    println!("  {}  Initialize a new project", "init".cyan());
    println!("  {}  Sync project with template changes", "sync".cyan());
    println!("  {}  Pull template from remote repository", "pull".cyan());
    println!("  {}  Validate project configuration", "validate".cyan());
    println!("  {}  Show differences with template", "diff".cyan());
    println!("  {}  Run diagnostics", "doctor".cyan());
    println!("  {}  Show version information", "version".cyan());
    println!();
    println!("Run {} {} for more information on a command.", "isolde".green(), "<COMMAND> --help".yellow());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_no_command() {
        // Should print usage without error
        let result = std::panic::catch_unwind(|| {
            let _ = Cli::try_parse_from(["isolde"]);
        });
        // No command is valid - it shows usage
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_format() {
        let result = execute_version(false, "text".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_format_json() {
        let result = execute_version(false, "json".to_string());
        assert!(result.is_ok());
    }
}
