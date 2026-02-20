# Test Expansion Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add ~24 focused tests to `isolde-core` covering the main gaps in test coverage, specifically targeting the critical `generate()` workflow and file I/O operations.

**Architecture:** Hybrid approach using real filesystem with `tempfile` for integration tests, and stubbed git operations via trait extraction. Test fixtures in `tests/fixtures/` directory structure.

**Tech Stack:** Rust, `tempfile` crate for temporary directories, existing test infrastructure

---

## Phase 1: Foundation Setup

### Task 1: Verify tempfile dependency

**Files:**
- Check: `isolde-core/Cargo.toml`

**Step 1: Check if tempfile is in dev-dependencies**

Run: `grep -A 10 "\[dev-dependencies\]" isolde-core/Cargo.toml`

Expected: Either `tempfile = "..."` present or need to add it

**Step 2: Add tempfile if missing**

If not present, add to `isolde-core/Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3.10"
```

**Step 3: Verify cargo build succeeds**

Run: `cd isolde-core && cargo build`

Expected: Build succeeds

**Step 4: Commit**

```bash
git add isolde-core/Cargo.toml
git commit -m "chore: add tempfile dev-dependency for testing

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

### Task 2: Create test fixtures directory structure

**Files:**
- Create: `isolde-core/tests/fixtures/templates/`
- Create: `isolde-core/tests/fixtures/templates/nested/`
- Create: `isolde-core/tests/fixtures/configs/`
- Create: `isolde-core/tests/fixtures/features/mock-feature/`

**Step 1: Create fixture directories**

Run: `mkdir -p isolde-core/tests/fixtures/{templates/nested,configs,features/mock-feature}`

**Step 2: Create sample template file**

Create: `isolde-core/tests/fixtures/templates/sample.tera`

```tera
Project: {{project_name}}
Image: {{docker_image}}
Version: {{lang_version}}
```

**Step 3: Create nested template**

Create: `isolde-core/tests/fixtures/templates/nested/subdir.tera`

```tera
Nested: {{project_name}}
```

**Step 4: Create sample config**

Create: `isolde-core/tests/fixtures/configs/minimal-isolde.yaml`

```yaml
name: minimal-test
version: 0.1.0
workspace:
  dir: .
docker:
  image: ubuntu:latest
  build_args: []
claude:
  provider: anthropic
```

**Step 5: Create mock feature**

Create: `isolde-core/tests/fixtures/features/mock-feature/install.sh`

```bash
#!/bin/bash
echo "Mock feature install"
```

**Step 6: Commit**

```bash
git add isolde-core/tests/
git commit -m "test: add test fixtures for template and config tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

### Task 3: Extract GitRunner trait for testability

**Files:**
- Modify: `isolde-core/src/generator.rs`

**Step 1: Write trait definition test**

First, let's refactor `run_git` to use a trait. Add at top of `generator.rs` after imports:

```rust
/// Trait for running git commands - allows stubbing in tests
pub trait GitRunner {
    /// Run a git command in the specified directory
    fn run_git(&self, dir: &Path, args: &[&str]) -> Result<()>;
}

/// Default implementation using real git
pub struct RealGitRunner;

impl GitRunner for RealGitRunner {
    fn run_git(&self, dir: &Path, args: &[&str]) -> Result<()> {
        use std::process::Command;

        let result = Command::new("git")
            .current_dir(dir)
            .args(args)
            .output()
            .map_err(|e| Error::Other(format!("Failed to execute git: {}", e)))?;

        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(Error::Other(format!("Git command failed: {}", stderr)));
        }

        Ok(())
    }
}
```

**Step 2: Modify Generator to use GitRunner**

Update `Generator` struct:

```rust
pub struct Generator {
    /// Configuration loaded from isolde.yaml
    config: Config,
    /// Isolde installation root (for templates and features)
    isolde_root: PathBuf,
    /// Git runner (allows stubbing in tests)
    git_runner: Box<dyn GitRunner>,
}
```

**Step 3: Update Generator::new**

```rust
pub fn new(config: Config) -> Result<Self> {
    let isolde_root = Self::find_isolde_root()?;

    Ok(Self {
        config,
        isolde_root,
        git_runner: Box::new(RealGitRunner),
    })
}
```

**Step 4: Update run_git call sites**

Change in `initialize_git_repos`:

```rust
self.git_runner.run_git(&workspace_dir, &["init", "-q"])?;
```

**Step 5: Remove old run_git method**

Delete the `fn run_git(&self, dir: &Path, args: &[&str]) -> Result<()>` method.

**Step 6: Run tests to verify**

