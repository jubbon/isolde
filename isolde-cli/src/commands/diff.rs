//! # Isolde diff command
//!
//! Show differences between current and generated files.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use isolde_core::config::Config;
use isolde_core::generator::Generator;
use isolde_core::{Error, Result};

/// Options for the diff command
#[derive(Debug, Clone)]
pub struct DiffOptions {
    /// Show differences for specific file only
    pub file: Option<String>,

    /// Output format
    pub format: DiffFormat,

    /// Number of context lines
    pub context: usize,

    /// Current working directory
    pub cwd: PathBuf,
}

/// Output format for diff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffFormat {
    /// Colored unified diff
    Color,
    /// Plain unified diff
    Unified,
    /// JSON output
    Json,
}

impl DiffFormat {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "color" => Some(Self::Color),
            "unified" => Some(Self::Unified),
            "json" => Some(Self::Json),
            _ => None,
        }
    }
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            file: None,
            format: DiffFormat::Color,
            context: 3,
            cwd: PathBuf::from("."),
        }
    }
}

/// Result of a diff operation
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// Files that would be created
    pub would_create: Vec<PathBuf>,
    /// Files that would be modified
    pub would_modify: Vec<PathBuf>,
    /// Files that would be deleted
    pub would_delete: Vec<PathBuf>,
    /// Files that are unchanged
    pub unchanged: Vec<PathBuf>,
    /// Detailed diffs for modified files
    pub file_diffs: Vec<FileDiff>,
}

/// Diff for a single file
#[derive(Debug, Clone)]
pub struct FileDiff {
    /// File path (relative to project root)
    pub path: PathBuf,
    /// Diff status
    pub status: DiffStatus,
    /// Line-by-line diff (if applicable)
    pub lines: Vec<DiffLine>,
}

/// Status of a file diff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffStatus {
    Created,
    Modified,
    Deleted,
    Unchanged,
}

/// A single line in a diff
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Line type
    pub line_type: DiffLineType,
    /// Line number (from original file if applicable)
    pub line_number: Option<usize>,
    /// Line content
    pub content: String,
}

/// Type of diff line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineType {
    Added,
    Removed,
    Context,
    Header,
}

/// Run the diff command
pub fn run(opts: DiffOptions) -> Result<DiffResult> {
    let config_path = opts.cwd.join("isolde.yaml");

    if !config_path.exists() {
        return Err(Error::InvalidTemplate(
            "isolde.yaml not found. Run 'isolde init' first.".to_string(),
        ));
    }

    println!("{}", "📊 Comparing project with template...".cyan());
    println!("{}", "─".repeat(50).dimmed());

    // Load configuration
    let config = Config::from_file(&config_path)?;

    // Create generator to get expected files
    let generator = Generator::new(config.clone())?;

    // Get dry run report
    let dry_run = generator.dry_run(&opts.cwd)?;

    // Generate file diffs
    let mut file_diffs = Vec::new();

    // If a specific file is requested, only diff that file
    if let Some(ref file) = opts.file {
        let file_path = opts.cwd.join(file);
        let diff = generate_file_diff(&file_path, &opts)?;
        file_diffs.push(diff);
    } else {
        // Diff all files that would be created or modified
        for path in &dry_run.would_create {
            let diff = FileDiff {
                path: path.clone(),
                status: DiffStatus::Created,
                lines: vec![],
            };
            file_diffs.push(diff);
        }

        for path in &dry_run.would_modify {
            let diff = generate_file_diff(path, &opts)?;
            file_diffs.push(diff);
        }
    }

    // Find files that exist but would not be generated (would be deleted)
    let would_delete = find_orphaned_files(&opts.cwd, &dry_run)?;

    // Collect unchanged files
    let devcontainer_dir = opts.cwd.join(".devcontainer");
    let mut unchanged = Vec::new();
    if devcontainer_dir.exists() {
        for entry in walk_dir(&devcontainer_dir)? {
            let relative = entry.strip_prefix(&opts.cwd).unwrap_or(&entry);
            if !dry_run.would_create.contains(&entry)
                && !dry_run.would_modify.contains(&entry)
            {
                unchanged.push(relative.to_path_buf());
            }
        }
    }

    let result = DiffResult {
        would_create: dry_run.would_create,
        would_modify: dry_run.would_modify,
        would_delete,
        unchanged,
        file_diffs,
    };

    // Print results
    print_diff_result(&result, &opts);

    Ok(result)
}

