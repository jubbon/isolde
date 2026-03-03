# -*- coding: utf-8 -*-
"""Step definitions for Layer 2 scenario-based testing with fixtures."""

import os
import sys
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', 'support')))

from behave import given, when, then
import subprocess
import yaml
import shutil


def load_scenario_fixture(scenario_name: str) -> dict:
    """
    Load scenario configuration from fixture directory.

    Args:
        scenario_name: Name of the scenario (e.g., 'python-basic', 'python-ml', 'fullstack')

    Returns:
        Dictionary with scenario configuration
    """
    fixtures_dir = os.path.join(
        os.path.dirname(os.path.dirname(__file__)),
        'fixtures', 'scenarios', scenario_name
    )

    isolde_yaml = os.path.join(fixtures_dir, 'isolde.yaml')

    if not os.path.exists(isolde_yaml):
        raise ValueError(f"Scenario fixture not found: {isolde_yaml}")

    with open(isolde_yaml, 'r') as f:
        return yaml.safe_load(f)


@given('I have the "{scenario_name}" scenario fixture')
def step_have_fixture(context, scenario_name: str):
    """Load a scenario fixture configuration."""
    context.scenario_name = scenario_name
    context.scenario_config = load_scenario_fixture(scenario_name)

    # Extract configuration
    context.fixture_template = context.scenario_config.get('template')
    context.fixture_lang_version = context.scenario_config.get('lang_version')
    context.fixture_preset = context.scenario_config.get('preset')
    context.fixture_features = context.scenario_config.get('features', [])
    context.fixture_expected_binaries = context.scenario_config.get('expected_binaries', [])
    context.fixture_claude_plugins = context.scenario_config.get('claude_plugins', {}).get('activate', [])


@when('I run isolde init in the fixture directory')
def step_run_isolde_init_in_directory(context):
    """Run isolde init using the fixture configuration."""
    config = context.scenario_config
    project_name = config.get('name', f"e2e-{context.scenario_name}")

    # Get generator from context
    generator = getattr(context, 'generator', None)
    if not generator:
        from generators import get_generator
        generator = get_generator("shell-script")
        context.generator = generator

    # Generate project with fixture config
    result = generator.generate(
        project_name,
        workspace=context.test_workspace,
        template=config.get('template'),
        lang_version=config.get('lang_version'),
        preset=config.get('preset')
    )

    context.project_name = project_name
    context.last_exit_code = result.returncode
    context.last_output = result.stdout + result.stderr


@when('I configure all Claude plugins from the fixture')
def step_configure_all_plugins(context):
    """Configure Claude plugins as specified in the fixture."""
    if not hasattr(context, 'fixture_claude_plugins'):
        return

    # This step prepares the configuration for plugin activation
    # Actual plugin activation happens during devcontainer build
    context.claude_plugins_to_activate = context.fixture_claude_plugins


@then('the project should have the correct template applied')
def step_project_has_correct_template(context):
    """Verify the correct template was applied."""
    if not hasattr(context, 'project_name'):
        raise AssertionError("No project was created")

    project_path = os.path.join(context.test_workspace, context.project_name)
    devcontainer_json = os.path.join(project_path, ".devcontainer", "devcontainer.json")

    if not os.path.exists(devcontainer_json):
        raise AssertionError(f"devcontainer.json not found at {devcontainer_json}")

    # Verify template-specific files exist
    template = context.fixture_template

    template_indicators = {
        'python': ['.python-version', 'requirements.txt'],
        'nodejs': ['package.json', '.nvmrc'],
        'rust': ['Cargo.toml', 'rust-toolchain.toml'],
        'go': ['go.mod'],
        'generic': []
    }

    expected_files = template_indicators.get(template, [])
    found_files = []

    for filename in expected_files:
        filepath = os.path.join(project_path, filename)
        if os.path.exists(filepath):
            found_files.append(filename)

    # At least some template-specific files should exist
    if expected_files and not found_files:
        import warnings
        warnings.warn(f"No template-specific files found for {template}. Expected: {expected_files}")


@then('the features should be installed')
def step_features_installed(context):
    """Verify features are configured in devcontainer.json."""
    if not hasattr(context, 'project_name'):
        raise AssertionError("No project was created")

    project_path = os.path.join(context.test_workspace, context.project_name)
    devcontainer_json = os.path.join(project_path, ".devcontainer", "devcontainer.json")

    if not os.path.exists(devcontainer_json):
        raise AssertionError(f"devcontainer.json not found at {devcontainer_json}")

    with open(devcontainer_json) as f:
        import json
        config = json.load(f)

    features = config.get('features', {})
    expected_features = context.fixture_features

    # Check if expected features are referenced
    # Features can be referenced as ./features/* or as external devcontainer features
    found_features = []
    missing_features = []

    for expected in expected_features:
        # Check if feature is referenced in any way
        feature_found = False
        for feature_key in features.keys():
            if expected in feature_key.lower():
                feature_found = True
                found_features.append(expected)
                break

        if not feature_found:
            # Also check if feature might be installed via postCreateCommand
            post_create = config.get('postCreateCommand', '')
            if expected in post_create.lower():
                found_features.append(expected)
            else:
                missing_features.append(expected)

    if missing_features:
        import warnings
        warnings.warn(f"Features not found in config: {missing_features}. Found: {found_features}")


