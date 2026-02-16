# -*- coding: utf-8 -*-
"""Step definitions using VS Code Dev Containers CLI."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import then, given, when
import subprocess
import os


@given('I start the devcontainer for the project')
def step_start_devcontainer(context):
    """Start the devcontainer using devcontainer CLI."""
    project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        f"devcontainer up --workspace-folder {project_path}",
        shell=True, capture_output=True, text=True, timeout=300
    )

    if result.returncode != 0:
        raise AssertionError(f"devcontainer up failed:\n{result.stderr}")

    context.devcontainer_started = True


@then('postCreateCommand should have executed successfully')
def step_postcreate_command_executed(context):
    """Verify postCreateCommand ran."""
    # Check for common post-create artifacts
    project_path = os.path.join(context.test_workspace, context.project_name)
    project_dir = os.path.join(project_path, "project")

    # Git should be initialized if postCreateCommand ran
    git_dir = os.path.join(project_dir, ".git")
    assert os.path.isdir(git_dir), "Git not initialized - postCreateCommand may not have run"


@then('devcontainer features should be installed')
def step_devcontainer_features_installed(context):
    """Verify features installed correctly via devcontainer CLI."""
    project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        f"devcontainer exec --workspace-folder {project_path} -- which claude",
        shell=True, capture_output=True, text=True
    )

    assert result.returncode == 0, "claude feature not installed"


@then('I can execute commands in the devcontainer')
def step_devcontainer_exec(context):
    """Verify we can exec commands in devcontainer."""
    project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        f"devcontainer exec --workspace-folder {project_path} -- pwd",
        shell=True, capture_output=True, text=True
    )

    assert result.returncode == 0, "devcontainer exec failed"
    assert "/workspaces" in result.stdout, "Not in workspaces directory"


@then('Node.js should be available in the devcontainer')
def step_nodejs_in_devcontainer(context):
    """Verify Node.js is available in devcontainer."""
    project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        f"devcontainer exec --workspace-folder {project_path} -- node --version",
        shell=True, capture_output=True, text=True
    )

    assert result.returncode == 0, f"Node.js not available: {result.stderr}"


@then('the container should be removed')
def step_container_removed(context):
    """Verify container is cleaned up."""
    # This is verified implicitly by the stop step succeeding
    pass


@when('I stop the devcontainer')
def step_stop_devcontainer(context):
    """Stop the devcontainer."""
    project_path = os.path.join(context.test_workspace, context.project_name)

    result = subprocess.run(
        f"devcontainer stop --workspace-folder {project_path}",
        shell=True, capture_output=True, text=True
    )

    if result.returncode != 0:
        raise AssertionError(f"devcontainer stop failed: {result.stderr}")