/// Generate diff for a single file
fn generate_file_diff(path: &Path, opts: &DiffOptions) -> Result<FileDiff> {
    let current_content = fs::read_to_string(path)
        .unwrap_or_else(|_| String::new());

    // Generate expected content
    let config = Config::from_file(&opts.cwd.join("isolde.yaml"))?;
    let generator = Generator::new(config)?;

    let expected_content = generate_expected_content(&generator, path, opts.cwd.clone())?;

    // Use similar crate for diff if available, otherwise simple line comparison
    let lines = compute_diff(&current_content, &expected_content, opts.context);

    let status = if current_content.is_empty() {
        DiffStatus::Created
    } else if expected_content.is_empty() {
        DiffStatus::Deleted
    } else if lines.iter().any(|l| matches!(l.line_type, DiffLineType::Added | DiffLineType::Removed)) {
        DiffStatus::Modified
    } else {
        DiffStatus::Unchanged
    };

    Ok(FileDiff {
        path: path.to_path_buf(),
        status,
        lines,
    })
}

/// Generate expected content for a file
fn generate_expected_content(generator: &Generator, path: &Path, cwd: PathBuf) -> Result<String> {
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    match file_name {
        "devcontainer.json" => {
            // Generate expected devcontainer.json
            let config = Config::from_file(&cwd.join("isolde.yaml"))?;
            generate_devcontainer_json(&config)
        }
        "Dockerfile" => {
            let config = Config::from_file(&cwd.join("isolde.yaml"))?;
            generate_dockerfile(&config)
        }
        "CLAUDE.md" => {
            let config = Config::from_file(&cwd.join("isolde.yaml"))?;
            generate_claude_md(&config)
        }
        ".gitignore" => {
            if path.to_str().unwrap_or("").contains("devcontainer") {
                Ok(generate_devcontainer_gitignore())
            } else {
                Ok(generate_project_gitignore())
            }
        }
        "README.md" => {
            let config = Config::from_file(&cwd.join("isolde.yaml"))?;
            generate_readme(&config)
        }
        _ => Ok(String::new()),
    }
}

/// Compute diff between two strings
fn compute_diff(old: &str, new: &str, context: usize) -> Vec<DiffLine> {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    let mut result = Vec::new();

    // Simple line-by-line comparison (for now, could use similar crate for better diffs)
    let mut old_idx = 0;
    let mut new_idx = 0;

    while old_idx < old_lines.len() || new_idx < new_lines.len() {
        let old_line = old_lines.get(old_idx);
        let new_line = new_lines.get(new_idx);

        match (old_line, new_line) {
            (Some(ol), Some(nl)) if ol == nl => {
                result.push(DiffLine {
                    line_type: DiffLineType::Context,
                    line_number: Some(old_idx + 1),
                    content: ol.to_string(),
                });
                old_idx += 1;
                new_idx += 1;
            }
            (Some(ol), Some(nl)) => {
                // Lines differ - show both
                result.push(DiffLine {
                    line_type: DiffLineType::Removed,
                    line_number: Some(old_idx + 1),
                    content: ol.to_string(),
                });
                result.push(DiffLine {
                    line_type: DiffLineType::Added,
                    line_number: Some(new_idx + 1),
                    content: nl.to_string(),
                });
                old_idx += 1;
                new_idx += 1;
            }
            (Some(ol), None) => {
                result.push(DiffLine {
                    line_type: DiffLineType::Removed,
                    line_number: Some(old_idx + 1),
                    content: ol.to_string(),
                });
                old_idx += 1;
            }
            (None, Some(nl)) => {
                result.push(DiffLine {
                    line_type: DiffLineType::Added,
                    line_number: Some(new_idx + 1),
                    content: nl.to_string(),
                });
                new_idx += 1;
            }
            _ => break,
        }
    }

    result
}