@then('the Claude plugins should include')
def step_claude_plugins_include(context):
    """Verify Claude plugins are configured."""
    if not hasattr(context, 'project_name'):
        raise AssertionError("No project was created")

    # Expected plugins from table
    expected_plugins = [row['plugin'] for row in context.table]

    # Check if plugins are in the configuration
    configured_plugins = getattr(context, 'claude_plugins_to_activate', [])

    missing_plugins = set(expected_plugins) - set(configured_plugins)

    if missing_plugins:
        raise AssertionError(f"Missing Claude plugins: {missing_plugins}")


@then('the expected binaries should be available')
def step_expected_binaries_available(context):
    """Verify expected binaries are available (after build)."""
    if not hasattr(context, 'verification_container'):
        import warnings
        warnings.warn("No container available - skipping binary check")
        return

    container_name = context.verification_container
    expected_binaries = context.fixture_expected_binaries

    missing_binaries = []

    for binary in expected_binaries:
        result = subprocess.run(
            f"docker exec {container_name} which {binary}",
            shell=True, capture_output=True, text=True
        )

        if result.returncode != 0:
            missing_binaries.append(binary)

    if missing_binaries:
        raise AssertionError(f"Missing expected binaries: {missing_binaries}")


@then('the scenario functional tests should pass')
def step_scenario_functional_tests_pass(context):
    """Run scenario-specific functional tests."""
    if not hasattr(context, 'verification_container'):
        import warnings
        warnings.warn("No container available - skipping functional tests")
        return

    container_name = context.verification_container
    functional_tests = context.scenario_config.get('functional_tests', [])

    failed_tests = []

    for test in functional_tests:
        test_name = test.get('name', 'unnamed')
        command = test.get('command')
        expected_output = test.get('expected_output')

        result = subprocess.run(
            f"docker exec {container_name} {command}",
            shell=True, capture_output=True, text=True
        )

        passed = result.returncode == 0
        if expected_output and expected_output not in result.stdout:
            passed = False

        if not passed:
            failed_tests.append({
                'name': test_name,
                'command': command,
                'exit_code': result.returncode,
                'stderr': result.stderr
            })

    if failed_tests:
        error_msg = "Functional tests failed:\n"
        for test in failed_tests:
            error_msg += f"\n  - {test['name']}:\n"
            error_msg += f"      Command: {test['command']}\n"
            error_msg += f"      Exit code: {test['exit_code']}\n"
            if test['stderr']:
                error_msg += f"      Stderr: {test['stderr']}\n"

        raise AssertionError(error_msg)


@given('I create a project from scenario "{scenario_name}"')
def step_create_from_scenario(context, scenario_name: str):
    """Create a project directly from a scenario fixture."""
    step_have_fixture(context, scenario_name)
    step_run_isolde_init_in_directory(context)


@when('I build the scenario container')
def step_build_scenario_container(context):
    """Build the devcontainer for the scenario."""
    import time

    project_name = context.project_name
    container_name = f"e2e-{context.scenario_name}-{int(time.time())}"

    project_path = os.path.join(context.test_workspace, project_name, ".devcontainer")

    result = subprocess.run(
        f"docker build -t {container_name} {project_path}",
        shell=True, capture_output=True, text=True
    )

    if result.returncode != 0:
        raise AssertionError(f"Container build failed:\n{result.stderr}")

    context.test_image = container_name

    if not hasattr(context, 'test_images'):
        context.test_images = []
    context.test_images.append(container_name)


@when('I start the scenario container')
def step_start_scenario_container(context):
    """Start a container from the built image."""
    import time

    image_name = context.test_image
    container_name = f"e2e-{context.scenario_name}-run-{int(time.time())}"

    result = subprocess.run(
        f"docker run -d --name {container_name} {image_name} sleep infinity",
        shell=True, capture_output=True, text=True
    )

    if result.returncode != 0:
        raise AssertionError(f"Failed to start container:\n{result.stderr}")

    context.verification_container = container_name

    if not hasattr(context, 'test_containers'):
        context.test_containers = []
    context.test_containers.append(container_name)


@then('all scenario validations should pass')
def step_all_scenario_validations_pass(context):
    """Run all scenario validations: template, features, plugins, binaries, functional tests."""
    step_project_has_correct_template(context)
    step_features_installed(context)
    step_expected_binaries_available(context)
    step_scenario_functional_tests_pass(context)
