# -*- coding: utf-8 -*-
"""Step definitions for VS Code compatibility."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import then
import json
import subprocess
import os


@then('devcontainer.json should be valid JSON')
def step_devcontainer_json_valid(context):
    """Verify devcontainer.json is valid JSON."""
    devcontainer_path = os.path.join(
        context.project_path, ".devcontainer", "devcontainer.json"
    )

    with open(devcontainer_path) as f:
        try:
            context.devcontainer_config = json.load(f)
        except json.JSONDecodeError as e:
            raise AssertionError(f"Invalid JSON in {devcontainer_path}: {e}")


@then('devcontainer.json should contain field "{field}"')
def step_devcontainer_has_field(context, field):
    """Verify field exists in devcontainer.json."""
    assert field in context.devcontainer_config, f"Field '{field}' not found in devcontainer.json"


@then('devcontainer.json should specify VS Code extensions')
def step_vscode_extensions(context):
    """Verify VS Code extensions are specified."""
    extensions = context.devcontainer_config.get("customizations", {}) \
                                           .get("vscode", {}) \
                                           .get("extensions", [])
    assert len(extensions) > 0, "No VS Code extensions specified"


@then('claude command should exist in the container')
def step_claude_command_exists(context):
    """Verify claude command exists in container."""
    result = subprocess.run(
        f"docker run --rm {context.test_image} which claude",
        shell=True, capture_output=True, text=True
    )
    assert result.returncode == 0, f"claude command not found: {result.stderr}"


@then('claude --version command should work')
def step_claude_version_works(context):
    """Verify claude --version works."""
    result = subprocess.run(
        f"docker run --rm {context.test_image} claude --version",
        shell=True, capture_output=True, text=True
    )
    assert result.returncode == 0, f"claude --version failed: {result.stderr}"