/// Find files that exist but are not in the template (orphans)
fn find_orphaned_files(cwd: &Path, dry_run: &isolde_core::generator::DryRunReport) -> Result<Vec<PathBuf>> {
    let mut orphans = Vec::new();
    let devcontainer_dir = cwd.join(".devcontainer");

    if !devcontainer_dir.exists() {
        return Ok(orphans);
    }

    let expected_files: HashSet<PathBuf> = dry_run.would_create.iter()
        .chain(dry_run.would_modify.iter())
        .cloned()
        .collect();

    for entry in walk_dir(&devcontainer_dir)? {
        if !expected_files.contains(&entry) {
            orphans.push(entry);
        }
    }

    Ok(orphans)
}

/// Walk a directory recursively
fn walk_dir(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)
        .map_err(|e| Error::FileError(std::io::Error::new(std::io::ErrorKind::Other, e)))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(walk_dir(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

/// Print diff result based on format
fn print_diff_result(result: &DiffResult, opts: &DiffOptions) {
    match opts.format {
        DiffFormat::Color | DiffFormat::Unified => {
            print_text_diff(result, opts);
        }
        DiffFormat::Json => {
            print_json_diff(result);
        }
    }
}

/// Print diff in text format
fn print_text_diff(result: &DiffResult, opts: &DiffOptions) {
    // Summary
    println!("\n{}", "Summary:".bold());
    if !result.would_create.is_empty() {
        println!(
            "  {} {}",
            "+".green(),
            format!("{} files would be created", result.would_create.len()).green()
        );
        for path in &result.would_create {
            let relative = path.strip_prefix(&opts.cwd).unwrap_or(path);
            println!("    {}", relative.display().to_string().green());
        }
    }

    if !result.would_modify.is_empty() {
        println!(
            "  {} {}",
            "~".yellow(),
            format!("{} files would be modified", result.would_modify.len()).yellow()
        );
        for path in &result.would_modify {
            let relative = path.strip_prefix(&opts.cwd).unwrap_or(path);
            println!("    {}", relative.display().to_string().yellow());
        }
    }

    if !result.would_delete.is_empty() {
        println!(
            "  {} {}",
            "-".red(),
            format!("{} files would be deleted (orphaned)", result.would_delete.len()).red()
        );
        for path in &result.would_delete {
            let relative = path.strip_prefix(&opts.cwd).unwrap_or(path);
            println!("    {}", relative.display().to_string().red());
        }
    }

    // Detailed diffs
    if !result.file_diffs.is_empty() && opts.format == DiffFormat::Color {
        println!("\n{}", "Detailed changes:".bold());
        for diff in &result.file_diffs {
            print_file_diff(diff, opts.context);
        }
    }

    println!("\n{}", "─".repeat(50).dimmed());
    println!("{}", "Run 'isolde sync' to apply changes.".dimmed());
}

/// Get icon for diff status
fn diff_status_icon(status: DiffStatus) -> colored::ColoredString {
    match status {
        DiffStatus::Created => "+".green(),
        DiffStatus::Modified => "~".yellow(),
        DiffStatus::Deleted => "-".red(),
        DiffStatus::Unchanged => "=".dimmed(),
    }
}

/// Get prefix for diff line type
fn diff_line_prefix(line_type: DiffLineType) -> colored::ColoredString {
    match line_type {
        DiffLineType::Added => "+".green(),
        DiffLineType::Removed => "-".red(),
        DiffLineType::Context => " ".dimmed(),
        DiffLineType::Header => "@".cyan(),
    }
}

/// Apply color to diff line content
fn line_content_color(content: &str, line_type: DiffLineType) -> String {
    match line_type {
        DiffLineType::Added => content.green().to_string(),
        DiffLineType::Removed => content.red().to_string(),
        DiffLineType::Header => content.cyan().to_string(),
        DiffLineType::Context => content.dimmed().to_string(),
    }
}

/// Print diff for a single file
fn print_file_diff(diff: &FileDiff, context_lines: usize) {
    let relative = diff
        .path
        .strip_prefix(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .unwrap_or(diff.path.as_path());

    println!(
        "\n{} {}",
        diff_status_icon(diff.status),
        relative.display().to_string().bold()
    );

    if !diff.lines.is_empty() {
        for line in &diff.lines {
            println!("{}{}", diff_line_prefix(line.line_type), line_content_color(&line.content, line.line_type));
        }
    }
}

/// Print diff in JSON format
fn print_json_diff(result: &DiffResult) {
    let json = serde_json::json!({
        "would_create": result.would_create.iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>(),
        "would_modify": result.would_modify.iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>(),
        "would_delete": result.would_delete.iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>(),
        "unchanged": result.unchanged.iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>(),
        "file_diffs": result.file_diffs.iter()
            .map(|d| serde_json::json!({
                "path": d.path.display().to_string(),
                "status": format!("{:?}", d.status),
                "lines": d.lines.iter().map(|l| serde_json::json!({
                    "type": format!("{:?}", l.line_type),
                    "line_number": l.line_number,
                    "content": l.content
                })).collect::<Vec<_>>()
            }))
            .collect::<Vec<_>>()
    });

    println!("{}", serde_json::to_string_pretty(&json).unwrap_or_default());
}

// Content generation helpers (simplified versions from sync command)

fn generate_devcontainer_json(config: &Config) -> Result<String> {
    let features = serde_json::json!({
        "ghcr.io/devcontainers/features/common-utils:2": {
            "installZsh": false,
            "installOhMyZsh": false,
            "upgradePackages": false
        }
    });

    let devcontainer = serde_json::json!({
        "name": format!("{} - Isolde Environment", config.name),
        "build": {
            "dockerfile": "Dockerfile",
            "context": ".."
        },
        "features": features
    });

    serde_json::to_string_pretty(&devcontainer)
        .map_err(|e| Error::Other(format!("Failed to serialize devcontainer.json: {}", e)))
}

fn generate_dockerfile(config: &Config) -> Result<String> {
    Ok(format!(
        r#"ARG BASE_IMAGE={}
FROM ${{BASE_IMAGE}}

ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=1000

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspaces

USER ${{USERNAME}}
"#,
        config.docker_image()
    ))
}

fn generate_claude_md(config: &Config) -> Result<String> {
    Ok(format!(
        r#"# Claude Code Configuration for {}

## Project Overview

**{}** - Schema Version {}

Docker Image: {}
Claude Provider: {}
"#,
        config.name, config.name, config.version, config.docker_image(), config.claude_provider()
    ))
}

fn generate_devcontainer_gitignore() -> String {
    r#".claude/
settings.json
.omc/
.vscode/
.idea/
"#.to_string()
}

fn generate_project_gitignore() -> String {
    r#".claude/
.vscode/
.idea/
.DS_Store
"#.to_string()
}

fn generate_readme(config: &Config) -> Result<String> {
    Ok(format!(
        r#"# {}

This project was created using the Isolle devcontainer template system.

## Getting Started

1. Open this project in VS Code
2. Reopen in Container when prompted
"#,
        config.name
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_options_default() {
        let opts = DiffOptions::default();
        assert!(opts.file.is_none());
        assert_eq!(opts.format, DiffFormat::Color);
        assert_eq!(opts.context, 3);
    }

    #[test]
    fn test_diff_format_from_str() {
        assert_eq!(DiffFormat::from_str("color"), Some(DiffFormat::Color));
        assert_eq!(DiffFormat::from_str("COLOR"), Some(DiffFormat::Color));
        assert_eq!(DiffFormat::from_str("unified"), Some(DiffFormat::Unified));
        assert_eq!(DiffFormat::from_str("json"), Some(DiffFormat::Json));
        assert_eq!(DiffFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_compute_diff_no_changes() {
        let content = "line1\nline2\nline3";
        let diff = compute_diff(content, content, 3);
        assert_eq!(diff.len(), 3);
        assert!(diff.iter().all(|d| d.line_type == DiffLineType::Context));
    }

    #[test]
    fn test_compute_diff_with_changes() {
        let old = "line1\nline2\nline3";
        let new = "line1\nmodified\nline3";
        let diff = compute_diff(old, new, 3);

        assert!(diff.iter().any(|d| d.line_type == DiffLineType::Removed));
        assert!(diff.iter().any(|d| d.line_type == DiffLineType::Added));
    }

    #[test]
    fn test_generate_devcontainer_gitignore() {
        let gitignore = generate_devcontainer_gitignore();
        assert!(gitignore.contains(".claude/"));
        assert!(gitignore.contains(".omc/"));
    }

    #[test]
    fn test_generate_project_gitignore() {
        let gitignore = generate_project_gitignore();
        assert!(gitignore.contains(".claude/"));
        assert!(gitignore.contains(".vscode/"));
    }
}
