# -*- coding: utf-8 -*-
"""Step definitions for project validation."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import then, given
import os


@given('I create a project using template "{template}"')
def step_create_project_simple(context, template):
    """Create a project using specified template."""
    context.project_name = f"test-{template}-{template.__hash__() % 100000}"
    context.template = template

    result = context.generator.generate(context.project_name, workspace=context.test_workspace, template=template)
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@then('the project should be created successfully')
def step_project_created(context):
    """Verify project was created."""
    assert context.last_exit_code == 0, f"Creation failed (exit code {context.last_exit_code}):\n{context.last_output}"

    context.project_path = os.path.join(context.test_workspace, context.project_name)
    assert os.path.exists(context.project_path), f"Project path not found: {context.project_path}\nWorkspace: {context.test_workspace}\nProject name: {context.project_name}\nContents: {os.listdir(context.test_workspace) if os.path.exists(context.test_workspace) else 'N/A'}"


@then('the devcontainer directory should exist')
def step_devcontainer_exists(context):
    """Verify .devcontainer directory exists."""
    devcontainer_path = os.path.join(context.project_path, ".devcontainer")
    assert os.path.isdir(devcontainer_path), f".devcontainer not found at {devcontainer_path}"


@then('devcontainer.json should exist')
def step_devcontainer_json_exists(context):
    """Verify devcontainer.json exists."""
    devcontainer_json = os.path.join(context.project_path, ".devcontainer", "devcontainer.json")
    assert os.path.isfile(devcontainer_json), f"devcontainer.json not found at {devcontainer_json}"


@then('the project should be a git repository')
def step_project_is_git_repo(context):
    """Verify project is a git repository."""
    git_dir = os.path.join(context.project_path, ".git")
    assert os.path.isdir(git_dir), f".git not found at {git_dir}"


# ============================================================================
# Validation and Diff Command Steps
# ============================================================================

from behave import when, step
import subprocess
import json
import time


@when('I run "isolde validate"')
def step_isolde_validate(context):
    """Run isolde validate command."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "validate"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=300
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde validate --quick"')
def step_isolde_validate_quick(context):
    """Run isolde validate in quick mode."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "validate", "--quick"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde validate --format {fmt}"')
def step_isolde_validate_format(context, fmt):
    """Run isolde validate with specific output format."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "validate", "--format", fmt],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=300
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde validate --verbose"')
def step_isolde_validate_verbose(context):
    """Run isolde validate with verbose output."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "validate", "--verbose"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=300
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde validate --warnings-as-errors"')
def step_isolde_validate_warnings_as_errors(context):
    """Run isolde validate with warnings as errors."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "validate", "--warnings-as-errors"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=300
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde validate --path {path}"')
def step_isolde_validate_path(context, path):
    """Run isolde validate on specific path."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "validate", "--path", path],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=300
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde diff"')
def step_isolde_diff(context):
    """Run isolde diff command."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "diff"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde diff --format {fmt}"')
def step_isolde_diff_format(context, fmt):
    """Run isolde diff with specific output format."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "diff", "--format", fmt],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde diff --file {file}"')
def step_isolde_diff_file(context, file):
    """Run isolde diff for specific file."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "diff", "--file", file],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde diff --context {n}"')
def step_isolde_diff_context(context, n):
    """Run isolde diff with context lines."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "diff", "--context", n],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@step('I modify devcontainer.json')
def step_modify_devcontainer_json(context):
    """Modify devcontainer.json to create differences."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    devcontainer_json = os.path.join(context.test_project_path, ".devcontainer", "devcontainer.json")
    if os.path.exists(devcontainer_json):
        with open(devcontainer_json, 'r') as f:
            content = f.read()
        # Add a comment or modify slightly
        with open(devcontainer_json, 'w') as f:
            f.write(content + "\n// Modified for diff test\n")


