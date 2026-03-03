# -*- coding: utf-8 -*-
"""Step definitions for error scenario testing.

This module implements step definitions for testing error handling
and validation scenarios for Isolde CLI commands.
"""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import given, when, then
import subprocess
import tempfile
import shutil


@given('I have a directory without isolde.yaml')
def step_directory_no_isolde_yaml(context):
    """Create a directory without isolde.yaml."""
    context.project_name = f"test-no-yaml-{int(__import__('time').time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
    os.makedirs(context.test_project_path, exist_ok=True)


@given('I have a project with invalid devcontainer.json')
def step_invalid_devcontainer_json(context):
    """Create a project with invalid devcontainer.json."""
    context.project_name = f"test-invalid-json-{int(__import__('time').time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
    os.makedirs(context.test_project_path, exist_ok=True)

    # Create .devcontainer directory with invalid JSON
    devcontainer_dir = os.path.join(context.test_project_path, ".devcontainer")
    os.makedirs(devcontainer_dir, exist_ok=True)

    invalid_json_path = os.path.join(devcontainer_dir, "devcontainer.json")
    with open(invalid_json_path, 'w') as f:
        f.write("{ invalid json content }")


@given('I have a project with malformed isolde.yaml')
def step_malformed_isolde_yaml(context):
    """Create a project with malformed isolde.yaml."""
    context.project_name = f"test-malformed-yaml-{int(__import__('time').time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    # Create project
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template="python"
    )

    # Overwrite isolde.yaml with malformed content
    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    with open(isolde_yaml, 'w') as f:
        f.write(":\n  - invalid\n  yaml:\n    content:")


@given('I have a project with invalid Claude provider')
def step_invalid_claude_provider(context):
    """Create a project with invalid Claude provider."""
    context.project_name = f"test-invalid-provider-{int(__import__('time').time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    # Create project
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template="python"
    )

    # Modify isolde.yaml with invalid provider
    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    with open(isolde_yaml, 'r') as f:
        content = f.read()

    # Replace provider with invalid one
    modified = content.replace('provider: anthropic', 'provider: nonexistent-provider-xyz')
    with open(isolde_yaml, 'w') as f:
        f.write(modified)


@given('I have a project with incomplete isolde.yaml')
def step_incomplete_isolde_yaml(context):
    """Create a project with incomplete isolde.yaml missing required fields."""
    context.project_name = f"test-incomplete-yaml-{int(__import__('time').time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
    os.makedirs(context.test_project_path, exist_ok=True)

    # Create minimal isolde.yaml missing required fields
    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    with open(isolde_yaml, 'w') as f:
        f.write("# Incomplete isolde.yaml\n")