Run: `cargo test -p isolde-core generator`

Expected: All tests still pass

**Step 7: Commit**

```bash
git add isolde-core/src/generator.rs
git commit -m "refactor: extract GitRunner trait for testability

Allows stubbing git operations in tests while using real git
in production code.

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Phase 2: Template Tests

### Task 4: Test TemplateEngine::from_dir

**Files:**
- Modify: `isolde-core/src/template.rs`

**Step 1: Write failing test**

Add to `template.rs` tests module:

```rust
#[test]
fn test_from_dir_loads_templates() {
    let temp_dir = tempfile::tempdir().unwrap();
    let templates_dir = temp_dir.path();

    // Create sample template files
    fs::write(templates_dir.join("test.tera"), "Hello {{name}}").unwrap();
    fs::write(templates_dir.join("other.template"), "World {{value}}").unwrap();

    let engine = TemplateEngine::from_dir(templates_dir).unwrap();

    assert_eq!(engine.templates.len(), 2);
    assert!(engine.templates.contains_key("test"));
    assert!(engine.templates.contains_key("other"));
}

#[test]
fn test_from_dir_empty_dir_returns_error() {
    let temp_dir = tempfile::tempdir().unwrap();
    let templates_dir = temp_dir.path();

    let result = TemplateEngine::from_dir(templates_dir);
    assert!(result.is_err());
    assert!(matches!(result, Err(Error::InvalidTemplate(_))));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p isolde-core template::tests::test_from_dir`

Expected: Tests fail (functionality not yet tested)

**Step 3: No implementation needed - function exists**

The `from_dir` function already exists. Run the tests again.

**Step 4: Run tests to verify they pass**

Run: `cargo test -p isolde-core template::tests::test_from_dir`

Expected: PASS

**Step 5: Commit**

```bash
git add isolde-core/src/template.rs
git commit -m "test: add TemplateEngine::from_dir tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

### Task 5: Test template error cases

**Files:**
- Modify: `isolde-core/src/template.rs`

**Step 1: Write failing tests**

```rust
#[test]
fn test_register_template_file_invalid_path() {
    let mut engine = TemplateEngine::new().unwrap();
    let non_existent = PathBuf::from("/non/existent/path.tera");

    let result = engine.register_template_file("test", &non_existent);
    assert!(result.is_err());
}

#[test]
fn test_render_template_not_found_error() {
    let engine = TemplateEngine::new().unwrap();
    let ctx = TemplateContext::new("test".to_string(), "ubuntu:latest".to_string());

    let result = engine.render_template("nonexistent", &ctx);
    assert!(result.is_err());
}

#[test]
fn test_from_dir_nonexistent_path() {
    let result = TemplateEngine::from_dir("/nonexistent/path");
    assert!(result.is_err());
}

#[test]
fn test_format_plugin_list_empty() {
    let result = format_plugin_list(&[]);
    assert_eq!(result, "");
}

#[test]
fn test_render_with_config_integration() {
    let config = create_test_config();
    let engine = TemplateEngine::new().unwrap();

    let result = engine.render_with_config("devcontainer.json", &config);
    assert!(result.is_ok());
    let rendered = result.unwrap();
    assert!(rendered.contains("test-project"));
}
```

**Step 2: Run tests**

Run: `cargo test -p isolde-core template::tests::test_`

Expected: Most should pass, verify each

**Step 3: Commit**

```bash
git add isolde-core/src/template.rs
git commit -m "test: add template error case tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Phase 3: Config Tests

### Task 6: Test config file I/O errors

**Files:**
- Modify: `isolde-core/src/config.rs`

**Step 1: Write failing tests**

```rust
#[test]
fn test_from_file_not_found_error() {
    let result = Config::from_file(Path::new("/nonexistent/isolde.yaml"));
    assert!(result.is_err());
}

#[test]
fn test_from_file_invalid_yaml_error() {
    let temp_dir = tempfile::tempdir().unwrap();
    let invalid_yaml = temp_dir.path().join("invalid.yaml");

    fs::write(&invalid_yaml, "name: value\n  bad indent: broken: yaml:").unwrap();

    let result = Config::from_file(&invalid_yaml);
    assert!(result.is_err());
}

#[test]
fn test_template_info_default_values() {
    let yaml = r#"
name: Test
description: Test template
version: "1.0"
lang_version_default: "1.0"
"#;
    let info: TemplateInfo = serde_yaml::from_str(yaml).unwrap();

    assert!(info.features.is_empty());
    assert!(info.supported_versions.is_empty());
}

#[test]
fn test_plugin_config_activate_default_true() {
    let yaml = r#"
marketplace: omc
name: test-plugin
"#;
    let plugin: PluginConfig = serde_yaml::from_str(yaml).unwrap();

    assert_eq!(plugin.activate, true);
}
```

**Step 2: Run tests**

Run: `cargo test -p isolde-core config::tests::test_from`

Expected: Tests verify error handling

**Step 3: Commit**

```bash
git add isolde-core/src/config.rs
git commit -m "test: add config file I/O error tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Phase 4: Generator Core Tests

### Task 7: Test generator helper functions

**Files:**
- Modify: `isolde-core/src/generator.rs`

**Step 1: Write failing tests**

```rust
#[test]
fn test_language_to_version_key_mappings() {
    use crate::generator::Generator;

    assert_eq!(Generator::language_to_version_key("python"), Some("PYTHON_VERSION".to_string()));
    assert_eq!(Generator::language_to_version_key("node"), Some("NODE_VERSION".to_string()));
    assert_eq!(Generator::language_to_version_key("nodejs"), Some("NODE_VERSION".to_string()));
    assert_eq!(Generator::language_to_version_key("javascript"), Some("NODE_VERSION".to_string()));
    assert_eq!(Generator::language_to_version_key("rust"), Some("RUST_VERSION".to_string()));
    assert_eq!(Generator::language_to_version_key("go"), Some("GO_VERSION".to_string()));
    assert_eq!(Generator::language_to_version_key("golang"), Some("GO_VERSION".to_string()));
    assert_eq!(Generator::language_to_version_key("unknown"), None);
}

#[test]
fn test_render_devcontainer_json_substitutions() {
    let config = create_test_config();
    let generator = Generator::new(config).unwrap();

    let result = generator.render_devcontainer_json();
    assert!(result.is_ok());

    let rendered = result.unwrap();
    assert!(rendered.contains("test-project"));
    assert!(!rendered.contains("{{PROJECT_NAME}}"));
    assert!(rendered.contains("\"haiku\":"));
    assert!(rendered.contains("\"sonnet\":"));
}
```

**Step 2: Run tests**

Run: `cargo test -p isolde-core generator::tests::test_language`

Expected: Verify substitutions work

**Step 3: Make language_to_version_key public if needed**

If compiler error, add `pub` to `fn language_to_version_key`.

**Step 4: Run tests again**

Expected: PASS

**Step 5: Commit**

```bash
git add isolde-core/src/generator.rs
git commit -m "test: add generator helper function tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

### Task 8: Test file copy operations

**Files:**
- Modify: `isolde-core/src/generator.rs`

**Step 1: Write failing tests**

```rust
#[test]
fn test_copy_dir_recursive() {
    let temp_dir = tempfile::tempdir().unwrap();
    let src = temp_dir.path().join("src");
    let dst = temp_dir.path().join("dst");

    // Create source structure
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("file1.txt"), "content1").unwrap();
    fs::create_dir_all(src.join("subdir")).unwrap();
    fs::write(src.join("subdir/file2.txt"), "content2").unwrap();

    let config = create_test_config();
    let generator = Generator::new(config).unwrap();

    generator.copy_dir_recursive(&src, &dst).unwrap();

    assert!(dst.exists());
    assert!(dst.join("file1.txt").exists());
    assert!(dst.join("subdir/file2.txt").exists());
}