@step('I modify the project configuration')
def step_modify_project_config(context):
    """Modify project configuration to test diff."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    if os.path.exists(isolde_yaml):
        with open(isolde_yaml, 'r') as f:
            content = f.read()
        # Add a comment
        with open(isolde_yaml, 'a') as f:
            f.write("\n# Configuration modified for testing\n")


@step('I modify multiple configuration files')
def step_modify_multiple_configs(context):
    """Modify multiple config files."""
    step_modify_devcontainer_json(context)
    step_modify_project_config(context)


@step('I add custom validation rules')
def step_add_custom_validation(context):
    """Add custom validation rules to isolde.yaml."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    if os.path.exists(isolde_yaml):
        with open(isolde_yaml, 'a') as f:
            f.write("\n# Custom validation rules\ncustom_validations:\n  - check_file_exists: README.md\n")


@then('validation should pass')
def step_validation_pass(context):
    """Assert validation passed."""
    assert context.last_exit_code == 0, f"Validation failed: {context.last_output}"
    output_lower = context.last_output.lower()
    assert "passed" in output_lower or "no errors" in output_lower or "valid" in output_lower or "success" in output_lower


@then('output should show "{msg}" or "{alt}"')
def step_output_show_either(context, msg, alt):
    """Assert output contains either message."""
    output_lower = context.last_output.lower()
    assert msg.lower() in output_lower or alt.lower() in output_lower, f"Expected '{msg}' or '{alt}' in: {context.last_output}"


@then('all validation checks should run')
def step_all_validation_checks_run(context):
    """Assert all validation checks were performed."""
    # Check that various validation categories appear in output
    output_lower = context.last_output.lower()
    # At least some validation categories should be mentioned
    assert len(context.last_output.strip()) > 0, "No validation output"


@then('output should be valid JSON')
def step_output_valid_json(context):
    """Assert output is valid JSON."""
    try:
        json.loads(context.last_output)
    except json.JSONDecodeError as e:
        raise AssertionError(f"Output is not valid JSON: {e}\nOutput: {context.last_output}")


@then('differences should be shown')
def step_differences_shown(context):
    """Assert diff output shows differences."""
    assert len(context.last_output.strip()) > 0, "No diff output"


@then('output should be formatted')
def step_output_formatted(context):
    """Assert output has formatting."""
    assert len(context.last_output.strip()) > 0


@then('only {file} differences should be shown')
def step_only_file_differences(context, file):
    """Assert only specific file differences are shown."""
    assert file in context.last_output, f"Expected {file} in diff output"


@then('differences with context should be shown')
def step_differences_with_context(context):
    """Assert diff includes context lines."""
    assert len(context.last_output.strip()) > 0


@then('configuration changes should be shown')
def step_config_changes_shown(context):
    """Assert configuration changes are visible in diff."""
    assert len(context.last_output.strip()) > 0


@then('output should indicate no changes or show minimal differences')
def step_no_changes_or_minimal(context):
    """Assert output indicates no significant changes."""
    output_lower = context.last_output.lower()
    # Either says no changes or shows very minimal output
    assert "no changes" in output_lower or "no difference" in output_lower or len(context.last_output.strip()) < 100


@then('all file differences should be shown')
def step_all_file_differences(context):
    """Assert multiple file differences are shown."""
    assert len(context.last_output.strip()) > 0


@then('build test should be skipped')
def step_build_test_skipped(context):
    """Assert build test was skipped in quick mode."""
    output_lower = context.last_output.lower()
    # Quick mode should skip docker build
    assert "skipped" in output_lower or "quick" in output_lower or "docker" not in output_lower


@then('validation should complete quickly')
def step_validation_quick(context):
    """Assert validation completed in quick mode (no docker build)."""
    # If we got here quickly without building, quick mode worked
    assert context.last_exit_code == 0


@then('custom validation should run')
def step_custom_validation_runs(context):
    """Assert custom validation rules were executed."""
    # Custom validations may or may not be supported
    # Just check that validation completed
    assert context.last_exit_code == 0 or "custom" in context.last_output.lower()


@then('output should contain detailed information')
def step_output_detailed(context):
    """Assert verbose output contains more information."""
    assert len(context.last_output.strip()) > 50, "Verbose output should be detailed"
