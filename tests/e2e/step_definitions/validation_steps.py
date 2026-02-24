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