#[test]
fn test_copy_core_features_with_temp_dir() {
    let temp_dir = tempfile::tempdir().unwrap();
    let features_dir = temp_dir.path().join("features");

    // Setup mock isolde root
    let mock_root = temp_dir.path().join("isolde");
    fs::create_dir_all(mock_root.join("core/features/feature1")).unwrap();
    fs::write(mock_root.join("core/features/feature1/install.sh"), "#!/bin/bash").unwrap();

    let config = create_test_config();
    let mut generator = Generator::new(config).unwrap();
    generator.isolde_root = mock_root;

    let copied = generator.copy_core_features(&features_dir).unwrap();

    assert!(!copied.is_empty());
    assert!(features_dir.join("feature1").exists());
}
```

**Step 2: Run tests**

Run: `cargo test -p isolde-core generator::tests::test_copy`

Expected: May need to make `copy_dir_recursive` and adjust isolde_root visibility

**Step 3: Adjust visibility if needed**

Add `pub(crate)` to `copy_dir_recursive` if needed.

**Step 4: Run tests again**

Expected: PASS

**Step 5: Commit**

```bash
git add isolde-core/src/generator.rs
git commit -m "test: add file copy operation tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

### Task 9: Create mock GitRunner for tests

**Files:**
- Modify: `isolde-core/src/generator.rs`

