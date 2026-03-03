# -*- coding: utf-8 -*-
"""Step definitions for Isolde CLI container management commands.

This module implements step definitions for testing the isolde CLI commands
related to container management: build, run, exec, stop, ps, and logs.
"""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import given, when, then, step
import subprocess
import time
import json


@given('I have a synced project')
def step_have_synced_project(context):
    """Create and sync a project for testing."""
    context.project_name = f"test-synced-{int(time.time())}"
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template="python"
    )
    context.last_exit_code = result.returncode
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    # Run sync
    sync_result = subprocess.run(
        ["isolde", "sync"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    assert sync_result.returncode == 0, f"Sync failed: {sync_result.stderr}"


@given('I have a synced project without running container')
def step_synced_no_container(context):
    """Create a synced project without starting a container."""
    step_have_synced_project(context)


@given('I have a project without .devcontainer')
def step_project_no_devcontainer(context):
    """Create a project directory without .devcontainer."""
    context.project_name = f"test-no-devcontainer-{int(time.time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
    subprocess.run(["mkdir", "-p", context.test_project_path], check=True)


@step('I run "isolde sync"')
def step_isolde_sync(context):
    """Run isolde sync command."""
    if not hasattr(context, 'test_project_path'):
        context.test_project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        ["isolde", "sync"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde build"')
def step_isolde_build(context):
    """Run isolde build command."""
    result = subprocess.run(
        ["isolde", "build"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=600
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde build --no-cache"')
def step_isolde_build_no_cache(context):
    """Run isolde build without cache."""
    result = subprocess.run(
        ["isolde", "build", "--no-cache"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=600
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde build --image-name {image}"')
def step_isolde_build_custom_image(context, image):
    """Run isolde build with custom image name."""
    result = subprocess.run(
        ["isolde", "build", "--image-name", image],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=600
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr
    context.custom_image_name = image


@when('I run "isolde build --workspace-folder {folder}"')
def step_isolde_build_workspace(context, folder):
    """Run isolde build with workspace folder option."""
    result = subprocess.run(
        ["isolde", "build", "--workspace-folder", folder],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=600
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde run --detach"')
def step_isolde_run_detach(context):
    """Run isolde in detached mode."""
    result = subprocess.run(
        ["isolde", "run", "--detach"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr
    time.sleep(3)  # Wait for container to start


@when('I run "isolde run --detach --workspace-folder {folder}"')
def step_isolde_run_detach_workspace(context, folder):
    """Run isolde in detached mode with workspace folder."""
    result = subprocess.run(
        ["isolde", "run", "--detach", "--workspace-folder", folder],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=60
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr
    time.sleep(3)


@when('I run "isolde exec {command}"')
def step_isolde_exec(context, command):
    """Run command in container via isolde exec."""
    cmd = ["isolde", "exec"] + command.split()
    result = subprocess.run(
        cmd,
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=30
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde ps"')
def step_isolde_ps(context):
    """List running containers."""
    result = subprocess.run(
        ["isolde", "ps"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde ps --all"')
def step_isolde_ps_all(context):
    """List all containers including stopped."""
    result = subprocess.run(
        ["isolde", "ps", "--all"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde logs"')
def step_isolde_logs(context):
    """Get container logs."""
    result = subprocess.run(
        ["isolde", "logs"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde logs --tail {n}"')
def step_isolde_logs_tail(context, n):
    """Get last n lines of container logs."""
    result = subprocess.run(
        ["isolde", "logs", "--tail", n],
        cwd=context.test_project_path,
        capture_output=True,
        text=True
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout


@when('I run "isolde stop"')
def step_isolde_stop(context):
    """Stop the running container."""
    result = subprocess.run(
        ["isolde", "stop"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=30
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I run "isolde stop --force"')
def step_isolde_stop_force(context):
    """Force stop the running container."""
    result = subprocess.run(
        ["isolde", "stop", "--force"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True,
        timeout=30
    )
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@then('isolde build should succeed')
def step_isolde_build_succeed(context):
    """Assert isolde build succeeded."""
    assert context.last_exit_code == 0, f"Build failed (exit code {context.last_exit_code}): {context.last_output}"
    assert "successfully" in context.last_output.lower() or "done" in context.last_output.lower() or "built" in context.last_output.lower()


@then('the build should not use layer cache')
def step_build_no_cache(context):
    """Assert build didn't use layer cache."""
    # When --no-cache is used, the output should mention it or show all steps being re-run
    output_lower = context.last_output.lower()
    assert "--no-cache" in output_lower or "no cache" in output_lower or "pulling image" in output_lower


@then('the image should be tagged "{tag}"')
def step_image_tagged(context, tag):
    """Assert image has correct tag."""
    result = subprocess.run(
        ["docker", "images", "--format", "{{.Repository}}:{{.Tag}}"],
        capture_output=True,
        text=True
    )
    assert tag in result.stdout, f"Image {tag} not found in: {result.stdout}"


@then('the container should be running')
def step_container_running(context):
    """Assert container is running."""
    time.sleep(2)
    result = subprocess.run(
        ["isolde", "ps"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True
    )
    assert len(result.stdout.strip()) > 0, "No containers running"


@then('the container should be running in background')
def step_container_running_background(context):
    """Assert container is running in detached/background mode."""
    step_container_running(context)


@then('the output should contain "{text}"')
def step_output_contains(context, text):
    """Assert output contains expected text."""
    assert text in context.last_output, f"Expected '{text}' in output, got: {context.last_output}"


@then('I should see the container in the list')
def step_container_in_list(context):
    """Assert container appears in ps output."""
    assert len(context.last_output.strip()) > 0, "Container list is empty"


@then('logs should be displayed')
def step_logs_displayed(context):
    """Assert logs were shown."""
    assert len(context.last_output.strip()) > 0, "No logs displayed"
    assert context.last_exit_code == 0, f"Logs command failed: {context.last_output}"


@then('logs should contain "{text}"')
def step_logs_contain(context, text):
    """Assert logs contain specific text."""
    assert text in context.last_output, f"Expected '{text}' in logs, got: {context.last_output}"


@then('the container should be stopped')
def step_container_stopped(context):
    """Assert container is stopped."""
    time.sleep(2)
    result = subprocess.run(
        ["isolde", "ps"],
        cwd=context.test_project_path,
        capture_output=True,
        text=True
    )
    # Container should not be in running list (output should be empty or just header)
    output = result.stdout.strip()
    assert len(output) == 0 or "NAME" in output, f"Container still running: {output}"


@then('I should see the stopped container')
def step_stopped_container_visible(context):
    """Assert stopped container appears in --all list."""
    assert len(context.last_output.strip()) > 0, "Stopped container not visible"


@then('both commands should execute')
def step_both_commands_execute(context):
    """Assert both commands in bash -c executed successfully."""
    assert context.last_exit_code == 0, f"Commands failed: {context.last_output}"
    assert "hello" in context.last_output, f"Expected 'hello' in output: {context.last_output}"


@then('pytest should be available')
def step_pytest_available(context):
    """Assert pytest is available."""
    assert context.last_exit_code == 0, f"pytest check failed: {context.last_output}"
    assert "pytest" in context.last_output.lower(), f"pytest not found in output: {context.last_output}"


@then('Python should be installed')
def step_python_installed_cli(context):
    """Assert Python is available in the container."""
    assert context.last_exit_code == 0, f"Python check failed: {context.last_output}"
    assert "Python" in context.last_output, f"Python not found in output: {context.last_output}"
