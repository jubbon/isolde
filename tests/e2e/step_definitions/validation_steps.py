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
    assert context.last_exit_code == 0, f"Creation failed: {context.last_output}"

    context.project_path = os.path.join(context.test_workspace, context.project_name)
    assert os.path.exists(context.project_path), f"Project path not found: {context.project_path}"


@then('the project should have dual git repositories')
def step_dual_git_repos(context):
    """Verify dual git repositories."""
    project_git = os.path.join(context.project_path, "project", ".git")
    devcontainer_git = os.path.join(context.project_path, ".devcontainer", ".git")

    assert os.path.isdir(project_git), f"project/.git not found at {project_git}"
    assert os.path.isdir(devcontainer_git), f".devcontainer/.git not found at {devcontainer_git}"


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


@then('the project directory should exist')
def step_project_dir_exists(context):
    """Verify project directory exists."""
    project_dir = os.path.join(context.project_path, "project")
    assert os.path.isdir(project_dir), f"project/ not found at {project_dir}"