**Step 1: Add MockGitRunner**

Add to tests module:

```rust
/// Mock git runner for testing
struct MockGitRunner {
    should_fail: bool,
}

impl MockGitRunner {
    fn new() -> Self {
        Self { should_fail: false }
    }

    fn failing() -> Self {
        Self { should_fail: true }
    }
}

impl GitRunner for MockGitRunner {
    fn run_git(&self, _dir: &Path, _args: &[&str]) -> Result<()> {
        if self.should_fail {
            Err(Error::Other("Mock git failure".to_string()))
        } else {
            Ok(())
        }
    }
}
```

**Step 2: Add test using mock**

```rust
#[test]
fn test_initialize_git_with_mock_runner() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config = create_test_config();

    let mut generator = Generator::new(config).unwrap();
    generator.git_runner = Box::new(MockGitRunner::new());

    let result = generator.initialize_git_repos(temp_dir.path());
    assert!(result.is_ok());
}

#[test]
fn test_run_git_command_fails_propagates_error() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config = create_test_config();

    let mut generator = Generator::new(config).unwrap();
    generator.git_runner = Box::new(MockGitRunner::failing());

    let result = generator.initialize_git_repos(temp_dir.path());
    assert!(result.is_err());
}
```

**Step 3: Run tests**

Run: `cargo test -p isolde-core generator::tests::test_initialize_git`

Expected: Tests verify git error handling

**Step 4: Commit**

```bash
git add isolde-core/src/generator.rs
git commit -m "test: add MockGitRunner and git error tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Phase 5: Integration Tests

### Task 10: Full generate() workflow test

**Files:**
- Modify: `isolde-core/src/generator.rs`

**Step 1: Write integration test**

```rust
#[test]
fn test_generate_full_workflow() {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_dir = temp_dir.path();

    // Setup mock isolde root
    let mock_root = temp_dir.path().join("isolde");
    fs::create_dir_all(mock_root.join("core/features/claude-code")).unwrap();
    fs::create_dir_all(mock_root.join("core/features/proxy")).unwrap();
    fs::create_dir_all(mock_root.join("core/features/plugin-manager")).unwrap();
    fs::write(mock_root.join("core/features/claude-code/install.sh"), "#!/bin/bash\necho claude").unwrap();
    fs::write(mock_root.join("core/features/proxy/install.sh"), "#!/bin/bash\necho proxy").unwrap();
    fs::write(mock_root.join("core/features/plugin-manager/install.sh"), "#!/bin/bash\necho plugin").unwrap();

    let config = create_test_config();
    let mut generator = Generator::new(config).unwrap();
    generator.isolde_root = mock_root;
    generator.git_runner = Box::new(MockGitRunner::new());

    let report = generator.generate(output_dir).unwrap();

    // Verify files created
    assert!(!report.files_created.is_empty());

    let devcontainer_dir = output_dir.join(".devcontainer");
    assert!(devcontainer_dir.exists());
    assert!(devcontainer_dir.join("devcontainer.json").exists());
    assert!(devcontainer_dir.join("Dockerfile").exists());
    assert!(devcontainer_dir.join("features/claude-code").exists());

    let workspace_dir = output_dir.join("./project");
    assert!(workspace_dir.exists());
    assert!(workspace_dir.join(".claude/config.json").exists());
    assert!(workspace_dir.join("README.md").exists());
    assert!(workspace_dir.join(".gitignore").exists());
}
```

**Step 2: Run test**

Run: `cargo test -p isolde-core generator::tests::test_generate_full`

Expected: PASS (after fixing any issues)

**Step 3: Commit**

```bash
git add isolde-core/src/generator.rs
git commit -m "test: add full generate workflow integration test

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

### Task 11: Test dry_run functionality

**Files:**
- Modify: `isolde-core/src/generator.rs`

**Step 1: Write test**

```rust
#[test]
fn test_dry_run_all_cases() {
    let temp_dir = tempfile::tempdir().unwrap();

    // Setup mock isolde root
    let mock_root = temp_dir.path().join("isolde");
    fs::create_dir_all(mock_root.join("core/features/test-feature")).unwrap();

    let config = create_test_config();
    let mut generator = Generator::new(config).unwrap();
    generator.isolde_root = mock_root;

    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();

    // First run - should show all creates
    let report = generator.dry_run(&output_dir).unwrap();
    assert!(!report.would_create.is_empty());
    assert!(report.would_modify.is_empty());

    // Create one file
    fs::write(output_dir.join(".devcontainer/devcontainer.json"), "{}").unwrap();

    // Second run - should show modify
    let report2 = generator.dry_run(&output_dir).unwrap();
    assert!(report2.would_modify.iter().any(|p| p.ends_with("devcontainer.json")));
}
```