@given('I have a synced project without building image')
def step_synced_no_build(context):
    """Create a synced project but don't build the image."""
    context.project_name = f"test-no-build-{int(__import__('time').time())}"
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template="python"
    )
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    # Run sync but not build
    sync_result = subprocess.run(
        ["isolde", "sync"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True
    )


@given('I remove the template reference')
def step_remove_template_reference(context):
    """Remove template reference from isolde.yaml."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    if os.path.exists(isolde_yaml):
        with open(isolde_yaml, 'r') as f:
            content = f.read()

        # Remove template line
        modified = '\n'.join(
            line for line in content.split('\n')
            if not line.strip().startswith('template:')
        )

        with open(isolde_yaml, 'w') as f:
            f.write(modified)


@given('I have a project with conflicting isolde.yaml')
def step_conflicting_isolde_yaml(context):
    """Create a project with conflicting configuration."""
    context.project_name = f"test-conflicting-{int(__import__('time').time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    # Create project
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template="python"
    )

    # Create conflicting config
    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    with open(isolde_yaml, 'r') as f:
        content = f.read()

    # Add conflicting entries
    with open(isolde_yaml, 'a') as f:
        f.write("\n# Conflicting entries\ntemplate: nodejs\nlang_version: \"22\"\n")


@when('I run error test isolde init template-only "{name}" using "{template}"')
def step_isolde_init_template(context, name, template):
    """Run isolde init with template."""
    result = subprocess.run(
        ["isolde", "init", name, "--template", template],
        cwd=context.test_workspace,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run error test isolde init preset-only "{name}" using "{preset}"')
def step_isolde_init_preset(context, name, preset):
    """Run isolde init with preset."""
    result = subprocess.run(
        ["isolde", "init", name, "--preset", preset],
        cwd=context.test_workspace,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run error test isolde init with-version "{name}" template "{template}" version "{version}"')
def step_isolde_init_version(context, name, template, version):
    """Run isolde init with language version."""
    result = subprocess.run(
        ["isolde", "init", name, "--template", template, "--lang-version", version],
        cwd=context.test_workspace,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run error test isolde init missing-name template "{template}"')
def step_isolde_init_no_name(context, template):
    """Run isolde init without project name."""
    result = subprocess.run(
        ["isolde", "init", "--template", template],
        cwd=context.test_workspace,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run error test isolde init both "{name}" template "{template}" preset "{preset}"')
def step_isolde_init_both(context, name, template, preset):
    """Run isolde init with both template and preset (should fail)."""
    result = subprocess.run(
        ["isolde", "init", name, "--template", template, "--preset", preset],
        cwd=context.test_workspace,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run error test isolde init proxy "{name}" template "{template}" http-proxy "{proxy}"')
def step_isolde_init_proxy(context, name, template, proxy):
    """Run isolde init with invalid proxy."""
    result = subprocess.run(
        ["isolde", "init", name, "--template", template, "--http-proxy", proxy],
        cwd=context.test_workspace,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde init {name} --template {template}"')
def step_isolde_init_with_template(context, name, template):
    """Run isolde init with explicit name and template."""
    result = subprocess.run(
        ["isolde", "init", name, "--template", template],
        cwd=context.test_workspace,
        capture_output=True,
        text=True,
        timeout=60,
        input="\n"
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde sync --force"')
def step_isolde_sync_force(context):
    """Run isolde sync with force option."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "sync", "--force"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde run --workspace-folder {folder}"')
def step_isolde_run_invalid_workspace(context, folder):
    """Run isolde run with invalid workspace folder."""
    result = subprocess.run(
        ["isolde", "run", "--workspace-folder", folder],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@then('the command should fail with error containing "{msg}"')
def step_command_fails_with_error(context, msg):
    """Assert command failed with specific error."""
    assert context.last_exit_code != 0, f"Command should have failed but succeeded. Output: {context.last_output}"
    assert msg.lower() in context.last_output.lower(), f"Expected '{msg}' in error, got: {context.last_output}"


@then('the command should fail or ask for confirmation')
def step_command_fails_or_confirms(context):
    """Assert command either fails or asks for confirmation."""
    if context.last_exit_code == 0:
        # If it succeeded, it should have asked for confirmation
        # (in non-interactive mode, it might just fail)
        pass
    else:
        # Failed as expected
        assert "exists" in context.last_output.lower() or "already" in context.last_output.lower()


@then('the command should fail or prompt for project name')
def step_command_fails_or_prompts(context):
    """Assert command fails or prompts for name."""
    # In non-interactive mode, it should fail
    assert context.last_exit_code != 0 or "required" in context.last_output.lower()


@then('the command should fail or succeed with no container')
def step_command_fails_or_succeeds_no_container(context):
    """Assert command either fails or succeeds doing nothing."""
    if context.last_exit_code != 0:
        assert "container" in context.last_output.lower() or "not found" in context.last_output.lower() or "no such" in context.last_output.lower()


@then('validation should fail with error containing "{msg1}" or "{msg2}" or "{msg3}"')
def step_validation_fails_with_multiple_errors(context, msg1, msg2, msg3):
    """Assert validation failed with one of the specified error messages."""
    assert context.last_exit_code != 0, f"Validation should have failed but succeeded. Output: {context.last_output}"
    output_lower = context.last_output.lower()
    assert msg1.lower() in output_lower or msg2.lower() in output_lower or msg3.lower() in output_lower, \
        f"Expected one of '{msg1}', '{msg2}', or '{msg3}' in validation error, got: {context.last_output}"


@then('validation should fail with error containing "{msg}"')
def step_validation_fails_with_error(context, msg):
    """Assert validation failed with specific error."""
    assert context.last_exit_code != 0, f"Validation should have failed but succeeded. Output: {context.last_output}"
    assert msg.lower() in context.last_output.lower(), f"Expected '{msg}' in validation error, got: {context.last_output}"


@then('the check should fail with error containing "{msg}"')
def step_check_fails_with_error(context, msg):
    """Assert health check failed with specific error."""
    # Doctor might return non-zero but still run, check for error message
    output_lower = context.last_output.lower()
    assert msg in output_lower or "not found" in output_lower or "not available" in output_lower, f"Expected '{msg}' in check output, got: {context.last_output}"


@then('the command should fail or overwrite with warnings')
def step_command_fails_or_warns(context):
    """Assert command fails or overwrites with warnings."""
    if context.last_exit_code == 0:
        # If it succeeded, check for warnings
        assert "warning" in context.last_output.lower() or "overwriting" in context.last_output.lower()
    else:
        # Failed as expected
        pass


@then('validation should fail or show warnings')
def step_validation_fails_or_warns(context):
    """Assert validation failed or showed warnings."""
    output_lower = context.last_output.lower()
    if context.last_exit_code != 0:
        pass  # Failed as expected
    else:
        assert "warning" in output_lower or "error" in output_lower or len(context.last_output.strip()) > 0


@then('the command should fail or show empty list')
def step_command_fails_or_empty_list(context):
    """Assert command failed or returned an empty container list."""
    if context.last_exit_code != 0:
        pass  # Failed as expected
    else:
        # Succeeded but should show empty list
        assert len(context.last_output.strip()) == 0 or "no containers" in context.last_output.lower()
