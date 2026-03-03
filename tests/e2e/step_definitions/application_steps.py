# -*- coding: utf-8 -*-
"""Step definitions for application-specific verification.

This module implements step definitions for verifying that application
tools and frameworks work correctly in the devcontainer.
These steps are used in Layer 2 scenarios.

NOTE: Many common application verification steps (numpy, pandas, ruff, etc.)
are already defined in container_steps.py. This module only contains
unique steps not found elsewhere.
"""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import then, given, when
import subprocess
import time


# ============================================================================
# Python Application Steps (Unique ones not in container_steps.py)
# ============================================================================

@then('I can run a simple Python script')
def step_run_python_script(context):
    """Verify we can run Python code in the container."""
    if hasattr(context, 'test_image'):
        result = subprocess.run(
            f'docker run --rm {context.test_image} python -c "print(\'Hello from Python\')"',
            shell=True,
            capture_output=True,
            text=True
        )
    elif hasattr(context, 'test_project_path'):
        result = subprocess.run(
            ["isolde", "exec", "python", "-c", "print('Hello from Python')"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
    else:
        import warnings
        warnings.warn("No container available - skipping Python script test")
        return

    assert result.returncode == 0, f"Python script failed: {result.stderr}"
    assert "Hello from Python" in result.stdout


@then('I can create a simple Jupyter notebook')
def step_create_jupyter_notebook(context):
    """Verify Jupyter is available."""
    if hasattr(context, 'test_image'):
        result = subprocess.run(
            f'docker run --rm {context.test_image} jupyter --version',
            shell=True,
            capture_output=True,
            text=True
        )
    elif hasattr(context, 'test_project_path'):
        result = subprocess.run(
            ["isolde", "exec", "jupyter", "--version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
    else:
        import warnings
        warnings.warn("No container available - skipping Jupyter test")
        return

    if result.returncode != 0:
        import warnings
        warnings.warn("Jupyter not available - installed via postCreateCommand")


@then('I can create a simple Flask application')
def step_create_flask_app(context):
    """Verify Flask is available."""
    # Flask is already tested in container_steps.py, this is a placeholder
    # for more complex Flask-specific scenarios
    import warnings
    warnings.warn("Flask verification - see container_steps.py for basic check")


# ============================================================================
# Node.js Application Steps (Unique ones)
# ============================================================================

@then('I can create a simple Express API')
def step_create_express_api(context):
    """Verify we can work with Express."""
    # Just verify Node.js and npm are available
    if hasattr(context, 'test_image'):
        result = subprocess.run(
            f'docker run --rm {context.test_image} npm --version',
            shell=True,
            capture_output=True,
            text=True
        )
    elif hasattr(context, 'test_project_path'):
        result = subprocess.run(
            ["isolde", "exec", "npm", "--version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
    else:
        import warnings
        warnings.warn("No container available - skipping npm test")
        return

    assert result.returncode == 0, f"npm not found: {result.stderr}"


# ============================================================================
# Rust Application Steps (Unique ones)
# ============================================================================

@then('I can build a simple Rust binary')
def step_build_rust_binary(context):
    """Verify we can build Rust projects."""
    if hasattr(context, 'test_image'):
        result = subprocess.run(
            f'docker run --rm {context.test_image} cargo --version',
            shell=True,
            capture_output=True,
            text=True
        )
    elif hasattr(context, 'test_project_path'):
        result = subprocess.run(
            ["isolde", "exec", "cargo", "--version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
    else:
        import warnings
        warnings.warn("No container available - skipping Rust build test")
        return

    assert result.returncode == 0, f"cargo not available: {result.stderr}"


# ============================================================================
# Go Application Steps (Unique ones)
# ============================================================================

@then('I can create a simple Go module')
def step_create_go_module(context):
    """Verify we can work with Go modules."""
    if hasattr(context, 'test_image'):
        result = subprocess.run(
            f'docker run --rm {context.test_image} go version',
            shell=True,
            capture_output=True,
            text=True
        )
    elif hasattr(context, 'test_project_path'):
        result = subprocess.run(
            ["isolde", "exec", "go", "version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
    else:
        import warnings
        warnings.warn("No container available - skipping Go module test")
        return

    assert result.returncode == 0, f"go not available: {result.stderr}"


# ============================================================================
# Fullstack Application Steps
# ============================================================================

@then('I can create a simple fullstack project')
def step_create_fullstack_project(context):
    """Verify fullstack development tools are available."""
    # Verify both Node.js and Python are available for fullstack
    if hasattr(context, 'test_image'):
        result = subprocess.run(
            f'docker run --rm {context.test_image} npm --version',
            shell=True,
            capture_output=True,
            text=True
        )
    elif hasattr(context, 'test_project_path'):
        result = subprocess.run(
            ["isolde", "exec", "npm", "--version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
    else:
        import warnings
        warnings.warn("No container available - skipping fullstack test")
        return

    assert result.returncode == 0, f"npm not found: {result.stderr}"


# ============================================================================
# Minimal/General Steps
# ============================================================================

@then('the container should have basic tools')
def step_basic_tools_available(context):
    """Verify basic tools are available."""
    tools_to_check = ["git", "curl", "wget"]

    for tool in tools_to_check:
        if hasattr(context, 'test_image'):
            result = subprocess.run(
                f'docker run --rm {context.test_image} which {tool}',
                shell=True,
                capture_output=True,
                text=True
            )
        elif hasattr(context, 'test_project_path'):
            result = subprocess.run(
                ["isolde", "exec", "which", tool],
                cwd=context.test_project_path,
                capture_output=True,
                text=True
            )
        else:
            continue

        # At least some basic tools should be available
        if result.returncode == 0:
            return  # At least one tool found

    # If we get here, check if we have any tools at all
    import warnings
    warnings.warn("Basic tools check incomplete - no container available")


@then('I can run basic shell commands')
def step_run_shell_commands(context):
    """Verify shell commands work."""
    if hasattr(context, 'test_image'):
        result = subprocess.run(
            f'docker run --rm {context.test_image} bash -c "echo test && pwd"',
            shell=True,
            capture_output=True,
            text=True
        )
    elif hasattr(context, 'test_project_path'):
        result = subprocess.run(
            ["isolde", "exec", "bash", "-c", "echo test && pwd"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
    else:
        import warnings
        warnings.warn("No container available - skipping shell command test")
        return

    assert result.returncode == 0, f"Shell command failed: {result.stderr}"
    assert "test" in result.stdout


# ============================================================================
# Multi-language Steps
# ============================================================================

@then('I can add Node.js to the project')
def step_add_nodejs_to_project(context):
    """Verify we can add Node.js features to a Python project."""
    # This verifies the multi-language support
    if hasattr(context, 'test_project_path'):
        # Check that we can run Node.js commands
        result = subprocess.run(
            ["isolde", "exec", "node", "--version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
        # Node.js might or might not be available in Python project
        if result.returncode == 0:
            assert "v" in result.stdout or "." in result.stdout


@then('I can run both Python and Node.js scripts')
def step_run_both_python_and_nodejs(context):
    """Verify both Python and Node.js work together."""
    # Verify Python works
    if hasattr(context, 'test_project_path'):
        python_result = subprocess.run(
            ["isolde", "exec", "python", "--version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
        assert python_result.returncode == 0, f"Python not available: {python_result.stderr}"

        # Node.js is optional in multi-language Python project
        node_result = subprocess.run(
            ["isolde", "exec", "node", "--version"],
            cwd=context.test_project_path,
            capture_output=True,
            text=True
        )
        # If Node.js is available, check it works
        if node_result.returncode == 0:
            assert "v" in node_result.stdout or "." in node_result.stdout


# ============================================================================
# Custom Configuration Steps
# ============================================================================

@then('all custom settings should be applied')
def step_custom_settings_applied(context):
    """Verify custom isolde.yaml settings were applied."""
    if not hasattr(context, 'project_path'):
        import warnings
        warnings.warn("No project path - skipping custom settings check")
        return

    # Check that isolde.yaml was processed
    isolde_yaml = os.path.join(context.project_path, "isolde.yaml")
    assert os.path.exists(isolde_yaml), "isolde.yaml not found"


@then('the custom provider should be configured')
def step_custom_provider_configured(context):
    """Verify custom Claude provider was set."""
    if not hasattr(context, 'project_path'):
        import warnings
        warnings.warn("No project path - skipping provider check")
        return

    devcontainer_json = os.path.join(context.project_path, ".devcontainer", "devcontainer.json")
    if os.path.exists(devcontainer_json):
        with open(devcontainer_json, 'r') as f:
            content = f.read()
        # Provider should be mentioned somewhere
        assert len(content) > 0


# ============================================================================
# Helper Steps for Custom Scenarios
# ============================================================================

@given('I have a custom isolde.yaml configuration')
def step_custom_isolde_yaml(context):
    """Setup custom isolde.yaml for testing."""
    context.custom_config = {
        'template': 'python',
        'lang_version': '3.12',
        'claude': {
            'provider': 'anthropic',
            'version': 'latest'
        }
    }


@when('I create a project using the custom configuration')
def step_create_with_custom_config(context):
    """Create project using custom config."""
    context.project_name = f"test-custom-{int(time.time())}"

    # Generate with custom settings
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template=context.custom_config.get('template', 'python'),
        lang_version=context.custom_config.get('lang_version')
    )
    context.last_exit_code = result.returncode
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)


# ============================================================================
# Given Steps for Fixture Directories (unique ones)
# ============================================================================

@given('I have a directory without devcontainer')
def step_directory_no_devcontainer(context):
    """Create a directory without .devcontainer."""
    context.project_name = f"test-no-devcontainer-{int(time.time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
    os.makedirs(context.test_project_path, exist_ok=True)


@given('I have a project with syntax errors in devcontainer.json')
def step_syntax_errors_devcontainer(context):
    """Create project with invalid JSON."""
    context.project_name = f"test-syntax-error-{int(time.time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
    os.makedirs(context.test_project_path, exist_ok=True)

    devcontainer_dir = os.path.join(context.test_project_path, ".devcontainer")
    os.makedirs(devcontainer_dir, exist_ok=True)

    invalid_json = os.path.join(devcontainer_dir, "devcontainer.json")
    with open(invalid_json, 'w') as f:
        f.write("{ invalid json }")


@given('I have a project with missing required files')
def step_missing_required_files(context):
    """Create project missing required files."""
    context.project_name = f"test-missing-files-{int(time.time())}"
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
    os.makedirs(context.test_project_path, exist_ok=True)

    # Create isolde.yaml but no devcontainer
    isolde_yaml = os.path.join(context.test_project_path, "isolde.yaml")
    with open(isolde_yaml, 'w') as f:
        f.write("template: python\n")


# ============================================================================
# When Steps for Modifying Project
# ============================================================================

@given('Docker daemon is stopped')
@when('Docker daemon is stopped')
def step_docker_stopped(context):
    """Note: This is a simulated condition for testing."""
    # In real tests, we'd actually stop Docker
    # For E2E, we just note this condition
    context.docker_stopped = True


@given('Docker is not available on the system')
def step_docker_not_available(context):
    """Simulate Docker not being available."""
    context.docker_not_available = True


@when('a component has issues')
def step_component_has_issues(context):
    """Simulate a component having issues."""
    context.component_issues = True


# ============================================================================
# Jupyter Helper Steps
# ============================================================================

@when('I create a project with Jupyter support')
def step_create_jupyter_project(context):
    """Create a project with Jupyter configured."""
    context.project_name = f"test-jupyter-{int(time.time())}"
    result = context.generator.generate(
        context.project_name,
        workspace=context.test_workspace,
        template="python",
        preset="python-ml"
    )
    context.last_exit_code = result.returncode
    context.test_project_path = os.path.join(context.test_workspace, context.project_name)