**Step 2: Run test**

Run: `cargo test -p isolde-core generator::tests::test_dry_run`

Expected: PASS

**Step 3: Commit**

```bash
git add isolde-core/src/generator.rs
git commit -m "test: add dry_run functionality tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Phase 6: Error Display Tests

### Task 12: Test error variant display messages

**Files:**
- Modify: `isolde-core/src/error.rs`

**Step 1: Write tests**

```rust
#[test]
fn test_all_error_variants_display() {
    use std::path::PathBuf;

    assert_eq!(
        Error::PresetNotFound("test".to_string()).to_string(),
        "Preset not found: test"
    );

    assert_eq!(
        Error::InvalidTemplate("bad template".to_string()).to_string(),
        "Invalid template configuration: bad template"
    );

    let path = PathBuf::from("/test/path");
    assert_eq!(
        Error::PathNotFound(path).to_string(),
        "Path not found: /test/path"
    );

    assert_eq!(
        Error::InvalidSubstitution("bad sub".to_string()).to_string(),
        "Invalid substitution: bad sub"
    );

    assert_eq!(
        Error::Other("generic error".to_string()).to_string(),
        "generic error"
    );

    assert_eq!(
        Error::InvalidMarketplace("bad url".to_string()).to_string(),
        "Invalid marketplace: bad url"
    );

    assert_eq!(
        Error::PluginNotFound("plugin".to_string()).to_string(),
        "Plugin not found: plugin"
    );

    assert_eq!(
        Error::InvalidPlugin("bad plugin".to_string()).to_string(),
        "Invalid plugin: bad plugin"
    );

    assert_eq!(
        Error::MarketplaceError("market error".to_string()).to_string(),
        "Marketplace error: market error"
    );
}
```

**Step 2: Run tests**

Run: `cargo test -p isolde-core error::tests::test_all`

Expected: PASS

**Step 3: Commit**

```bash
git add isolde-core/src/error.rs
git commit -m "test: add error variant display tests

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

### Task 13: Test marketplace Display implementations

**Files:**
- Modify: `isolde-core/src/marketplace.rs`

**Step 1: Write tests**

```rust
#[test]
fn test_marketplace_display_format() {
    let marketplace = Marketplace {
        name: "test-market".to_string(),
        url: "https://example.com/market".to_string(),
    };

    let display = format!("{}", marketplace);
    assert!(display.contains("test-market"));
    assert!(display.contains("https://example.com/market"));
}

#[test]
fn test_plugin_display_format() {
    let plugin = Plugin::new(
        "test-plugin".to_string(),
        "test-market".to_string(),
        "A test plugin".to_string(),
    )
    .with_version("1.0.0".to_string());

    let display = format!("{}", plugin);
    assert!(display.contains("test-plugin"));
    assert!(display.contains("1.0.0"));
    assert!(display.contains("test-market"));
}
```

**Step 2: Run tests**

Run: `cargo test -p isolde-core marketplace::tests::test_.*_display`

Expected: PASS

**Step 3: Commit**

```bash
git add isolde-core/src/marketplace.rs
git commit -m "test: add Display trait tests for marketplace

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Phase 7: Final Verification

### Task 14: Run full test suite and verify coverage

**Files:**
- None (verification only)

**Step 1: Run all tests**

Run: `cargo test -p isolde-core`

Expected: All tests pass (should be ~70+ tests now)

**Step 2: Run clippy**

Run: `cargo clippy -p isolde-core`

Expected: No new warnings

**Step 3: Check coverage (optional)**

If `tarpaulin` is installed:

Run: `cargo tarpaulin -p isolde-core --out Term`

Expected: Coverage increased from ~50% to ~70%+

**Step 4: Final commit**

```bash
git add -A
git commit -m "test: complete test expansion for isolde-core

Added ~24 new tests covering:
- Template loading and error cases
- Config file I/O errors
- Generator helper functions
- File copy operations
- Git operation stubbing
- Full generate() workflow integration test
- Dry run functionality
- Error display messages
- Display trait implementations

Coverage increased from ~50% to ~70%+

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Summary

**Total tasks: 14**
**Estimated new tests: ~24**
**Estimated time: 3.5 hours**

**Test additions by module:**
- `template.rs`: +6 tests
- `config.rs`: +4 tests
- `generator.rs`: +10 tests
- `error.rs`: +2 tests
- `marketplace.rs`: +2 tests
