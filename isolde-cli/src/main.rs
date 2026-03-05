//! # Isolde CLI
//!
//! Command-line interface for Isolde v2 - ISOLated Development Environment template system.

#![warn(missing_docs)]
#![warn(clippy::all)]

mod cli;
mod commands;
mod version_info;

use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

use cli::{Cli, Commands, print_info, print_warning};

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
            agent,
            agent_version,
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
            agent,
            agent_version,
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
            quick,
            format: doctor_format,
            dry_run,
        } => execute_doctor(fix, doctor_verbose, component, report, quick, doctor_format, dry_run, verbose),

        Commands::Version {
            verbosity,
            format,
        } => execute_version(verbosity, format),

        Commands::Build {
            workspace_folder,
            no_cache,
            image_name,
        } => execute_build(workspace_folder, no_cache, image_name, verbose),

        Commands::Run {
            workspace_folder,
            detach,
        } => execute_run(workspace_folder, detach, verbose),

        Commands::Exec {
            command,
            workspace_folder,
        } => execute_exec(command, workspace_folder, verbose),

        Commands::Stop {
            workspace_folder,
            force,
        } => execute_stop(workspace_folder, force, verbose),

        Commands::Ps {
            all,
        } => execute_ps(all, verbose),

        Commands::Logs {
            workspace_folder,
            follow,
            tail,
        } => execute_logs(workspace_folder, follow, tail, verbose),
    }
}

/// Execute the init command
fn execute_init(
    name: Option<String>,
    template: Option<String>,
    preset: Option<String>,
    lang_version: Option<String>,
    agent: String,
    agent_version: String,
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
        lang_version,
        agent,
        agent_version,
        http_proxy,
        https_proxy,
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

    // Print summary for text format
    if format_enum == commands::ValidateFormat::Text {
        if report.passed(warnings_as_errors) {
            println!(
                "\n{} Validation passed ({} checks, {} warnings)",
                "✔".green(),
                report.checks.len(),
                report.warning_count
            );
        } else {
            println!(
                "\n{} Validation failed ({} errors, {} warnings)",
                "✗".red(),
                report.error_count,
                report.warning_count
            );
        }
    }

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
    quick: bool,
    format: String,
    dry_run: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    if dry_run {
        println!("{}", "Running in dry-run mode - no changes will be made".yellow());
    }

    let opts = commands::DoctorOptions {
        fix: fix && !dry_run,
        verbose: doctor_verbose,
        component,
        report: report.map(PathBuf::from),
        cwd: PathBuf::from("."),
        output_json: format.as_str() == "json",
    };

    let result = commands::run_doctor(opts).map_err(|e| anyhow::anyhow!(e))?;

    if dry_run && fix {
        println!("{}", "Would apply fixes for any issues found above.".yellow());
    }

    // Exit with error code if diagnostics found errors
    if result.error_count > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Execute the version command
fn execute_version(verbose: u8, format: String) -> anyhow::Result<()> {
    let build_info = version_info::BuildInfo::get();

    match format.as_str() {
        "json" => {
            build_info.display_json();
        }
        _ => {
            match verbose {
                0 => build_info.display_basic(),
                1 => build_info.display_verbose1(),
                2 => build_info.display_verbose2(),
                _ => build_info.display_verbose3(),
            }
        }
    }
    Ok(())
}

/// Execute the build command
fn execute_build(
    workspace_folder: Option<String>,
    no_cache: bool,
    image_name: Option<String>,
    verbose: bool,
) -> anyhow::Result<()> {
    let opts = commands::BuildOptions {
        workspace_folder: workspace_folder.map(PathBuf::from),
        no_cache,
        image_name,
        cwd: PathBuf::from("."),
        verbose,
    };
    commands::run_build(opts).map_err(|e| anyhow::anyhow!(e))
}

/// Execute the run command
fn execute_run(
    workspace_folder: Option<String>,
    detach: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let opts = commands::RunOptions {
        workspace_folder: workspace_folder.map(PathBuf::from),
        detach,
        cwd: PathBuf::from("."),
        verbose,
    };
    commands::run_run(opts).map_err(|e| anyhow::anyhow!(e))
}

/// Execute the exec command
fn execute_exec(
    command: Vec<String>,
    workspace_folder: Option<String>,
    verbose: bool,
) -> anyhow::Result<()> {
    let opts = commands::ExecOptions {
        command,
        workspace_folder: workspace_folder.map(PathBuf::from),
        cwd: PathBuf::from("."),
        verbose,
    };
    commands::run_exec(opts).map_err(|e| anyhow::anyhow!(e))
}

/// Execute the stop command
fn execute_stop(
    workspace_folder: Option<String>,
    force: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let opts = commands::StopOptions {
        workspace_folder: workspace_folder.map(PathBuf::from),
        force,
        cwd: PathBuf::from("."),
        verbose,
    };
    commands::run_stop(opts).map_err(|e| anyhow::anyhow!(e))
}

/// Execute the ps command
fn execute_ps(
    all: bool,
    verbose: bool,
) -> anyhow::Result<()> {
    let opts = commands::PsOptions {
        all,
        verbose,
    };
    let _containers = commands::run_ps(opts).map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

/// Execute the logs command
fn execute_logs(
    workspace_folder: Option<String>,
    follow: bool,
    tail: usize,
    verbose: bool,
) -> anyhow::Result<()> {
    let opts = commands::LogsOptions {
        workspace_folder: workspace_folder.map(PathBuf::from),
        follow,
        tail,
        cwd: PathBuf::from("."),
        verbose,
    };
    commands::run_logs(opts).map_err(|e| anyhow::anyhow!(e))
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
    println!("  {}  Validate project configuration", "validate".cyan());
    println!("  {}  Show differences with template", "diff".cyan());
    println!("  {}  Run diagnostics", "doctor".cyan());
    println!("  {}  Show version information", "version".cyan());
    println!("  {}  Build devcontainer image", "build".green());
    println!("  {}  Run devcontainer (start and enter shell)", "run".green());
    println!("  {}  Execute command in running container", "exec".green());
    println!("  {}  Stop running container", "stop".green());
    println!("  {}  List running containers", "ps".green());
    println!("  {}  View container logs", "logs".green());
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
        let result = execute_version(0, "text".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_format_json() {
        let result = execute_version(0, "json".to_string());
        assert!(result.is_ok());
    }
}
